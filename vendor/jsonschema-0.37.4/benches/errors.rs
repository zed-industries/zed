#[cfg(not(target_arch = "wasm32"))]
mod bench {
    pub(crate) use benchmark::run_error_formatting_benchmarks;
    pub(crate) use criterion::{criterion_group, BenchmarkId, Criterion};
    pub(crate) use serde_json::Value;

    pub(crate) fn bench_error_formatting(
        c: &mut Criterion,
        name: &str,
        schema: &Value,
        instance: &Value,
    ) {
        let validator = jsonschema::validator_for(schema).expect("Valid schema");
        let error = validator.validate(instance).unwrap_err();

        c.bench_with_input(
            BenchmarkId::new("error_formatting", name),
            &error,
            |b, error| b.iter_with_large_drop(|| error.to_string()),
        );
    }

    pub(crate) fn run_benchmarks(c: &mut Criterion) {
        run_error_formatting_benchmarks(&mut |name, schema, instance| {
            bench_error_formatting(c, name, schema, instance);
        });
    }

    criterion_group!(error_formatting, run_benchmarks);
}

#[cfg(not(target_arch = "wasm32"))]
criterion::criterion_main!(bench::error_formatting);

#[cfg(target_arch = "wasm32")]
fn main() {}
