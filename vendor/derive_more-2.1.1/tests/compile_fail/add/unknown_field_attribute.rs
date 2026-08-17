#[derive(derive_more::Add)]
struct Foo(#[add(unknown)] i32);

#[derive(derive_more::Add)]
enum Enum {
    Bar { #[add(unknown)] i: i32 },
}

fn main() {}
