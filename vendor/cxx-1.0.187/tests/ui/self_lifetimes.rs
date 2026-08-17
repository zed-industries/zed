#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Thing<'a, 'b>;

        fn zero(self: &Thing<>);
        fn one<'a>(self: &Thing<'a>);
        fn three<'a, 'b, 'c>(self: &Thing<'a, 'b, 'c>);
    }
}

fn main() {}
