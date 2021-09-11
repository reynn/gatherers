use cfg_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, FromConfig, Deserialize, Serialize)]
pub struct DownloadersConfig {
    pub storage_dir: String,
}
