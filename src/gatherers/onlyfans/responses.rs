use super::structs::*;
use serde::{Deserialize, Serialize};

// pub(super) type AccountsResponse = OnlyFansResponse<Vec<Account>>;
pub(super) type SubscriptionResponse = Vec<Subscription>;
pub(super) type ValidationResponse = Me;
// pub(super) type PostsResponse = OnlyFansResponse<PostsInner>;
