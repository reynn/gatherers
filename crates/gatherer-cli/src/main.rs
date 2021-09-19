// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod cli;
mod config;

use cli::*;
use config::{Config, ConfigErrors};
use gatherer_core::{
    directories::Directories,
    downloaders::*,
    gatherers::{self, Gatherer, Media},
    AsyncResult,
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::exit,
    sync::Arc,
};
use tabled::{Style, Table};
use tokio::join;
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => println!("Gathering completed"),
        Err(err) => println!("Gatherers has encountered an error {:?}", err),
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn run() -> Result<()> {
    // This parses all incoming arguments from stdin based on user input.
    // Use this as a starting point to configure the overall app behaviour
    let cli = Cli::new();

    // Setup logging, if the verbose flag is provided provided more detailed output
    init_logging(&cli);

    // If the user provided a config file path use that, we fall back to the default if not
    let cfg_path = if let Some(path) = cli.config_file_path {
        path
    } else {
        Directories::new().get_default_config_dir()
    };

    // Load the app config defaults with user options applied
    let config = match Config::load(&cfg_path) {
        Ok(loaded) => {
            debug!("Successfully loaded config {:?}", &loaded);
            loaded
        }
        Err(load_err) => {
            error!("Failed to load config: {}", load_err);
            match load_err {
                ConfigErrors::LoadConfigFailure(path, err) => {
                    error!("Failed to load config from {:?}. Error {}", path, err);
                    exit(1);
                }
                ConfigErrors::FromToml(e) => {
                    panic!("Unable to get config from current TOMOL file: {}", e)
                }
                ConfigErrors::ToToml(e) => {
                    error!("Failed to convert config to TOML: {}", e)
                }
                ConfigErrors::IoError(e) => {
                    error!("Failed to read or write file {}", e);
                    // exit(1)
                }
            };
            Arc::new(Config::default())
        }
    };

    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();

    #[cfg(feature = "fansly")]
    add_fansly_gatherer(&config, &mut gatherers).await?;

    if gatherers.is_empty() {
        println!("No gatherers to run");
        exit(0);
    }

    // Initialize our downloader
    let downloader = Downloader::new(&config.downloaders);

    let downloader_channel = downloader.sender.clone();
    let downloads_directory = if let Some(cli_download_dir) = &cli.download_to {
        cli_download_dir.to_owned()
    } else if let Some(config_download_dir) = &config.downloaders.storage_dir {
        config_download_dir.to_owned()
    } else {
        Path::new(".").join("downloads").to_path_buf()
    };
    // Go through all gatherers and run through a sequence of requests to scrape data from the provider APIs
    for gatherer in gatherers.into_iter() {
        let gatherer_name = gatherer.name();
        let config = Arc::clone(&config);
        println!("Getting subscriptions for {}", gatherer_name);
        let gatherer_downloads_directory = downloads_directory
            .clone()
            .join(gatherer_name.to_lowercase());
        // Get a list of subscriptions
        let downloader = downloader_channel.clone();
        // tokio::spawn(async move {
        match gatherer.gather_subscriptions().await {
            Ok(subs) => {
                if subs.is_empty() {
                    return Err(format!("No subscriptions for {}", gatherer_name).into());
                };
                // let subs = subs.into_iter().take(5).collect::<Vec<_>>();
                println!(
                    "Found {} subs for {}\n{}",
                    subs.len(),
                    gatherer_name,
                    Table::new(&subs).with(Style::pseudo_clean())
                );
                info!(
                    "Found subscription ids: {}",
                    subs.iter()
                        .map(|sub| &sub.id[..])
                        .collect::<Vec<&str>>()
                        .join(",")
                );
                for sub in subs {
                    let subscription_downloads_directory =
                        gatherer_downloads_directory.join(&sub.name.username.to_lowercase());
                    println!(
                        "{} is getting data for subscriber: {}",
                        gatherer_name, sub.name
                    );
                    let sub_gatherer = gatherer.clone();
                    let downloader = downloader.clone();
                    tokio::spawn(async move {
                        let gatherer_name = sub_gatherer.name();
                        match gatherers::run_gatherer_for_subscription(sub_gatherer, &sub).await {
                            Ok(medias) => {
                                let response = Downloader::add_downloadables(
                                    downloader,
                                    medias
                                        .into_iter()
                                        .take(10)
                                        .map(|media| {
                                            let downloadable_path =
                                                subscription_downloads_directory
                                                    .clone()
                                                    .join(if media.paid { "paid" } else { "free" });
                                            Downloadable::from_media_with_path(
                                                &media,
                                                downloadable_path,
                                            )
                                        })
                                        .collect(),
                                )
                                .await;
                                match response {
                                    Ok(_) => {
                                        info!("Successfully sent post content items to the download queue");
                                    }
                                    Err(err) => {
                                        error!("Failed to send items to the queue: {}", err);
                                    }
                                };
                            }
                            Err(gatherer_err) => error!(
                                "The {} gatherer was unable to get subscriptions. {:?}",
                                gatherer_name, gatherer_err
                            ),
                        }
                    });
                }
                // Ok(())
            }
            Err(subs_err) => {
                eprintln!("No subscribers found for {}. ({})", gatherer_name, subs_err);
                // Err(format!("Subscription error: {:?}", subs_err))
            }
        }
        // });
    }

    drop(downloader_channel);
    // tokio::spawn(async move {
    match downloader.process_downloads().await {
        Ok(count) => {
            println!("Successfully downloaded {} files", count);
            Ok(())
        }
        Err(download_err) => {
            println!("Failed to process all downloads. {:?}", download_err);
            Err(download_err)
        }
    }
    // });
    // Ok(())
}

#[cfg(feature = "fansly")]
async fn add_fansly_gatherer(
    config: &'_ Config,
    gatherers: &mut Vec<Arc<dyn Gatherer>>,
) -> AsyncResult<()> {
    if config.fansly.enabled {
        let fansly_gatherer = gatherer_fansly::Fansly::new(
            Arc::new(config.fansly.clone()),
            Arc::new(config.api_config.clone()),
        )
        .await?;
        gatherers.push(Arc::new(fansly_gatherer));
        Ok(())
    } else {
        Ok(())
    }
}

fn init_logging(cli: &'_ Cli) {
    let tracing_level;
    let mut tracing_subscriber_builder = FmtSubscriber::builder();
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.) will be written to stdout.
    match cli.verbose {
        0 => tracing_level = Level::ERROR,
        1 => tracing_level = Level::INFO,
        2 => tracing_level = Level::DEBUG,
        _ => tracing_level = Level::TRACE,
    };
    tracing_subscriber_builder = tracing_subscriber_builder.with_max_level(tracing_level);
    // if cli.pretty {
    // tracing_subscriber_builder = tracing_subscriber.pretty();
    // };
    let tracing_subscriber = tracing_subscriber_builder.finish();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting default subscriber failed");
    debug!("{:#?}", &cli);
}
