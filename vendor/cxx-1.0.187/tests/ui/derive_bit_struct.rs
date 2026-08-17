#[cxx::bridge]
mod ffi {
    #[derive(BitAnd, BitOr, BitXor)]
    struct Struct {
        x: i32,
    }
}

fn main() {}
