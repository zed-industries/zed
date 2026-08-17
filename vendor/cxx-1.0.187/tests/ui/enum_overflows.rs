#[cxx::bridge]
mod ffi {
    enum Good1 {
        A = 0xFFFF_FFFF_FFFF_FFFF,
    }
    enum Good2 {
        B = 0xFFFF_FFFF_FFFF_FFFF,
        C = 2020,
    }
    enum Bad {
        D = 0xFFFF_FFFF_FFFF_FFFE,
        E,
        F,
    }
}

fn main() {}
