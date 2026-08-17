#[cxx::bridge]
mod ffi {
    enum A {
        Field(u64),
    }
}

fn main() {}
