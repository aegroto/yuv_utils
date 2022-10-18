use crate::{common::*, from_yuv::YUVToRGBAConversionContext, yuv2rgba_benchmarks};

pub struct ConversionContext {
    indices: Vec<(usize, usize)>,
}

impl ConversionContext {
    fn generate_indices(width: usize, height: usize) -> Vec<(usize, usize)> {
        let mut indices = Vec::new();
        for row in 0..height {
            for column in 0..width {
                let i = row * width + column;
                let uv_i = (row / 2) * width / 2 + (column / 2);
                indices.push((i, uv_i));
            }
        }
        indices
    }
}

impl YUVToRGBAConversionContext for ConversionContext {
    fn new(width: u32, height: u32) -> Self {
        Self {
            indices: Self::generate_indices(width as usize, height as usize),
        }
    }

    fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        for (i, uv_i) in &self.indices {
            let (y, u, v) = load_yuv_squared(y_pixels, u_pixels, v_pixels, *i, *uv_i);
            let (b, g, r) = yuv_to_bgr(y, u, v);
            write_rgba_pixel(rgba_pixels, *i, r, g, b);
        }
    }
}

yuv2rgba_benchmarks!();
