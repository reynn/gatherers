mod errors;

use serde::{Deserialize, Serialize};
pub use errors::DownloaderErrors;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloadersConfig {
    pub storage_dir: String,
}
