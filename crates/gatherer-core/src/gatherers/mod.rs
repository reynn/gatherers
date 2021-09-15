mod errors;

use crate::{config::Config, AsyncResult, Result};
use chrono::prelude::*;
pub use errors::GathererErrors;
use std::{fmt::Display, future, ops::Deref, sync::Arc};

#[async_trait::async_trait]
pub trait Gatherer: std::fmt::Debug + Sync + Send {
    async fn gather_subscriptions(&self) -> AsyncResult<Vec<Subscription>>;
    async fn gather_media_from_posts(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>>;
    async fn gather_media_from_messages(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>>;
    async fn gather_media_from_stories(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>>;
    fn name(&self) -> &'static str;
    fn is_enabled(&self) -> bool {
        false
    }
}

pub async fn run_gatherer(gatherer: &dyn Gatherer) -> Result<()> {
    Ok(())
}

#[derive(Debug, Default)]
pub struct Post {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub media: Vec<Media>,
    pub paid: bool,
}

#[derive(Debug, Default)]
pub struct Message {}

//  This may need to get abstracted out into a multiple types of subs
#[derive(Debug)]
pub struct Subscription {
    pub id: String,
    pub username: String,
    pub plan: String,
    pub started: DateTime,
    pub renewal_date: DateTime,
    pub rewewal_price: SubscriptionCost,
    pub ends_at: DateTime,
}

#[derive(Debug, Default)]
pub struct Media {
    filename: String,
    url: String,
}

#[derive(Debug, Default)]
pub struct Story {}

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
            Some(dt) => write!(f, "{}", dt.format("%Y-%m-%d %H:%M:%S")),
            None => write!(f, "No Date Available"),
        }
    }
}

#[derive(Debug, Default)]
pub struct SubscriptionCost(pub Option<i64>);

impl std::fmt::Display for SubscriptionCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(cost) => write!(f, "{}", cost),
            None => write!(f, "*Free*"),
        }
    }
}

impl From<i64> for SubscriptionCost {
    fn from(s: i64) -> Self {
        Self(Some(s))
    }
}
