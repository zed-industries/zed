#[derive(derive_more::Add)]
struct Foo(#[add(skip)] i32);

#[derive(derive_more::Add)]
enum Enum {
    Bar { #[add(skip)] x: i32, #[add(ignore)] y: i32 },
}

fn main() {}
