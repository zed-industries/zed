#[derive(derive_more::Mul)]
struct Foo(#[mul(skip)] i32);

#[derive(derive_more::Mul)]
#[mul(forward)]
struct Bar(#[mul(skip)] i32);

#[derive(derive_more::Mul)]
#[mul(forward)]
enum Enum {
    Baz { #[mul(skip)] x: i32, #[mul(ignore)] y: i32 },
}

fn main() {}
