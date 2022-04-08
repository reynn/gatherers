use gatherer_core::gatherers::SubscriptionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub view: String,
    pub avatar: Option<String>,
    #[serde(rename = "avatarThumbs")]
    pub avatar_thumbs: Option<AvatarThumbs>,
    pub header: Option<String>,
    #[serde(rename = "headerSize")]
    pub header_size: Option<HeaderSize>,
    #[serde(rename = "headerThumbs")]
    pub header_thumbs: Option<HeaderThumbs>,
    pub id: i64,
    pub name: String,
    pub username: String,
    #[serde(rename = "canLookStory")]
    pub can_look_story: Option<bool>,
    #[serde(rename = "canCommentStory")]
    pub can_comment_story: Option<bool>,
    #[serde(rename = "hasNotViewedStory")]
    pub has_not_viewed_story: Option<bool>,
    #[serde(rename = "isVerified")]
    pub is_verified: Option<bool>,
    #[serde(rename = "canPayInternal")]
    pub can_pay_internal: Option<bool>,
    #[serde(rename = "hasScheduledStream")]
    pub has_scheduled_stream: Option<bool>,
    #[serde(rename = "hasStream")]
    pub has_stream: Option<bool>,
    #[serde(rename = "hasStories")]
    pub has_stories: Option<bool>,
    #[serde(rename = "tipsEnabled")]
    pub tips_enabled: Option<bool>,
    #[serde(rename = "tipsTextEnabled")]
    pub tips_text_enabled: Option<bool>,
    #[serde(rename = "tipsMin")]
    pub tips_min: i64,
    #[serde(rename = "tipsMax")]
    pub tips_max: i64,
    #[serde(rename = "canEarn")]
    pub can_earn: Option<bool>,
    #[serde(rename = "canAddSubscriber")]
    pub can_add_subscriber: Option<bool>,
    #[serde(rename = "subscribePrice")]
    pub subscribe_price: f64,
    // #[serde(rename = "subscriptionBundles")]
    // pub subscription_bundles: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "isPaywallRestriction")]
    pub is_paywall_restriction: Option<bool>,
    pub unprofitable: Option<bool>,
    #[serde(rename = "listsStates")]
    pub lists_states: Vec<ListsState>,
    #[serde(rename = "isMuted")]
    pub is_muted: Option<bool>,
    #[serde(rename = "isRestricted")]
    pub is_restricted: Option<bool>,
    #[serde(rename = "canRestrict")]
    pub can_restrict: Option<bool>,
    #[serde(rename = "subscribedBy")]
    pub subscribed_by: Option<bool>,
    #[serde(rename = "subscribedByExpire")]
    pub subscribed_by_expire: Option<bool>,
    #[serde(rename = "subscribedByExpireDate")]
    pub subscribed_by_expire_date: String,
    #[serde(rename = "subscribedByAutoprolong")]
    pub subscribed_by_autoprolong: Option<bool>,
    #[serde(rename = "subscribedIsExpiredNow")]
    pub subscribed_is_expired_now: bool,
    #[serde(rename = "currentSubscribePrice")]
    pub current_subscribe_price: Option<f64>,
    #[serde(rename = "subscribedOn")]
    pub subscribed_on: Option<bool>,
    #[serde(rename = "subscribedOnExpiredNow")]
    pub subscribed_on_expired_now: Option<bool>,
    #[serde(rename = "subscribedOnDuration")]
    pub subscribed_on_duration: Option<String>,
    #[serde(rename = "canReport")]
    pub can_report: Option<bool>,
    #[serde(rename = "canReceiveChatMessage")]
    pub can_receive_chat_message: Option<bool>,
    #[serde(rename = "hideChat")]
    pub hide_chat: Option<bool>,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<String>,
    #[serde(rename = "isPerformer")]
    pub is_performer: Option<bool>,
    #[serde(rename = "isRealPerformer")]
    pub is_real_performer: Option<bool>,
    #[serde(rename = "subscribedByData")]
    pub subscribed_by_data: SubscribedByData,
    #[serde(rename = "subscribedOnData")]
    pub subscribed_on_data: Option<SubscribedOnData>,
    #[serde(rename = "canTrialSend")]
    pub can_trial_send: Option<bool>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

impl From<Subscription> for gatherer_core::gatherers::Subscription {
    fn from(of_sub: Subscription) -> Self {
        Self {
            id: of_sub.id.to_string(),
            name: SubscriptionName {
                username: of_sub.username,
                display_name: Some(of_sub.name),
            },
            plan: "paid".into(),
            started: None,
            renewal_date: None,
            rewewal_price: of_sub.current_subscribe_price.unwrap_or(0.).into(),
            ends_at: None,
            video_count: 0,
            image_count: 0,
            bundle_count: 0,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DynamicRule {
    pub app_token: String,
    pub checksum_constant: i32,
    pub checksum_indexes: Vec<usize>,
    pub error_code: u32,
    pub format: String,
    pub message: String,
    pub remove_headers: Vec<String>,
    pub static_param: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Me {
    pub view: String,
    pub id: i64,
    pub name: String,
    pub username: String,
    #[serde(rename = "canLookStory")]
    pub can_look_story: Option<bool>,
    #[serde(rename = "canCommentStory")]
    pub can_comment_story: Option<bool>,
    #[serde(rename = "hasNotViewedStory")]
    pub has_not_viewed_story: Option<bool>,
    #[serde(rename = "isVerified")]
    pub is_verified: Option<bool>,
    #[serde(rename = "canPayInternal")]
    pub can_pay_internal: Option<bool>,
    #[serde(rename = "vatName")]
    pub vat_name: String,
    #[serde(rename = "vatCountry")]
    pub vat_country: String,
    #[serde(rename = "vatState")]
    pub vat_state: String,
    #[serde(rename = "canSendChatToAll")]
    pub can_send_chat_to_all: Option<bool>,
    #[serde(rename = "creditsMin")]
    pub credits_min: i64,
    #[serde(rename = "creditsMax")]
    pub credits_max: i64,
    #[serde(rename = "isPaywallRestriction")]
    pub is_paywall_restriction: Option<bool>,
    pub unprofitable: Option<bool>,
    #[serde(rename = "listsSort")]
    pub lists_sort: String,
    #[serde(rename = "listsSortOrder")]
    pub lists_sort_order: String,
    #[serde(rename = "canCreateLists")]
    pub can_create_lists: Option<bool>,
    #[serde(rename = "joinDate")]
    pub join_date: String,
    #[serde(rename = "isReferrerAllowed")]
    pub is_referrer_allowed: Option<bool>,
    pub about: String,
    #[serde(rename = "rawAbout")]
    pub raw_about: String,
    // pub website: Option<serde_json::Value>,
    // pub wishlist: Option<serde_json::Value>,
    // pub location: Option<serde_json::Value>,
    #[serde(rename = "postsCount")]
    pub posts_count: i64,
    #[serde(rename = "archivedPostsCount")]
    pub archived_posts_count: i64,
    #[serde(rename = "photosCount")]
    pub photos_count: i64,
    #[serde(rename = "videosCount")]
    pub videos_count: i64,
    #[serde(rename = "audiosCount")]
    pub audios_count: i64,
    #[serde(rename = "mediasCount")]
    pub medias_count: i64,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: Option<i64>,
    #[serde(rename = "favoritedCount")]
    pub favorited_count: i64,
    #[serde(rename = "showPostsInFeed")]
    pub show_posts_in_feed: Option<bool>,
    #[serde(rename = "canReceiveChatMessage")]
    pub can_receive_chat_message: Option<bool>,
    #[serde(rename = "isPerformer")]
    pub is_performer: Option<bool>,
    #[serde(rename = "isRealPerformer")]
    pub is_real_performer: Option<bool>,
    #[serde(rename = "isSpotifyConnected")]
    pub is_spotify_connected: Option<bool>,
    #[serde(rename = "subscribersCount")]
    pub subscribers_count: i64,
    #[serde(rename = "hasPinnedPosts")]
    pub has_pinned_posts: Option<bool>,
    #[serde(rename = "canChat")]
    pub can_chat: Option<bool>,
    #[serde(rename = "canAddPhone")]
    pub can_add_phone: Option<bool>,
    // #[serde(rename = "phoneLast4")]
    // pub phone_last4: Option<serde_json::Value>,
    // #[serde(rename = "phoneMask")]
    // pub phone_mask: Option<serde_json::Value>,
    #[serde(rename = "hasNewTicketReplies")]
    pub has_new_ticket_replies: HasNewTicketReplies,
    #[serde(rename = "hasInternalPayments")]
    pub has_internal_payments: Option<bool>,
    #[serde(rename = "isCreditsEnabled")]
    pub is_credits_enabled: Option<bool>,
    #[serde(rename = "creditBalance")]
    pub credit_balance: f64,
    #[serde(rename = "isMakePayment")]
    pub is_make_payment: Option<bool>,
    #[serde(rename = "isAgeVerified")]
    pub is_age_verified: Option<bool>,
    #[serde(rename = "ageVerificationRequired")]
    pub age_verification_required: Option<bool>,
    #[serde(rename = "isOtpEnabled")]
    pub is_otp_enabled: Option<bool>,
    pub email: String,
    #[serde(rename = "isEmailChecked")]
    pub is_email_checked: Option<bool>,
    #[serde(rename = "isLegalApprovedAllowed")]
    pub is_legal_approved_allowed: Option<bool>,
    #[serde(rename = "isTwitterConnected")]
    pub is_twitter_connected: Option<bool>,
    // #[serde(rename = "twitterUsername")]
    // pub twitter_username: Option<serde_json::Value>,
    #[serde(rename = "isAllowTweets")]
    pub is_allow_tweets: Option<bool>,
    #[serde(rename = "isPaymentCardConnected")]
    pub is_payment_card_connected: Option<bool>,
    #[serde(rename = "referalUrl")]
    pub referal_url: String,
    #[serde(rename = "isVisibleOnline")]
    pub is_visible_online: Option<bool>,
    #[serde(rename = "subscribesCount")]
    pub subscribes_count: i64,
    #[serde(rename = "canPinPost")]
    pub can_pin_post: Option<bool>,
    #[serde(rename = "hasNewAlerts")]
    pub has_new_alerts: Option<bool>,
    #[serde(rename = "hasNewHints")]
    pub has_new_hints: Option<bool>,
    #[serde(rename = "hasNewChangedPriceSubscriptions")]
    pub has_new_changed_price_subscriptions: Option<bool>,
    #[serde(rename = "notificationsCount")]
    pub notifications_count: i64,
    #[serde(rename = "chatMessagesCount")]
    pub chat_messages_count: i64,
    #[serde(rename = "isWantComments")]
    pub is_want_comments: Option<bool>,
    #[serde(rename = "watermarkText")]
    pub watermark_text: String,
    // #[serde(rename = "customWatermarkText")]
    // pub custom_watermark_text: Option<serde_json::Value>,
    #[serde(rename = "hasWatermarkPhoto")]
    pub has_watermark_photo: Option<bool>,
    #[serde(rename = "hasWatermarkVideo")]
    pub has_watermark_video: Option<bool>,
    #[serde(rename = "canDelete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "isTelegramConnected")]
    pub is_telegram_connected: Option<bool>,
    // #[serde(rename = "advBlock")]
    // pub adv_block: Vec<Option<serde_json::Value>>,
    #[serde(rename = "hasPurchasedPosts")]
    pub has_purchased_posts: Option<bool>,
    #[serde(rename = "isEmailRequired")]
    pub is_email_required: Option<bool>,
    #[serde(rename = "isPayoutLegalApproved")]
    pub is_payout_legal_approved: Option<bool>,
    #[serde(rename = "payoutLegalApproveState")]
    pub payout_legal_approve_state: String,
    // #[serde(rename = "payoutLegalApproveRejectReason")]
    // pub payout_legal_approve_reject_reason: Option<serde_json::Value>,
    #[serde(rename = "enabledImageEditorForChat")]
    pub enabled_image_editor_for_chat: Option<bool>,
    #[serde(rename = "shouldReceiveLessNotifications")]
    pub should_receive_less_notifications: Option<bool>,
    #[serde(rename = "canCalling")]
    pub can_calling: Option<bool>,
    #[serde(rename = "paidFeed")]
    pub paid_feed: Option<bool>,
    #[serde(rename = "canSendSms")]
    pub can_send_sms: Option<bool>,
    #[serde(rename = "canAddFriends")]
    pub can_add_friends: Option<bool>,
    #[serde(rename = "isRealCardConnected")]
    pub is_real_card_connected: Option<bool>,
    #[serde(rename = "countPriorityChat")]
    pub count_priority_chat: i64,
    #[serde(rename = "hasScenario")]
    pub has_scenario: Option<bool>,
    #[serde(rename = "isWalletAutorecharge")]
    pub is_wallet_autorecharge: Option<bool>,
    #[serde(rename = "walletAutorechargeAmount")]
    pub wallet_autorecharge_amount: i64,
    #[serde(rename = "walletAutorechargeMin")]
    pub wallet_autorecharge_min: i64,
    #[serde(rename = "walletFirstRebills")]
    pub wallet_first_rebills: Option<bool>,
    #[serde(rename = "closeFriends")]
    pub close_friends: i64,
    #[serde(rename = "canAlternativeWalletTopUp")]
    pub can_alternative_wallet_top_up: Option<bool>,
    #[serde(rename = "needIVApprove")]
    pub need_iv_approve: Option<bool>,
    #[serde(rename = "ivStatus")]
    pub iv_status: String,
    // #[serde(rename = "ivFailReason")]
    // pub iv_fail_reason: Option<serde_json::Value>,
    #[serde(rename = "canCheckDocsOnAddCard")]
    pub can_check_docs_on_add_card: Option<bool>,
    #[serde(rename = "faceIdAvailable")]
    pub face_id_available: Option<bool>,
    // #[serde(rename = "ivCountry")]
    // pub iv_country: Option<serde_json::Value>,
    #[serde(rename = "ivForcedVerified")]
    pub iv_forced_verified: Option<bool>,
    #[serde(rename = "ivFlow")]
    pub iv_flow: String,
    // #[serde(rename = "connectedOfAccounts")]
    // pub connected_of_accounts: Vec<Option<serde_json::Value>>,
    #[serde(rename = "hasPassword")]
    pub has_password: Option<bool>,
    #[serde(rename = "canConnectOfAccount")]
    pub can_connect_of_account: Option<bool>,
    #[serde(rename = "pinnedPostsCount")]
    pub pinned_posts_count: i64,
    #[serde(rename = "maxPinnedPostsCount")]
    pub max_pinned_posts_count: i64,
    #[serde(rename = "isDeleteInitiated")]
    pub is_delete_initiated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchasedItem {
    #[serde(rename = "responseType")]
    pub response_type: String,
    pub text: Option<String>,
    #[serde(rename = "lockedText")]
    pub locked_text: Option<bool>,
    #[serde(rename = "isFree")]
    pub is_free: Option<bool>,
    pub price: f64,
    #[serde(rename = "isMediaReady")]
    pub is_media_ready: Option<bool>,
    #[serde(rename = "mediaCount")]
    pub media_count: Option<i64>,
    pub media: Option<Vec<Media>>,
    pub previews: Option<Vec<i64>>,
    #[serde(rename = "isTip")]
    pub is_tip: Option<bool>,
    #[serde(rename = "isReportedByMe")]
    pub is_reported_by_me: Option<bool>,
    #[serde(rename = "fromUser")]
    pub from_user: Option<FromUser>,
    pub author: Option<FromUser>,
    pub id: i64,
    #[serde(rename = "isOpened")]
    pub is_opened: Option<bool>,
    #[serde(rename = "isNew")]
    pub is_new: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "changedAt")]
    pub changed_at: Option<String>,
    #[serde(rename = "cancelSeconds")]
    pub cancel_seconds: Option<i64>,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "canPurchase")]
    pub can_purchase: Option<bool>,
    #[serde(rename = "canReport")]
    pub can_report: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "responseType")]
    pub response_type: String,
    pub text: Option<String>,
    #[serde(rename = "lockedText")]
    pub locked_text: Option<bool>,
    #[serde(rename = "isFree")]
    pub is_free: Option<bool>,
    pub price: f64,
    #[serde(rename = "isMediaReady")]
    pub is_media_ready: Option<bool>,
    #[serde(rename = "mediaCount")]
    pub media_count: Option<i64>,
    pub media: Option<Vec<Media>>,
    #[serde(rename = "isTip")]
    pub is_tip: Option<bool>,
    #[serde(rename = "isReportedByMe")]
    pub is_reported_by_me: Option<bool>,
    #[serde(rename = "fromUser")]
    pub from_user: Option<FromUser>,
    #[serde(rename = "isFromQueue")]
    pub is_from_queue: Option<bool>,
    pub id: i64,
    #[serde(rename = "isOpened")]
    pub is_opened: Option<bool>,
    #[serde(rename = "isNew")]
    pub is_new: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "changedAt")]
    pub changed_at: Option<String>,
    #[serde(rename = "cancelSeconds")]
    pub cancel_seconds: Option<i64>,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "canPurchase")]
    pub can_purchase: Option<bool>,
    #[serde(rename = "canPurchaseReason")]
    pub can_purchase_reason: String,
    #[serde(rename = "canReport")]
    pub can_report: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromUser {
    pub id: i64,
    #[serde(rename = "_view")]
    pub view: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUser {
    pub view: String,
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar: String,
    #[serde(rename = "avatarThumbs")]
    pub avatar_thumbs: Option<AvatarThumbs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview {
    pub width: i64,
    pub height: i64,
    pub size: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HasNewTicketReplies {
    pub open: Option<bool>,
    pub solved: Option<bool>,
    pub closed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarThumbs {
    pub c50: String,
    pub c144: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderSize {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderThumbs {
    pub w480: String,
    pub w760: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListsState {
    pub id: i64,
    #[serde(rename = "type")]
    pub lists_state_type: String,
    pub name: String,
    #[serde(rename = "hasUser")]
    pub has_user: Option<bool>,
    #[serde(rename = "canAddUser")]
    pub can_add_user: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribedByData {
    pub price: f64,
    #[serde(rename = "newPrice")]
    pub new_price: f64,
    #[serde(rename = "regularPrice")]
    pub regular_price: f64,
    #[serde(rename = "subscribePrice")]
    pub subscribe_price: f64,
    #[serde(rename = "discountPercent")]
    pub discount_percent: i64,
    #[serde(rename = "discountPeriod")]
    pub discount_period: i64,
    #[serde(rename = "subscribeAt")]
    pub subscribe_at: Option<String>,
    #[serde(rename = "expiredAt")]
    pub expired_at: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "isMuted")]
    pub is_muted: Option<bool>,
    #[serde(rename = "unsubscribeReason")]
    pub unsubscribe_reason: String,
    pub duration: String,
    #[serde(rename = "showPostsInFeed")]
    pub show_posts_in_feed: Option<bool>,
    pub subscribes: Vec<Subscribe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: i64,
    pub date: String,
    pub duration: i64,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "expireDate")]
    pub expire_date: String,
    pub price: f64,
    #[serde(rename = "regularPrice")]
    pub regular_price: f64,
    pub discount: i64,
    #[serde(rename = "earningId")]
    pub earning_id: i64,
    pub action: String,
    #[serde(rename = "type")]
    pub subscribe_type: String,
    #[serde(rename = "offerStart")]
    pub offer_start: Option<String>,
    #[serde(rename = "isCurrent")]
    pub is_current: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribedOnData {
    pub price: f64,
    #[serde(rename = "newPrice")]
    pub new_price: f64,
    #[serde(rename = "regularPrice")]
    pub regular_price: f64,
    #[serde(rename = "subscribePrice")]
    pub subscribe_price: f64,
    #[serde(rename = "discountPercent")]
    pub discount_percent: i64,
    #[serde(rename = "discountPeriod")]
    pub discount_period: i64,
    #[serde(rename = "subscribeAt")]
    pub subscribe_at: String,
    #[serde(rename = "expiredAt")]
    pub expired_at: String,
    #[serde(rename = "renewedAt")]
    pub renewed_at: Option<String>,
    #[serde(rename = "isMuted")]
    pub is_muted: Option<bool>,
    #[serde(rename = "unsubscribeReason")]
    pub unsubscribe_reason: String,
    pub duration: String,
    #[serde(rename = "messagesSumm")]
    pub messages_summ: i64,
    #[serde(rename = "tipsSumm")]
    pub tips_summ: i64,
    #[serde(rename = "subscribesSumm")]
    pub subscribes_summ: i64,
    #[serde(rename = "postsSumm")]
    pub posts_summ: i64,
    #[serde(rename = "streamsSumm")]
    pub streams_summ: i64,
    #[serde(rename = "totalSumm")]
    pub total_summ: i64,
    pub subscribes: Vec<Subscribe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "responseType")]
    pub response_type: String,
    pub id: i64,
    #[serde(rename = "postedAt")]
    pub posted_at: String,
    #[serde(rename = "postedAtPrecise")]
    pub posted_at_precise: String,
    #[serde(rename = "expiredAt")]
    pub expired_at: Option<String>,
    pub author: Option<Author>,
    pub text: Option<String>,
    #[serde(rename = "rawText")]
    pub raw_text: Option<String>,
    #[serde(rename = "lockedText")]
    pub locked_text: Option<bool>,
    #[serde(rename = "isFavorite")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "canReport")]
    pub can_report: Option<bool>,
    #[serde(rename = "canDelete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "canComment")]
    pub can_comment: Option<bool>,
    #[serde(rename = "canEdit")]
    pub can_edit: Option<bool>,
    #[serde(rename = "isPinned")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: Option<i64>,
    #[serde(rename = "mediaCount")]
    pub media_count: Option<i64>,
    #[serde(rename = "isMediaReady")]
    pub is_media_ready: Option<bool>,
    #[serde(rename = "isOpened")]
    pub is_opened: Option<bool>,
    #[serde(rename = "canToggleFavorite")]
    pub can_toggle_favorite: Option<bool>,
    #[serde(rename = "streamId")]
    pub stream_id: Option<i64>,
    pub price: Option<f64>,
    #[serde(rename = "hasVoting")]
    pub has_voting: Option<bool>,
    #[serde(rename = "isAddedToBookmarks")]
    pub is_added_to_bookmarks: Option<bool>,
    #[serde(rename = "isArchived")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    #[serde(rename = "hasUrl")]
    pub has_url: Option<bool>,
    #[serde(rename = "commentsCount")]
    pub comments_count: Option<i64>,
    #[serde(rename = "tipsAmount")]
    pub tips_amount: Option<String>,
    #[serde(rename = "tipsAmountRaw")]
    pub tips_amount_raw: Option<f64>,
    pub media: Option<Vec<Media>>,
    #[serde(rename = "canViewMedia")]
    pub can_view_media: Option<bool>,
    #[serde(rename = "votingType")]
    pub voting_type: Option<i64>,
    #[serde(rename = "canVote")]
    pub can_vote: Option<bool>,
    #[serde(rename = "fundRaising")]
    pub fund_raising: Option<FundRaising>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundRaising {
    pub target: f64,
    #[serde(rename = "targetProgress")]
    pub target_progress: f64,
    pub presets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i64,
    #[serde(rename = "_view")]
    pub view: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: i64,
    #[serde(rename = "type")]
    pub media_type: String,
    #[serde(rename = "convertedToVideo")]
    pub converted_to_video: Option<bool>,
    #[serde(rename = "canView")]
    pub can_view: Option<bool>,
    #[serde(rename = "hasError")]
    pub has_error: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub info: Option<Info>,
    pub source: Option<Source>,
    #[serde(rename = "squarePreview")]
    pub square_preview: Option<String>,
    pub full: Option<String>,
    pub preview: Option<String>,
    pub thumb: Option<String>,
    pub files: Option<Files>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub preview: Option<FileSource>,
    pub source: Option<FileSource>,
    pub thumb: Option<FileSource>,
    #[serde(rename = "squarePreview")]
    pub square_preview: Option<FileSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSource {
    pub url: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub size: Option<i64>,
    pub duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub source: Option<Source>,
    pub preview: Option<InfoPreview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoPreview {
    pub width: i64,
    pub height: i64,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub source: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub size: Option<i64>,
    pub duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "expiredAt")]
    pub expired_at: Option<String>,
    #[serde(rename = "isReady")]
    pub is_ready: Option<bool>,
    #[serde(rename = "viewersCount")]
    pub viewers_count: i64,
    #[serde(rename = "canLike")]
    pub can_like: Option<bool>,
    #[serde(rename = "mediaCount")]
    pub media_count: Option<i64>,
    #[serde(rename = "isWatched")]
    pub is_watched: Option<bool>,
    #[serde(rename = "isLiked")]
    pub is_liked: Option<bool>,
    #[serde(rename = "canDelete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "isHighlightCover")]
    pub is_highlight_cover: Option<bool>,
    #[serde(rename = "isLastInHighlight")]
    pub is_last_in_highlight: Option<bool>,
    pub media: Option<Vec<Media>>,
    pub answered: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "vatName")]
    pub vat_name: Option<String>,
    #[serde(rename = "vatPercentage")]
    pub vat_percentage: Option<f64>,
    pub amount: f64,
    #[serde(rename = "vatAmount")]
    pub vat_amount: Option<f64>,
    pub net: Option<i64>,
    pub fee: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub currency: String,
    pub description: String,
    pub status: Option<String>,
    pub user: Option<User>,
    pub source: Option<String>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub view: String,
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    #[serde(rename = "avatarThumbs")]
    pub avatar_thumbs: Option<AvatarThumbs>,
}
