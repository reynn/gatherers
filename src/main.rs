// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use crate::{cli::Cli, config::Config};

mod cli;
mod config;
mod directories;
mod downloaders;
mod gatherers;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
    }
}

fn run() -> anyhow::Result<()> {
    // This parses all incoming arguments from stdin based on user input.
    // Use this as a starting point to configure the overall app behaviour
    let cli = Cli::new();

    init_logging(&cli);

    // If the user provided a config file path use that, we fall back to the default if not
    let cfg_path = if let Some(path) = cli.config_file_path {
        path
    } else {
        config::get_default_path()
    };

    // Load the app config defaults with user options applied
    let config = if let Ok(loaded_config) = Config::load(&cfg_path) {
        log::debug!("Successfully loaded config {:?}", &loaded_config);
        loaded_config
    } else {
        log::debug!("Initializing a default config");
        Config::default()
    };

    let gatherers = gatherers::validated_gatherers(config)?;
    for gatherer in gatherers.into_iter() {
        let gatherer_name = gatherer.name();
        match gatherer.gather_subscriptions() {
            Ok(subs) => log::debug!("{} got {} subs: {:?}", gatherer_name, subs.len(), { subs }),
            Err(subs_err) => log::error!(
                "Failed to get subs from the {} gatherer. \n{}",
                gatherer_name,
                subs_err
            ),
        }
    }
    // config
    //     .get_available_gatherers()?
    //     .iter()
    //     .for_each(|po_gatherer| {
    //         if let Ok(subscriptions) = po_gatherer.gather_subscriptions() {
    //             log::debug!("Subs: {:?}", subscriptions);
    //         } else {
    //             log::error!("Failed to get subscriptions from the {:?}", po_gatherer);
    //         }
    //     });
    // match po_gatherer.gather_subscriptions() {
    //     Ok(subs) => match gatherer.gather_subscriptions() {
    //         Ok(some_subs) => {
    //             log::info!("Found subs from {}: {:?}", gatherer.name(), some_subs);
    //             log::info!("Outer subs from {}: {:?}", gatherer.name(), subs);
    //         }
    //         Err(subs_err) => log::error!("Failed to gather subscriptions. {}", subs_err),
    //     },
    //     Err(err) => log::error!("Failed to configure the Fansly gatherer. {}", err),
    // });
    log::info!("Gatherings Completed");
    Ok(())
}

fn init_logging(cli: &'_ Cli) {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("fansly.log").unwrap())
        .apply()
        .expect("Could not initialize the logger");
}
