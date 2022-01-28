use std::{fmt::Display, path::PathBuf, sync::Arc};

use crate::{
    downloaders::Downloadable,
    gatherers::{GatherType, Gatherer},
};
use async_channel::Sender;
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub struct RunLimits {
    pub media: Option<usize>,
    pub subscriptions: Option<usize>,
}

pub struct GathererInfo {
    pub base_path: PathBuf,
    pub gather_type: GatherType,
    pub gatherer: Arc<dyn Gatherer>,
    pub subscription: Subscription,
    pub downloader: Sender<Downloadable>,
    pub name: String,
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub struct Message {
    pub from: SubscriptionName,
    pub to: SubscriptionName,
    pub message: String,
    pub attached_media: Vec<Media>,
}

//  This may need to get abstracted out into a multiple types of subs
#[derive(Debug, Clone, Default)]
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

// impl Clone for Subscription {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id.clone(),
//             name: self.name.clone(),
//             plan: self.plan.clone(),
//             started: None,
//             renewal_date: None,
//             rewewal_price: self.rewewal_price.clone(),
//             ends_at: None,
//             video_count: self.video_count,
//             image_count: self.image_count,
//             bundle_count: self.bundle_count,
//         }
//     }
// }

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

#[derive(Debug, Clone, Default)]
pub struct Media {
    pub file_name: String,
    pub paid: bool,
    pub mime_type: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Story {
    pub id: String,
    pub content_text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub total_amount: f64,
    pub user_name: String,
    pub date: DateTime,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DateTime(pub Option<chrono::DateTime<Utc>>);

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
pub struct SubscriptionCost(pub Option<f64>);

impl std::fmt::Display for SubscriptionCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(cost) => write!(f, "{}", cost),
            None => write!(f, "*Free*"),
        }
    }
}

impl From<f64> for SubscriptionCost {
    fn from(s: f64) -> Self {
        Self(Some(s))
    }
}
