#[cxx::bridge]
mod ffi {
    #[derive(Clone, Clone)]
    struct Struct {
        x: usize,
    }
}

fn main() {}
