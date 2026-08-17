#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Opaque;
        fn f(arg: &mut Opaque);
        fn g(&mut self);
        fn h(self: &mut Opaque);
        fn s(s: &mut CxxString);
        fn v(v: &mut CxxVector<u8>);
    }
}

fn main() {}
