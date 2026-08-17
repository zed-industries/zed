#[cxx::bridge]
mod ffi {
    enum Bad {
        A = 1u16,
        B = 2i64,
    }
}

fn main() {}
