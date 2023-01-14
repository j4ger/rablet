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

use log::error;

pub(crate) trait LogExpect<T> {
    fn log_expect(self, msg: impl AsRef<str>) -> T;
}

impl<T, E: std::fmt::Debug> LogExpect<T> for Result<T, E> {
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
