use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FanslyResponse<T> {
    pub response: T,
    pub success: bool,
}

// Response the consts::USER_STORIES_URL endpoint
pub type AccountStoriesResponse = FanslyResponse<Vec<crate::structs::Story>>;
// Response the consts::USER_ACCOUNT_URL endpoint
pub type AccountsResponse = FanslyResponse<Vec<crate::structs::Account>>;
// Response the consts::FOLLOWED_ACCOUNTS_URL endpoint
pub type FollowedAccountsResponse = FanslyResponse<Vec<crate::structs::FollowedAccount>>;
// Response the consts::GROUP_MESSAGES_URL endpoint
pub type GroupMessagesResponse = FanslyResponse<inner::GroupMessages>;
// Response the consts::MEDIA_BUNDLE_URL endpoint
pub type MediaBundleResponse = FanslyResponse<Vec<crate::structs::MediaBundle>>;
// Response the consts::MEDIA_URL endpoint
pub type MediaResponse = FanslyResponse<Vec<crate::structs::Media>>;
// Response the consts::MEDIA_GROUPS_URL endpoint
pub type MessageGroupsResponse = FanslyResponse<inner::MessageGroups>;
// Response the consts::POSTS_URL endpoint
pub type PostsResponse = FanslyResponse<inner::Posts>;
// Response the consts::PURCHASED_URL endpoint
pub type PurchasedContentResponse = FanslyResponse<inner::PurchasedContent>;
// Response the consts::STATUS_URL endpoint
pub type StatusResponse = FanslyResponse<inner::Status>;
// Response the consts::SUBS_URL endpoint
pub type SubscriptionResponse = FanslyResponse<inner::Subscriptions>;
// Response the consts::TRANSACTIONS_URL endpoint
pub type TransactionsResponse = FanslyResponse<crate::structs::Transaction>;
// Response the consts::WALLET_TRANSACTIONS_URL endpoint
pub type WalletTransactionsResponse = FanslyResponse<crate::structs::WalletTransaction>;

pub(crate) mod inner {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Subscriptions {
        #[serde(rename = "subscriptionPlans")]
        pub subscription_plans: Vec<crate::structs::SubscriptionPlan>,
        pub subscriptions: Vec<crate::structs::Subscription>,
        pub stats: crate::structs::SubscriptionStats,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PurchasedContent {
        #[serde(rename = "accountMediaOrders")]
        pub account_media_orders: Vec<String>,
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

    #[derive(Serialize, Deserialize)]
    pub struct Posts {
        pub posts: Option<Vec<crate::structs::Post>>,
        #[serde(rename = "aggregatedPosts")]
        pub aggregated_posts: Option<Vec<crate::structs::Post>>,
        #[serde(rename = "accountMediaBundles")]
        pub account_media_bundles: Option<Vec<crate::structs::Media>>,
        #[serde(rename = "accountMedia")]
        pub account_media: Option<Vec<crate::structs::Media>>,
        // pub accounts: Vec<Account>,
        pub tips: Option<Vec<String>>,
        // #[serde(rename = "tipGoals")]
        // pub tip_goals: Option<Vec<TipGoals>>,
        pub stories: Option<Vec<crate::structs::Story>>,
    }

    impl std::fmt::Debug for Posts {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let posts_count = if let Some(posts) = &self.posts {
                posts.len()
            } else {
                0
            };
            f.debug_struct("Posts")
                .field("posts", &posts_count)
                .finish_non_exhaustive()
        }
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
