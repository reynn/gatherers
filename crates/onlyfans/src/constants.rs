pub const BASE_URL: &str = "https://onlyfans.com";
pub const INIT_URL: &str = "/api2/v2/init";
pub const DC_DYNAMIC_RULE: &str =
    "https://raw.githubusercontent.com/DATAHOARDERS/dynamic-rules/main/onlyfans.json";
pub const SUBSCRIPTIONS_URL: &str =
    "/api2/v2/subscriptions/subscribes?limit={}&offset={}&type=active";
pub const LISTS_URL: &str = "/api2/v2/lists?limit=100&offset=0";
pub const LISTS_USERS_URL: &str = "/api2/v2/lists/{}/users?limit={}&offset={}&query=";
pub const LIST_CHATS_URL: &str = "/api2/v2/chats?limit={}&offset={}&order=desc";
pub const POST_BY_ID_URL: &str = "/api2/v2/posts/{}";
pub const MESSAGE_BY_ID_URL: &str = "/api2/v2/chats/{}/messages?limit=10&offset=0&firstId={}&order=desc&skip_users=all&skip_users_dups=1";
pub const SEARCH_CHAT_URL: &str = "/api2/v2/chats/{}/messages/search?query={}";
pub const MESSAGE_URL: &str = "/api2/v2/chats/{}/messages?limit={}&offset={}&order=desc";
pub const SEARCH_MESSAGES_URL: &str =
    "/api2/v2/chats/{}?limit=10&offset=0&filter=&order=activity&query={}";
pub const MASS_MESSAGES_URL: &str =
    "/api2/v2/messages/queue/stats?limit=100&offset=0&format=infinite";
pub const STORIES_URL: &str = "/api2/v2/users/{}/stories?limit=100&offset=0&order=desc";
pub const LIST_HIGHLIGHTS_URL: &str =
    "/api2/v2/users/{}/stories/highlights?limit=100&offset=0&order=desc";
pub const HIGHLIGHT_URL: &str = "/api2/v2/stories/highlights/{}";
pub const POST_URL: &str =
    "/api2/v2/users/{}/posts?limit={}&offset={}&order=publish_date_desc&skip_users_dups=0";
pub const ARCHIVED_POSTS_URL: &str =
    "/api2/v2/users/{}/posts/archived?limit={}&offset={}&order=publish_date_desc";
pub const ARCHIVED_STORIES_URL: &str =
    "/api2/v2/stories/archive/?limit=100&offset=0&order=publish_date_desc";
pub const PAID_URL: &str = "/api2/v2/posts/paid?{}&offset={}";
pub const PAY_URL: &str = "/api2/v2/payments/pay";
pub const SUBSCRIBE_URL: &str = "/api2/v2/users/{}/subscribe";
pub const LIKE_URL: &str = "/api2/v2/{}/{}/like";
pub const FAVORITE_URL: &str = "/api2/v2/{}/{}/favorites/{}";
pub const TRANSACTIONS_URL: &str = "/api2/v2/payments/all/transactions?limit=10&offset=0";
pub const ME_URL: &str = "/api2/v2/users/me";
pub const USERS_URL: &str = "/api2/v2/users/";
pub const SUBS_URL: &str = "/api2/v2/subscriptions/subscribes";
pub const MFA_URL: &str = "/api2/v2/users/otp/check";
