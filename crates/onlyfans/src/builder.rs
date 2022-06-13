use gatherer_core::http::{Method, Request, RequestBuilder, Response};
use std::future::Future;
use url::Url;
use {
    crate::{constants, responses, structs, OnlyFans, OnlyFansConfig},
    gatherer_core::{http::Client, Result},
};

pub(crate) struct OnlyFansBuilder {
    config: OnlyFansConfig,
    dynamic_rule: Option<structs::DynamicRule>,
    http_client: Option<Client>,
}

impl OnlyFansBuilder {
    pub fn new(config: OnlyFansConfig) -> Self {
        Self {
            config,
            dynamic_rule: None,
            http_client: None,
        }
    }
}

impl OnlyFansBuilder {
    pub async fn build(self) -> Result<OnlyFans> {
        let config = self.config;
        let dynamic_rule = self.dynamic_rule.unwrap();
        let http_client = self.http_client.as_ref().unwrap();

        // Handle init call
        let mut init_headers =
            crate::generate_request_headers(&config, constants::INIT_URL, &dynamic_rule);

        init_headers.remove("user-id");

        async {
            let req = http_client
                .request(Method::GET, constants::INIT_URL.parse::<Url>().unwrap())
                .headers(init_headers)
                .build()
                .unwrap();
            let init_response = http_client.execute(req).await;
            // let init_response = http_client
            //     .get(constants::INIT_URL, Some(init_headers))
            //     .await;

            match init_response {
                Ok(success) => {
                    let success_headers = success.headers().get("set-cookie").unwrap();

                    log::debug!("'set-cookie' header: {:?}", success_headers)
                }
                Err(init_failed) => log::debug!("OnlyFans failed to init: {:?}", init_failed),
            };
        }
        .await;

        // Check the constants::ME_URL endpoint to ensure proper config
        // Returns a ready to use OnlyFans gatherer
        let me_headers = crate::generate_request_headers(&config, constants::ME_URL, &dynamic_rule);

        let curr_user: responses::MeResponse = match http_client
            .execute(
                http_client
                    .request(Method::GET, constants::ME_URL.parse::<Url>().unwrap())
                    .headers(me_headers)
                    .build()
                    .unwrap(),
            )
            .await
        {
            Ok(resp) => match resp.json().await {
                Ok(me_json) => me_json,
                Err(e) => {
                    eyre::bail!(e);
                }
            },
            Err(resp_err) => {
                eyre::bail!(resp_err)
            }
        };
        //
        // let curr_user: responses::MeResponse =
        //     match http_client.get(constants::ME_URL, Some(me_headers)).await {
        //         Ok(me) => me.as_json().await?,
        //         Err(me_err) => return Err(me_err),
        //     };

        Ok(OnlyFans {
            config,
            dynamic_rule,
            http_client: http_client.clone(),
            authed_user: curr_user,
        })
    }

    pub fn add_http_client(&mut self, client: Client) -> &mut Self {
        self.http_client = Some(client);
        self
    }

    pub fn with_dynamic_rule(&mut self, dr: structs::DynamicRule) -> &mut Self {
        self.dynamic_rule = Some(dr);
        self
    }
}
