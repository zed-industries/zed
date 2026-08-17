pub struct MyBuilder<'a> {
    _s: &'a str,
}

type OptBuilder<'a> = Option<MyBuilder<'a>>;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type OptBuilder<'a>;
    }

    struct MyBuilder<'a> {
        rs: Box<OptBuilder<'a>>,
    }
}

fn main() {}
