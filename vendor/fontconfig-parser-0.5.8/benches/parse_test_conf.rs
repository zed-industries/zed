use criterion::{criterion_group, criterion_main, Criterion};

fn parse_bench(c: &mut Criterion) {
    c.bench_function("parse_full", |b| {
        b.iter(|| {
            let s = std::fs::read_to_string("test-conf/fonts.conf").unwrap();
            fontconfig_parser::parse_config_parts(&s).unwrap();
        });
    });
}

fn merge_bench(c: &mut Criterion) {
    c.bench_function("merge_full", |b| {
        b.iter(|| {
            let mut c = fontconfig_parser::FontConfig::default();
            c.merge_config("test-conf/fonts.conf").unwrap();
        });
    });
}

criterion_group!(benches, parse_bench, merge_bench);
criterion_main!(benches);
