use crate::Win32::System::Com::IDispatch;

impl From<IDispatch> for windows_core::VARIANT {
    fn from(value: IDispatch) -> Self {
        unsafe {
            Self::from_raw(windows_core::imp::VARIANT {
                Anonymous: windows_core::imp::VARIANT_0 {
                    Anonymous: windows_core::imp::VARIANT_0_0 { vt: 9, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: windows_core::imp::VARIANT_0_0_0 { pdispVal: core::mem::transmute(value) } },
                },
            })
        }
    }
}

impl From<IDispatch> for windows_core::PROPVARIANT {
    fn from(value: IDispatch) -> Self {
        unsafe {
            Self::from_raw(windows_core::imp::PROPVARIANT {
                Anonymous: windows_core::imp::PROPVARIANT_0 {
                    Anonymous: windows_core::imp::PROPVARIANT_0_0 { vt: 9, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: windows_core::imp::PROPVARIANT_0_0_0 { pdispVal: core::mem::transmute(value) } },
                },
            })
        }
    }
}

impl TryFrom<&windows_core::VARIANT> for IDispatch {
    type Error = windows_core::Error;
    fn try_from(from: &windows_core::VARIANT) -> windows_core::Result<Self> {
        let from = from.as_raw();
        unsafe {
            if from.Anonymous.Anonymous.vt == 9 && !from.Anonymous.Anonymous.Anonymous.pdispVal.is_null() {
                let dispatch: &IDispatch = core::mem::transmute(&from.Anonymous.Anonymous.Anonymous.pdispVal);
                Ok(dispatch.clone())
            } else {
                Err(windows_core::Error::from_hresult(windows_core::imp::TYPE_E_TYPEMISMATCH))
            }
        }
    }
}

impl TryFrom<&windows_core::PROPVARIANT> for IDispatch {
    type Error = windows_core::Error;
    fn try_from(from: &windows_core::PROPVARIANT) -> windows_core::Result<Self> {
        let from = from.as_raw();
        unsafe {
            if from.Anonymous.Anonymous.vt == 9 && !from.Anonymous.Anonymous.Anonymous.pdispVal.is_null() {
                let dispatch: &IDispatch = core::mem::transmute(&from.Anonymous.Anonymous.Anonymous.pdispVal);
                Ok(dispatch.clone())
            } else {
                Err(windows_core::Error::from_hresult(windows_core::imp::TYPE_E_TYPEMISMATCH))
            }
        }
    }
}
