use std::marker::PhantomData;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type T;

        fn f(t: &T) -> &str;
    }
}

pub struct T<'a> {
    _lifetime: PhantomData<&'a ()>,
}

fn f<'a>(_t: &T<'a>) -> &'a str {
    ""
}

fn main() {}
