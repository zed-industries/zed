#[cxx::bridge]
mod ffi {
    struct S {
        x: u8,
    }

    impl UniquePtr<S> {
        fn new() -> Self;
    }
}

fn main() {}
