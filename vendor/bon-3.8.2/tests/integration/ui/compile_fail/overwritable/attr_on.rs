use bon::builder;

#[builder(on(String, overwritable))]
fn unnecessary_overwritable(#[builder(overwritable)] _x: String) {}

fn main() {}
