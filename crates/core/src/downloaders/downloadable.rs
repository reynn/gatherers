use {
    super::InMemoryFileDownloader,
    crate::{gatherers::Media, Result},
    std::{fmt::Display, path::PathBuf},
};

// const DEFAULT_BUFFER_SIZE: u32 = 1024; // ~1 mb
// const DEFAULT_MIN_SIZE_TO_CHUNK: u64 = (100 * 1024) * 1024; // roughly 100 mb

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

    pub fn get_file_path(&self) -> PathBuf {
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
