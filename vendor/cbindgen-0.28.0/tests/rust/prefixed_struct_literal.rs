#[repr(C)]
struct Foo {
    a: i32,
    b: u32,
}

impl Foo {
    pub const FOO: Foo = Foo{ a: 42, b: 47, };
}

pub const BAR: Foo = Foo{ a: 42, b: 1337, };

#[no_mangle]
pub extern "C" fn root(x: Foo) { }
