#[inline]
pub fn yuv_to_bgr(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y: f64 = y as f64;
    let u: f64 = ((u as i16) - 128) as f64;
    let v: f64 = ((v as i16) - 128) as f64;

    let r = (y + v * 1.40200) as u8;
    let g = (y + u * -0.34414 + v * -0.71414) as u8;
    let b = (y + u * 1.77200) as u8;

    (b, g, r)
}

#[inline]
pub fn write_rgba_pixel(rgba_pixels: &mut [u8], i: usize, r: u8, g: u8, b: u8) {
    rgba_pixels[i * 4] = r;
    rgba_pixels[i * 4 + 1] = g;
    rgba_pixels[i * 4 + 2] = b;
    rgba_pixels[i * 4 + 3] = 255;
}

#[inline]
pub fn write_bgra_pixel(rgba_pixels: &mut [u8], i: usize, r: u8, g: u8, b: u8) {
    rgba_pixels[i * 4] = b;
    rgba_pixels[i * 4 + 1] = g;
    rgba_pixels[i * 4 + 2] = r;
    rgba_pixels[i * 4 + 3] = 255;
}

#[inline]
pub fn load_yuv_squared(
    y_pixels: &[u8],
    u_pixels: &[u8],
    v_pixels: &[u8],
    i: usize,
    uv_i: usize,
) -> (u8, u8, u8) {
    let y = y_pixels[i];
    let u = u_pixels[uv_i];
    let v = v_pixels[uv_i];
    (y, u, v)
}
