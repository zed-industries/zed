#[derive(derive_more::MulAssign)]
struct Foo(#[mul_assign(skip)] i32);

#[derive(derive_more::MulAssign)]
#[mul_assign(forward)]
struct Bar(#[mul_assign(skip)] i32);

fn main() {}
