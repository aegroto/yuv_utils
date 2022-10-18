mod squared;
mod vectorized;

use crate::{Indicization, PixelOrder};

pub struct ConversionContext {
    width: u32,
    height: u32,
    pixel_order: PixelOrder,
    indicization: Indicization,
}

impl ConversionContext {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixel_order: PixelOrder::RGBA,
            indicization: Indicization::Squared,
        }
    }

    pub fn set_pixel_order(&mut self, value: PixelOrder) {
        self.pixel_order = value;
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
            Indicization::Squared => match self.pixel_order {
                PixelOrder::RGBA => squared::convert_to_rgba(
                    self.width as usize,
                    self.height as usize,
                    y_pixels,
                    u_pixels,
                    v_pixels,
                    rgba_pixels,
                ),
                PixelOrder::BGRA => squared::convert_to_bgra(
                    self.width as usize,
                    self.height as usize,
                    y_pixels,
                    u_pixels,
                    v_pixels,
                    rgba_pixels,
                ),
            },
        }
    }
}
