use rayon::prelude::*;

use crate::{common::*, from_yuv::YUVToRGBAConversionContext, yuv2rgba_benchmarks};

type IndexRef = (usize, usize, usize);

pub struct ConversionContext {
    indices: Vec<IndexRef>,
    chunk_size: usize,
}

impl ConversionContext {
    fn generate_indices(width: usize, height: usize, chunk_size: usize) -> Vec<IndexRef> {
        let mut indices = Vec::new();
        for row in 0..height {
            for column in 0..width {
                let i = row * width + column;
                let uv_i = (row / 2) * width / 2 + (column / 2);
                indices.push((i, uv_i, i % chunk_size));
            }
        }
        indices
    }

    fn run_on_indices(
        indices: &[IndexRef],
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        for (i, uv_i, rgba_i) in indices {
            let (y, u, v) = load_yuv_squared(y_pixels, u_pixels, v_pixels, *i, *uv_i);
            let (b, g, r) = yuv_to_bgr(y, u, v);
            write_bgra_pixel(rgba_pixels, *rgba_i, r, g, b);
        }
    }
}

impl YUVToRGBAConversionContext for ConversionContext {
    fn new(width: u32, height: u32) -> Self {
        let chunks_count = usize::from(std::thread::available_parallelism().unwrap()) * 4;
        let chunk_size = (width * height) as usize / chunks_count;

        Self {
            chunk_size,
            indices: Self::generate_indices(width as usize, height as usize, chunk_size),
        }
    }

    fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        let indices_chunks = self.indices.chunks(self.chunk_size);
        let pixel_chunks = rgba_pixels.chunks_mut(self.chunk_size * 4);

        indices_chunks
            .zip(pixel_chunks)
            .par_bridge()
            .for_each(|(indices_chunk, pixel_chunk)| {
                Self::run_on_indices(indices_chunk, y_pixels, u_pixels, v_pixels, pixel_chunk);
            });
    }
}

yuv2rgba_benchmarks!();
