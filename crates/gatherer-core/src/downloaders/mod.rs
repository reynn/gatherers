mod errors;

use serde::{Deserialize, Serialize};
pub use errors::DownloaderErrors;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloadersConfig {
    pub storage_dir: String,
    // pub r#type: DownloaderType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DownloaderType {
    Sequential,
    MultiThreaded,
}

impl Default for DownloaderType {
    fn default() -> Self {
        Self::MultiThreaded
    }
}