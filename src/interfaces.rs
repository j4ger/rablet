//todo: fine grained scope

use crate::{
    config::Config,
    device_info::{DeviceDB, DeviceInfo},
};
use log::info;
use parking_lot::RwLock;
use serde::Deserialize;
use std::{collections::HashSet, fmt::Display, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub(crate) enum PartialUpdate {
    Pen(PenStatus),
    Button(Button, ButtonState),
    Wheel(WheelDirection),
}

#[derive(Debug)]
pub(crate) struct PenStatus {
    pub(crate) position: (f32, f32),
    pub(crate) tilt: Option<(i32, i32)>,
    pub(crate) pressure: Option<i32>,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub(crate) enum Button {
    PenPrimary,
    PenSecondary,
    Eraser,
    PenTip,
    Tablet(u32),
    Wheel,
}

#[derive(Debug, Clone)]
pub(crate) enum ButtonState {
    Press,
    Release,
}

impl From<bool> for ButtonState {
    fn from(input: bool) -> Self {
        if input {
            Self::Press
        } else {
            Self::Release
        }
    }
}

impl ButtonState {
    pub(crate) fn code(&self) -> i32 {
        match self {
            Self::Press => 1,
            Self::Release => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WheelDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug, Deserialize, Copy)]
#[serde(try_from = "String")]
pub(crate) struct DeviceID {
    pub(crate) vid: u16,
    pub(crate) pid: u16,
}

impl TryFrom<String> for DeviceID {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut segments = value.split(":");
        let vid = segments.next().ok_or("Invalid format.")?;
        let vid = u16::from_str_radix(&vid, 16).map_err(|_| "Invalid VID.")?;
        let pid = segments.next().ok_or("Invalid format.")?;
        let pid = u16::from_str_radix(&pid, 16).map_err(|_| "Invalid PID.")?;
        Ok(Self { vid, pid })
    }
}

impl Display for DeviceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}:{:04x}", self.vid, self.pid);
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct DeviceStateInner {
    pub(crate) id: DeviceID,
    pub(crate) pen_position: (u32, u32),
    pub(crate) button_state: HashSet<Button>,
    pub(crate) tilt: Option<u32>,
}

impl DeviceStateInner {
    pub(crate) fn update_button(
        &mut self,
        button: Button,
        pressed: bool,
        sender: &UnboundedSender<PartialUpdate>,
    ) {
        if self.button_state.contains(&button) ^ pressed {
            if pressed {
                self.button_state.insert(button);
                sender
                    .send(PartialUpdate::Button(button, ButtonState::Press))
                    .unwrap_or_else(|_| {
                        info!("Failed to send partial update.");
                    });
            } else {
                self.button_state.remove(&button);
                sender
                    .send(PartialUpdate::Button(button, ButtonState::Release))
                    .unwrap_or_else(|_| {
                        info!("Failed to send partial update.");
                    });
            }
        }
    }
}

pub(crate) type DeviceState = Arc<RwLock<DeviceStateInner>>;

pub(crate) fn new_device_state(device_info: &DeviceInfo) -> DeviceState {
    let mut button_state = HashSet::new();
    Arc::new(RwLock::new(DeviceStateInner {
        id: device_info.id,
        pen_position: (0, 0),
        button_state,
        tilt: None,
    }))
}

#[derive(Debug)]
pub(crate) struct GlobalState {
    pub(crate) devices: Vec<DeviceState>,
    pub(crate) config: Arc<RwLock<Config>>,
    pub(crate) device_db: DeviceDB,
}

pub(crate) fn new_global_state(config: Config, device_db: DeviceDB) -> GlobalState {
    GlobalState {
        devices: Vec::new(),
        config: Arc::new(RwLock::new(config)),
        device_db,
    }
}
