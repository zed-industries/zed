#[cxx::bridge]
mod ffi {
    #[namespace = "::"]
    extern "Rust" {}

    #[namespace = ""]
    extern "Rust" {}

    #[namespace = ]
    extern "Rust" {}
}

fn main() {}
