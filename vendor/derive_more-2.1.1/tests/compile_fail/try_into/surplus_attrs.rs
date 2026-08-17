#[derive(derive_more::TryInto)]
#[try_into(owned)]
#[try_into(foo)]
enum MixedData {
    Int(u32),
    String(String),
}

fn main() {}
