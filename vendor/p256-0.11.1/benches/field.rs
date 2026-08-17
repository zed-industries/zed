//! secp256r1 field element benchmarks

use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
use hex_literal::hex;
use p256::FieldElement;

fn test_field_element_x() -> FieldElement {
    FieldElement::from_bytes(
        &hex!("1ccbe91c075fc7f4f033bfa248db8fccd3565de94bbfb12f3c59ff46c271bf83").into(),
    )
    .unwrap()
}

fn test_field_element_y() -> FieldElement {
    FieldElement::from_bytes(
        &hex!("ce4014c68811f9a21a1fdb2c0e6113e06db7ca93b7404e78dc7ccd5ca89a4ca9").into(),
    )
    .unwrap()
}

fn bench_field_element_mul<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let x = test_field_element_x();
    let y = test_field_element_y();
    group.bench_function("mul", |b| b.iter(|| &x * &y));
}

fn bench_field_element_square<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let x = test_field_element_x();
    group.bench_function("square", |b| b.iter(|| x.square()));
}

fn bench_field_element_sqrt<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let x = test_field_element_x();
    group.bench_function("sqrt", |b| b.iter(|| x.sqrt()));
}

fn bench_field_element_invert<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let x = test_field_element_x();
    group.bench_function("invert", |b| b.iter(|| x.invert()));
}

fn bench_field_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("field element operations");
    bench_field_element_mul(&mut group);
    bench_field_element_square(&mut group);
    bench_field_element_invert(&mut group);
    bench_field_element_sqrt(&mut group);
    group.finish();
}

criterion_group!(benches, bench_field_element);
criterion_main!(benches);
