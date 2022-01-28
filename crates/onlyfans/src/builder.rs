use std::collections::HashMap;

use crate::{constants, responses, structs, OnlyFans, OnlyFansConfig};
use gatherer_core::{
    http::{Client, Cookie},
    Result,
};

pub(crate) struct OnlyFansBuilder {
    config: OnlyFansConfig,
    dynamic_rule: Option<structs::DynamicRule>,
    http_client: Option<Client>,
    cookie: Option<Cookie>,
}

impl OnlyFansBuilder {
    pub fn new(config: OnlyFansConfig) -> Self {
        Self {
            config,
            dynamic_rule: None,
            http_client: None,
            cookie: None,
        }
    }
}

impl OnlyFansBuilder {
    pub async fn build(self) -> Result<OnlyFans> {
        let config = self.config;
        let dynamic_rule = self.dynamic_rule.unwrap();
        let http_client = self.http_client.as_ref().unwrap();
        let cookie = Cookie::parse(&config.cookie).unwrap();

        log::debug!("Config cookie         : {:?}", cookie);
        log::debug!("Config cookie session : {:?}", cookie.get("sess"));
        log::debug!("Config cookie csrf    : {:?}", cookie.get("csrf"));

        // Handle init call
        let mut init_headers =
            crate::generate_request_headers(&config, constants::INIT_URL, &dynamic_rule);

        init_headers.remove("user-id");

        let init_success = async {
            let init_response = http_client
                .get(constants::INIT_URL, Some(init_headers))
                .await;

            match init_response {
                Ok(success) => {
                    let success_headers =
                        Cookie::parse(success.get_header("set-cookie").unwrap_or("")).unwrap();

                    log::debug!("'set-cookie' header: {:?}", success_headers)
                }
                Err(init_failed) => log::debug!("OnlyFans failed to init: {:?}", init_failed),
            };
        }
        .await;

        // Check the constants::ME_URL endpoint to ensure proper config
        // Returns a ready to use OnlyFans gatherer
        let me_headers = crate::generate_request_headers(&config, constants::ME_URL, &dynamic_rule);

        let curr_user: responses::MeResponse =
            match http_client.get(constants::ME_URL, Some(me_headers)).await {
                Ok(me) => me.as_json().await?,
                Err(me_err) => return Err(me_err),
            };

        Ok(OnlyFans {
            config,
            dynamic_rule,
            http_client: http_client.clone(),
            authed_user: curr_user,
            cookie: cookie.into(),
        })
    }

    pub fn add_http_client(&mut self, client: Client) -> &mut Self {
        self.http_client = Some(client);
        self
    }

    pub fn parse_cookie_string(&mut self) -> &mut Self {
        let cookie = String::from(&self.config.cookie);
        let cookie = Cookie::parse(&cookie).expect("OnlyFans: provided `cookie` is invalid");
        self.cookie = Some(cookie);
        self
    }

    pub fn with_dynamic_rule(&mut self, dr: structs::DynamicRule) -> &mut Self {
        self.dynamic_rule = Some(dr);
        self
    }
}
