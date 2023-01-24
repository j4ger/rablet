use std::sync::Arc;

use crate::{
    device_handler::device_handler,
    interfaces::{new_device_state, GlobalState},
    tablet_device::TabletDevice,
    utils::LogExpect,
};
use log::{error, info, warn};

//use rusb::hotplug after initial thread spawning

pub(crate) fn spawn_device_pool_thread(mut global_state: GlobalState) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let mut parser_handles = Vec::new();
            'outer: for device in rusb::devices()
                .log_expect("Failed to get USB device list, check permissions.")
                .iter()
            {
                if let Ok(device_desc) = device.device_descriptor() {
                    for existing_config in &global_state.device_db {
                        if (device_desc.vendor_id() == existing_config.id.vid)
                            & (device_desc.product_id() == existing_config.id.pid)
                        {
                            info!(
                                "Found valid device {:04x}:{:04x}, opening.",
                                device_desc.vendor_id(),
                                device_desc.product_id()
                            );
                            if let Ok(device_handle) = device.open() {
                                let tablet = TabletDevice::new(device_handle, existing_config);
                                let device_state = new_device_state(existing_config);
                                global_state.devices.push(Arc::clone(&device_state));
                                let handle = tokio::task::spawn_blocking(|| {
                                    device_handler(tablet, device_state)
                                });
                                parser_handles.push(handle);
                                continue 'outer;
                            } else {
                                error!(
                                    "Failed to open matched device {:04x}:{:04x}.",
                                    device_desc.vendor_id(),
                                    device_desc.product_id()
                                );
                                break;
                            }
                        }
                    }
                } else {
                    warn!(
                        "Failed to get information of device at {}-{}, skipping.",
                        device.address(),
                        device.port_number()
                    );
                }
            }
            futures::future::join_all(parser_handles).await;
        });
}
