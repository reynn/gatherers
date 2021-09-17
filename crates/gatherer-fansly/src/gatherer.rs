use std::convert::TryInto;

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
        let posts = self.get_posts_by_user_id(&sub.id).await?;
        println!("Found {} posts for {}", &posts.len(), sub.username);
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
        println!("Media files found from posts: {}", media_ids.len());

        let bundle_ids: Vec<String> = posts
            .iter()
            .flat_map(|post| {
                post.account_media_bundles
                    .iter()
                    .map(|bundle| bundle.id.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();

        info!("Bundle ids: {:?}", &bundle_ids);
        let bundles = self.get_media_bundles_by_ids(&bundle_ids).await?;
        let mut bundle_media_ids: Vec<String> = bundles
        .into_iter()
        .flat_map(|bundle| {
            bundle.account_media_ids.to_vec()
        })
        .collect();
        println!("Media files found from bundles: {}", bundle_media_ids.len());
        media_ids.append(&mut bundle_media_ids);
        let all_media = self.get_media_by_ids(&media_ids).await?;
        debug!("all_media: {:?}", all_media);
        // tracing::error!("Found {} for user {}", post_ids.len(), sub.username);
        Ok(all_media
            .into_iter()
            .filter_map(|fansly_media| {
                match fansly_media.details {
                    Some(details) => match details.try_into() {
                        Ok(linkk) => Some(linkk),
                        Err(e) => {
                            debug!("{}", e);
                            None
                        }
                    },
                    None => None,
                }
                // fansly_media
                //     .details
                //     .map(|details| match details.try_into() {
                //         Ok(link) => Some(link),
                //         Err(_) => todo!(),
                //     })
            })
            .collect())
    }

    async fn gather_media_from_messages(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().into(),
            feature: "stories".into(),
        }))
    }

    async fn gather_media_from_stories(&self, _sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().into(),
            feature: "messages".into(),
        }))
    }

    async fn gather_media_from_bundles(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        let mut media = Vec::new();
        // Wall
        let account = self.get_user_accounts_by_ids(&[sub.id.clone()]).await?;
        let account = account.response.get(0).unwrap();

        if let Some(avatar) = &account.avatar {
            info!("Adding avatar for {}", sub.username);
            media.push(Media {
                filename: avatar.filename.to_string(),
                mime_type: avatar.mimetype.to_string(),
                url: avatar
                    .locations
                    .get(0)
                    .map(|f| f.location.to_string())
                    .unwrap_or_default(),
            })
        };
        if let Some(banner) = &account.banner {
            info!("Adding banner for {}", sub.username);
            media.push(Media {
                filename: banner.filename.to_string(),
                mime_type: banner.mimetype.to_string(),
                url: banner
                    .locations
                    .get(0)
                    .map(|f| f.location.to_string())
                    .unwrap_or_default(),
            })
        };
        // media.append();
        // PinnedPost

        Ok(media)
    }

    fn name(&self) -> &'static str {
        "Fansly"
    }

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }
}
