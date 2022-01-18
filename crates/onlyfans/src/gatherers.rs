use gatherer_core::{
    gatherers::{Gatherer, Media, Subscription, SubscriptionCost, SubscriptionName},
    Result,
};

#[async_trait::async_trait]
impl Gatherer for crate::OnlyFans {
    async fn gather_subscriptions(&self) -> Result<Vec<Subscription>> {
        match self.get_subscriptions().await {
            Ok(subs) => Ok(subs.into_iter().map(|of_sub| of_sub.into()).collect()),
            Err(subs_err) => Err(subs_err),
        }
    }

    async fn gather_media_from_posts(&self, sub: &'_ Subscription) -> Result<Vec<Media>> {
        match self.get_user_posts(&sub.id).await {
            Ok(user_posts) => Ok(user_posts
                .into_iter()
                .flat_map(|post| {
                    post.media
                        .into_iter()
                        .map(|post_media| post_media.into())
                        .collect::<Vec<_>>()
                })
                .collect()),
            Err(posts_err) => Err(format!("Failed to get posts for user {}", &sub.id).into()),
        }
        // todo!()
    }

    async fn gather_media_from_messages(&self, _sub: &'_ Subscription) -> Result<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_stories(&self, _sub: &'_ Subscription) -> Result<Vec<Media>> {
        Ok(Vec::new())
    }

    async fn gather_media_from_bundles(&self, _sub: &'_ Subscription) -> Result<Vec<Media>> {
        Ok(Vec::new())
    }

    fn name(&self) -> &'static str {
        "onlyfans"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

impl From<crate::structs::Media> for Media {
    fn from(of_media: crate::structs::Media) -> Self {
        let mut file_name = of_media.id.to_string();
        let mime_type = match of_media.media_type.as_str() {
            "photo" => {
                file_name += ".jpg";
                "image/jpeg".to_string()
            }
            "video" => {
                file_name += ".mp4";
                "video/mp4".to_string()
            }
            _ => "unknown".to_string(),
        };
        Media {
            file_name,
            paid: false,
            mime_type,
            url: of_media.source.source,
        }
    }
}
