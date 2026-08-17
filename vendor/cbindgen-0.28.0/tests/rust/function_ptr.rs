pub type MyCallback = Option<unsafe extern "C" fn(a: usize, b: usize)>;

pub type MyOtherCallback = Option<unsafe extern "C" fn(a: usize, lot: usize, of: usize, args: usize, and_then_some: usize)>;

#[no_mangle]
pub extern "C" fn my_function(a: MyCallback, b: MyOtherCallback) {}
