use gatherer_core::{directories::Directories, Result};
#[cfg(feature = "fansly")]
use gatherer_fansly::FanslyConfig;
#[cfg(feature = "onlyfans")]
use gatherer_onlyfans::OnlyFansConfig;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config_dir: String,
    pub download_dir: String,
    pub workers: u8,
    pub fansly: FanslyConfig,
    pub onlyfans: OnlyFansConfig,
}

impl Config {
    pub fn load_or_default<P: Into<PathBuf>>(path: P) -> Result<Arc<Self>> {
        let path: PathBuf = path.into();
        log::debug!("Config::load({:?})", &path);
        Self::load_or_create_config(&path)
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Arc<Self>> {
        log::debug!("Loading config from path: {:?}", &config_dir);
        if !config_dir.exists() {
            log::debug!("Creating directory: {:?}", config_dir);
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
        log::debug!("Successfully saved config file to {:?}", config_dir);
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
            fansly: FanslyConfig::default(),
            onlyfans: OnlyFansConfig::default(),
            download_dir: String::from("/tmp"),
            workers: 8,
        }
    }
}
