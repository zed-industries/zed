#[cxx::bridge]
mod ffi {
    extern "C++" {
        extern "Java" fn f();
    }
}

fn main() {}
