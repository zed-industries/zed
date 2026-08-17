#[macro_use]
extern crate bencher;

extern crate exr;
use exr::prelude::*;

use bencher::Bencher;
use std::fs;
use std::io::Cursor;
use exr::image::pixel_vec::PixelVec;

const PROFILING_REPETITIONS: i32 = 1; // make this 100 for profiling longer periods

/// This is a suuuper long benchmark, to allow you to hook up a profiler while running it
/// but this means we don't want it in our normal benchmarks
fn read_single_image_from_buffer_rgba_f32_as_f16(bench: &mut Bencher) {

    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_uncompressed.exr").unwrap();
    bencher::black_box(&mut file);

    bench.iter(||{
        for _ in 0 .. PROFILING_REPETITIONS {
            let image = exr::prelude::read()
                .no_deep_data().largest_resolution_level()
                .rgba_channels(PixelVec::<(f16,f16,f16,f16)>::constructor, PixelVec::set_pixel)
                .first_valid_layer().all_attributes()
                .non_parallel()
                .from_buffered(Cursor::new(file.as_slice())).unwrap();

            bencher::black_box(image);
        }
    })
}

benchmark_group!(profiling,
    read_single_image_from_buffer_rgba_f32_as_f16
);

benchmark_main!(profiling);