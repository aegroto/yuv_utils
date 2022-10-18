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
        let width = self.width as usize;
        let height = self.height as usize;
        for row in 0..height {
            for column in 0..width {
                let i = row * width + column;
                let uv_i = (row / 2) * width / 2 + (column / 2);
                let (y, u, v) = load_yuv_squared(y_pixels, u_pixels, v_pixels, i, uv_i);
                let (b, g, r) = yuv_to_bgr(y, u, v);
                write_rgba_pixel(rgba_pixels, i, r, g, b);
            }
        }
    }
}

yuv2rgba_benchmarks!();
