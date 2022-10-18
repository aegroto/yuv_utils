pub mod for_loop;
mod benchmarking;

pub fn yuv_to_bgr(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y: f64 = y as f64;
    let u: f64 = ((u as i16) - 128) as f64;
    let v: f64 = ((v as i16) - 128) as f64;

    let r = (y + v * 1.40200) as u8;
    let g = (y + u * -0.34414 + v * -0.71414) as u8;
    let b = (y + u * 1.77200) as u8;

    (b, g, r)
}
