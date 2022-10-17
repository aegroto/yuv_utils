#[macro_export]
macro_rules! benchmarks {
    () => {
        #[cfg(test)]
        mod tests {
            use test::{black_box, Bencher};

            use super::ConversionContext;

            fn bench(bencher: &mut Bencher, width: usize, height: usize) {
                let mut context = black_box(ConversionContext::new(width as u32, height as u32));

                let bgra_pixels = black_box(vec![0u8; width * height * 4]);
                let mut y_pixels = black_box(vec![0u8; width * height]);
                let mut u_pixels = black_box(vec![0u8; (width * height) / 4]);
                let mut v_pixels = black_box(vec![0u8; (width * height) / 4]);

                bencher.iter(|| {
                    context.bgra_to_yuv_separate(
                        &bgra_pixels,
                        &mut y_pixels,
                        &mut u_pixels,
                        &mut v_pixels,
                    );
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
