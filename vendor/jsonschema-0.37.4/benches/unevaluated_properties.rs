#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use codspeed_criterion_compat::{criterion_group, BenchmarkId, Criterion};
    use serde_json::{json, Value};
    use std::hint::black_box;

    fn large_schema() -> Value {
        let mut properties = serde_json::Map::new();
        for i in 0..50 {
            properties.insert(format!("prop{i}"), json!({"type": "string"}));
        }

        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": properties,
            "patternProperties": {
                "^meta_": {"type": "string"}
            },
            "allOf": [
                {
                    "if": {
                        "properties": {
                            "type": {"const": "user"}
                        }
                    },
                    "then": {
                        "properties": {
                            "email": {"type": "string", "format": "email"},
                            "age": {"type": "integer", "minimum": 0}
                        }
                    },
                    "else": {
                        "properties": {
                            "organization": {"type": "string"}
                        }
                    }
                }
            ],
            "anyOf": [
                {
                    "properties": {
                        "status": {"enum": ["active", "inactive"]}
                    }
                },
                {
                    "properties": {
                        "archived": {"type": "boolean"}
                    }
                }
            ],
            "dependentSchemas": {
                "credit_card": {
                    "properties": {
                        "billing_address": {"type": "string"}
                    }
                }
            },
            "unevaluatedProperties": false
        })
    }

    fn large_invalid_instance() -> Value {
        let mut obj = serde_json::Map::new();
        for i in 0..50 {
            obj.insert(format!("prop{i}"), json!("value"));
        }
        obj.insert("unexpected".to_string(), json!("property"));
        Value::Object(obj)
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
        bench_build(c, "unevaluated_properties", &large);

        let validator = jsonschema::draft202012::new(&large).unwrap();
        let invalid = large_invalid_instance();

        bench_is_valid(c, "unevaluated_properties", &validator, &invalid);
        bench_validate(c, "unevaluated_properties", &validator, &invalid);
        bench_evaluate(c, "unevaluated_properties", &validator, &invalid);
        bench_iter_errors(c, "unevaluated_properties", &validator, &invalid);
    }

    criterion_group!(benches, run_benchmarks);
}

#[cfg(not(target_arch = "wasm32"))]
codspeed_criterion_compat::criterion_main!(bench::benches);

#[cfg(target_arch = "wasm32")]
fn main() {}
