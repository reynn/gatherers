use {
    reqwest::cookie::Jar,
    serde::{Deserialize, Serialize},
    std::{str::FromStr, sync::Arc},
};

pub use reqwest::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientConfig<'a> {
    pub base_url: Option<&'a str>,
    pub cookies: Option<&'a str>,
}

pub fn new(cfg: ClientConfig) -> Client {
    if let Some(cookie) = cfg.cookies {
        let base_url = cfg
            .base_url
            .expect("`base_url` must be provided to client config if cookies are set")
            .parse::<Url>()
            .unwrap();
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(cookie, &base_url);
        ClientBuilder::new()
            .cookie_provider(Arc::new(cookie_jar))
            .build()
            .expect("Unable to build general reqwest client")
    } else {
        Client::default()
    }
}
