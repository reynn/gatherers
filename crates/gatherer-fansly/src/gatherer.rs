use std::{convert::TryInto, time::Duration};

use gatherer_core::{
    gatherers::{Gatherer, GathererErrors, Media, Subscription},
    AsyncResult,
};
use tracing::{debug, info};

use crate::Fansly;

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
                        p.attachments
                            .iter()
                            .map(|a| a.content_id.to_string())
                            .collect::<Vec<_>>()
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

    async fn gather_media_from_messages(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_stories(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_bundles(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        let mut media = Vec::new();

        let account = self.get_user_accounts_by_ids(&[sub.id.clone()]).await?;
        let account = account.response.get(0).unwrap();

        if let Some(avatar) = &account.avatar {
            info!("Adding avatar for {}", sub.name);
            media.push(Media {
                filename: avatar.filename.to_string(),
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
            info!("Adding banner for {}", sub.name);
            media.push(Media {
                filename: banner.filename.to_string(),
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
