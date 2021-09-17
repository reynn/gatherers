use super::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

pub type AccountsResponse = FanslyResponse<Vec<Account>>;
pub type SubscriptionResponse = FanslyResponse<SubsInner>;
pub type StatusResponse = FanslyResponse<StatusInner>;
pub type PostsResponse = FanslyResponse<PostsInner>;
pub type MediaResponse = FanslyResponse<Vec<Media>>;
pub type MediaBundleResponse = FanslyResponse<Vec<MediaBundle>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubsInner {
    #[serde(rename = "subscriptionPlans")]
    pub subscription_plans: Vec<SubscriptionPlan>,
    pub subscriptions: Vec<Subscription>,
    pub stats: SubscriptionStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusInner {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "statusId")]
    pub status_id: Option<i8>,
    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsInner {
    pub posts: Vec<Post>,
    // #[serde(rename = "aggregatedPosts")]
    // pub aggregated_posts: Vec<String>,
    #[serde(rename = "accountMediaBundles")]
    pub account_media_bundles: Vec<Media>,
    #[serde(rename = "accountMedia")]
    pub account_media: Vec<Media>,
    // pub accounts: Vec<Account>,
    pub tips: Vec<String>,
    // #[serde(rename = "tipGoals")]
    // pub tip_goals: Option<Vec<TipGoals>>,
    pub stories: Option<Vec<String>>,
}
