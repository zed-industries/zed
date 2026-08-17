#[no_mangle]
static FIRST: u32 = 10;

#[export_name = "RENAMED"]
static SECOND: u32 = 42;

#[no_mangle]
extern "C" fn first()
{ }

#[export_name = "renamed"]
extern fn second()
{ }
