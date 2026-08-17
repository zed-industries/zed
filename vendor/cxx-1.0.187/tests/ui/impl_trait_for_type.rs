#[cxx::bridge]
mod ffi {
    struct S {
        x: u8,
    }

    impl UniquePtrTarget for S {}
}

fn main() {}
