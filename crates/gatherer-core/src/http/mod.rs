mod errors;

use crate::{AsyncResult, Result};
pub use errors::HttpErrors;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use surf::{
    Url, {Client, Request, RequestBuilder},
};
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
        self.execute(headers, self.client.get(endpoint).build())
            .await
    }

    pub async fn put<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.put(endpoint);
        if let Some(body) = body {
            req = req.body_json(&body)?;
        }
        self.execute(headers, req.build()).await
    }

    pub async fn delete<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.delete(endpoint);
        if let Some(body) = body {
            req = req.body_json(&body)?;
        }
        self.execute(headers, req.build()).await
    }

    pub async fn post<T: for<'de> serde::Deserialize<'de>, B: Serialize>(
        &self,
        endpoint: &'_ str,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        body: Option<B>,
    ) -> AsyncResult<T> {
        let mut req = self.client.post(endpoint);
        // if let Some(headers) = headers {
        //     for (hn, hk) in headers.into_iter() {
        //     }
        // };
        // for (hn, hk) in
        // if let Ok(h_map) = process_headers(&req, headers) {
        //     req = req.headers(h_map);
        // };
        if let Some(body) = body {
            req = req.body_json(&body)?;
        }
        self.execute(headers, req.build()).await
    }

    async fn execute<T: for<'de> serde::Deserialize<'de>>(
        &self,
        headers: Option<HashMap<&'_ str, &'_ str>>,
        req: Request,
    ) -> AsyncResult<T> {
        let mut req = req;
        if let Some(headers) = headers {
            for (hk, hv) in headers.into_iter() {
                req.insert_header(hk, hv);
            }
        }
        debug!("Making a {} request to {}", req.method(), req.url());
        match self.client.send(req).await {
            Ok(mut resp) => {
                debug!("{:?}", resp);
                // if we get 100->399 it should be good to go
                let status = resp.status();
                let body_text = resp.body_string().await?;
                if status.is_success() {
                    debug!("Response: \n\n{}", body_text);
                    match serde_json::from_str(&body_text[..]) {
                        Ok(json_resp) => Ok(json_resp),
                        Err(json_err) => {
                            info!("Failed to create proper response body from JSON returned. {:?}", json_err);
                            Err(Box::new(errors::HttpErrors::JsonError(json_err)))
                        }
                    }
                } else {
                    debug!("{}", body_text);
                    Err(Box::new(errors::HttpErrors::BadStatus {
                        status_code: status,
                        body: body_text,
                    }))
                }
            }
            Err(exec_err) => Err(Box::new(HttpErrors::InternalHttpClientError(exec_err))),
        }
    }

    fn parse_url(url: &'_ str) -> Result<url::Url> {
        Ok(Url::parse(url)?)
    }
}
