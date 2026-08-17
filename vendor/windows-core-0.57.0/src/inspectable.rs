use super::*;
use core::ffi::c_void;
use core::mem::{transmute, transmute_copy};
use core::ptr::null_mut;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// [`IInspectable`] represents the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IInspectable(pub IUnknown);

interface_hierarchy!(IInspectable, IUnknown);

impl IInspectable {
    /// Returns the canonical type name for the underlying object.
    pub fn GetRuntimeClassName(&self) -> Result<HSTRING> {
        unsafe {
            let mut abi = null_mut();
            (self.vtable().GetRuntimeClassName)(transmute_copy(self), &mut abi).ok()?;
            Ok(transmute::<*mut c_void, HSTRING>(abi))
        }
    }

    /// Gets the trust level of the current object.
    pub fn GetTrustLevel(&self) -> Result<i32> {
        unsafe {
            let mut value = 0;
            (self.vtable().GetTrustLevel)(transmute_copy(self), &mut value).ok()?;
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
        unsafe extern "system" fn GetRuntimeClassName<T: RuntimeName>(
            _: *mut c_void,
            value: *mut *mut c_void,
        ) -> HRESULT {
            if value.is_null() {
                return imp::E_POINTER;
            }
            let h: HSTRING = T::NAME.into(); // TODO: should be try_into
            *value = transmute::<HSTRING, *mut c_void>(h);
            HRESULT(0)
        }
        unsafe extern "system" fn GetTrustLevel<T: IUnknownImpl, const OFFSET: isize>(
            this: *mut c_void,
            value: *mut i32,
        ) -> HRESULT {
            if value.is_null() {
                return imp::E_POINTER;
            }
            let this = (this as *mut *mut c_void).offset(OFFSET) as *mut T;
            (*this).GetTrustLevel(value)
        }
        Self {
            base: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIids,
            GetRuntimeClassName: GetRuntimeClassName::<Name>,
            GetTrustLevel: GetTrustLevel::<Identity, OFFSET>,
        }
    }
}

impl core::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents.
        let name = <Self as Interface>::cast::<imp::IStringable>(self)
            .and_then(|s| s.ToString())
            .or_else(|_| self.GetRuntimeClassName())
            .unwrap_or_default();
        write!(f, "\"{}\"", name)
    }
}

macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl TryFrom<$t> for IInspectable {
            type Error = Error;
            fn try_from(value: $t) -> Result<Self> {
                imp::PropertyValue::$m(value)
            }
        }
        impl TryFrom<IInspectable> for $t {
            type Error = Error;
            fn try_from(value: IInspectable) -> Result<Self> {
                <IInspectable as Interface>::cast::<imp::IReference<$t>>(&value)?.Value()
            }
        }
        impl TryFrom<&IInspectable> for $t {
            type Error = Error;
            fn try_from(value: &IInspectable) -> Result<Self> {
                <IInspectable as Interface>::cast::<imp::IReference<$t>>(value)?.Value()
            }
        })*
    };
}
primitive_boxed_type! {
    (bool, CreateBoolean),
    (u8, CreateUInt8),
    (i16, CreateInt16),
    (u16, CreateUInt16),
    (i32, CreateInt32),
    (u32, CreateUInt32),
    (i64, CreateInt64),
    (u64, CreateUInt64),
    (f32, CreateSingle),
    (f64, CreateDouble)
}
impl TryFrom<&str> for IInspectable {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value: HSTRING = value.into();
        imp::PropertyValue::CreateString(&value)
    }
}
impl TryFrom<HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: HSTRING) -> Result<Self> {
        imp::PropertyValue::CreateString(&value)
    }
}
impl TryFrom<&HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: &HSTRING) -> Result<Self> {
        imp::PropertyValue::CreateString(value)
    }
}
impl TryFrom<IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<imp::IReference<HSTRING>>(&value)?.Value()
    }
}
impl TryFrom<&IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: &IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<imp::IReference<HSTRING>>(value)?.Value()
    }
}
