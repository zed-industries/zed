#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Borrowed<'a>;
        fn borrowed() -> UniquePtr<Borrowed>;
    }
}

fn main() {}
