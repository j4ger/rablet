use crate::{
    interfaces::{Button, ButtonState},
    utils::LogExpect,
};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, EventType, InputEvent, Key, UinputAbsSetup,
};
use log::warn;
// todo: remove once_cell
use std::{cell::RefCell, collections::HashMap};

const ABS_X_MAX: i32 = 3840 - 1;
const ABS_Y_MAX: i32 = 2160 - 1;
const ABS_FUZZ: i32 = 20;
const ABS_FLAT: i32 = 20;
const ABS_RES: i32 = 1;
const PRESSURE_MAX: i32 = 8192 - 1;
const TILT_MIN: i32 = -128 + 1;
const TILT_MAX: i32 = 128 - 1;

thread_local! {
    static EVDEV: RefCell<VirtualDevice> = {
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

        RefCell::new(VirtualDeviceBuilder::new()
            .log_expect("Failed to create virtual input device.")
            .name("rablet uinput source")
            .with_keys(&keys)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_x)
            .log_expect("Failed to submit virtual input device capabilities.")
            .with_absolute_axis(&abs_y)
            .log_expect("Failed to submit virtual input device capabilities.")
            .build()
            .log_expect("Failed to virtual input device."))
    };

    static KEY_MAPPING: RefCell<HashMap<Button, u16>> = {
        let mut mapping = HashMap::new();
        mapping.insert(Button::PenTip, Key::BTN_TOUCH.code());
        mapping.insert(Button::PenPrimary, Key::BTN_STYLUS.code());
        mapping.insert(Button::PenSecondary, Key::BTN_STYLUS2.code());

        RefCell::new(mapping)
    };
}

// todo: notification
pub(crate) fn update_cursor(position: (f32, f32), tilt: Option<(i32, i32)>, pressure: Option<i32>) {
    let x = (position.0 * ABS_X_MAX as f32) as i32;
    let y = (position.1 * ABS_Y_MAX as f32) as i32;
    let mut events = Vec::with_capacity(5);
    events.push(InputEvent::new(
        EventType::ABSOLUTE,
        AbsoluteAxisType::ABS_X.0,
        x,
    ));
    events.push(InputEvent::new(
        EventType::ABSOLUTE,
        AbsoluteAxisType::ABS_Y.0,
        y,
    ));
    if let Some((tilt_x, tilt_y)) = tilt {
        events.push(InputEvent::new(
            EventType::ABSOLUTE,
            AbsoluteAxisType::ABS_TILT_X.0,
            tilt_x,
        ));
        events.push(InputEvent::new(
            EventType::ABSOLUTE,
            AbsoluteAxisType::ABS_TILT_Y.0,
            tilt_y,
        ));
    }
    if let Some(pressure) = pressure {
        events.push(InputEvent::new(
            EventType::ABSOLUTE,
            AbsoluteAxisType::ABS_PRESSURE.0,
            pressure,
        ));
    }
    EVDEV.with(|device| {
        device.borrow_mut().emit(&events).unwrap_or_else(|_| {
            warn!("Failed to emit input event.");
        });
    });
}

// todo: dynamic mapping

pub(crate) fn update_button(button: Button, button_state: ButtonState) {
    if let Some(key) = KEY_MAPPING.with(|mapping| mapping.borrow().get(&button).cloned()) {
        let events = [InputEvent::new(EventType::KEY, key, button_state.code())];
        EVDEV.with(|device| {
            device.borrow_mut().emit(&events).unwrap_or_else(|_| {
                warn!("Failed to emit input event.");
            });
        });
    }
}
