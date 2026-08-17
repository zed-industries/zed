#[repr(C)]
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

#[no_mangle]
pub extern "C" fn root(ptr: *const (), t: TraitObject) -> *mut () {
    std::ptr::null_mut()
}
