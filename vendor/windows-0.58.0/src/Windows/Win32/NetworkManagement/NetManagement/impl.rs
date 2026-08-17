pub trait IEnumNetCfgBindingInterface_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: *const Option<IEnumNetCfgBindingInterface>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumNetCfgBindingInterface {}
impl IEnumNetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetCfgBindingInterface_Vtbl
    where
        Identity: IEnumNetCfgBindingInterface_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingInterface_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingInterface_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingInterface_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingInterface_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingInterface as windows_core::Interface>::IID
    }
}
pub trait IEnumNetCfgBindingPath_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: *const Option<IEnumNetCfgBindingPath>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumNetCfgBindingPath {}
impl IEnumNetCfgBindingPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetCfgBindingPath_Vtbl
    where
        Identity: IEnumNetCfgBindingPath_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingPath_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingPath_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingPath_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgBindingPath_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingPath as windows_core::Interface>::IID
    }
}
pub trait IEnumNetCfgComponent_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<INetCfgComponent>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: *const Option<IEnumNetCfgComponent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumNetCfgComponent {}
impl IEnumNetCfgComponent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetCfgComponent_Vtbl
    where
        Identity: IEnumNetCfgComponent_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgComponent_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgComponent_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgComponent_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetCfgComponent_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgComponent as windows_core::Interface>::IID
    }
}
pub trait INetCfg_Impl: Sized {
    fn Initialize(&self, pvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EnumComponents(&self, pguidclass: *const windows_core::GUID, ppenumcomponent: *mut Option<IEnumNetCfgComponent>) -> windows_core::Result<()>;
    fn FindComponent(&self, pszwinfid: &windows_core::PCWSTR, pcomponent: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn QueryNetCfgClass(&self, pguidclass: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfg {}
impl INetCfg_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfg_Vtbl
    where
        Identity: INetCfg_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvreserved: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::Initialize(this, core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::Uninitialize(this).into()
        }
        unsafe extern "system" fn Apply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::Apply(this).into()
        }
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn EnumComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidclass: *const windows_core::GUID, ppenumcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::EnumComponents(this, core::mem::transmute_copy(&pguidclass), core::mem::transmute_copy(&ppenumcomponent)).into()
        }
        unsafe extern "system" fn FindComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwinfid: windows_core::PCWSTR, pcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::FindComponent(this, core::mem::transmute(&pszwinfid), core::mem::transmute_copy(&pcomponent)).into()
        }
        unsafe extern "system" fn QueryNetCfgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidclass: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfg_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfg_Impl::QueryNetCfgClass(this, core::mem::transmute_copy(&pguidclass), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EnumComponents: EnumComponents::<Identity, OFFSET>,
            FindComponent: FindComponent::<Identity, OFFSET>,
            QueryNetCfgClass: QueryNetCfgClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfg as windows_core::Interface>::IID
    }
}
pub trait INetCfgBindingInterface_Impl: Sized {
    fn GetName(&self, ppszwinterfacename: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetUpperComponent(&self, ppnccitem: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn GetLowerComponent(&self, ppnccitem: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgBindingInterface {}
impl INetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgBindingInterface_Vtbl
    where
        Identity: INetCfgBindingInterface_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwinterfacename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingInterface_Impl::GetName(this, core::mem::transmute_copy(&ppszwinterfacename)).into()
        }
        unsafe extern "system" fn GetUpperComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnccitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingInterface_Impl::GetUpperComponent(this, core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn GetLowerComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnccitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingInterface_Impl::GetLowerComponent(this, core::mem::transmute_copy(&ppnccitem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetUpperComponent: GetUpperComponent::<Identity, OFFSET>,
            GetLowerComponent: GetLowerComponent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgBindingInterface as windows_core::Interface>::IID
    }
}
pub trait INetCfgBindingPath_Impl: Sized {
    fn IsSamePathAs(&self, ppath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn IsSubPathOf(&self, ppath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<()>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetPathToken(&self, ppszwpathtoken: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetOwner(&self, ppcomponent: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn GetDepth(&self) -> windows_core::Result<u32>;
    fn EnumBindingInterfaces(&self, ppenuminterface: *mut Option<IEnumNetCfgBindingInterface>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgBindingPath {}
impl INetCfgBindingPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgBindingPath_Vtbl
    where
        Identity: INetCfgBindingPath_Impl,
    {
        unsafe extern "system" fn IsSamePathAs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::IsSamePathAs(this, windows_core::from_raw_borrowed(&ppath)).into()
        }
        unsafe extern "system" fn IsSubPathOf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::IsSubPathOf(this, windows_core::from_raw_borrowed(&ppath)).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::IsEnabled(this).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetPathToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwpathtoken: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::GetPathToken(this, core::mem::transmute_copy(&ppszwpathtoken)).into()
        }
        unsafe extern "system" fn GetOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::GetOwner(this, core::mem::transmute_copy(&ppcomponent)).into()
        }
        unsafe extern "system" fn GetDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinterfaces: *mut u32) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetCfgBindingPath_Impl::GetDepth(this) {
                Ok(ok__) => {
                    pcinterfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenuminterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgBindingPath_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgBindingPath_Impl::EnumBindingInterfaces(this, core::mem::transmute_copy(&ppenuminterface)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsSamePathAs: IsSamePathAs::<Identity, OFFSET>,
            IsSubPathOf: IsSubPathOf::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            GetPathToken: GetPathToken::<Identity, OFFSET>,
            GetOwner: GetOwner::<Identity, OFFSET>,
            GetDepth: GetDepth::<Identity, OFFSET>,
            EnumBindingInterfaces: EnumBindingInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgBindingPath as windows_core::Interface>::IID
    }
}
pub trait INetCfgClass_Impl: Sized {
    fn FindComponent(&self, pszwinfid: &windows_core::PCWSTR, ppnccitem: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn EnumComponents(&self, ppenumcomponent: *mut Option<IEnumNetCfgComponent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgClass {}
impl INetCfgClass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgClass_Vtbl
    where
        Identity: INetCfgClass_Impl,
    {
        unsafe extern "system" fn FindComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwinfid: windows_core::PCWSTR, ppnccitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClass_Impl::FindComponent(this, core::mem::transmute(&pszwinfid), core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn EnumComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClass_Impl::EnumComponents(this, core::mem::transmute_copy(&ppenumcomponent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindComponent: FindComponent::<Identity, OFFSET>,
            EnumComponents: EnumComponents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgClass as windows_core::Interface>::IID
    }
}
pub trait INetCfgClassSetup_Impl: Sized {
    fn SelectAndInstall(&self, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn Install(&self, pszwinfid: &windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: &windows_core::PCWSTR, pszwanswersections: &windows_core::PCWSTR, ppnccitem: *mut Option<INetCfgComponent>) -> windows_core::Result<()>;
    fn DeInstall(&self, pcomponent: Option<&INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgClassSetup {}
impl INetCfgClassSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgClassSetup_Vtbl
    where
        Identity: INetCfgClassSetup_Impl,
    {
        unsafe extern "system" fn SelectAndInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgClassSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClassSetup_Impl::SelectAndInstall(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pobotoken), core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwinfid: windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: windows_core::PCWSTR, pszwanswersections: windows_core::PCWSTR, ppnccitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgClassSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClassSetup_Impl::Install(this, core::mem::transmute(&pszwinfid), core::mem::transmute_copy(&pobotoken), core::mem::transmute_copy(&dwsetupflags), core::mem::transmute_copy(&dwupgradefrombuildno), core::mem::transmute(&pszwanswerfile), core::mem::transmute(&pszwanswersections), core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn DeInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomponent: *mut core::ffi::c_void, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgClassSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClassSetup_Impl::DeInstall(this, windows_core::from_raw_borrowed(&pcomponent), core::mem::transmute_copy(&pobotoken), core::mem::transmute_copy(&pmszwrefs)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectAndInstall: SelectAndInstall::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            DeInstall: DeInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgClassSetup as windows_core::Interface>::IID
    }
}
pub trait INetCfgClassSetup2_Impl: Sized + INetCfgClassSetup_Impl {
    fn UpdateNonEnumeratedComponent(&self, picomp: Option<&INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgClassSetup2 {}
impl INetCfgClassSetup2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgClassSetup2_Vtbl
    where
        Identity: INetCfgClassSetup2_Impl,
    {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, picomp: *mut core::ffi::c_void, dwsetupflags: u32, dwupgradefrombuildno: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgClassSetup2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgClassSetup2_Impl::UpdateNonEnumeratedComponent(this, windows_core::from_raw_borrowed(&picomp), core::mem::transmute_copy(&dwsetupflags), core::mem::transmute_copy(&dwupgradefrombuildno)).into()
        }
        Self { base__: INetCfgClassSetup_Vtbl::new::<Identity, OFFSET>(), UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as windows_core::Interface>::IID || iid == &<INetCfgClassSetup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait INetCfgComponent_Impl: Sized {
    fn GetDisplayName(&self, ppszwdisplayname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetDisplayName(&self, pszwdisplayname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHelpText(&self, pszwhelptext: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetId(&self, ppszwid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
    fn GetInstanceGuid(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPnpDevNodeId(&self, ppszwdevnodeid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetClassGuid(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetBindName(&self, ppszwbindname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self) -> windows_core::Result<u32>;
    fn OpenParamKey(&self, phkey: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn RaisePropertyUi(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for INetCfgComponent {}
#[cfg(feature = "Win32_System_Registry")]
impl INetCfgComponent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponent_Vtbl
    where
        Identity: INetCfgComponent_Impl,
    {
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetDisplayName(this, core::mem::transmute_copy(&ppszwdisplayname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwdisplayname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::SetDisplayName(this, core::mem::transmute(&pszwdisplayname)).into()
        }
        unsafe extern "system" fn GetHelpText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwhelptext: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetHelpText(this, core::mem::transmute_copy(&pszwhelptext)).into()
        }
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetId(this, core::mem::transmute_copy(&ppszwid)).into()
        }
        unsafe extern "system" fn GetCharacteristics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetCfgComponent_Impl::GetCharacteristics(this) {
                Ok(ok__) => {
                    pdwcharacteristics.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetInstanceGuid(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetPnpDevNodeId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwdevnodeid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetPnpDevNodeId(this, core::mem::transmute_copy(&ppszwdevnodeid)).into()
        }
        unsafe extern "system" fn GetClassGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetClassGuid(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetBindName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwbindname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::GetBindName(this, core::mem::transmute_copy(&ppszwbindname)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetCfgComponent_Impl::GetDeviceStatus(this) {
                Ok(ok__) => {
                    pulstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::OpenParamKey(this, core::mem::transmute_copy(&phkey)).into()
        }
        unsafe extern "system" fn RaisePropertyUi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponent_Impl::RaisePropertyUi(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&punkcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetHelpText: GetHelpText::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
            GetInstanceGuid: GetInstanceGuid::<Identity, OFFSET>,
            GetPnpDevNodeId: GetPnpDevNodeId::<Identity, OFFSET>,
            GetClassGuid: GetClassGuid::<Identity, OFFSET>,
            GetBindName: GetBindName::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            OpenParamKey: OpenParamKey::<Identity, OFFSET>,
            RaisePropertyUi: RaisePropertyUi::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponent as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentBindings_Impl: Sized {
    fn BindTo(&self, pnccitem: Option<&INetCfgComponent>) -> windows_core::Result<()>;
    fn UnbindFrom(&self, pnccitem: Option<&INetCfgComponent>) -> windows_core::Result<()>;
    fn SupportsBindingInterface(&self, dwflags: u32, pszwinterfacename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsBoundTo(&self, pnccitem: Option<&INetCfgComponent>) -> windows_core::Result<()>;
    fn IsBindableTo(&self, pnccitem: Option<&INetCfgComponent>) -> windows_core::Result<()>;
    fn EnumBindingPaths(&self, dwflags: u32, ppienum: *mut Option<IEnumNetCfgBindingPath>) -> windows_core::Result<()>;
    fn MoveBefore(&self, pncbitemsrc: Option<&INetCfgBindingPath>, pncbitemdest: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn MoveAfter(&self, pncbitemsrc: Option<&INetCfgBindingPath>, pncbitemdest: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentBindings {}
impl INetCfgComponentBindings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentBindings_Vtbl
    where
        Identity: INetCfgComponentBindings_Impl,
    {
        unsafe extern "system" fn BindTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnccitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::BindTo(this, windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn UnbindFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnccitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::UnbindFrom(this, windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn SupportsBindingInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszwinterfacename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::SupportsBindingInterface(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszwinterfacename)).into()
        }
        unsafe extern "system" fn IsBoundTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnccitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::IsBoundTo(this, windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn IsBindableTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnccitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::IsBindableTo(this, windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn EnumBindingPaths<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::EnumBindingPaths(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppienum)).into()
        }
        unsafe extern "system" fn MoveBefore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncbitemsrc: *mut core::ffi::c_void, pncbitemdest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::MoveBefore(this, windows_core::from_raw_borrowed(&pncbitemsrc), windows_core::from_raw_borrowed(&pncbitemdest)).into()
        }
        unsafe extern "system" fn MoveAfter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncbitemsrc: *mut core::ffi::c_void, pncbitemdest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentBindings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentBindings_Impl::MoveAfter(this, windows_core::from_raw_borrowed(&pncbitemsrc), windows_core::from_raw_borrowed(&pncbitemdest)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindTo: BindTo::<Identity, OFFSET>,
            UnbindFrom: UnbindFrom::<Identity, OFFSET>,
            SupportsBindingInterface: SupportsBindingInterface::<Identity, OFFSET>,
            IsBoundTo: IsBoundTo::<Identity, OFFSET>,
            IsBindableTo: IsBindableTo::<Identity, OFFSET>,
            EnumBindingPaths: EnumBindingPaths::<Identity, OFFSET>,
            MoveBefore: MoveBefore::<Identity, OFFSET>,
            MoveAfter: MoveAfter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentBindings as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentControl_Impl: Sized {
    fn Initialize(&self, picomp: Option<&INetCfgComponent>, pinetcfg: Option<&INetCfg>, finstalling: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ApplyRegistryChanges(&self) -> windows_core::Result<()>;
    fn ApplyPnpChanges(&self, picallback: Option<&INetCfgPnpReconfigCallback>) -> windows_core::Result<()>;
    fn CancelChanges(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentControl {}
impl INetCfgComponentControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentControl_Vtbl
    where
        Identity: INetCfgComponentControl_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, picomp: *mut core::ffi::c_void, pinetcfg: *mut core::ffi::c_void, finstalling: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentControl_Impl::Initialize(this, windows_core::from_raw_borrowed(&picomp), windows_core::from_raw_borrowed(&pinetcfg), core::mem::transmute_copy(&finstalling)).into()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentControl_Impl::ApplyRegistryChanges(this).into()
        }
        unsafe extern "system" fn ApplyPnpChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentControl_Impl::ApplyPnpChanges(this, windows_core::from_raw_borrowed(&picallback)).into()
        }
        unsafe extern "system" fn CancelChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentControl_Impl::CancelChanges(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ApplyRegistryChanges: ApplyRegistryChanges::<Identity, OFFSET>,
            ApplyPnpChanges: ApplyPnpChanges::<Identity, OFFSET>,
            CancelChanges: CancelChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentControl as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyBinding_Impl: Sized {
    fn QueryBindingPath(&self, dwchangeflag: u32, pipath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn NotifyBindingPath(&self, dwchangeflag: u32, pipath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentNotifyBinding {}
impl INetCfgComponentNotifyBinding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentNotifyBinding_Vtbl
    where
        Identity: INetCfgComponentNotifyBinding_Impl,
    {
        unsafe extern "system" fn QueryBindingPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeflag: u32, pipath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentNotifyBinding_Impl::QueryBindingPath(this, core::mem::transmute_copy(&dwchangeflag), windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn NotifyBindingPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeflag: u32, pipath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentNotifyBinding_Impl::NotifyBindingPath(this, core::mem::transmute_copy(&dwchangeflag), windows_core::from_raw_borrowed(&pipath)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBindingPath: QueryBindingPath::<Identity, OFFSET>,
            NotifyBindingPath: NotifyBindingPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyBinding as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyGlobal_Impl: Sized {
    fn GetSupportedNotifications(&self) -> windows_core::Result<u32>;
    fn SysQueryBindingPath(&self, dwchangeflag: u32, pipath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn SysNotifyBindingPath(&self, dwchangeflag: u32, pipath: Option<&INetCfgBindingPath>) -> windows_core::Result<()>;
    fn SysNotifyComponent(&self, dwchangeflag: u32, picomp: Option<&INetCfgComponent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentNotifyGlobal {}
impl INetCfgComponentNotifyGlobal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentNotifyGlobal_Vtbl
    where
        Identity: INetCfgComponentNotifyGlobal_Impl,
    {
        unsafe extern "system" fn GetSupportedNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnotifications: *mut u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyGlobal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetCfgComponentNotifyGlobal_Impl::GetSupportedNotifications(this) {
                Ok(ok__) => {
                    dwnotifications.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeflag: u32, pipath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyGlobal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentNotifyGlobal_Impl::SysQueryBindingPath(this, core::mem::transmute_copy(&dwchangeflag), windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeflag: u32, pipath: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyGlobal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentNotifyGlobal_Impl::SysNotifyBindingPath(this, core::mem::transmute_copy(&dwchangeflag), windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeflag: u32, picomp: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentNotifyGlobal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentNotifyGlobal_Impl::SysNotifyComponent(this, core::mem::transmute_copy(&dwchangeflag), windows_core::from_raw_borrowed(&picomp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedNotifications: GetSupportedNotifications::<Identity, OFFSET>,
            SysQueryBindingPath: SysQueryBindingPath::<Identity, OFFSET>,
            SysNotifyBindingPath: SysNotifyBindingPath::<Identity, OFFSET>,
            SysNotifyComponent: SysNotifyComponent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyGlobal as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentPropertyUi_Impl: Sized {
    fn QueryPropertyUi(&self, punkreserved: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetContext(&self, punkreserved: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MergePropPages(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ValidateProperties(&self, hwndsheet: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn ApplyProperties(&self) -> windows_core::Result<()>;
    fn CancelProperties(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentPropertyUi {}
impl INetCfgComponentPropertyUi_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentPropertyUi_Vtbl
    where
        Identity: INetCfgComponentPropertyUi_Impl,
    {
        unsafe extern "system" fn QueryPropertyUi<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkreserved: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::QueryPropertyUi(this, windows_core::from_raw_borrowed(&punkreserved)).into()
        }
        unsafe extern "system" fn SetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkreserved: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::SetContext(this, windows_core::from_raw_borrowed(&punkreserved)).into()
        }
        unsafe extern "system" fn MergePropPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::MergePropPages(this, core::mem::transmute_copy(&pdwdefpages), core::mem::transmute_copy(&pahpspprivate), core::mem::transmute_copy(&pcpages), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pszstartpage)).into()
        }
        unsafe extern "system" fn ValidateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::ValidateProperties(this, core::mem::transmute_copy(&hwndsheet)).into()
        }
        unsafe extern "system" fn ApplyProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::ApplyProperties(this).into()
        }
        unsafe extern "system" fn CancelProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentPropertyUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentPropertyUi_Impl::CancelProperties(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryPropertyUi: QueryPropertyUi::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            MergePropPages: MergePropPages::<Identity, OFFSET>,
            ValidateProperties: ValidateProperties::<Identity, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, OFFSET>,
            CancelProperties: CancelProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentPropertyUi as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentSetup_Impl: Sized {
    fn Install(&self, dwsetupflags: u32) -> windows_core::Result<()>;
    fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> windows_core::Result<()>;
    fn ReadAnswerFile(&self, pszwanswerfile: &windows_core::PCWSTR, pszwanswersections: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Removing(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentSetup {}
impl INetCfgComponentSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentSetup_Vtbl
    where
        Identity: INetCfgComponentSetup_Impl,
    {
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsetupflags: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSetup_Impl::Install(this, core::mem::transmute_copy(&dwsetupflags)).into()
        }
        unsafe extern "system" fn Upgrade<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSetup_Impl::Upgrade(this, core::mem::transmute_copy(&dwsetupflags), core::mem::transmute_copy(&dwupgradefombuildno)).into()
        }
        unsafe extern "system" fn ReadAnswerFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwanswerfile: windows_core::PCWSTR, pszwanswersections: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSetup_Impl::ReadAnswerFile(this, core::mem::transmute(&pszwanswerfile), core::mem::transmute(&pszwanswersections)).into()
        }
        unsafe extern "system" fn Removing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSetup_Impl::Removing(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Install: Install::<Identity, OFFSET>,
            Upgrade: Upgrade::<Identity, OFFSET>,
            ReadAnswerFile: ReadAnswerFile::<Identity, OFFSET>,
            Removing: Removing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentSetup as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentSysPrep_Impl: Sized {
    fn SaveAdapterParameters(&self, pncsp: Option<&INetCfgSysPrep>, pszwanswersections: &windows_core::PCWSTR, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreAdapterParameters(&self, pszwanswerfile: &windows_core::PCWSTR, pszwanswersection: &windows_core::PCWSTR, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentSysPrep {}
impl INetCfgComponentSysPrep_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentSysPrep_Vtbl
    where
        Identity: INetCfgComponentSysPrep_Impl,
    {
        unsafe extern "system" fn SaveAdapterParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncsp: *mut core::ffi::c_void, pszwanswersections: windows_core::PCWSTR, padapterinstanceguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSysPrep_Impl::SaveAdapterParameters(this, windows_core::from_raw_borrowed(&pncsp), core::mem::transmute(&pszwanswersections), core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwanswerfile: windows_core::PCWSTR, pszwanswersection: windows_core::PCWSTR, padapterinstanceguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentSysPrep_Impl::RestoreAdapterParameters(this, core::mem::transmute(&pszwanswerfile), core::mem::transmute(&pszwanswersection), core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveAdapterParameters: SaveAdapterParameters::<Identity, OFFSET>,
            RestoreAdapterParameters: RestoreAdapterParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentSysPrep as windows_core::Interface>::IID
    }
}
pub trait INetCfgComponentUpperEdge_Impl: Sized {
    fn GetInterfaceIdsForAdapter(&self, padapter: Option<&INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn AddInterfacesToAdapter(&self, padapter: Option<&INetCfgComponent>, dwnuminterfaces: u32) -> windows_core::Result<()>;
    fn RemoveInterfacesFromAdapter(&self, padapter: Option<&INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgComponentUpperEdge {}
impl INetCfgComponentUpperEdge_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgComponentUpperEdge_Vtbl
    where
        Identity: INetCfgComponentUpperEdge_Impl,
    {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut core::ffi::c_void, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentUpperEdge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentUpperEdge_Impl::GetInterfaceIdsForAdapter(this, windows_core::from_raw_borrowed(&padapter), core::mem::transmute_copy(&pdwnuminterfaces), core::mem::transmute_copy(&ppguidinterfaceids)).into()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut core::ffi::c_void, dwnuminterfaces: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentUpperEdge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentUpperEdge_Impl::AddInterfacesToAdapter(this, windows_core::from_raw_borrowed(&padapter), core::mem::transmute_copy(&dwnuminterfaces)).into()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut core::ffi::c_void, dwnuminterfaces: u32, pguidinterfaceids: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetCfgComponentUpperEdge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgComponentUpperEdge_Impl::RemoveInterfacesFromAdapter(this, windows_core::from_raw_borrowed(&padapter), core::mem::transmute_copy(&dwnuminterfaces), core::mem::transmute_copy(&pguidinterfaceids)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterfaceIdsForAdapter: GetInterfaceIdsForAdapter::<Identity, OFFSET>,
            AddInterfacesToAdapter: AddInterfacesToAdapter::<Identity, OFFSET>,
            RemoveInterfacesFromAdapter: RemoveInterfacesFromAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgComponentUpperEdge as windows_core::Interface>::IID
    }
}
pub trait INetCfgLock_Impl: Sized {
    fn AcquireWriteLock(&self, cmstimeout: u32, pszwclientdescription: &windows_core::PCWSTR, ppszwclientdescription: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn ReleaseWriteLock(&self) -> windows_core::Result<()>;
    fn IsWriteLocked(&self, ppszwclientdescription: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgLock {}
impl INetCfgLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgLock_Vtbl
    where
        Identity: INetCfgLock_Impl,
    {
        unsafe extern "system" fn AcquireWriteLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmstimeout: u32, pszwclientdescription: windows_core::PCWSTR, ppszwclientdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgLock_Impl::AcquireWriteLock(this, core::mem::transmute_copy(&cmstimeout), core::mem::transmute(&pszwclientdescription), core::mem::transmute_copy(&ppszwclientdescription)).into()
        }
        unsafe extern "system" fn ReleaseWriteLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetCfgLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgLock_Impl::ReleaseWriteLock(this).into()
        }
        unsafe extern "system" fn IsWriteLocked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwclientdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgLock_Impl::IsWriteLocked(this, core::mem::transmute_copy(&ppszwclientdescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireWriteLock: AcquireWriteLock::<Identity, OFFSET>,
            ReleaseWriteLock: ReleaseWriteLock::<Identity, OFFSET>,
            IsWriteLocked: IsWriteLocked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgLock as windows_core::Interface>::IID
    }
}
pub trait INetCfgPnpReconfigCallback_Impl: Sized {
    fn SendPnpReconfig(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: &windows_core::PCWSTR, pszwlower: &windows_core::PCWSTR, pvdata: *const core::ffi::c_void, dwsizeofdata: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgPnpReconfigCallback {}
impl INetCfgPnpReconfigCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgPnpReconfigCallback_Vtbl
    where
        Identity: INetCfgPnpReconfigCallback_Impl,
    {
        unsafe extern "system" fn SendPnpReconfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: windows_core::PCWSTR, pszwlower: windows_core::PCWSTR, pvdata: *const core::ffi::c_void, dwsizeofdata: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgPnpReconfigCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgPnpReconfigCallback_Impl::SendPnpReconfig(this, core::mem::transmute_copy(&layer), core::mem::transmute(&pszwupper), core::mem::transmute(&pszwlower), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&dwsizeofdata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SendPnpReconfig: SendPnpReconfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgPnpReconfigCallback as windows_core::Interface>::IID
    }
}
pub trait INetCfgSysPrep_Impl: Sized {
    fn HrSetupSetFirstDword(&self, pwszsection: &windows_core::PCWSTR, pwszkey: &windows_core::PCWSTR, dwvalue: u32) -> windows_core::Result<()>;
    fn HrSetupSetFirstString(&self, pwszsection: &windows_core::PCWSTR, pwszkey: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn HrSetupSetFirstStringAsBool(&self, pwszsection: &windows_core::PCWSTR, pwszkey: &windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HrSetupSetFirstMultiSzField(&self, pwszsection: &windows_core::PCWSTR, pwszkey: &windows_core::PCWSTR, pmszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetCfgSysPrep {}
impl INetCfgSysPrep_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetCfgSysPrep_Vtbl
    where
        Identity: INetCfgSysPrep_Impl,
    {
        unsafe extern "system" fn HrSetupSetFirstDword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsection: windows_core::PCWSTR, pwszkey: windows_core::PCWSTR, dwvalue: u32) -> windows_core::HRESULT
        where
            Identity: INetCfgSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgSysPrep_Impl::HrSetupSetFirstDword(this, core::mem::transmute(&pwszsection), core::mem::transmute(&pwszkey), core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsection: windows_core::PCWSTR, pwszkey: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgSysPrep_Impl::HrSetupSetFirstString(this, core::mem::transmute(&pwszsection), core::mem::transmute(&pwszkey), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsection: windows_core::PCWSTR, pwszkey: windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: INetCfgSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgSysPrep_Impl::HrSetupSetFirstStringAsBool(this, core::mem::transmute(&pwszsection), core::mem::transmute(&pwszkey), core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsection: windows_core::PCWSTR, pwszkey: windows_core::PCWSTR, pmszvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetCfgSysPrep_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetCfgSysPrep_Impl::HrSetupSetFirstMultiSzField(this, core::mem::transmute(&pwszsection), core::mem::transmute(&pwszkey), core::mem::transmute(&pmszvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HrSetupSetFirstDword: HrSetupSetFirstDword::<Identity, OFFSET>,
            HrSetupSetFirstString: HrSetupSetFirstString::<Identity, OFFSET>,
            HrSetupSetFirstStringAsBool: HrSetupSetFirstStringAsBool::<Identity, OFFSET>,
            HrSetupSetFirstMultiSzField: HrSetupSetFirstMultiSzField::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetCfgSysPrep as windows_core::Interface>::IID
    }
}
pub trait INetLanConnectionUiInfo_Impl: Sized {
    fn GetDeviceGuid(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for INetLanConnectionUiInfo {}
impl INetLanConnectionUiInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetLanConnectionUiInfo_Vtbl
    where
        Identity: INetLanConnectionUiInfo_Impl,
    {
        unsafe extern "system" fn GetDeviceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetLanConnectionUiInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetLanConnectionUiInfo_Impl::GetDeviceGuid(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceGuid: GetDeviceGuid::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetLanConnectionUiInfo as windows_core::Interface>::IID
    }
}
pub trait INetRasConnectionIpUiInfo_Impl: Sized {
    fn GetUiInfo(&self, pinfo: *mut RASCON_IPUI) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetRasConnectionIpUiInfo {}
impl INetRasConnectionIpUiInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetRasConnectionIpUiInfo_Vtbl
    where
        Identity: INetRasConnectionIpUiInfo_Impl,
    {
        unsafe extern "system" fn GetUiInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> windows_core::HRESULT
        where
            Identity: INetRasConnectionIpUiInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetRasConnectionIpUiInfo_Impl::GetUiInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUiInfo: GetUiInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetRasConnectionIpUiInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IProvisioningDomain_Impl: Sized {
    fn Add(&self, pszwpathtofolder: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Query(&self, pszwdomain: &windows_core::PCWSTR, pszwlanguage: &windows_core::PCWSTR, pszwxpathquery: &windows_core::PCWSTR) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IProvisioningDomain {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IProvisioningDomain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvisioningDomain_Vtbl
    where
        Identity: IProvisioningDomain_Impl,
    {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwpathtofolder: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IProvisioningDomain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProvisioningDomain_Impl::Add(this, core::mem::transmute(&pszwpathtofolder)).into()
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwdomain: windows_core::PCWSTR, pszwlanguage: windows_core::PCWSTR, pszwxpathquery: windows_core::PCWSTR, nodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProvisioningDomain_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvisioningDomain_Impl::Query(this, core::mem::transmute(&pszwdomain), core::mem::transmute(&pszwlanguage), core::mem::transmute(&pszwxpathquery)) {
                Ok(ok__) => {
                    nodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvisioningDomain as windows_core::Interface>::IID
    }
}
pub trait IProvisioningProfileWireless_Impl: Sized {
    fn CreateProfile(&self, bstrxmlwirelessconfigprofile: &windows_core::BSTR, bstrxmlconnectionconfigprofile: &windows_core::BSTR, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IProvisioningProfileWireless {}
impl IProvisioningProfileWireless_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvisioningProfileWireless_Vtbl
    where
        Identity: IProvisioningProfileWireless_Impl,
    {
        unsafe extern "system" fn CreateProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxmlwirelessconfigprofile: core::mem::MaybeUninit<windows_core::BSTR>, bstrxmlconnectionconfigprofile: core::mem::MaybeUninit<windows_core::BSTR>, padapterinstanceguid: *const windows_core::GUID, pulstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProvisioningProfileWireless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvisioningProfileWireless_Impl::CreateProfile(this, core::mem::transmute(&bstrxmlwirelessconfigprofile), core::mem::transmute(&bstrxmlconnectionconfigprofile), core::mem::transmute_copy(&padapterinstanceguid)) {
                Ok(ok__) => {
                    pulstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProfile: CreateProfile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvisioningProfileWireless as windows_core::Interface>::IID
    }
}
