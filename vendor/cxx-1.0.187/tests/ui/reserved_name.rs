#[cxx::bridge]
mod ffi {
    struct UniquePtr {
        val: usize,
    }

    extern "C++" {
        type Box;
    }

    extern "Rust" {
        type String;
    }
}

fn main() {}
