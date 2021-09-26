mod downloadable;
mod errors;
mod multi_threaded;
mod sequential;

pub use self::{
    downloadable::Downloadable, errors::DownloaderErrors, multi_threaded::MultiThreadedDownloader,
    sequential::SequentialDownloader,
};
use crate::{gatherers::Media, AsyncResult};
use async_channel::unbounded;
pub use async_channel::{Receiver, Sender};
use async_fs::{File, OpenOptions};
use async_task::Task;
use futures::{lock::Mutex, Future};
use futures_lite::{
    io::{copy, AsyncWriteExt, BufReader},
    stream, StreamExt,
};
use indicatif::{MultiProgress, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::Display,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use surf::http::headers::HeaderValue;
use tracing::{debug, error, info, trace};

//
#[async_trait::async_trait]
pub trait Downloader: Send + Sync {
    // Add a single item to the downloader queue
    async fn add_item_to_queue(&self, item: Downloadable) -> AsyncResult<()>;
    // Process a single item from the queue
    async fn process_single_item(&self, worker_num: usize) -> AsyncResult<u64>;
    // Loop through download queue until closed or empty
    async fn process_all_items(&self) -> AsyncResult<DownloaderStats>;
    async fn get_sender(&self) -> AsyncResult<Sender<Downloadable>>;
}

// TODO: not entirely sure this does what is expected.
// theory is Downloader::default() should reurn a  sequential downloader in its default state
impl dyn Downloader {
    fn default() -> SequentialDownloader {
        SequentialDownloader::default()
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct DownloaderStats {
    pub total: usize,
    pub failed: usize,
    pub success: usize,
    pub previously_saved: usize,
}
