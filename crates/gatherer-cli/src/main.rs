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
use tracing::{debug, error, info, subscriber, Level};
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
        println!("Getting subscriptions for {}", gatherer_name);
        // Get a list of subscriptions
        // tokio::spawn(async move {
        match gatherer.gather_subscriptions().await {
            Ok(subs) => {
                let config = Arc::clone(&config);
                info!(
                    "Found subscription ids: {}",
                    subs.iter()
                        .map(|sub| &sub.id[..])
                        .collect::<Vec<&str>>()
                        .join(",")
                );
                if subs.is_empty() {
                    return Err(format!("No subscriptions for {}", gatherer_name).into());
                };
                println!(
                    "Found {} subs for {}\n{}",
                    subs.len(),
                    gatherer_name,
                    Table::new(&subs).with(Style::pseudo_clean())
                );
                let subs: Vec<gatherers::Subscription> = subs.into_iter().take(10).collect();
                for sub in subs.iter() {
                    let subscription_downloads_directory = downloads_directory
                        .join(gatherer_name.to_lowercase())
                        .join(&sub.name.username.to_lowercase());
                    println!(
                        "{} is getting data for subscriber: {}",
                        gatherer_name, sub.name
                    );

                    let posts_media = gatherer.gather_media_from_posts(sub);
                    let messages_media = gatherer.gather_media_from_messages(sub);
                    let stories_media = gatherer.gather_media_from_stories(sub);
                    let bundles_media = gatherer.gather_media_from_bundles(sub);

                    let (posts, messages, stories, bundles) =
                        join!(posts_media, messages_media, stories_media, bundles_media);
                    match posts {
                        Ok(posts) => {
                            if !posts.is_empty() {
                                println!(
                                    "\tFound {} items from posts.\n",
                                    posts.len(),
                                    // Table::new(&posts).with(Style::pseudo_clean())
                                );
                                let downloadables = posts
                                    .into_iter()
                                    .map(|post_media| {
                                        let downloadable_path = subscription_downloads_directory
                                            .clone()
                                            .join(if post_media.paid { "paid" } else { "free" });
                                        Downloadable::from_media_with_path(
                                            post_media,
                                            downloadable_path,
                                        )
                                    })
                                    .collect();
                                match downloader.add_downloadables(downloadables).await {
                                    Ok(_) => info!("Successfully sent post content items to the download queue"),
                                    Err(err) => error!("Failed to send items to the queue: {}", err),
                                };
                            } else {
                                info!("\tNo posts");
                            }
                        }
                        Err(e) => println!("\tError getting : {}", e),
                    };
                    match messages {
                        Ok(messages) => {
                            if !messages.is_empty() {
                                println!(
                                    "\tFound {} items from messages.\n",
                                    messages.len(),
                                    // Table::new(&messages).with(Style::pseudo_clean())
                                )
                            } else {
                                println!("\tNo messages found.");
                            }
                        }
                        Err(e) => println!("\tError getting messages: {}", e),
                    };
                    match stories {
                        Ok(stories) => {
                            if !stories.is_empty() {
                                println!(
                                    "\tFound {} items from stories.\n",
                                    stories.len(),
                                    // Table::new(&stories).with(Style::pseudo_clean())
                                )
                            } else {
                                println!("\tNo stories found");
                            }
                        }
                        Err(e) => println!("\tError getting stories: {}", e),
                    };
                    match bundles {
                        Ok(bundles) => {
                            if !bundles.is_empty() {
                                println!(
                                    "\tFound {} items from bundles.\n",
                                    bundles.len(),
                                    // Table::new(&bundles).with(Style::pseudo_clean())
                                )
                            } else {
                                println!("\tNo bundles found");
                            }
                        }
                        Err(e) => println!("\tError getting bundles: {}", e),
                    };
                }
            }
            Err(subs_err) => {
                error!("Subscription error: {:?}", subs_err);
                eprintln!("No subscribers found for {}. ({})", gatherer_name, subs_err)
            }
        };
        println!("Gathered all from {}", gatherer_name);
        // });
    }

    println!("Downloading files");
    let receiver = downloader.receiver;
    // tokio::spawn(async move {
        Downloader::process_downloads(receiver).await;
    // });
    Ok(())
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
    println!("{:#?}", &cli);
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(match cli.verbose {
            0 => Level::ERROR,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .pretty()
        // completes the builder.
        .finish();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
