use {
    directories::{BaseDirs, ProjectDirs},
    std::path::{Path, PathBuf},
};

pub struct Directories<'a> {
    project_dirs: Option<ProjectDirs>,
    base_dirs: Option<BaseDirs>,
    app_name: &'a str,
}

impl<'a> Directories<'a> {
    pub fn new() -> Self {
        let app_name = "gatherers";
        let project_dirs = ProjectDirs::from("dev", "reynn", app_name);
        let base_dirs = BaseDirs::new();
        Self {
            project_dirs,
            base_dirs,
            app_name,
        }
    }

    pub fn get_default_download_dir(&self) -> PathBuf {
        match &self.project_dirs {
            Some(project_dirs) => project_dirs.config_dir().to_path_buf(),
            None => match &self.base_dirs {
                Some(base_dirs) => base_dirs.data_local_dir().join(self.app_name),
                None => panic!("Unable to determine a config directory for this machine."),
            },
        }
    }

    pub fn get_default_config_dir(&self) -> PathBuf {
        match &self.project_dirs {
            Some(project_dirs) => project_dirs.config_dir().to_path_buf(),
            None => match &self.base_dirs {
                Some(base_dirs) => base_dirs.config_dir().to_path_buf(),
                None => panic!("Unable to determine a config directory for this machine."),
            },
        }
    }

    pub fn get_default_temp_dir(&self) -> PathBuf {
        let tmp_dir = std::env::var("TEMP_DIR").unwrap_or_default();
        Path::new(&tmp_dir).into()
    }
}

impl<'a> Default for Directories<'a> {
    fn default() -> Self {
        Self::new()
    }
}
