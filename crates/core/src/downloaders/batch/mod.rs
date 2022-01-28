mod multi_threaded;
mod sequential;

pub use self::{multi_threaded::MultiThreadedDownloader, sequential::SequentialDownloader};
use crate::Result;
use async_trait::async_trait;

//
#[async_trait]
pub trait BatchDownloader: Send + Sync + std::fmt::Display {
    fn name(&self) -> String;
    // Add a single item to the downloader queue
    async fn add_item_to_queue(&self, item: super::Downloadable) -> Result<()>;
    // Process a single item from the queue
    async fn process_single_item(&self, worker_num: usize) -> Result<u64>;
    // Loop through download queue until closed or empty
    async fn process_all_items(&self) -> Result<super::DownloaderStats>;
}

// TODO: not entirely sure this does what is expected.
// theory is Downloader::default() should reurn a  sequential downloader in its default state
impl dyn BatchDownloader {
    fn default() -> SequentialDownloader {
        SequentialDownloader::default()
    }
}
