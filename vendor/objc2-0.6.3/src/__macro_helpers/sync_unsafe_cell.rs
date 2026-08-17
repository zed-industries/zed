use core::cell::UnsafeCell;

#[repr(transparent)]
#[derive(Debug)]
pub struct SyncUnsafeCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

// SAFETY: This type is used for making `static`s which contain `UnsafeCell`.
// The user upholds that such usage is correct.
unsafe impl<T: ?Sized + Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: ?Sized> SyncUnsafeCell<T> {
    #[inline]
    pub const fn get(&self) -> *mut T {
        self.value.get()
    }
}
