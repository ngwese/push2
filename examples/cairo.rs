use cairo::{Context, Format, ImageSurface, Operator};

use push2::bitmap::{self, pack_rgb, Bitmap};
use push2::device::Device;
use push2::display::Buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let usb_context = rusb::Context::new()?;

    let device = Device::open(&usb_context)?;
    println!("device: {:?}", device);

    let mut bitmap = Bitmap::new();
    let mut buffer = Buffer::new();

    let screen = Screen::new()?;

    println!(
        "h {} w {} s {} f {:?}",
        screen.surface().height(),
        screen.surface().width(),
        screen.surface().stride(),
        screen.surface().format(),
    );

    // for i in (0..u8::MAX).step_by(4) {
    for i in 0..bitmap::DISPLAY_WIDTH {
        screen.clear();
        // screen.level(i);

        screen.context().set_source_rgb(1.0, 0.0, 0.0);
        screen.context().set_line_width(1.0);
        screen.context().rectangle(i as f64, 10.0, 400.0, 10.0);
        screen.context().stroke()?;
        // screen.context().paint()?;


        // bitmap.fill_constant(pack_rgb(0, 0, 255));
        screen.update(&mut bitmap);
        buffer.update(&bitmap);
        device.write(&buffer)?;
    }

    Ok(())
}

struct Screen {
    surface: ImageSurface,
    context: Context,
}

impl Screen {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let surface = ImageSurface::create(
            Format::Rgb24,
            bitmap::DISPLAY_WIDTH.try_into()?,
            bitmap::DISPLAY_HEIGHT.try_into()?,
        )?;
        let context = Context::new(&surface)?;
        Ok(Self { surface, context })
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn surface(&self) -> &ImageSurface {
        &self.surface
    }

    pub fn clear(&self) {
        self.level(0);
        self.context.set_operator(Operator::Clear);
        self.context.paint().unwrap();
        self.context.set_operator(Operator::Over);
    }

    pub fn level(&self, level: u8) {
        let v = (level as f64) / u8::MAX as f64;
        self.context.set_source_rgb(v, v, v);
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn _pack(dst: &mut[u16], src: &[u8], dst_i: usize, src_offset: usize) {
        dst[dst_i] = pack_rgb(src[src_offset + 1], src[src_offset + 2], src[src_offset + 3]); // big endian RGB24
    }

    #[cfg(target_endian = "little")]
    #[inline]
    fn _pack(dst: &mut[u16], src: &[u8], dst_i: usize, src_offset: usize) {
        dst[dst_i] = pack_rgb(src[src_offset + 2], src[src_offset + 1], src[src_offset]); // little endian RGB24
    }

    pub fn update(&self, bitmap: &mut Bitmap) {
        assert_eq!(self.surface.format(), Format::Rgb24);
        self.surface
            .with_data(|src| {
                let dst = bitmap.data_mut();
                for i in 0..dst.len() {
                    Self::_pack(dst, src, i, i * 4);
                }
            })
            .unwrap();
    }
}
