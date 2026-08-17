use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use referencing::{Draft, Registry};
use serde_json::json;

fn bench_anchor_lookup(c: &mut Criterion) {
    let data = json!({
      "definitions": {
        "foo": {
          "id": "#foo",
          "foo": "bar"
        }
      }
    });
    let resource = Draft::Draft4.create_resource(data);
    let registry =
        Registry::try_new("http://example.com/", resource).expect("Invalid registry input");

    let mut group = c.benchmark_group("Anchor Lookup");

    // Benchmark lookup of existing anchor
    group.bench_with_input(
        BenchmarkId::new("resolve", "small"),
        &registry,
        |b, registry| {
            let resolver = registry
                .try_resolver("http://example.com/")
                .expect("Invalid base URI");
            b.iter_with_large_drop(|| resolver.lookup(black_box("#foo")));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_anchor_lookup);
criterion_main!(benches);
