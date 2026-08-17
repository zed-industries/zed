use arg_enum_proc_macro::ArgEnum;

pub enum Foo {
    Bar,
    /// Foo
    Baz,
}

#[derive(ArgEnum)]
pub enum Complex {
    A,
    B(Foo),
    C { a: usize, b: usize },
}

fn main() {}
