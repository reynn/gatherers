// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod cli;
mod config;

use cli::*;
use config::{Config, ConfigErrors};
use gatherer_core::{
    directories::Directories,
    gatherers::{self, Gatherer, Media},
    AsyncResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tabled::{Style, Table};
use tokio::{join, try_join};
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
            tracing::debug!("Successfully loaded config {:?}", &loaded);
            loaded
        }
        Err(load_err) => {
            tracing::error!("Failed to load config: {}", load_err);
            match load_err {
                ConfigErrors::LoadConfigFailure(path, err) => {
                    tracing::error!("Failed to load config from {:?}. Error {}", path, err);
                    std::process::exit(1);
                }
                ConfigErrors::FromToml(e) => {
                    panic!("Unable to get config from current TOMOL file: {}", e)
                }
                ConfigErrors::ToToml(e) => {
                    tracing::error!("Failed to convert config to TOML: {}", e)
                }
                ConfigErrors::IoError(e) => {
                    tracing::error!("Failed to read or write file {}", e);
                    // std::process::exit(1)
                }
            };
            Arc::new(Config::default())
        }
    };

    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();

    #[cfg(feature = "fansly")]
    add_fansly_gatherer(config, &mut gatherers).await?;

    if gatherers.is_empty() {
        println!("No gatherers to run");
        std::process::exit(0);
    }

    // Go through all gatherers and run through a sequence of requests to scrape data from the provider APIs
    for gatherer in gatherers.into_iter() {
        let gatherer_name = gatherer.name();
        info!("Starting the {} gatherer.", gatherer_name);
        println!("Getting subscriptions for {}", gatherer_name);
        // Get a list of subscriptions
        match gatherer.gather_subscriptions().await {
            Ok(subs) => {
                info!(
                    "Found subscriptions: {}",
                    subs.iter()
                        .map(|sub| &sub.id[..])
                        .collect::<Vec<&str>>()
                        .join(",")
                );
                if subs.is_empty() {
                    return Err(format!("No subscriptions for {}", gatherer_name).into());
                };
                println!("Found {} subs for {}", subs.len(), gatherer_name);
                let subs_table = Table::new(&subs).with(Style::pseudo_clean());
                println!("{}", subs_table);
                for sub in subs.iter() {
                    tracing::info!(
                        "{} is getting data for subscriber: {}",
                        gatherer_name,
                        sub.username
                    );

                    let posts_media = gatherer.gather_media_from_posts(sub);
                    let messages_media = gatherer.gather_media_from_messages(sub);
                    let stories_media = gatherer.gather_media_from_stories(sub);
                    let bundles_media = gatherer.gather_media_from_bundles(sub);

                    let (posts, messages, stories, bundles) =
                        join!(posts_media, messages_media, stories_media, bundles_media);
                    match posts {
                        Ok(posts) => {
                            posts.len().gt(&0).then(|| {
                                println!(
                                    "Found {} items from posts.\n{}",
                                    posts.len(),
                                    Table::new(&posts).with(Style::pseudo_clean())
                                )
                            });
                        }
                        Err(e) => println!("{}", e),
                    };
                    match messages {
                        Ok(messages) => {
                            messages.len().gt(&0).then(|| {
                                println!(
                                    "Found {} items from messages.\n{}",
                                    messages.len(),
                                    Table::new(&messages).with(Style::pseudo_clean())
                                )
                            });
                        }
                        Err(e) => println!("{}", e),
                    };
                    match stories {
                        Ok(stories) => {
                            stories.len().gt(&0).then(|| {
                                println!(
                                    "Found {} items from stories.\n{}",
                                    stories.len(),
                                    Table::new(&stories).with(Style::pseudo_clean())
                                )
                            });
                        }
                        Err(e) => println!("{}", e),
                    };
                    match bundles {
                        Ok(bundles) => {
                            bundles.len().gt(&0).then(|| {
                                println!(
                                    "Found {} items from bundles.\n{}",
                                    bundles.len(),
                                    Table::new(&bundles).with(Style::pseudo_clean())
                                )
                            });
                        }
                        Err(e) => println!("{}", e),
                    };
                    // std::process::exit(0);
                }
            }
            Err(subs_err) => {
                error!("{:#?}", subs_err);
                eprintln!("No subscribers found for {}. ({})", gatherer_name, subs_err)
            }
        };
    }

    Ok(())
}

#[cfg(feature = "fansly")]
async fn add_fansly_gatherer(
    config: Arc<Config>,
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
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
