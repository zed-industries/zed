#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn f(callback: fn(p: *const u8));
    }
}

fn main() {}
