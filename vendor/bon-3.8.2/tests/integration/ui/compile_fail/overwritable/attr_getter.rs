use bon::Builder;

#[derive(Builder)]
struct OverwritableCompat {
    #[builder(getter, overwritable)]
    x: u32,
}

fn main() {}
