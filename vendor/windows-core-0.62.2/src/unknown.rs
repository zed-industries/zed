use super::*;
use core::ffi::c_void;
use core::ptr::NonNull;

/// Base interface for all COM interfaces.
///
/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(NonNull<c_void>);

#[doc(hidden)]
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_Vtbl;
    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().AddRef)(core::mem::transmute_copy(self));
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release)(core::mem::transmute_copy(self));
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // First we test for ordinary pointer equality. If two COM interface pointers have the
        // same pointer value, then they are the same object. This can save us a lot of time,
        // since calling QueryInterface is much more expensive than a single pointer comparison.
        //
        // However, interface pointers may have different values and yet point to the same object.
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        core::ptr::eq(self.as_raw(), other.as_raw())
            || self.cast::<Self>().unwrap().0 == other.cast::<Self>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl core::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("IUnknown").field(&self.as_raw()).finish()
    }
}

/// The `#[implement]` macro generates implementations of this trait for the types
/// that it generates, e.g. `MyApp_Impl`,
///
/// `ComObject` uses this trait to interact with boxed COM objects.
#[doc(hidden)]
pub trait IUnknownImpl {
    /// The contained user type, e.g. `MyApp`. Also known as the "inner" type.
    type Impl;

    /// Get a reference to the backing implementation.
    fn get_impl(&self) -> &Self::Impl;

    /// Get a mutable reference to the contained (inner) object.
    fn get_impl_mut(&mut self) -> &mut Self::Impl;

    /// Consumes the box and returns the contained (inner) object. This is the opposite of `new_box`.
    fn into_inner(self) -> Self::Impl;

    /// The classic `QueryInterface` method from COM.
    ///
    /// # Safety
    ///
    /// This function is safe to call as long as the interface pointer is non-null and valid for writes
    /// of an interface pointer.
    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT;

    /// Increments the reference count of the interface
    fn AddRef(&self) -> u32;

    /// Decrements the reference count causing the interface's memory to be freed when the count is 0
    ///
    /// # Safety
    ///
    /// This function should only be called when the interface pointer is no longer used as calling `Release`
    /// on a non-aliased interface pointer and then using that interface pointer may result in use after free.
    ///
    /// This function takes `*mut Self` because the object may be freed by the time this method returns.
    /// Taking `&self` would violate Rust's rules on reference lifetime.
    unsafe fn Release(self_: *mut Self) -> u32;

    /// Returns `true` if the reference count of the box is equal to 1.
    fn is_reference_count_one(&self) -> bool;

    /// Gets the trust level of the current object.
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT;

    /// Gets a borrowed reference to an interface that is implemented by this ComObject.
    ///
    /// The returned reference does not have an additional reference count.
    /// You can AddRef it by calling to_owned().
    #[inline(always)]
    fn as_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        Self: ComObjectInterface<I>,
    {
        <Self as ComObjectInterface<I>>::as_interface_ref(self)
    }

    /// Gets an owned (counted) reference to an interface that is implemented by this ComObject.
    #[inline(always)]
    fn to_interface<I: Interface>(&self) -> I
    where
        Self: ComObjectInterface<I>,
    {
        <Self as ComObjectInterface<I>>::as_interface_ref(self).to_owned()
    }

    /// Creates a new owned reference to this object.
    ///
    /// # Safety
    ///
    /// This function can only be safely called by `<Foo>_Impl` objects that are embedded in a
    /// `ComObject`. Since we only allow safe Rust code to access these objects using a `ComObject`
    /// or a `&<Foo>_Impl` that points within a `ComObject`, this is safe.
    fn to_object(&self) -> ComObject<Self::Impl>
    where
        Self::Impl: ComObjectInner<Outer = Self>;
}

impl IUnknown_Vtbl {
    pub const fn new<T: IUnknownImpl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
            iid: *const GUID,
            interface: *mut *mut c_void,
        ) -> HRESULT {
            unsafe {
                let this = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                (*this).QueryInterface(iid, interface)
            }
        }
        unsafe extern "system" fn AddRef<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                (*this).AddRef()
            }
        }
        unsafe extern "system" fn Release<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
        ) -> u32 {
            unsafe {
                let this = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                T::Release(this)
            }
        }
        Self {
            QueryInterface: QueryInterface::<T, OFFSET>,
            AddRef: AddRef::<T, OFFSET>,
            Release: Release::<T, OFFSET>,
        }
    }
}
