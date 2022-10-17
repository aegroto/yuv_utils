use itertools::{izip, Itertools};
use rayon::prelude::*;

use crate::{benchmarks, RGBAPixel, bgr_to_yuv_f32};

pub struct ConversionContext {}

impl ConversionContext {
    pub fn new(_width: usize, _height: usize) -> Self {
        Self {}
    }

    pub fn bgra_to_yuv_separate(
        &mut self,
        bgra_pixels: &[u8],
        y_pixels: &mut [u8],
        u_pixels: &mut [u8],
        v_pixels: &mut [u8],
    ) {
        y_pixels.fill(0);
        u_pixels.fill(0);
        v_pixels.fill(0);

        let bgra_iter =
            bgra_pixels
                .iter()
                .tuples::<RGBAPixel>()
                .tuples::<(RGBAPixel, RGBAPixel, RGBAPixel, RGBAPixel)>();

        let y_iter = y_pixels
            .iter_mut()
            .tuples::<(&mut u8, &mut u8, &mut u8, &mut u8)>();
        let u_iter = u_pixels.iter_mut();
        let v_iter = v_pixels.iter_mut();

        let uv_iter = izip!(y_iter, u_iter, v_iter);
        let iter = bgra_iter.zip(uv_iter);

        iter.par_bridge().for_each(|(bgra_block, yuv_block)| {
            let (bgra0, bgra1, bgra2, bgra3) = bgra_block;
            let ((y0_ptr, y1_ptr, y2_ptr, y3_ptr), u_ptr, v_ptr) = yuv_block;

            let (y0, u0, v0) = bgr_to_yuv_f32(*bgra0.0, *bgra0.1, *bgra0.2);
            let (y1, u1, v1) = bgr_to_yuv_f32(*bgra1.0, *bgra1.1, *bgra1.2);
            let (y2, u2, v2) = bgr_to_yuv_f32(*bgra2.0, *bgra2.1, *bgra2.2);
            let (y3, u3, v3) = bgr_to_yuv_f32(*bgra3.0, *bgra3.1, *bgra3.2);

            *y0_ptr = y0 as u8;
            *y1_ptr = y1 as u8;
            *y2_ptr = y2 as u8;
            *y3_ptr = y3 as u8;

            *u_ptr = (u0 * 0.25 + u1 * 0.25 + u2 * 0.25 + u3 * 0.25) as u8;
            *v_ptr = (v0 * 0.25 + v1 * 0.25 + v2 * 0.25 + v3 * 0.25) as u8;
        });
    }
}

benchmarks!();
