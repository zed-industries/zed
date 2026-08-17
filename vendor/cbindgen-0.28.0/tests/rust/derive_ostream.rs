/// cbindgen:derive-ostream
#[repr(C)]
pub struct A(i32);

/// cbindgen:field-names=[x, y]
/// cbindgen:derive-ostream
#[repr(C)]
pub struct B(i32, f32);

/// cbindgen:derive-ostream
#[repr(u32)]
pub enum C {
    X = 2,
    Y,
}

/// cbindgen:derive-ostream
#[repr(C)]
pub struct D {
    List: u8,
    Of: usize,
    Things: B,
}

/// cbindgen:derive-ostream
#[repr(u8)]
pub enum F {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

/// cbindgen:derive-ostream
#[repr(C, u8)]
pub enum H {
    Hello(i16),
    There { x: u8, y: i16 },
    Everyone
}

/// cbindgen:derive-ostream=false
#[repr(C, u8)]
pub enum I {
    /// cbindgen:derive-ostream=true
    ThereAgain { x: u8, y: i16 },
    SomethingElse
}

#[no_mangle]
pub extern "C" fn root(
    a: A,
    b: B,
    c: C,
    d: D,
    f: F,
    h: H,
    i: I,
) { }

