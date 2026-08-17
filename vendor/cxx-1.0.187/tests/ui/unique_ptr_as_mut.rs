use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {
    struct Shared {
        x: i32,
    }

    extern "C++" {
        type Opaque;
    }

    impl UniquePtr<Shared> {}
    impl UniquePtr<Opaque> {}
}

fn main() {
    let mut shared = UniquePtr::<ffi::Shared>::null();
    let _: &mut ffi::Shared = &mut shared;

    let mut opaque = UniquePtr::<ffi::Opaque>::null();
    let _: &mut ffi::Opaque = &mut opaque;
}
