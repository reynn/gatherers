// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod cli;
mod config;

use async_task::Task;
use cli::*;
use config::{Config, ConfigErrors};
use futures::Future;
use gatherer_core::{
    self,
    directories::Directories,
    downloaders::{Downloadable, Downloader, MultiThreadedDownloader, SequentialDownloader},
    gatherers::{self, Gatherer, Media},
    tasks::spawn_on_thread,
    AsyncResult, *,
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::exit,
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

// #[tokio::main]
fn main() {
    smol::block_on(async {
        match run().await {
            Ok(_) => println!("Gathering completed"),
            Err(err) => println!("Gatherers has encountered an error {:?}", err),
        }
    });
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
    #[cfg(feature = "onlyfans")]
    add_onlyfans_gatherer(&config, &mut gatherers).await?;

    if gatherers.is_empty() {
        println!("No gatherers to run");
        exit(0);
    }

    let (tx, rx) = async_channel::unbounded();

    // Initialize our downloader
    let downloader = MultiThreadedDownloader::new(cli.worker_count, rx);

    let downloads_directory = if let Some(cli_download_dir) = &cli.target_folder {
        cli_download_dir.to_owned()
    } else {
        Path::new(&config.download_dir).to_path_buf()
    };

    let mut primary_threads = Vec::new();

    for gatherer in gatherers.into_iter() {
        let gatherers_downloader = tx.clone();
        primary_threads.push(spawn_on_thread({
            let base_path = downloads_directory.clone();
            let download_tx = tx.clone();
            let limits = gatherers::RunLimits {
                media: cli.limit_media,
                subscriptions: cli.limit_subs,
            };
            async move {
                let gatherer_name = gatherer.name();
                let start_time = Instant::now();
                println!(
                    "{}: Starting to gather for all subscriptions.",
                    gatherer_name
                );
                match gatherers::run_gatherer_for_all(gatherer, base_path, download_tx, limits)
                    .await
                {
                    Ok(_) => println!(
                        "{}: Finished after {:.2} seconds",
                        gatherer_name,
                        Instant::now().duration_since(start_time).as_secs_f64()
                    ),
                    Err(gatherer_err) => {
                        error!("{}: Failed to complete. {:?}", gatherer_name, gatherer_err)
                    }
                }
            }
        }));
    }

    primary_threads.insert(
        0,
        spawn_on_thread(async move {
            println!("Starting downloader..");
            let start_time = Instant::now();
            match downloader.process_all_items().await {
                Ok(stats) => info!(
                    "Successfully completed downloads: {:?}. Took {:.2} seconds",
                    stats,
                    Instant::now().duration_since(start_time).as_secs_f32()
                ),
                Err(down_err) => error!("Failed to process downloads: {:?}", down_err),
            }
        }),
    );

    drop(tx);
    futures::future::join_all(primary_threads).await;
    // info!("Expected {} total downloads");
    Ok(())
}

#[cfg(feature = "fansly")]
async fn add_fansly_gatherer(
    config: &'_ Config,
    gatherers: &mut Vec<Arc<dyn Gatherer>>,
) -> AsyncResult<()> {
    if config.fansly.enabled {
        gatherers.push(Arc::new(
            gatherer_fansly::Fansly::new(
                Arc::new(config.fansly.clone()),
                Arc::new(config.api_config.clone()),
            )
            .await?,
        ));
    };
    Ok(())
}

#[cfg(feature = "onlyfans")]
async fn add_onlyfans_gatherer(
    config: &'_ Config,
    gatherers: &mut Vec<Arc<dyn Gatherer>>,
) -> AsyncResult<()> {
    if config.onlyfans.enabled {
        gatherers.push(Arc::new(
            gatherer_onlyfans::OnlyFans::new(
                Arc::new(config.onlyfans.clone()),
                Arc::new(config.api_config.clone()),
            )
            .await?,
        ));
    };
    Ok(())
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
}
