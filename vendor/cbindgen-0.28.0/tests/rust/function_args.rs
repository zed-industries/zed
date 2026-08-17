#[no_mangle]
pub unsafe extern fn array_print(a: &[u64]) {
    eprintln!("{:?}", a);
}

#[no_mangle]
pub unsafe extern fn array_test(a: [u64; 3]) {
    array_print(&a);
}

#[no_mangle]
pub unsafe extern fn unnamed(_: *const u64) {
}

#[no_mangle]
pub unsafe extern fn pointer_test(a: *const u64) {
    let a = std::slice::from_raw_parts(a, 3);
    array_print(a);
}

#[no_mangle]
pub unsafe extern fn print_from_rust() {
    let a = [0, 1, 2];
    array_print(&a);
}
