use crate::runtime::vm::VMStore;
use crate::{StoreContextMut, store::StoreId};
use core::marker::PhantomData;

/// Represents a proof that a store with a given `StoreId` has a data type
/// parameter `T`.
///
/// This may be used to safely convert a `&mut dyn VMStore` into a
/// `StoreContextMut<T>` using `StoreToken::as_context_mut` having witnessed
/// that the type parameter matches what was seen earlier in `StoreToken::new`.
pub struct StoreToken<T> {
    id: StoreId,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for StoreToken<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for StoreToken<T> {}

impl<T: 'static> StoreToken<T> {
    /// Create a new `StoreToken`, witnessing that this store has data type
    /// parameter `T`.
    pub fn new(store: StoreContextMut<T>) -> Self {
        Self {
            id: store.0.id(),
            _phantom: PhantomData,
        }
    }

    /// Convert the specified `&mut dyn VMStore` into a `StoreContextMut<T>`.
    ///
    /// This will panic if passed a store with a different `StoreId` than the
    /// one passed to `StoreToken::new` when creating this object.
    pub fn as_context_mut<'a>(&self, store: &'a mut dyn VMStore) -> StoreContextMut<'a, T> {
        assert_eq!(store.store_opaque().id(), self.id);
        // SAFETY: We know the store with this ID has data type parameter `T`
        // because we witnessed that in `Self::new`, which is the only way
        // `self` could have been safely created:
        unsafe { store.unchecked_context_mut::<T>() }
    }
}
