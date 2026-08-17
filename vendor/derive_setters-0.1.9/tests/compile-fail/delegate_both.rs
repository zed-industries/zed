use derive_setters::Setters;

#[derive(Default, Setters)]
#[setters(generate_delegates(ty = "Other", field = "f", method = "m"))]
struct BothFieldAndMethod {
    a: u32,
}

struct Other {
    f: BothFieldAndMethod,
}

impl Other {
    fn m(&self) -> &BothFieldAndMethod {
        &self.f
    }
}

fn main() {}
