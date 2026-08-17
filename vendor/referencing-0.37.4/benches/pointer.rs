use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use referencing::{Draft, Registry};
use serde_json::{json, Value};

fn create_deep_nested_json(depth: usize) -> Value {
    let mut current = json!({
        "leaf": "value",
        "array": [1, 2, 3],
        "special/field": "escaped",
        "encoded field": "percent encoded"
    });

    for i in (0..depth).rev() {
        current = json!({
            format!("level_{}", i): current,
            "sibling": format!("sibling_value_{}", i)
        });
    }

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": current
    })
}

fn bench_pointers(c: &mut Criterion) {
    let data = create_deep_nested_json(15);
    let resource = Draft::Draft202012.create_resource(data);
    let registry = Registry::try_new("http://example.com/schema.json", resource)
        .expect("Invalid registry input");

    let cases = [
        ("single", "#/properties"),
        ("double", "#/properties/level_0"),
        ("long", "#/properties/level_0/level_1/level_2/level_3/level_4/level_5/level_6/level_7/level_8/level_9/level_10/level_11/level_12/level_13/level_14/array/1"),
    ];

    let mut group = c.benchmark_group("JSON Pointer");

    for (name, pointer) in cases {
        group.bench_with_input(
            BenchmarkId::new("pointer", name),
            &registry,
            |b, registry| {
                let resolver = registry
                    .try_resolver("http://example.com/schema.json")
                    .expect("Invalid base URI");
                b.iter_with_large_drop(|| resolver.lookup(black_box(pointer)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_pointers);
criterion_main!(benches);
