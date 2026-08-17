use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use proptest::{prelude::*, strategy::ValueTree, test_runner::TestRunner};
use rangemap::*;
use std::{any::type_name, fmt::Debug, ops::*};
use test_strategy::Arbitrary;

type Key = i64;
type Value = i64;

const COUNT: usize = 100000;
const OPERATIONS: usize = 100000;
const LOOKUPS: usize = 1000000;

#[derive(Debug, Clone, Arbitrary)]
enum Operation<K, V> {
    Insert(K, V),
    Delete(K),
}

fn range_inclusive_map<K: Ord + StepLite + Debug + Clone, V: Eq + Clone + Debug>(
    values: impl Strategy<Value = (RangeInclusive<K>, V)>,
    size: usize,
) -> impl Strategy<Value = RangeInclusiveMap<K, V>> {
    prop::collection::vec(values, size)
        .prop_map(|ranges| ranges.into_iter().collect::<RangeInclusiveMap<K, V>>())
}

fn range_map<K: Ord + StepLite + Debug + Clone, V: Eq + Clone + Debug>(
    values: impl Strategy<Value = (Range<K>, V)>,
    size: usize,
) -> impl Strategy<Value = RangeMap<K, V>> {
    prop::collection::vec(values, size)
        .prop_map(|ranges| ranges.into_iter().collect::<RangeMap<K, V>>())
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();

    let mut group = c.benchmark_group(format!(
        "RangeMap<{}, {}>",
        type_name::<Key>(),
        type_name::<Value>()
    ));

    group.throughput(Throughput::Elements(COUNT as u64));
    group.bench_function("insert", |b| {
        let entries = prop::collection::vec(any::<(Range<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        b.iter_with_large_drop(|| {
            let mut map = RangeMap::new();
            for (range, value) in entries.clone().into_iter() {
                map.insert(range, value);
            }
            map
        })
    });

    group.throughput(Throughput::Elements(OPERATIONS as u64));
    group.bench_function("operations", |b| {
        let map = range_map(any::<(Range<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        let operations = prop::collection::vec(any::<Operation<Range<Key>, Value>>(), OPERATIONS)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        b.iter_with_large_drop(|| {
            let mut map = map.clone();
            for operation in operations.clone().into_iter() {
                match operation {
                    Operation::Insert(key, value) => map.insert(key, value),
                    Operation::Delete(key) => map.remove(key),
                }
            }
            map
        })
    });

    group.throughput(Throughput::Elements(LOOKUPS as u64));
    group.bench_function("lookups", |b| {
        let map = range_map(any::<(Range<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        let lookups = prop::collection::vec(any::<Key>(), LOOKUPS)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        b.iter(|| {
            for lookup in lookups.iter() {
                black_box(map.get(lookup));
            }
        })
    });

    group.finish();

    let mut group = c.benchmark_group(format!(
        "RangeInclusiveMap<{}, {}>",
        type_name::<Key>(),
        type_name::<Value>()
    ));

    group.throughput(Throughput::Elements(COUNT as u64));
    group.bench_function("insert", |b| {
        let entries = prop::collection::vec(any::<(RangeInclusive<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        b.iter_with_large_drop(|| {
            let mut map = RangeInclusiveMap::new();
            for (range, value) in entries.clone().into_iter() {
                map.insert(range, value);
            }
            map
        })
    });

    group.throughput(Throughput::Elements(OPERATIONS as u64));
    group.bench_function("operations", |b| {
        let map = range_inclusive_map(any::<(RangeInclusive<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        let operations =
            prop::collection::vec(any::<Operation<RangeInclusive<Key>, Value>>(), OPERATIONS)
                .new_tree(&mut runner)
                .unwrap()
                .current();
        b.iter_with_large_drop(|| {
            let mut map = map.clone();
            for operation in operations.clone().into_iter() {
                match operation {
                    Operation::Insert(key, value) => map.insert(key, value),
                    Operation::Delete(key) => map.remove(key),
                }
            }
            map
        })
    });

    group.throughput(Throughput::Elements(LOOKUPS as u64));
    group.bench_function("lookups", |b| {
        let map = range_inclusive_map(any::<(RangeInclusive<Key>, Value)>(), COUNT)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        let lookups = prop::collection::vec(any::<Key>(), LOOKUPS)
            .new_tree(&mut runner)
            .unwrap()
            .current();
        b.iter(|| {
            for lookup in lookups.iter() {
                black_box(map.get(lookup));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
