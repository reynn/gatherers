mod in_memory;
mod streaming;

pub use self::{in_memory::InMemoryFileDownloader, streaming::StreamingFileDownloader};
use {async_trait::async_trait, std::path::PathBuf};

#[async_trait]
pub trait FileDownloader: Send + Sync {
    async fn download(&self, url: &'_ str, output_path: PathBuf) -> crate::Result<u64>;
}
