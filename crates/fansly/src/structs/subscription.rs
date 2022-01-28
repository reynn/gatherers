use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
    #[serde(rename = "subscriptionTierName")]
    pub subscription_tier_name: Option<String>,
    #[serde(rename = "subscriptionTierColor")]
    pub subscription_tier_color: Option<String>,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "promoId")]
    pub promo_id: Option<String>,
    // #[serde(rename = "giftCodeId")]
    // pub gift_code_id: Option<String>,
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
pub struct SubscriptionStats {
    #[serde(rename = "totalActive")]
    pub total_active: i64,
    #[serde(rename = "totalExpired")]
    pub total_expired: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
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
pub enum SubscriptionStatus {
    Active,  // 3
    Expired, // 5
    Invalid,
}

impl From<i64> for SubscriptionStatus {
    fn from(i: i64) -> Self {
        match i {
            3 => Self::Active,
            5 => Self::Expired,
            _ => Self::Invalid,
        }
    }
}
