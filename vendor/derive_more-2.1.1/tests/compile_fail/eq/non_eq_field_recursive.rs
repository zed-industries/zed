#[derive(derive_more::Eq)]
struct IntOrFloat {
    a: Box<NotEq<Self>>,
    b: Box<NotEq<IntOrFloat>>,
}

impl PartialEq for IntOrFloat {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(PartialEq)]
struct NotEq<T>(f32, T);

fn main() {}
