use crate::{benchmarks, bgr_to_yuv_f32};

pub struct ConversionContext {}

impl ConversionContext {
    pub fn new(_width: u32, _height: u32) -> Self {
        Self {}
    }

    pub fn bgra_to_yuv_separate(
        &mut self,
        bgra_pixels: &[u8],
        y_pixels: &mut [u8],
        u_pixels: &mut [u8],
        v_pixels: &mut [u8],
    ) {
        for i in 0..y_pixels.len() {
            let (b, g, r) = (
                bgra_pixels[i * 4],
                bgra_pixels[i * 4 + 1],
                bgra_pixels[i * 4 + 2],
            );
            let (y, u, v) = bgr_to_yuv_f32(b, g, r);

            y_pixels[i] = y as u8;
            u_pixels[i / 4] += u as u8 / 4;
            v_pixels[i / 4] += v as u8 / 4;
        }
    }
}

benchmarks!();
