use eyre::eyre;
use std::fmt::Formatter;
use {
    crate::{config::Config, get_available_gatherers},
    clap::{Parser, Subcommand},
    gatherer_core::Result,
    std::{path::PathBuf, str::FromStr, sync::Arc},
};

#[derive(Debug, Clone)]
pub enum TransactionFormat {
    Json,
    PlainText,
    Table,
}

impl FromStr for TransactionFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "json" => Ok(TransactionFormat::Json),
            "text" => Ok(TransactionFormat::PlainText),
            "table" => Ok(TransactionFormat::Table),
            _ => Err(format!("[{s}] is not recognized")),
        }
    }
}

impl std::fmt::Display for TransactionFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionFormat::Json => {
                write!(f, "json")
            }
            TransactionFormat::PlainText => {
                write!(f, "text")
            }
            TransactionFormat::Table => {
                write!(f, "table")
            }
        }
    }
}

impl Default for TransactionFormat {
    fn default() -> Self {
        Self::PlainText
    }
}

/// Gatherers will get media you have access to from paid sites such as OnlyFans and Fansly
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
    #[clap(long)]
    pub content_types: Option<ContentTypes>,
    #[clap(short, long)]
    pub target_folder: Option<PathBuf>,
    #[clap(short, long)]
    pub log_file: Option<PathBuf>,
    #[clap(subcommand)]
    pub action: CliAction,
    #[clap(short, long)]
    pub gatherers: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum CliAction {
    /// Start running the main gatherer logic
    Start {
        #[clap(short, long)]
        user_names: Vec<String>,
        #[clap(short, long, default_value_t = 8)]
        worker_count: u8,
        #[clap(short = 'S', long)]
        limit_subs: Option<usize>,
        #[clap(short = 'M', long)]
        limit_media: Option<usize>,
        #[clap(short, long)]
        ignored_user_names: Vec<String>,
    },
    /// Gather only purchased content
    Purchased,
    /// Like posts from users you are subscribed to
    Like {
        #[clap(short, long)]
        all: Option<bool>,
        #[clap(short, long)]
        user: Option<String>,
    },
    /// Unlike posts from users you are subscribed to
    Unlike {
        #[clap(short, long)]
        all: Option<bool>,
        #[clap(short, long)]
        user: Option<String>,
    },
    /// List users you currently have active subscriptions to
    List,
    /// Get a list of transactions based on the authenticated user account
    Transactions {
        #[clap(short, long)]
        user_names: Vec<String>,
        #[clap(short, long, default_value_t = TransactionFormat::Table)]
        format: TransactionFormat,
    },
}

impl CliAction {
    // take ownership the action of self
    pub async fn exec(self, conf: Arc<Config>, gatherers: &[String]) -> Result<()> {
        match self {
            CliAction::Start {
                user_names,
                worker_count,
                limit_subs,
                limit_media,
                ignored_user_names,
            } => match get_available_gatherers(&conf, gatherers).await {
                Ok(gatherers) => {
                    crate::cli_tasks::start(
                        gatherers,
                        &conf,
                        worker_count,
                        user_names,
                        limit_media,
                        limit_subs,
                        ignored_user_names,
                    )
                    .await?;
                    Ok(())
                }
                Err(err) => Err(eyre!("Failed to get configured gatherers. {:?}", err)),
            },
            CliAction::Like {
                all: like_all,
                user: like_user,
            } => {
                println!("Trying to like posts...");
                log::debug!("Opts: {:?}, {:?}", like_all, like_user);
                Ok(())
            }
            CliAction::Unlike {
                all: like_all,
                user: like_user,
            } => {
                println!(
                    "Unliking posts [gatherers: {:?}; like_all: {:?}; like_user: {:?}];",
                    gatherers, like_all, like_user
                );
                match get_available_gatherers(&conf, gatherers).await {
                    Ok(gatherers) => match crate::cli_tasks::unlike(gatherers).await {
                        Ok(_) => (),
                        Err(err) => {
                            log::error!("Error unliking posts: {:?}", err);
                        }
                    },
                    Err(gatherers_err) => {
                        log::error!("Error getting available gatherers: {:?}", gatherers_err);
                    }
                }
                Ok(())
            }
            CliAction::Purchased => match get_available_gatherers(&conf, gatherers).await {
                Ok(gatherers) => Ok(crate::cli_tasks::purchased(gatherers, &conf).await?),
                Err(err) => Err(eyre!("Failed to get configured gatherers. {:?}", err)),
            },
            CliAction::List => match get_available_gatherers(&conf, gatherers).await {
                Ok(gatherers) => Ok(crate::cli_tasks::list(gatherers).await?),
                Err(err) => Err(eyre!("Failed to get configured gatherers. {:?}", err)),
            },
            CliAction::Transactions { user_names, format } => {
                match get_available_gatherers(&conf, gatherers).await {
                    Ok(gatherers) => {
                        Ok(crate::cli_tasks::transactions(gatherers, user_names, format).await?)
                    }
                    Err(err) => Err(eyre!("Failed to get configured gatherers. {:?}", err)),
                }
            }
        }
    }
}

impl Default for CliAction {
    fn default() -> Self {
        Self::Start {
            worker_count: Default::default(),
            limit_subs: Default::default(),
            limit_media: Default::default(),
            user_names: Default::default(),
            ignored_user_names: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ContentTypes {
    All,
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
