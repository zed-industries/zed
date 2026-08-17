#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn f() {}
    }
}

fn main() {}
