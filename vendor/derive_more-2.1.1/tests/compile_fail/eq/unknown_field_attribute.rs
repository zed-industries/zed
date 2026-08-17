#[derive(derive_more::Eq)]
struct Foo(#[eq(unknown)] i32);

impl PartialEq for Foo {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(derive_more::Eq)]
enum Enum {
    Bar { #[eq(unknown)] i: i32 },
}

impl PartialEq for Enum {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

fn main() {}
