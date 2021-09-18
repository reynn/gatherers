mod errors;

use std::{fmt::Display, path::PathBuf};

use crate::{
    downloaders::{Downloadable, DownloaderConfig},
    AsyncResult, Result,
};
use chrono::prelude::*;
pub use errors::GathererErrors;
use tabled::Tabled;

#[async_trait::async_trait]
pub trait Gatherer: std::fmt::Debug + Sync + Send {
    async fn gather_subscriptions(&self) -> AsyncResult<Vec<Subscription>>;
    async fn gather_media_from_bundles(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>>;
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

#[derive(Debug, Default, Tabled)]
pub struct Post {
    #[header(hidden = true)]
    pub id: String,
    pub title: String,
    pub content: String,
    #[header(hidden = true)]
    pub media: Vec<Media>,
    pub paid: bool,
}

#[derive(Debug, Default, Tabled)]
pub struct Message {}

//  This may need to get abstracted out into a multiple types of subs
#[derive(Debug, Default, Tabled)]
pub struct Subscription {
    #[header(hidden = true)]
    pub id: String,
    #[header("Name")]
    pub name: SubscriptionName,
    #[header("Tier")]
    pub plan: String,
    #[header(hidden = true)]
    pub started: DateTime,
    #[header(hidden = true)]
    pub renewal_date: DateTime,
    #[header(hidden = true)]
    pub rewewal_price: SubscriptionCost,
    #[header(hidden = true)]
    pub ends_at: DateTime,
    #[header("Videos")]
    pub video_count: i32,
    #[header("Images")]
    pub image_count: i32,
    #[header("Bundles")]
    pub bundle_count: i32,
}

#[derive(Debug, Default)]
pub struct SubscriptionName {
    pub username: String,
    pub display_name: Option<String>,
}

impl Display for SubscriptionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self.display_name {
            Some(display_name) => &display_name[..],
            None => &self.username[..],
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Default, Tabled)]
pub struct Media {
    pub filename: String,
    pub paid: bool,
    pub mime_type: String,
    #[header(hidden = true)]
    pub url: String,
}

#[derive(Debug, Default, Tabled)]
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
