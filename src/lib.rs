#![feature(test)]
extern crate test;

pub(crate) mod common;
pub(crate) mod benchmarking;

pub mod bgra2yuv;
pub mod from_yuv;

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
