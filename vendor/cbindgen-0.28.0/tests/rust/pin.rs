#[repr(C)]
struct PinTest {
    pinned_box: Pin<Box<i32>>,
    pinned_ref: Pin<&mut i32>
}

#[no_mangle]
pub extern "C" fn root(s: Pin<&mut i32>, p: PinTest) {}
