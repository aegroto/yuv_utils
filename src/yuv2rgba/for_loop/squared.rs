use crate::{yuv2rgba::yuv_to_bgr, PixelOffset};

pub fn convert(
    width: usize,
    height: usize,
    pixel_offset: PixelOffset,
    y_pixels: &[u8],
    u_pixels: &[u8],
    v_pixels: &[u8],
    rgba_pixels: &mut [u8],
) {
    for row in 0..height {
        for column in 0..width {
            let i = row * width + column;

            let y = y_pixels[i];
            let u = u_pixels[(row / 2) * width / 2 + (column / 2)];
            let v = v_pixels[(row / 2) * width / 2 + (column / 2)];

            let (b, g, r) = yuv_to_bgr(y, u, v);

            rgba_pixels[i * 4 + pixel_offset.r] = r;
            rgba_pixels[i * 4 + pixel_offset.g] = g;
            rgba_pixels[i * 4 + pixel_offset.b] = b;
            rgba_pixels[i * 4 + pixel_offset.a] = 255;
        }
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::{PixelOffset, bench_cases};

    use super::convert;

    fn bench(bencher: &mut Bencher, width: usize, height: usize) {
        let y_pixels = black_box(vec![0u8; width * height]);
        let u_pixels = black_box(vec![0u8; (width * height) / 4]);
        let v_pixels = black_box(vec![0u8; (width * height) / 4]);
        let mut bgra_pixels = black_box(vec![0u8; width * height * 4]);

        bencher.iter(|| {
            convert(
                width,
                height,
                PixelOffset::RGBA,
                &y_pixels,
                &u_pixels,
                &v_pixels,
                &mut bgra_pixels,
            );
        })
    }

    bench_cases!();
}


