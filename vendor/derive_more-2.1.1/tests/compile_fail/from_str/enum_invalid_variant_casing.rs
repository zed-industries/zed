#[derive(derive_more::FromStr)]
enum Enum {
    #[from_str(rename_all = "Whatever")]
    UnitVariant,
}

fn main() {}
