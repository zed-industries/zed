#[cxx::bridge]
mod ffi {
    enum Enum {
        Variant,
    }
    extern "Rust" {
        fn f(self: &Enum);
    }
}

fn main() {}
