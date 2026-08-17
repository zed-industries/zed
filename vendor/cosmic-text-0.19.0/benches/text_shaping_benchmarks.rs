use cosmic_text as ct;
use cosmic_text::BidiParagraphs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_ascii_fast_path(c: &mut Criterion) {
    let mut fs = ct::FontSystem::new();
    let mut buffer = ct::Buffer::new(&mut fs, ct::Metrics::new(14.0, 20.0));
    buffer.set_size(Some(500.0), None);

    let ascii_text = "Pure ASCII text for BidiParagraphs optimization testing.\n".repeat(50);

    c.bench_function("ShapeLine/ASCII Fast Path", |b| {
        b.iter(|| {
            buffer.set_text(
                black_box(&ascii_text),
                &ct::Attrs::new(),
                ct::Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut fs, false);
        });
    });
}

fn bench_bidi_processing(c: &mut Criterion) {
    let mut fs = ct::FontSystem::new();
    let mut buffer = ct::Buffer::new(&mut fs, ct::Metrics::new(14.0, 20.0));
    buffer.set_size(Some(500.0), None);

    let bidi_text = "Mixed English and العربية النص العربي text for BiDi testing.\nThis tests adjust_levels and combined BiDi optimizations.\n".repeat(30);

    c.bench_function("ShapeLine/BiDi Processing", |b| {
        b.iter(|| {
            buffer.set_text(
                black_box(&bidi_text),
                &ct::Attrs::new(),
                ct::Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut fs, false);
        });
    });
}

fn bench_lang_mixed(c: &mut Criterion) {
    let mut fs = ct::FontSystem::new();
    let mut buffer = ct::Buffer::new(&mut fs, ct::Metrics::new(14.0, 20.0));
    buffer.set_size(Some(500.0), None);

    let bidi_text = include_str!("../sample/hello.txt");

    c.benchmark_group("bench_lang_mixed")
        .sample_size(10)
        .bench_function("ShapeLine/Mixed-Language Text", |b| {
            b.iter(|| {
                buffer.set_text(
                    black_box(&bidi_text),
                    &ct::Attrs::new(),
                    ct::Shaping::Advanced,
                    None,
                );
                buffer.shape_until_scroll(&mut fs, false);
            });
        });
}

fn bench_layout_heavy(c: &mut Criterion) {
    let mut fs = ct::FontSystem::new();
    let mut buffer = ct::Buffer::new(&mut fs, ct::Metrics::new(14.0, 20.0));
    buffer.set_size(Some(500.0), None);

    let layout_text = "This is a very long line that will wrap multiple times and stress the reorder optimization through intensive layout processing with comprehensive buffer reuse testing. ".repeat(30);

    c.bench_function("ShapeLine/Layout Heavy", |b| {
        b.iter(|| {
            buffer.set_text(
                black_box(&layout_text),
                &ct::Attrs::new(),
                ct::Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut fs, false);
        });
    });
}

fn bench_combined_stress(c: &mut Criterion) {
    let mut fs = ct::FontSystem::new();
    let mut buffer = ct::Buffer::new(&mut fs, ct::Metrics::new(14.0, 20.0));
    buffer.set_size(Some(500.0), None);

    let stress_text = format!("{}\n{}\n{}\n{}\n",
        "ASCII line for BidiParagraphs optimization. ".repeat(15),
        "Mixed English + العربية for BiDi optimizations. ".repeat(12),
        "Very long wrapping line that will trigger reorder optimizations multiple times through intensive layout processing. ".repeat(8),
        "Cache key generation line for ShapeRunKey optimization testing. ".repeat(10)
    ).repeat(10);

    c.bench_function("ShapeLine/Combined Stress", |b| {
        b.iter(|| {
            buffer.set_text(
                black_box(&stress_text),
                &ct::Attrs::new(),
                ct::Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut fs, false);
        });
    });
}

fn bench_bidi_paragraphs_ascii(c: &mut Criterion) {
    let ascii_text = "Simple ASCII text\nwith multiple lines\n".repeat(50);

    c.bench_function("BidiParagraphs/ASCII", |b| {
        b.iter(|| {
            let paras = BidiParagraphs::new(black_box(&ascii_text));
            black_box(paras.count());
        });
    });
}

fn bench_bidi_paragraphs_mixed(c: &mut Criterion) {
    let mixed_text = "Mixed English and العربية text\nwith multiple lines\n".repeat(30);

    c.bench_function("BidiParagraphs/Mixed", |b| {
        b.iter(|| {
            let paras = BidiParagraphs::new(black_box(&mixed_text));
            black_box(paras.count());
        });
    });
}

criterion_group!(
    benches,
    bench_ascii_fast_path,
    bench_bidi_processing,
    bench_lang_mixed,
    bench_layout_heavy,
    bench_combined_stress,
    bench_bidi_paragraphs_ascii,
    bench_bidi_paragraphs_mixed
);
criterion_main!(benches);
