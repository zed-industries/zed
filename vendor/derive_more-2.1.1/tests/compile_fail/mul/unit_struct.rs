#[derive(derive_more::Mul)]
struct Foo;

#[derive(derive_more::Mul)]
#[mul(forward)]
struct Bar;

fn main() {}
