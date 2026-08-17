#[derive(derive_more::FromStr)]
#[from_str(unknown = "unknown")]
pub enum Foo {
    Bar,
}

fn main() {}
