/// cbindgen:derive-lt=true
/// cbindgen:derive-lte=true
/// cbindgen:derive-constructor=true
/// cbindgen:rename-all=GeckoCase
#[repr(C)]
struct A(i32);

/// cbindgen:field-names=[x, y]
#[repr(C)]
struct B(i32, f32);

/// cbindgen:trailing-values=[Z, W]
#[repr(u32)]
enum C {
    X = 2,
    Y,
}

/// cbindgen:derive-helper-methods=true
#[repr(u8)]
enum F {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz
}

/// cbindgen:derive-helper-methods
#[repr(C, u8)]
enum H {
    Hello(i16),
    There { x: u8, y: i16 },
    Everyone
}

#[no_mangle]
pub extern "C" fn root(
    x: A,
    y: B,
    z: C,
    f: F,
    h: H,
) { }

