#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Logger;

        fn logger<'static>() -> Pin<&'static Logger>;
    }
}

fn main() {}
