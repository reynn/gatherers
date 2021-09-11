use super::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

pub(super) type AccountsResponse = FanslyResponse<Vec<Account>>;
pub(super) type SubscriptionResponse = FanslyResponse<SubsInner>;
pub(super) type StatusResponse = FanslyResponse<StatusInner>;
pub(super) type PostsResponse = FanslyResponse<PostsInner>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub(super) struct SubsInner {
    #[serde(rename = "subscriptionPlans")]
    pub subscription_plans: Vec<Plan>,
    pub subscriptions: Vec<FanslySub>,
    pub stats: SubscriptionStats,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PostsInner {
    pub posts: Vec<Post>,
    #[serde(rename = "aggregatedPosts")]
    pub aggregated_posts: String,
    #[serde(rename = "accountMediaBundles")]
    pub account_media_bundles: String,
    #[serde(rename = "accountMedia")]
    pub account_media: String,
    pub accounts: String,
    pub tips: Vec<String>,
    #[serde(rename = "tipGoals")]
    pub tip_goals: Vec<String>,
    pub stories: Vec<String>,
    #[serde(rename = "timelineReadPermissionFlags")]
    pub timeline_read_permission_flags: String,
    #[serde(rename = "accountTimelineReadPermissionFlags")]
    pub account_timeline_read_permission_flags: String,
}
