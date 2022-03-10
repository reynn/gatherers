//! Gatherer Modifiers

pub trait Like {
    fn like_post(post_id: u64);
    fn like_msg_from_user(msg_id: u64);
    fn like_media_file(media_id: u64);
}
