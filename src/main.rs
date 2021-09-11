// Turn off common dev assertions only for debug builds, release builds will still work as normal
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod cli;
mod config;
mod directories;
mod downloaders;
mod gatherers;
mod http;

use tabled::{Style, Table};
use crate::{cli::Cli, config::Config};
// use crate::gatherers::errors::GathererErrors;

// #[tokio::main]
fn main() {
    match run() {
        Ok(_) => eprintln!("----------------------- Completed -----------------------"),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // This parses all incoming arguments from stdin based on user input.
    // Use this as a starting point to configure the overall app behaviour
    let cli = Cli::new();

    // Setup logging, if the verbose flag is provided provided more detailed output
    init_logging(&cli);

    // If the user provided a config file path use that, we fall back to the default if not
    let cfg_path = if let Some(path) = cli.config_file_path {
        path
    } else {
        config::get_default_path()
    };

    // Load the app config defaults with user options applied
    let config = match Config::load(&cfg_path) {
        Ok(loaded) => {
            log::debug!("Successfully loaded config {:?}", &loaded);
            loaded
        }
        Err(load_err) => {
            log::error!("Failed to load config: {}", load_err);
            Config::default()
        }
    };

    // Use the config to get all enabled and properly configured gatherers
    let gatherers = gatherers::validated_gatherers(&config)?;

    // Go through all gatherers and run through a sequence of requests to scrape data from the provider APIs
    for gatherer in gatherers.into_iter() {
        let gatherer_name = gatherer.name();
        // Get a list of subscriptions
        match gatherer.gather_subscriptions() {
            Ok(subs) => {
                let subs_table = Table::new(&subs).with(Style::psql());
                eprintln!("{}", subs_table);
            }
            Err(subs_err) => log::error!(
                "Failed to get subs from the {} gatherer. \n{}",
                gatherer_name,
                subs_err
            ),
        }
    }
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
        // .chain(fern::log_file("fansly.log").unwrap())
        .apply()
        .expect("Could not initialize the logger");
}
