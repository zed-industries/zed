use criterion::{criterion_group, Criterion};
use plotters::data::Quartiles;

struct Lcg {
    state: u32,
}

impl Lcg {
    fn new() -> Lcg {
        Lcg { state: 0 }
    }
}

impl Iterator for Lcg {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.state = self.state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.state %= 1 << 31;
        Some(self.state)
    }
}

fn quartiles_calc(c: &mut Criterion) {
    let src: Vec<u32> = Lcg::new().take(100000).collect();
    c.bench_function("data::quartiles_calc", |b| {
        b.iter(|| {
            Quartiles::new(&src);
        })
    });
}

criterion_group! {
    name = quartiles_group;
    config = Criterion::default().sample_size(10);
    targets = quartiles_calc
}
