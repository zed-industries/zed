use super::*;

#[doc(hidden)]
#[repr(C)]
pub struct ScopedHeap {
    pub vtable: *const std::ffi::c_void,
    pub this: *const std::ffi::c_void,
}

#[doc(hidden)]
pub struct ScopedInterface<'a, T: Interface> {
    interface: T,
    lifetime: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Interface> ScopedInterface<'a, T> {
    pub fn new(interface: T) -> Self {
        Self { interface, lifetime: std::marker::PhantomData }
    }
}

impl<'a, T: Interface> std::ops::Deref for ScopedInterface<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.interface
    }
}

impl<'a, T: Interface> Drop for ScopedInterface<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.interface.as_raw() as *const _ as *mut ScopedHeap);
        }
    }
}
