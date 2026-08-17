mod parser;

use criterion::black_box;
use winnow::prelude::*;

fn pratt(c: &mut criterion::Criterion) {
    let i =
        "a = 2*-2 / ( &**foo.a->p! -+1) + 3^1 / 4 == 1 * (2 - 7 + 567 *12 /2) + 3*(1+2*( 45 /2))";
    parser::pratt_parser.parse(i).expect("should parse");
    c.bench_function("pratt_parser", |b| {
        b.iter(|| black_box(parser::pratt_parser.parse(i).unwrap()));
    });
}

criterion::criterion_group!(benches, pratt);
criterion::criterion_main!(benches);
