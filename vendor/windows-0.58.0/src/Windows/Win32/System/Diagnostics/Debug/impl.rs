#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IDebugExtendedProperty_Impl: Sized + IDebugProperty_Impl {
    fn GetExtendedPropertyInfo(&self, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> windows_core::Result<()>;
    fn EnumExtendedMembers(&self, dwfieldspec: u32, nradix: u32) -> windows_core::Result<IEnumDebugExtendedPropertyInfo>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IDebugExtendedProperty {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IDebugExtendedProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugExtendedProperty_Vtbl
    where
        Identity: IDebugExtendedProperty_Impl,
    {
        unsafe extern "system" fn GetExtendedPropertyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> windows_core::HRESULT
        where
            Identity: IDebugExtendedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebugExtendedProperty_Impl::GetExtendedPropertyInfo(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&pextendedpropertyinfo)).into()
        }
        unsafe extern "system" fn EnumExtendedMembers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppeepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDebugExtendedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDebugExtendedProperty_Impl::EnumExtendedMembers(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix)) {
                Ok(ok__) => {
                    ppeepi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDebugProperty_Vtbl::new::<Identity, OFFSET>(),
            GetExtendedPropertyInfo: GetExtendedPropertyInfo::<Identity, OFFSET>,
            EnumExtendedMembers: EnumExtendedMembers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugExtendedProperty as windows_core::Interface>::IID || iid == &<IDebugProperty as windows_core::Interface>::IID
    }
}
pub trait IDebugProperty_Impl: Sized {
    fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> windows_core::Result<()>;
    fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const windows_core::GUID, rgvar: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetValueAsString(&self, pszvalue: &windows_core::PCWSTR, nradix: u32) -> windows_core::Result<()>;
    fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const windows_core::GUID) -> windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetParent(&self) -> windows_core::Result<IDebugProperty>;
}
impl windows_core::RuntimeName for IDebugProperty {}
impl IDebugProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugProperty_Vtbl
    where
        Identity: IDebugProperty_Impl,
    {
        unsafe extern "system" fn GetPropertyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> windows_core::HRESULT
        where
            Identity: IDebugProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebugProperty_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&ppropertyinfo)).into()
        }
        unsafe extern "system" fn GetExtendedInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinfos: u32, rgguidextendedinfo: *const windows_core::GUID, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IDebugProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebugProperty_Impl::GetExtendedInfo(this, core::mem::transmute_copy(&cinfos), core::mem::transmute_copy(&rgguidextendedinfo), core::mem::transmute_copy(&rgvar)).into()
        }
        unsafe extern "system" fn SetValueAsString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, nradix: u32) -> windows_core::HRESULT
        where
            Identity: IDebugProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebugProperty_Impl::SetValueAsString(this, core::mem::transmute(&pszvalue), core::mem::transmute_copy(&nradix)).into()
        }
        unsafe extern "system" fn EnumMembers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, refiid: *const windows_core::GUID, ppepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDebugProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDebugProperty_Impl::EnumMembers(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&refiid)) {
                Ok(ok__) => {
                    ppepi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdebugprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDebugProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDebugProperty_Impl::GetParent(this) {
                Ok(ok__) => {
                    ppdebugprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            GetExtendedInfo: GetExtendedInfo::<Identity, OFFSET>,
            SetValueAsString: SetValueAsString::<Identity, OFFSET>,
            EnumMembers: EnumMembers::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugProperty as windows_core::Interface>::IID
    }
}
pub trait IDebugPropertyEnumType_All_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_All {}
impl IDebugPropertyEnumType_All_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugPropertyEnumType_All_Vtbl
    where
        Identity: IDebugPropertyEnumType_All_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idebugpropertyenumtype_all0000: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDebugPropertyEnumType_All_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDebugPropertyEnumType_All_Impl::GetName(this) {
                Ok(ok__) => {
                    __midl__idebugpropertyenumtype_all0000.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
pub trait IDebugPropertyEnumType_Arguments_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Arguments {}
impl IDebugPropertyEnumType_Arguments_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugPropertyEnumType_Arguments_Vtbl
    where
        Identity: IDebugPropertyEnumType_Arguments_Impl,
    {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Arguments as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
pub trait IDebugPropertyEnumType_Locals_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Locals {}
impl IDebugPropertyEnumType_Locals_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugPropertyEnumType_Locals_Vtbl
    where
        Identity: IDebugPropertyEnumType_Locals_Impl,
    {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Locals as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
pub trait IDebugPropertyEnumType_LocalsPlusArgs_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl windows_core::RuntimeName for IDebugPropertyEnumType_LocalsPlusArgs {}
impl IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugPropertyEnumType_LocalsPlusArgs_Vtbl
    where
        Identity: IDebugPropertyEnumType_LocalsPlusArgs_Impl,
    {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_LocalsPlusArgs as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
pub trait IDebugPropertyEnumType_Registers_Impl: Sized + IDebugPropertyEnumType_All_Impl {}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Registers {}
impl IDebugPropertyEnumType_Registers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebugPropertyEnumType_Registers_Vtbl
    where
        Identity: IDebugPropertyEnumType_Registers_Impl,
    {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Registers as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IEnumDebugExtendedPropertyInfo_Impl: Sized {
    fn Next(&self, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugExtendedPropertyInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IEnumDebugExtendedPropertyInfo {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IEnumDebugExtendedPropertyInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumDebugExtendedPropertyInfo_Vtbl
    where
        Identity: IEnumDebugExtendedPropertyInfo_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugExtendedPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugExtendedPropertyInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgextendedpropertyinfo), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugExtendedPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugExtendedPropertyInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDebugExtendedPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugExtendedPropertyInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedpe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDebugExtendedPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDebugExtendedPropertyInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    pedpe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugExtendedPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDebugExtendedPropertyInfo_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugExtendedPropertyInfo as windows_core::Interface>::IID
    }
}
pub trait IEnumDebugPropertyInfo_Impl: Sized {
    fn Next(&self, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumDebugPropertyInfo {}
impl IEnumDebugPropertyInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumDebugPropertyInfo_Vtbl
    where
        Identity: IEnumDebugPropertyInfo_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugPropertyInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pi), core::mem::transmute_copy(&pceltsfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugPropertyInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDebugPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDebugPropertyInfo_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumDebugPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDebugPropertyInfo_Impl::Clone(this) {
                Ok(ok__) => {
                    ppepi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumDebugPropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDebugPropertyInfo_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugPropertyInfo as windows_core::Interface>::IID
    }
}
pub trait IObjectSafety_Impl: Sized {
    fn GetInterfaceSafetyOptions(&self, riid: *const windows_core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> windows_core::Result<()>;
    fn SetInterfaceSafetyOptions(&self, riid: *const windows_core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectSafety {}
impl IObjectSafety_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectSafety_Vtbl
    where
        Identity: IObjectSafety_Impl,
    {
        unsafe extern "system" fn GetInterfaceSafetyOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IObjectSafety_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectSafety_Impl::GetInterfaceSafetyOptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdwsupportedoptions), core::mem::transmute_copy(&pdwenabledoptions)).into()
        }
        unsafe extern "system" fn SetInterfaceSafetyOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> windows_core::HRESULT
        where
            Identity: IObjectSafety_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectSafety_Impl::SetInterfaceSafetyOptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwoptionsetmask), core::mem::transmute_copy(&dwenabledoptions)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterfaceSafetyOptions: GetInterfaceSafetyOptions::<Identity, OFFSET>,
            SetInterfaceSafetyOptions: SetInterfaceSafetyOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectSafety as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IPerPropertyBrowsing2_Impl: Sized {
    fn GetDisplayString(&self, dispid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: i32) -> windows_core::Result<windows_core::GUID>;
    fn GetPredefinedStrings(&self, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> windows_core::Result<()>;
    fn SetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IPerPropertyBrowsing2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IPerPropertyBrowsing2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPerPropertyBrowsing2_Vtbl
    where
        Identity: IPerPropertyBrowsing2_Impl,
    {
        unsafe extern "system" fn GetDisplayString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerPropertyBrowsing2_Impl::GetDisplayString(this, core::mem::transmute_copy(&dispid)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pclsidproppage: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerPropertyBrowsing2_Impl::MapPropertyToPage(this, core::mem::transmute_copy(&dispid)) {
                Ok(ok__) => {
                    pclsidproppage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerPropertyBrowsing2_Impl::GetPredefinedStrings(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pcastrings), core::mem::transmute_copy(&pcacookies)).into()
        }
        unsafe extern "system" fn SetPredefinedValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerPropertyBrowsing2_Impl::SetPredefinedValue(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, OFFSET>,
            SetPredefinedValue: SetPredefinedValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing2 as windows_core::Interface>::IID
    }
}
