mod errors;

use crate::{gatherers::Media, AsyncResult};
// use async_channel::{bounded, Receiver, Sender};
pub use errors::DownloaderErrors;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, fmt::Display, fs::File, path::PathBuf};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::{debug, error, info};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DownloaderConfig {
    pub storage_dir: Option<PathBuf>,
    pub worker_count: Option<usize>,
}

pub struct Downloader {
    receiver: Receiver<Downloadable>,
    pub sender: Sender<Downloadable>,
    successfully_processed: i16,
    failed_to_process: i16,
}

impl Downloader {
    pub fn new(conf: &'_ DownloaderConfig) -> Self {
        let worker_limit = conf.worker_count.unwrap_or(8);
        let (sender, receiver) = channel(worker_limit);
        Self {
            receiver,
            sender,
            successfully_processed: 0,
            failed_to_process: 0,
        }
    }

    pub async fn add_downloadables(
        sender: Sender<Downloadable>,
        downloadables: Vec<Downloadable>,
    ) -> AsyncResult<()> {
        info!("Adding {} items to the queue", downloadables.len());
        // tokio::spawn(async move {
        for downloadable in downloadables {
            let downloadable = downloadable;
            let file_name = downloadable.file_name.to_string();
            match sender.send(downloadable).await {
                Ok(_) => debug!("{} sent to be processed by the downloader", file_name),
                Err(err) => error!("Failed to send ({}) to the downloader: {}", file_name, err),
            }
        }
        // });
        Ok(())
    }

    pub async fn process_downloads(self) -> AsyncResult<i16> {
        // tokio::spawn(async move {
        println!("Starting to process downloads");
        let mut receiver = self.receiver;

        while let Some(downloadable) = receiver.recv().await {
            // println!("Sender still open? {}", self.sender.  );
            match downloadable.save_item().await {
                Ok(bytes_written) => {
                    if bytes_written == 0 {
                        println!("{} already existed", downloadable.file_name);
                        // Ok(())
                    } else {
                        println!(
                            "Successfully wrote {} bytes to {}",
                            bytes_written, downloadable.file_name
                        );
                        // Ok(())
                    }
                }
                Err(save_err) => {
                    error!("Failed to save item {:?}", save_err);
                    // Err(format!("Failed to save item {:?}", save_err))
                }
            }
        }
        receiver.close();
        // })
        Ok(0)
    }
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
            info!("File {} already exists", self.file_name);
            return Ok(0);
        }
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(&parent).unwrap_or_else(|err| {
                    error!("Unable to create base path {:?}. \n{:?}", parent, err)
                });
            }
        }
        info!("Downloading item {} from {}", &self, &self.public_url);
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
