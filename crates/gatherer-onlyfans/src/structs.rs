use serde::{Deserialize, Serialize};

pub type Subscriptions = Vec<Subscription>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub view: String,
    pub avatar: String,
    #[serde(rename = "avatarThumbs")]
    pub avatar_thumbs: AvatarThumbs,
    pub header: String,
    #[serde(rename = "headerSize")]
    pub header_size: HeaderSize,
    #[serde(rename = "headerThumbs")]
    pub header_thumbs: HeaderThumbs,
    pub id: i64,
    pub name: String,
    pub username: String,
    #[serde(rename = "canLookStory")]
    pub can_look_story: bool,
    #[serde(rename = "canCommentStory")]
    pub can_comment_story: bool,
    #[serde(rename = "hasNotViewedStory")]
    pub has_not_viewed_story: bool,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "canPayInternal")]
    pub can_pay_internal: bool,
    #[serde(rename = "hasScheduledStream")]
    pub has_scheduled_stream: bool,
    #[serde(rename = "hasStream")]
    pub has_stream: bool,
    #[serde(rename = "hasStories")]
    pub has_stories: bool,
    #[serde(rename = "tipsEnabled")]
    pub tips_enabled: bool,
    #[serde(rename = "tipsTextEnabled")]
    pub tips_text_enabled: bool,
    #[serde(rename = "tipsMin")]
    pub tips_min: i64,
    #[serde(rename = "tipsMax")]
    pub tips_max: i64,
    #[serde(rename = "canEarn")]
    pub can_earn: bool,
    #[serde(rename = "canAddSubscriber")]
    pub can_add_subscriber: bool,
    #[serde(rename = "subscribePrice")]
    pub subscribe_price: f64,
    pub vat: Vat,
    // #[serde(rename = "subscriptionBundles")]
    // pub subscription_bundles: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "isPaywallRestriction")]
    pub is_paywall_restriction: bool,
    pub unprofitable: Option<bool>,
    #[serde(rename = "listsStates")]
    pub lists_states: Vec<ListsState>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "isRestricted")]
    pub is_restricted: bool,
    #[serde(rename = "canRestrict")]
    pub can_restrict: bool,
    #[serde(rename = "subscribedBy")]
    pub subscribed_by: bool,
    #[serde(rename = "subscribedByExpire")]
    pub subscribed_by_expire: bool,
    #[serde(rename = "subscribedByExpireDate")]
    pub subscribed_by_expire_date: String,
    #[serde(rename = "subscribedByAutoprolong")]
    pub subscribed_by_autoprolong: bool,
    #[serde(rename = "subscribedIsExpiredNow")]
    pub subscribed_is_expired_now: bool,
    #[serde(rename = "currentSubscribePrice")]
    pub current_subscribe_price: i64,
    #[serde(rename = "subscribedOn")]
    pub subscribed_on: bool,
    #[serde(rename = "subscribedOnExpiredNow")]
    pub subscribed_on_expired_now: Option<bool>,
    #[serde(rename = "subscribedOnDuration")]
    pub subscribed_on_duration: Option<String>,
    #[serde(rename = "canReport")]
    pub can_report: bool,
    #[serde(rename = "canReceiveChatMessage")]
    pub can_receive_chat_message: bool,
    #[serde(rename = "hideChat")]
    pub hide_chat: bool,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<String>,
    #[serde(rename = "isPerformer")]
    pub is_performer: bool,
    #[serde(rename = "isRealPerformer")]
    pub is_real_performer: bool,
    #[serde(rename = "subscribedByData")]
    pub subscribed_by_data: SubscribedByData,
    #[serde(rename = "subscribedOnData")]
    pub subscribed_on_data: Option<SubscribedOnData>,
    #[serde(rename = "canTrialSend")]
    pub can_trial_send: bool,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    // #[serde(rename = "displayName")]
    // pub display_name: Option<serde_json::Value>,
    // pub notice: Option<serde_json::Value>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicRule {
    pub app_token: String,
    pub checksum_constant: i64,
    pub checksum_indexes: Vec<i64>,
    pub error_code: i64,
    pub format: String,
    pub message: String,
    pub remove_headers: Vec<String>,
    pub static_param: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub view: String,
    // pub avatar: Option<Avatar>,
    // #[serde(rename = "avatarThumbs")]
    // pub avatar_thumbs: Option<serde_json::Value>,
    // pub header: Option<serde_json>,
    // #[serde(rename = "headerSize")]
    // pub header_size: Option<serde_json::Value>,
    // #[serde(rename = "headerThumbs")]
    // pub header_thumbs: Option<serde_json::Value>,
    pub id: i64,
    pub name: String,
    pub username: String,
    #[serde(rename = "canLookStory")]
    pub can_look_story: bool,
    #[serde(rename = "canCommentStory")]
    pub can_comment_story: bool,
    #[serde(rename = "hasNotViewedStory")]
    pub has_not_viewed_story: bool,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "canPayInternal")]
    pub can_pay_internal: bool,
    #[serde(rename = "vatName")]
    pub vat_name: String,
    #[serde(rename = "vatCountry")]
    pub vat_country: String,
    #[serde(rename = "vatState")]
    pub vat_state: String,
    #[serde(rename = "canSendChatToAll")]
    pub can_send_chat_to_all: bool,
    #[serde(rename = "creditsMin")]
    pub credits_min: i64,
    #[serde(rename = "creditsMax")]
    pub credits_max: i64,
    #[serde(rename = "isPaywallRestriction")]
    pub is_paywall_restriction: bool,
    pub unprofitable: bool,
    #[serde(rename = "listsSort")]
    pub lists_sort: String,
    #[serde(rename = "listsSortOrder")]
    pub lists_sort_order: String,
    #[serde(rename = "canCreateLists")]
    pub can_create_lists: bool,
    #[serde(rename = "joinDate")]
    pub join_date: String,
    #[serde(rename = "isReferrerAllowed")]
    pub is_referrer_allowed: bool,
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
    // #[serde(rename = "lastSeen")]
    // pub last_seen: Option<serde_json::Value>,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    #[serde(rename = "favoritedCount")]
    pub favorited_count: i64,
    #[serde(rename = "showPostsInFeed")]
    pub show_posts_in_feed: bool,
    #[serde(rename = "canReceiveChatMessage")]
    pub can_receive_chat_message: bool,
    #[serde(rename = "isPerformer")]
    pub is_performer: bool,
    #[serde(rename = "isRealPerformer")]
    pub is_real_performer: bool,
    #[serde(rename = "isSpotifyConnected")]
    pub is_spotify_connected: bool,
    #[serde(rename = "subscribersCount")]
    pub subscribers_count: i64,
    #[serde(rename = "hasPinnedPosts")]
    pub has_pinned_posts: bool,
    #[serde(rename = "canChat")]
    pub can_chat: bool,
    #[serde(rename = "canAddPhone")]
    pub can_add_phone: bool,
    // #[serde(rename = "phoneLast4")]
    // pub phone_last4: Option<serde_json::Value>,
    // #[serde(rename = "phoneMask")]
    // pub phone_mask: Option<serde_json::Value>,
    #[serde(rename = "hasNewTicketReplies")]
    pub has_new_ticket_replies: HasNewTicketReplies,
    #[serde(rename = "hasInternalPayments")]
    pub has_internal_payments: bool,
    #[serde(rename = "isCreditsEnabled")]
    pub is_credits_enabled: bool,
    #[serde(rename = "creditBalance")]
    pub credit_balance: f64,
    #[serde(rename = "isMakePayment")]
    pub is_make_payment: bool,
    #[serde(rename = "isAgeVerified")]
    pub is_age_verified: bool,
    #[serde(rename = "ageVerificationRequired")]
    pub age_verification_required: bool,
    #[serde(rename = "isOtpEnabled")]
    pub is_otp_enabled: bool,
    pub email: String,
    #[serde(rename = "isEmailChecked")]
    pub is_email_checked: bool,
    #[serde(rename = "isLegalApprovedAllowed")]
    pub is_legal_approved_allowed: bool,
    #[serde(rename = "isTwitterConnected")]
    pub is_twitter_connected: bool,
    // #[serde(rename = "twitterUsername")]
    // pub twitter_username: Option<serde_json::Value>,
    #[serde(rename = "isAllowTweets")]
    pub is_allow_tweets: bool,
    #[serde(rename = "isPaymentCardConnected")]
    pub is_payment_card_connected: bool,
    #[serde(rename = "referalUrl")]
    pub referal_url: String,
    #[serde(rename = "isVisibleOnline")]
    pub is_visible_online: bool,
    #[serde(rename = "subscribesCount")]
    pub subscribes_count: i64,
    #[serde(rename = "canPinPost")]
    pub can_pin_post: bool,
    #[serde(rename = "hasNewAlerts")]
    pub has_new_alerts: bool,
    #[serde(rename = "hasNewHints")]
    pub has_new_hints: bool,
    #[serde(rename = "hasNewChangedPriceSubscriptions")]
    pub has_new_changed_price_subscriptions: bool,
    #[serde(rename = "notificationsCount")]
    pub notifications_count: i64,
    #[serde(rename = "chatMessagesCount")]
    pub chat_messages_count: i64,
    #[serde(rename = "isWantComments")]
    pub is_want_comments: bool,
    #[serde(rename = "watermarkText")]
    pub watermark_text: String,
    // #[serde(rename = "customWatermarkText")]
    // pub custom_watermark_text: Option<serde_json::Value>,
    #[serde(rename = "hasWatermarkPhoto")]
    pub has_watermark_photo: bool,
    #[serde(rename = "hasWatermarkVideo")]
    pub has_watermark_video: bool,
    #[serde(rename = "canDelete")]
    pub can_delete: bool,
    #[serde(rename = "isTelegramConnected")]
    pub is_telegram_connected: bool,
    // #[serde(rename = "advBlock")]
    // pub adv_block: Vec<Option<serde_json::Value>>,
    #[serde(rename = "hasPurchasedPosts")]
    pub has_purchased_posts: bool,
    #[serde(rename = "isEmailRequired")]
    pub is_email_required: bool,
    #[serde(rename = "isPayoutLegalApproved")]
    pub is_payout_legal_approved: bool,
    #[serde(rename = "payoutLegalApproveState")]
    pub payout_legal_approve_state: String,
    // #[serde(rename = "payoutLegalApproveRejectReason")]
    // pub payout_legal_approve_reject_reason: Option<serde_json::Value>,
    #[serde(rename = "enabledImageEditorForChat")]
    pub enabled_image_editor_for_chat: bool,
    #[serde(rename = "shouldReceiveLessNotifications")]
    pub should_receive_less_notifications: bool,
    #[serde(rename = "canCalling")]
    pub can_calling: bool,
    #[serde(rename = "paidFeed")]
    pub paid_feed: bool,
    #[serde(rename = "canSendSms")]
    pub can_send_sms: bool,
    #[serde(rename = "canAddFriends")]
    pub can_add_friends: bool,
    #[serde(rename = "isRealCardConnected")]
    pub is_real_card_connected: bool,
    #[serde(rename = "countPriorityChat")]
    pub count_priority_chat: i64,
    #[serde(rename = "hasScenario")]
    pub has_scenario: bool,
    #[serde(rename = "isWalletAutorecharge")]
    pub is_wallet_autorecharge: bool,
    #[serde(rename = "walletAutorechargeAmount")]
    pub wallet_autorecharge_amount: i64,
    #[serde(rename = "walletAutorechargeMin")]
    pub wallet_autorecharge_min: i64,
    #[serde(rename = "walletFirstRebills")]
    pub wallet_first_rebills: bool,
    #[serde(rename = "closeFriends")]
    pub close_friends: i64,
    #[serde(rename = "canAlternativeWalletTopUp")]
    pub can_alternative_wallet_top_up: bool,
    #[serde(rename = "needIVApprove")]
    pub need_iv_approve: bool,
    #[serde(rename = "ivStatus")]
    pub iv_status: String,
    // #[serde(rename = "ivFailReason")]
    // pub iv_fail_reason: Option<serde_json::Value>,
    #[serde(rename = "canCheckDocsOnAddCard")]
    pub can_check_docs_on_add_card: bool,
    #[serde(rename = "faceIdAvailable")]
    pub face_id_available: bool,
    // #[serde(rename = "ivCountry")]
    // pub iv_country: Option<serde_json::Value>,
    #[serde(rename = "ivForcedVerified")]
    pub iv_forced_verified: bool,
    #[serde(rename = "ivFlow")]
    pub iv_flow: String,
    // #[serde(rename = "connectedOfAccounts")]
    // pub connected_of_accounts: Vec<Option<serde_json::Value>>,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "canConnectOfAccount")]
    pub can_connect_of_account: bool,
    #[serde(rename = "pinnedPostsCount")]
    pub pinned_posts_count: i64,
    #[serde(rename = "maxPinnedPostsCount")]
    pub max_pinned_posts_count: i64,
    #[serde(rename = "isDeleteInitiated")]
    pub is_delete_initiated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasNewTicketReplies {
    pub open: bool,
    pub solved: bool,
    pub closed: bool,
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
    pub has_user: bool,
    #[serde(rename = "canAddUser")]
    pub can_add_user: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribedByData {
    pub price: i64,
    #[serde(rename = "newPrice")]
    pub new_price: i64,
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
    // #[serde(rename = "renewedAt")]
    // pub renewed_at: Option<serde_json::Value>,
    // #[serde(rename = "discountFinishedAt")]
    // pub discount_finished_at: Option<serde_json::Value>,
    // #[serde(rename = "discountStartedAt")]
    // pub discount_started_at: Option<serde_json::Value>,
    pub status: String,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "unsubscribeReason")]
    pub unsubscribe_reason: String,
    pub duration: String,
    #[serde(rename = "showPostsInFeed")]
    pub show_posts_in_feed: bool,
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
    // #[serde(rename = "cancelDate")]
    // pub cancel_date: Option<serde_json::Value>,
    pub price: i64,
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
    // #[serde(rename = "offerEnd")]
    // pub offer_end: Option<serde_json::Value>,
    #[serde(rename = "isCurrent")]
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribedOnData {
    pub price: i64,
    #[serde(rename = "newPrice")]
    pub new_price: i64,
    #[serde(rename = "regularPrice")]
    pub regular_price: i64,
    #[serde(rename = "subscribePrice")]
    pub subscribe_price: i64,
    #[serde(rename = "discountPercent")]
    pub discount_percent: i64,
    #[serde(rename = "discountPeriod")]
    pub discount_period: i64,
    #[serde(rename = "subscribeAt")]
    pub subscribe_at: String,
    #[serde(rename = "expiredAt")]
    pub expired_at: String,
    #[serde(rename = "renewedAt")]
    pub renewed_at: String,
    // #[serde(rename = "discountFinishedAt")]
    // pub discount_finished_at: Option<serde_json::Value>,
    // #[serde(rename = "discountStartedAt")]
    // pub discount_started_at: Option<serde_json::Value>,
    // pub status: Option<serde_json::Value>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
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
pub struct Vat {
    pub tips: i64,
    pub subscribes: f64,
    pub chat_messages: f64,
    pub post: f64,
    pub stream: f64,
    pub credits: i64,
}
