#[repr(C)]
struct A {
    x: i32,
    y: f32,
}

#[repr(C)]
struct B {
    data: A,
}
