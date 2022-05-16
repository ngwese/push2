//
// Copyright (c) 2022 Greg Wuller
//
// SPDX-License-Identifier: MIT
//

use crate::bitmap::{Bitmap, Pixel};

const DATA_SOURCE_WIDTH: usize = 1024;
const DATA_SOURCE_HEIGHT: usize = 160;

const DISPLAY_XOR_MASKS: [u16; 2] = [0xf3e7, 0xffe7];

pub struct Buffer {
    data: [Pixel; DATA_SOURCE_WIDTH * DATA_SOURCE_HEIGHT],
}

impl Buffer {
    pub const fn pixel_count() -> usize {
        DATA_SOURCE_WIDTH * DATA_SOURCE_HEIGHT
    }

    pub fn new() -> Self {
        Buffer {
            data: [0; DATA_SOURCE_WIDTH * DATA_SOURCE_HEIGHT],
        }
    }

    pub fn update(&mut self, bitmap: &Bitmap) {
        assert!(bitmap.height() == DATA_SOURCE_HEIGHT);
        let width = bitmap.width();
        for line in 0..DATA_SOURCE_HEIGHT {
            let src_start = line * width;
            let src = &bitmap.data()[src_start..src_start + width];
            let dst_start = line * DATA_SOURCE_WIDTH;
            let dst = &mut self.data[dst_start..dst_start + width];
            // dst.copy_from_slice(src);
            for i in 0..src.len() {
                dst[i] = src[i] ^ DISPLAY_XOR_MASKS[i % 2];
            }
        }
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.data
    }
}
