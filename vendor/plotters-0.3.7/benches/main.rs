use criterion::criterion_main;

mod benches;

criterion_main! {
    benches::data::quartiles_group
}
