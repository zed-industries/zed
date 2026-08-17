#[derive(derive_more::FromStr)]
#[from_str(rename_all = "Whatever")]
enum Enum {
    UnitVariant,
}

fn main() {}
