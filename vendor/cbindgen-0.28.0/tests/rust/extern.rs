#[repr(C)]
struct Normal {
    x: i32,
    y: f32,
}

extern "C" {
    fn foo() -> i32;

    fn bar(a: Normal);
}

extern {
    fn baz() -> i32;
}
