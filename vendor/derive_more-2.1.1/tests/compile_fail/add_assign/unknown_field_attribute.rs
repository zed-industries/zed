#[derive(derive_more::AddAssign)]
struct Foo(#[add_assign(unknown)] i32);

fn main() {}
