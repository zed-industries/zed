#[cxx::bridge]
mod ffi {
    extern "Rust" {
        /// Incorrect.
        type Alias = crate::Type;
    }
}

fn main() {}
