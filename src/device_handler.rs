use rusb::{DeviceHandle, GlobalContext};
use tokio::sync::mpsc::UnboundedSender;

use crate::interfaces::{GlobalState, PartialUpdate};

pub(crate) async fn input_parser(
    device: DeviceHandle<GlobalContext>,
    global_state: GlobalState,
    sender: UnboundedSender<PartialUpdate>,
) {
    // todo: parse input with lua script


}
