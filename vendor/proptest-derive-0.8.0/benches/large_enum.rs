use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proptest::{prelude::*, test_runner::TestRunner};
use proptest_derive::Arbitrary;

#[derive(Arbitrary, Debug)]
#[proptest(no_params)]
enum LargeEnum1 {
    V1(String),
    V2(String),
    V3(String),
    V4(String),
    V5(String),
    V6(String),
    V7(String),
    V8(String),
    V9(String),
    V10(String),
    V11(String),
    V12(String),
    V13(String),
    V14(String),
    V15(String),
    V16(String),
}

#[derive(Arbitrary, Debug)]
#[proptest(no_params)]
enum LargeEnum2 {
    V1(LargeEnum1),
    V2(LargeEnum1),
    V3(LargeEnum1),
    V4(LargeEnum1),
    V5(LargeEnum1),
    V6(LargeEnum1),
    V7(LargeEnum1),
    V8(LargeEnum1),
    V9(LargeEnum1),
    V10(LargeEnum1),
    V11(LargeEnum1),
    V12(LargeEnum1),
    V13(LargeEnum1),
    V14(LargeEnum1),
    V15(LargeEnum1),
    V16(LargeEnum1),
}

fn enum1_bench(runner: &mut TestRunner) {
    let strategy = any::<LargeEnum1>();
    let _ = black_box(strategy.new_tree(runner));
}

fn enum2_bench(runner: &mut TestRunner) {
    let strategy = any::<LargeEnum2>();
    let _ = black_box(strategy.new_tree(runner));
}

fn enum_benchmark(c: &mut Criterion) {
    c.bench_function("enum 1", |b| {
        let mut runner = TestRunner::default();
        b.iter(|| enum1_bench(&mut runner))
    });
    c.bench_function("enum 2", |b| {
        let mut runner = TestRunner::default();
        b.iter(|| enum2_bench(&mut runner))
    });
}

criterion_group!(benches, enum_benchmark);
criterion_main!(benches);
