#[repr(C)]
struct Foo {
    a: i32,
    b: u32,
    bar: Bar,
}

#[repr(C)]
struct Bar {
    a: i32,
}

pub const VAL: Foo = Foo {
    a: 42,
    b: 1337,
    bar: Bar { a: 323 },
};

#[no_mangle]
pub extern "C" fn root(x: Foo) {}
