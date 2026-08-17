enum Opaque {
    Foo(i32),
    Bar,
}

#[repr(u64)]
enum A {
    a1 = 0,
    a2 = 2,
    a3,
    a4 = 5,
}

#[repr(u32)]
enum B {
    b1 = 0,
    b2 = 2,
    b3,
    b4 = 5,
}

#[repr(u16)]
enum C {
    c1 = 0,
    c2 = 2,
    c3,
    c4 = 5,
}

#[repr(u8)]
enum D {
    d1 = 0,
    d2 = 2,
    d3,
    d4 = 5,
}

#[repr(usize)]
enum E {
    e1 = 0,
    e2 = 2,
    e3,
    e4 = 5,
}

#[repr(isize)]
enum F {
    f1 = 0,
    f2 = 2,
    f3,
    f4 = 5,
}

#[repr(u8)]
enum G {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

/// cbindgen:prefix-with-name
#[repr(C)]
enum H {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

/// cbindgen:prefix-with-name
#[repr(C, u8)]
enum I {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

#[repr(C, u8, u16)]
enum J {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

#[repr(C, u8, unknown_hint)]
enum K {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

#[repr(C)]
enum L {
    l1,
    l2,
    l3,
    l4,
}

#[repr(i8)]
enum M {
    m1 = -1,
    m2 = 0,
    m3 = 1,
}

/// cbindgen:enum-class=false
#[repr(C)]
enum N {
    n1,
    n2,
    n3,
    n4,
}

/// cbindgen:enum-class=false
#[repr(i8)]
enum O {
    o1,
    o2,
    o3,
    o4,
}

#[repr(C, u8)]
enum P {
    P0(u8),
    P1(u8, u8, u8),
}

#[repr(C)]
enum Q {
    Ok(Box<u32>),
    Err(u32),
}

/// cbindgen:rename-variant-name-fields=None
#[repr(C)]
enum R {
    IRFoo(i16),
    IRBar { x: u8, y: i16 },
    IRBaz,
}

#[no_mangle]
pub extern "C" fn root(
    opaque: *mut Opaque,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O,
    p: P,
    q: Q,
    r: R,
) {
}
