// #![feature(test)]
// extern crate rand;
// extern crate test;
//
// use test::Bencher;
// use test::black_box;
// use rand::Rng;
// use rand::StdRng;
// use rand::OsRng;
//
// // OsRng is supposed to be the default for crypto uses.
// #[bench]
// fn bench_osrng_u64(b : &mut Bencher) {
//     if let Ok(mut gen) = OsRng::new() {
//         b.bytes = 8;
//         b.iter(|| {
//             black_box(gen.next_u64());
//         });
//     }
// }
//
// // StdRng is the default for everything else.
// #[bench]
// fn bench_stdrng_u64(b : &mut Bencher) {
//     if let Ok(mut gen) = StdRng::new() {
//         b.bytes = 8;
//         b.iter(|| {
//             gen.next_u64();
//         });
//     }
// }
