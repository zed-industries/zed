#![deny(deprecated)]

#[cxx::bridge]
pub mod ffi {
    struct StructX {
        a: u64,
    }

    #[namespace = "mine"]
    unsafe extern "C++" {
        type StructX;
    }
}

fn main() {}
