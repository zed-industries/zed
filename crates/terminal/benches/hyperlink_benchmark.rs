use alacritty_terminal::{
    grid::Dimensions,
    index::{Column, Point as AlacPoint},
    term::test::mock_term,
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use terminal::bench::find_from_grid_point_bench;

pub fn hyperlink_benchmark(c: &mut Criterion) {
    let line =
        "    Compiling terminal v0.1.0 (/Users/dave/Source/zed-hyperlinks/crates/terminal)\r\n"
            .repeat(4000);
    let term = mock_term(&line);
    let point = AlacPoint::new(
        term.grid().bottommost_line() - 1,
        Column(term.grid().last_column().0 / 2),
    );

    c.bench_function("find_from_grid_point_bench", |b| {
        b.iter(|| black_box(find_from_grid_point_bench(&term, point)))
    });
}

criterion_group!(benches, hyperlink_benchmark);
criterion_main!(benches);
