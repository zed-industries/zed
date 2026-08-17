#[cxx::bridge]
mod ffi {
    enum A {
        Field = 2020 + 1,
    }
}

fn main() {}
