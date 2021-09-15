use gatherer_core::{
    gatherers::{Gatherer, GathererErrors, Media, Subscription},
    AsyncResult,
};

use crate::Fansly;

#[async_trait::async_trait]
impl<'a> Gatherer for Fansly<'a> {
    async fn gather_subscriptions(&self) -> AsyncResult<Vec<Subscription>> {
        self.get_account_subscriptions().await
    }

    async fn gather_media_from_posts(&self, sub: &'_ Subscription) -> AsyncResult<Vec<Media>> {
        // let posts = self.get_posts_by_user_id(&sub.id, 0).await?;
        // let post_ids: Vec<&str> = posts.iter().map(|p| p.id.as_str()).collect();
        // tracing::error!("Found {} for user {}", post_ids.len(), sub.username);
        Ok(Vec::new())
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

    fn name(&self) -> &'static str {
        "fansly"
    }

    fn is_enabled(&self) -> bool {
        self.conf.enabled
    }
}
