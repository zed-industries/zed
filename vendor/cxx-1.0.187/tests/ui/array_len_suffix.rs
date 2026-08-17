#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn array() -> [String; 12u16];
    }
}

fn main() {}
