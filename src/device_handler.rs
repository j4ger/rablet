use log::debug;

use crate::{
    interfaces::{Button, DeviceState, PartialUpdate, PenStatus},
    platform::{VirtualInput, VirtualInputDevice},
    tablet_device::TabletDevice,
    utils::SelectBit,
};

pub(crate) fn device_handler(tablet: TabletDevice, device_state: DeviceState) {
    // todo: parse input with scripting language
    //
    // todo: listen for kill signal, [https://tokio.rs/tokio/topics/shutdown]
    //
    // todo: async read?

    let mut vinput = VirtualInput::new(&tablet.device_info);

    // todo: larger buffer
    let mut buffer = [0; 12];
    loop {
        if let Some(_) = tablet.read(&mut buffer) {
            match buffer[1] {
                0b10000000 | 0b10000010 | 0b10000100 | 0b10000001 | 0b10000011 | 0b10000101
                | 0b10000111 => {
                    // pen => b10000000
                    // pen primary button => b10000010
                    // pen secondary button => b10000100
                    // pen tip => b10000001
                    let x: u32 = buffer[2] as u32 + ((buffer[3] as u32) << 8);
                    let x = x as f32 / tablet.device_info.width;
                    let y: u32 = buffer[4] as u32 + ((buffer[5] as u32) << 8);
                    let y = y as f32 / tablet.device_info.height;

                    let tilt_x = buffer[10] as i32;
                    let tilt_y = buffer[11] as i32;

                    let pressure = buffer[6] as i32;

                    let pen_status = PenStatus {
                        position: (x, y),
                        pressure: Some(pressure),
                        tilt: Some((tilt_x, tilt_y)),
                    };
                    vinput.submit_cursor(pen_status);

                    let mut device_state = device_state.write();

                    let button_state = buffer[1];

                    let pen_tip = button_state.is_bit_set(7);
                    device_state.update_button(Button::PenTip, pen_tip);
                    // todo: vinput

                    let pen_primary = button_state.is_bit_set(6);
                    device_state.update_button(Button::PenPrimary, pen_primary);

                    let pen_secondary = button_state.is_bit_set(5);
                    device_state.update_button(Button::PenSecondary, pen_secondary);
                }
                0b11110001 => {
                    // wheel
                }
                0b11100000 => {
                    // button
                }
                _ => {}
            }
        };
    }
}
