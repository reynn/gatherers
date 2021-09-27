mod errors;

pub use errors::CliErrors;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(long)]
    pub content_types: Option<ContentTypes>,
    #[structopt(short, long)]
    pub target_folder: Option<PathBuf>,
    #[structopt(long)]
    pub pretty: bool,
    #[structopt(short = "C", long, default_value = "8")]
    pub worker_count: usize,
    #[structopt(short, long, default_value = "0")]
    pub limit_subs: usize,
    #[structopt(short = "L", long, default_value = "0")]
    pub limit_media: usize,
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
