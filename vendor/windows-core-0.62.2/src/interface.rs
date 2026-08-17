use super::*;
use core::any::Any;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{forget, transmute_copy, MaybeUninit};
use core::ptr::NonNull;

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
    #[inline(always)]
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
    #[inline(always)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        unsafe { &**(self.as_raw() as *mut *mut T::Vtable) }
    }

    /// Returns the raw COM interface pointer. The resulting pointer continues to be owned by the `Interface` implementation.
    #[inline(always)]
    fn as_raw(&self) -> *mut c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        unsafe { transmute_copy(self) }
    }

    /// Returns the raw COM interface pointer and releases ownership. It the caller's responsibility to release the COM interface pointer.
    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        // SAFETY: implementors of this trait must guarantee that the implementing type has a pointer in-memory representation
        let raw = self.as_raw();
        forget(self);
        raw
    }

    /// Creates an `Interface` by taking ownership of the `raw` COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be owned by the caller and represent a valid COM interface pointer. In other words,
    /// it must point to a vtable beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        unsafe { transmute_copy(&raw) }
    }

    /// Creates an `Interface` that is valid so long as the `raw` COM interface pointer is valid.
    ///
    /// # Safety
    ///
    /// The `raw` pointer must be a valid COM interface pointer. In other words, it must point to a vtable
    /// beginning with the `IUnknown` function pointers and match the vtable of `Interface`.
    #[inline(always)]
    unsafe fn from_raw_borrowed(raw: &*mut c_void) -> Option<&Self> {
        unsafe {
            if raw.is_null() {
                None
            } else {
                Some(transmute_copy(&raw))
            }
        }
    }

    /// Attempts to cast the current interface to another interface using `QueryInterface`.
    ///
    /// The name `cast` is preferred to `query` because there is a WinRT method named query but not one
    /// named cast.
    #[inline(always)]
    fn cast<T: Interface>(&self) -> Result<T> {
        // SAFETY: `result` is valid for writing an interface pointer and it is safe
        // to cast the `result` pointer as `T` on success because we are using the `IID` tied
        // to `T` which the implementor of `Interface` has guaranteed is correct
        unsafe {
            // If query() returns a failure code then we propagate that failure code to the caller.
            // In that case, we ignore the contents of 'result' (which will _not_ be dropped,
            // because MaybeUninit intentionally does not drop its contents).
            //
            // This guards against implementations of COM interfaces which may store non-null values
            // in 'result' but still return E_NOINTERFACE.
            let mut result = MaybeUninit::<Option<T>>::zeroed();
            self.query(&T::IID, result.as_mut_ptr() as _).ok()?;

            // If we get here, then query() has succeeded, but we still need to double-check
            // that the output pointer is non-null.
            if let Some(obj) = result.assume_init() {
                Ok(obj)
            } else {
                Err(imp::E_POINTER.into())
            }
        }
    }

    /// This casts the given COM interface to [`&dyn Any`].
    ///
    /// Applications should generally _not_ call this method directly. Instead, use the
    /// [`Interface::cast_object_ref`] or [`Interface::cast_object`] methods.
    ///
    /// `T` must be a type that has been annotated with `#[implement]`; this is checked at
    /// compile-time by the generic constraints of this method. However, note that the
    /// returned `&dyn Any` refers to the _outer_ implementation object that was generated by
    /// `#[implement]`, i.e. the `MyApp_Impl` type, not the inner `MyApp` type.
    ///
    /// If the given object is not a Rust object, or is a Rust object but not `T`, or is a Rust
    /// object that contains non-static lifetimes, then this function will return `Err(E_NOINTERFACE)`.
    ///
    /// # Safety
    ///
    /// **IMPORTANT!!**  This uses a non-standard protocol for QueryInterface!  The `DYNAMIC_CAST_IID`
    /// IID identifies this protocol, but there is no `IDynamicCast` interface. Instead, objects
    /// that recognize `DYNAMIC_CAST_IID` simply store their `&dyn Any` directly at the interface
    /// pointer that was passed to `QueryInterface. This means that the returned value has a
    /// size that is twice as large (`size_of::<&dyn Any>() == 2 * size_of::<*const c_void>()`).
    ///
    /// This means that callers that use this protocol cannot simply pass `&mut ptr` for
    /// an ordinary single-pointer-sized pointer. Only this method understands this protocol.
    ///
    /// Another part of this protocol is that the implementation of `QueryInterface` _does not_
    /// AddRef the object. The caller must guarantee the liveness of the COM object. In Rust,
    /// this means tying the lifetime of the IUnknown* that we used for the QueryInterface
    /// call to the lifetime of the returned `&dyn Any` value.
    ///
    /// This method preserves type safety and relies on these invariants:
    ///
    /// * All `QueryInterface` implementations that recognize `DYNAMIC_CAST_IID` are generated by
    ///   the `#[implement]` macro and respect the rules described here.
    #[inline(always)]
    fn cast_to_any<T>(&self) -> Result<&dyn Any>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        unsafe {
            let mut any_ref_arg: MaybeUninit<&dyn Any> = MaybeUninit::zeroed();
            self.query(
                &DYNAMIC_CAST_IID,
                any_ref_arg.as_mut_ptr() as *mut *mut c_void,
            )
            .ok()?;
            Ok(any_ref_arg.assume_init())
        }
    }

    /// Returns `true` if the given COM interface refers to an implementation of `T`.
    ///
    /// `T` must be a type that has been annotated with `#[implement]`; this is checked at
    /// compile-time by the generic constraints of this method.
    ///
    /// If the given object is not a Rust object, or is a Rust object but not `T`, or is a Rust
    /// object that contains non-static lifetimes, then this function will return `false`.
    #[inline(always)]
    fn is_object<T>(&self) -> bool
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        if let Ok(any) = self.cast_to_any::<T>() {
            any.is::<T::Outer>()
        } else {
            false
        }
    }

    /// This casts the given COM interface to [`&dyn Any`]. It returns a reference to the "outer"
    /// object, e.g. `&MyApp_Impl`, not the inner `&MyApp` object.
    ///
    /// `T` must be a type that has been annotated with `#[implement]`; this is checked at
    /// compile-time by the generic constraints of this method. However, note that the
    /// returned `&dyn Any` refers to the _outer_ implementation object that was generated by
    /// `#[implement]`, i.e. the `MyApp_Impl` type, not the inner `MyApp` type.
    ///
    /// If the given object is not a Rust object, or is a Rust object but not `T`, or is a Rust
    /// object that contains non-static lifetimes, then this function will return `Err(E_NOINTERFACE)`.
    ///
    /// The returned value is borrowed. If you need an owned (counted) reference, then use
    /// [`Interface::cast_object`].
    #[inline(always)]
    fn cast_object_ref<T>(&self) -> Result<&T::Outer>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        let any: &dyn Any = self.cast_to_any::<T>()?;
        if let Some(outer) = any.downcast_ref::<T::Outer>() {
            Ok(outer)
        } else {
            Err(imp::E_NOINTERFACE.into())
        }
    }

    /// This casts the given COM interface to [`&dyn Any`]. It returns a reference to the "outer"
    /// object, e.g. `MyApp_Impl`, not the inner `MyApp` object.
    ///
    /// `T` must be a type that has been annotated with `#[implement]`; this is checked at
    /// compile-time by the generic constraints of this method. However, note that the
    /// returned `&dyn Any` refers to the _outer_ implementation object that was generated by
    /// `#[implement]`, i.e. the `MyApp_Impl` type, not the inner `MyApp` type.
    ///
    /// If the given object is not a Rust object, or is a Rust object but not `T`, or is a Rust
    /// object that contains non-static lifetimes, then this function will return `Err(E_NOINTERFACE)`.
    ///
    /// The returned value is an owned (counted) reference; this function calls `AddRef` on the
    /// underlying COM object. If you do not need an owned reference, then you can use the
    /// [`Interface::cast_object_ref`] method instead, and avoid the cost of `AddRef` / `Release`.
    #[inline(always)]
    fn cast_object<T>(&self) -> Result<ComObject<T>>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        let object_ref = self.cast_object_ref::<T>()?;
        Ok(object_ref.to_object())
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<imp::IWeakReferenceSource>()
            .map(|source| Weak::downgrade(&source))
    }

    /// Call `QueryInterface` on this interface
    ///
    /// # Safety
    ///
    /// `interface` must be a non-null, valid pointer for writing an interface pointer.
    #[inline(always)]
    unsafe fn query(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        unsafe {
            if Self::UNKNOWN {
                (self.assume_vtable::<IUnknown>().QueryInterface)(self.as_raw(), iid, interface)
            } else {
                panic!("Non-COM interfaces cannot be queried.")
            }
        }
    }

    /// Creates an `InterfaceRef` for this reference. The `InterfaceRef` tracks lifetimes statically,
    /// and eliminates the need for dynamic reference count adjustments (AddRef/Release).
    fn to_ref(&self) -> InterfaceRef<'_, Self> {
        InterfaceRef::from_interface(self)
    }
}

/// This has the same memory representation as `IFoo`, but represents a borrowed interface pointer.
///
/// This type has no `Drop` impl; it does not AddRef/Release the given interface. However, because
/// it has a lifetime parameter, it always represents a non-null pointer to an interface.
#[repr(transparent)]
pub struct InterfaceRef<'a, I>(NonNull<c_void>, PhantomData<&'a I>);

impl<I> Copy for InterfaceRef<'_, I> {}

impl<I> Clone for InterfaceRef<'_, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: core::fmt::Debug + Interface> core::fmt::Debug for InterfaceRef<'_, I> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <I as core::fmt::Debug>::fmt(&**self, f)
    }
}

impl<I: Interface> InterfaceRef<'_, I> {
    /// Creates an `InterfaceRef` from a raw pointer. _This is extremely dangerous, since there
    /// is no lifetime tracking at all!_
    ///
    /// # Safety
    /// The caller must guarantee that the `'a` lifetime parameter is bound by context to a correct
    /// lifetime.
    #[inline(always)]
    pub unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self(ptr, PhantomData)
    }

    /// Creates an `InterfaceRef` from an interface reference. This safely associates the lifetime
    /// of the interface reference with the `'a` parameter of `InterfaceRef`. This allows for
    /// lifetime checking _without_ calling AddRef/Release on the underlying lifetime, which can
    /// improve efficiency.
    #[inline(always)]
    pub fn from_interface(interface: &I) -> Self {
        unsafe {
            // SAFETY: new_unchecked() should be valid because Interface::as_raw should always
            // return a non-null pointer.
            Self(NonNull::new_unchecked(interface.as_raw()), PhantomData)
        }
    }

    /// Calls AddRef on the underlying COM interface and returns an "owned" (counted) reference.
    #[inline(always)]
    pub fn to_owned(self) -> I {
        (*self).clone()
    }
}

impl<'a, 'i: 'a, I: Interface> From<&'i I> for InterfaceRef<'a, I> {
    #[inline(always)]
    fn from(interface: &'a I) -> Self {
        InterfaceRef::from_interface(interface)
    }
}

impl<I: Interface> core::ops::Deref for InterfaceRef<'_, I> {
    type Target = I;

    #[inline(always)]
    fn deref(&self) -> &I {
        unsafe { core::mem::transmute(self) }
    }
}

/// This IID identifies a special protocol, used by [`Interface::cast_to_any`]. This is _not_
/// an ordinary COM interface; it uses special lifetime rules and a larger interface pointer.
/// See the comments on [`Interface::cast_to_any`].
#[doc(hidden)]
pub const DYNAMIC_CAST_IID: GUID = GUID::from_u128(0xae49d5cb_143f_431c_874c_2729336e4eca);
