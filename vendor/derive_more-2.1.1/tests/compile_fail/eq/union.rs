#[derive(derive_more::Eq)]
union IntOrFloat {
    i: u32,
}

impl PartialEq for IntOrFloat {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

fn main() {}
