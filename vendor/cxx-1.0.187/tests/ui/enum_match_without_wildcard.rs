#[cxx::bridge]
mod ffi {
    enum A {
        FieldA,
        FieldB,
    }
}

fn main() {}

fn matcher(a: ffi::A) -> u32 {
    match a {
        ffi::A::FieldA => 2020,
        ffi::A::FieldB => 2021,
    }
}
