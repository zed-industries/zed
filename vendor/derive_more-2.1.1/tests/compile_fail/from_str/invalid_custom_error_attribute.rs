#[derive(derive_more::FromStr)]
#[from_str(error(CustomError, CustomError::new, CustomError::from))]
pub enum Foo {
    Bar,
}

#[derive(derive_more::FromStr)]
#[from_str(error(CustomError, CustomError::from, CustomError::new))]
struct Baz;

#[derive(derive_more::FromStr)]
#[from_str(error())]
struct Empty;

fn main() {}
