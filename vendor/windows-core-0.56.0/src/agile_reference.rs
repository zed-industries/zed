use super::*;
use std::marker::PhantomData;

/// A type representing an agile reference to a COM/WinRT object.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(imp::IAgileReference, PhantomData<T>);

impl<T: Interface> AgileReference<T> {
    /// Creates an agile reference to the object.
    pub fn new(object: &T) -> Result<Self> {
        // TODO: this assert is required until we can catch this at compile time using an "associated const equality" constraint.
        // For example, <T: Interface<UNKNOWN = true>>
        // https://github.com/rust-lang/rust/issues/92827
        assert!(T::UNKNOWN);
        unsafe { imp::RoGetAgileReference(imp::AGILEREFERENCE_DEFAULT, &T::IID, std::mem::transmute::<&T, &IUnknown>(object)).map(|reference| Self(reference, Default::default())) }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
        unsafe { self.0.Resolve() }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}

impl<T> std::fmt::Debug for AgileReference<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgileReference({:?})", &self.0)
    }
}
