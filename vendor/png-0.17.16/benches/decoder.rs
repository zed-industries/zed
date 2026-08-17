use std::fs;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{Decoder, Reader, Transformations};

#[path = "../src/test_utils.rs"]
mod test_utils;

fn load_all(c: &mut Criterion) {
    let mut g = c.benchmark_group("decode");
    for entry in fs::read_dir("tests/benches/").unwrap().flatten() {
        match entry.path().extension() {
            Some(st) if st == "png" => {}
            _ => continue,
        }

        let data = fs::read(entry.path()).unwrap();
        bench_file(&mut g, data, entry.file_name().into_string().unwrap());
    }
    g.finish();

    // Small IDATS
    let mut g = c.benchmark_group("generated-noncompressed-4k-idat");
    bench_noncompressed_png(&mut g, 8, 4096); // 256 B
    bench_noncompressed_png(&mut g, 128, 4096); // 64 KB
    bench_noncompressed_png(&mut g, 2048, 4096); // 16 MB
    bench_noncompressed_png(&mut g, 12288, 4096); // 576 MB
    g.finish();

    // Normal IDATS
    let mut g = c.benchmark_group("generated-noncompressed-64k-idat");
    bench_noncompressed_png(&mut g, 128, 65536); // 64 KB
    bench_noncompressed_png(&mut g, 2048, 65536); // 16 MB
    bench_noncompressed_png(&mut g, 12288, 65536); // 576 MB
    g.finish();

    // Large IDATS
    let mut g = c.benchmark_group("generated-noncompressed-2g-idat");
    bench_noncompressed_png(&mut g, 2048, 0x7fffffff); // 16 MB
    bench_noncompressed_png(&mut g, 12288, 0x7fffffff); // 576 MB
    g.finish();
}

criterion_group! {benches, load_all}
criterion_main!(benches);

fn bench_noncompressed_png(g: &mut BenchmarkGroup<WallTime>, size: u32, idat_bytes: usize) {
    let mut data = Vec::new();
    test_utils::write_noncompressed_png(&mut data, size, idat_bytes);
    bench_file(g, data, format!("{size}x{size}.png"));
}

fn bench_file(g: &mut BenchmarkGroup<WallTime>, data: Vec<u8>, name: String) {
    if data.len() > 1_000_000 {
        g.sample_size(10);
    }

    fn create_reader(data: &[u8]) -> Reader<&[u8]> {
        let mut decoder = Decoder::new(data);

        // Cover default transformations used by the `image` crate when constructing
        // `image::codecs::png::PngDecoder`.
        decoder.set_transformations(Transformations::EXPAND);

        decoder.read_info().unwrap()
    }

    let mut reader = create_reader(data.as_slice());
    let mut image = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut image).unwrap();

    g.throughput(Throughput::Bytes(info.buffer_size() as u64));
    g.bench_with_input(name, &data, |b, data| {
        b.iter(|| {
            let mut reader = create_reader(data.as_slice());
            reader.next_frame(&mut image).unwrap();
        })
    });
}
