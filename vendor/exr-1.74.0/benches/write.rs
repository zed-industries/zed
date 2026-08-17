#[macro_use]
extern crate bencher;

extern crate exr;
use exr::prelude::*;

use bencher::Bencher;
use std::io::Cursor;

fn write_parallel_any_channels_to_buffered(bench: &mut Bencher) {
    let path = "tests/images/valid/custom/crowskull/crow_rle.exr";
    let image = read_all_flat_layers_from_file(path).unwrap();

    bench.iter(||{
        let mut result = Vec::new();
        image.write().to_buffered(Cursor::new(&mut result)).unwrap();
        bencher::black_box(result);
    })
}

fn write_parallel_zip1_to_buffered(bench: &mut Bencher) {
    let path = "tests/images/valid/custom/crowskull/crow_rle.exr";

    let mut image = read_first_flat_layer_from_file(path).unwrap();
    image.layer_data.encoding.compression = Compression::ZIP1;

    bench.iter(||{
        let mut result = Vec::new();
        image.write().to_buffered(Cursor::new(&mut result)).unwrap();
        bencher::black_box(result);
    })
}

fn write_nonparallel_zip1_to_buffered(bench: &mut Bencher) {
    let path = "tests/images/valid/custom/crowskull/crow_rle.exr";

    let mut image = read_first_flat_layer_from_file(path).unwrap();
    image.layer_data.encoding.compression = Compression::ZIP1;

    bench.iter(||{
        let mut result = Vec::new();
        image.write().non_parallel().to_buffered(Cursor::new(&mut result)).unwrap();
        bencher::black_box(result);
    })
}

fn write_parallel_zip16_to_buffered(bench: &mut Bencher) {
    let path = "tests/images/valid/custom/crowskull/crow_rle.exr";

    let mut image = read_first_flat_layer_from_file(path).unwrap();
    image.layer_data.encoding.compression = Compression::ZIP16;

    bench.iter(||{
        let mut result = Vec::new();
        image.write().to_buffered(Cursor::new(&mut result)).unwrap();
        bencher::black_box(result);
    })
}

fn write_uncompressed_to_buffered(bench: &mut Bencher) {
    let path = "tests/images/valid/custom/crowskull/crow_uncompressed.exr";
    let image = read_all_flat_layers_from_file(path).unwrap();
    assert!(image.layer_data.iter().all(|layer| layer.encoding.compression == Compression::Uncompressed));

    bench.iter(||{
        let mut result = Vec::new();
        image.write().to_buffered(Cursor::new(&mut result)).unwrap();
        bencher::black_box(result);
    })
}

benchmark_group!(write,
    write_parallel_any_channels_to_buffered,
    write_nonparallel_zip1_to_buffered,
    write_parallel_zip1_to_buffered,
    write_parallel_zip16_to_buffered,
    write_uncompressed_to_buffered
);

benchmark_main!(write);