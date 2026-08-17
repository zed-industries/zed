#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Complex<'a, 'b: 'a>;
    }
}

fn main() {}
