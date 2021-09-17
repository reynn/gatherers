use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigErrors {
  // TODO: there is likely a better way to do this but I'm too high to figure it out
  #[error("Failed to load config from specified file, {0}. {1}")]
  LoadConfigFailure(PathBuf, Box<dyn std::error::Error>),
  #[error("Failed to convert from TOML. {0:?}")]
  FromToml(#[from] toml::de::Error),
  #[error("Failed to convert to TOML format. {0:?}")]
  ToToml(#[from] toml::ser::Error),
  #[error("There was an unexpected I/O error: {0}")]
  IoError(#[from] std::io::Error),
}