#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn f() -> Result<()>;
    }
}

pub struct NonError;

fn f() -> Result<(), NonError> {
    Ok(())
}

fn main() {}
