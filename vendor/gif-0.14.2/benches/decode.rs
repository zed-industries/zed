use criterion::{measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use gif::Decoder;
use std::hint::black_box;

fn read_image(image: &[u8]) -> Option<Vec<u8>> {
    let decoder = Decoder::new(black_box(image));
    //decoder.set_param(gif::ColorOutput::RGBA);
    let mut reader = decoder.unwrap();

    if reader.next_frame_info().unwrap().is_some() {
        let mut v = vec![0; reader.buffer_size()];
        reader.fill_buffer(&mut v).unwrap();
        Some(v)
    } else {
        None
    }
}

fn read_metadata(image: &[u8]) {
    let decoder = Decoder::new(black_box(image));
    decoder.unwrap();
}

fn main() {
    struct BenchDef {
        data: &'static [u8],
        id: &'static str,
        sample_size: usize,
    }

    fn run_bench_def<M: Measurement>(group: &mut BenchmarkGroup<'_, M>, def: BenchDef) {
        group
            .sample_size(def.sample_size)
            .throughput(Throughput::Bytes(def.data.len() as u64))
            .bench_with_input(
                BenchmarkId::new(def.id, def.data.len()),
                def.data,
                |b, input| {
                    b.iter(|| read_image(input));
                },
            );
    }

    let mut c = Criterion::default().configure_from_args();
    let mut group = c.benchmark_group("gif");

    run_bench_def(
        &mut group,
        BenchDef {
            data: include_bytes!("note.gif"),
            id: "note.gif",
            sample_size: 100,
        },
    );

    run_bench_def(
        &mut group,
        BenchDef {
            data: include_bytes!("photo.gif"),
            id: "photo.gif",
            sample_size: 20,
        },
    );

    run_bench_def(
        &mut group,
        BenchDef {
            data: include_bytes!("../tests/samples/sample_1.gif"),
            id: "sample_1.gif",
            sample_size: 100,
        },
    );

    run_bench_def(
        &mut group,
        BenchDef {
            data: include_bytes!("../tests/samples/sample_big.gif"),
            id: "sample_big.gif",
            sample_size: 20,
        },
    );

    group.bench_with_input(
        "extract-metadata-note",
        include_bytes!("note.gif"),
        |b, input| {
            b.iter(|| read_metadata(input));
        },
    );

    group.finish();

    c.final_summary();
}
