
#[repr(C)]
#[must_use]
pub struct OwnedPtr<T> {
    ptr: *mut T,
}

#[repr(C, u8)]
#[must_use]
pub enum MaybeOwnedPtr<T> {
    Owned(*mut T),
    None,
}

#[no_mangle]
#[must_use]
pub extern "C" fn maybe_consume(input: OwnedPtr<i32>) -> MaybeOwnedPtr<i32> {
}
