use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::distributions::{Alphanumeric};
use rand::prelude::{Distribution, StdRng};
use rand::{SeedableRng};
use gaoya::minhash::{compute_minhash_similarity, MinHasher, MinHasher32, MinHashIndex};
use gaoya::simhash::{SimHash, SimHashBits, SimHashIndex, SimSipHasher128, SimSipHasher64};
use gaoya::text::shingle_text;


fn bench_create_minhash(c: &mut Criterion) {
    let min_hasher_32_256 = MinHasher32::new(256);
    let mut group = c.benchmark_group("bench_create_minhash");
    let text =  "In computer science and data mining, MinHash (or the min-wise independent permutations locality sensitive hashing scheme) is a technique for quickly estimating how similar two sets are. ";
    group.throughput(Throughput::Elements(1));
    group.bench_function("create_minhash", |b| b.iter(|| {
             black_box(min_hasher_32_256.create_signature(shingle_text(text, 3)));
    }));
    group.finish();
}


fn bench_minhash_similarity(c: &mut Criterion) {
    let min_hasher_32_256 = MinHasher32::new(256);
    let minhash1 = min_hasher_32_256.create_signature(shingle_text("Hello, World", 3));
    let minhash2 = min_hasher_32_256.create_signature(shingle_text("Bonjour, Monde", 3));
    let mut group = c.benchmark_group("bench_minhash");
    group.throughput(Throughput::Elements(1));
    group.bench_function("compute_similarity", |b| b.iter(|| {
        black_box(compute_minhash_similarity(minhash1.as_slice(), minhash2.as_slice()));
    }));
    group.finish();
}

fn minhash_million_items_most_similar(c: &mut Criterion) {
    let min_hasher_32_256 = MinHasher32::new(250);
    let seed = [1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

    let mut rng = StdRng::from_seed(seed);
    let signatures: Vec<_> = (0..1_000_000).into_iter()
        .map(|_i| {
            let s: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(100)
                .map(char::from)
                .collect();
            min_hasher_32_256.create_signature(shingle_text(s.as_str(), 3))
        }).collect();
    let ids = (0..1_000_000).into_iter().collect();
    let mut min_hash_index = MinHashIndex::new(50, 5, 0.7);
    min_hash_index.par_bulk_insert(ids, signatures.clone());
    let first = signatures.first().unwrap().clone();

    let mut group = c.benchmark_group("minhash_most_similar");
    group.throughput(Throughput::Elements(1));
    group.bench_function("brute-force", |b| b.iter(|| {
        let result = signatures.iter().map(|s| {
            compute_minhash_similarity(first.as_slice(), &s)
        }).max_by(|x, y| x.partial_cmp(y).unwrap());
        black_box(result);
    }));

    group.bench_function("minhash-index", |b| b.iter(|| {
        let result = min_hash_index.query_one(&first);
        assert_ne!(result, None);
        black_box(result);
    }));

    group.finish();
}


fn simhash_million_items_most_similar(c: &mut Criterion) {
    let sim_hash = SimHash::<SimSipHasher128, u128, 128>::new(SimSipHasher128::new(5, 6));

    let seed = [1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];

    let mut rng = StdRng::from_seed(seed);
    let signatures: Vec<_> = (0..1_000_000).into_iter()
        .map(|_i| {
            let s: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(100)
                .map(char::from)
                .collect();
            sim_hash.create_signature(shingle_text(s.as_str(), 3))
        }).collect();
    let ids = (0..1_000_000).into_iter().collect();
    let mut index = SimHashIndex::new(12, 9);
    index.par_bulk_insert(ids, signatures.clone());
    let first = signatures.first().unwrap().clone();

    let mut group = c.benchmark_group("simhash_most_similar");
    group.throughput(Throughput::Elements(1));
    group.bench_function("brute-force", |b| b.iter(|| {
        let result = signatures.iter().map(|s| {
            first.hamming_distance(&s)
        }).max_by(|x, y| x.cmp(y));
        black_box(result);
    }));

    group.bench_function("simhash-index", |b| b.iter(|| {
        let result = index.query_one(&first);
        assert_ne!(result, None);
        black_box(result);
    }));

    group.finish();
}


criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_create_minhash,bench_minhash_similarity,minhash_million_items_most_similar, simhash_million_items_most_similar
}


criterion_main!(benches);
