use darling::FromMeta;

#[derive(FromMeta)]
struct WrongReturnType {
    #[darling(default = usize::default)]
    not_usize: String,

    #[darling(default = "usize::default")]
    also_not_usize: String,

    #[darling(default = || usize::default())]
    still_not_usize: String,
}

#[derive(FromMeta)]
struct PreMapReturnType {
    #[darling(default = f64::default, map = f64::to_bits)]
    not_f64: u64,

    #[darling(default = "f64::default", map = f64::to_bits)]
    also_not_f64: u64,

    #[darling(default = || f64::default(), map = f64::to_bits)]
    still_not_f64: u64,
}

#[derive(FromMeta)]
struct NotMultiple {
    #[darling(multiple, default = bool::default)]
    multiple: Vec<bool>,

    #[darling(multiple, default = "bool::default")]
    also_multiple: Vec<bool>,

    #[darling(multiple, default = || bool::default())]
    still_multiple: Vec<bool>,
}

#[derive(FromMeta)]
struct ExtraneousParams {
    #[darling(default = String::with_capacity)]
    path_expr: String,

    #[darling(default = "String::with_capacity")]
    path_lit: String,

    #[darling(default = |cap| String::with_capacity(cap))]
    closure: String,
}

#[derive(FromMeta)]
struct QuotedClosure {
    #[darling(default = r#"|| "world".to_owned()"#)]
    hello: String,
}

fn main() {}
