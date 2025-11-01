use alacritty_terminal::{
    event::VoidListener,
    grid::Dimensions,
    index::{Column, Point as AlacPoint},
    term::Term,
    term::test::mock_term,
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use terminal::terminal_hyperlinks::bench::find_from_grid_point_bench;

fn build_test_term(line: &str) -> (Term<VoidListener>, AlacPoint) {
    let content = line.repeat(500);
    let term = mock_term(&content);
    let point = AlacPoint::new(
        term.grid().bottommost_line() - 1,
        Column(term.grid().last_column().0 / 2),
    );

    (term, point)
}

fn bench_find_from_grid_point(c: &mut Criterion, bench_name: &str, line: &str) {
    let (term, point) = build_test_term(line);
    c.bench_function(bench_name, |b| {
        b.iter(|| {
            black_box(
                find_from_grid_point_bench(&term, point).expect("Hyperlink should have been found"),
            )
        })
    });
}

pub fn cargo_hyperlink_benchmark(c: &mut Criterion) {
    const LINE: &str = "    Compiling terminal v0.1.0 (/Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal)\r\n";
    bench_find_from_grid_point(c, "cargo_hyperlink_benchmark", LINE);
}

pub fn rust_hyperlink_benchmark(c: &mut Criterion) {
    const LINE: &str =
        "    --> /Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal/terminal.rs:1000:42\r\n";
    bench_find_from_grid_point(c, "rust_hyperlink_benchmark", LINE);
}

pub fn ls_hyperlink_benchmark(c: &mut Criterion) {
    const LINE: &str =
        "Cargo.toml        experiments        notebooks        rust-toolchain.toml    tooling\r\n";
    bench_find_from_grid_point(c, "ls_hyperlink_benchmark", LINE);
}

criterion_group!(
    benches,
    cargo_hyperlink_benchmark,
    rust_hyperlink_benchmark,
    ls_hyperlink_benchmark
);
criterion_main!(benches);
