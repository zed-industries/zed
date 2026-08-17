use std::marker::PhantomPinned;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Pinned;
    }
}

pub struct Pinned {
    _pinned: PhantomPinned,
}

fn main() {}
