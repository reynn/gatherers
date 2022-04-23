use {
    crate::{config::Config, get_available_gatherers},
    bpaf::*,
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

impl Default for TransactionFormat {
    fn default() -> Self {
        Self::PlainText
    }
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Cli {
    #[bpaf(short, long)]
    pub config_file_path: Option<PathBuf>,
    #[bpaf(external(verbose))]
    pub verbose: Option<usize>,
    #[bpaf(long)]
    pub content_types: Option<ContentTypes>,
    #[bpaf(short, long)]
    pub target_folder: Option<PathBuf>,
    #[bpaf(short, long)]
    pub log_file: Option<PathBuf>,
    #[bpaf(external(cli_action))]
    pub action: CliAction,
    #[bpaf(short, long)]
    pub gatherers: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        cli().run()
    }
}

fn verbose() -> Parser<Option<usize>> {
    short('v')
        .long("verbose")
        .help("Increase the verbosity of output\nSpecify no more than 3 times\n-v -v -v or -vvv")
        .req_flag(())
        .many()
        .map(|xs| xs.len())
        .guard(|&x| x <= 3, "Cannot have more than 3 levels of verbosity")
        .optional()
}

/// the set of subcommands for [`Cli`] along with the needed arguments
#[derive(Debug, Clone, Bpaf)]
pub enum CliAction {
    /// Start running the main gatherer logic
    #[bpaf(command("start"))]
    Start {
        #[bpaf(short, long, fallback(Vec::new()))]
        user_names: Vec<String>,
        #[bpaf(short, long, fallback(8))]
        worker_count: u8,
        #[bpaf(short, long)]
        limit_subs: Option<usize>,
        #[bpaf(short, long)]
        limit_media: Option<usize>,
        #[bpaf(short, long, fallback(Vec::new()))]
        ignored_user_names: Vec<String>,
    },
    #[bpaf(command("purchased"))]
    /// Gather only purchased content
    Purchased,
    /// Like posts from users you are subscribed to
    #[bpaf(command("like"))]
    Like {
        #[bpaf(short, long)]
        like_all: Option<bool>,
        #[bpaf(short, long)]
        like_user: Option<String>,
    },
    /// Unlike posts from users you are subscribed to
    #[bpaf(command("unlike"))]
    Unlike {
        #[bpaf(short, long)]
        like_all: Option<bool>,
        #[bpaf(short, long)]
        like_user: Option<String>,
    },
    /// List users you currently have active subscriptions to
    #[bpaf(command("list"))]
    List,
    /// Get a list of transactions based on the authenticated user account
    #[bpaf(command("transactions"))]
    Transactions {
        #[bpaf(short, long, fallback(Vec::new()))]
        user_names: Vec<String>,
        #[bpaf(short, long, fallback(TransactionFormat::Table))]
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
                Err(err) => Err(format!("Failed to get configured gatherers. {:?}", err).into()),
            },
            CliAction::Like {
                like_all,
                like_user,
            } => {
                println!("Trying to like posts...");
                log::debug!("Opts: {:?}, {:?}", like_all, like_user);
                Ok(())
            }
            CliAction::Unlike {
                like_all,
                like_user,
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
                Err(err) => Err(format!("Failed to get configured gatherers. {:?}", err).into()),
            },
            CliAction::List => match get_available_gatherers(&conf, gatherers).await {
                Ok(gatherers) => Ok(crate::cli_tasks::list(gatherers).await?),
                Err(err) => Err(format!("Failed to get configured gatherers. {:?}", err).into()),
            },
            CliAction::Transactions { user_names, format } => {
                match get_available_gatherers(&conf, gatherers).await {
                    Ok(gatherers) => {
                        Ok(crate::cli_tasks::transactions(gatherers, user_names, format).await?)
                    }
                    Err(err) => {
                        Err(format!("Failed to get configured gatherers. {:?}", err).into())
                    }
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
