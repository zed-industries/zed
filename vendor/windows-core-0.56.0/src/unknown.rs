use super::*;

/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

#[doc(hidden)]
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(this: *mut std::ffi::c_void, iid: *const GUID, interface: *mut *mut std::ffi::c_void) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_Vtbl;
    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().AddRef)(std::mem::transmute_copy(self));
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release)(std::mem::transmute_copy(self));
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl std::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IUnknown").field(&self.0).finish()
    }
}

#[doc(hidden)]
pub trait IUnknownImpl {
    type Impl;
    /// Get a reference to the backing implementation.
    fn get_impl(&self) -> &Self::Impl;

    /// The classic `QueryInterface` method from COM.
    ///
    /// # Safety
    ///
    /// This function is safe to call as long as the interface pointer is non-null and valid for writes
    /// of an interface pointer.
    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut std::ffi::c_void) -> HRESULT;

    /// Increments the reference count of the interface
    fn AddRef(&self) -> u32;

    /// Decrements the reference count causing the interface's memory to be freed when the count is 0
    ///
    /// # Safety
    ///
    /// This function should only be called when the interfacer pointer is no longer used as calling `Release`
    /// on a non-aliased interface pointer and then using that interface pointer may result in use after free.
    unsafe fn Release(&self) -> u32;

    /// Gets the trust level of the current object.
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT;
}

impl IUnknown_Vtbl {
    pub const fn new<T: IUnknownImpl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<T: IUnknownImpl, const OFFSET: isize>(this: *mut std::ffi::c_void, iid: *const GUID, interface: *mut *mut std::ffi::c_void) -> HRESULT {
            let this = (this as *mut *mut std::ffi::c_void).offset(OFFSET) as *mut T;
            (*this).QueryInterface(iid, interface)
        }
        unsafe extern "system" fn AddRef<T: IUnknownImpl, const OFFSET: isize>(this: *mut std::ffi::c_void) -> u32 {
            let this = (this as *mut *mut std::ffi::c_void).offset(OFFSET) as *mut T;
            (*this).AddRef()
        }
        unsafe extern "system" fn Release<T: IUnknownImpl, const OFFSET: isize>(this: *mut std::ffi::c_void) -> u32 {
            let this = (this as *mut *mut std::ffi::c_void).offset(OFFSET) as *mut T;
            (*this).Release()
        }
        Self { QueryInterface: QueryInterface::<T, OFFSET>, AddRef: AddRef::<T, OFFSET>, Release: Release::<T, OFFSET> }
    }
}
