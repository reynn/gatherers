#![warn(clippy::all)]
// Turn off common dev assertions only for debug builds, release builds will continue to generate all warnings
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_macros, unused_imports, unused_variables)
)]

mod cli;
mod cli_tasks;
mod config;
mod macros;

use self::{
    cli::{Cli, CliAction},
    config::Config,
};
use gatherer_core::{
    self,
    directories::Directories,
    downloaders::{BatchDownloader, MultiThreadedDownloader},
    gatherers::{self, Gatherer},
    tasks::spawn_on_thread,
};
use log::{Level, LevelFilter};
use std::{path::Path, sync::Arc, time::Instant};

fn main() {
    smol::block_on(async {
        // This parses all incoming arguments from stdin based on user input.
        // Use this as a starting point to configure the overall app behaviour
        let cli = Cli::new();

        // Setup logging, if the verbose flag is provided provided more detailed output
        init_logging(&cli).expect("Failed to initailize the logger");

        // If the user provided a config file path use that, we fall back to the default if not
        let cfg_path = if let Some(path) = &cli.config_file_path {
            path.to_owned()
        } else {
            Directories::new().get_default_config_dir()
        };

        // Load the app config defaults with user options applied
        let config = match Config::load_or_default(&cfg_path) {
            Ok(loaded) => {
                log::debug!("Successfully loaded config {:?}", &loaded);
                loaded
            }
            Err(load_err) => {
                log::error!("Failed to load config from {:?}. Using default", cfg_path);
                log::debug!("Failed to load config file. Error: {:?}", load_err);
                Arc::new(Config::default())
            }
        };

        match cli.action.exec(config).await {
            Ok(()) => println!("Completed"),
            Err(err) => log::error!("Command failed: {:?}", err),
        };
    });
}

async fn get_available_gatherers(
    conf: &'_ Config,
    gatherer_names: &[String],
) -> gatherer_core::Result<Vec<Arc<dyn Gatherer>>> {
    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();
    log::debug!("Gatherer names from CLI args: {:?}", gatherer_names);
    if !gatherer_names.is_empty() {
        let names: Vec<&str> = gatherer_names.iter().map(|n| n.as_str()).collect();
        for name in names.into_iter() {
            log::debug!("Checking for gatherer named {}", name);
            match name {
                "fansly" => {
                    #[cfg(feature = "fansly")]
                    add_gatherer!(&mut gatherers, gatherer_fansly::Fansly, conf.fansly.clone());
                }
                "onlyfans" | "only_fans" => {
                    #[cfg(feature = "onlyfans")]
                    add_gatherer!(
                        &mut gatherers,
                        gatherer_onlyfans::OnlyFans,
                        conf.onlyfans.clone()
                    );
                }
                _ => log::info!("Gatherer {} is not known at this time.", name),
            }
        }
    } else {
        #[cfg(feature = "fansly")]
        add_gatherer!(&mut gatherers, gatherer_fansly::Fansly, conf.fansly.clone());

        #[cfg(feature = "onlyfans")]
        add_gatherer!(
            &mut gatherers,
            gatherer_onlyfans::OnlyFans,
            conf.onlyfans.clone()
        );
    }

    Ok(gatherers)
}

fn init_logging(cli: &'_ Cli) -> gatherer_core::Result<()> {
    let mut fern_logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(match &cli.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .filter(|metadata| !metadata.target().starts_with("polling"))
        .filter(|metadata| !metadata.target().starts_with("async_io"))
        .filter(|metadata| !metadata.target().starts_with("async_h1"))
        .filter(|metadata| !metadata.target().starts_with("rustls"))
        .chain(std::io::stdout());
    if let Some(log_path) = &cli.log_file {
        fern_logger = fern_logger.chain(fern::log_file(log_path)?);
    }
    fern_logger.apply()?;
    Ok(())
}
