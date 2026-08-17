pub const CONSTANT_I64: i64 = 216;
pub const CONSTANT_FLOAT32: f32 = 312.292;
pub const DELIMITER: char = ':';
pub const LEFTCURLY: char = '{';
#[repr(C)]
struct Foo {
    x: i32,
}

pub const SomeFoo: Foo = Foo { x: 99, };

impl Foo {
    pub const CONSTANT_I64_BODY: i64 = 216;
}
