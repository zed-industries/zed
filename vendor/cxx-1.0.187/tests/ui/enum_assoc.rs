#[cxx::bridge]
mod ffi {
    enum Enum {
        Variant,
    }
    extern "Rust" {
        #[Self = "Enum"]
        fn f();
    }
}

impl ffi::Enum {
    fn f() {}
}

fn main() {}
