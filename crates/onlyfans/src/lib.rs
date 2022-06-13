mod builder;
mod constants;
mod gatherer;
mod responses;
mod structs;

use eyre::{bail, eyre};
use gatherer_core::http;
use {
    crate::{
        builder::OnlyFansBuilder,
        structs::{DynamicRule, ListUser},
    },
    gatherer_core::{
        http::{
            header::{HeaderMap, HeaderValue},
            Client, ClientConfig, Method, Request, Url,
        },
        Result,
    },
    serde::{Deserialize, Serialize},
    sha1::{Digest, Sha1},
    std::time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OnlyFansConfig {
    pub enabled: bool,
    pub app_token: Option<String>,
    pub cookie: String,
    pub auth_id: String,
    pub x_bc: String,
    pub user_agent: String,
    pub ignore_lists: Vec<String>,
}

#[derive(Debug)]
pub struct OnlyFans {
    config: OnlyFansConfig,
    dynamic_rule: DynamicRule,
    http_client: Client,
    authed_user: structs::Me,
}

/// Impl block for OnlyFans basic struct functions
impl OnlyFans {
    pub async fn new(of_conf: OnlyFansConfig) -> Result<OnlyFans> {
        if !of_conf.enabled {
            bail!("OnlyFans is not enabled");
        }

        let http_client = http::new(ClientConfig {
            base_url: Some(constants::BASE_URL),
            cookies: Some(&of_conf.cookie),
        });

        let mut ofb = OnlyFansBuilder::new(of_conf);
        ofb.with_dynamic_rule(get_dc_dynamic_rule(&http_client).await?);
        ofb.add_http_client(http_client);
        ofb.build().await
    }
}

fn generate_request_headers<'a>(
    config: &'a OnlyFansConfig,
    path: &'_ str,
    dynamic_rule: &'a DynamicRule,
) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(
        "accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    let token = match &config.app_token {
        None => &dynamic_rule.app_token,
        Some(t) => t,
    };

    h.insert("app-token", HeaderValue::from_str(token).unwrap());
    h.insert("x-bc", HeaderValue::from_str(&config.x_bc).unwrap());
    h.insert("referer", HeaderValue::from_static("https://onlyfans.com"));
    h.insert(
        "user-agent",
        HeaderValue::from_str(&config.user_agent).unwrap(),
    );
    h.insert("cookie", HeaderValue::from_str(&config.cookie).unwrap());
    h.insert("user-id", HeaderValue::from_str(&config.auth_id).unwrap());

    // Add the necessary signed headers
    let (signed, epoch_time) = create_signed_headers(path, &config.auth_id, dynamic_rule);
    h.insert("sign", HeaderValue::from_str(&signed).unwrap());
    h.insert("time", HeaderValue::from_str(&epoch_time).unwrap());

    h
}

/// Impl block for OnlyFans API calls
impl OnlyFans {
    fn create_req(&self, method: Method, endpoint: &'_ str) -> http::Request {
        self.http_client
            .request(method, endpoint.parse::<Url>().unwrap())
            .headers(generate_request_headers(
                &self.config,
                endpoint,
                &self.dynamic_rule,
            ))
            .build()
            .unwrap()
    }

    async fn get_subscriptions(
        &self,
        sub_status: Option<&'_ str>,
    ) -> Result<Vec<structs::Subscription>> {
        let mut subscriptions = Vec::new();
        let mut offset = 0;
        let mut more_pages = true;

        while more_pages {
            let endpoint = if let Some(status) = sub_status {
                format!("{}/api2/v2/subscriptions/subscribes?offset={offset}&type={status}&sort=desc&field=expire_date&limit=10", constants::BASE_URL)
            } else {
                format!("{}/api2/v2/subscriptions/subscribes?offset={offset}&sort=desc&field=expire_date&limit=10", constants::BASE_URL)
            };
            let req = self.create_req(Method::GET, &endpoint);
            if let Ok(partial_subs) = self.http_client.execute(req).await {
                let subs: responses::SubscriptionResponse = partial_subs.json().await?;
                if subs.len() < 10 {
                    more_pages = false;
                }

                // return only the active subscribers
                subscriptions.append(
                    &mut subs
                        .into_iter()
                        .filter(|s| !s.subscribed_is_expired_now)
                        .collect(),
                );
                offset += 10;
            }
        }
        Ok(subscriptions)
    }

    async fn get_paid_content(&self) -> Result<Vec<structs::PurchasedItem>> {
        let mut paid_content = Vec::new();
        let mut has_more = true;
        let mut offset = 0;

        while has_more {
            let endpoint = format!(
                "{}/api2/v2/posts/paid?limit=10&skip_users=all&format=infinite&offset={}",
                constants::BASE_URL,
                offset
            );
            let req = self.create_req(Method::GET, &endpoint);

            match self.http_client.execute(req).await {
                Ok(response) => {
                    let purchases_response: http::Result<responses::PurchasedItemsResponse> =
                        response.json().await;
                    match purchases_response {
                        Ok(mut purchases) => {
                            paid_content.append(&mut purchases.list);
                            if !purchases.has_more {
                                has_more = false;
                            }
                        }
                        Err(json_err) => {
                            log::error!("Failed to serialize JSON into struct: {:?}", json_err)
                        }
                    }
                }
                Err(response_err) => {
                    log::debug!(
                        "Failed to get data from endpoint {}. {:?}",
                        endpoint,
                        response_err
                    );
                }
            }
            offset += 10;
        }

        Ok(paid_content)
    }

    async fn get_user_posts(&self, user_id: &str) -> Result<Vec<structs::Post>> {
        let mut posts = Vec::new();

        // Get pinned posts for user
        let endpoint = format!(
            "{}/api2/v2/users/{user_id}/posts?skip_users=all&pinned=1&counters=0&format=infinite",
            constants::BASE_URL
        );
        let req = self.create_req(Method::GET, &endpoint);

        match self.http_client.execute(req).await {
            Ok(posts_response) => {
                let response: http::Result<responses::PostsResponse> = posts_response.json().await;
                match response {
                    Ok(mut pinned_posts) => posts.append(&mut pinned_posts.list),
                    Err(pinned_err) => {
                        log::error!("Failed to get pinned posts for {user_id}: {pinned_err}")
                    }
                }
            }
            Err(response_err) => return Err(eyre!(response_err)),
        }

        let mut last_pub_time: Option<String> = None;

        loop {
            let endpoint = if let Some(pub_time) = last_pub_time {
                format!("{}/api2/v2/users/{user_id}/posts?limit=10&order=publish_date_desc&skip_users=all&pinned=0&format=infinite&beforePublishTime={pub_time}", constants::BASE_URL)
            } else {
                format!("{}/api2/v2/users/{user_id}/posts?limit=10&order=publish_date_desc&skip_users=all&pinned=0&format=infinite", constants::BASE_URL)
            };
            let req = self.create_req(Method::GET, &endpoint);

            match self.http_client.execute(req).await {
                Ok(posts_response) => {
                    let mut response: responses::PostsResponse = posts_response.json().await?;
                    if !response.has_more {
                        break;
                    } else {
                        last_pub_time = response
                            .list
                            .iter()
                            .last()
                            .map(|last_item| last_item.posted_at_precise.clone());
                    }
                    posts.append(&mut response.list);
                }
                Err(response_err) => return Err(eyre!(response_err)),
            }
        }

        Ok(posts)
    }

    async fn get_user_messages(&self, user_id: &str) -> Result<Vec<structs::Message>> {
        let mut messages = Vec::new();

        let mut last_message_id: Option<i64> = None;

        loop {
            let endpoint = match last_message_id {
                None => format!("{}/api2/v2/chats/{user_id}/messages?limit=10&offset=0&order=desc&skip_users=all", constants::BASE_URL),
                Some(message_id) => format!("{}/api2/v2/chats/{user_id}/messages?limit=10&offset=0&id={message_id}&order=desc&skip_users=all", constants::BASE_URL)
            };

            match self
                .http_client
                .execute(self.create_req(Method::GET, &endpoint))
                .await
            {
                Ok(success_response) => {
                    let msg_success: http::Result<responses::MessagesResponse> =
                        success_response.json().await;
                    match msg_success {
                        Ok(curr_messages) => {
                            let authed_user_id = self.authed_user.id;
                            if curr_messages.has_more {
                                last_message_id = curr_messages
                                    .list
                                    .iter()
                                    .last()
                                    .map(|last_item| last_item.id);
                            } else {
                                break;
                            }
                            // filter out messages that have been sent by the authed user
                            for curr_msg in curr_messages.list.into_iter() {
                                match &curr_msg.from_user {
                                    None => messages.push(curr_msg),
                                    Some(from_user) => {
                                        if from_user.id != authed_user_id {
                                            messages.push(curr_msg)
                                        }
                                    }
                                }
                            }
                        }
                        Err(as_json_err) => {
                            log::debug!(
                                "Failed to convert message response into JSON: {:?}",
                                as_json_err
                            );
                            return Err(eyre!(as_json_err));
                        }
                    }
                }
                Err(error_response) => {
                    log::debug!(
                        "Received a bad response while getting messages for {}. {:?}",
                        user_id,
                        error_response
                    );
                    return Err(eyre!(error_response));
                }
            }
        }

        Ok(messages)
    }

    async fn get_user_stories(&self, user_id: &str) -> Result<Vec<structs::Story>> {
        let endpoint = format!(
            "{}/api2/v2/users/{user_id}/stories?unf=1",
            constants::BASE_URL
        );
        match self
            .http_client
            .execute(self.create_req(Method::GET, &endpoint))
            .await
        {
            Ok(success_resp) => {
                let user_stories: http::Result<responses::StoriesResponse> =
                    success_resp.json().await;
                match user_stories {
                    Ok(all_stories) => Ok(all_stories),
                    Err(invalid_json_err) => Err(eyre!(
                        "Got user stories for {} but unable to convert to JSON. {}",
                        user_id,
                        invalid_json_err
                    )),
                }
            }
            Err(err_resp) => Err(eyre!(
                "Failed to get stories for user {}. {}",
                user_id,
                err_resp
            )),
        }
    }

    async fn get_transactions(&self) -> Result<Vec<structs::Transaction>> {
        let mut transactions = Vec::new();
        let mut has_more = true;
        let mut marker: Option<i64> = None;

        while has_more {
            let endpoint = if let Some(marker) = marker {
                format!(
                    "{}/api2/v2/payments/all/transactions?limit=10&marker={marker}&type=payment",
                    constants::BASE_URL
                )
            } else {
                format!(
                    "{}/api2/v2/payments/all/transactions?limit=10&type=payment",
                    constants::BASE_URL
                )
            };

            match self
                .http_client
                .execute(self.create_req(Method::GET, &endpoint))
                .await
            {
                Ok(success_resp) => {
                    let available_transactions: http::Result<responses::TransactionsResponse> =
                        success_resp.json().await;
                    match available_transactions {
                        Ok(mut available_transactions) => {
                            has_more = available_transactions.has_more;
                            marker = available_transactions.next_marker;
                            transactions.append(&mut available_transactions.list)
                        }
                        Err(json_err) => {
                            log::error!(
                                "Response from {endpoint} did not return expected response. {:?}",
                                json_err
                            )
                        }
                    }
                }
                Err(transaction_err) => {
                    log::error!("{:?}", transaction_err);
                    bail!(transaction_err);
                }
            }
        }

        Ok(transactions)
    }

    async fn get_users_by_id(&self, user_ids: &[i64]) -> Result<Vec<ListUser>> {
        let endpoint = format!(
            "{}/api2/v2/users/list?{}",
            constants::BASE_URL,
            user_ids
                .iter()
                .map(|user_id| format!("t[]={}", user_id))
                .collect::<Vec<String>>()
                .join("&")
        );

        match self
            .http_client
            .execute(self.create_req(Method::GET, &endpoint))
            .await
        {
            Ok(response) => {
                let users: http::Result<responses::ListOfUsersResponse> = response.json().await;
                match users {
                    Ok(users_list) => Ok(users_list.into_iter().map(|(_, user)| user).collect()),
                    Err(json_err) => Err(eyre!(json_err)),
                }
            }
            Err(http_err) => Err(eyre!(http_err)),
        }
    }
}

async fn get_dc_dynamic_rule(api_client: &'_ Client) -> http::Result<DynamicRule> {
    let req = api_client.get(constants::DC_DYNAMIC_RULE).build()?;
    api_client.execute(req).await?.json().await
}

fn create_signed_headers(
    path: &'_ str,
    user_id: &'_ str,
    rule: &'_ DynamicRule,
) -> (String, String) {
    let rule = rule.clone();
    let cur_time = SystemTime::now();
    let since_epoch = cur_time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let static_param = &rule.static_param;
    let msg = vec![static_param.as_str(), since_epoch.as_str(), path, user_id].join("\n");
    let sha = calculate_sha1(msg);
    let sha_ascii = sha.to_ascii_lowercase();

    let mut result: i32 = 0;
    let sha_ascii = sha_ascii.as_bytes();
    for idx in rule.checksum_indexes.into_iter() {
        let ascii_byte = sha_ascii.get(idx).unwrap_or(&0);
        result += *ascii_byte as i32;
    }
    let checksum = (result + rule.checksum_constant).abs();
    log::trace!("Header checksum bytes: {:?}", checksum);
    // might be a better way to do this?
    let py_format = rule.format.clone();
    // format! macro can't do this unfortunately
    let final_sign = py_format
        .replace("{}", &sha.to_ascii_lowercase())
        .replace("{:x}", format!("{:x}", checksum).as_str());

    log::trace!("Final Signed header value: {}", final_sign);

    (final_sign, since_epoch)
}

fn calculate_sha1(s: String) -> String {
    data_encoding::HEXLOWER.encode(Sha1::digest(s.as_bytes()).as_ref())
}
