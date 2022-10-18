#[macro_export]
macro_rules! yuv2rgba_benchmarks {
    () => {
        #[cfg(test)]
        mod tests {
            use super::ConversionContext;
            use crate::from_yuv::YUVToRGBAConversionContext;
            use test::{black_box, Bencher};

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

            #[bench]
            fn bench_1280x720(bencher: &mut Bencher) {
                bench(bencher, 1280, 720);
            }

            #[bench]
            fn bench_1280x720_x2(bencher: &mut Bencher) {
                bench(bencher, 1280 * 2, 720 * 2);
            }
        }
    };
}
