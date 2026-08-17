#[repr(transparent)]
pub struct Foo(NonZeroU64);

#[repr(C)]
pub struct Bar {
  foo: Option<Foo>,
}

#[no_mangle]
pub extern "C" fn root(f: Bar) {}
