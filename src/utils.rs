use log::error;

use crate::{device_info::DeviceInfo, interfaces::DeviceID, tablet_device::TabletDevice};

pub(crate) fn list_devices() {
    println!("Printing all available hid devices:");

    for device in rusb::devices()
        .expect("Failed to get devices, chech permissions.")
        .iter()
    {
        let device_desc = device
            .device_descriptor()
            .expect("Failed to get device descriptor.");
        println!(
            "{:04x}:{:04x}",
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }
}

pub(crate) fn print_huion_device_input() {
    const VID: u16 = 0x256c;
    const PID: u16 = 0x006d;
    let id = DeviceID { vid: VID, pid: PID };
    let device_info = DeviceInfo {
        id,
        width: 2000f32,
        height: 1000f32,
        button_available: Vec::new(),
        wheel: true,
        packet_length: 12,
    };
    let device = rusb::open_device_with_vid_pid(VID, PID).expect("Failed to open test device.");
    let tablet = TabletDevice::new(device, &device_info);
    let mut buffer = [0; 12];
    loop {
        tablet.read(&mut buffer);
        println!("Read data: {:?}", buffer);
    }
}

pub(crate) trait LogExpect<T> {
    fn log_expect(self, msg: impl AsRef<str>) -> T;
}

// todo: better call-site indication, probably through macros

impl<T, E: std::fmt::Debug> LogExpect<T> for Result<T, E> {
    #[track_caller]
    fn log_expect(self, message: impl AsRef<str>) -> T {
        match self {
            Ok(inner) => inner,
            Err(error) => {
                error!("{} - {:?}", message.as_ref(), error);
                panic!("Exiting.");
            }
        }
    }
}

impl<T> LogExpect<T> for Option<T> {
    #[track_caller]
    fn log_expect(self, msg: impl AsRef<str>) -> T {
        match self {
            Some(inner) => inner,
            None => {
                error!("{}", msg.as_ref());
                panic!("Exiting.");
            }
        }
    }
}

pub(crate) trait SelectBit {
    fn is_bit_set(&self, index: Self) -> bool;
}

impl SelectBit for u8 {
    fn is_bit_set(&self, index: Self) -> bool {
        (self >> index) & 1 == 1
    }
}
