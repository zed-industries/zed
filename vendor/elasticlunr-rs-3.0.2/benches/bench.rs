use criterion::{black_box, criterion_group, criterion_main, Criterion};
use elasticlunr::Index;

fn bench_main(c: &mut Criterion) {
    // BTreeMap<String, IndexItem>: 3,165,389 ns/iter (+/- 420,869)
    // BTreeMap<char, IndexItem>:   2,920,902 ns/iter (+/- 118,729)
    c.bench_function("create_index", |b| {
        let text = include_str!("../tests/data/en.in.txt");
        let sections: Vec<_> = text.split("\n\n").collect();
        b.iter(|| {
            let mut index = Index::new(&["section"]);
            for (i, section) in sections.iter().enumerate() {
                index.add_doc(&format!("section_{}", i), &[section]);
            }
            black_box(index.to_json());
        })
    });
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
