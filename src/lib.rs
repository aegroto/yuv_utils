#![feature(test)]
extern crate test;

pub mod iter_over_bgra;
pub mod for_loop;
pub mod split_parallel_for_loop;
pub mod indices_less;

mod benchmarking;

pub type RGBAPixel<'a> = (&'a u8, &'a u8, &'a u8, &'a u8);

pub fn bgr_to_yuv_f32(b: u8, g: u8, r: u8) -> (f32, f32, f32) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = r * 0.29900 + g * 0.58700 + b * 0.11400;
    let u = (r * -0.16874 + g * -0.33126 + b * 0.50000) + 128.0;
    let v = (r * 0.50000 + g * -0.41869 + b * -0.08131) + 128.0;

    (y, u, v)
}
