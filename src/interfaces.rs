//todo: fine grained scope

use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use serde::Deserialize;

use crate::{
    config::Config,
    device_info::{DeviceDB, DeviceInfo},
};

pub(crate) enum PartialUpdate {
    Pen(PenStatus),
    ButtonDown(Button),
    ButtonUp(Button),
}

pub(crate) struct PenStatus {
    pub(crate) position: (u32, u32),
    pub(crate) tilt: Option<i32>,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
pub(crate) enum Button {
    PenPrimary,
    PenSecondary,
    Eraser,
    Pen,
    Tablet(u32),
}

pub(crate) type DeviceID = (u16, u16);

pub(crate) struct DeviceStateInner {
    pub(crate) id: DeviceID,
    pub(crate) pen_position: (u32, u32),
    pub(crate) button_state: HashMap<Button, bool>,
}

pub(crate) type DeviceState = Arc<RwLock<DeviceStateInner>>;

pub(crate) fn new_device_state(device_info: &DeviceInfo) -> DeviceState {
    let mut button_state = HashMap::new();
    for button in &device_info.button_available {
        button_state.insert(button.clone(), false);
    }
    Arc::new(RwLock::new(DeviceStateInner {
        id: device_info.id,
        pen_position: (0, 0),
        button_state,
    }))
}

pub(crate) struct GlobalStateInner {
    pub(crate) devices: Vec<DeviceState>,
    pub(crate) config: Config,
    pub(crate) device_db: DeviceDB,
}

pub(crate) type GlobalState = Arc<RwLock<GlobalStateInner>>;

pub(crate) fn new_global_state(config: Config, device_db: DeviceDB) -> GlobalState {
    Arc::new(RwLock::new(GlobalStateInner {
        devices: Vec::new(),
        config,
        device_db,
    }))
}
