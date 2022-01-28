use crate::structs::User;
use gatherer_core::gatherers::{structs::DateTime, Transaction};
use gatherer_core::{
    gatherers::{Gatherer, Media, Subscription, SubscriptionCost, SubscriptionName},
    Result,
};
use std::future::Future;
// use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use url::*;

#[async_trait::async_trait]
impl Gatherer for crate::OnlyFans {
    async fn gather_subscriptions(&self) -> Result<Vec<Subscription>> {
        match self.get_subscriptions().await {
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
                        match post_media.try_into() {
                            Ok(valid_media) => {
                                let mut valid_media: gatherer_core::gatherers::Media = valid_media;
                                // If the post has a cost and it has been opened than we have paid for it
                                valid_media.paid = if let Some(price) = post.price {
                                    post.is_opened
                                } else {
                                    false
                                };
                                media.push(valid_media)
                            }
                            Err(invalid_media) => log::debug!(
                                "Failed to get media from user post. {:?}",
                                invalid_media
                            ),
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
                        match msg_media.try_into() {
                            Ok(valid_media) => {
                                let mut valid_media: gatherer_core::gatherers::Media = valid_media;
                                // if the post is not free, and you cannot purchase it but it is opened than you have paid for this content
                                valid_media.paid =
                                    !msg.is_free && !msg.can_purchase && msg.is_opened;
                                media.push(valid_media)
                            }
                            Err(invalid_media) => {
                                log::debug!("Failed to get media from msg. {:?}", invalid_media)
                            }
                        }
                    }
                }
                Ok(media)
            }
            Err(messages_err) => {
                Err(format!("Failed to get messages for user: {}", sub.name.username).into())
            }
        }
    }

    async fn gather_media_from_stories(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_user_stories(&sub.id).await {
            Ok(user_stories) => {
                let mut media = vec![];
                for story in user_stories {
                    for story_media in story.media {
                        match story_media.try_into() {
                            Ok(valid_media) => media.push(valid_media),
                            Err(invalid_media) => log::debug!(
                                "Failed to get media from user story. {:?}",
                                invalid_media
                            ),
                        }
                    }
                }
                Ok(media)
            }
            Err(stories_err) => Err(stories_err),
        }
    }

    async fn gather_paid_content(&self) -> Result<Vec<Media>> {
        log::error!("Not yet implemented for {}", self.name());
        Ok(Vec::new())
    }

    async fn gather_transaction_details(&self, user_names: &[String]) -> Result<Vec<Transaction>> {
        match self.get_transactions().await {
            Ok(transactions) => Ok(transactions
                .into_iter()
                .map(|of_transaction| {
                    let date_time =
                        chrono::DateTime::parse_from_rfc3339(&of_transaction.created_at).unwrap();
                    Transaction {
                        total_amount: of_transaction.amount
                            + of_transaction.vat_amount.unwrap_or_default(),
                        user_name: match of_transaction.user {
                            None => "unknown".into(),
                            Some(transaction_user) => transaction_user.username,
                        },
                        date: DateTime(Some(date_time.into())),
                        description: Some(of_transaction.description),
                    }
                })
                .collect()),
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

impl TryFrom<crate::structs::Media> for Media {
    type Error = Box<dyn std::error::Error>;

    fn try_from(of_media: crate::structs::Media) -> std::result::Result<Self, Self::Error> {
        let of_media = of_media;
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
            return Err(format!(
                "Unable to determine the file source for OF Media {:?}",
                &of_media
            )
            .into());
        };

        let url = possible_media_link.clone().unwrap();

        let file_name: String = {
            if let Ok(url) = Url::parse(&url) {
                if let Some(segments) = url.path_segments() {
                    segments.into_iter().last().unwrap_or_default().to_string()
                } else {
                    return Err(format!("No path segments available in URL {}", url).into());
                }
            } else {
                return Err(format!("Unable to determine file name from URL: {:?}", &url).into());
            }
        };

        Ok(Self {
            file_name,
            paid: false,
            mime_type: mime_type.to_string(),
            url,
        })
    }
}
