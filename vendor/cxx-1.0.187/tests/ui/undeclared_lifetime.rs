#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn f0(_: &'a CxxString);
        fn g0<'a>(_: &'b CxxString);

        type This<'a>;
        fn f1(self: &This, _: &'a CxxString);
        fn g1<'a>(self: &This, _: &'b CxxString);
        fn f2(self: &'a This);
        fn g2<'a>(self: &'b This);
        fn f3(self: &This<'a>);
        fn g3<'a>(self: &This<'b>);
    }
}

fn main() {}
