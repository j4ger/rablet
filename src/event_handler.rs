use crate::{
    interfaces::PartialUpdate,
    platform::{update_button, update_cursor},
};
use log::{debug, info};
use tokio::sync::mpsc::UnboundedReceiver;

pub(crate) async fn event_handler(mut rx: UnboundedReceiver<PartialUpdate>) {
    // terminate signal
    loop {
        match rx.recv().await {
            Some(content) => match content {
                PartialUpdate::Pen(pen) => {
                    debug!("Got pen event: {:?}.", pen);
                    update_cursor(pen.position, pen.tilt, pen.pressure);
                }
                PartialUpdate::Button(button, button_state) => {
                    debug!("Got button event: {:?} - {:?}.", button, button_state);
                    update_button(button, button_state);
                }
                PartialUpdate::Wheel(direction) => {
                    debug!("Got wheel event: {:?}.", direction);
                }
            },
            None => {
                info!("Channel closed, terminating.");
                break;
            }
        }
    }
}
