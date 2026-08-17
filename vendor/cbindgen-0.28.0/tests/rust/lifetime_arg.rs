#[repr(C)]
struct A<'a> {
    data: &'a i32
}

#[repr(C)]
enum E<'a> {
    V,
    U(&'a u8),
}

#[no_mangle]
pub extern "C" fn root<'a>(_a: A<'a>, _e: E<'a>)
{ }
