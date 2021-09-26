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
async fn run_gatherer(
    name: impl Into<String>,
    gatherer: &'_ Arc<dyn Gatherer>,
    gather_type: GatherType,
    base_path: &'_ Path,
    sub: &'_ Subscription,
) -> AsyncResult<(PathBuf, Vec<Media>)> {
    let name = name.into();
    info!(
        "{}: Starting to gather {} for {}",
        name, gather_type, sub.name
    );
    let all_media = match gather_type {
        GatherType::Posts => gatherer.gather_media_from_posts(sub),
        GatherType::Messages => gatherer.gather_media_from_messages(sub),
        GatherType::Bundles => gatherer.gather_media_from_bundles(sub),
        GatherType::Stories => gatherer.gather_media_from_stories(sub),
    }
    .await;

    match all_media {
        Ok(media) => {
            let sub_path = base_path.join(&sub.name.username.to_ascii_lowercase());
            info!(
                "{}: Completed gathering {} for {}. Found {} items",
                name,
                gather_type,
                sub.name,
                media.len()
            );
            Ok((sub_path, media))
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
    downloader: Sender<Downloadable>,
    base_path: impl Into<PathBuf>,
) -> AsyncResult<()> {
    let gatherer_name = gatherer.name();
    let sub_result = gatherer.gather_subscriptions().await;
    match sub_result {
        Ok(subscriptions) => {
            println!(
                "{}: Found {} subscriptions",
                gatherer_name,
                subscriptions.len()
            );
            let base_path = base_path.into();
            let base_path = base_path.join(gatherer.name().to_ascii_lowercase());
            // let gatherer = gatherer.clone();
            for sub in subscriptions.iter().take(4) {
                let mut sub_tasks = Vec::new();
                for gather_type in GatherType::iter() {
                    sub_tasks.push(run_gatherer(
                        gatherer_name,
                        &gatherer,
                        gather_type,
                        &base_path,
                        sub,
                    ));
                }

                let results = futures::future::join_all(sub_tasks).await;
                for result in results {
                    match result {
                        Ok((base_pah, medias)) => {
                            for media in medias.iter().take(20) {
                                let downloadable_path =
                                    base_pah
                                        .clone()
                                        .join(if media.paid { "paid" } else { "free" });
                                let item =
                                    Downloadable::from_media_with_path(media, downloadable_path);
                                match downloader.try_send(item) {
                                    Ok(_) => {
                                        debug!("{}: Sent item to download queue", gatherer_name)
                                    }
                                    Err(send_err) => {
                                        error!(
                                            "{}: Failed to send to queue. {:?}",
                                            gatherer_name, send_err
                                        )
                                    }
                                }
                            }
                        }
                        Err(gather_err) => {
                            error!("{:?}", gather_err);
                        }
                    }
                }
            }
        }
        Err(sub_err) => return Err(sub_err),
    }
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
