/// cbindgen:ptrs-as-arrays=[[arg;3]]
#[no_mangle]
pub unsafe extern "C" fn ptr_as_array(n: u32, arg: *const u32, v: *const u64) {}

/// cbindgen:ptrs-as-arrays=[[arg;3], [v; 4]]
#[no_mangle]
pub unsafe extern "C" fn ptr_as_array1(n: u32, arg: *const u32, v: *mut u64) {}

/// cbindgen:ptrs-as-arrays=[[n;2], [arg; ], [v;], [k; 3]]
#[no_mangle]
pub unsafe extern "C" fn ptr_as_array2(n: u32, arg: *mut u32, v: *const u64) {}

/// cbindgen:ptrs-as-arrays=[[a;2;3]]
#[no_mangle]
pub unsafe extern "C" fn ptr_as_array_wrong_syntax(arg: *mut u32, v: *const u32, _: *const u32) {}

/// cbindgen:ptrs-as-arrays=[[_;2], [_;3]]
#[no_mangle]
pub unsafe extern "C" fn ptr_as_array_unnamed(_: *mut u32, _: *const u32) {}
