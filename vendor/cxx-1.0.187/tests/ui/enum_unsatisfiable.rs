#[cxx::bridge]
mod ffi {
    enum Bad {
        A = -0xFFFF_FFFF_FFFF_FFFF,
        B = 0xFFFF_FFFF_FFFF_FFFF,
    }
}

fn main() {}
