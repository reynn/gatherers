mod batch;
mod downloadable;
mod file;

pub use self::{
    batch::{BatchDownloader, MultiThreadedDownloader, SequentialDownloader},
    downloadable::Downloadable,
    file::{FileDownloader, InMemoryFileDownloader, StreamingFileDownloader},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct DownloaderStats {
    pub total: usize,
    pub failed: usize,
    pub success: usize,
    pub previously_saved: usize,
}
