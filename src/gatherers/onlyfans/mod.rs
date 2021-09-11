mod responses;
mod structs;

use crate::http::Result as HttpResult;
use crate::{
    config::Config,
    gatherers::{
        errors::GathererErrors,
        onlyfans::{responses::ValidationResponse, structs::DynamicRule},
        Gatherer,
    },
    http::ApiClient,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// self.subscriptions = f"https://onlyfans.com/api2/v2/subscriptions/subscribes?limit={global_limit}&offset={global_offset}&type=active"
// self.lists = f"https://onlyfans.com/api2/v2/lists?limit=100&offset=0"
// self.lists_users = f"https://onlyfans.com/api2/v2/lists/{identifier}/users?limit={global_limit}&offset={global_offset}&query="
// self.list_chats = f"https://onlyfans.com/api2/v2/chats?limit={global_limit}&offset={global_offset}&order=desc"
// self.post_by_id = f"https://onlyfans.com/api2/v2/posts/{identifier}"
// self.message_by_id = f"https://onlyfans.com/api2/v2/chats/{identifier}/messages?limit=10&offset=0&firstId={identifier2}&order=desc&skip_users=all&skip_users_dups=1"
// self.search_chat = f"https://onlyfans.com/api2/v2/chats/{identifier}/messages/search?query={text}"
// self.message_api = f"https://onlyfans.com/api2/v2/chats/{identifier}/messages?limit={global_limit}&offset={global_offset}&order=desc"
// self.search_messages = f"https://onlyfans.com/api2/v2/chats/{identifier}?limit=10&offset=0&filter=&order=activity&query={text}"
// self.mass_messages_api = f"https://onlyfans.com/api2/v2/messages/queue/stats?limit=100&offset=0&format=infinite"
// self.stories_api = f"https://onlyfans.com/api2/v2/users/{identifier}/stories?limit=100&offset=0&order=desc"
// self.list_highlights = f"https://onlyfans.com/api2/v2/users/{identifier}/stories/highlights?limit=100&offset=0&order=desc"
// self.highlight = f"https://onlyfans.com/api2/v2/stories/highlights/{identifier}"
// self.post_api = f"https://onlyfans.com/api2/v2/users/{identifier}/posts?limit={global_limit}&offset={global_offset}&order=publish_date_desc&skip_users_dups=0"
// self.archived_posts = f"https://onlyfans.com/api2/v2/users/{identifier}/posts/archived?limit={global_limit}&offset={global_offset}&order=publish_date_desc"
// self.archived_stories = f"https://onlyfans.com/api2/v2/stories/archive/?limit=100&offset=0&order=publish_date_desc"
// self.paid_api = f"https://onlyfans.com/api2/v2/posts/paid?{global_limit}&offset={global_offset}"
// self.pay = f"https://onlyfans.com/api2/v2/payments/pay"
// self.subscribe = f"https://onlyfans.com/api2/v2/users/{identifier}/subscribe"
// self.like = f"https://onlyfans.com/api2/v2/{identifier}/{identifier2}/like"
// self.favorite = f"https://onlyfans.com/api2/v2/{identifier}/{identifier2}/favorites/{identifier3}"
// self.transactions = (
//     f"https://onlyfans.com/api2/v2/payments/all/transactions?limit=10&offset=0"
// )

const ONLYFANS_ME_URL: &str = "https://onlyfans.com/api2/v2/users/me";
const ONLYFANS_USERS_URL: &str = "https://onlyfans.com/api2/v2/users/";
const ONLYFANS_SUBS_URL: &str = "https://onlyfans.com/api2/v2/subscriptions/subscribes";
const ONLYFANS_MFA_URL: &str = "https://onlyfans.com/api2/v2/users/otp/check";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OnlyFansConfig {
    pub(crate) enabled: bool,
    pub(crate) session_token: String,
    pub(crate) user_agent: String,
    pub(crate) app_token: String,
    pub(crate) user_id: String,
    pub(crate) cookie: String,
    pub(crate) ignore_lists: Vec<String>,
}

#[derive(Debug)]
pub struct OnlyFans {
    config: OnlyFansConfig,
    http_client: ApiClient,
}

impl OnlyFans {
    pub fn new(conf: &'_ Config) -> super::Result<Self> {
        let of_conf = &conf.only_fans;
        if !of_conf.enabled {
            return Err(GathererErrors::NotEnabled {
                name: String::from("OnlyFans"),
            });
        }

        let s = Self {
            config: conf.only_fans.clone(),
            http_client: ApiClient::new(conf),
        };

        match s.valididate_token() {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn get_default_headers(&self, _rules: &[DynamicRule]) -> HashMap<&str, &str> {
        let mut h = HashMap::new();
        h.insert("authorization", &self.config.session_token[..]);
        h.insert("user-agent", &self.config.user_agent[..]);
        h.insert("app-token", &self.config.app_token[..]);
        h
    }

    fn valididate_token(&self) -> super::Result<()> {
        let rules: Vec<DynamicRule> = Vec::new();
        // let headers = self.get_default_headers(&rules);
        let resp: HttpResult<ValidationResponse> = self
            .http_client
            .get(ONLYFANS_ME_URL, Some(self.get_default_headers(&rules)));
        match resp {
            Ok(ok) => {
                log::debug!("Validation Response: {:?}\n---------------------------", ok);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn get_subscriptions(&self) -> Vec<super::Subscription> {
        Vec::new()
    }
}

#[async_trait::async_trait]
impl Gatherer for OnlyFans {
    fn gather_subscriptions(&self) -> super::Result<Vec<super::Subscription>> {
        todo!()
    }

    fn gather_posts(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Post>> {
        todo!()
    }

    fn gather_messages(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Message>> {
        todo!()
    }

    fn gather_stories(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Story>> {
        todo!()
    }

    fn name(&self) -> &'static str {
        "onlyfans"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
