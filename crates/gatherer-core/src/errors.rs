use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunErrors {
  #[error("Error found with the [Config] module. {0}")]
  Config(#[from] crate::config::ConfigErrors),
  #[error("Error found with the [Gatherers] module. {0}")]
  Gatherers(#[from] crate::gatherers::GathererErrors),
  #[error("Error found with the [HTTP] module. {0}")]
  Http(#[from] crate::http::HttpErrors),
  #[error("Error found with the [Downloader] module. {0}")]
  Downloader(#[from] crate::downloaders::DownloaderErrors),
}
