#![allow(unexpected_cfgs)]

#[cxx::bridge]
mod ffi {
    struct Empty {}
}

#[cxx::bridge]
mod ffi2 {
    struct ConditionallyEmpty {
        #[cfg(target_os = "nonexistent")]
        never: u8,
        #[cfg(target_os = "another")]
        another: u8,
    }
}

fn main() {}
