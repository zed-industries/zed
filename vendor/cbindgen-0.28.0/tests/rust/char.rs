#[repr(C)]
struct Foo {
    a: char,
}

#[no_mangle]
pub extern "C" fn root(a: Foo) {}
