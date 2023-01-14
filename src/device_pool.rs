use std::sync::Arc;

use crate::{device_handler::input_parser, interfaces::GlobalState, utils::LogExpect};
use log::{error, info};
use tokio::sync::mpsc;

pub(crate) fn spawn_device_pool_thread(global_state: GlobalState) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let mut parser_handles = Vec::new();
            let (tx, rx) = mpsc::unbounded_channel();
            for device in rusb::devices()
                .log_expect("Failed to get USB device list, check permissions.")
                .iter()
            {
                if let Ok(device_desc) = device.device_descriptor() {
                    for existing_config in &global_state.read().device_db {
                        if (device_desc.vendor_id() == existing_config.id.0)
                            & (device_desc.product_id() == existing_config.id.1)
                        {
                            info!(
                                "Found valid device {:04x}:{:04x}, opening.",
                                device_desc.vendor_id(),
                                device_desc.product_id()
                            );
                            if let Ok(device) = device.open() {
                                let tx_clone = tx.clone();
                                let handle = tokio::spawn(input_parser(
                                    device,
                                    Arc::clone(&global_state),
                                    tx_clone,
                                ));
                                parser_handles.push(handle);
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
                    info!(
                        "Failed to get information of device at {}-{}, skipping.",
                        device.address(),
                        device.port_number()
                    );
                }
            }
        });
}
