use super::InMemoryFileDownloader;
use crate::{downloaders::DownloaderErrors, gatherers::Media, Result};
use async_fs::File;
use futures_lite::{
    io::{copy, AsyncWriteExt, BufReader},
    stream, StreamExt,
};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};
use surf::{
    http::{
        headers::{HeaderValue, CONTENT_LENGTH, CONTENT_RANGE},
        Method,
    },
    Request, StatusCode, Url,
};

const DEFAULT_BUFFER_SIZE: u32 = 1024; // ~1 mb
const DEFAULT_MIN_SIZE_TO_CHUNK: u64 = (100 * 1024) * 1024; // roughly 100 mb
const TEST_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36";

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
    pub async fn save_item(
        self,
        file_downloader: Option<Box<dyn super::FileDownloader>>,
    ) -> Result<u64> {
        let file_downloader =
            file_downloader.unwrap_or_else(|| Box::new(InMemoryFileDownloader {}));

        file_downloader
            .download(&self.public_url, self.get_file_path())
            .await
    }

    fn get_file_path(&self) -> PathBuf {
        self.base_path.join(&self.file_name)
    }

    pub fn from_media_with_path(media: &'_ Media, path: PathBuf) -> Self {
        log::debug!(
            "Creating downloadable for {} in {:?}",
            media.file_name,
            path
        );
        Self {
            file_name: media.file_name.to_string(),
            base_path: path,
            public_url: media.url.to_string(),
        }
    }
}
