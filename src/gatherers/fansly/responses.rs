use super::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

pub(super) type AccountResponse = FanslyResponse<Vec<Account>>;
pub(super) type SubscriptionResponse = FanslyResponse<SubsInner>;
pub(super) type StatusResponse = FanslyResponse<StatusInner>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub(super) struct SubsInner {
    #[serde(rename = "subscriptionPlans")]
    pub subscription_plans: Vec<Plan>,
    pub subscriptions: Vec<Subscription>,
    pub stats: SubscriptionStats,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct StatusInner {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "statusId")] 
    status_id: i8,
    #[serde(rename = "lastSeenAt")]
    last_seen_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
}