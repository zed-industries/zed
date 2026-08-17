use super::*;
use core::marker::PhantomData;

/// `Weak` holds a non-owning reference to an object.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Weak<I: Interface>(Option<imp::IWeakReference>, PhantomData<I>);

impl<I: Interface> Weak<I> {
    /// Creates a new `Weak` object without any backing object.
    pub const fn new() -> Self {
        Self(None, PhantomData)
    }

    /// Attempts to upgrade the weak reference to a strong reference.
    pub fn upgrade(&self) -> Option<I> {
        self.0
            .as_ref()
            .and_then(|inner| unsafe { inner.Resolve().ok() })
    }

    pub(crate) fn downgrade(source: &imp::IWeakReferenceSource) -> Result<Self> {
        let reference = unsafe { source.GetWeakReference().ok() };
        Ok(Self(reference, PhantomData))
    }
}

unsafe impl<I: Interface> Send for Weak<I> {}
unsafe impl<I: Interface> Sync for Weak<I> {}
