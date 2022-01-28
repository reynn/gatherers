use crate::cli_tasks::cli_task_gatherers_like;
use crate::{config::Config, get_available_gatherers};
use gatherer_core::gatherers::Gatherer;
use gatherer_core::Result;
use std::{path::PathBuf, str::FromStr, sync::Arc};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Cli {
    #[structopt(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(long)]
    pub content_types: Option<ContentTypes>,
    #[structopt(short, long)]
    pub target_folder: Option<PathBuf>,
    #[structopt(short, long)]
    pub log_file: Option<PathBuf>,
    #[structopt(subcommand)]
    pub action: CliAction,
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}

/// the set of subcommands for [`Cli`] along with the needed arguments
#[derive(Debug, Clone, StructOpt)]
pub enum CliAction {
    Start {
        #[structopt(short, long)]
        gatherers: Vec<String>,
        #[structopt(short, long)]
        user_names: Vec<String>,
        #[structopt(short = "C", long)]
        worker_count: Option<u8>,
        #[structopt(short, long)]
        limit_subs: Option<usize>,
        #[structopt(short = "L", long)]
        limit_media: Option<usize>,
        #[structopt(short, long)]
        ignored_user_names: Vec<String>,
    },
    Like {
        #[structopt(short, long)]
        gatherers: Vec<String>,
        #[structopt(short = "L", long)]
        like_all: Option<bool>,
        #[structopt(short, long)]
        like_user: Option<String>,
    },
    Unlike {
        #[structopt(short, long)]
        gatherers: Vec<String>,
        #[structopt(short = "L", long)]
        like_all: Option<bool>,
        #[structopt(short, long)]
        like_user: Option<String>,
    },
    List {
        #[structopt(short, long)]
        gatherers: Vec<String>,
    },
    Transactions {
        #[structopt(short, long)]
        gatherers: Vec<String>,
        #[structopt(short, long)]
        user_names: Vec<String>,
    },
}

impl Default for CliAction {
    fn default() -> Self {
        Self::Start {
            gatherers: Default::default(),
            worker_count: Default::default(),
            limit_subs: Default::default(),
            limit_media: Default::default(),
            user_names: Default::default(),
            ignored_user_names: Default::default(),
        }
    }
}

impl CliAction {
    // take ownership the action of self
    pub async fn exec(self, conf: Arc<Config>) -> Result<()> {
        match self {
            CliAction::Start {
                gatherers,
                user_names,
                worker_count,
                limit_subs,
                limit_media,
                ignored_user_names,
            } => {
                match get_available_gatherers(&conf, &gatherers).await {
                    Ok(gatherers) => {
                        crate::cli_tasks::cli_task_gatherers_start(
                            gatherers,
                            &conf,
                            // TODO: default should be system dependant maybe num_cpu crate?
                            worker_count.unwrap_or(conf.workers),
                            user_names,
                            limit_media,
                            limit_subs,
                            ignored_user_names,
                        )
                        .await?;
                        Ok(())
                    }
                    Err(err) => {
                        Err(format!("Failed to get configured gatherers. {:?}", err).into())
                    }
                }
            }
            CliAction::Like {
                gatherers,
                like_all,
                like_user,
            } => {
                println!("Trying to like posts...");
                Ok(())
            }
            CliAction::Unlike {
                gatherers,
                like_all,
                like_user,
            } => {
                println!(
                    "Unliking posts [gatherers: {:?}; like_all: {:?}; like_user: {:?}];",
                    gatherers, like_all, like_user
                );
                Ok(())
            }
            CliAction::List { gatherers } => {
                println!("Listing subscriptions [gatherers: {:?};]", gatherers);
                Ok(())
            }
            CliAction::Transactions {
                gatherers,
                user_names,
            } => {
                println!(
                    "Getting transaction information for gatherers {:?}",
                    gatherers
                );
                match get_available_gatherers(&conf, &gatherers).await {
                    Ok(gatherers) => Ok(crate::cli_tasks::cli_task_gatherers_transactions(
                        gatherers, user_names,
                    )
                    .await?),
                    Err(err) => {
                        Err(format!("Failed to get configured gatherers. {:?}", err).into())
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, StructOpt)]
pub enum ContentTypes {
    All,
    Paid,
    Images,
    Videos,
    Messages,
}

impl FromStr for ContentTypes {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let options: Vec<String> = ["All", "Images", "Videos", "Messages"]
            .iter()
            .map(|o| String::from(*o))
            .collect();
        match s.to_lowercase().as_str() {
            "all" => Ok(Self::All),
            "images" => Ok(Self::Images),
            "videos" => Ok(Self::Videos),
            "messages" => Ok(Self::Messages),
            _ => Err(format!(
                "'{}' is not a valid option. Valid: {:?}",
                s, options
            )),
        }
    }
}

impl Default for ContentTypes {
    fn default() -> Self {
        Self::All
    }
}
