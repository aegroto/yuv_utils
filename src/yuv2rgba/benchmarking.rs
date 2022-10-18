#[macro_export]
macro_rules! bench_cases {
    () => {
        #[bench]
        fn bench_1280x720(bencher: &mut Bencher) {
            bench(bencher, 1280, 720);
        }

        #[bench]
        fn bench_1280x720_x2(bencher: &mut Bencher) {
            bench(bencher, 1280 * 2, 720 * 2);
        }
    }
}