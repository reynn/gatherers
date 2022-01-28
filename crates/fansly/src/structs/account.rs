use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub flags: i64,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "followCount")]
    pub follow_count: i64,
    #[serde(rename = "subscriberCount")]
    pub subscriber_count: i64,
    #[serde(rename = "timelineStats")]
    pub timeline_stats: Option<TimelineStats>,
    #[serde(rename = "statusId")]
    pub status_id: i64,
    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: i64,
    #[serde(rename = "profileAccessFlags")]
    pub profile_access_flags: i64,
    #[serde(rename = "profileFlags")]
    pub profile_flags: i64,
    // pub about: String,
    pub location: Option<String>,
    #[serde(rename = "subscriptionTiers")]
    pub subscription_tiers: Option<Vec<SubscriptionTier>>,
    pub avatar: Option<super::Media>,
    pub banner: Option<super::Media>,
    #[serde(rename = "postLikes")]
    pub post_likes: i64,
    #[serde(rename = "accountMediaLikes")]
    pub account_media_likes: i64,
    #[serde(rename = "profileAccess")]
    pub profile_access: bool,
    pub walls: Option<Vec<Wall>>,
    #[serde(rename = "pinnedPosts")]
    pub pinned_posts: Option<Vec<PinnedPost>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowedAccount {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub status: i64,
    #[serde(rename = "storyCount")]
    pub story_count: i64,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub id: String,
    #[serde(rename = "type")]
    pub avatar_type: i64,
    pub status: i64,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub mimetype: String,
    pub filename: String,
    pub width: i64,
    pub height: i64,
    pub metadata: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub variants: Vec<Variant>,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "locationId")]
    pub location_id: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    #[serde(rename = "type")]
    pub variant_type: i64,
    pub status: i64,
    pub mimetype: String,
    pub filename: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinnedPost {
    #[serde(rename = "postId")]
    pub post_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub pos: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionTier {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    pub color: String,
    pub pos: i64,
    pub price: i64,
    #[serde(rename = "subscriptionBenefits")]
    pub subscription_benefits: Vec<String>,
    pub plans: Vec<Plan>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub status: i64,
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i64,
    pub price: i64,
    #[serde(rename = "useAmounts")]
    pub use_amounts: i64,
    pub promos: Vec<Promo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Promo {
    pub id: String,
    pub status: i64,
    pub price: i64,
    pub duration: i64,
    #[serde(rename = "maxUses")]
    pub max_uses: i64,
    #[serde(rename = "maxUsesBefore")]
    pub max_uses_before: Option<i64>,
    #[serde(rename = "newSubscribersOnly")]
    pub new_subscribers_only: i64,
    #[serde(rename = "startsAt")]
    pub starts_at: i64,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineStats {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "imageCount")]
    pub image_count: i32,
    #[serde(rename = "videoCount")]
    pub video_count: i32,
    #[serde(rename = "bundleCount")]
    pub bundle_count: i32,
    #[serde(rename = "bundleImageCount")]
    pub bundle_image_count: i32,
    #[serde(rename = "bundleVideoCount")]
    pub bundle_video_count: i32,
    #[serde(rename = "fetchedAt")]
    pub fetched_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wall {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    pub description: Option<String>,
    pub metadata: String,
}
