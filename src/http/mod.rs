pub mod errors;

use crate::config::Config;
use reqwest::{
    blocking::{Client, Request},
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap, str::FromStr};

pub type Result<T, E = errors::HttpErrors> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiClientConfig {}

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new(conf: &'_ Config) -> Self {
        // let client = reqwest::Client::new();
        Self {
            client: Client::new(),
        }
    }

    pub fn set_cookies(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
    ) -> Result<T> {
        let mut req = self.client.get(endpoint);
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                let h_k = match HeaderName::from_str(key) {
                    Ok(hn) => hn,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderName {
                            source: e,
                            value: String::from(key),
                        })
                    }
                };
                let h_v = match HeaderValue::from_str(value) {
                    Ok(hk) => hk,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderValue {
                            source: e,
                            value: String::from(value),
                        })
                    }
                };
                header_map.insert(h_k, h_v);
            }
            req = req.headers(header_map);
        }
        self.execute(req.build()?)
    }

    pub fn put<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> Result<T> {
        let mut req = self.client.put(endpoint);
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                let h_k = match HeaderName::from_str(key) {
                    Ok(hn) => hn,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderName {
                            source: e,
                            value: String::from(key),
                        })
                    }
                };
                let h_v = match HeaderValue::from_str(value) {
                    Ok(hk) => hk,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderValue {
                            source: e,
                            value: String::from(value),
                        })
                    }
                };
                header_map.insert(h_k, h_v);
            }
            req = req.headers(header_map);
        }
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?)
    }

    pub fn delete<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> Result<T> {
        let mut req = self.client.delete(endpoint);
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                let h_k = match HeaderName::from_str(key) {
                    Ok(hn) => hn,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderName {
                            source: e,
                            value: String::from(key),
                        })
                    }
                };
                let h_v = match HeaderValue::from_str(value) {
                    Ok(hk) => hk,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderValue {
                            source: e,
                            value: String::from(value),
                        })
                    }
                };
                header_map.insert(h_k, h_v);
            }
            req = req.headers(header_map);
        }
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?)
    }

    pub fn post<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> Result<T> {
        let mut req = self.client.post(endpoint);
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, src) in headers {
                let hn = match HeaderName::from_str(key) {
                    Ok(hn) => hn,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderName {
                            source: e,
                            value: String::from(key),
                        })
                    }
                };
                let hv = match HeaderValue::from_str(src) {
                    Ok(hv) => hv,
                    Err(e) => {
                        return Err(errors::HttpErrors::InvalidHeaderValue {
                            source: e,
                            value: String::from(src),
                        })
                    }
                };
                header_map.insert(hn, hv);
            }
            req = req.headers(header_map);
        }
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?)
    }

    fn execute<T: for<'de> serde::Deserialize<'de>>(&self, req: Request) -> Result<T> {
        let url = format!("{}", req.url());
        match self.client.execute(req) {
            Ok(resp) => {
                // if we get 100->399 it should be good to go
                // if response.status_code < 400 {
                let body_text = resp.text()?;
                log::debug!("{} response: \n\n{}", url, body_text);
                match serde_json::from_str(&body_text[..]) {
                    Ok(json_resp) => Ok(json_resp),
                    Err(json_err) => Err(errors::HttpErrors::JsonError(json_err)),
                }
                // } else {
                //     let body_text = resp.text()?;
                //     log::error!("{}", body_text);
                //     Err(HttpErrors::)
                // }
            }
            Err(exec_err) => todo!(),
        }
    }

    fn parse_url(url: &'_ str) -> Result<url::Url> {
        Ok(Url::parse(url)?)
    }
}
