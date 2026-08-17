#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use codspeed_criterion_compat::{criterion_group, BenchmarkId, Criterion};
    use serde_json::{json, Value};
    use std::hint::black_box;

    fn large_schema() -> Value {
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                {"type": "string"},
                {"type": "number"},
                {"type": "boolean"}
            ],
            "contains": {
                "type": "object",
                "properties": {
                    "special": {"type": "boolean"}
                }
            },
            "allOf": [
                {
                    "if": {
                        "contains": {"const": "trigger"}
                    },
                    "then": {
                        "prefixItems": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"},
                            {"type": "string"}
                        ]
                    },
                    "else": {
                        "prefixItems": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"},
                            {"type": "number"}
                        ]
                    }
                }
            ],
            "anyOf": [
                {
                    "contains": {"type": "string"}
                },
                {
                    "contains": {"type": "number"}
                }
            ],
            "oneOf": [
                {
                    "prefixItems": [
                        {"type": "string"},
                        {"type": "number"},
                        {"type": "boolean"},
                        {"type": "string"},
                        {"type": "array"}
                    ]
                },
                {
                    "prefixItems": [
                        {"type": "string"},
                        {"type": "number"},
                        {"type": "boolean"},
                        {"type": "number"},
                        {"type": "object"}
                    ]
                }
            ],
            "unevaluatedItems": false
        })
    }

    fn large_invalid_instance() -> Value {
        json!([
            "string",
            42,
            true,
            123,
            {"special": true},
            "unexpected"  // This should trigger unevaluatedItems: false
        ])
    }

    fn bench_build(c: &mut Criterion, name: &str, schema: &Value) {
        c.bench_with_input(BenchmarkId::new("build", name), schema, |b, schema| {
            b.iter_with_large_drop(|| jsonschema::draft202012::new(schema).unwrap());
        });
    }

    fn bench_is_valid(
        c: &mut Criterion,
        name: &str,
        validator: &jsonschema::Validator,
        instance: &Value,
    ) {
        c.bench_with_input(
            BenchmarkId::new("is_valid", name),
            instance,
            |b, instance| {
                b.iter(|| {
                    black_box(validator.is_valid(instance));
                });
            },
        );
    }

    fn bench_validate(
        c: &mut Criterion,
        name: &str,
        validator: &jsonschema::Validator,
        instance: &Value,
    ) {
        c.bench_with_input(
            BenchmarkId::new("validate", name),
            instance,
            |b, instance| {
                b.iter(|| {
                    let _ = black_box(validator.validate(instance));
                });
            },
        );
    }

    fn bench_evaluate(
        c: &mut Criterion,
        name: &str,
        validator: &jsonschema::Validator,
        instance: &Value,
    ) {
        c.bench_with_input(
            BenchmarkId::new("evaluate", name),
            instance,
            |b, instance| {
                b.iter_with_large_drop(|| black_box(validator.evaluate(instance)));
            },
        );
    }

    fn bench_iter_errors(
        c: &mut Criterion,
        name: &str,
        validator: &jsonschema::Validator,
        instance: &Value,
    ) {
        c.bench_with_input(
            BenchmarkId::new("iter_errors", name),
            instance,
            |b, instance| {
                b.iter(|| {
                    for error in validator.iter_errors(instance) {
                        black_box(error);
                    }
                });
            },
        );
    }

    fn run_benchmarks(c: &mut Criterion) {
        let large = large_schema();
        bench_build(c, "unevaluated_items", &large);

        let validator = jsonschema::draft202012::new(&large).unwrap();
        let invalid = large_invalid_instance();

        bench_is_valid(c, "unevaluated_items", &validator, &invalid);
        bench_validate(c, "unevaluated_items", &validator, &invalid);
        bench_evaluate(c, "unevaluated_items", &validator, &invalid);
        bench_iter_errors(c, "unevaluated_items", &validator, &invalid);
    }

    criterion_group!(benches, run_benchmarks);
}

#[cfg(not(target_arch = "wasm32"))]
codspeed_criterion_compat::criterion_main!(bench::benches);

#[cfg(target_arch = "wasm32")]
fn main() {}
