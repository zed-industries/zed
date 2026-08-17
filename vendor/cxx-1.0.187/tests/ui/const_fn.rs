#[cxx::bridge]
mod ffi {
    extern "Rust" {
        const fn f();
    }
}

const fn f() {}

fn main() {}
