use crate::{common::*, from_yuv::YUVToRGBAConversionContext, yuv2rgba_benchmarks};

pub struct ConversionContext {
    width: u32,
    height: u32,
}

impl YUVToRGBAConversionContext for ConversionContext {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        let pixels_count = (self.width * self.height) as usize;
        for i in 0..pixels_count {
            let (y, u, v) = (y_pixels[i], u_pixels[i / 4], v_pixels[i / 4]);
            let (b, g, r) = yuv_to_bgr(y, u, v);
            write_bgra_pixel(rgba_pixels, i, r, g, b);
        }
    }
}

yuv2rgba_benchmarks!();
