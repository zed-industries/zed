#[cfg(not(target_arch = "wasm32"))]
mod bench {
    pub(crate) use benchmark::run_keyword_benchmarks;
    pub(crate) use criterion::{criterion_group, BenchmarkId, Criterion};
    pub(crate) use serde_json::Value;

    pub(crate) fn validator_for(schema: &Value) -> jsonschema::Validator {
        jsonschema::options()
            .with_draft(jsonschema::Draft::Draft7)
            .build(schema)
            .expect("Schema used in benchmarks should compile")
    }

    pub(crate) fn bench_keyword_build(c: &mut Criterion, name: &str, schema: &Value) {
        c.bench_function(&format!("keyword/{name}/build"), |b| {
            b.iter_with_large_drop(|| validator_for(schema));
        });
    }

    pub(crate) fn bench_keyword_is_valid(
        c: &mut Criterion,
        name: &str,
        schema: &Value,
        instance: &Value,
    ) {
        let validator = validator_for(schema);
        c.bench_with_input(
            BenchmarkId::new(format!("keyword/{name}"), "is_valid"),
            instance,
            |b, instance| {
                b.iter(|| {
                    let _ = validator.is_valid(instance);
                });
            },
        );
    }

    pub(crate) fn bench_keyword_validate(
        c: &mut Criterion,
        name: &str,
        schema: &Value,
        instance: &Value,
    ) {
        let validator = validator_for(schema);
        c.bench_with_input(
            BenchmarkId::new(format!("keyword/{name}"), "validate"),
            instance,
            |b, instance| {
                b.iter(|| {
                    let _ = validator.validate(instance);
                });
            },
        );
    }

    pub(crate) fn run_benchmarks(c: &mut Criterion) {
        run_keyword_benchmarks(&mut |name, schema, instances| {
            bench_keyword_build(c, name, schema);
            for instance in instances {
                let name = format!("jsonschema/{}/{}", name, instance.name);
                bench_keyword_is_valid(c, &name, schema, &instance.data);
                bench_keyword_validate(c, &name, schema, &instance.data);
            }
        });
    }

    criterion_group!(keywords, run_benchmarks);
}

#[cfg(not(target_arch = "wasm32"))]
criterion::criterion_main!(bench::keywords);

#[cfg(target_arch = "wasm32")]
fn main() {}
