#[repr(C)]
pub struct ArrayVec<T, const CAP: usize> {
    // the `len` first elements of the array are initialized
    xs: [T; CAP],
    len: u32,
}

#[no_mangle]
pub unsafe extern "C" fn push(v: *mut ArrayVec<*mut u8, 100>, elem: *mut u8) -> i32 {
    if (*v).len < 100 {
        (*v).xs[(*v).len] = elem;
        (*v).len += 1;
        1
    } else {
        0
    }
}
