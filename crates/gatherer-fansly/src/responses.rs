use super::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

pub(super) type AccountsResponse = FanslyResponse<Vec<Account>>;
pub(super) type SubscriptionResponse = FanslyResponse<SubsInner>;
pub(super) type StatusResponse = FanslyResponse<StatusInner>;
pub(super) type PostsResponse = FanslyResponse<PostsInner>;
pub(super) type MediaResponse = FanslyResponse<Vec<MediaInner>>;
pub(super) type MediaBundleResponse = FanslyResponse<MediaBundleInner>;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct SubsInner {
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
    // #[serde(rename = "accountMediaBundles")]
    // pub account_media_bundles: Vec<AccountMedia>,
    // #[serde(rename = "accountMedia")]
    // pub account_media: Vec<String>,
    // pub accounts: Vec<Account>,
    pub tips: Vec<String>,
    // #[serde(rename = "tipGoals")]
    // pub tip_goals: Option<Vec<TipGoals>>,
    pub stories: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaInner {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
    pub price: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
    pub deleted: bool,
    pub media: Media,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
    pub preview: Option<Media>,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaBundleInner {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    pub price: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
    pub deleted: bool,
    #[serde(rename = "accountMediaIds")]
    pub account_media_ids: Vec<String>,
    // #[serde(rename = "bundleContent")]
    // pub bundle_content: Vec<BundleContent>,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
}
