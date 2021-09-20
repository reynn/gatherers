mod errors;

use crate::{gatherers::Media, AsyncResult};
use crossbeam_channel::*;
// use async_channel::{bounded, Receiver, Sender};
pub use errors::DownloaderErrors;
use futures::{stream, StreamExt};
use indicatif::{MultiProgress, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::Display,
    fs::File,
    path::PathBuf,
    time::{Duration, Instant},
};
// use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::{debug, error, info};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloaderConfig {
    pub storage_dir: Option<PathBuf>,
    pub worker_count: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Downloader {
    // The download Queue will send to a worker pool
    receiver: Receiver<Downloadable>,
    // Download Queue
    pub sender: Sender<Downloadable>,
    successfully_processed: i16,
    failed_to_process: i16,
}

impl Downloader {
    pub fn new(conf: &'_ DownloaderConfig) -> Self {
        // let worker_limit = conf.worker_count.unwrap_or(12);
        let (sender, receiver) = unbounded();
        Self {
            receiver,
            sender,
            successfully_processed: 0,
            failed_to_process: 0,
        }
    }

    pub async fn add_downloadables(
        &self,
        // sender: Sender<Downloadable>,
        downloadables: Vec<Downloadable>,
    ) -> AsyncResult<()> {
        // let sender.
        info!(
            "Adding {} items to Queue({})",
            downloadables.len(),
            self.receiver.len()
        );
        // tokio::spawn(async move {
        let sender = self.sender.clone();
        futures::stream::iter(downloadables)
            .for_each_concurrent(10, |queueable| {
                let sender = sender.clone();
                async move {
                    let file_name = queueable.file_name.to_string();
                    match sender.send(queueable) {
                        Ok(_) => {
                            debug!("{} sent to be processed by the downloader", file_name);
                            // Ok(())
                        }
                        Err(err) => {
                            error!("Failed to send ({}) to the downloader: {}", file_name, err)
                        }
                    }
                }
            })
            .await;
        Ok(())
        // for downloadable in downloadables {
        //     let downloadable = downloadable;
        //     let file_name = downloadable.file_name.to_string();
        //     match sender.send(downloadable) {
        //         Ok(_) => debug!("{} sent to be processed by the downloader", file_name),
        //         Err(err) => error!("Failed to send ({}) to the downloader: {}", file_name, err),
        //     }
        // }
        // });
        // Ok(())
    }

    pub async fn process_downloads(self) -> AsyncResult<DownloaderStats> {
        let mut downloader_stats = DownloaderStats::default();
        let mut iterations = 1;
        loop {
            info!("Queue manager loop({}) length", iterations);
            select! {
                recv(self.receiver)-> downloadable => {
                    downloader_stats.total+=1;
                    match downloadable {
                        Ok(downloadable) => {
                            info!("Items still to process {}", self.receiver.len());
                            match downloadable.save_item().await {
                                Ok(bytes_written) => {
                                    if bytes_written == 0 {
                                        info!("{} already existed", downloadable.file_name);
                                        downloader_stats.previously_saved += 1;
                                    } else {
                                        downloader_stats.success += 1;
                                        info!(
                                            "Successfully wrote {} bytes to {}",
                                            bytes_written, downloadable.file_name
                                        );
                                    }
                                }
                                Err(save_err) => {
                                    error!("Failed to save item {:?}", save_err);
                                    downloader_stats.failed += 1;
                                }
                            }
                        },
                        Err(receiver_err) => {
                            error!("Unable to handle more downloadables. {:?}", receiver_err);
                        }
                    }
                },
                default(Duration::from_secs(5)) => {
                    error!("Queue empty for too long");
                    break;
                },
            }

            iterations += 1;
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(downloader_stats)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloaderStats {
    pub total: usize,
    pub failed: usize,
    pub success: usize,
    pub previously_saved: usize,
}

/// base_path.join(additional_path).join(file_name)
#[derive(Debug)]
pub struct Downloadable {
    pub public_url: String,
    pub file_name: String,
    pub base_path: PathBuf,
}

impl Display for Downloadable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.base_path.join(&self.file_name[..]);
        write!(f, "{:?}", path)
    }
}

impl Downloadable {
    async fn save_item(&self) -> AsyncResult<u64> {
        let file_path = &self.base_path.join(&self.file_name);
        if file_path.exists() {
            return Ok(0);
        }
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(&parent).unwrap_or_else(|err| {
                    error!("Unable to create base path {:?}. \n{:?}", parent, err)
                });
            }
        }
        info!("Downloading item {}", &self);
        let response = reqwest::get(&self.public_url).await;
        match response {
            Ok(resp) => {
                debug!("Response from {}. {:?}", &self, resp);
                match resp.bytes().await {
                    Ok(bytes) => {
                        let mut data = bytes.as_ref();
                        match File::create(file_path) {
                            Ok(mut out_file) => match std::io::copy(&mut data, &mut out_file) {
                                Ok(s) => {
                                    info!("Saved file {:?}", file_path);
                                    Ok(s)
                                }
                                Err(save_err) => Err(format!(
                                    "Failed to save data to file {:?}. Error: {:?}",
                                    &out_file, save_err
                                )
                                .into()),
                            },
                            Err(create_err) => {
                                Err(format!("Failed to create file {}: {:?}", &self, create_err)
                                    .into())
                            }
                        }
                    }
                    Err(bytes_err) => {
                        Err(format!("Failed to get response body. {:?}", bytes_err).into())
                    }
                }
            }
            Err(download_err) => {
                Err(format!("Failed to download {}. {:#?}", &self, download_err).into())
            }
        }
    }

    pub fn from_media_with_path(media: &'_ Media, path: PathBuf) -> Self {
        debug!(
            "Creating downloadable for {} in {:?}",
            media.file_name, path
        );
        Self {
            file_name: media.file_name.to_string(),
            base_path: path,
            public_url: media.url.to_string(),
        }
    }
}
