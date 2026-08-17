use super::*;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::transmute;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, PhantomData<&'a T>);

impl<'a, T: Type<T, Default = Option<T>, Abi = *mut c_void>> Ref<'a, T> {
    /// Converts the argument to a [Result<&T>] reference.
    pub fn ok(&self) -> Result<&T> {
        if self.0.is_null() {
            Err(Error::from_hresult(imp::E_POINTER))
        } else {
            unsafe { Ok(transmute::<&*mut c_void, &T>(&self.0)) }
        }
    }
}

impl<'a, T: Type<T>> core::ops::Deref for Ref<'a, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}
