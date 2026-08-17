#[repr(C)]
pub struct Foo<T> {
    something: *const i32,
    phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
pub union Bar {
    something: i32,
    subexpressions: Foo<Self>,
}

#[no_mangle]
pub extern "C" fn root(b: Bar) {}
