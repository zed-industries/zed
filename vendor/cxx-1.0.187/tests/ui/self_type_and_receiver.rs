#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type T;

        #[Self = "T"]
        fn method(self: &T);
    }
}

fn main() {}
