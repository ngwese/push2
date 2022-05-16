use std::time::Instant;

use push2::bitmap::{pack_rgb, Bitmap};
use push2::device::Device;
use push2::display::Buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let context = rusb::Context::new()?;

    let device = Device::open(&context)?;
    print!("device: {:?}", device);

    let mut bitmap = Bitmap::new();
    let mut buffer = Buffer::new();

    for _ in 0..2 {
        // red ramp
        for r in (0..u8::MAX).step_by(4) {
            bitmap.fill_constant(pack_rgb(r, 0, 0));
            buffer.update(&bitmap);
            device.write(&buffer)?;
        }

        // blue ramp
        for g in (0..u8::MAX).step_by(4) {
            bitmap.fill_constant(pack_rgb(0, g, 0));
            buffer.update(&bitmap);
            device.write(&buffer)?;
        }

        // green ramp
        for b in (0..u8::MAX).step_by(4) {
            bitmap.fill_constant(pack_rgb(0, 0, b));
            buffer.update(&bitmap);
            device.write(&buffer)?;
        }

        // gray ramp
        for v in (0..u8::MAX).step_by(8) {
            bitmap.fill_constant(pack_rgb(v, v, v));
            buffer.update(&bitmap);
            device.write(&buffer)?;
        }

        // something ramp
        for v in (0..u8::MAX).step_by(4) {
            let g: u8 = (((v as u16) * 6) % u8::MAX as u16) as u8;
            bitmap.fill_constant(pack_rgb(v, 0, g / 3));
            buffer.update(&bitmap);
            device.write(&buffer)?;
        }
    }

    Ok(())
}
