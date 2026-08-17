use derive_setters::Setters;

#[derive(Default, Setters)]
#[setters(generate_delegates(ty = "Other"))]
struct NeitherFieldNorMethod {
    a: u32,
}

struct Other {
    f: NeitherFieldNorMethod,
}

fn main() {}
