extern crate proc_macro_error;
use proc_macro_error::abort;

struct Foo;

#[allow(unused)]
fn foo() {
    abort!(Foo, "BOOM");
}

fn main() {}
