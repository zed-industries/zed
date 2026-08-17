#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn f(self: &Unrecognized);
    }
}

fn main() {}
