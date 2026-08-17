#[derive(derive_more::Mul)]
#[mul(ignore)]
struct Foo(i32);

#[derive(derive_more::Mul)]
#[mul(skip)]
enum Enum {
    Bar { i: i32 },
}

#[derive(derive_more::Mul)]
#[mul(forward)]
enum Enum2 {
    #[mul(skip)]
    Bar { i: i32 },
}

fn main() {}
