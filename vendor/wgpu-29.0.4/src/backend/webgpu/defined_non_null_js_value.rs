use core::ops::{Deref, DerefMut};

use wasm_bindgen::JsValue;

/// Derefs to a [`JsValue`] that's known not to be `undefined` or `null`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinedNonNullJsValue<T>(T);

impl<T> DefinedNonNullJsValue<T>
where
    T: AsRef<JsValue>,
{
    pub fn new(value: T) -> Option<Self> {
        if value.as_ref().is_undefined() || value.as_ref().is_null() {
            None
        } else {
            Some(Self(value))
        }
    }
}

impl<T> Deref for DefinedNonNullJsValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DefinedNonNullJsValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for DefinedNonNullJsValue<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for DefinedNonNullJsValue<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
