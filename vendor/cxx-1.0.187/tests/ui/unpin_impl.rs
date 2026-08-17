#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Opaque;
    }
}

impl Unpin for ffi::Opaque {}

fn main() {}
