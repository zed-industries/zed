#[repr(C)]
pub struct Foo<T> {
    something: *const i32,
    phantom: std::marker::PhantomData<T>,
}

#[repr(u8)]
pub enum Bar {
    Min(Foo<Self>),
    Max(Foo<Self>),
    Other,
}

#[no_mangle]
pub extern "C" fn root(b: Bar) {}
