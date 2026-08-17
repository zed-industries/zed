#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type C;

        fn get_neither_const_nor_mut() -> *C;
    }
}

fn main() {}
