mod gatherer;
mod responses;
mod structs;

pub use self::gatherer::*;
use gatherer_core::{
    gatherers::{Gatherer, GathererErrors, Subscription},
    http::{ApiClient, ApiClientConfig},
    AsyncResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const FANLSY_POST_LIMIT_COUNT: usize = 10;
const FANSLY_API_STATUS_URL: &str = "https://apiv2.fansly.com/api/v1/status";
const FANSLY_API_USER_ACCOUNT_URL: &str = "https://apiv2.fansly.com/api/v1/account";
const FANSLY_API_SUBS_URL: &str = "https://apiv2.fansly.com/api/v1/subscriptions";
const FANSLY_API_TIMELINE_URL: &str = "https://apiv2.fansly.com/api/v1/timeline";
const FANSLY_API_MEDIA_URL: &str = "https://apiv2.fansly.com/api/v1/account/media";
const FANSLY_API_MEDIA_BUNDLE_URL: &str = "https://apiv2.fansly.com/api/v1/account/media/bundle";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FanslyConfig {
    pub(crate) enabled: bool,
    pub(crate) auth_token: String,
    pub(crate) ignore_lists: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Fansly<'a> {
    conf: &'a FanslyConfig,
    http_client: ApiClient<'a>,
}

impl<'a> Fansly<'a> {
    pub async fn new(
        fansly_conf: &'a FanslyConfig,
        api_conf: &'a ApiClientConfig,
    ) -> AsyncResult<Fansly<'a>> {
        if !fansly_conf.enabled {
            return Err(Box::new(GathererErrors::NotEnabled {
                name: String::from("Fansly"),
            }));
        };

        tracing::debug!("Initializing Fansly...");
        let s = Self {
            http_client: ApiClient::new(api_conf),
            conf: fansly_conf,
        };

        match s.validate_auth_token().await {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    async fn get_user_accounts_by_ids(
        &self,
        account_ids: &[&'_ str],
    ) -> AsyncResult<responses::AccountsResponse> {
        let endpoint = format!(
            "{}?ids={}",
            FANSLY_API_USER_ACCOUNT_URL,
            account_ids.join(",")
        );
        Ok(self.http_client.get(&endpoint, None).await?)
    }

    async fn get_media_by_ids(&self, media_ids: &[&'_ str]) -> AsyncResult<Vec<structs::Media>> {
        let endpoint = format!("{}?ids={}", FANSLY_API_MEDIA_URL, media_ids.join(","));
        let media = self
            .http_client
            .get::<responses::MediaResponse>(&endpoint, self.get_default_headers())
            .await;

        match media {
            Ok(response) => tracing::info!("Media Response {:#?}\n", response.response),
            Err(err) => tracing::error!("Failed to download media info due to {}", err),
        };

        Ok(Vec::new())
        // match media {
        //     Ok(media_info) => Ok(media_info
        //         .response
        //         .iter()
        //         .map(|info| super::Media { filename: (), url: () })
        //         .collect()),
        //     Err(err) => todo!(),
        // }
    }

    async fn get_posts_by_user_id(
        &self,
        account_id: &'_ str,
        offset: usize,
    ) -> AsyncResult<Vec<structs::Post>> {
        let mut posts: Vec<structs::Post> = Vec::new();
        let mut more_pages = true;
        let mut offset: usize = 0;
        let mut before_post_id = String::new();

        while more_pages {
            let posts_url = format!(
                "{}/{}?before={}&after=0",
                FANSLY_API_TIMELINE_URL, account_id, before_post_id
            );
            tracing::debug!("Endpoint: [{}]", posts_url);
            let response = self
                .http_client
                .get::<responses::PostsResponse>(&posts_url, self.get_default_headers())
                .await;
            match response {
                Ok(post_response) => {
                    let inner_posts = post_response.response.posts;
                    if inner_posts.len() < FANLSY_POST_LIMIT_COUNT {
                        more_pages = false
                    }
                    inner_posts.into_iter().for_each(|post| {
                        before_post_id = post.id.to_string();
                        posts.push(post)
                    });
                    tracing::debug!("Got page {} of users posts", offset + 1);
                }
                Err(err) => return Err(err),
            }
            offset += 1;
        }

        Ok(posts)
    }

    async fn get_account_subscriptions(&self) -> AsyncResult<Vec<Subscription>> {
        let subs = self
            .http_client
            .get::<responses::SubscriptionResponse>(FANSLY_API_SUBS_URL, self.get_default_headers())
            .await;

        match subs {
            Ok(resp) => {
                if resp.success {
                    let sub_account_ids: Vec<&str> = resp
                        .response
                        .subscriptions
                        .iter()
                        .map(|fan_sub| fan_sub.account_id.as_str())
                        .collect();
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
                            tracing::error!(
                                "Failed to gather user accounts: {:?}",
                                user_account_err
                            );
                            Err(user_account_err)
                        }
                    }
                } else {
                    Err(Box::new(GathererErrors::NoSubscriptionsFound {
                        gatherer: self.name().to_string(),
                        data: format!("{:?}", resp.response),
                    }))
                }
            }
            Err(resp_err) => Err(resp_err.into()),
        }
    }

    async fn validate_auth_token(&self) -> AsyncResult<&Fansly<'a>> {
        if self.conf.auth_token.is_empty() {
            return Err(Box::new(GathererErrors::InvalidCredentials {
                name: "Fansly".into(),
                msg: "Cannot be used without an auth token.".into(),
            }));
        };
        let res: responses::StatusResponse = self
            .http_client
            .post(
                FANSLY_API_STATUS_URL,
                self.get_default_headers(),
                Some([("statusId", 1)]),
            )
            .await?;
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
    Vec::new()
}
