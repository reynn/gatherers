use std::collections::HashMap;

pub use surf::http::{
    headers::{HeaderName, HeaderValue, Headers},
    Method,
};

use crate::http::Response;

#[derive(Debug, Clone)]
pub struct Request(surf::Request);

impl Request {
    pub async fn add_headers(
        &mut self,
        headers: HashMap<HeaderName, HeaderValue>,
    ) -> crate::Result<()> {
        for (n, v) in headers.into_iter() {
            self.0.append_header(n, v);
        }
        Ok(())
    }
    pub fn add_header(&mut self, name: &'_ str, value: &'_ str) {
        self.0.append_header(name, value)
    }
    pub fn method(&self) -> Method {
        self.0.method()
    }
    pub fn url(&self) -> &'_ str {
        self.0.url().as_str()
    }
    pub fn header_names(&self) -> Vec<&'_ str> {
        self.0
            .header_names()
            .into_iter()
            .map(|name| name.as_str())
            .collect()
    }
}

impl From<Request> for surf::Request {
    fn from(req: Request) -> Self {
        req.0
    }
}

impl From<surf::Request> for Request {
    fn from(req: surf::Request) -> Self {
        Self(req)
    }
}
