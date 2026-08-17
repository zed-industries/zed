#[derive(derive_more::Mul)]
struct Foo(#[mul(unknown)] i32);

#[derive(derive_more::Mul)]
enum Enum {
    Bar { #[mul(unknown)] i: i32 },
}

fn main() {}
