mod errors;

pub use errors::CliErrors;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(long)]
    pub content_types: Option<ContentTypes>,
}

#[derive(Debug, StructOpt)]
pub enum ContentTypes {
    All,
    Images,
    Videos,
    Messages,
}

impl FromStr for ContentTypes {
    type Err = CliErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let options = ["All", "Images", "Videos", "Messages"]
            .iter()
            .map(|o| String::from(*o))
            .collect();
        match s.to_lowercase().as_str() {
            "all" => Ok(Self::All),
            "images" => Ok(Self::Images),
            "videos" => Ok(Self::Videos),
            "messages" => Ok(Self::Messages),
            _ => Err(CliErrors::InvalidContentType {
                provided: String::from(s),
                valid_options: options,
            }),
        }
    }
}

impl Default for ContentTypes {
    fn default() -> Self {
        Self::All
    }
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}
