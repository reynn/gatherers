use serde::{Deserialize, Serialize};
// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountMedia {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    // #[serde(rename = "previewId")]
    // pub preview_id: Option<serde_json::Value>,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
    pub price: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<i64>,
    pub deleted: bool,
    pub media: Option<Avatar>,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
    pub preview: Option<Avatar>,
    #[serde(rename = "accountMediaIds")]
    pub account_media_ids: Option<Vec<String>>,
    #[serde(rename = "bundleContent")]
    pub bundle_content: Option<Vec<BundleContent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleContent {
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub pos: i64,
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
    #[serde(rename = "variantHash")]
    pub variant_hash: VariantHash,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "locationId")]
    pub location_id: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantHash {}

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
pub struct AccountElement {
    pub id: String,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub flags: i64,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "followCount")]
    pub follow_count: i64,
    #[serde(rename = "subscriberCount")]
    pub subscriber_count: i64,
    #[serde(rename = "timelineStats")]
    pub timeline_stats: TimelineStats,
    #[serde(rename = "statusId")]
    pub status_id: i64,
    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: i64,
    pub walls: Vec<Wall>,
    #[serde(rename = "profileAccessFlags")]
    pub profile_access_flags: i64,
    #[serde(rename = "profileFlags")]
    pub profile_flags: i64,
    pub about: String,
    pub location: String,
    #[serde(rename = "pinnedPosts")]
    pub pinned_posts: Vec<PinnedPost>,
    pub following: bool,
    pub subscribed: bool,
    pub subscription: Subscription,
    #[serde(rename = "subscriptionTiers")]
    pub subscription_tiers: Vec<SubscriptionTier>,
    pub banner: Avatar,
    pub avatar: Avatar,
    #[serde(rename = "postLikes")]
    pub post_likes: i64,
    #[serde(rename = "accountMediaLikes")]
    pub account_media_likes: i64,
    #[serde(rename = "profileAccess")]
    pub profile_access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleAccountPermissionFlags {
    pub flags: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinnedPost {
    #[serde(rename = "postId")]
    pub post_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub pos: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
    #[serde(rename = "subscriptionTierName")]
    pub subscription_tier_name: String,
    #[serde(rename = "subscriptionTierColor")]
    pub subscription_tier_color: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "promoId")]
    pub promo_id: String,
    #[serde(rename = "giftCodeId")]
    pub gift_code_id: Option<String>,
    pub status: i64,
    pub price: i64,
    #[serde(rename = "renewPrice")]
    pub renew_price: i64,
    #[serde(rename = "renewCorrelationId")]
    pub renew_correlation_id: String,
    #[serde(rename = "autoRenew")]
    pub auto_renew: i64,
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i64,
    pub duration: i64,
    #[serde(rename = "renewDate")]
    pub renew_date: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
    #[serde(rename = "promoPrice")]
    pub promo_price: Option<i64>,
    #[serde(rename = "promoDuration")]
    pub promo_duration: Option<i64>,
    #[serde(rename = "promoStatus")]
    pub promo_status: Option<i64>,
    #[serde(rename = "promoStartsAt")]
    pub promo_starts_at: Option<i64>,
    #[serde(rename = "promoEndsAt")]
    pub promo_ends_at: Option<i64>,
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
    pub uses: i64,
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
    pub uses: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineStats {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "imageCount")]
    pub image_count: i64,
    #[serde(rename = "videoCount")]
    pub video_count: i64,
    #[serde(rename = "bundleCount")]
    pub bundle_count: i64,
    #[serde(rename = "bundleImageCount")]
    pub bundle_image_count: i64,
    #[serde(rename = "bundleVideoCount")]
    pub bundle_video_count: i64,
    #[serde(rename = "fetchedAt")]
    pub fetched_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wall {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    pub description: String,
    pub metadata: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub content: String,
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: Option<i64>,
    #[serde(rename = "inReplyToRoot")]
    pub in_reply_to_root: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<i64>,
    pub attachments: Vec<Attachment>,
    #[serde(rename = "likeCount")]
    pub like_count: i64,
    #[serde(rename = "mediaLikeCount")]
    pub media_like_count: i64,
    #[serde(rename = "totalTipAmount")]
    pub total_tip_amount: i64,
    #[serde(rename = "attachmentTipAmount")]
    pub attachment_tip_amount: i64,
    #[serde(rename = "accountMentions")]
    pub account_mentions: Vec<Option<String>>,
    #[serde(rename = "replyCount")]
    pub reply_count: Option<i64>,
    pub liked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub pos: String,
    #[serde(rename = "contentType")]
    pub content_type: i64,
    #[serde(rename = "contentId")]
    pub content_id: String,
}
