mod errors;
#[macro_use]
mod macros;
mod cookies;
mod request;
mod response;

pub use self::{cookies::Cookie, errors::HttpErrors, request::*, response::Response};
use crate::Result;
use serde::{Deserialize, Serialize};
pub use serde_json::json;
use std::{collections::HashMap, convert::TryInto, str::FromStr};
use surf::{
    http::headers::{HeaderValue, COOKIE},
    Config,
};

pub type Url = surf::Url;
pub type Headers = HashMap<String, String>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientConfig {
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    client: surf::Client,
    cookies: Option<Headers>,
}

impl Client {
    pub fn new(cfg: ClientConfig) -> Self {
        let mut config = Config::default();
        if let Some(base) = cfg.base_url.clone() {
            let base = Url::parse(&base).unwrap();
            config = config.set_base_url(base);
        };
        let client: surf::Client = config
            .try_into()
            .expect("Failed to create a client from the base config");
        Self {
            client,
            cookies: None,
        }
    }

    http_request!(get);
    http_request_with_body!(delete);
    http_request_with_body!(post);
    http_request_with_body!(put);

    /// Execute a request via the underlying configured client, configures headers inc. cookies as needed
    ///
    ///
    async fn execute(&self, headers: Option<Headers>, req: Request) -> Result<Response> {
        let mut req = req;
        if let Some(headers) = headers {
            log::debug!("Adding {} additional headers ", headers.len());
            log::trace!("Headers: {:?}", &headers);
            // req.add_headers(headers);
            req.add_headers(
                headers
                    .into_iter()
                    .filter_map(|(n, v)| {
                        let n = match HeaderName::from_str(&n) {
                            Ok(n) => n,
                            Err(h_err) => {
                                log::debug!("Invalid header name {}: {:?}", n, h_err);
                                return None;
                            }
                        };
                        let v = match HeaderValue::from_str(&v) {
                            Ok(v) => v,
                            Err(v_err) => {
                                log::debug!("Invalid header values {}. {:?}", v, v_err);
                                return None;
                            }
                        };
                        Some((n, v))
                    })
                    .collect::<HashMap<_, _>>(),
            )
            .await?;
        }
        if let Some(cookies) = self.cookies.clone() {
            let cookies = cookies
                .into_iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>();
            log::debug!("Adding {} items to the {} header", cookies.len(), COOKIE);
            log::trace!("Cookies: {:?}", &cookies);
            req.add_header(COOKIE.as_str(), &cookies.join("; "))
        };
        log::debug!("Making a {} request to {}", req.method(), req.url());
        let headers = req.header_names();
        log::trace!("Headers: {:?}", headers);
        let req: surf::Request = req.into();
        let resp = self.client.send(req).await?;
        Ok(Response::from_surf(resp))
    }

    pub fn set_cookies(&mut self, cookies: Headers) {
        self.cookies = Some(cookies);
    }
}
