#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Mut<'a>;
    }

    unsafe extern "C++" {
        type Thing;

        fn f(t: &Thing) -> Pin<&mut CxxString>;
        unsafe fn g(t: &Thing) -> Pin<&mut CxxString>;
        fn h(t: Box<Mut>) -> Pin<&mut CxxString>;
        fn i<'a>(t: Box<Mut<'a>>) -> Pin<&'a mut CxxString>;
        fn j(t: &Thing) -> &mut [u8];
    }
}

fn main() {}
