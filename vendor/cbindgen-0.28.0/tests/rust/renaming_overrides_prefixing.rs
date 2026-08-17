struct A {
    x: i32,
    y: f32,
}

#[repr(C)]
struct B {
    x: i32,
    y: f32,
}

#[no_mangle]
pub extern "C" fn root(a: *const A, b: B) {}
