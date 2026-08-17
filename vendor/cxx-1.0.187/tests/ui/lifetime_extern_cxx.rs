#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Opaque;
        unsafe fn f<'a>(&'a self, arg: &str) -> &'a str;
    }
}

fn main() {}
