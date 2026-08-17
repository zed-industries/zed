#[repr(transparent)]
pub struct Foo<T, P = c_void> {
    field: T,
    _phantom: std::marker::PhantomData<P>,
}

#[repr(C)]
pub struct Bar<T, P> {
    f: Foo<T>,
    p: P,
}

pub type Baz<T> = Foo<T>;

#[no_mangle]
pub extern "C" fn foo_root(f: Foo<i16>, b: Bar<i32, u32>, z: Baz<i64>) {}
