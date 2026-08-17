
pub const MY_CONST: u8 = 4;

#[no_mangle]
pub extern "C" fn ExternFunction() {
}

#[repr(u8)]
pub enum OnlyThisShouldBeGenerated {
    Foo,
    Bar,
}
