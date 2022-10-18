#![feature(test)]
extern crate test;

pub mod bgra2yuv;
pub mod yuv2rgba;

#[derive(Clone, Copy)]
pub struct PixelOffset {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
}

impl PixelOffset {
    pub const RGBA: Self = Self { r: 0, g: 1, b: 2, a: 3 };
    pub const BGRA: Self = Self { r: 2, g: 1, b: 0, a: 3 };
}

#[derive(Clone, Copy)]
pub enum Indicization {
    Vectorized,
    Squared
}
