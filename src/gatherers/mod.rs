mod errors;
pub mod fansly;
pub mod onlyfans;

use crate::config::Config;
use crate::gatherers::onlyfans::OnlyFans;

use self::errors::GathererErrors;
use self::fansly::Fansly;
use std::sync::Arc;

pub type Result<'ge, T, E = GathererErrors> = std::result::Result<T, E>;

pub trait Gatherer: std::fmt::Debug + Sync + Send {
    fn gather_subscriptions(&self) -> Result<Vec<Subscription>>;
    fn gather_posts(&self, _sub: &'_ Subscription) -> Result<Vec<Post>>;
    fn gather_messages(&self, _sub: &'_ Subscription) -> Result<Vec<Message>>;
    fn gather_stories(&self, _sub: &'_ Subscription) -> Result<Vec<Story>>;
    fn name(&self) -> &'static str;
    fn is_enabled(&self) -> bool {
        false
    }
}

pub fn validated_gatherers(conf: Config) -> Result<'static, Vec<Arc<dyn Gatherer>>> {
    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();

    let fansly_cfg = &conf.fansly;
    match Fansly::new(fansly_cfg) {
        Ok(fansly_client) => gatherers.push(Arc::new(fansly_client)),
        Err(e) => log::error!("Failed to initialize the Fansly gatherer: {}", e),
    }

    let onlyfan_cfg = &conf.only_fans;
    match OnlyFans::new(onlyfan_cfg) {
        Ok(onlyfans_client) => gatherers.push(Arc::new(onlyfans_client)),
        Err(e) => log::error!("Failed to initialize the OnlyFans gatherer: {}", e),
    };

    Ok(gatherers)
}

pub struct Post {
    id: String,
    title: Option<String>,
    content: Option<String>,
    media: Vec<Media>,
    paid: bool,
}

#[derive(Debug, Default)]
pub struct Message {}

#[derive(Debug, Default)]
pub struct Subscription {
    pub username: String,
    pub id: String,
    pub plan: Option<String>,
    pub expires: String,
    pub started: String,
}

#[derive(Debug, Default)]
pub struct Media {}

#[derive(Debug, Default)]
pub struct Story {}
