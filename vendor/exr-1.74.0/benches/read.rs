#[macro_use]
extern crate bencher;

extern crate exr;
use exr::prelude::*;

use bencher::Bencher;
use std::fs;
use std::io::Cursor;
use exr::image::pixel_vec::PixelVec;

/// Read uncompressed (always single core)
fn read_single_image_uncompressed_non_parallel_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_uncompressed.exr").unwrap();
    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data().largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers().all_attributes()
            .non_parallel()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read from in-memory in parallel
fn read_single_image_uncompressed_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_uncompressed.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data().largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers().all_attributes()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read with multi-core RLE decompression
fn read_single_image_rle_all_channels(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_rle.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data()
            .largest_resolution_level()
            .all_channels()
            .all_layers()
            .all_attributes()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read without multi-core RLE decompression
fn read_single_image_rle_non_parallel_all_channels(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_rle.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        // copied from `read_all_flat_layers_from_file` and added `.non_parallel()`
        let image = exr::prelude::read()
            .no_deep_data()
            .largest_resolution_level()
            .all_channels()
            .all_layers()
            .all_attributes()
            .non_parallel()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read with multi-core RLE decompression
fn read_single_image_rle_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_rle.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data()
            .largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers()
            .all_attributes()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read without multi-core RLE decompression
fn read_single_image_rle_non_parallel_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_rle.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        // copied from `read_all_flat_layers_from_file` and added `.non_parallel()`
        let image = exr::prelude::read()
            .no_deep_data()
            .largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers()
            .all_attributes()
            .non_parallel()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read with multi-core zip decompression
fn read_single_image_zips_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_zips.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data().largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers().all_attributes()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

/// Read without multi-core ZIP decompression
fn read_single_image_zips_non_parallel_rgba(bench: &mut Bencher) {
    let mut file = fs::read("tests/images/valid/custom/crowskull/crow_zips.exr").unwrap();

    bench.iter(||{
        bencher::black_box(&mut file);

        let image = exr::prelude::read()
            .no_deep_data().largest_resolution_level()
            .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
            .all_layers().all_attributes()
            .non_parallel()
            .from_buffered(Cursor::new(file.as_slice())).unwrap();

        bencher::black_box(image);
    })
}

benchmark_group!(read,
    read_single_image_uncompressed_rgba,
    read_single_image_uncompressed_non_parallel_rgba,
    read_single_image_rle_rgba,
    read_single_image_rle_non_parallel_rgba,
    read_single_image_rle_all_channels,
    read_single_image_rle_non_parallel_all_channels,
    read_single_image_zips_rgba,
    read_single_image_zips_non_parallel_rgba,
);

benchmark_main!(read);