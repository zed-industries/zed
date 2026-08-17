use std::ptr::NonNull;

struct Opaque;

#[repr(C)]
pub struct Foo<T> {
    a: NonNull<f32>,
    b: NonNull<T>,
    c: NonNull<Opaque>,
    d: NonNull<NonNull<T>>,
    e: NonNull<NonNull<f32>>,
    f: NonNull<NonNull<Opaque>>,
    g: Option<NonNull<T>>,
    h: Option<NonNull<i32>>,
    i: Option<NonNull<NonNull<i32>>>,
}

#[no_mangle]
pub extern "C" fn root(arg: NonNull<i32>, foo: *mut Foo<u64>, d: NonNull<NonNull<Opaque>>) { }
