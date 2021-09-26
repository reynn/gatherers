use std::{sync::Arc, thread, time::Duration};

use crate::{downloaders::Downloadable, tasks::spawn_on_thread};
use async_channel::{Receiver, Sender};
use futures::lock::Mutex;
use futures_lite::StreamExt;
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct MultiThreadedDownloader {
    worker_threads: usize,
    chunk_size: Option<u32>,
    min_size_to_chunk: Option<u64>,
    sender: Sender<Downloadable>,
    receiver: Receiver<Downloadable>,
}

impl MultiThreadedDownloader {
    pub fn new(w_c: usize) -> Self {
        let (sender, receiver) = async_channel::unbounded();
        Self {
            worker_threads: w_c,
            chunk_size: None,
            min_size_to_chunk: None,
            sender,
            receiver,
        }
    }
}

impl Default for MultiThreadedDownloader {
    fn default() -> Self {
        Self::new(8)
    }
}

#[async_trait::async_trait]
impl super::Downloader for MultiThreadedDownloader {
    async fn add_item_to_queue(&self, item: super::Downloadable) -> crate::AsyncResult<()> {
        Ok(self.sender.try_send(item)?)
    }

    async fn process_single_item(&self, worker_num: usize) -> crate::AsyncResult<u64> {
        todo!()
    }

    async fn get_sender(&self) -> crate::AsyncResult<Sender<Downloadable>> {
        Ok(self.sender.clone())
    }

    async fn process_all_items(&self) -> crate::AsyncResult<super::DownloaderStats> {
        let stats = Arc::new(Mutex::new(super::DownloaderStats::default()));
        let mut worker_threads = Vec::new();
        let chunk_size = self.chunk_size;
        let min_size_to_chunk = self.min_size_to_chunk;

        for worker_num in 0..self.worker_threads {
            let worker_num = worker_num + 1;
            let receiver = self.receiver.clone();
            let stats = Arc::clone(&stats);
            worker_threads.push(spawn_on_thread(async move {
                info!("W({:2}): Waiting for items...", worker_num);
                while !receiver.is_closed() {
                    info!(
                        "W({:2}): Senders: {}. Receivers: {}",
                        worker_num,
                        receiver.sender_count(),
                        receiver.receiver_count()
                    );

                    let recv_item = receiver.recv().await;
                    let mut total_stats = stats.lock().await;
                    total_stats.total += 1;
                    drop(total_stats);

                    match recv_item {
                        Ok(item) => {
                            info!(
                                "W({:2}): Received new item {:?}, {} still left.",
                                worker_num,
                                item.file_name,
                                receiver.len()
                            );
                            match item.save_item(chunk_size, min_size_to_chunk).await {
                                Ok(bw) => {
                                    debug!(
                                        "W({:2}): Successfully saved item {:?} wrote {} bytes",
                                        worker_num, item.file_name, bw
                                    );
                                    let mut stats = stats.lock().await;
                                    stats.success += 1
                                }
                                Err(save_err) => {
                                    error!(
                                        "W({:2}): Failed to saved item {:?}. Error: {:?}",
                                        worker_num, item.file_name, save_err
                                    );
                                    let mut stats = stats.lock().await;
                                    stats.failed += 1
                                }
                            }
                        }
                        Err(recv_err) => {
                            let mut stats = stats.lock().await;
                            stats.failed += 1;
                            error!(
                                "W({:2}): Failed to receive item from download queue. {:?}",
                                worker_num, recv_err
                            );
                            return;
                        }
                    }
                }
                info!("W({:2}): ", worker_num);
            }));
        }

        futures::future::join_all(worker_threads).await;

        let stats = stats.lock().await;

        Ok(stats.to_owned())
    }
}
