mod errors;

pub use errors::ConfigErrors;
use gatherer_core::{
    directories::Directories, downloaders::DownloaderConfig, http::ApiClientConfig,
};
use gatherer_fansly::FanslyConfig;
use gatherer_onlyfans::OnlyFansConfig;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub type Result<T, E = errors::ConfigErrors> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // #[serde(skip)]
    pub config_dir: String,
    pub api_config: ApiClientConfig,
    pub downloaders: DownloaderConfig,
    pub fansly: FanslyConfig,
    pub only_fans: OnlyFansConfig,
}

impl Config {
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Arc<Self>> {
        let path: PathBuf = path.into();
        tracing::debug!("Config::load({:?})", &path);
        Self::load_or_create_config(&path)
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Arc<Self>> {
        tracing::debug!("Loading config from path: {:?}", &config_dir);
        if !config_dir.exists() {
            tracing::debug!("Creating directory: {:?}", config_dir);
            std::fs::create_dir_all(config_dir)?;
        }
        let conf_file_path = config_dir.join("config.toml");
        let file_contents = std::fs::read_to_string(&conf_file_path)?;
        Ok(Arc::new(toml::from_str(&file_contents[..])?))
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
            api_config: ApiClientConfig::default(),
            downloaders: DownloaderConfig::default(),
            fansly: FanslyConfig::default(),
            only_fans: OnlyFansConfig::default(),
        }
    }
}
