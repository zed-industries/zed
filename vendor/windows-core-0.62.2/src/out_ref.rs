use super::*;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
///
/// This is a mutable version of [Ref] meant to support out parameters.
#[repr(transparent)]
pub struct OutRef<'a, T: Type<T>>(*mut T::Abi, core::marker::PhantomData<&'a T>);

impl<T: Type<T>> OutRef<'_, T> {
    /// Returns `true` if the argument is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Overwrites a memory location with the given value without reading or dropping the old value.
    pub fn write(self, value: T::Default) -> Result<()> {
        if self.0.is_null() {
            Err(Error::from_hresult(imp::E_POINTER))
        } else {
            unsafe { *self.0 = core::mem::transmute_copy(&value) }
            core::mem::forget(value);
            Ok(())
        }
    }
}

impl<'a, T: Type<T>> From<&'a mut T::Default> for OutRef<'a, T> {
    fn from(from: &'a mut T::Default) -> Self {
        unsafe { core::mem::transmute(from) }
    }
}

impl<T: Type<T>> Default for OutRef<'_, T> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
