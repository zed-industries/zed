#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type ThingC;
        fn repro_c(t: &&ThingC);
    }
    extern "Rust" {
        type ThingR;
        fn repro_r(t: &&ThingR);
    }
}

fn main() {}
