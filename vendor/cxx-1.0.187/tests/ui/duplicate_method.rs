#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type T;
        fn t_method(&self);
        fn t_method(&self);
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type U;
        fn u_method(&self);
        fn u_method(&mut self);
    }
}

fn main() {}
