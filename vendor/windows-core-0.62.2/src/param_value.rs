use super::*;
use core::mem::transmute_copy;

#[doc(hidden)]
pub enum ParamValue<T: Type<T>> {
    Owned(T),
    Borrowed(T::Abi),
}

impl<T: Type<T>> ParamValue<T> {
    // TODO: replace with `borrow` in windows-bindgen
    pub fn abi(&self) -> T::Abi {
        unsafe {
            match self {
                Self::Owned(item) => transmute_copy(item),
                Self::Borrowed(borrowed) => transmute_copy(borrowed),
            }
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        unsafe { transmute_copy(&self.abi()) }
    }
}
