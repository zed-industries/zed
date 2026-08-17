#[macro_use]
extern crate derive_builder;

#[derive(Builder)]
#[builder(public, vis = "pub(crate)")]
pub struct Example {
    #[builder(public, private)]
    field: String,
}

#[derive(Builder)]
#[builder(public, vis = "pub(crate)", build_fn(private, public))]
pub struct SecondExample {
    #[builder(private, vis = "pub")]
    a_field: String,
}

fn main() {
    ExampleBuilder::default().build();
    SecondExampleBuilder::default().build();
}
