use super::structs::*;
use serde::{Deserialize, Serialize};

pub(super) type SubscriptionResponse = Vec<Subscription>;
pub(super) type MeResponse = Me;
pub(super) type DynamicRuleResponse = DynamicRule;
pub(super) type PostsResponse = ListResponse<Post>;
pub(super) type MessagesResponse = ListResponse<Message>;
pub(super) type StoriesResponse = Vec<Story>;
pub(super) type TransactionsResponse = Transactions;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub list: Vec<T>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Transactions {
    pub list: Vec<Transaction>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    pub marker: i64,
    #[serde(rename = "nextMarker")]
    pub next_marker: Option<i64>,
}
