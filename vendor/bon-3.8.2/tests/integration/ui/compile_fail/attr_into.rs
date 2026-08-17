use bon::builder;

#[builder]
fn invalid_into_false(#[builder(into = false)] _x: u32) {}

fn main() {}
