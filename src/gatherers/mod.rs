mod errors;
pub mod fansly;
pub mod onlyfans;

use self::{errors::GathererErrors, fansly::Fansly};
use crate::{config::Config, gatherers::onlyfans::OnlyFans};
use chrono::prelude::*;
use std::{fmt::Display, sync::Arc};
use tabled::Tabled;

pub type Result<'ge, T, E = GathererErrors> = std::result::Result<T, E>;

#[async_trait::async_trait]
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

pub fn validated_gatherers(conf: &'_ Config) -> Result<'static, Vec<Arc<dyn Gatherer>>> {
    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();

    match Fansly::new(conf) {
        Ok(fansly_client) => gatherers.push(Arc::new(fansly_client)),
        Err(e) => log::error!("Failed to initialize the Fansly gatherer: {}", e),
    }

    match OnlyFans::new(conf) {
        Ok(onlyfans_client) => gatherers.push(Arc::new(onlyfans_client)),
        Err(e) => log::error!("Failed to initialize the OnlyFans gatherer: {}", e),
    };

    Ok(gatherers)
}

#[derive(Debug, Default)]
pub struct DateTime(Option<chrono::DateTime<Utc>>);

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(f: chrono::DateTime<Utc>) -> Self {
        Self(Some(f))
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(dt) => write!(f, "{}", dt.to_rfc2822()),
            None => write!(f, "No date"),
        }
    }
}

#[derive(Debug, Default)]
pub struct SubscriptionCost(Option<i64>);

impl std::fmt::Display for SubscriptionCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(cost) => write!(f, "{}", cost),
            None => write!(f, "Not paying (currently)"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Post {
    id: String,
    title: Option<String>,
    content: Option<String>,
    media: Vec<Media>,
    paid: bool,
}

#[derive(Debug, Default)]
pub struct Message {}

//  This may need to get abstracted out into a multiple types of subs
#[derive(Debug, Tabled)]
pub struct Subscription {
    #[header(hidden)]
    pub id: String,
    pub username: String,
    pub plan: String,
    // #[header("")]
    pub started: DateTime,
    // #[header("")]
    pub renewal_date: DateTime,
    pub rewewal_price: SubscriptionCost,
    // #[header("")]
    pub ends_at: DateTime,
}

#[derive(Debug, Default)]
pub struct Media {}

#[derive(Debug, Default)]
pub struct Story {}
