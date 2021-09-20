use std::{convert::TryInto, time::Duration};

use gatherer_core::{
    gatherers::{Gatherer, GathererErrors, Media, Subscription},
    AsyncResult,
};
use tracing::{debug, error, info};

use crate::{structs::MessageGroup, Fansly};

#[async_trait::async_trait]
impl Gatherer for Fansly {
    async fn gather_subscriptions(&self) -> AsyncResult<Vec<Subscription>> {
        self.get_account_subscriptions().await
    }

    async fn gather_media_from_posts(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        info!("Getting posts for user: {}", sub.id);

        let posts = self.get_posts_by_user_id(&sub.id).await?;
        let mut media_ids: Vec<String> = posts
            .iter()
            .flat_map(|p| {
                let media_ids = p
                    .posts
                    .iter()
                    .flat_map(|p| {
                        p.iter().flat_map(|post| {
                            post.attachments
                                .iter()
                                .map(|a| a.content_id.to_string())
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect::<Vec<String>>();
                media_ids
            })
            .collect();

        info!("Collecting media bundles");
        let bundle_ids: Vec<String> = posts
            .iter()
            .flat_map(|post| {
                post.account_media_bundles
                    .iter()
                    .map(|bundle| bundle.id.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();
        debug!("Bundle ids: {:?}", &bundle_ids);
        let bundles = self.get_media_bundles_by_ids(&bundle_ids).await?;
        let mut bundle_media_ids: Vec<String> = bundles
            .into_iter()
            .flat_map(|bundle| bundle.account_media_ids.to_vec())
            .collect();
        media_ids.append(&mut bundle_media_ids);

        let mut account_media_ids: Vec<String> = posts
            .iter()
            .flat_map(|post| {
                post.account_media
                    .iter()
                    .map(|a_media| a_media.id.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();
        media_ids.append(&mut account_media_ids);

        // sort the ids so they are consecutive
        media_ids.sort();
        // dedup the list in just in case
        media_ids.dedup();
        // Get all discovered media file information
        let all_media = self.get_media_by_ids(&media_ids).await?;
        info!(
            "Downloaded info on {} total items for {}",
            all_media.len(),
            sub.name
        );
        debug!("all_media: {:?}", all_media);
        Ok(all_media
            .into_iter()
            .filter_map(|fansly_media| match fansly_media.try_into() {
                Ok(link) => Some(link),
                Err(e) => None,
            })
            .collect())
    }

    async fn gather_media_from_messages(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        let groups = self.get_messages_groups().await;
        match groups {
            Ok(groups) => {
                // TODO: might be a better way to do this without grabbing all of them first
                debug!("Found {} total message threads from fansly", groups.len());
                let subscription_threads: Vec<MessageGroup> = groups
                    .into_iter()
                    .filter(|group| {
                        group
                            .users
                            .iter()
                            .any(|g_user| sub.id[..] == g_user.user_id)
                    })
                    .collect();
                let mut media_ids_from_messages = Vec::new();
                for thread in subscription_threads {
                    let messages = self.get_all_messages_from_group(&thread.id).await;
                    match messages {
                        Ok(thread_messages) => {
                            debug!(
                                "Found {} messages in thread {}",
                                thread_messages.len(),
                                thread.id
                            );
                            let mut thread_media_ids: Vec<String> = thread_messages
                                .iter()
                                .flat_map(|m| {
                                    m.attachments
                                        .iter()
                                        .map(|a| a.content_id.to_string())
                                        .collect::<Vec<_>>()
                                })
                                .collect();
                            info!(
                                "Found {} media items from messages for {}",
                                thread_media_ids.len(),
                                sub.name
                            );
                            media_ids_from_messages.append(&mut thread_media_ids);
                        }
                        Err(group_message_err) => {
                            return Err(format!(
                                "Failed to get messages for {}({}). {:#}",
                                sub.name, thread.id, group_message_err
                            )
                            .into())
                        }
                    }
                }

                let media_items = self.get_media_by_ids(&media_ids_from_messages).await;
                match media_items {
                    Ok(media) => {
                        debug!(
                            "Get data on {} media items from messages for user {}",
                            media.len(),
                            sub.name
                        );
                        // let media: Vec<Media> = ;
                        Ok(media
                            .into_iter()
                            .filter_map(|fansly_media| match fansly_media.try_into() {
                                Ok(link) => Some(link),
                                Err(e) => None,
                            })
                            .collect())
                    }
                    Err(media_err) => Err(format!(
                        "Failed to get media details for {} items for user {}. {:?}",
                        media_ids_from_messages.len(),
                        sub.name,
                        media_err,
                    )
                    .into()),
                }

                // Ok(messages_media)
            }
            Err(group_err) => {
                Err(format!("Failed to get message groups from Fansly. {:?}", group_err).into())
            }
        }
    }

    async fn gather_media_from_stories(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_bundles(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        let mut media = Vec::new();

        let account = self.get_user_accounts_by_ids(&[sub.id.clone()]).await?;
        let account = account.response.get(0).unwrap();

        if let Some(avatar) = &account.avatar {
            debug!("Adding avatar for {}", sub.name);
            media.push(Media {
                file_name: avatar.filename.to_string(),
                mime_type: avatar.mimetype.to_string(),
                url: avatar
                    .locations
                    .get(0)
                    .map(|f| f.location.to_string())
                    .unwrap_or_default(),
                paid: false,
            })
        };

        if let Some(banner) = &account.banner {
            debug!("Adding banner for {}", sub.name);
            media.push(Media {
                file_name: banner.filename.to_string(),
                mime_type: banner.mimetype.to_string(),
                url: banner
                    .locations
                    .get(0)
                    .map(|f| f.location.to_string())
                    .unwrap_or_default(),
                paid: false,
            })
        };

        Ok(media)
    }

    fn name(&self) -> &'static str {
        "Fansly"
    }

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }
}
