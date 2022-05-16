//
// Copyright (c) 2022 Greg Wuller
//
// SPDX-License-Identifier: MIT
//

const DISPLAY_WIDTH: usize = 960;
const DISPLAY_HEIGHT: usize = 160;
const DISPLAY_PIXEL_COUNT: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub type Pixel = u16;

pub struct Bitmap {
    data: [Pixel; DISPLAY_PIXEL_COUNT],
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            data: [0; DISPLAY_PIXEL_COUNT],
        }
    }

    #[inline(always)]
    pub fn data(&self) -> &[u16] {
        &self.data
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        DISPLAY_WIDTH
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        DISPLAY_HEIGHT
    }

    pub fn fill_constant(&mut self, pixel: Pixel) {
        // TODO: figure out a more idiomatic way of doing this
        for i in 0..DISPLAY_PIXEL_COUNT {
            self.data[i] = pixel
        }
    }
}

#[inline(always)]
pub fn pack_rgb(r: u8, g: u8, b: u8) -> Pixel {
    let mut pixel = ((b as u16) & 0xf8) >> 3;
    pixel <<= 6;
    pixel += ((g as u16) & 0xfc) >> 2;
    pixel <<= 5;
    pixel += ((r as u16) & 0xf8) >> 3;
    pixel
}
