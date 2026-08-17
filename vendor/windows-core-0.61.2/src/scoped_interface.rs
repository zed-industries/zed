use super::*;
use core::ffi::c_void;
use core::marker::PhantomData;

#[doc(hidden)]
#[repr(C)]
pub struct ScopedHeap {
    pub vtable: *const c_void,
    pub this: *const c_void,
}

#[doc(hidden)]
pub struct ScopedInterface<'a, T: Interface> {
    interface: T,
    lifetime: PhantomData<&'a T>,
}

impl<T: Interface> ScopedInterface<'_, T> {
    pub fn new(interface: T) -> Self {
        Self {
            interface,
            lifetime: PhantomData,
        }
    }
}

impl<T: Interface> core::ops::Deref for ScopedInterface<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.interface
    }
}

impl<T: Interface> Drop for ScopedInterface<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.interface.as_raw() as *const _ as *mut ScopedHeap);
        }
    }
}
