#[repr(C)]
struct Dummy {
    x: i32,
    y: f32,
}

#[no_mangle]
pub extern "C" fn root(d: Dummy) {}
