#[cxx::bridge]
mod ffi {
    #[repr(u32)]
    enum Bad1 {
        A = 0xFFFF_FFFF_FFFF_FFFF,
    }
    enum Bad2 {
        A = 2000,
        B = 1u8,
    }
}

fn main() {}
