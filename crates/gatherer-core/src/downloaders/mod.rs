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
    pub receiver: Receiver<Downloadable>,
    sender: Sender<Downloadable>,
}

impl Downloader {
    pub fn new(conf: &'_ DownloaderConfig) -> Self {
        let worker_limit = conf.worker_count.unwrap_or(32);
        let (sender, receiver) = channel(worker_limit);
        Self { receiver, sender }
    }

    pub async fn add_downloadables(&self, downloadables: Vec<Downloadable>) -> AsyncResult<()> {
        info!("Adding {} items to the queue", downloadables.len());
        let channel = self.sender.clone();
        tokio::spawn(async move {
            for downloadable in downloadables {
                let downloadable = downloadable;
                let file_name = downloadable.file_name.to_string();
                match channel.send(downloadable).await {
                    Ok(_) => info!("Sent ({}) to the download queue", file_name),
                    Err(err) => error!("Failed to send ({}) to the downloader: {}", file_name, err),
                }
            }
        });
        Ok(())
    }

    pub async fn process_downloads(receiver: Receiver<Downloadable>) {
        info!("Starting to process downloads");
        let mut receiver = receiver;
        // tokio::spawn(async move {
        while let Some(downloadable) = receiver.recv().await {
            let file_path = &downloadable.base_path.join(&downloadable.file_name);
            if file_path.exists() {
                println!("File {} already exists", downloadable.file_name);
                continue;
            }
            // if downloadable.base_path
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent).unwrap_or_else(|err| {
                        error!("Unable to create base path {:?}. \n{:?}", parent, err)
                    });
                }
            }
            info!(
                "Downloading item {} from {}",
                &downloadable, &downloadable.public_url
            );
            let response = reqwest::get(&downloadable.public_url).await;
            match response {
                Ok(resp) => {
                    debug!("Response from {}. {:?}", &downloadable, resp);
                    if let Ok(bytes) = resp.bytes().await {
                        let mut data = bytes.as_ref();
                        match File::create(file_path) {
                            Ok(mut out_file) => match std::io::copy(&mut data, &mut out_file) {
                                Ok(_) => println!("Successfully downloaded {}", &downloadable),
                                Err(save_err) => println!(
                                    "Failed to save data to file {:?}. Error: {:?}",
                                    &out_file, save_err
                                ),
                            },
                            Err(create_err) => {
                                error!("Failed to create file {}: {:?}", &downloadable, create_err);
                            }
                        }
                    } else {
                        error!("Failed to get response body");
                    }
                }
                Err(download_err) => {
                    error!("Failed to download {}. {:#?}", &downloadable, download_err);
                    println!("Failed to download download {}", &downloadable);
                }
            }
        }
        println!("Completed downloading all items");
        // });
    }
}

/// base_path.join(additional_path).join(file_name)
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
    pub fn from_media_with_path(media: Media, path: PathBuf) -> Self {
        info!("Creating downloadable for {} in {:?}", media.filename, path);
        Self {
            file_name: media.filename,
            base_path: path,
            public_url: media.url,
        }
    }
}
