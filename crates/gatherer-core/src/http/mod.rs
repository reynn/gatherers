mod errors;

use crate::{AsyncResult, Result};
pub use errors::HttpErrors;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url, {Client, Request, RequestBuilder},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tracing::{debug, info};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiClientConfig {}

#[derive(Debug, Clone)]
pub struct ApiClient {
    conf: Arc<ApiClientConfig>,
    client: Client,
}

impl ApiClient {
    pub fn new(conf: Arc<ApiClientConfig>) -> Self {
        Self {
            client: Client::new(),
            conf,
        }
    }

    pub fn set_cookies(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
    ) -> AsyncResult<T> {
        let mut req = self.client.get(endpoint);
        if let Ok(h_map) = process_headers(&req, headers) {
            req = req.headers(h_map);
        };
        self.execute(req.build()?).await
    }

    pub async fn put<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.put(endpoint);
        if let Ok(h_map) = process_headers(&req, headers) {
            req = req.headers(h_map);
        };
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?).await
    }

    pub async fn delete<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.delete(endpoint);
        if let Ok(h_map) = process_headers(&req, headers) {
            req = req.headers(h_map);
        };
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?).await
    }

    pub async fn post<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.post(endpoint);
        if let Ok(h_map) = process_headers(&req, headers) {
            req = req.headers(h_map);
        };
        if let Some(body) = body {
            req = req.json(&body);
        }
        self.execute(req.build()?).await
    }

    async fn execute<T: for<'de> serde::Deserialize<'de>>(&self, req: Request) -> AsyncResult<T> {
        let url = format!("{}", req.url());
        debug!("Making a {} request to {}", req.method(), &url,);
        match self.client.execute(req).await {
            Ok(resp) => {
                debug!("{:#?}", resp);
                // if we get 100->399 it should be good to go
                let status = resp.status();
                let body_text = resp.text().await?;
                if status.is_success() {
                    debug!("{} response: \n\n{}", url, body_text);
                    match serde_json::from_str(&body_text[..]) {
                        Ok(json_resp) => Ok(json_resp),
                        Err(json_err) => {
                            info!("Failed to create proper response body from JSON returned from {}. {:?}", url, json_err);
                            Err(Box::new(errors::HttpErrors::JsonError(json_err)))
                        }
                    }
                } else {
                    debug!("{}", body_text);
                    Err(Box::new(errors::HttpErrors::BadStatus {
                        status_code: status.as_u16(),
                        body: body_text,
                    }))
                }
            }
            Err(exec_err) => Err(Box::new(exec_err)),
        }
    }

    fn parse_url(url: &'_ str) -> Result<url::Url> {
        Ok(Url::parse(url)?)
    }
}

fn process_headers(
    req: &RequestBuilder,
    headers: Option<HashMap<&'_ str, &'_ str>>,
) -> Result<HeaderMap> {
    let mut header_map = HeaderMap::new();
    match headers {
        Some(headers) => {
            for (key, src) in headers {
                let hn = match HeaderName::from_str(key) {
                    Ok(hn) => hn,
                    Err(e) => {
                        return Err(Box::new(errors::HttpErrors::InvalidHeaderName {
                            source: e,
                            value: String::from(key),
                        }))
                    }
                };
                let hv = match HeaderValue::from_str(src) {
                    Ok(hv) => hv,
                    Err(e) => {
                        return Err(Box::new(errors::HttpErrors::InvalidHeaderValue {
                            source: e,
                            value: String::from(src),
                        }))
                    }
                };
                header_map.insert(hn, hv);
            }
            Ok(header_map)
        }
        None => Ok(header_map),
    }
}
