#[derive(derive_more::Eq)]
#[eq(unknown)]
struct Foo(i32);

impl PartialEq for Foo {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

fn main() {}
