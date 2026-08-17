use criterion::{black_box, criterion_group, Criterion};

use anes::parser::Parser;

pub fn parser(c: &mut Criterion) {
    const XTERM_MOUSE: &str = "\x1B[<28;20;10;m";

    let mut parser = Parser::default();

    c.bench_function("advance and consume", |b| {
        let input = XTERM_MOUSE.as_bytes();

        b.iter(|| {
            parser.advance(black_box(input), black_box(true));
            while let Some(_) = parser.next() {}
        })
    });
}

criterion_group!(benches, parser);
