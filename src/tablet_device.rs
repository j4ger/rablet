use crate::{device_info::DeviceInfo, utils::LogExpect};
use log::{debug, error};
use rusb::{DeviceHandle, GlobalContext};
use std::{thread, time::Duration};

const READ_TIMEOUT: Duration = Duration::new(10, 0);
const RESET_WAIT_TIMEOUT: Duration = Duration::from_millis(50);

pub(crate) struct TabletDevice {
    device: DeviceHandle<GlobalContext>,
    endpoint: u8,
    pub(crate) device_info: DeviceInfo,
}

impl TabletDevice {
    pub(crate) fn new(
        mut handle: DeviceHandle<GlobalContext>,
        device_info: &DeviceInfo,
    ) -> TabletDevice {
        let device = handle.device();

        // source: https://github.com/DavidBM/huion-linux-driver-rust

        let config_descriptor = device
            .active_config_descriptor()
            .log_expect("Failed to get active config descriptor.");

        //println!("Active config descriptor: {:?}", config_descriptor);
        debug!(
            "Active config descriptor number: {:?}",
            config_descriptor.number()
        );

        debug!(
            "Interfaces count: {:?}",
            config_descriptor.interfaces().count()
        );

        let active_config = handle.active_configuration();

        debug!("Current active config: {:?}", active_config);

        if let Ok(config) = active_config {
            if config == 1 {
                thread::sleep(RESET_WAIT_TIMEOUT);
                debug!(
                    "Set active config to 1: {:?}",
                    handle.set_active_configuration(1)
                );
            }
        }

        debug!("Finding interfaces...");

        let mut available_endpoints: Vec<u8> = vec![];

        for interface in config_descriptor.interfaces() {
            let interface_number = interface.number();

            let interface_descriptors: Vec<rusb::InterfaceDescriptor> =
                interface.descriptors().collect();

            let interface_descriptor = &interface_descriptors[0];

            let endpoint_descriptors: Vec<rusb::EndpointDescriptor> =
                interface_descriptor.endpoint_descriptors().collect();

            let endpoint_descriptor = &endpoint_descriptors[0];

            available_endpoints.push(endpoint_descriptor.address());

            debug!("Found interface: {:?}", interface.number());

            let is_kernel_active = handle
                .kernel_driver_active(interface.number())
                .unwrap_or_else(|_| {
                    error!(
                        "Error checking if kernel driver is active interface: {}",
                        interface_number
                    );
                    panic!("Exiting...");
                });

            if is_kernel_active {
                handle
                    .detach_kernel_driver(interface.number())
                    .unwrap_or_else(|_| {
                        error!("Error detaching kernel driver: {}", interface_number);
                        panic!("Exiting...");
                    });
            }

            handle
                .claim_interface(interface.number())
                .unwrap_or_else(|_| {
                    error!("Error claiming interface: {}", interface_number);
                    panic!("Exiting...");
                });

            debug!("Claimed interface {}", interface_number);
        }

        let endpoint = available_endpoints[0];

        TabletDevice {
            device: handle,
            endpoint,
            device_info: device_info.clone(),
        }
    }

    pub(crate) fn read(&self, buffer: &mut [u8]) -> Option<()> {
        match self.device.read_bulk(self.endpoint, buffer, READ_TIMEOUT) {
            Ok(_) => Some(()),
            Err(err) => {
                debug!("Failed to read from device: {}.", err);
                None
            }
        }
    }

    // pub(crate) async fn async_read(self, buffer: &'static mut Vec<u8>) -> Option<()> {
    //     let data = tokio::task::spawn_blocking(move|| {
    //         self.device.read_bulk(self.endpoint, buffer, READ_TIMEOUT)
    //     })
    //     .await;
    //     match data {
    //         Ok(_) => Some(()),
    //         Err(err) => {
    //             error!("Failed to read from device: {}.", err);
    //             None
    //         }
    //     }
    // }
}
