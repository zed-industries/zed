#[cxx::bridge]
mod ffi {
    #[repr(align(3))]
    struct SharedA {
        b: [u8; 4],
    }

    // 1073741824 = 2^30
    #[repr(align(1073741824))]
    struct SharedB {
        b: [u8; 4],
    }

    #[repr(align(-2))]
    struct SharedC {
        b: [u8; 4],
    }
}

fn main() {}
