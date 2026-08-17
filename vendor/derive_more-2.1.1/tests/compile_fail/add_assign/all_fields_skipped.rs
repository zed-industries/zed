#[derive(derive_more::AddAssign)]
struct Foo(#[add_assign(skip)] i32);

fn main() {}
