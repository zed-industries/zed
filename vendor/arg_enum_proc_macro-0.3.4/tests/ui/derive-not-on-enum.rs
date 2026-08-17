use arg_enum_proc_macro::ArgEnum;

#[derive(ArgEnum)]
pub struct Complicated {
    pub foo: u8,
    pub bar: u8,
}

fn main() {}
