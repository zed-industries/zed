#[no_mangle]
pub extern "C" fn c() {}

#[no_mangle]
pub extern "C-unwind" fn c_unwind() {}
