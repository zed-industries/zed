#[no_mangle]
pub extern "C" fn root(
	a: &Vec<String>,
    b: &Option<i32>,
    c: &Result<i32, String>
) { }
