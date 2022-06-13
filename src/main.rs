#![warn(clippy::all)]
mod cli;
mod cli_tasks;
mod config;
mod macros;

use {
    self::{cli::Cli, config::Config},
    gatherer_core::{self, directories::Directories, gatherers::Gatherer},
    std::sync::Arc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This parses all incoming arguments from stdin based on user input.
    // Use this as a starting point to configure the overall app behaviour
    let cli = Cli::new();

    // Setup logging, if the verbose flag is provided provided more detailed output
    init_logging(&cli).expect("Failed to initialize the logger");

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

    match cli.action.exec(config, &cli.gatherers).await {
        Ok(()) => eprintln!("Completed"),
        Err(err) => log::error!("Command failed: {:?}", err),
    };
    Ok(())
}

async fn get_available_gatherers(
    conf: &'_ Config,
    gatherer_names: &[String],
) -> gatherer_core::Result<Vec<Arc<dyn Gatherer>>> {
    let mut gatherers: Vec<Arc<dyn Gatherer>> = Vec::new();
    log::debug!("Gatherer names from CLI args: {:?}", gatherer_names);
    if !gatherer_names.is_empty() {
        for name in gatherer_names.iter().map(|n| n.as_str()) {
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
    let level_filter = cli.verbose.log_level_filter();
    println!("LevelFilter: {:?}", level_filter);
    let mut fern_logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level_filter)
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
