use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct SubscriptionStats {
    pub total: i32,
    #[serde(rename = "totalActive")]
    pub total_active: i32,
    #[serde(rename = "totalExpired")]
    pub total_expired: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Account {
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
    pub permissions: Permissions,
    #[serde(rename = "timelineStats")]
    pub timeline_stats: TimelineStats,
    #[serde(rename = "statusId")]
    pub status_id: i64,
    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: i64,
    pub following: bool,
    #[serde(rename = "postLikes")]
    pub post_likes: i64,
    #[serde(rename = "profileAccessFlags")]
    pub profile_access_flags: i64,
    pub about: String,
    pub location: String,
    #[serde(rename = "accountMediaLikes")]
    pub account_media_likes: i64,
    pub subscribed: bool,
    pub subscription: Subscription,
    #[serde(rename = "subscriptionTiers")]
    pub subscription_tiers: Vec<SubscriptionTier>,
    pub avatar: Avatar,
    pub banner: Avatar,
    #[serde(rename = "profileAccess")]
    pub profile_access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Avatar {
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
pub struct Permissions {
    #[serde(rename = "accountPermissionFlags")]
    pub(super) account_permission_flags: Option<AccountPermissionFlags>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub(super) id: Option<i64>,
    #[serde(rename = "accountId")]
    pub(super) account_id: Option<String>,
    pub(super) balance: Option<i64>,
    #[serde(rename = "type")]
    pub(super) earnings_wallet_type: Option<i64>,
    #[serde(rename = "walletVersion")]
    pub(super) wallet_version: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AccountMedia {
    pub access: bool,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountMediaIds")]
    pub account_media_ids: Option<Vec<String>>,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    #[serde(rename = "bundleContent")]
    pub bundle_content: Option<Vec<BundleContent>>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub deleted: bool,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<serde_json::Value>,
    pub id: String,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
    pub media: Option<Media>,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
    pub permissions: AccountMediaPermissions,
    pub preview: Option<Media>,
    #[serde(rename = "previewId")]
    pub preview_id: Option<serde_json::Value>,
    pub price: i64,
    pub purchased: bool,
    pub whitelisted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct BundleContent {
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub pos: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Media {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub filename: String,
    pub height: i64,
    pub id: String,
    pub locations: Vec<Location>,
    pub metadata: String,
    pub mimetype: Mimetype,
    pub status: Option<i64>,
    #[serde(rename = "type")]
    pub media_type: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    #[serde(rename = "variantHash")]
    pub variant_hash: VariantHash,
    pub variants: Vec<Variant>,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Location {
    pub location: String,
    #[serde(rename = "locationId")]
    pub location_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct VariantHash {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Variant {
    pub filename: String,
    pub height: i64,
    pub id: String,
    pub locations: Vec<Location>,
    pub mimetype: Mimetype,
    pub status: Option<i64>,
    #[serde(rename = "type")]
    pub variant_type: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AccountMediaPermissions {
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: AccountPermissionFlags,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: Vec<PermissionFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AccountPermissionFlags {
    pub flags: i64,
    pub metadata: AccountTimelineReadPermissionFlagsMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct PermissionFlag {
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub flags: i64,
    pub id: String,
    pub metadata: PermissionFlagMetadata,
    pub price: i64,
    #[serde(rename = "type")]
    pub permission_flag_type: i64,
    #[serde(rename = "validAfter")]
    pub valid_after: Option<serde_json::Value>,
    #[serde(rename = "validBefore")]
    pub valid_before: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct AccountPermissions {
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: PurpleAccountPermissionFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct PurpleAccountPermissionFlags {
    pub flags: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct PinnedPost {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub pos: i64,
    #[serde(rename = "postId")]
    pub post_id: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub(super) struct Subscription {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "autoRenew")]
    pub auto_renew: i64,
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub duration: i64,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
    #[serde(rename = "giftCodeId")]
    pub gift_code_id: Option<String>,
    pub id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub price: i64,
    #[serde(rename = "promoDuration")]
    pub promo_duration: Option<i64>,
    #[serde(rename = "promoEndsAt")]
    pub promo_ends_at: Option<i64>,
    #[serde(rename = "promoId")]
    pub promo_id: String,
    #[serde(rename = "promoPrice")]
    pub promo_price: Option<i64>,
    #[serde(rename = "promoStartsAt")]
    pub promo_starts_at: Option<i64>,
    #[serde(rename = "promoStatus")]
    pub promo_status: Option<i64>,
    #[serde(rename = "renewCorrelationId")]
    pub renew_correlation_id: String,
    #[serde(rename = "renewDate")]
    pub renew_date: i64,
    #[serde(rename = "renewPrice")]
    pub renew_price: i64,
    pub status: Option<i64>,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
    #[serde(rename = "subscriptionTierName")]
    pub subscription_tier_name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct SubscriptionTier {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub color: String,
    pub id: String,
    pub name: String,
    pub plans: Vec<Plan>,
    pub price: i64,
    #[serde(rename = "subscriptionBenefits")]
    pub subscription_benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Plan {
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i64,
    pub id: String,
    pub price: i64,
    pub promos: Vec<Promo>,
    pub status: Option<i64>,
    #[serde(rename = "useAmounts")]
    pub use_amounts: i64,
    pub uses: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Promo {
    pub duration: i64,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
    pub id: String,
    #[serde(rename = "maxUses")]
    pub max_uses: i64,
    #[serde(rename = "maxUsesBefore")]
    pub max_uses_before: Option<i64>,
    #[serde(rename = "newSubscribersOnly")]
    pub new_subscribers_only: i64,
    pub price: i64,
    #[serde(rename = "startsAt")]
    pub starts_at: i64,
    pub status: Option<i64>,
    pub uses: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TimelineStats {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "bundleCount")]
    pub bundle_count: i64,
    #[serde(rename = "bundleImageCount")]
    pub bundle_image_count: i64,
    #[serde(rename = "bundleVideoCount")]
    pub bundle_video_count: i64,
    #[serde(rename = "fetchedAt")]
    pub fetched_at: i64,
    #[serde(rename = "imageCount")]
    pub image_count: i64,
    #[serde(rename = "videoCount")]
    pub video_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Post {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountMentions")]
    pub account_mentions: Vec<Option<serde_json::Value>>,
    #[serde(rename = "accountTimelineReadPermissionFlags")]
    pub account_timeline_read_permission_flags: AccountPermissionFlags,
    pub attachments: Vec<Attachment>,
    #[serde(rename = "attachmentTipAmount")]
    pub attachment_tip_amount: i64,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub id: String,
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: Option<serde_json::Value>,
    #[serde(rename = "inReplyToRoot")]
    pub in_reply_to_root: Option<serde_json::Value>,
    #[serde(rename = "likeCount")]
    pub like_count: i64,
    pub liked: Option<bool>,
    #[serde(rename = "mediaLikeCount")]
    pub media_like_count: i64,
    #[serde(rename = "postReplyPermissionFlags")]
    pub post_reply_permission_flags: Option<Vec<PostReplyPermissionFlag>>,
    #[serde(rename = "replyCount")]
    pub reply_count: Option<i64>,
    #[serde(rename = "timelineReadPermissionFlags")]
    pub timeline_read_permission_flags: Vec<Option<serde_json::Value>>,
    #[serde(rename = "totalTipAmount")]
    pub total_tip_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Attachment {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "contentType")]
    pub content_type: i64,
    pub pos: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct PostReplyPermissionFlag {
    pub flags: i64,
    pub id: String,
    pub metadata: String,
    #[serde(rename = "postId")]
    pub post_id: String,
    #[serde(rename = "type")]
    pub post_reply_permission_flag_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TipGoal {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "currentAmount")]
    pub current_amount: i64,
    #[serde(rename = "currentPercentage")]
    pub current_percentage: i64,
    pub description: String,
    #[serde(rename = "goalAmount")]
    pub goal_amount: i64,
    #[serde(rename = "hideAmounts")]
    pub hide_amounts: i64,
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mimetype {
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "video/mp4")]
    VideoMp4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountTimelineReadPermissionFlagsMetadata {
    #[serde(rename = "{}")]
    Empty,
    #[serde(rename = "{\"4\":\"{\\\"subscriptionTierId\\\":\\\"227989260228100096\\\"}\"}")]
    The4SubscriptionTierId227989260228100096,
    #[serde(rename = "{\"4\":\"{\\\"subscriptionTierId\\\":\\\"285261438442807296\\\"}\"}")]
    The4SubscriptionTierId285261438442807296,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionFlagMetadata {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "{\"1\":\"{\\\"price\\\":3000}\"}")]
    The1Price3000,
    #[serde(rename = "{\"1\":\"{\\\"price\\\":5000}\"}")]
    The1Price5000,
    #[serde(rename = "{\"4\":\"{\\\"subscriptionTierId\\\":\\\"\\\"}\"}")]
    The4SubscriptionTierId,
}
