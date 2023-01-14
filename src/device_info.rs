use crate::{
    interfaces::{Button, DeviceID},
    utils::LogExpect,
};
use log::{debug, error, info};
use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Deserialize)]
pub(crate) struct DeviceInfo {
    pub(crate) id: DeviceID,
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) button_available: Vec<Button>,
    pub(crate) wheel: bool,
}

pub(crate) type DeviceDB = Vec<DeviceInfo>;

pub(crate) fn load_db(path: &PathBuf) -> DeviceDB {
    if !path.is_dir() {
        error!(
            "Provided device database path {} is not a directory!",
            path.display()
        );
        panic!("Exiting.")
    } else {
        let mut db = Vec::new();
        read_db_dir(path, &mut db);
        info!("Loaded {} device info files.", db.len());
        db
    }
}

fn read_db_dir(path: &PathBuf, db: &mut DeviceDB) -> DeviceDB {
    debug!("Reading {}", path.display());
    if path.is_file() {}
    for entry in path
        .read_dir()
        .log_expect(format!("Failed to access {}.", path.display()))
    {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    read_db_dir(&path, db);
                } else {
                    if let Some(device_info) = parse_device_info(&path) {
                        db.push(device_info);
                    }
                }
            }
            Err(err) => {
                error!("Failed to access file: {}, skipping.", err);
                continue;
            }
        }
    }
    unimplemented!()
}

fn parse_device_info(path: &PathBuf) -> Option<DeviceInfo> {
    let file = File::open(path);
    if let Ok(mut file) = file {
        let mut content = Vec::new();
        if let Ok(_) = file.read_to_end(&mut content) {
            if let Ok(device_info) = serde_json::from_slice::<DeviceInfo>(&content) {
                debug!(
                    "Loaded config for device {:04x}:{:04x}",
                    device_info.id.0, device_info.id.1
                );
                Some(device_info)
            } else {
                info!(
                    "{} is not a valid device info file, skipping.",
                    path.display()
                );
                None
            }
        } else {
            error!("Failed to read file {}, skipping.", path.display());
            None
        }
    } else {
        error!("Failed to read file {}, skipping.", path.display());
        None
    }
}
