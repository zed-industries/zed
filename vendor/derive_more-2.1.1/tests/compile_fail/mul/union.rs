#[derive(derive_more::Mul)]
union IntOrFloat {
    i: u32,
}

#[derive(derive_more::Mul)]
#[mul(forward)]
union Float {
    f: f32,
}

fn main() {}
