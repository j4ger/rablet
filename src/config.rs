use crate::utils::LogExpect;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

impl Config {
    fn save(&self, path: &PathBuf) {
        let content = serde_json::to_string(self)
            .log_expect(format!("Failed to serialize current config: {:#?}", self));
        let mut file = File::create(path).log_expect("Failed to open config file.");
        file.write_all(content.as_bytes())
            .log_expect("Failed to write to config file.");
    }

    fn new_default(path: &PathBuf) -> Config {
        let config: Config = Default::default();
        config.save(path);
        info!("Generated new config: {:#?}", config);
        config
    }

    fn new_default_prompt(path: &PathBuf) -> Config {
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            stdin
                .read_line(&mut buffer)
                .log_expect("Failed to get stdin input.");
            match buffer.to_lowercase().trim() {
                "y" => {
                    return Config::new_default(path);
                }
                "n" | "" => {
                    panic!("Exiting.")
                }
                _ => {
                    error!("Invalid input. [y/N]");
                }
            }
        }
    }
}

impl Config {
    pub(crate) fn load_config(config_path: &PathBuf) -> Config {
        if config_path.exists() {
            if config_path.is_file() {
                let mut file_content = Vec::new();
                File::open(config_path)
                    .log_expect(format!("Failed to read file {}.", config_path.display()))
                    .read_to_end(&mut file_content);
                match serde_json::from_slice(&file_content) {
                    Ok(config) => config,
                    Err(_err) => {
                        error!("Invalid config file! Remove and create a new default? [y/N]");
                        Config::new_default_prompt(config_path)
                    }
                }
            } else {
                error!(
                    "{} is a directory! Remove and create a new default? [y/N]",
                    config_path.display()
                );
                Config::new_default_prompt(config_path)
            }
        } else {
            info!("Config file does not exist, creating default.");
            Config::new_default(config_path)
        }
    }
}
