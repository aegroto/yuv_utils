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

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::bench_cases;

    use super::convert;

    fn bench(bencher: &mut Bencher, width: usize, height: usize) {
        let y_pixels = black_box(vec![0u8; width * height]);
        let u_pixels = black_box(vec![0u8; (width * height) / 4]);
        let v_pixels = black_box(vec![0u8; (width * height) / 4]);
        let mut bgra_pixels = black_box(vec![0u8; width * height * 4]);

        bencher.iter(|| {
            convert(
                &y_pixels,
                &u_pixels,
                &v_pixels,
                &mut bgra_pixels,
            );
        })
    }

    bench_cases!();
}
