mod responses;
mod structs;

use std::collections::HashMap;

use super::GathererErrors;
use crate::{config::Config, gatherers::{Gatherer, Media, Subscription, fansly::{self, responses::{AccountsResponse, PostsResponse, StatusResponse, SubscriptionResponse}, structs::FanslySub}}, http::ApiClient};
use serde::{Deserialize, Serialize};

const FANSLY_API_STATUS_URL: &str = "https://apiv2.fansly.com/api/v1/status";
const FANSLY_API_USER_ACCOUNT_URL: &str = "https://apiv2.fansly.com/api/v1/account";
const FANSLY_API_SUBS_URL: &str = "https://apiv2.fansly.com/api/v1/subscriptions";
const FANSLY_API_TIMELINE_URL: &str = "https://apiv2.fansly.com/api/v1/timeline";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FanslyConfig {
    pub(crate) enabled: bool,
    pub(crate) auth_token: String,
}

#[derive(Debug, Clone)]
pub struct Fansly {
    conf: FanslyConfig,
    http_client: ApiClient,
}

impl Fansly {
    pub fn new(conf: &'_ Config) -> super::Result<Self> {
        let fansly_conf = &conf.fansly;
        if !fansly_conf.enabled {
            return Err(GathererErrors::NotEnabled {
                name: String::from("Fansly"),
            });
        };

        log::debug!("Initializing Fansly...");
        let s = Self {
            http_client: ApiClient::new(conf),
            conf: fansly_conf.clone(),
        };

        match s.validate_auth_token() {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn get_user_accounts(&self, account_ids: &[&'_ str]) -> super::Result<AccountsResponse> {
        let endpoint = format!(
            "{}?ids={}",
            FANSLY_API_USER_ACCOUNT_URL,
            account_ids.join(",")
        );
        Ok(self.http_client.get(&endpoint, None)?)
    }

    fn get_account_subscriptions(&self) -> super::Result<Vec<super::Subscription>> {
        let subs = self
            .http_client
            .get::<SubscriptionResponse>(FANSLY_API_SUBS_URL, self.get_default_headers());

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
                    match self.get_user_accounts(&sub_account_ids) {
                        Ok(account_infos) => {
                            let account_infos = account_infos.response;
                            log::info!(
                                "Found {} accounts, for the {} subscribers",
                                account_infos.len(),
                                account_infos.len()
                            );
                            let subscriptions = resp.response.subscriptions;
                            Ok(subscriptions
                                .into_iter()
                                .map(|sub| {
                                    let account_info = account_infos
                                        .iter()
                                        .find(|info| info.id == sub.account_id)
                                        .unwrap();
                                    let username = match &account_info.display_name {
                                        Some(disp_name) => disp_name.to_string(),
                                        None => account_info.username.to_string(),
                                    };
                                    let rewewal_price = Some(
                                        sub.renew_price
                                            .to_string()
                                            .parse::<f64>()
                                            .unwrap_or_default(),
                                    );
                                    let mut new_sub: super::Subscription = sub.into();
                                    new_sub.username = username;
                                    new_sub
                                })
                                .collect())
                        }
                        Err(user_account_err) => {
                            log::error!("Failed to gather user accounts: {:?}", user_account_err);
                            Err(user_account_err)
                        }
                    }
                } else {
                    Err(GathererErrors::NoSubscriptionsFound {
                        gatherer: self.name().to_string(),
                        data: format!("{:?}", resp.response),
                    })
                }
            }
            Err(resp_err) => Err(resp_err.into()),
        }
    }

    fn validate_auth_token(&self) -> super::Result<&Self> {
        let res: StatusResponse = self.http_client.post(
            FANSLY_API_STATUS_URL,
            self.get_default_headers(),
            Some([("statusId", 1)]),
        )?;
        Ok(self)
    }

    fn get_default_headers(&self) -> Option<HashMap<&'_ str, &'_ str>> {
        let mut hm = HashMap::new();
        hm.insert("Authorization", &self.conf.auth_token[..]);
        Some(hm)
    }
}

// #[async_trait::async_trait]
impl Gatherer for Fansly {
    fn gather_subscriptions<'a>(&self) -> super::Result<Vec<super::Subscription>> {
        self.get_account_subscriptions()
    }

    fn gather_posts(&self, sub: &'_ super::Subscription) -> super::Result<Vec<super::Post>> {
        let posts_url = format!("{}/{}", FANSLY_API_TIMELINE_URL, sub.id);
        let response = self
            .http_client
            .get::<PostsResponse>(&posts_url, self.get_default_headers());
        match response {
            Ok(post_response) => {
                log::info!("Response: {:?}", post_response);

                Ok(post_response
                    .response
                    .posts
                    .iter()
                    .map(|post| super::Post {
                        id: String::from(&post.id),
                        title: None,
                        content: Some(String::from(&post.content)),
                        media: post
                            .attachments
                            .iter()
                            .map(|f| super::Media {})
                            .collect::<Vec<Media>>(),
                        paid: false,
                    })
                    .collect())
            }
            Err(err) => {
                log::error!("Failed to gather posts for {}. {}", sub.username, err);
                Err(err.into())
            }
        }
    }

    fn gather_messages(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Message>> {
        Err(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().into(),
            feature: "messages".into(),
        })
    }

    fn gather_stories(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Story>> {
        Err(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().into(),
            feature: "stories".into(),
        })
    }

    fn name(&self) -> &'static str {
        "fansly"
    }

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }
}
