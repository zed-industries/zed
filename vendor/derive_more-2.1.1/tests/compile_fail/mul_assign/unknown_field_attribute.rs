#[derive(derive_more::MulAssign)]
struct Foo(#[mul_assign(unknown)] i32);

fn main() {}
