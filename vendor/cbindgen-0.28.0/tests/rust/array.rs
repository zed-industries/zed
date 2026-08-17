#[repr(C)]
enum Foo {
    A([f32; 20])
}

#[no_mangle]
pub extern "C" fn root(a: Foo) {}
