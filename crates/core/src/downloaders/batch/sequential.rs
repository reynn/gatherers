use {
    crate::{
        downloaders::{BatchDownloader, Downloadable, DownloaderStats},
        Result,
    },
    async_channel::{Receiver, Sender, TrySendError},
    async_trait::async_trait,
    std::{fmt::Formatter, time::Duration},
};

#[derive(Debug, Clone)]
pub struct SequentialDownloader {
    // The download Queue will send to a worker pool
    receiver: Receiver<Downloadable>,
    // Download Queue
    sender: Sender<Downloadable>,
}

impl SequentialDownloader {
    pub fn new() -> Self {
        let (sender, receiver) = async_channel::unbounded();
        Self { receiver, sender }
    }

    pub fn sender(&self) -> Sender<Downloadable> {
        self.sender.clone()
    }
}

impl std::fmt::Display for SequentialDownloader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[async_trait]
impl BatchDownloader for SequentialDownloader {
    fn name(&self) -> String {
        "sequential".into()
    }

    async fn add_item_to_queue(&self, item: Downloadable) -> Result<()> {
        let mut item = item;
        loop {
            match self.sender.try_send(item) {
                Ok(_) => return Ok(()),
                Err(try_err) => match try_err {
                    TrySendError::Full(e) => {
                        async_io::Timer::after(Duration::from_millis(20));
                        item = e;
                    }
                    TrySendError::Closed(_) => {
                        return Err("Download queue has been closed already".into())
                    }
                },
            }
        }
    }

    async fn process_single_item(&self, worker_num: usize) -> Result<u64> {
        let item = self.receiver.try_recv()?;
        let file_name = item.file_name.clone();
        log::debug!("W({}) received a new item: {:?}", worker_num, file_name);
        match item.save_item(None).await {
            Ok(bytes_written) => {
                log::info!("W({}) Successfully downloaded, {:?}", worker_num, file_name);
                Ok(bytes_written)
            }
            Err(down_err) => Err(format!(
                "W({}): Failed to download file {:?}. {:?}",
                worker_num, file_name, down_err
            )
            .into()),
        }
    }

    async fn process_all_items(&self) -> Result<DownloaderStats> {
        let mut stats = DownloaderStats::default();
        loop {
            match self.process_single_item(0).await {
                Ok(_) => {
                    stats.success += 1;
                }
                Err(_) => {
                    stats.failed += 1;
                }
            };
        }
    }
}

impl Default for SequentialDownloader {
    fn default() -> Self {
        Self::new()
    }
}
