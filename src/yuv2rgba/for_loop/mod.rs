mod squared;
mod vectorized;

use crate::{Indicization, PixelOffset};

pub struct ConversionContext {
    width: u32,
    height: u32,
    pixel_offset: PixelOffset,
    indicization: Indicization,
}

impl ConversionContext {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixel_offset: PixelOffset::RGBA,
            indicization: Indicization::Squared,
        }
    }

    pub fn set_pixel_offset(&mut self, value: PixelOffset) {
        self.pixel_offset = value;
    }

    pub fn set_indicization(&mut self, value: Indicization) {
        self.indicization = value;
    }

    pub fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        match self.indicization {
            Indicization::Vectorized => {
                vectorized::convert(y_pixels, u_pixels, v_pixels, rgba_pixels)
            }
            Indicization::Squared => squared::convert(
                self.width as usize,
                self.height as usize,
                self.pixel_offset,
                y_pixels,
                u_pixels,
                v_pixels,
                rgba_pixels,
            ),
        }
    }
}
