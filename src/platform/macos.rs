use super::VirtualInputDevice;

pub(crate) struct VirtualDevice {}

impl VirtualInputDevice for VirtualDevice {
    fn new(device_info: &crate::device_info::DeviceInfo) -> Self {
        todo!()
    }
    fn submit_cursor(&mut self, update_info: crate::interfaces::PenStatus) {
        todo!()
    }
    fn submit_action(&mut self, action: super::InputAction, pressed: bool) {
        todo!()
    }
}
