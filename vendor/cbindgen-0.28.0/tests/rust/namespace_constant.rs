pub const FOO: i32 = 10;
pub const BAR: &'static str = "hello world";
pub const ZOM: f32 = 3.14;

#[repr(C)]
struct Foo {
    x: [i32; FOO],
}

#[no_mangle]
pub extern "C" fn root(x: Foo) { }
