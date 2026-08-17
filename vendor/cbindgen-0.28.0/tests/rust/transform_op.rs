#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[repr(u8)]
pub enum Foo<T> {
    Foo { x: i32, y: Point<T>, z: Point<f32>, },
    Bar(T),
    Baz(Point<T>),
    Bazz,
}

#[repr(C)]
pub enum Bar<T> {
    Bar1 { x: i32, y: Point<T>, z: Point<f32>, u: unsafe extern "C" fn(i32) -> i32,  },
    Bar2(T),
    Bar3(Point<T>),
    Bar4,
}

#[repr(u8)]
pub enum Baz {
    Baz1(Bar<u32>),
    Baz2(Point<i32>),
    Baz3,
}

#[repr(C, u8)]
pub enum Taz {
    Taz1(Bar<u32>),
    Taz2(Baz),
    Taz3,
}

#[no_mangle]
pub extern "C" fn foo(
    foo: *const Foo<i32>,
    bar: *const Bar<i32>,
    baz: *const Baz,
    taz: *const Taz,
) {}
