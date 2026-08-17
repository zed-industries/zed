#[no_mangle]
pub extern "C" fn loop_forever() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn normal_return(arg: Example, other: extern "C" fn(u8) -> !) -> u8 {
    0
}

#[repr(C)]
pub struct Example {
    pub f: extern "C" fn(usize, usize) -> !,
}
