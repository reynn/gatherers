// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![warn(clippy::all)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_macros, unused_imports, unused_variables)
)]

mod constants;
mod gatherer;
mod responses;
#[cfg(test)]
mod responses_test;
mod structs;

pub use self::gatherer::*;
use chrono::prelude::*;
use gatherer_core::{
    gatherers::{self, Gatherer, GathererErrors, Subscription, SubscriptionName},
    http::{self, Client, ClientConfig, Headers, Url},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, sync::Arc};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FanslyConfig {
    pub enabled: bool,
    pub auth_token: String,
    pub ignore_lists: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Fansly {
    conf: FanslyConfig,
    http_client: Client,
}

/// Functions to interact with the fansly struct
impl Fansly {
    pub async fn new(fansly_conf: FanslyConfig) -> Result<Fansly> {
        if !fansly_conf.enabled {
            return Err(Box::new(GathererErrors::NotEnabled {
                name: String::from("Fansly"),
            }));
        };

        log::info!("Initializing Fansly...");
        let api_config = ClientConfig {
            base_url: Some(constants::BASE_URL.to_string()),
        };
        let s = Self {
            http_client: Client::new(api_config),
            conf: fansly_conf,
        };
        match s.validate_auth_token().await {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_accounts_by_names(
        &self,
        names: &[String],
    ) -> Result<Vec<structs::Account>> {
        let endpoint = format!(
            "{}?usernames={}",
            constants::USER_ACCOUNT_URL,
            names.join(",")
        );
        let resp = self.http_client.get(&endpoint, None).await;
        match resp {
            Ok(out) => {
                let out: Result<responses::AccountsResponse> = out.as_json().await;
                match out {
                    Ok(ret_val) => Ok(ret_val.response),
                    Err(json_err) => Err(format!(
                        "Failed to convert to JSON from {}. {:?}",
                        &endpoint, json_err
                    )
                    .into()),
                }
            }
            Err(bundles_err) => Err(bundles_err),
        }
    }

    pub async fn validate_auth_token(&self) -> Result<&Fansly> {
        if self.conf.auth_token.is_empty() {
            return Err(Box::new(GathererErrors::InvalidCredentials {
                name: "Fansly".into(),
                msg: "Cannot be used without an auth token.".into(),
            }));
        };
        let resp = self
            .http_client
            .post(
                constants::STATUS_URL,
                self.get_default_headers(),
                Some(http::json!([("statusId", 1)])),
            )
            .await?;
        log::debug!("validate_auth_token: {:?}", resp);
        Ok(self)
    }

    fn get_default_headers(&self) -> Option<Headers> {
        let mut hm = HashMap::new();
        hm.insert(
            "Authorization".to_string(),
            self.conf.auth_token.to_string(),
        );
        Some(hm)
    }
}

/// API functions to gather data
impl Fansly {
    pub async fn get_user_accounts_by_ids(
        &self,
        account_ids: &[String],
    ) -> Result<responses::AccountsResponse> {
        let endpoint = &format!(
            "{}?ids={}",
            constants::USER_ACCOUNT_URL,
            account_ids.join(",")
        );
        let resp = self.http_client.get(&endpoint, None).await?;
        Ok(resp.as_json().await?)
    }

    pub async fn get_media_by_ids(&self, media_ids: &[String]) -> Result<Vec<structs::Media>> {
        let mut returned_media = Vec::new();
        if media_ids.is_empty() {
            return Ok(Vec::new());
        }
        log::debug!("Attempting to get {} media files", media_ids.len());
        for ids_chunked in media_ids.chunks(100) {
            let endpoint = format!("{}?ids={}", constants::MEDIA_URL, ids_chunked.join(","));
            let media = self
                .http_client
                .get(&endpoint, self.get_default_headers())
                .await;
            match media {
                Ok(response) => {
                    let mut response: responses::MediaResponse = response.as_json().await?;
                    log::trace!("Media Response {:?}\n", response.response);
                    returned_media.append(&mut response.response);
                    // Some(response.response)
                }
                Err(err) => {
                    log::error!("Failed to download media info due to {}", err);
                    // None
                }
            }
        }
        Ok(returned_media)
    }

    pub async fn get_media_bundles_by_ids(
        &self,
        bundle_ids: &[String],
    ) -> Result<Vec<structs::MediaBundle>> {
        let endpoint = &format!(
            "{}?ids={}",
            constants::MEDIA_BUNDLE_URL,
            bundle_ids.join(",")
        );
        let bundles = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;
        match bundles {
            Ok(bundles_response) => {
                let bundles_response: responses::MediaBundleResponse =
                    bundles_response.as_json().await?;
                Ok(bundles_response.response)
            }
            Err(bundles_err) => Err(bundles_err),
        }
    }

    pub async fn get_posts_by_user_id(
        &self,
        account_id: &'_ str,
    ) -> Result<Vec<responses::inner::Posts>> {
        let mut posts: Vec<responses::inner::Posts> = Vec::new();
        let mut more_pages = true;
        let mut offset: usize = 0;
        let mut before_post_id = String::from("0");

        while more_pages {
            let endpoint = format!(
                "{}/{}?before={}&after=0",
                constants::TIMELINE_URL,
                account_id,
                before_post_id
            );
            log::debug!("Endpoint: [{}]", endpoint);
            let response = self
                .http_client
                .get(&endpoint, self.get_default_headers())
                .await;
            match response {
                Ok(post_response) => {
                    let post_response: responses::PostsResponse = post_response.as_json().await?;
                    if post_response.response.account_media.is_none()
                        && post_response.response.account_media_bundles.is_none()
                        && post_response.response.aggregated_posts.is_none()
                        && post_response.response.posts.is_none()
                        && post_response.response.stories.is_none()
                    {
                        break;
                    }
                    if let Some(user_posts) = &post_response.response.posts {
                        if user_posts.len() < constants::POSTS_LIMIT_COUNT as usize
                            && user_posts.is_empty()
                        {
                            more_pages = false
                        }
                        user_posts.iter().for_each(|post| {
                            before_post_id = post.id.to_string();
                        });
                        posts.push(post_response.response);
                        log::debug!("Got page {} of users posts", offset + 1);
                    }
                }
                Err(err) => return Err(err),
            }
            offset += 1;
        }

        Ok(posts)
    }

    pub async fn get_account_subscriptions(&self) -> Result<Vec<Subscription>> {
        let endpoint = constants::SUBS_URL;
        let subs = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;
        log::debug!("Subs response: {:?}", subs);
        match subs {
            Ok(resp) => {
                let resp: responses::SubscriptionResponse = resp.as_json().await?;
                if resp.success {
                    let mut sub_account_ids: Vec<String> = resp
                        .response
                        .subscriptions
                        .iter()
                        .filter_map(|fan_sub| {
                            if fan_sub.status == 3 {
                                Some(fan_sub.account_id.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                    log::info!(
                        "Found {} accounts that are being subscribed to",
                        sub_account_ids.len()
                    );
                    if let Ok(account_stubs) = self.get_followed_accounts_stubs().await {
                        let mut stub_ids: Vec<String> =
                            account_stubs.into_iter().map(|s| s.account_id).collect();
                        log::info!(
                            "Found {} accounts that are being followed but not subscribed to",
                            stub_ids.len()
                        );
                        sub_account_ids.append(&mut stub_ids)
                    };
                    log::info!(
                        "Total accounts (subs + followed) = {}",
                        sub_account_ids.len()
                    );
                    log::debug!("Subscription ids: {:?}", sub_account_ids);
                    // get the full user account info so we can attach extra data to our subscription info
                    match self.get_user_accounts_by_ids(&sub_account_ids).await {
                        Ok(account_infos) => {
                            let account_infos = account_infos.response;
                            let subscriptions = resp.response.subscriptions;
                            Ok(combine_subs_and_account_info(
                                &subscriptions,
                                &account_infos,
                            ))
                        }
                        Err(user_account_err) => {
                            log::error!("Failed to gather user accounts: {:?}", user_account_err);
                            Err(user_account_err)
                        }
                    }
                } else {
                    log::error!("Response from Fansly failed, {:?}", resp);
                    Err(Box::new(GathererErrors::NoSubscriptionsFound {
                        gatherer: self.name().to_string(),
                        data: format!("{:?}", resp.response),
                    }))
                }
            }
            Err(resp_err) => {
                log::error!("Response from  {:?}", resp_err);
                Err(resp_err)
            }
        }
    }

    pub async fn get_messages_groups(&self) -> Result<Vec<structs::MessageGroup>> {
        let endpoint = constants::MESSAGE_GROUPS_URL;
        let all_groups_resp = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;
        match all_groups_resp {
            Ok(resp) => {
                let groups: responses::MessageGroupsResponse = resp.as_json().await?;
                Ok(groups.response.groups)
            }
            Err(groups_err) => {
                Err(format!("Error getting message groups: {:?}", groups_err).into())
            }
        }
    }

    pub async fn get_followed_accounts_stubs(&self) -> Result<Vec<structs::FollowedAccount>> {
        let endpoint = format!("{}?limit={}&offset=0", constants::MEDIA_BUNDLE_URL, 100);
        let followed = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;
        match followed {
            Ok(resp) => {
                let accounts: responses::FollowedAccountsResponse = resp.as_json().await?;
                Ok(accounts.response)
            }
            Err(resp_err) => Err(resp_err),
        }
    }

    pub async fn get_account_stories(&self, account_id: &'_ str) -> Result<Vec<structs::Story>> {
        let endpoint = format!("{}?accountId={}", constants::USER_STORIES_URL, account_id);
        let stories = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;
        match stories {
            Ok(resp) => {
                let stories: responses::AccountStoriesResponse = resp.as_json().await?;
                Ok(stories.response)
            }
            Err(resp_err) => Err(resp_err),
        }
    }

    pub async fn get_all_messages_from_group(
        &self,
        group_id: &'_ str,
    ) -> Result<Vec<structs::Message>> {
        let endpoint = format!(
            "{}?groupId={}&limit={}",
            constants::GROUP_MESSAGES_URL,
            &group_id,
            constants::GROUP_MESSAGES_LIMIT
        );
        let resp_res = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;

        match resp_res {
            Ok(resp) => {
                let resp: responses::GroupMessagesResponse = resp.as_json().await?;
                log::debug!("Response for thread {}. {:?}", group_id, resp.response);
                Ok(resp.response.messages)
            }
            Err(message_err) => Err(format!(
                "Failed to get messages from group {}. {:?}",
                group_id, message_err,
            )
            .into()),
        }
    }

    pub async fn get_purchased_content(&self) -> Result<Vec<structs::PurchasedMedia>> {
        Ok(Vec::new())
    }
}

fn combine_subs_and_account_info(
    subs: &[structs::Subscription],
    accounts: &[structs::Account],
) -> Vec<Subscription> {
    subs.iter()
        .filter_map(|sub| {
            let account_info = accounts.iter().find(|c| c.id == sub.account_id);
            match account_info {
                Some(info) => {
                    let mut video_count = 0;
                    let mut image_count = 0;
                    let mut bundle_count = 0;
                    if let Some(stats) = &info.timeline_stats {
                        video_count = stats.video_count;
                        image_count = stats.image_count;
                        bundle_count = stats.bundle_count;
                    };
                    let sub_tier = if let Some(tier_name) = &sub.subscription_tier_name {
                        tier_name
                    } else {
                        "Unknown"
                    };
                    Some(Subscription {
                        id: info.id.to_string(),
                        name: SubscriptionName {
                            username: info.username.to_string(),
                            display_name: info.display_name.to_owned(),
                        },
                        plan: String::from(sub_tier),
                        started: Some(Utc.timestamp_millis(sub.created_at).into()),
                        renewal_date: Some(Utc.timestamp_millis(sub.renew_date).into()),
                        rewewal_price: (sub.price as f64).into(),
                        ends_at: Some(Utc.timestamp_millis(sub.ends_at).into()),
                        video_count,
                        image_count,
                        bundle_count,
                    })
                }
                None => None,
            }
        })
        .collect()
}

fn fansly_details_to_gatherers_media(
    details: &'_ structs::MediaDetails,
    loc: &'_ str,
    purchased: bool,
) -> gatherers::Media {
    let file_name = Path::new(&details.filename);
    let ext = match file_name.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => {
            let mimetype = &details.mimetype;
            log::error!(
                "File name `{}` does not have an extension. Inferring from mimetype: {}",
                file_name.display(),
                &mimetype
            );
            mimetype.split('/').last().unwrap()
        }
    };
    gatherers::Media {
        file_name: format!("{}.{}", details.id, ext),
        paid: purchased,
        mime_type: details.mimetype.to_string(),
        url: loc.to_string(),
    }
}

fn fansly_media_to_gatherers_media(fansly: structs::Media) -> Option<gatherers::Media> {
    log::trace!("Converting to gatherer_core::Media. {:?}", fansly);
    if let Some(details) = &fansly.details {
        if let Some(location) = &details.locations.get(0) {
            Some(fansly_details_to_gatherers_media(
                details,
                &location.location[..],
                fansly.purchased,
            ))
        } else {
            log::debug!("Unable to determine a location for {}", details.filename);
            None
        }
    } else {
        log::debug!("Unable to find details for {}", fansly.id);
        None
    }
}
