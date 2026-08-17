#[repr(C)]
struct A {
    namespace: i32,
    float: f32,
}

/// cbindgen:field-names=[namespace, float]
#[repr(C)]
struct B(i32, f32);

#[repr(C, u8)]
enum C {
    D { namespace: i32, float: f32 },
}

#[repr(C, u8)]
enum E {
    Double(f64),
    Float(f32),
}

#[repr(C, u8)]
enum F {
    double(f64),
    float(f32),
}

#[no_mangle]
pub extern "C" fn root(
    a: A,
    b: B,
    c: C,
    e: E,
    f: F,
    namespace: i32,
    float: f32,
) { }
