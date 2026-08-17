#[derive(derive_more::Eq)]
struct IntOrFloat {
    f: f32,
}

impl PartialEq for IntOrFloat {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

fn main() {}
