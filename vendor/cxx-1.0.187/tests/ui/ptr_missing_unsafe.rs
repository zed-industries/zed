#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type C;

        fn not_unsafe_ptr(c: *mut C);
    }
}

fn main() {}
