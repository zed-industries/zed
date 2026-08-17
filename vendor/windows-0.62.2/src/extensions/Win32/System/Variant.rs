use crate::core::*;
use crate::Win32::Foundation::*;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
use crate::Win32::System::Com::StructuredStorage::*;
use crate::Win32::System::Com::*;
use crate::Win32::System::Variant::*;
use core::mem::*;

macro_rules! variant_from_value {
    ($from:ident, $vt:ident, $field:ident, $value:expr) => {
        impl From<$from> for VARIANT {
            fn from(value: $from) -> Self {
                Self {
                    Anonymous: VARIANT_0 {
                        Anonymous: ManuallyDrop::new(VARIANT_0_0 { vt: $vt, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: VARIANT_0_0_0 { $field: $value(value) } }),
                    },
                }
            }
        }
    };
}

impl Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe {
            let mut value = Self::default();
            _ = VariantCopy(&mut value, self);
            value
        }
    }
}

impl Drop for VARIANT {
    fn drop(&mut self) {
        unsafe { _ = VariantClear(self) };
    }
}
impl VARIANT {
    pub fn vt(&self) -> VARENUM {
        unsafe { self.Anonymous.Anonymous.vt }
    }

    pub fn is_empty(&self) -> bool {
        self.vt() == VT_EMPTY
    }
}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::fmt::Debug for VARIANT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut debug = f.debug_struct("VARIANT");
        debug.field("type", &unsafe { self.Anonymous.Anonymous.vt });

        if let Ok(value) = BSTR::try_from(self) {
            debug.field("value", &value);
        }

        debug.finish()
    }
}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::fmt::Display for VARIANT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::write!(f, "{}", BSTR::try_from(self).unwrap_or_default())
    }
}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            if self.Anonymous.Anonymous.vt != other.Anonymous.Anonymous.vt {
                return false;
            }

            // Convert to PROPVARIANT since VarCmp does not compare various primitive types.
            let this = PROPVARIANT::try_from(self);
            let other = PROPVARIANT::try_from(other);

            if let (Ok(this), Ok(other)) = (this, other) {
                this.eq(&other)
            } else {
                false
            }
        }
    }
}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Eq for VARIANT {}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl TryFrom<&PROPVARIANT> for VARIANT {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        unsafe { PropVariantToVariant(from) }
    }
}

// VT_UNKNOWN

variant_from_value!(IUnknown, VT_UNKNOWN, punkVal, |v: IUnknown| ManuallyDrop::new(Some(v)));

impl TryFrom<&VARIANT> for IUnknown {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe {
            if from.Anonymous.Anonymous.vt == VT_UNKNOWN && !from.Anonymous.Anonymous.Anonymous.punkVal.is_none() {
                let unknown: &Self = transmute(&from.Anonymous.Anonymous.Anonymous.punkVal);
                Ok(unknown.clone())
            } else {
                Err(Error::from_hresult(TYPE_E_TYPEMISMATCH))
            }
        }
    }
}

// VT_BSTR

variant_from_value!(BSTR, VT_BSTR, bstrVal, |v: BSTR| ManuallyDrop::new(v));

impl From<&str> for VARIANT {
    fn from(value: &str) -> Self {
        BSTR::from(value).into()
    }
}

#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl TryFrom<&VARIANT> for BSTR {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let pv = PROPVARIANT::try_from(from)?;
        Self::try_from(&pv)
    }
}

// VT_BOOL

variant_from_value!(bool, VT_BOOL, boolVal, |v: bool| VARIANT_BOOL(if v { -1 } else { 0 }));

impl TryFrom<&VARIANT> for bool {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToBoolean(from) }.map(|ok| ok.0 != 0)
    }
}

// VT_UI1

variant_from_value!(u8, VT_UI1, bVal, |v: u8| v);

// VT_I1

variant_from_value!(i8, VT_I1, cVal, |v: i8| v);

// VT_UI2

variant_from_value!(u16, VT_UI2, uiVal, |v: u16| v);

impl TryFrom<&VARIANT> for u16 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToUInt16(from) }
    }
}

// VT_I2

variant_from_value!(i16, VT_I2, iVal, |v: i16| v);

impl TryFrom<&VARIANT> for i16 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToInt16(from) }
    }
}

// VT_UI4

variant_from_value!(u32, VT_UI4, ulVal, |v: u32| v);

impl TryFrom<&VARIANT> for u32 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToUInt32(from) }
    }
}

// VT_I4

variant_from_value!(i32, VT_I4, lVal, |v: i32| v);

impl TryFrom<&VARIANT> for i32 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToInt32(from) }
    }
}

// VT_UI8

impl From<u64> for VARIANT {
    fn from(value: u64) -> Self {
        Self {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 { vt: VT_UI8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: VARIANT_0_0_0 { ullVal: value } }),
            },
        }
    }
}

impl TryFrom<&VARIANT> for u64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToUInt64(from) }
    }
}

// VT_I8

impl From<i64> for VARIANT {
    fn from(value: i64) -> Self {
        Self {
            Anonymous: VARIANT_0 { Anonymous: ManuallyDrop::new(VARIANT_0_0 { vt: VT_I8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: VARIANT_0_0_0 { llVal: value } }) },
        }
    }
}

impl TryFrom<&VARIANT> for i64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToInt64(from) }
    }
}

// VT_R4

variant_from_value!(f32, VT_R4, fltVal, |v: f32| v);

// VT_R8

variant_from_value!(f64, VT_R8, dblVal, |v: f64| v);

impl TryFrom<&VARIANT> for f64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe { VariantToDouble(from) }
    }
}

// VT_DISPATCH

impl From<IDispatch> for VARIANT {
    fn from(value: IDispatch) -> Self {
        unsafe {
            Self {
                Anonymous: VARIANT_0 {
                    Anonymous: ManuallyDrop::new(VARIANT_0_0 { vt: VT_DISPATCH, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: VARIANT_0_0_0 { pdispVal: transmute(value) } }),
                },
            }
        }
    }
}

impl TryFrom<&VARIANT> for IDispatch {
    type Error = windows_core::Error;
    fn try_from(from: &VARIANT) -> windows_core::Result<Self> {
        unsafe {
            if from.Anonymous.Anonymous.vt == VT_DISPATCH && !from.Anonymous.Anonymous.Anonymous.pdispVal.is_none() {
                let dispatch: &Self = transmute(&from.Anonymous.Anonymous.Anonymous.pdispVal);
                Ok(dispatch.clone())
            } else {
                Err(windows_core::Error::from_hresult(TYPE_E_TYPEMISMATCH))
            }
        }
    }
}
