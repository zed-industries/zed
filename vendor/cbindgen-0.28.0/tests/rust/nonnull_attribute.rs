use std::ptr::NonNull;

struct Opaque;

#[repr(C)]
pub struct Pointers<T> {
    a: NonNull<f32>,
    b: NonNull<T>,
    c: NonNull<Opaque>,
    d: NonNull<NonNull<T>>,
    e: NonNull<NonNull<f32>>,
    f: NonNull<NonNull<Opaque>>,
    g: Option<NonNull<T>>,
    h: Option<NonNull<i32>>,
    i: Option<NonNull<NonNull<i32>>>,
    j: *const T,
    k: *mut T,
}

#[repr(C)]
pub struct References<'a> {
    a: &'a Opaque,
    b: &'a mut Opaque,
    c: Option<&'a Opaque>,
    d: Option<&'a mut Opaque>,
}

#[no_mangle]
pub extern "C" fn value_arg(arg: References<'static>) {}

#[no_mangle]
pub extern "C" fn mutltiple_args(
    arg: NonNull<i32>,
    foo: *mut Pointers<u64>,
    d: NonNull<NonNull<Opaque>>,
) {
}

#[no_mangle]
pub extern "C" fn ref_arg(arg: &Pointers<u64>) {}

#[no_mangle]
pub extern "C" fn mut_ref_arg(arg: &mut Pointers<u64>) {}

#[no_mangle]
pub extern "C" fn optional_ref_arg(arg: Option<&Pointers<u64>>) {}

#[no_mangle]
pub extern "C" fn optional_mut_ref_arg(arg: Option<&mut Pointers<u64>>) {}

#[no_mangle]
pub extern "C" fn nullable_const_ptr(arg: *const Pointers<u64>) {}

#[no_mangle]
pub extern "C" fn nullable_mut_ptr(arg: *mut Pointers<u64>) {}
