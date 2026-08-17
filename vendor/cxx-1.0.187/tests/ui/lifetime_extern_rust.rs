#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Opaque;
        fn f<'a>(&'a self, arg: &str) -> &'a str;
    }
}

pub struct Opaque;

impl Opaque {
    fn f(&self, _arg: &str) -> &str {
        ""
    }
}

fn main() {}
