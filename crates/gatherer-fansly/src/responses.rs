use super::structs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

pub type AccountsResponse = FanslyResponse<Vec<Account>>;
pub type SubscriptionResponse = FanslyResponse<inner::Subscriptions>;
pub type StatusResponse = FanslyResponse<inner::Status>;
pub type PostsResponse = FanslyResponse<inner::Posts>;
pub type MediaResponse = FanslyResponse<Vec<Media>>;
pub type MediaBundleResponse = FanslyResponse<Vec<MediaBundle>>;
pub type MessageGroupsResponse = FanslyResponse<inner::MessageGroups>;
pub type GroupMessagesResponse = FanslyResponse<inner::GroupMessages>;

pub mod inner {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Subscriptions {
        #[serde(rename = "subscriptionPlans")]
        pub subscription_plans: Vec<crate::structs::SubscriptionPlan>,
        pub subscriptions: Vec<crate::structs::Subscription>,
        pub stats: crate::structs::SubscriptionStats,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Status {
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
    pub struct Posts {
        pub posts: Vec<crate::structs::Post>,
        // #[serde(rename = "aggregatedPosts")]
        // pub aggregated_posts: Vec<String>,
        #[serde(rename = "accountMediaBundles")]
        pub account_media_bundles: Vec<crate::structs::Media>,
        #[serde(rename = "accountMedia")]
        pub account_media: Vec<crate::structs::Media>,
        // pub accounts: Vec<Account>,
        pub tips: Vec<String>,
        // #[serde(rename = "tipGoals")]
        // pub tip_goals: Option<Vec<TipGoals>>,
        pub stories: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MessageGroups {
        pub groups: Vec<crate::structs::MessageGroup>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GroupMessages {
        pub messages: Vec<crate::structs::Message>,
        #[serde(rename = "accountMedia")]
        pub account_media: Vec<crate::structs::Media>,
    }
}
