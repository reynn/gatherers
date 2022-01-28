// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![warn(clippy::all)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_macros, unused_imports, unused_variables)
)]

mod builder;
mod constants;
mod gatherer;
mod responses;
mod structs;

use crate::responses::MessagesResponse;
use builder::OnlyFansBuilder;
use cookie::Cookie;
use gatherer_core::http::json;
use gatherer_core::{
    gatherers::{Gatherer, GathererErrors},
    http::{self, Client, ClientConfig, Headers, Url},
    Result,
};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::future::Future;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use structs::DynamicRule;

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
    dynamic_rule: structs::DynamicRule,
    http_client: Client,
    cookie: String,
    authed_user: structs::Me,
}

/// Impl block for OnlyFans basic struct functions
impl OnlyFans {
    pub async fn new(of_conf: OnlyFansConfig) -> Result<OnlyFans> {
        if !of_conf.enabled {
            return Err(Box::new(GathererErrors::NotEnabled {
                name: String::from("OnlyFans"),
            }));
        }

        let http_client = Client::new(ClientConfig {
            base_url: Some(constants::BASE_URL.to_string()),
        });

        let mut ofb = OnlyFansBuilder::new(of_conf);
        ofb.with_dynamic_rule(get_dc_dynamic_rule(&http_client).await?);
        ofb.add_http_client(http_client);
        ofb.parse_cookie_string();
        Ok(ofb.build().await?)
    }
}

fn generate_request_headers(
    config: &'_ OnlyFansConfig,
    path: &'_ str,
    dynamic_rule: &'_ DynamicRule,
) -> Headers {
    let cookie = &config.cookie;

    let mut h = HashMap::new();
    h.insert(
        "accept".to_string(),
        "application/json, text/plain, */*".to_string(),
    );
    h.insert(
        "app-token".to_string(),
        match config.app_token.clone() {
            Some(token) => token,
            None => dynamic_rule.app_token.clone(),
        },
    );
    h.insert("x-bc".to_string(), config.x_bc.to_string());
    h.insert("referer".to_string(), "https://onlyfans.com".to_string());
    h.insert("user-agent".to_string(), config.user_agent.to_string());
    h.insert("cookie".to_string(), cookie.to_string());
    h.insert("user-id".to_string(), config.auth_id.to_string());

    // Add the necessary signed headers
    let (signed, epoch_time) = create_signed_headers(path, &config.auth_id, dynamic_rule);
    h.insert("sign".to_string(), signed);
    h.insert("time".to_string(), epoch_time);

    h
}

/// Impl block for OnlyFans API calls
impl OnlyFans {
    async fn get_subscriptions(&self) -> Result<Vec<structs::Subscription>> {
        let mut subscriptions = Vec::new();
        let mut offset = 0;
        let mut more_pages = true;

        while more_pages {
            let endpoint = format!("/api2/v2/subscriptions/subscribes?offset={offset}&type=active&sort=desc&field=expire_date&limit=10");
            if let Ok(partial_subs) = self
                .http_client
                .get(
                    &endpoint,
                    Some(crate::generate_request_headers(
                        &self.config,
                        &endpoint,
                        &self.dynamic_rule,
                    )),
                )
                .await
            {
                let mut subs: responses::SubscriptionResponse = partial_subs.as_json().await?;
                if subs.len() < 10 {
                    more_pages = false;
                }
                subscriptions.append(&mut subs);
                offset += 10;
            }
        }

        Ok(subscriptions)
    }

    async fn get_user_posts(&self, user_id: &str) -> Result<Vec<structs::Post>> {
        let mut posts = Vec::new();

        // Get pinned posts for user
        let endpoint = format!(
            "/api2/v2/users/{user_id}/posts?skip_users=all&pinned=1&counters=0&format=infinite"
        );
        match self
            .http_client
            .get(
                &endpoint,
                Some(crate::generate_request_headers(
                    &self.config,
                    &endpoint,
                    &self.dynamic_rule,
                )),
            )
            .await
        {
            Ok(posts_response) => {
                let response: Result<responses::PostsResponse> = posts_response.as_json().await;
                match response {
                    Ok(mut pinned_posts) => posts.append(&mut pinned_posts.list),
                    Err(pinned_err) => {
                        log::error!("Failed to get pinned posts for {user_id}: {pinned_err}")
                    }
                }
            }
            Err(response_err) => return Err(response_err),
        }

        let mut last_pub_time: Option<String> = None;

        loop {
            let endpoint = if let Some(pub_time) = last_pub_time {
                format!("/api2/v2/users/{user_id}/posts?limit=10&order=publish_date_desc&skip_users=all&pinned=0&format=infinite&beforePublishTime={pub_time}")
            } else {
                format!("/api2/v2/users/{user_id}/posts?limit=10&order=publish_date_desc&skip_users=all&pinned=0&format=infinite")
            };
            match self
                .http_client
                .get(
                    &endpoint,
                    Some(crate::generate_request_headers(
                        &self.config,
                        &endpoint,
                        &self.dynamic_rule,
                    )),
                )
                .await
            {
                Ok(posts_response) => {
                    let mut response: responses::PostsResponse = posts_response.as_json().await?;
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
                Err(response_err) => return Err(response_err),
            }
        }

        Ok(posts)
    }

    async fn get_user_messages(&self, user_id: &str) -> Result<Vec<structs::Message>> {
        let mut messages = Vec::new();

        let mut last_message_id: Option<i64> = None;

        loop {
            let endpoint = match last_message_id {
                None => format!("/api2/v2/chats/{user_id}/messages?limit=10&offset=0&order=desc&skip_users=all"),
                Some(message_id) => format!("/api2/v2/chats/{user_id}/messages?limit=10&offset=0&id={message_id}&order=desc&skip_users=all")
            };

            match self
                .http_client
                .get(
                    &endpoint,
                    Some(crate::generate_request_headers(
                        &self.config,
                        &endpoint,
                        &self.dynamic_rule,
                    )),
                )
                .await
            {
                Ok(success_response) => {
                    let msg_success: Result<responses::MessagesResponse> =
                        success_response.as_json().await;
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
                                if curr_msg.from_user.id != authed_user_id {
                                    messages.push(curr_msg);
                                }
                            }
                        }
                        Err(as_json_err) => {
                            log::debug!(
                                "Failed to convert message response into JSON: {:?}",
                                as_json_err
                            );
                            return Err(as_json_err);
                        }
                    }
                }
                Err(error_response) => {
                    log::debug!(
                        "Received a bad response while getting messages for {}. {:?}",
                        user_id,
                        error_response
                    );
                    return Err(error_response);
                }
            }
        }

        Ok(messages)
    }

    async fn get_user_stories(&self, user_id: &str) -> Result<Vec<structs::Story>> {
        let endpoint = format!("/api2/v2/users/{user_id}/stories?unf=1");
        match self
            .http_client
            .get(
                &endpoint,
                Some(crate::generate_request_headers(
                    &self.config,
                    &endpoint,
                    &self.dynamic_rule,
                )),
            )
            .await
        {
            Ok(success_resp) => {
                let user_stories: Result<responses::StoriesResponse> = success_resp.as_json().await;
                match user_stories {
                    Ok(all_stories) => {
                        Ok(all_stories)
                    }
                    Err(invalid_json_err) => {
                        Err(format!("Got user stories for {user_id} but unable to convert to JSON. {invalid_json_err}").into())
                    }
                }
            }
            Err(err_resp) => {
                Err(format!("Failed to get stories for user {user_id}. {err_resp}").into())
            }
        }
    }

    async fn get_transactions(&self) -> Result<Vec<structs::Transaction>> {
        let mut transactions = Vec::new();
        let mut has_more = true;
        let mut marker: Option<i64> = None;

        while has_more {
            let endpoint = if let Some(marker) = marker {
                format!("/api2/v2/payments/all/transactions?limit=10&marker={marker}&type=payment")
            } else {
                "/api2/v2/payments/all/transactions?limit=10&type=payment".into()
            };

            match self
                .http_client
                .get(
                    &endpoint,
                    Some(crate::generate_request_headers(
                        &self.config,
                        &endpoint,
                        &self.dynamic_rule,
                    )),
                )
                .await
            {
                Ok(success_resp) => {
                    let available_transactions: Result<responses::TransactionsResponse> =
                        success_resp.as_json().await;
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
                Err(transaction_err) => {}
            }
        }

        Ok(transactions)
    }
}

async fn get_dc_dynamic_rule(api_client: &'_ Client) -> Result<DynamicRule> {
    let url = Url::parse(constants::DC_DYNAMIC_RULE).unwrap();
    let resp = api_client.get(&url, None).await?;
    Ok(resp.as_json().await?)
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
    let checksum_constant = &rule.checksum_constant;
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
