use super::*;
use core::ffi::c_void;
use core::ptr::null_mut;

/// Parent interface for all WinRT interfaces.
///
/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// [`IInspectable`] represents the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IInspectable(pub IUnknown);

interface_hierarchy!(IInspectable, IUnknown);

impl IInspectable {
    /// Returns the canonical type name for the underlying object.
    #[cfg(windows)]
    pub fn GetRuntimeClassName(&self) -> Result<HSTRING> {
        unsafe {
            let mut abi = null_mut();
            (self.vtable().GetRuntimeClassName)(core::mem::transmute_copy(self), &mut abi).ok()?;
            Ok(core::mem::transmute::<*mut c_void, HSTRING>(abi))
        }
    }

    /// Gets the trust level of the current object.
    pub fn GetTrustLevel(&self) -> Result<i32> {
        unsafe {
            let mut value = 0;
            (self.vtable().GetTrustLevel)(core::mem::transmute_copy(self), &mut value).ok()?;
            Ok(value)
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct IInspectable_Vtbl {
    pub base: IUnknown_Vtbl,
    pub GetIids: unsafe extern "system" fn(
        this: *mut c_void,
        count: *mut u32,
        values: *mut *mut GUID,
    ) -> HRESULT,
    pub GetRuntimeClassName:
        unsafe extern "system" fn(this: *mut c_void, value: *mut *mut c_void) -> HRESULT,
    pub GetTrustLevel: unsafe extern "system" fn(this: *mut c_void, value: *mut i32) -> HRESULT,
}

unsafe impl Interface for IInspectable {
    type Vtable = IInspectable_Vtbl;
    const IID: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
}

impl RuntimeType for IInspectable {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl RuntimeName for IInspectable {}

impl IInspectable_Vtbl {
    pub const fn new<Identity: IUnknownImpl, Name: RuntimeName, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIids(
            _: *mut c_void,
            count: *mut u32,
            values: *mut *mut GUID,
        ) -> HRESULT {
            unsafe {
                if count.is_null() || values.is_null() {
                    return imp::E_POINTER;
                }
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *count = 0;
                *values = null_mut();
                HRESULT(0)
            }
        }
        unsafe extern "system" fn GetRuntimeClassName<T: RuntimeName>(
            _: *mut c_void,
            value: *mut *mut c_void,
        ) -> HRESULT {
            unsafe {
                if value.is_null() {
                    return imp::E_POINTER;
                }

                #[cfg(windows)]
                {
                    *value = core::mem::transmute::<HSTRING, *mut c_void>(T::NAME.into());
                }

                #[cfg(not(windows))]
                {
                    *value = core::ptr::null_mut();
                }

                HRESULT(0)
            }
        }
        unsafe extern "system" fn GetTrustLevel<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
            value: *mut i32,
        ) -> HRESULT {
            unsafe {
                if value.is_null() {
                    return imp::E_POINTER;
                }
                let this = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
                (*this).GetTrustLevel(value)
            }
        }
        Self {
            base: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIids,
            GetRuntimeClassName: GetRuntimeClassName::<Name>,
            GetTrustLevel: GetTrustLevel::<Identity, OFFSET>,
        }
    }
}
