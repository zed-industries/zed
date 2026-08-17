#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type TypeR;
    }
}

struct TypeR(str);

fn main() {}
