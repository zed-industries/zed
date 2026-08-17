#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use std::hint::black_box;

    pub(crate) use benchmark::Benchmark;
    pub(crate) use codspeed_criterion_compat::{criterion_group, BenchmarkId, Criterion};
    pub(crate) use serde_json::Value;

    pub(crate) fn bench_build(c: &mut Criterion, name: &str, schema: &Value) {
        c.bench_with_input(BenchmarkId::new("build", name), schema, |b, schema| {
            b.iter_with_large_drop(|| jsonschema::validator_for(schema).expect("Valid schema"));
        });
    }

    pub(crate) fn bench_is_valid(c: &mut Criterion, name: &str, schema: &Value, instance: &Value) {
        let validator = jsonschema::validator_for(schema).expect("Valid schema");
        c.bench_with_input(
            BenchmarkId::new("is_valid", name),
            instance,
            |b, instance| {
                b.iter(|| black_box(validator.is_valid(instance)));
            },
        );
    }

    pub(crate) fn bench_validate(c: &mut Criterion, name: &str, schema: &Value, instance: &Value) {
        let validator = jsonschema::validator_for(schema).expect("Valid schema");
        c.bench_with_input(
            BenchmarkId::new("validate", name),
            instance,
            |b, instance| {
                b.iter(|| black_box(validator.validate(instance)));
            },
        );
    }

    pub(crate) fn bench_evaluate(c: &mut Criterion, name: &str, schema: &Value, instance: &Value) {
        let validator = jsonschema::validator_for(schema).expect("Valid schema");
        c.bench_with_input(
            BenchmarkId::new("evaluate", name),
            instance,
            |b, instance| {
                b.iter_with_large_drop(|| black_box(validator.evaluate(instance)));
            },
        );
    }

    pub(crate) fn run_benchmarks(c: &mut Criterion) {
        for benchmark in Benchmark::iter() {
            benchmark.run(&mut |name, schema, instances| {
                bench_build(c, name, schema);
                for instance in instances {
                    let name = format!("{}/{}", name, instance.name);
                    bench_is_valid(c, &name, schema, &instance.data);
                    bench_validate(c, &name, schema, &instance.data);
                    bench_evaluate(c, &name, schema, &instance.data);
                }
            });
        }
    }

    criterion_group!(jsonschema, run_benchmarks);
}

#[cfg(not(target_arch = "wasm32"))]
codspeed_criterion_compat::criterion_main!(bench::jsonschema);

#[cfg(target_arch = "wasm32")]
fn main() {}
