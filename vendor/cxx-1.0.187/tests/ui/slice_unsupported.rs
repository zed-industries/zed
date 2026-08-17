#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Opaque;

        fn f(_: &mut [Opaque]);
    }
}

fn main() {}
