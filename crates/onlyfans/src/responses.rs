use super::structs::*;
use serde::{Deserialize, Serialize};

pub(super) type SubscriptionResponse = Vec<Subscription>;
pub(super) type MeResponse = Me;
pub(super) type DynamicRuleResponse = DynamicRule;
pub(super) type PostsResponse = PostsResponseInner;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsResponseInner {
    pub list: Vec<crate::structs::Post>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    pub counters: Option<Counters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counters {
    #[serde(rename = "audiosCount")]
    pub audios_count: i64,
    #[serde(rename = "photosCount")]
    pub photos_count: i64,
    #[serde(rename = "videosCount")]
    pub videos_count: i64,
    #[serde(rename = "mediasCount")]
    pub medias_count: i64,
    #[serde(rename = "postsCount")]
    pub posts_count: i64,
    #[serde(rename = "archivedPostsCount")]
    pub archived_posts_count: i64,
}
