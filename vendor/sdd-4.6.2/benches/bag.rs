use criterion::{Criterion, criterion_group, criterion_main};
use sdd::Bag;

fn bag_push_pop(c: &mut Criterion) {
    let bag: Bag<usize> = Bag::default();
    let mut i: usize = 0;
    c.bench_function("Bag: push-pop", |b| {
        b.iter(|| {
            bag.push(i);
            let p = bag.pop();
            assert_eq!(p, Some(i));
            i += 1;
        })
    });
}

criterion_group!(bag, bag_push_pop);
criterion_main!(bag);
