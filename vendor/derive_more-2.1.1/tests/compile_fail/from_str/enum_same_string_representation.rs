#![allow(non_camel_case_types)]

#[derive(derive_more::FromStr)]
enum Enum {
    abc,
    #[from_str(rename_all = "lowercase")]
    AB_C,
}

#[derive(derive_more::FromStr)]
#[from_str(rename_all = "SCREAMING_SNAKE_CASE")]
enum EnumTop {
    BaR,
    BA_R,
}

fn main() {}
