use super::{ClickInput, InputKey, VirtualInputDevice};
use crate::{device_info::DeviceInfo, interfaces::PenStatus, utils::LogExpect};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, EventType, InputEvent, Key, UinputAbsSetup,
};
use keycode::KeyMap;
use log::warn;

const ABS_X_MAX: i32 = 3840 - 1;
const ABS_Y_MAX: i32 = 2160 - 1;
const ABS_FUZZ: i32 = 20;
const ABS_FLAT: i32 = 20;
const ABS_RES: i32 = 1;
const PRESSURE_MAX: i32 = 8192 - 1;
const TILT_MIN: i32 = -128 + 1;
const TILT_MAX: i32 = 128 - 1;

pub(crate) struct VirtualInput {
    evdev: VirtualDevice,
    buffer: Vec<InputEvent>, // todo: buffer?
}

impl VirtualInputDevice for VirtualInput {
    fn new(device_info: &DeviceInfo) -> Self {
        // source: rustdesk/rustdesk
        let mut keys = AttributeSet::<Key>::new();
        for i in evdev::Key::KEY_ESC.code()..(evdev::Key::BTN_TRIGGER_HAPPY40.code() + 1) {
            let key = evdev::Key::new(i);
            if !format!("{:?}", &key).contains("unknown key") {
                keys.insert(key);
            }
        }
        let abs_x_info = AbsInfo::new(ABS_X_MAX / 2, 0, ABS_X_MAX, ABS_FUZZ, ABS_FLAT, ABS_RES);
        let abs_x = UinputAbsSetup::new(AbsoluteAxisType::ABS_X, abs_x_info);
        let abs_y_info = AbsInfo::new(ABS_Y_MAX / 2, 0, ABS_Y_MAX, ABS_FUZZ, ABS_FLAT, ABS_RES);
        let abs_y = UinputAbsSetup::new(AbsoluteAxisType::ABS_Y, abs_y_info);
        let abs_pressure_info = AbsInfo::new(
            PRESSURE_MAX / 2,
            0,
            PRESSURE_MAX,
            ABS_FUZZ,
            ABS_FLAT,
            ABS_RES,
        );
        let abs_pressure = UinputAbsSetup::new(AbsoluteAxisType::ABS_PRESSURE, abs_pressure_info);
        let abs_tilt_info = AbsInfo::new(0, TILT_MIN, TILT_MAX, ABS_FUZZ, ABS_FLAT, ABS_RES);
        let abs_tilt_x = UinputAbsSetup::new(AbsoluteAxisType::ABS_TILT_X, abs_tilt_info);
        let abs_tilt_y = UinputAbsSetup::new(AbsoluteAxisType::ABS_TILT_Y, abs_tilt_info);

        let virtual_device = VirtualDeviceBuilder::new()
            .log_expect("Failed to create virtual input device.")
            .name(format!("rablet - {}", device_info.id).as_str())
            .with_keys(&keys)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_x)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_y)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_pressure)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_tilt_x)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_tilt_y)
            .log_expect("Failed to submit virtual input device capabilities.")
            .build()
            .log_expect("Failed to virtual input device.");

        let buffer = Vec::with_capacity(5);

        Self {
            evdev: virtual_device,
            buffer,
        }
    }

    fn submit_cursor(&mut self, update_info: PenStatus) {
        self.buffer.clear();
        let x = (update_info.position.0 * ABS_X_MAX as f32) as i32;
        let y = (update_info.position.1 * ABS_Y_MAX as f32) as i32;
        self.buffer.push(InputEvent::new(
            EventType::ABSOLUTE,
            AbsoluteAxisType::ABS_X.0,
            x,
        ));
        self.buffer.push(InputEvent::new(
            EventType::ABSOLUTE,
            AbsoluteAxisType::ABS_Y.0,
            y,
        ));
        if let Some((tilt_x, tilt_y)) = update_info.tilt {
            self.buffer.push(InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_TILT_X.0,
                tilt_x,
            ));
            self.buffer.push(InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_TILT_Y.0,
                tilt_y,
            ));
        }
        if let Some(pressure) = update_info.pressure {
            self.buffer.push(InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_PRESSURE.0,
                pressure,
            ));
        }
        self.evdev.emit(&self.buffer).unwrap_or_else(|_| {
            warn!("Failed to emit input event.");
        });
    }

    fn submit_action(&mut self, action: super::InputAction, pressed: bool) {
        self.buffer.clear();
        if action.super_key {
            self.buffer.push(InputEvent::new(EventType::KEY, 125u16, 1));
            // KEY_LEFTMETA
        }
        if action.ctrl {
            self.buffer.push(InputEvent::new(EventType::KEY, 29u16, 1));
            // KEY_LEFTCTRL
        }
        if action.alt {
            self.buffer.push(InputEvent::new(EventType::KEY, 56u16, 1));
            // KEY_LEFTCTRL
        }
        if action.shift {
            self.buffer.push(InputEvent::new(EventType::KEY, 42u16, 1));
            // KEY_LEFTCTRL
        }

        let key = match action.key {
            InputKey::Keyboard(key) => KeyMap::from(key).evdev,
            InputKey::Mouse(key) => match key {
                ClickInput::LeftClick => 0x110,
                ClickInput::RightClick => 0x111,
                ClickInput::MiddleClick => 0x112,
                ClickInput::Touch => 0x14a,
            },
        };

        self.buffer
            .push(InputEvent::new(EventType::KEY, key, pressed as i32));

        self.evdev.emit(&self.buffer).unwrap_or_else(|_| {
            warn!("Failed to emit input event.");
        });
    }
}
