mod linux;
mod macos;
mod windows;

use keycode::KeyMappingId;
use std::fmt::Display;

#[cfg(target_os = "linux")]
pub(crate) use linux::VirtualInput;

#[cfg(target_os = "windows")]
pub(crate) use windows::VirtualInput;

#[cfg(target_os = "macos")]
pub(crate) use macos::VirtualInput;

use crate::{
    device_info::DeviceInfo,
    interfaces::{Button, ButtonState, PenStatus},
};

pub(crate) trait VirtualInputDevice {
    fn new(device_info: &DeviceInfo) -> Self;
    fn submit_cursor(&mut self, update_info: PenStatus);
    fn submit_action(&mut self, action: InputAction, pressed: bool);
}

#[derive(Debug, Clone)]
pub(crate) enum ClickInput {
    LeftClick,
    RightClick,
    MiddleClick,
    Touch,
}

impl Display for ClickInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::LeftClick => write!(f, "Left Click"),
            &Self::RightClick => write!(f, "Right Click"),
            &Self::MiddleClick => write!(f, "Middle Click"),
            &Self::Touch => write!(f, "Touch"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum InputKey {
    Keyboard(KeyMappingId),
    Mouse(ClickInput),
}

impl Display for InputKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyboard(inner) => write!(f, "K({inner})"),
            Self::Mouse(inner) => write!(f, "M({inner})"),
        }
    }
}

pub(crate) struct InputAction {
    shift: bool,
    ctrl: bool,
    alt: bool,
    super_key: bool,
    key: InputKey, // use Option for modifiers-only situation?
}

impl Display for InputAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.super_key {
            write!(f, "Super ")?;
        }
        if self.ctrl {
            write!(f, "Ctrl ")?;
        }
        if self.alt {
            write!(f, "Alt ")?;
        }
        if self.shift {
            write!(f, "Shift ")?;
        }
        write!(f, "{}", self.key)
    }
}
