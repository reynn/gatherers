use eyre::{bail, eyre};
use {
    crate::{
        structs::{self, MessageGroup},
        Fansly,
    },
    async_trait::async_trait,
    gatherer_core::{
        gatherers::{Gatherer, Media, Subscription, Transaction},
        Result,
    },
    std::path::Path,
};

#[async_trait]
impl Gatherer for Fansly {
    async fn gather_subscriptions(&self) -> Result<Vec<Subscription>> {
        self.get_account_subscriptions().await
    }

    async fn gather_media_from_bundles(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        let mut bundle_media = Vec::new();

        let account = self.get_user_accounts_by_ids(&[sub.id.clone()]).await?;
        let account = account.response.get(0).unwrap();

        if let Some(avatar) = account.avatar.clone() {
            log::debug!("Adding avatar for {}", sub.name);
            if let Some(media) = crate::fansly_media_to_gatherers_media(avatar, &sub.name.username)
            {
                bundle_media.push(media)
            }
        };

        if let Some(banner) = account.banner.clone() {
            log::debug!("Adding banner for {}", sub.name);
            if let Some(media) = crate::fansly_media_to_gatherers_media(banner, &sub.name.username)
            {
                bundle_media.push(media)
            }
        };

        Ok(bundle_media)
    }

    async fn gather_media_from_posts(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        log::debug!("Getting posts for user: {}", sub.id);

        let posts = self.get_posts_by_user_id(&sub.id).await?;
        log::debug!("Found posts from user: {}. {:?}", &sub.name.username, posts);
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
                                .map(|a| a.content_id.clone().unwrap_or_default())
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect::<Vec<String>>();
                media_ids
            })
            .collect();

        log::debug!("Collecting media bundles");
        let bundle_ids: Vec<String> = posts
            .iter()
            .flat_map(|post| {
                if let Some(bundles) = &post.account_media_bundles {
                    bundles
                        .iter()
                        .map(|bundle| bundle.id.to_string())
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            })
            .collect();
        let bundles = self.get_media_bundles_by_ids(&bundle_ids).await?;
        let mut bundle_media_ids: Vec<String> = bundles
            .into_iter()
            .flat_map(|bundle| bundle.account_media_ids.to_vec())
            .collect();
        media_ids.append(&mut bundle_media_ids);

        let mut account_media_ids: Vec<String> = posts
            .iter()
            .flat_map(|post| {
                if let Some(account_medias) = &post.account_media {
                    account_medias
                        .iter()
                        .map(|a_media| a_media.id.to_string())
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            })
            .collect();
        media_ids.append(&mut account_media_ids);

        // sort the ids so they are consecutive
        media_ids.sort();
        // dedup the list in just in case
        media_ids.dedup();
        // Get all discovered media file information
        let all_media = self.get_media_by_ids(&media_ids).await?;
        log::debug!(
            "{}: Downloaded info on {} total items for {}",
            self.name(),
            all_media.len(),
            sub.name
        );
        log::debug!("all_media: {:?}", all_media);
        Ok(all_media
            .into_iter()
            .filter_map(|media| super::fansly_media_to_gatherers_media(media, &sub.name.username))
            .collect())
    }

    async fn gather_media_from_messages(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        let groups = self.get_messages_groups().await;
        match groups {
            Ok(groups) => {
                // TODO: might be a better way to do this without grabbing all of them first
                log::debug!(
                    "{}: Found {} total message threads",
                    self.name(),
                    groups.len()
                );
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
                            log::debug!(
                                "Found {} messages in thread {}",
                                thread_messages.len(),
                                thread.id
                            );
                            let mut thread_media_ids: Vec<String> = thread_messages
                                .iter()
                                .flat_map(|m| {
                                    m.attachments
                                        .iter()
                                        .map(|a| a.content_id.clone().unwrap_or_default())
                                        .collect::<Vec<_>>()
                                })
                                .collect();
                            log::debug!(
                                "Found {} media items from messages for {}",
                                thread_media_ids.len(),
                                sub.name
                            );
                            media_ids_from_messages.append(&mut thread_media_ids);
                        }
                        Err(group_message_err) => {
                            bail!(
                                "Failed to get messages for {}({}). {:#}",
                                sub.name,
                                thread.id,
                                group_message_err
                            );
                        }
                    }
                }

                let media_items = self.get_media_by_ids(&media_ids_from_messages).await;
                match media_items {
                    Ok(media) => {
                        log::debug!(
                            "Get data on {} media items from messages for user {}",
                            media.len(),
                            sub.name
                        );
                        // let media: Vec<Media> = ;
                        Ok(media
                            .into_iter()
                            .filter_map(|media| {
                                super::fansly_media_to_gatherers_media(media, &sub.name.username)
                            })
                            .collect())
                    }
                    Err(media_err) => Err(eyre!(
                        "Failed to get media details for {} items for user {}. {:?}",
                        media_ids_from_messages.len(),
                        sub.name,
                        media_err,
                    )),
                }
            }
            Err(group_err) => Err(eyre!(
                "{}: Failed to get message groups for user {}. {}",
                self.name(),
                sub.name.username,
                group_err
            )),
        }
    }

    async fn gather_media_from_stories(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_account_stories(&sub.id).await {
            Ok(user_stories) => {
                let story_content_ids: Vec<String> = user_stories
                    .into_iter()
                    .map(|s| s.content_id.unwrap_or_default())
                    .collect();
                match self.get_media_by_ids(&story_content_ids).await {
                    Ok(media) => Ok(media
                        .into_iter()
                        .filter_map(|fansly_media| {
                            match to_gatherer_media(fansly_media, &sub.name.username) {
                                Ok(link) => Some(link),
                                Err(_) => None,
                            }
                        })
                        .collect()),
                    Err(media_err) => Err(media_err),
                }
            }
            Err(stories_err) => Err(stories_err),
        }
    }

    async fn gather_paid_content(&self) -> Result<Vec<Media>> {
        let purchased = self.get_purchased_content().await?;
        let media_ids: Vec<String> = purchased
            .iter()
            .map(|media| media.account_media_id.to_string())
            .collect();
        let media = self.get_media_by_ids(&media_ids).await?;
        Ok(media
            .into_iter()
            .filter_map(|media| super::fansly_media_to_gatherers_media(media, ""))
            .collect())
    }

    async fn gather_transaction_details(&self, user_names: &[String]) -> Result<Vec<Transaction>> {
        match self.get_transaction_details(user_names).await {
            Ok(transactions) => {
                let user_ids = transactions
                    .iter()
                    .filter_map(|t| t.receiver_id.clone())
                    .collect::<Vec<_>>();
                let user_accounts: crate::responses::AccountsResponse =
                    self.get_user_accounts_by_ids(&user_ids).await?;
                let mut all_transaction_details = Vec::new();
                for transaction in transactions
                    .into_iter()
                    .filter(|transaction| transaction.status == 2)
                {
                    if let Some(receiver_id) = transaction.receiver_id {
                        if let Some(account) = user_accounts
                            .response
                            .iter()
                            .find(|account| account.id == receiver_id)
                        {
                            let amount = transaction.amount as f64 / 1000.;
                            all_transaction_details.push(Transaction {
                                total_amount: amount,
                                user_name: account.username.clone(),
                                date: Default::default(),
                                description: None,
                            })
                        }
                    }
                }
                Ok(all_transaction_details)
            }
            Err(transaction_err) => Err(transaction_err),
        }
    }

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }

    fn name(&self) -> &'static str {
        "fansly"
    }
}

pub fn to_gatherer_media(
    fansly_media: structs::Media,
    sub_name: &'_ str,
) -> Result<gatherer_core::gatherers::Media> {
    if let Some(details) = fansly_media.details {
        if details.locations.is_empty() {
            return Err(eyre!("Content not available: {:?}", details));
        }
        let filename = details.file_name.unwrap_or_default();
        let original_file_path = Path::new(&filename);
        // debug!("The original upload file name was {:?}", original_file_path);
        let mut file_name = fansly_media.id.clone();
        file_name += &original_file_path
            .extension()
            .map(|ext| format!(".{}", &ext.to_str().unwrap_or_default()))
            .unwrap_or_default();
        Ok(Media {
            file_name,
            url: details.locations[0].location.to_string(),
            mime_type: details.mimetype,
            paid: fansly_media.purchased,
            user_name: sub_name.to_string(),
        })
    } else {
        Err(eyre!("Content not available: {:?}", fansly_media))
    }
}
