use std::marker::PhantomData;

union Opaque {
    x: i32,
    y: f32,
}

#[repr(C)]
union Normal {
    x: i32,
    y: f32,
}

#[repr(C)]
union NormalWithZST {
    x: i32,
    y: f32,
    z: (),
    w: PhantomData<i32>,
}

#[no_mangle]
pub extern "C" fn root(
    a: *mut Opaque,
    b: Normal,
    c: NormalWithZST
) { }
