#[derive(derive_more::Eq)]
enum Enum {
    #[eq(unknown)]
    Bar { i: i32 },
}

impl PartialEq for Enum {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

fn main() {}
