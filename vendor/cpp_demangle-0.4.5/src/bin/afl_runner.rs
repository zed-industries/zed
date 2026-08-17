#[macro_use]
extern crate afl;
extern crate cpp_demangle;

fn main() {
    afl::fuzz!(|bytes| {
        let _ = cpp_demangle::Symbol::new(bytes);
    });
}
