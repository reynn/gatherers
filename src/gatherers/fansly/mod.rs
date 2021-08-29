mod responses;
mod structs;

use super::GathererErrors;
use crate::{
    config::Config,
    gatherers::{
        fansly::responses::{AccountResponse, StatusResponse},
        Gatherer,
    },
};
use reqwest::{
    blocking::{Client, Request, RequestBuilder},
    header::{self, HeaderValue},
    Method, Url,
};
use serde::{Deserialize, Serialize};

const FANSLY_API_VALIDATION_URL: &str = "https://apiv2.fansly.com/api/v1/status";
const FANSLY_API_USER_ACCOUNT_URL: &str = "https://apiv2.fansly.com/api/v1/account";
const FANSLY_API_SUBS_URL: &str = "https://apiv2.fansly.com/api/v1/subscriptions";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FanslyConfig {
    pub(crate) enabled: bool,
    pub(crate) auth_token: String,
}

#[derive(Debug, Clone)]
pub struct Fansly {
    conf: FanslyConfig,
    http_client: Client,
}

impl Fansly {
    pub fn new(conf: &'_ FanslyConfig) -> super::Result<Self> {
        if !conf.enabled {
            return Err(GathererErrors::NotEnabled {
                name: String::from("Fansly"),
            });
        };

        log::debug!("Initializing Fansly...");
        let s = Self {
            http_client: Client::new(),
            conf: conf.clone(),
        };

        if let Err(validate_err) = s.validate_auth_token() {
            log::error!("Failed to validate the Fansly token. {}", &validate_err);
        };

        Ok(s)
    }

    fn api<T: for<'de> serde::Deserialize<'de>, U: Into<Url>>(
        &self,
        method: Method,
        endpoint: U,
    ) -> super::Result<T> {
        let mut req = Request::new(method, endpoint.into());

        let headers = req.headers_mut();
        headers.append(
            header::AUTHORIZATION,
            HeaderValue::from_str(&self.conf.auth_token)?,
        );
        let api_endpoint = String::from(req.url().as_str());
        let resp = self.http_client.execute(req)?;

        let status = &resp.status();
        if !status.is_success() {
            return Err(GathererErrors::HttpError {
                status: *status,
                response_body: Some(resp.text().unwrap_or_default()),
            });
        };

        let body = &resp.text()?;
        log::debug!("Response from {:?}.\n{}", api_endpoint, body);
        Ok(serde_json::from_str(body)?)
    }

    fn get_user_accounts(&self, account_ids: &[&'_ str]) -> super::Result<Vec<structs::Account>> {
        match Url::parse(FANSLY_API_USER_ACCOUNT_URL) {
            Ok(mut account_url) => {
                account_url.set_query(Some(&format!("ids={}", account_ids.join(","))));
                let accounts: AccountResponse = self.api(Method::GET, account_url)?;
                Ok(accounts.response)
            }
            Err(e) => Err(GathererErrors::UrlError(e)),
        }
    }

    fn validate_auth_token(&self) -> super::Result<&Self> {
        let status: StatusResponse =
            self.api(Method::POST, Url::parse(FANSLY_API_VALIDATION_URL)?)?;
        log::debug!("Fansly Auth Status: {:?}", status);
        if status.success {
            Ok(self)
        } else {
            Err(GathererErrors::InvalidCredentials {
                name: self.name().into(),
                msg: "Bad credentials".into(),
            })
        }
    }
}

// #[async_trait::async_trait]
impl Gatherer for Fansly {
    fn name(&self) -> &'static str {
        "fansly"
    }

    fn gather_subscriptions<'a>(&self) -> super::Result<Vec<super::Subscription>> {
        log::info!("Gathering Fansly subscriptions");

        let sub_reponse: responses::SubscriptionResponse =
            self.api(Method::GET, Url::parse(FANSLY_API_SUBS_URL)?)?;

        if sub_reponse.success {
            let subs = &sub_reponse.response.subscriptions;
            let account_ids: Vec<&str> = subs.iter().map(|sub| sub.account_id.as_str()).collect();
            let sub_accounts = self.get_user_accounts(&account_ids)?;
            log::debug!("Found accounts: {:?}", &sub_accounts);
            // let fansly_subs = sub_reponse
            //     .response
            //     .subscriptions
            //     .into_iter()
            //     .map(|sub| {
            //         let sub = &sub;
            //         match self.get_user_accounts(&[&sub.account_id]) {
            //             Ok(account) => {
            //                 let account = account.get(0).unwrap();
            //                 log::debug!("AccountID: {}. {:?}", &account.username, &account);
            //                 super::Subscription {
            //                     username: account.username.to_owned(),
            //                     id: String::from(&sub.account_id),
            //                     plan: None,
            //                     expires: sub.ends_at.to_string(),
            //                     started: sub.created_at.to_string(),
            //                 }
            //             }
            //             Err(err) => {
            //                 log::error!("Failed to get account, {:?}", err);
            //                 super::Subscription {
            //                     username: "".into(),
            //                     id: String::from(&sub.account_id),
            //                     plan: None,
            //                     expires: "".into(),
            //                     started: "".into(),
            //                 }
            //             }
            //         }
            //     })
            //     .collect();
            todo!();
            // Ok(fansly_subs)
        } else {
            log::error!("Failure in response: {:?}", sub_reponse);
            Ok(Vec::new())
        }
    }

    fn gather_posts(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Post>> {
        Err(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().into(),
            feature: "posts".into(),
        })
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

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }
}
