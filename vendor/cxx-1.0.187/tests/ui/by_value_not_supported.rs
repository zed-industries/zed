#[cxx::bridge]
mod ffi {
    struct S {
        c: C,
        r: R,
        s: CxxString,
    }

    extern "C++" {
        type C;
    }

    extern "Rust" {
        type R;

        fn f(c: C) -> C;
        fn g(r: R) -> R;
        fn h(s: CxxString) -> CxxString;
    }
}

fn main() {}
