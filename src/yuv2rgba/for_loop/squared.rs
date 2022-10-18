use crate::{common::*, yuv2rgba::YUVToRGBAConversionContext};

pub struct ConversionContext {
    width: u32,
    height: u32,
}

impl YUVToRGBAConversionContext for ConversionContext {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn convert(
        &mut self,
        y_pixels: &[u8],
        u_pixels: &[u8],
        v_pixels: &[u8],
        rgba_pixels: &mut [u8],
    ) {
        let width = self.width as usize;
        let height = self.height as usize;
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
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use crate::{bench_cases, yuv2rgba::YUVToRGBAConversionContext};

    use super::ConversionContext;

    fn bench(bencher: &mut Bencher, width: usize, height: usize) {
        let mut context = black_box(ConversionContext::new(width as u32, height as u32));

        let y_pixels = black_box(vec![0u8; width * height]);
        let u_pixels = black_box(vec![0u8; (width * height) / 4]);
        let v_pixels = black_box(vec![0u8; (width * height) / 4]);
        let mut rgba_pixels = black_box(vec![0u8; width * height * 4]);

        bencher.iter(|| {
            context.convert(&y_pixels, &u_pixels, &v_pixels, &mut rgba_pixels);
        })
    }

    bench_cases!();
}
