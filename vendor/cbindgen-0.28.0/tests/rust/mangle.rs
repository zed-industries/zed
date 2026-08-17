#[repr(C)]
pub struct Foo<T> {
    a: T,
}

pub type Boo = Foo<u8>;

/// cbindgen:prefix-with-name=true
#[repr(C)]
pub enum Bar {
    Some,
    Thing,
}

#[no_mangle]
pub extern "C" fn root(
    x: Boo,
    y: Bar,
) { }

#[unsafe(no_mangle)]
pub extern "C" fn unsafe_root(
    x: Boo,
    y: Bar,
) { }
