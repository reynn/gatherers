use crate::{downloaders::Downloadable, tasks::spawn_on_thread};
use async_channel::{Receiver, Sender};
use async_task::Task;
use futures::{lock::Mutex, StreamExt};
use futures_lite::prelude::*;
// use futures_lite::{future, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::Arc, thread, time::Duration};
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct MultiThreadedDownloader {
    worker_threads: usize,
    chunk_size: Option<u32>,
    min_size_to_chunk: Option<u64>,
    receiver: Receiver<Downloadable>,
}

impl MultiThreadedDownloader {
    pub fn new(w_c: usize, rx: Receiver<Downloadable>) -> Self {
        // let (sender, _) = async_channel::unbounded();
        Self {
            worker_threads: w_c,
            chunk_size: None,
            min_size_to_chunk: None,
            receiver: rx,
        }
    }
}

impl Default for MultiThreadedDownloader {
    fn default() -> Self {
        let (_, rx) = async_channel::unbounded();
        Self::new(8, rx)
    }
}

#[async_trait::async_trait]
impl super::Downloader for MultiThreadedDownloader {
    async fn add_item_to_queue(&self, item: super::Downloadable) -> crate::AsyncResult<()> {
        Ok(())
    }

    async fn process_single_item(&self, worker_num: usize) -> crate::AsyncResult<u64> {
        todo!()
    }

    async fn process_all_items(&self) -> crate::AsyncResult<super::DownloaderStats> {
        let stats = Arc::new(Mutex::new(super::DownloaderStats::default()));
        let chunk_size = self.chunk_size;
        let min_size_to_chunk = self.min_size_to_chunk;
        // let mut worker_threads = Vec::new();

        // for worker_num in 0..self.worker_threads {
        //     let worker_num = worker_num + 1;
        //     let receiver = self.receiver.clone();
        //     let stats = Arc::clone(&stats);
        //     worker_threads.push(spawn_on_thread(async move {
        //         debug!("W({:2}): Waiting for items...", worker_num);
        //         loop {
        //             if receiver.is_closed() && receiver.is_empty() {
        //                 break;
        //             }
        //             debug!(
        //                 "W({:2}): Senders: {}. Receivers: {}",
        //                 worker_num,
        //                 receiver.sender_count(),
        //                 receiver.receiver_count()
        //             );
        //             let recv_item = receiver.recv().await;
        //             let mut total_stats = stats.lock().await;
        //             total_stats.total += 1;
        //             drop(total_stats);
        //             match recv_item {
        //                 Ok(item) => {
        //                     debug!(
        //                         "W({:2}): Received new item {:?}, {} still left.",
        //                         worker_num,
        //                         item.file_name,
        //                         receiver.len()
        //                     );
        //                     match item.save_item(chunk_size, min_size_to_chunk).await {
        //                         Ok(bw) => {
        //                             let mut stats = stats.lock().await;
        //                             if bw.eq(&u64::MIN) {
        //                                 stats.previously_saved += 1;
        //                             } else {
        //                                 info!(
        //                                     "W({:2}): Successfully saved item {:?} wrote {} bytes",
        //                                     worker_num, item.file_name, bw
        //                                 );
        //                                 stats.success += 1;
        //                             }
        //                         }
        //                         Err(save_err) => {
        //                             error!(
        //                                 "W({:2}): Failed to save item {}. Save Error: {:?}",
        //                                 worker_num, item, save_err
        //                             );
        //                             let mut stats = stats.lock().await;
        //                             stats.failed += 1;
        //                         }
        //                     }
        //                 }
        //                 Err(recv_err) => {
        //                     // let mut stats = stats.lock().await;
        //                     // stats.failed += 1;
        //                     debug!(
        //                         "W({:2}): Failed to receive item from download queue. {:?}",
        //                         worker_num, recv_err
        //                     );
        //                 }
        //             }
        //         }
        //         debug!("W({:2}): No more items to process", worker_num);
        //     }));
        // }

        // Generate a collection of tasks that will gather sub data
        let worker_threads_2: Vec<Task<()>> = (0..self.worker_threads)
            .map(|worker_num| {
                let worker_num = worker_num + 1;
                let receiver = self.receiver.clone();
                let stats = Arc::clone(&stats);
                spawn_on_thread(async move {
                    debug!("W({:2}): Waiting for items...", worker_num);
                    loop {
                        if receiver.is_closed() && receiver.is_empty() {
                            break;
                        }
                        debug!(
                            "W({:2}): Senders: {}. Receivers: {}",
                            worker_num,
                            receiver.sender_count(),
                            receiver.receiver_count()
                        );

                        let recv_item = receiver.recv().await;
                        match recv_item {
                            Ok(item) => {
                                debug!(
                                    "W({:2}): Received new item {:?}, {} still left.",
                                    worker_num,
                                    item.file_name,
                                    receiver.len()
                                );
                                match item.save_item(chunk_size, min_size_to_chunk).await {
                                    Ok(bw) => {
                                        let mut stats = stats.lock().await;
                                        stats.total += 1;
                                        if bw.eq(&u64::MIN) {
                                            stats.previously_saved += 1;
                                        } else {
                                            info!(
                                                "W({:2}): Successfully saved item {:?} wrote {} bytes",
                                                worker_num, item.file_name, bw
                                            );
                                            stats.success += 1;
                                        }
                                    }
                                    Err(save_err) => {
                                        error!(
                                            "W({:2}): Failed to save item {}. Save Error: {:?}",
                                            worker_num, item, save_err
                                        );
                                        let mut stats = stats.lock().await;
                                        stats.total += 1;
                                        stats.failed += 1;
                                    }
                                }
                            }
                            Err(recv_err) => {
                                // let mut stats = stats.lock().await;
                                // stats.failed += 1;
                                debug!(
                                    "W({:2}): Failed to receive item from download queue. {:?}",
                                    worker_num, recv_err
                                );
                            }
                        }
                    }
                    debug!("W({:2}): No more items to process", worker_num);
                })
            })
            .collect();

        // Work through subscriptions 10 at a time, waiting until they complete
        // as they finish a new should take it's place (TODO: confirm)
        futures::stream::iter(worker_threads_2)
            .for_each_concurrent(10, |sub_worker| async move { sub_worker.await })
            .await;

        // futures::future::join_all(worker_threads).await;

        let stats = stats.lock().await;

        Ok(stats.to_owned())
    }
}