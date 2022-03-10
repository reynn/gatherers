use crate::{
    downloaders::{BatchDownloader, Downloadable, DownloaderStats},
    tasks::spawn_on_thread,
};
use async_channel::Receiver;
use async_task::Task;
use futures::lock::Mutex;
use std::fmt::Formatter;
use std::sync::Arc;

#[derive(Debug)]
pub struct MultiThreadedDownloader {
    worker_threads: u8,
    // chunk_size: Option<u32>,
    // min_size_to_chunk: Option<u64>,
    receiver: Receiver<Downloadable>,
}

impl MultiThreadedDownloader {
    pub fn new(w_c: u8, rx: Receiver<Downloadable>) -> Self {
        // let (sender, _) = async_channel::unbounded();
        Self {
            worker_threads: w_c,
            // chunk_size: None,
            // min_size_to_chunk: None,
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

impl std::fmt::Display for MultiThreadedDownloader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[async_trait::async_trait]
impl BatchDownloader for MultiThreadedDownloader {
    fn name(&self) -> String {
        "multi-threaded".into()
    }

    async fn add_item_to_queue(&self, _item: Downloadable) -> crate::Result<()> {
        Ok(())
    }

    async fn process_single_item(&self, _worker_num: usize) -> crate::Result<u64> {
        todo!()
    }

    async fn process_all_items(&self) -> crate::Result<DownloaderStats> {
        let stats = Arc::new(Mutex::new(DownloaderStats::default()));

        // Generate a collection of tasks that will gather sub data
        let worker_threads_2: Vec<Task<()>> = (0..self.worker_threads)
            .map(|worker_num| {
                let worker_num = worker_num + 1;
                let receiver = self.receiver.clone();
                let stats = Arc::clone(&stats);
                spawn_on_thread(async move {
                    log::debug!("W({:2}): Waiting for items...", worker_num);
                    loop {
                        if receiver.is_closed() && receiver.is_empty() {
                            break;
                        }
                        log::debug!(
                            "W({:2}): Senders: {}. Receivers: {}",
                            worker_num,
                            receiver.sender_count(),
                            receiver.receiver_count()
                        );

                        let recv_item = receiver.recv().await;
                        match recv_item {
                            Ok(item) => {
                                let file_path = item.get_file_path();
                                let file_path = file_path.as_path();
                                let file_name = item.file_name.clone();
                                log::debug!(
                                    "W({:2}): Received new item {:?}, {} still left.",
                                    worker_num,
                                    file_name,
                                    receiver.len()
                                );
                                match item.save_item(None).await {
                                    Ok(bw) => {
                                        let mut stats = stats.lock().await;
                                        stats.total += 1;
                                        if bw.eq(&u64::MIN) {
                                            stats.previously_saved += 1;
                                        } else {
                                            log::info!(
                                                "W({:2}): Successfully saved item {:?} wrote {} bytes",
                                                worker_num, file_path, bw
                                            );
                                            stats.success += 1;
                                        }
                                    }
                                    Err(save_err) => {
                                        log::error!(
                                            "W({:2}): Failed to save item {:?}. Save Error: {:?}",
                                            worker_num, file_path, save_err
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
                                log::debug!(
                                    "W({:2}): Failed to receive item from download queue. {:?}",
                                    worker_num, recv_err
                                );
                            }
                        }
                    }
                    log::debug!("W({:2}): No more items to process", worker_num);
                })
            })
            .collect();

        // Work through subscriptions 10 at a time, waiting until they complete
        // as they finish a new should take it's place (TODO: confirm)
        // futures::stream::iter(worker_threads_2)
        //     .for_each_concurrent(10, |sub_worker| async move { sub_worker.await })
        //     .await;

        futures::future::join_all(worker_threads_2).await;

        let stats = stats.lock().await;

        Ok(stats.to_owned())
    }
}
