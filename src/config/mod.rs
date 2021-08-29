mod errors;

use crate::{config::errors::ConfigErrors, gatherers::{
    fansly::{Fansly, FanslyConfig},
    onlyfans::OnlyFansConfig,
}};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub type Result<T, E = errors::ConfigErrors> = std::result::Result<T, E>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config_dir: String,
    pub fansly: FanslyConfig,
    pub only_fans: OnlyFansConfig,
}

impl Config {
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        log::debug!("Config::load({:?})", &path);
        Self::load_or_create_config(&path)
    }

    fn load_or_create_config(config_dir: &Path) -> Result<Self> {
        log::debug!("Loading config from path: {:?}", &config_dir);
        if !config_dir.exists() {
            log::debug!("Creating directory: {:?}", config_dir);
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
        log::debug!("Successfully saved config file to {:?}", config_dir);
        Ok(())
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if let Err(err) = self.save() {
            log::error!("Failed to save the config file, {:?}", err);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_directory = get_default_path();
        Self {
            config_dir: String::from(default_directory.to_str().unwrap_or_default()),
            fansly: FanslyConfig {
                enabled: true,
                auth_token: String::new(),
            },
            only_fans: OnlyFansConfig {
                enabled: true,
                session_token: String::new(),
                user_agent: String::new(),
                app_token: String::new(),
            },
        }
    }
}

pub(crate) fn get_default_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "reynn", env!("CARGO_PKG_NAME")) {
        proj_dirs.config_dir().to_owned()
    } else if let Some(base_dirs) = BaseDirs::new() {
        let app_name = env!("CARGO_PKG_NAME");
        base_dirs.config_dir().join(app_name)
    } else {
        panic!("Unable to determine a home directory. Unable to proceed, ensure your shell is properly configured")
    }
}
