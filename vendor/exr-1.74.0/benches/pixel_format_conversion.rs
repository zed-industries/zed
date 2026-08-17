#[macro_use]
extern crate bencher;

extern crate exr;
use exr::prelude::*;

use bencher::Bencher;
use std::fs;
use std::io::Cursor;
use exr::image::pixel_vec::PixelVec;
use exr::block::samples::FromNativeSample;

const F32_ZIPS_PATH: &'static str = "tests/images/valid/custom/crowskull/crow_zips.exr";
const F32_UNCOMPRESSED_PATH: &'static str = "tests/images/valid/custom/crowskull/crow_uncompressed.exr";
const F16_UNCOMPRESSED_PATH: &'static str = "tests/images/valid/custom/crowskull/crow_uncompressed_half.exr";
const F16_ZIP_PATH: &'static str = "tests/images/valid/custom/crowskull/crow_zip_half.exr";

/// Read an image from an in-memory buffer into its native f32 format
fn read_f32_as_f32_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F32_UNCOMPRESSED_PATH, false);
}

/// Read image and convert the samples to u32 (from native f32)
fn read_f32_as_u32_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<u32>(bench, F32_UNCOMPRESSED_PATH, false);
}

/// f16 is not natively supported by CPUs, which introduces unique performance pitfalls
fn read_f32_as_f16_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F32_UNCOMPRESSED_PATH, false);
}

fn read_f16_as_f16_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F16_UNCOMPRESSED_PATH, false);
}

fn read_f16_as_f32_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F16_UNCOMPRESSED_PATH, false);
}

fn read_f16_as_u32_uncompressed_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<u32>(bench, F16_UNCOMPRESSED_PATH, false);
}


fn read_f32_as_f16_zips_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F32_ZIPS_PATH, false);
}

fn read_f16_as_f32_zip_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F16_ZIP_PATH, false);
}

fn read_f32_as_f16_zips_nthreads(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F32_ZIPS_PATH, true);
}

fn read_f16_as_f32_zip_nthreads(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F16_ZIP_PATH, true);
}

fn read_f32_as_f32_zips_nthreads(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F32_ZIPS_PATH, true);
}

fn read_f16_as_f16_zip_nthreads(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F16_ZIP_PATH, true);
}

fn read_f32_as_f32_zips_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f32>(bench, F32_ZIPS_PATH, false);
}

fn read_f16_as_f16_zip_1thread(bench: &mut Bencher) {
    bench_read_image_rgba_as::<f16>(bench, F16_ZIP_PATH, false);
}

fn bench_read_image_rgba_as<T>(bench: &mut Bencher, path: &str, parallel: bool) {
    let mut file = fs::read(path).unwrap();
    bencher::black_box(&mut file);

    bench.iter(||{
        let image = read_file_from_memory_as::<f16>(file.as_slice(), parallel);
        bencher::black_box(image);
    })
}

fn read_file_from_memory_as<T>(file: &[u8], parallel: bool) -> RgbaImage<PixelVec<(T, T, T, T)>>
    where T: FromNativeSample
{
    let read = exr::prelude::read()
        .no_deep_data().largest_resolution_level()
        .rgba_channels(PixelVec::<(T, T, T, T)>::constructor, PixelVec::set_pixel)
        .first_valid_layer().all_attributes();

    let read = if parallel { read } else { read.non_parallel() };
    read.from_buffered(Cursor::new(file)).unwrap()
}

benchmark_group!(pixel_format_conversion,
    read_f32_as_f32_uncompressed_1thread,
    read_f32_as_u32_uncompressed_1thread,
    read_f32_as_f16_uncompressed_1thread,
    read_f32_as_f16_zips_1thread,
    read_f32_as_f16_zips_nthreads,
    read_f32_as_f32_zips_nthreads,
    read_f32_as_f32_zips_1thread,

    read_f16_as_f16_uncompressed_1thread,
    read_f16_as_u32_uncompressed_1thread,
    read_f16_as_f32_uncompressed_1thread,
    read_f16_as_f32_zip_1thread,
    read_f16_as_f32_zip_nthreads,
    read_f16_as_f16_zip_nthreads,
    read_f16_as_f16_zip_1thread,
);

benchmark_main!(pixel_format_conversion);