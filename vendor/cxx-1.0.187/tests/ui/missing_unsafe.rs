#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn f(x: i32);
    }
}

unsafe fn f(_x: i32) {}

fn main() {}
