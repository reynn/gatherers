mod errors;

use crate::{
    directories::Directories,
    downloaders::DownloadersConfig,
    http::ApiClientConfig,
};
use directories::{BaseDirs, ProjectDirs};
pub use errors::ConfigErrors;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub type Result<T, E = errors::ConfigErrors> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config_dir: String,
    pub api_config: Option<ApiClientConfig>,
    pub downloaders: Option<DownloadersConfig>,
    // pub fansly: FanslyConfig,
    // pub only_fans: OnlyFansConfig,
}

impl Config {
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        tracing::debug!("Config::load({:?})", &path);
        Self::load_or_create_config(&path)
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Self> {
        tracing::debug!("Loading config from path: {:?}", &config_dir);
        if !config_dir.exists() {
            tracing::debug!("Creating directory: {:?}", config_dir);
            std::fs::create_dir_all(config_dir)?;
        }
        let conf_file_path = config_dir.join("config.toml");
        match std::fs::read_to_string(&conf_file_path) {
            Ok(file_contents) => Ok({
                let mut s: Self = toml::from_str(&file_contents[..])?;
                if s.config_dir.is_empty() {
                    s.config_dir = String::from(config_dir.to_str().unwrap_or_default());
                };
                s
            }),
            Err(file_read_err) => Err(ConfigErrors::LoadConfigFailure(
                config_dir.to_path_buf(),
                Box::new(file_read_err),
            )),
        }
    }

    /// Save the config file
    fn save(&self) -> Result<()> {
        let config_dir = Path::new(&self.config_dir);
        // save should generally only been called from the drop impl
        // since the config would have tried to load well before a drop would happen we are skipping the dir creation
        std::fs::write(
            config_dir.join("config.toml"),
            toml::to_string_pretty(self)?,
        )?;
        tracing::debug!("Successfully saved config file to {:?}", config_dir);
        Ok(())
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if let Err(err) = self.save() {
            panic!("Failed to save the config file, {:?}", err);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let dirs = Directories::new();
        let config_directory = dirs.get_default_config_dir();
        Self {
            config_dir: String::from(config_directory.to_str().unwrap_or_default()),
            // fansly: FanslyConfig::default(),
            // only_fans: OnlyFansConfig::default(),
            api_config: None,
            downloaders: Some(DownloadersConfig {
                storage_dir: "/tmp".into(),
            }),
        }
    }
}
