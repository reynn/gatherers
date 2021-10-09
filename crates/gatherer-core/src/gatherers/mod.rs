mod errors;
mod r#type;

pub use self::{errors::GathererErrors, r#type::GatherType};
use crate::{
    downloaders::{Downloadable, Downloader},
    AsyncResult, Result,
};
use async_channel::Sender;
use chrono::prelude::*;
use futures::Future;
use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};
use strum::IntoEnumIterator;
use tracing::{debug, error, info};

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

#[derive(Debug, Clone, Copy)]
pub struct RunLimits {
    pub media: usize,
    pub subscriptions: usize,
}

pub struct GathererInfo {
    base_path: PathBuf,
    gather_type: GatherType,
    gatherer: Arc<dyn Gatherer>,
    subscription: Subscription,
    downloader: Sender<Downloadable>,
    name: String,
}

async fn run_gatherer(info: GathererInfo) -> AsyncResult<()> {
    let name = info.name;
    let gather_type = info.gather_type;
    let sub = info.subscription;
    info!(
        "{:>10}: Starting to gather {:8} for {:^20}",
        name, gather_type, &sub.name.username
    );
    let all_media = match gather_type {
        GatherType::Posts => info.gatherer.gather_media_from_posts(&sub),
        GatherType::Messages => info.gatherer.gather_media_from_messages(&sub),
        GatherType::Bundles => info.gatherer.gather_media_from_bundles(&sub),
        GatherType::Stories => info.gatherer.gather_media_from_stories(&sub),
    }
    .await;

    match all_media {
        Ok(medias) => {
            let sub_path = info.base_path.join(&sub.name.username.to_ascii_lowercase());

            info!(
                "{:>10}: Completed gathering {:8} for {:^20}. Found [{:^4}] items",
                name,
                gather_type,
                sub.name.username,
                medias.len()
            );

            for media in medias.iter() {
                let downloadable_path =
                    info.base_path
                        .clone()
                        .join(if media.paid { "paid" } else { "free" });
                // let item = Downloadable::from_media_with_path(media, downloadable_path);
                match info
                    .downloader
                    .try_send(Downloadable::from_media_with_path(media, downloadable_path))
                {
                    Ok(_) => {
                        debug!("{:>12}: Sent item to download queue", name)
                    }
                    Err(send_err) => {
                        error!("{:>12}: Failed to send to queue. {:?}", name, send_err)
                    }
                }
            }
            Ok(())
        }
        Err(gather_err) => Err(format!(
            "{}: Failed to gather {}. Error: {:?}",
            name, gather_type, gather_err
        )
        .into()),
    }
}

pub async fn run_gatherer_for_all(
    gatherer: Arc<dyn Gatherer>,
    base_path: impl Into<PathBuf>,
    download_tx: Sender<Downloadable>,
    limits: RunLimits,
) -> AsyncResult<()> {
    let gatherer_name = gatherer.name();
    let sub_result = gatherer.gather_subscriptions().await;
    let mut subs_tasks = Vec::new();
    match sub_result {
        Ok(subscriptions) => {
            // subs_tasks.push(async move {
            println!(
                "{}: Found {} subscriptions",
                gatherer_name,
                subscriptions.len()
            );
            let base_path: PathBuf = base_path.into();
            let base_path = base_path.join(gatherer.name().to_ascii_lowercase());
            let subscriptions = if limits.subscriptions == 0 {
                subscriptions
            } else {
                debug!(
                    "{:>10}: Limiting to only {} subscriptions",
                    gatherer_name, limits.subscriptions
                );
                subscriptions
                    .into_iter()
                    .take(limits.subscriptions)
                    .collect()
            };

            for sub in subscriptions.iter() {
                // for gather_type in GatherType::iter() {
                    let info = GathererInfo {
                        base_path: base_path.clone().join(&sub.name.username),
                        gather_type: GatherType::Stories,
                        gatherer: gatherer.clone(),
                        subscription: sub.clone(),
                        downloader: download_tx.clone(),
                        name: gatherer_name.into(),
                    };
                    subs_tasks.push(run_gatherer(info));
                // }
            }
        }
        Err(sub_err) => return Err(sub_err),
    }

    // subs.tasks.
    futures::future::join_all(subs_tasks).await;
    info!(
        "{}: Completed gathering everything for all subs",
        gatherer_name
    );
    Ok(())
}

#[derive(Debug, Default)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub media: Vec<Media>,
    pub paid: bool,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Post(id={}; title={:?})", self.id, self.title)
    }
}

#[derive(Debug, Default)]
pub struct Message {
    pub from: SubscriptionName,
    pub to: SubscriptionName,
    pub message: String,
    pub attached_media: Vec<Media>,
}

//  This may need to get abstracted out into a multiple types of subs
#[derive(Debug, Default)]
pub struct Subscription {
    pub id: String,
    pub name: SubscriptionName,
    pub plan: String,
    pub started: Option<DateTime>,
    pub renewal_date: Option<DateTime>,
    pub rewewal_price: SubscriptionCost,
    pub ends_at: Option<DateTime>,
    pub video_count: i32,
    pub image_count: i32,
    pub bundle_count: i32,
}

impl Display for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Subscription(id={}; user_name={})",
            self.id, self.name.username
        )
    }
}

impl Clone for Subscription {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            plan: self.plan.clone(),
            started: None,
            renewal_date: None,
            rewewal_price: self.rewewal_price.clone(),
            ends_at: None,
            video_count: self.video_count,
            image_count: self.image_count,
            bundle_count: self.bundle_count,
        }
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Default)]
pub struct Media {
    pub file_name: String,
    pub paid: bool,
    pub mime_type: String,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct Story {
    pub id: String,
    pub content_text: Option<String>,
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
            Some(dt) => write!(f, "{}", dt.format("%Y-%m-%d %H:%M:%S")),
            None => write!(f, "No Date Available"),
        }
    }
}

#[derive(Debug, Clone, Default)]
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
