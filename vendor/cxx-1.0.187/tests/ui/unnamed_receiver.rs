#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type One;
        type Two;
        fn f(&mut self);
    }

    extern "Rust" {
        fn f(self: &Self);
    }
}

fn main() {}
