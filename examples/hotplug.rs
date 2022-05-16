use log::{debug, error};
use rusb::UsbContext;

use push2::device::Monitor;

fn main() -> rusb::Result<()> {
    env_logger::init();

    if !rusb::has_hotplug() {
        error!("libusb hotplug monitoring is not available");
        return Ok(());
    }

    let context = rusb::Context::new()?;

    let mut registration = Some(
        rusb::HotplugBuilder::new()
            .enumerate(true)
            .register(&context, Box::new(Monitor::new()))?,
    );

    loop {
        context.handle_events(None)?;
        if let Some(reg) = registration.take() {
            debug!("registration: {:?}", reg);
            context.unregister_callback(reg);
            break;
        }
    }

    Ok(())
}
