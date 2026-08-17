#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn foo(x: CxxString);
        fn bar(x: &cxx::CxxString);
    }
}

fn foo(_: &cxx::CxxString) {
    todo!()
}

fn bar(_: &cxx::CxxString) {
    todo!()
}

fn main() {}
