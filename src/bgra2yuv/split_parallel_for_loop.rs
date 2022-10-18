use itertools::izip;
use rayon::prelude::*;

use crate::{benchmarks};
use super::bgr_to_yuv_f32;

pub struct ConversionContext { 
    chunks_count: usize
}

impl ConversionContext {
    pub fn new(_width: u32, _height: u32) -> Self {
        Self { 
            chunks_count: usize::from(std::thread::available_parallelism().unwrap())
        }
    }

    pub fn chunks_count(&mut self, value: usize) {
        self.chunks_count = value;
    }

    pub fn bgra_to_yuv_separate(
        &mut self,
        bgra_pixels: &[u8],
        y_pixels: &mut [u8],
        u_pixels: &mut [u8],
        v_pixels: &mut [u8],
    ) {
        u_pixels.fill(0);
        v_pixels.fill(0);

        let pixels_count = y_pixels.len();

        let chunks_count = self.chunks_count;

        let bgra_chunks = bgra_pixels.chunks((pixels_count * 4) / chunks_count);
        let y_chunks = y_pixels.chunks_mut(pixels_count / chunks_count);
        let u_chunks = u_pixels.chunks_mut(pixels_count / 4 / chunks_count);
        let v_chunks = v_pixels.chunks_mut(pixels_count / 4 / chunks_count);

        let chunks = izip!(bgra_chunks, y_chunks, u_chunks, v_chunks);

        chunks.par_bridge().for_each(|(bgra_chunk, y_chunk, u_chunk, v_chunk)| {
            self.run_on_slices(bgra_chunk, y_chunk, u_chunk, v_chunk);
        });
    }

    fn run_on_slices(
        &self,
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
