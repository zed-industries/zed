#[derive(derive_more::Display, derive_more::UpperHex)]
enum Enum {
    UnitVariant,
}

#[derive(derive_more::Display, derive_more::UpperHex)]
#[upper_hex("default with {_variant}")]
enum EnumWithVariantInFmt {
    UnitVariant,
}

fn main() {}
