#[derive(derive_more::TryInto)]
#[try_into(foo)]
enum MixedData {
    Int(u32),
    String(String),
}

fn main() {}
