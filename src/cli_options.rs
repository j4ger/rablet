// todo: log expect
use std::path::PathBuf;

use clap::Parser;
use log::debug;

use crate::utils::LogExpect;

#[derive(Parser, Debug)]
#[command(author, version)]
pub(crate) struct CliOptions {
    //// (optional) config file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(short, long)]
    device_db: Option<PathBuf>,
}

impl CliOptions {
    pub(crate) fn get_config(&self) -> &PathBuf {
        self.config.as_ref().unwrap()
    }

    pub(crate) fn get_device_db(&self) -> &PathBuf {
        self.device_db.as_ref().unwrap()
    }
}

pub(crate) fn parse_cli_options() -> CliOptions {
    let mut options = CliOptions::parse();
    debug!("Read command-line options: {:#?}", options);
    if options.config.is_none() {
        options.config=Some(dirs::config_dir().log_expect(
            "No config file provided, nor can the location of default config can be located.",
        ).join("rablet").join("config.json"));
    };
    if options.device_db.is_none() {
        options.device_db=Some(dirs::config_dir().log_expect(
            "No device database path provided, nor can the location of default device database can be located.",
        ).join("rablet").join("device_db"));
    };
    options
}
