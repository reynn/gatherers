use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[structopt(short, long)]
    pub verbose: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}
