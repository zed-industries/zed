use super::*;
use core::mem::transmute;

/// A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures.
#[repr(transparent)]
pub struct Ref<'a, T: Type<T>>(T::Abi, core::marker::PhantomData<&'a T>);

impl<T: Type<T>> Ref<'_, T> {
    /// Returns `true` if the argument is null.
    pub fn is_null(&self) -> bool {
        T::is_null(&self.0)
    }

    /// Converts the argument to a [`Result<&T>`] reference.
    pub fn ok(&self) -> Result<&T> {
        self.as_ref()
            .ok_or_else(|| Error::from_hresult(imp::E_POINTER))
    }

    /// Converts the argument to a [`Option<&T>`] reference.
    pub fn as_ref(&self) -> Option<&T> {
        if self.is_null() {
            None
        } else {
            unsafe { Some(self.assume_init_ref()) }
        }
    }

    /// Converts the argument to a `&T` reference.
    ///
    /// # Panics
    ///
    /// Panics if the argument is null.
    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.as_ref().expect("called `Ref::unwrap` on a null value")
    }

    /// Converts the argument to an [`Option<T>`] by cloning the reference.
    pub fn cloned(&self) -> Option<T> {
        self.as_ref().cloned()
    }

    unsafe fn assume_init_ref(&self) -> &T {
        unsafe { T::assume_init_ref(&self.0) }
    }
}

impl<T: Type<T>> core::ops::Deref for Ref<'_, T> {
    type Target = T::Default;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}

impl<'a, T: Type<T>> From<&'a T::Default> for Ref<'a, T> {
    fn from(from: &'a T::Default) -> Self {
        unsafe { core::mem::transmute_copy(from) }
    }
}
