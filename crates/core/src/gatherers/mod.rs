//! Gatherer
//!
//! A `Gatherer` is responsible for collect content from whatever source it chooses to imlement.
//! Initially this is designed around getting **PAID** content from subscription sites.

mod errors;
mod modifiers;
pub mod structs;

pub use self::{errors::GathererErrors, structs::*};
use crate::{
    downloaders::{BatchDownloader, Downloadable},
    Result,
};
use async_channel::Sender;
use chrono::prelude::*;
use futures::Future;
use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
};
use strum::IntoEnumIterator;

#[async_trait::async_trait]
pub trait Gatherer: std::fmt::Debug + Sync + Send {
    /// Interface with the source site to get the subscriptions of the authed user
    ///
    /// TODO: add more detail
    async fn gather_subscriptions(&self) -> Result<Vec<structs::Subscription>>;
    /// Interface with the source site to get the specified subs bundles
    ///
    /// TODO: add more detail
    async fn gather_media_from_bundles(
        &self,
        _sub: &'_ structs::Subscription,
    ) -> Result<Vec<structs::Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().to_string(),
            feature: "content_bundles".to_string(),
        }))
    }
    /// Interface with the source site to get the specified subs posts
    ///
    /// TODO: add more detail
    async fn gather_media_from_posts(
        &self,
        _sub: &'_ structs::Subscription,
    ) -> Result<Vec<structs::Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().to_string(),
            feature: "posts".to_string(),
        }))
    }
    /// Interface with the source site to get the specified subs messages
    ///
    /// TODO: add more detail
    async fn gather_media_from_messages(
        &self,
        _sub: &'_ structs::Subscription,
    ) -> Result<Vec<structs::Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().to_string(),
            feature: "messages".to_string(),
        }))
    }
    /// Interface with the source site to get the specified subs stories
    ///
    /// TODO: add more detail
    async fn gather_media_from_stories(
        &self,
        _sub: &'_ structs::Subscription,
    ) -> Result<Vec<structs::Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().to_string(),
            feature: "stories".to_string(),
        }))
    }
    /// This should grab content that the user has paid for
    ///
    /// The user would be the currently authenticated user as provided by authorization token.
    /// Is likely to cause some duplicates from posts if the user is still subscribed.
    /// This is an acceptable scenario as the Downloader should be handling this case.
    async fn gather_paid_content(&self) -> Result<Vec<structs::Media>> {
        Err(Box::new(GathererErrors::NotSupportedByGatherer {
            gatherer_name: self.name().to_string(),
            feature: "paid content".to_string(),
        }))
    }
    /// Provide whether the gatherer should be considered enabled
    fn is_enabled(&self) -> bool {
        false
    }
    /// Provide a simple name for the gatherer, is added to paths as needed
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, strum::Display, Eq, PartialEq, strum::EnumIter)]
pub enum GatherType {
    Posts,
    Messages,
    Bundles,
    Stories,
    Purchased,
}

async fn run_gatherer(info: structs::GathererInfo) -> Result<()> {
    let name = info.name;
    let gather_type = info.gather_type;
    let sub = info.subscription;
    log::info!(
        "{:>10}: Starting to gather {:8} for {:^20}",
        name,
        gather_type,
        &sub.name.username
    );
    let all_media = match gather_type {
        GatherType::Posts => info.gatherer.gather_media_from_posts(&sub),
        GatherType::Messages => info.gatherer.gather_media_from_messages(&sub),
        GatherType::Bundles => info.gatherer.gather_media_from_bundles(&sub),
        GatherType::Stories => info.gatherer.gather_media_from_stories(&sub),
        GatherType::Purchased => info.gatherer.gather_paid_content(),
        // _ => {
        //     info!("Skipped gathering {:?}", gather_type);
        //     return Ok(());
        // }
    }
    .await;

    match all_media {
        Ok(medias) => {
            let sub_path = info.base_path.join(&sub.name.username.to_ascii_lowercase());
            log::info!(
                "{:>10}: Completed gathering {:8} for {:^20}. Found [{:^4}] items",
                name,
                gather_type,
                sub.name.username,
                medias.len()
            );
            for media in medias.iter() {
                let downloadable_path =
                    info.base_path
                        .clone()
                        .join(if media.paid { "paid" } else { "free" });
                // let item = Downloadable::from_media_with_path(media, downloadable_path);
                match info
                    .downloader
                    .try_send(Downloadable::from_media_with_path(media, downloadable_path))
                {
                    Ok(_) => {
                        log::debug!("{:>12}: Sent item to download queue", name)
                    }
                    Err(send_err) => {
                        log::error!("{:>12}: Failed to send to queue. {:?}", name, send_err)
                    }
                }
            }
            Ok(())
        }
        Err(gather_err) => Err(format!(
            "{}: Failed to gather {}. Error: {:?}",
            name, gather_type, gather_err
        )
        .into()),
    }
}

pub async fn run_gatherer_for_subscriber(
    gatherer: Arc<dyn Gatherer>,
    base_path: PathBuf,
    download_tx: Sender<Downloadable>,
    sub: &'_ Subscription,
) -> Result<()> {
    // Get a custom iter over our gatherable types, filtering out unneeded values for this function
    let sub_gatherables: Vec<_> = GatherType::iter()
        .filter(|t| !t.eq(&GatherType::Purchased))
        .collect();
    for gather_type in sub_gatherables.iter() {
        let gatherer_name = gatherer.name().to_ascii_lowercase();
        // Start building output directory for our gatherer
        let base_path = base_path.join(&gatherer_name).join(&sub.name.username);
        // TODO: likely a better way to achieve something like this in rust, defaulted to go style :'(
        // This is the parameters fed into our gatherer
        let info = structs::GathererInfo {
            // Add the users name to the gatherers base path for output dir
            base_path: base_path.clone().join(&sub.name.username),
            gather_type: *gather_type,
            gatherer: gatherer.clone(),
            subscription: sub.clone(),
            downloader: download_tx.clone(),
            name: gatherer_name,
        };
    }
    Ok(())
}

pub async fn run_gatherer_for_all(
    gatherer: Arc<dyn Gatherer>,
    base_path: impl Into<PathBuf>,
    download_tx: Sender<Downloadable>,
    limits: structs::RunLimits,
    user_names: &[String],
) -> Result<()> {
    let gatherer_name = gatherer.name();
    let sub_result = gatherer.gather_subscriptions().await;
    let mut subs_tasks = Vec::new();
    match sub_result {
        Ok(subscriptions) => {
            println!(
                "{}: Found {} subscriptions",
                gatherer_name,
                subscriptions.len()
            );
            let mut subscriptions = if limits.subscriptions == 0 {
                subscriptions
            } else {
                log::debug!(
                    "{:>10}: Limiting to only {} subscriptions",
                    gatherer_name,
                    limits.subscriptions
                );
                subscriptions
                    .into_iter()
                    .take(limits.subscriptions)
                    .collect()
            };
            // If any usernames are provided treat them as a filter for the gatherer
            if !user_names.is_empty() {
                subscriptions = subscriptions
                    .into_iter()
                    .filter(|sub| user_names.contains(&sub.name.username))
                    .collect();
            }
            // Start building output directory for our gatherer
            let base_path: PathBuf = base_path.into();
            let base_path = base_path.join(gatherer.name().to_ascii_lowercase());

            // Get a custom iter over our gatherable types, filtering out unneeded values for this function
            let sub_gatherables: Vec<_> = GatherType::iter()
                .filter(|t| !t.eq(&GatherType::Purchased))
                .collect();
            // Start looping through the subscriptions found for the gatherer
            for sub in subscriptions.iter() {
                for gather_type in sub_gatherables.iter() {
                    // TODO: likely a better way to achieve something like this in rust, defaulted to go style :'(
                    // This is the parameters fed into our gatherer
                    let info = structs::GathererInfo {
                        // Add the users name to the gatherers base path for output dir
                        base_path: base_path.clone().join(&sub.name.username),
                        gather_type: *gather_type,
                        gatherer: gatherer.clone(),
                        subscription: sub.clone(),
                        downloader: download_tx.clone(),
                        name: gatherer_name.into(),
                    };
                    subs_tasks.push(run_gatherer(info));
                }
            }
        }
        Err(sub_err) => return Err(sub_err),
    }

    futures::future::join_all(subs_tasks).await;
    log::info!(
        "{}: Completed gathering everything for all subs",
        gatherer_name
    );

    Ok(())
}
