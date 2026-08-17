#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn f(callback: fn() -> Result<()>);
    }
}

fn main() {}
