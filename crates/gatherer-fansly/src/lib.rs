// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod gatherer;
mod responses;
mod structs;

use crate::structs::Message;

pub use self::gatherer::*;
use chrono::prelude::*;
use gatherer_core::{
    gatherers::{Gatherer, GathererErrors, Subscription, SubscriptionName},
    http::{ApiClient, ApiClientConfig},
    AsyncResult,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};

const FANLSY_POST_LIMIT_COUNT: usize = 10;
const FANSLY_API_STATUS_URL: &str = "https://apiv2.fansly.com/api/v1/status";
const FANSLY_API_USER_ACCOUNT_URL: &str = "https://apiv2.fansly.com/api/v1/account";
const FANSLY_API_SUBS_URL: &str = "https://apiv2.fansly.com/api/v1/subscriptions";
const FANSLY_API_TIMELINE_URL: &str = "https://apiv2.fansly.com/api/v1/timeline";
const FANSLY_API_MEDIA_URL: &str = "https://apiv2.fansly.com/api/v1/account/media";
const FANSLY_API_MEDIA_BUNDLE_URL: &str = "https://apiv2.fansly.com/api/v1/account/media/bundle";
const FANSLY_API_WALL_URL: &str = "https://apiv2.fansly.com/api/v1/wall/";
const FANSLY_API_MESSAGE_GROUPS_URL: &str = "https://apiv2.fansly.com/api/v1/group";
const FANSLY_API_GROUP_MESSAGES_URL: &str = "https://apiv2.fansly.com/api/v1/message";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FanslyConfig {
    pub enabled: bool,
    pub auth_token: String,
    pub ignore_lists: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Fansly {
    conf: Arc<FanslyConfig>,
    http_client: ApiClient,
}

impl Fansly {
    pub async fn new(
        fansly_conf: Arc<FanslyConfig>,
        api_conf: Arc<ApiClientConfig>,
    ) -> AsyncResult<Fansly> {
        if !fansly_conf.enabled {
            return Err(Box::new(GathererErrors::NotEnabled {
                name: String::from("Fansly"),
            }));
        };

        println!("Initializing Fansly...");
        let s = Self {
            http_client: ApiClient::new(api_conf),
            conf: fansly_conf,
        };

        match s.validate_auth_token().await {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_accounts_by_ids(
        &self,
        account_ids: &[String],
    ) -> AsyncResult<responses::AccountsResponse> {
        let endpoint = format!(
            "{}?ids={}",
            FANSLY_API_USER_ACCOUNT_URL,
            account_ids.join(",")
        );
        Ok(self.http_client.get(&endpoint, None).await?)
    }

    pub async fn get_media_by_ids(&self, media_ids: &[String]) -> AsyncResult<Vec<structs::Media>> {
        let mut returned_media = Vec::new();
        if media_ids.is_empty() {
            return Ok(Vec::new())
        }
        info!("Attempting to get {} media files", media_ids.len());
        for chunks_ids in media_ids.chunks(100) {
            let endpoint = format!("{}?ids={}", FANSLY_API_MEDIA_URL, chunks_ids.join(","));
            let media = self
                .http_client
                .get::<responses::MediaResponse>(&endpoint, self.get_default_headers())
                .await;
            match media {
                Ok(mut response) => {
                    debug!("Media Response {:#?}\n", response.response);
                    returned_media.append(&mut response.response);
                    // Some(response.response)
                }
                Err(err) => {
                    error!("Failed to download media info due to {}", err);
                    // None
                }
            }
        }
        Ok(returned_media)
    }

    pub async fn get_media_bundles_by_ids(
        &self,
        bundle_ids: &[String],
    ) -> AsyncResult<Vec<structs::MediaBundle>> {
        let endpoint = format!(
            "{}?ids={}",
            FANSLY_API_MEDIA_BUNDLE_URL,
            bundle_ids.join(",")
        );
        let bundles = self
            .http_client
            .get::<responses::MediaBundleResponse>(&endpoint, self.get_default_headers())
            .await;
        match bundles {
            Ok(bundles_response) => Ok(bundles_response.response),
            Err(bundles_err) => Err(bundles_err),
        }
    }

    pub async fn get_posts_by_user_id(
        &self,
        account_id: &'_ str,
    ) -> AsyncResult<Vec<responses::inner::Posts>> {
        let mut posts: Vec<responses::inner::Posts> = Vec::new();
        let mut more_pages = true;
        let mut offset: usize = 0;
        let mut before_post_id = String::new();

        while more_pages {
            let posts_url = format!(
                "{}/{}?before={}&after=0",
                FANSLY_API_TIMELINE_URL, account_id, before_post_id
            );
            debug!("Endpoint: [{}]", posts_url);
            let response = self
                .http_client
                .get::<responses::PostsResponse>(&posts_url, self.get_default_headers())
                .await;
            match response {
                Ok(post_response) => {
                    let inner_posts = &post_response.response.posts;
                    if inner_posts.len() < FANLSY_POST_LIMIT_COUNT {
                        more_pages = false
                    }
                    inner_posts.iter().for_each(|post| {
                        before_post_id = post.id.to_string();
                        // posts.push(post)
                    });
                    posts.push(post_response.response);
                    debug!("Got page {} of users posts", offset + 1);
                }
                Err(err) => return Err(err),
            }
            offset += 1;
        }

        Ok(posts)
    }

    pub async fn get_account_subscriptions(&self) -> AsyncResult<Vec<Subscription>> {
        let subs = self
            .http_client
            .get::<responses::SubscriptionResponse>(FANSLY_API_SUBS_URL, self.get_default_headers())
            .await;
        debug!("Subs response: {:?}", subs);
        match subs {
            Ok(resp) => {
                if resp.success {
                    let sub_account_ids: Vec<String> = resp
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
                    debug!("Subscription ids: {:?}", sub_account_ids);
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
                            error!("Failed to gather user accounts: {:?}", user_account_err);
                            Err(user_account_err)
                        }
                    }
                } else {
                    error!("Response from Fansly failed, {:?}", resp);
                    debug!("{:#?}", resp);
                    Err(Box::new(GathererErrors::NoSubscriptionsFound {
                        gatherer: self.name().to_string(),
                        data: format!("{:?}", resp.response),
                    }))
                }
            }
            Err(resp_err) => {
                error!("Response from  {:#?}", resp_err);
                Err(resp_err)
            }
        }
    }

    pub async fn get_messages_groups(&self) -> AsyncResult<Vec<structs::MessageGroup>> {
        let all_groups_resp: AsyncResult<responses::MessageGroupsResponse> = self
            .http_client
            .get(FANSLY_API_MESSAGE_GROUPS_URL, self.get_default_headers())
            .await;
        match all_groups_resp {
            Ok(resp) => Ok(resp.response.groups),
            Err(groups_err) => Err(format!("{:?}", groups_err).into()),
        }
    }

    pub async fn get_all_messages_from_group(
        &self,
        group_id: &'_ str,
    ) -> AsyncResult<Vec<Message>> {
        let endpoint = format!(
            "{}?groupId={}&limit={}",
            FANSLY_API_GROUP_MESSAGES_URL, &group_id, 50
        );
        // let mut messages: Vec<Message> = Vec::new();
        // while more_pages {
        let resp_res: AsyncResult<responses::GroupMessagesResponse> = self
            .http_client
            .get(&endpoint, self.get_default_headers())
            .await;

        match resp_res {
            Ok(resp) => {
                debug!("Response for thread {}. {:#?}", group_id, resp.response);
                Ok(resp.response.messages)
            }
            Err(message_err) => Err(format!(
                "Failed to get messages from group {}. {:?}",
                group_id, message_err,
            )
            .into()),
        }
        // Ok(Vec::new())
    }

    pub async fn validate_auth_token(&self) -> AsyncResult<&Fansly> {
        if self.conf.auth_token.is_empty() {
            return Err(Box::new(GathererErrors::InvalidCredentials {
                name: "Fansly".into(),
                msg: "Cannot be used without an auth token.".into(),
            }));
        };
        let resp: responses::StatusResponse = self
            .http_client
            .post(
                FANSLY_API_STATUS_URL,
                self.get_default_headers(),
                Some([("statusId", 1)]),
            )
            .await?;
        debug!("{:?}", resp);
        Ok(self)
    }

    fn get_default_headers(&self) -> Option<HashMap<&'_ str, &'_ str>> {
        let mut hm = HashMap::new();
        hm.insert("Authorization", &self.conf.auth_token[..]);
        Some(hm)
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
                    Some(Subscription {
                        id: info.id.to_string(),
                        name: SubscriptionName {
                            username: info.username.to_string(),
                            display_name: info.display_name.to_owned(),
                        },
                        plan: sub.subscription_tier_name.to_string(),
                        started: Some(Utc.timestamp_millis(sub.created_at).into()),
                        renewal_date: Some(Utc.timestamp_millis(sub.renew_date).into()),
                        rewewal_price: sub.price.into(),
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
