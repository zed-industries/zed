#[no_mangle]
pub static mut MUT_GLOBAL_ARRAY: [c_char; 128] = [0; 128];

#[no_mangle]
pub static CONST_GLOBAL_ARRAY: [c_char; 128] = [0; 128];
