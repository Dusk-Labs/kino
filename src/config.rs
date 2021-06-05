use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    token: Option<String>,
    player: Option<String>,
}

impl Config {
    pub fn set_token(&mut self, tok: Option<String>) {
        self.token = tok;
    }
    pub fn token(&self) -> &Option<String> {
        &self.token
    }
    pub fn set_player(&mut self, val: Option<String>) {
        self.player = val;
    }
    pub fn player(&self) -> &Option<String> {
        &self.player
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        Self::create_if_not_exists()?;
        let config = toml::from_str(&std::fs::read_to_string(Self::config_file_path())?)?;
        Ok(config)
    }
    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        std::fs::write(Self::config_file_path(), toml::to_string(self)?)?;
        Ok(())
    }

    fn config_folder_path() -> PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| {
            let mut dir = dirs::home_dir().expect("No home directory");
            dir.push(".config");
            dir
        });
        config_dir.push("kino");
        config_dir
    }
    fn config_file_path() -> PathBuf {
        let mut path = Self::config_folder_path();
        path.push("config.toml");
        path
    }

    fn create_if_not_exists() -> Result<(), Box<dyn Error>> {
        let config_dir = Self::config_folder_path();
        std::fs::create_dir_all(config_dir)?;
        let config_file = Self::config_file_path();
        if !config_file.exists() {
            std::fs::File::create(config_file)?;
        }
        Ok(())
    }
}
