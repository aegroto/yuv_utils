use crate::yuv2rgba::{load_yuv_squared, write_rgba_pixel, yuv_to_bgr, write_bgra_pixel};

pub fn convert_to_rgba(
    width: usize,
    height: usize,
    y_pixels: &[u8],
    u_pixels: &[u8],
    v_pixels: &[u8],
    rgba_pixels: &mut [u8],
) {
    for row in 0..height {
        for column in 0..width {
            let i = row * width + column;
            let uv_i = (row / 2) * width / 2 + (column / 2);
            let (y, u, v) = load_yuv_squared(y_pixels, u_pixels, v_pixels, i, uv_i);
            let (b, g, r) = yuv_to_bgr(y, u, v);
            write_rgba_pixel(rgba_pixels, i, r, g, b);
        }
    }
}

pub fn convert_to_bgra(
    width: usize,
    height: usize,
    y_pixels: &[u8],
    u_pixels: &[u8],
    v_pixels: &[u8],
    rgba_pixels: &mut [u8],
) {
    for row in 0..height {
        for column in 0..width {
            let i = row * width + column;
            let uv_i = (row / 2) * width / 2 + (column / 2);
            let (y, u, v) = load_yuv_squared(y_pixels, u_pixels, v_pixels, i, uv_i);
            let (b, g, r) = yuv_to_bgr(y, u, v);
            write_bgra_pixel(rgba_pixels, i, r, g, b);
        }
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::bench_cases;

    use super::convert_to_rgba;

    fn bench(bencher: &mut Bencher, width: usize, height: usize) {
        let y_pixels = black_box(vec![0u8; width * height]);
        let u_pixels = black_box(vec![0u8; (width * height) / 4]);
        let v_pixels = black_box(vec![0u8; (width * height) / 4]);
        let mut bgra_pixels = black_box(vec![0u8; width * height * 4]);

        bencher.iter(|| {
            convert_to_rgba(
                width,
                height,
                &y_pixels,
                &u_pixels,
                &v_pixels,
                &mut bgra_pixels,
            );
        })
    }

    bench_cases!();
}
