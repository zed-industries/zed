#[derive(derive_more::FromStr)]
#[from_str(rename_all = "lowercase")]
pub struct Foo {
    bar: i32,
}

fn main() {}
