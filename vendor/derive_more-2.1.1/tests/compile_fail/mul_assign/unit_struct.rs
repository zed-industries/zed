#[derive(derive_more::MulAssign)]
struct Foo;

#[derive(derive_more::MulAssign)]
#[mul_assign(forward)]
struct Bar;

fn main() {}
