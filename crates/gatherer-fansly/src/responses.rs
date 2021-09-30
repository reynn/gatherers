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
            // write!(f, "Posts(post_count={})", &posts_count)
            f.debug_struct("Posts").field("posts", &posts_count).finish_non_exhaustive()
            //     .field("aggregated_posts", &self.aggregated_posts)
            //     .field("account_media_bundles", &self.account_media_bundles)
            //     .field("account_media", &self.account_media)
            //     .field("tips", &self.tips)
            //     .field("stories", &self.stories)
            //     .finish()
        }
        // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //     // let posts_count = if let Some(posts) = &self.posts {} else {};
        //     f.debug_struct("Posts")
        //         .field("posts", &self.posts.unwrap_or(vec![]).len())
        //         .field("aggregated_posts", &self.aggregated_posts.unwrap_or(vec![]).len())
        //         .field("account_media_bundles", &self.account_media_bundles.len())
        //         .field("account_media", &self.account_media.len())
        //         .field("tips", &self.tips)
        //         .field("stories", &self.stories.unwrap_or(vec![]).len())
        //         .finish()
        // }
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