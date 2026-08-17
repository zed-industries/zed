#[cxx::bridge]
mod ffi {
    #[repr(align(2int))]
    struct StructSuffix {
        i: i32,
    }
}

fn main() {}
