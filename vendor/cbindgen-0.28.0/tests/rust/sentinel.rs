#[repr(u8)]
pub enum A {
    A1,
    A2,
    A3,
}

#[repr(u8)]
pub enum B {
    B1,
    B2,
    B3,
}

#[repr(u8)]
pub enum C {
    C1 { a: u32 },
    C2 { b: u32 },
    C3,
}

#[no_mangle]
pub extern "C" fn root(a: A, b: B, c: C) {}
