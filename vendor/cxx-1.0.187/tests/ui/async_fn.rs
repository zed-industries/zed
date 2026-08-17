#[cxx::bridge]
mod ffi {
    extern "Rust" {
        async fn f();
    }

    extern "C++" {
        async fn g();
    }
}

async fn f() {}

fn main() {}
