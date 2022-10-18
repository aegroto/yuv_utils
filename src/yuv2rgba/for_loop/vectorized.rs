use crate::yuv2rgba::yuv_to_bgr;

pub fn convert(
    y_pixels: &[u8],
    u_pixels: &[u8],
    v_pixels: &[u8],
    rgba_pixels: &mut [u8],
) {
    for i in 0..y_pixels.len() {
        let (y, u, v) = (y_pixels[i], u_pixels[i / 4], v_pixels[i / 4]);

        let (b, g, r) = yuv_to_bgr(y, u, v);

        rgba_pixels[i * 4] = b;
        rgba_pixels[i * 4 + 1] = g;
        rgba_pixels[i * 4 + 2] = r;
        rgba_pixels[i * 4 + 3] = 255;
    }
}