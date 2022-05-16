//
// Copyright (c) 2022 Greg Wuller
//
// SPDX-License-Identifier: MIT
//

use std::time::Duration;

use anyhow::{anyhow, Result};
use log::info;
use rusb::UsbContext;

use crate::bitmap::Pixel;
use crate::display::Buffer;

const ABLETON_VENDOR_ID: u16 = 0x2982;
const PUSH2_PRODUCT_ID: u16 = 0x1967;
const PUSH2_BULK_EP_OUT: u8 = 0x01;

const PUSH2_FRAME_HEADER: [u8; 16] = [
    0xff, 0xcc, 0xaa, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub struct Monitor;

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: rusb::UsbContext> rusb::Hotplug<T> for Monitor {
    fn device_arrived(&mut self, device: rusb::Device<T>) {
        info!("connect: {:?}", device);
    }

    fn device_left(&mut self, device: rusb::Device<T>) {
        info!("disconnect: {:?}", device);
    }
}

#[derive(Debug)]
pub struct Device {
    handle: rusb::DeviceHandle<rusb::Context>,
    write_timeout: Duration,
}

impl Device {
    pub fn open(context: &rusb::Context) -> Result<Self> {
        let handle = context.open_device_with_vid_pid(ABLETON_VENDOR_ID, PUSH2_PRODUCT_ID);
        if let Some(mut handle) = handle {
            handle.claim_interface(0)?;
            Ok(Self {
                handle,
                write_timeout: Duration::from_millis(1000),
            })
        } else {
            Err(anyhow!("push2 device not found"))
        }
    }

    pub fn write(&self, buffer: &Buffer) -> Result<()> {
        // let start = Instant::now();
        let header_size =
            self.handle
                .write_bulk(PUSH2_BULK_EP_OUT, &PUSH2_FRAME_HEADER, self.write_timeout)?;
        assert_eq!(header_size, PUSH2_FRAME_HEADER.len());

        let raw = to_u8_slice(buffer.pixels());
        let data_size = self
            .handle
            .write_bulk(PUSH2_BULK_EP_OUT, raw, self.write_timeout)?;

        const RAW_SIZE: usize = Buffer::pixel_count() * std::mem::size_of::<Pixel>();
        assert_eq!(data_size, RAW_SIZE);

        // debug!("device write took: {:?}", start.elapsed());
        Ok(())
    }
}

#[inline(always)]
fn to_u8_slice(slice: &[u16]) -> &[u8] {
    let byte_len = 2 * slice.len();
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), byte_len) }
}
