#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Generic<T>;
    }
}

fn main() {}
