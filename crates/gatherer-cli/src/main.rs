// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod cli;

use std::sync::Arc;

use cli::*;
use gatherer_core::{
    config::{Config, ConfigErrors},
    directories::Directories,
    gatherers::{self, Gatherer},
};
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    // ..Config,
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => eprintln!("----------------------- Completed -----------------------"),
        Err(err) => eprintln!("{:?}", err),
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
                    tracing::error!("Failed to read or write the {}", e);
                    std::process::exit(1)
                }
            };
            Config::default()
        }
    };

    let api_config = config.api_config.clone().unwrap_or_default();
    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();

    #[cfg(feature = "fansly")]
    add_fansly_gatherer(api_config.clone(), &mut gatherers)?;

    // Go through all gatherers and run through a sequence of requests to scrape data from the provider APIs
    for gatherer in gatherers.into_iter() {
        let gatherer_name = gatherer.name();
        // Get a list of subscriptions
        match gatherer.gather_subscriptions().await {
            Ok(subs) => {
                // let subs_table = Table::new(&subs)
                //     .with(Style::pseudo_clean())
                //     .with(Modify::new(Column(3..3)).with(|s: &str| format!("{:.3}", s)));
                // eprintln!("{}", subs_table);
                // subs.into_par_iter()
                //     .map(|sub| {})
                //     .collect_into_vec();
                // subs.iter().for_each(|sub| async {
                //     let resp = gatherer.gather_media_from_posts(sub).await;
                //     match resp {
                //         Ok(posts) => {
                //             tracing::info!("Found {} posts for {}", posts.len(), sub.username);
                //         }
                //         Err(posts_err) => {
                //             tracing::error!(
                //                 "Failed to gather posts for {}, {}",
                //                 sub.username,
                //                 posts_err
                //             );
                //             eprintln!("-----------------------------------");
                //         }
                //     }
                // });
            }
            Err(subs_err) => tracing::error!(
                "Failed to get subs from the {} gatherer. \n{}",
                gatherer_name,
                subs_err
            ),
        }
    }

    Ok(())
}

#[cfg(feature = "fansly")]
fn add_fansly_gatherer(
    api_config: gatherer_core::http::ApiClientConfig,
    gatherers: &mut Vec<Arc<dyn Gatherer>>,
) -> Result<()> {
    todo!()
}

fn init_logging(cli: &'_ Cli) {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(if cli.verbose {
            Level::DEBUG
        } else {
            Level::INFO
        })
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
