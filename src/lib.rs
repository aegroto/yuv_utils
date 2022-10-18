#![feature(test)]
extern crate test;

pub mod bgra2yuv;
pub mod yuv2rgba;

#[derive(Clone, Copy)]
pub enum PixelOrder {
    RGBA,
    BGRA
}

#[derive(Clone, Copy)]
pub enum Indicization {
    Vectorized,
    Squared
}
