#[derive(derive_more::Mul)]
#[mul(unknown)]
struct Foo( i32);

#[derive(derive_more::Mul)]
#[mul(unknown)]
enum Enum {
    Bar { i: i32 },
}

fn main() {}
