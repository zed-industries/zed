#[cxx::bridge]
mod ffi {
    struct S {
        x: u8,
    }

    impl fn() -> &S {}
}

fn main() {}
