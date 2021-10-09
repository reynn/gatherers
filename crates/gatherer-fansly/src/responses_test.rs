#[test]
fn response_accounts_test() {}

#[test]
fn response_subscription_test() {}

#[test]
fn response_status_test() {}

#[test]
fn response_posts_test() {}

#[test]
fn response_media_test() {}

#[test]
fn response_media_bundle_test() {}

#[test]
fn response_followed_accounts_test() {}

#[test]
fn response_message_groups_test() {}

macro_rules! response_test {
    ($name:ident) => {
        #[test]
        fn $name() {}
    };
}

response_test! {group_messages}
