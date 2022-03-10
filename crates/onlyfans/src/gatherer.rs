use gatherer_core::{
    gatherers::{structs::DateTime, Gatherer, Media, Subscription, Transaction},
    Result,
};
use std::collections::HashMap;
use url::*;

#[async_trait::async_trait]
impl Gatherer for crate::OnlyFans {
    async fn gather_subscriptions(&self) -> Result<Vec<Subscription>> {
        match self.get_subscriptions(Some("active")).await {
            Ok(subs) => Ok(subs.into_iter().map(|of_sub| of_sub.into()).collect()),
            Err(subs_err) => Err(subs_err),
        }
    }

    async fn gather_media_from_bundles(&self, _sub: &'_ Subscription) -> Result<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_posts(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_user_posts(&sub.id).await {
            Ok(user_posts) => {
                let mut media = Vec::new();

                for post in user_posts {
                    for post_media in post.media {
                        match to_gatherer_media(&post_media, &sub.name.username) {
                            Some(valid_media) => {
                                let mut valid_media: gatherer_core::gatherers::Media = valid_media;
                                // If the post has a cost and it has been opened than we have paid for it
                                valid_media.paid = if let Some(price) = post.price {
                                    post.is_opened && (price > 0.)
                                } else {
                                    false
                                };
                                media.push(valid_media)
                            }
                            None => {
                                log::debug!("Failed to get media from user post. {}", post_media.id)
                            }
                        }
                    }
                }

                Ok(media)
            }
            Err(posts_err) => Err(format!(
                "Failed to get posts for user {}. {:?}",
                sub.name.username, posts_err
            )
            .into()),
        }
    }

    async fn gather_media_from_messages(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_user_messages(&sub.id).await {
            Ok(user_messages) => {
                let mut media = Vec::new();
                for msg in user_messages {
                    for msg_media in msg.media {
                        match to_gatherer_media(&msg_media, &sub.name.username) {
                            Some(valid_media) => {
                                let mut valid_media: gatherer_core::gatherers::Media = valid_media;
                                // if the post is not free, and you cannot purchase it but it is opened than you have paid for this content
                                valid_media.paid = !msg.is_free
                                    && !msg.can_purchase.unwrap_or_default()
                                    && msg.is_opened;
                                media.push(valid_media)
                            }
                            None => {
                                log::debug!("Failed to get media from msg. {}", msg.id)
                            }
                        }
                    }
                }
                Ok(media)
            }
            Err(messages_err) => Err(format!(
                "Failed to get messages for user: {}. {:?}",
                sub.name.username, messages_err
            )
            .into()),
        }
    }

    async fn gather_media_from_stories(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_user_stories(&sub.id).await {
            Ok(user_stories) => {
                let mut media = vec![];
                for story in user_stories {
                    for story_media in story.media {
                        match to_gatherer_media(&story_media, &sub.name.username) {
                            Some(valid_media) => media.push(valid_media),
                            None => {
                                log::debug!("Failed to get media from user story. {}", story.id)
                            }
                        }
                    }
                }
                Ok(media)
            }
            Err(stories_err) => Err(stories_err),
        }
    }

    async fn gather_paid_content(&self) -> Result<Vec<Media>> {
        let mut known_users: HashMap<i64, String> = HashMap::new();
        match self.get_paid_content().await {
            Ok(paid_content) => {
                let mut results = Vec::new();
                for item in paid_content.into_iter() {
                    let user_id: i64 = match &item.response_type[..] {
                        "message" => {
                            if let Some(from_user) = &item.from_user {
                                from_user.id
                            } else {
                                0
                            }
                        }
                        "post" => {
                            if let Some(author) = &item.author {
                                author.id
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    };
                    let user_name = if let Some(user_name) = known_users.get(&user_id) {
                        String::from(user_name)
                    } else {
                        match self.get_users_by_id(&[user_id]).await {
                            Ok(users) => {
                                if !users.is_empty() {
                                    known_users.insert(user_id, String::from(&users[0].username));
                                    String::from(&users[0].username)
                                } else {
                                    String::from("unknown user")
                                }
                            }
                            Err(user_err) => {
                                log::debug!(
                                    "Error getting user by id [{}]. {:?}",
                                    user_id,
                                    user_err
                                );
                                String::from("unknown user")
                            }
                        }
                    };
                    for media in item.media.into_iter() {
                        if let Some(mut purchased_media) = to_gatherer_media(&media, &user_name) {
                            purchased_media.paid = true;
                            results.push(purchased_media);
                        }
                    }
                }
                // let mut results = paid_content
                //     .into_iter()
                //     .flat_map(|item| {
                //         let user_id: i64 = match &item.response_type[..] {
                //             "message" => {
                //                 if let Some(from_user) = item.from_user {
                //                     from_user.id
                //                 } else {
                //                     0
                //                 }
                //             }
                //             "post" => {
                //                 if let Some(author) = item.author {
                //                     0
                //                 } else {
                //                     0
                //                 }
                //             }
                //             _ => 0,
                //         };
                //         let user_name = if let Some(user_name) = users.get(&user_id) {
                //             user_name
                //         } else {
                //             match self.get_users_by_id(&[user_id]).await {
                //                 Ok(users) => {}
                //                 Err(user_err) => {}
                //             }
                //         };
                //         item.media
                //             .into_iter()
                //             .filter_map(|purchased_media| {
                //                 to_gatherer_media(&purchased_media, user_name)
                //             })
                //             .collect::<Vec<_>>()
                //     })
                //     .collect::<Vec<_>>();
                // set all of these results to ensure paid flag is set properly
                Ok(results)
            }
            Err(paid_content_err) => Err(paid_content_err),
        }
    }

    async fn gather_transaction_details(&self, _: &[String]) -> Result<Vec<Transaction>> {
        match self.get_transactions().await {
            Ok(transactions) => {
                Ok(transactions
                    .into_iter()
                    .filter_map(|of_transaction| {
                        let date = if let Some(created_at) = &of_transaction.created_at {
                            DateTime(Some(
                                chrono::DateTime::parse_from_rfc3339(created_at)
                                    .unwrap()
                                    .into(),
                            ))
                        } else {
                            DateTime(None)
                        };
                        if &of_transaction.status.unwrap_or_default() == "done" {
                            Some(Transaction {
                                // Taxes and requested amount are separate, combine them for our total
                                total_amount: of_transaction.amount
                                    + of_transaction.vat_amount.unwrap_or_default(),
                                user_name: match of_transaction.user {
                                    None => "unknown".into(),
                                    Some(transaction_user) => transaction_user.username,
                                },
                                date,
                                description: Some(of_transaction.description),
                            })
                        } else {
                            None
                        }
                    })
                    .collect())
            }
            Err(transaction_err) => Err(transaction_err),
        }
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn name(&self) -> &'static str {
        "onlyfans"
    }
}

pub(crate) fn to_gatherer_media(
    of_media: &'_ crate::structs::Media,
    of_sub_name: &'_ str,
) -> Option<Media> {
    let mime_type = match of_media.media_type.as_str() {
        "photo" => "image/jpeg",
        "video" | "gif" => "video/mp4",
        _ => "unknown",
    };
    let mut possible_media_link: &Option<String> = &of_media.full;
    if possible_media_link.is_none() {
        if let Some(media_info) = &of_media.info {
            if let Some(info_source) = &media_info.source {
                possible_media_link = &info_source.source;
            }
        }
    };
    if possible_media_link.is_none() {
        if let Some(media_source) = &of_media.source {
            possible_media_link = &media_source.source;
        }
    };
    if possible_media_link.is_none() {
        if let Some(files) = &of_media.files {
            if let Some(file_source) = &files.source {
                possible_media_link = &file_source.url;
            }
        }
    };
    if possible_media_link.is_none() {
        log::debug!(
            "Unable to determine the file source for OF Media {:?}",
            &of_media
        );
        return None;
    };

    let url = possible_media_link.clone().unwrap();

    let file_name: String = {
        if let Ok(url) = Url::parse(&url) {
            if let Some(segments) = url.path_segments() {
                segments.into_iter().last().unwrap_or_default().to_string()
            } else {
                log::debug!("No path segments available in URL {}", url);
                return None;
            }
        } else {
            log::debug!("Unable to determine file name from URL: {:?}", &url);
            return None;
        }
    };

    Some(Media {
        file_name,
        paid: false,
        mime_type: mime_type.to_string(),
        url,
        user_name: of_sub_name.to_string(),
    })
}
