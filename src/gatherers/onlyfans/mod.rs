pub use serde::{Deserialize, Serialize};

use crate::gatherers::{errors::GathererErrors, Gatherer};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OnlyFansConfig {
    pub(crate) enabled: bool,
    pub(crate) session_token: String,
    pub(crate) user_agent: String,
    pub(crate) app_token: String,
}

#[derive(Debug)]
pub struct OnlyFans<'o> {
    config: &'o OnlyFansConfig,
}

impl<'o> OnlyFans<'o> {
    pub fn new(conf: &'_ OnlyFansConfig) -> super::Result<Self> {
        Err(GathererErrors::NotSupportedByGatherer {
            gatherer_name: "onlyfans".into(),
            feature: "anything!".into(),
        })
    }
}

impl<'o> Gatherer for OnlyFans<'o> {
    fn gather_subscriptions(&self) -> super::Result<Vec<super::Subscription>> {
        todo!()
    }

    fn gather_posts(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Post>> {
        todo!()
    }

    fn gather_messages(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Message>> {
        todo!()
    }

    fn gather_stories(&self, _sub: &'_ super::Subscription) -> super::Result<Vec<super::Story>> {
        todo!()
    }

    fn name(&self) -> &'static str {
        "onlyfans"
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
