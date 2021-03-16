use crate::error::{Error, Result};
use crate::utils::{create_file, open_file};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub lang: String,
    pub os_token: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            lang: String::from("en"),
            os_token: String::from(""),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let project_dir = Self::project_dir()?;
        let config_dir = project_dir.config_dir();
        fs::create_dir_all(&config_dir)?;
        let filename = Self::config_path()?;

        match open_file(&filename)? {
            None => {
                let defaults = Config::default();
                defaults.save()?;
                Ok(defaults)
            }
            Some(file) => serde_json::from_reader(file)
                .map_err(|_| Error::MalformedFile(filename.clone()))
                .and_then(|cfg: Config| {
                    if cfg.lang.len() == 0 {
                        Err(Error::MalformedFile(filename))
                    } else {
                        Ok(cfg)
                    }
                }),
        }
    }

    pub fn project_dir() -> Result<ProjectDirs> {
        ProjectDirs::from("xyz", "Aman Harwara", "subtitle").ok_or_else(|| Error::ProjectDir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::project_dir()?.config_dir().join("config.json"))
    }

    pub fn set_os_token(&mut self, token: String) -> Result<()> {
        self.os_token = token;
        self.save()
    }

    pub fn save(&self) -> Result<()> {
        let filename = Self::config_path()?;
        let file = create_file(&filename)?;
        Ok(serde_json::to_writer(file, &self)?)
    }
}
