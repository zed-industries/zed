use super::*;

/// Provides low-level access to an interface vtable.
///
/// This trait is automatically implemented by the generated bindings and should not be
/// implemented manually.
///
/// # Safety
pub unsafe trait Interface: Sized + Clone {
    #[doc(hidden)]
    type Vtable;

    /// The `GUID` associated with the interface.
    const IID: GUID;

    #[doc(hidden)]
    const UNKNOWN: bool = true;

    /// A reference to the interface's vtable
    #[doc(hidden)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Cast this interface as a reference to the supplied interfaces `Vtable`
    ///
    /// # Safety
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of `Self::Vtable`.
    #[doc(hidden)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        &**(self.as_raw() as *mut *mut T::Vtable)
    }

    /// Returns the raw COM interface pointer. The resulting pointer continues to be owned by the `Interface` implementation.
    #[inline(always)]
    fn as_raw(&self) -> *mut std::ffi::c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Returns the raw COM interface pointer and releases ownership. It the caller's responsibility to release the COM interface pointer.
    fn into_raw(self) -> *mut std::ffi::c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }

    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    unsafe fn from_raw(raw: *mut std::ffi::c_void) -> Self {
        std::mem::transmute_copy(&raw)
    }

    /// Creates an `Interface` that is valid so long as the `raw` COM interface pointer is valid.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be a valid COM interface pointer. In other words, it must point to a vtable
    /// beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    unsafe fn from_raw_borrowed(raw: &*mut std::ffi::c_void) -> Option<&Self> {
        if raw.is_null() {
            None
        } else {
            Some(std::mem::transmute_copy(&raw))
        }
    }

    /// Attempts to cast the current interface to another interface using `QueryInterface`.
    ///
    /// The name `cast` is preferred to `query` because there is a WinRT method named query but not one
    /// named cast.
    fn cast<T: Interface>(&self) -> Result<T> {
        let mut result = None;

        // SAFETY: `result` is valid for writing an interface pointer and it is safe
        // to cast the `result` pointer as `T` on success because we are using the `IID` tied
        // to `T` which the implementor of `Interface` has guaranteed is correct
        unsafe { _ = self.query(&T::IID, &mut result as *mut _ as _) };

        result.ok_or_else(|| Error::from_hresult(imp::E_NOINTERFACE))
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<imp::IWeakReferenceSource>().and_then(|source| Weak::downgrade(&source))
    }

    /// Call `QueryInterface` on this interface
    ///
    /// # Safety
    ///
    /// `interface` must be a non-null, valid pointer for writing an interface pointer.
    unsafe fn query(&self, iid: *const GUID, interface: *mut *mut std::ffi::c_void) -> HRESULT {
        if Self::UNKNOWN {
            (self.assume_vtable::<IUnknown>().QueryInterface)(self.as_raw(), iid, interface)
        } else {
            panic!("Non-COM interfaces cannot be queried.")
        }
    }
}

/// # Safety
#[doc(hidden)]
pub unsafe fn from_raw_borrowed<T: Interface>(raw: &*mut std::ffi::c_void) -> Option<&T> {
    T::from_raw_borrowed(raw)
}
