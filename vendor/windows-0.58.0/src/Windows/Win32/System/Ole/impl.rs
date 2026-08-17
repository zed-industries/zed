#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkEx_Impl: Sized + super::Com::IAdviseSink_Impl {
    fn OnViewStatusChange(&self, dwviewstatus: u32);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IAdviseSinkEx {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAdviseSinkEx_Vtbl
    where
        Identity: IAdviseSinkEx_Impl,
    {
        unsafe extern "system" fn OnViewStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwviewstatus: u32)
        where
            Identity: IAdviseSinkEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAdviseSinkEx_Impl::OnViewStatusChange(this, core::mem::transmute_copy(&dwviewstatus))
        }
        Self { base__: super::Com::IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnViewStatusChange: OnViewStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSinkEx as windows_core::Interface>::IID || iid == &<super::Com::IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICanHandleException_Impl: Sized {
    fn CanHandleException(&self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICanHandleException {}
#[cfg(feature = "Win32_System_Com")]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICanHandleException_Vtbl
    where
        Identity: ICanHandleException_Impl,
    {
        unsafe extern "system" fn CanHandleException<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICanHandleException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICanHandleException_Impl::CanHandleException(this, core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&pvar)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanHandleException: CanHandleException::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICanHandleException as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IClassFactory2_Impl: Sized + super::Com::IClassFactory_Impl {
    fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::Result<()>;
    fn RequestLicKey(&self, dwreserved: u32) -> windows_core::Result<windows_core::BSTR>;
    fn CreateInstanceLic(&self, punkouter: Option<&windows_core::IUnknown>, punkreserved: Option<windows_core::IUnknown>, riid: *const windows_core::GUID, bstrkey: &windows_core::BSTR, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IClassFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl IClassFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClassFactory2_Vtbl
    where
        Identity: IClassFactory2_Impl,
    {
        unsafe extern "system" fn GetLicInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plicinfo: *mut LICINFO) -> windows_core::HRESULT
        where
            Identity: IClassFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClassFactory2_Impl::GetLicInfo(this, core::mem::transmute_copy(&plicinfo)).into()
        }
        unsafe extern "system" fn RequestLicKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pbstrkey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IClassFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IClassFactory2_Impl::RequestLicKey(this, core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    pbstrkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceLic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, punkreserved: *mut core::ffi::c_void, riid: *const windows_core::GUID, bstrkey: core::mem::MaybeUninit<windows_core::BSTR>, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClassFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClassFactory2_Impl::CreateInstanceLic(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute(&punkreserved), core::mem::transmute_copy(&riid), core::mem::transmute(&bstrkey), core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base__: super::Com::IClassFactory_Vtbl::new::<Identity, OFFSET>(),
            GetLicInfo: GetLicInfo::<Identity, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory2 as windows_core::Interface>::IID || iid == &<super::Com::IClassFactory as windows_core::Interface>::IID
    }
}
pub trait IContinue_Impl: Sized {
    fn FContinue(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContinue {}
impl IContinue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContinue_Vtbl
    where
        Identity: IContinue_Impl,
    {
        unsafe extern "system" fn FContinue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContinue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContinue_Impl::FContinue(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FContinue: FContinue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinue as windows_core::Interface>::IID
    }
}
pub trait IContinueCallback_Impl: Sized {
    fn FContinue(&self) -> windows_core::Result<()>;
    fn FContinuePrinting(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContinueCallback {}
impl IContinueCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContinueCallback_Vtbl
    where
        Identity: IContinueCallback_Impl,
    {
        unsafe extern "system" fn FContinue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContinueCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContinueCallback_Impl::FContinue(this).into()
        }
        unsafe extern "system" fn FContinuePrinting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContinueCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContinueCallback_Impl::FContinuePrinting(this, core::mem::transmute_copy(&ncntprinted), core::mem::transmute_copy(&ncurpage), core::mem::transmute(&pwszprintstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FContinue: FContinue::<Identity, OFFSET>,
            FContinuePrinting: FContinuePrinting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinueCallback as windows_core::Interface>::IID
    }
}
pub trait ICreateErrorInfo_Impl: Sized {
    fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetSource(&self, szsource: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDescription(&self, szdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpFile(&self, szhelpfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICreateErrorInfo {}
impl ICreateErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateErrorInfo_Vtbl
    where
        Identity: ICreateErrorInfo_Impl,
    {
        unsafe extern "system" fn SetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateErrorInfo_Impl::SetGUID(this, core::mem::transmute_copy(&rguid)).into()
        }
        unsafe extern "system" fn SetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsource: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateErrorInfo_Impl::SetSource(this, core::mem::transmute(&szsource)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateErrorInfo_Impl::SetDescription(this, core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn SetHelpFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfile: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateErrorInfo_Impl::SetHelpFile(this, core::mem::transmute(&szhelpfile)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateErrorInfo_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGUID: SetGUID::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SetHelpFile: SetHelpFile::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo_Impl: Sized {
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetTypeFlags(&self, utypeflags: u32) -> windows_core::Result<()>;
    fn SetDocString(&self, pstrdoc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()>;
    fn AddRefTypeInfo(&self, ptinfo: Option<&super::Com::ITypeInfo>, phreftype: *const u32) -> windows_core::Result<()>;
    fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> windows_core::Result<()>;
    fn AddImplType(&self, index: u32, hreftype: u32) -> windows_core::Result<()>;
    fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> windows_core::Result<()>;
    fn SetAlignment(&self, cbalignment: u16) -> windows_core::Result<()>;
    fn SetSchema(&self, pstrschema: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> windows_core::Result<()>;
    fn SetFuncAndParamNames(&self, index: u32, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::Result<()>;
    fn SetVarName(&self, index: u32, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> windows_core::Result<()>;
    fn DefineFuncAsDllEntry(&self, index: u32, szdllname: &windows_core::PCWSTR, szprocname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFuncDocString(&self, index: u32, szdocstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetVarDocString(&self, index: u32, szdocstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetMops(&self, index: u32, bstrmops: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> windows_core::Result<()>;
    fn LayOut(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICreateTypeInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICreateTypeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateTypeInfo_Vtbl
    where
        Identity: ICreateTypeInfo_Impl,
    {
        unsafe extern "system" fn SetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetGuid(this, core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetTypeFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, utypeflags: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetTypeFlags(this, core::mem::transmute_copy(&utypeflags)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdoc: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetDocString(this, core::mem::transmute(&pstrdoc)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetVersion(this, core::mem::transmute_copy(&wmajorvernum), core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn AddRefTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptinfo: *mut core::ffi::c_void, phreftype: *const u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::AddRefTypeInfo(this, windows_core::from_raw_borrowed(&ptinfo), core::mem::transmute_copy(&phreftype)).into()
        }
        unsafe extern "system" fn AddFuncDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::AddFuncDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pfuncdesc)).into()
        }
        unsafe extern "system" fn AddImplType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, hreftype: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::AddImplType(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&hreftype)).into()
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetImplTypeFlags(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&impltypeflags)).into()
        }
        unsafe extern "system" fn SetAlignment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbalignment: u16) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetAlignment(this, core::mem::transmute_copy(&cbalignment)).into()
        }
        unsafe extern "system" fn SetSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrschema: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetSchema(this, core::mem::transmute(&pstrschema)).into()
        }
        unsafe extern "system" fn AddVarDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::AddVarDesc(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pvardesc)).into()
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, rgsznames: *const windows_core::PCWSTR, cnames: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetFuncAndParamNames(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames)).into()
        }
        unsafe extern "system" fn SetVarName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetVarName(this, core::mem::transmute_copy(&index), core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetTypeDescAlias(this, core::mem::transmute_copy(&ptdescalias)).into()
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdllname: windows_core::PCWSTR, szprocname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::DefineFuncAsDllEntry(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdllname), core::mem::transmute(&szprocname)).into()
        }
        unsafe extern "system" fn SetFuncDocString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdocstring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetFuncDocString(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdocstring)).into()
        }
        unsafe extern "system" fn SetVarDocString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, szdocstring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetVarDocString(this, core::mem::transmute_copy(&index), core::mem::transmute(&szdocstring)).into()
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetFuncHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetVarHelpContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetMops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, bstrmops: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetMops(this, core::mem::transmute_copy(&index), core::mem::transmute(&bstrmops)).into()
        }
        unsafe extern "system" fn SetTypeIdldesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::SetTypeIdldesc(this, core::mem::transmute_copy(&pidldesc)).into()
        }
        unsafe extern "system" fn LayOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo_Impl::LayOut(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGuid: SetGuid::<Identity, OFFSET>,
            SetTypeFlags: SetTypeFlags::<Identity, OFFSET>,
            SetDocString: SetDocString::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            AddRefTypeInfo: AddRefTypeInfo::<Identity, OFFSET>,
            AddFuncDesc: AddFuncDesc::<Identity, OFFSET>,
            AddImplType: AddImplType::<Identity, OFFSET>,
            SetImplTypeFlags: SetImplTypeFlags::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            SetSchema: SetSchema::<Identity, OFFSET>,
            AddVarDesc: AddVarDesc::<Identity, OFFSET>,
            SetFuncAndParamNames: SetFuncAndParamNames::<Identity, OFFSET>,
            SetVarName: SetVarName::<Identity, OFFSET>,
            SetTypeDescAlias: SetTypeDescAlias::<Identity, OFFSET>,
            DefineFuncAsDllEntry: DefineFuncAsDllEntry::<Identity, OFFSET>,
            SetFuncDocString: SetFuncDocString::<Identity, OFFSET>,
            SetVarDocString: SetVarDocString::<Identity, OFFSET>,
            SetFuncHelpContext: SetFuncHelpContext::<Identity, OFFSET>,
            SetVarHelpContext: SetVarHelpContext::<Identity, OFFSET>,
            SetMops: SetMops::<Identity, OFFSET>,
            SetTypeIdldesc: SetTypeIdldesc::<Identity, OFFSET>,
            LayOut: LayOut::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo2_Impl: Sized + ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(&self, index: u32) -> windows_core::Result<()>;
    fn DeleteFuncDescByMemId(&self, memid: i32, invkind: super::Com::INVOKEKIND) -> windows_core::Result<()>;
    fn DeleteVarDesc(&self, index: u32) -> windows_core::Result<()>;
    fn DeleteVarDescByMemId(&self, memid: i32) -> windows_core::Result<()>;
    fn DeleteImplType(&self, index: u32) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetFuncCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetVarCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn Invalidate(&self) -> windows_core::Result<()>;
    fn SetName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICreateTypeInfo2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ICreateTypeInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateTypeInfo2_Vtbl
    where
        Identity: ICreateTypeInfo2_Impl,
    {
        unsafe extern "system" fn DeleteFuncDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::DeleteFuncDesc(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::DeleteFuncDescByMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)).into()
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::DeleteVarDesc(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::DeleteVarDescByMemId(this, core::mem::transmute_copy(&memid)).into()
        }
        unsafe extern "system" fn DeleteImplType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::DeleteImplType(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetCustData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetFuncCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetParamCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetVarCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetFuncHelpStringContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetVarHelpStringContext(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::Invalidate(this).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeInfo2_Impl::SetName(this, core::mem::transmute(&szname)).into()
        }
        Self {
            base__: ICreateTypeInfo_Vtbl::new::<Identity, OFFSET>(),
            DeleteFuncDesc: DeleteFuncDesc::<Identity, OFFSET>,
            DeleteFuncDescByMemId: DeleteFuncDescByMemId::<Identity, OFFSET>,
            DeleteVarDesc: DeleteVarDesc::<Identity, OFFSET>,
            DeleteVarDescByMemId: DeleteVarDescByMemId::<Identity, OFFSET>,
            DeleteImplType: DeleteImplType::<Identity, OFFSET>,
            SetCustData: SetCustData::<Identity, OFFSET>,
            SetFuncCustData: SetFuncCustData::<Identity, OFFSET>,
            SetParamCustData: SetParamCustData::<Identity, OFFSET>,
            SetVarCustData: SetVarCustData::<Identity, OFFSET>,
            SetImplTypeCustData: SetImplTypeCustData::<Identity, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, OFFSET>,
            SetFuncHelpStringContext: SetFuncHelpStringContext::<Identity, OFFSET>,
            SetVarHelpStringContext: SetVarHelpStringContext::<Identity, OFFSET>,
            Invalidate: Invalidate::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeInfo2 as windows_core::Interface>::IID || iid == &<ICreateTypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateTypeLib_Impl: Sized {
    fn CreateTypeInfo(&self, szname: &windows_core::PCWSTR, tkind: super::Com::TYPEKIND) -> windows_core::Result<ICreateTypeInfo>;
    fn SetName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()>;
    fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetDocString(&self, szdoc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpFileName(&self, szhelpfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()>;
    fn SetLcid(&self, lcid: u32) -> windows_core::Result<()>;
    fn SetLibFlags(&self, ulibflags: u32) -> windows_core::Result<()>;
    fn SaveAllChanges(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICreateTypeLib {}
#[cfg(feature = "Win32_System_Com")]
impl ICreateTypeLib_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateTypeLib_Vtbl
    where
        Identity: ICreateTypeLib_Impl,
    {
        unsafe extern "system" fn CreateTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICreateTypeLib_Impl::CreateTypeInfo(this, core::mem::transmute(&szname), core::mem::transmute_copy(&tkind)) {
                Ok(ok__) => {
                    ppctinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetName(this, core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetVersion(this, core::mem::transmute_copy(&wmajorvernum), core::mem::transmute_copy(&wminorvernum)).into()
        }
        unsafe extern "system" fn SetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetGuid(this, core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn SetDocString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdoc: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetDocString(this, core::mem::transmute(&szdoc)).into()
        }
        unsafe extern "system" fn SetHelpFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhelpfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetHelpFileName(this, core::mem::transmute(&szhelpfilename)).into()
        }
        unsafe extern "system" fn SetHelpContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetHelpContext(this, core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        unsafe extern "system" fn SetLcid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetLcid(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLibFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulibflags: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SetLibFlags(this, core::mem::transmute_copy(&ulibflags)).into()
        }
        unsafe extern "system" fn SaveAllChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib_Impl::SaveAllChanges(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTypeInfo: CreateTypeInfo::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            SetDocString: SetDocString::<Identity, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, OFFSET>,
            SetLcid: SetLcid::<Identity, OFFSET>,
            SetLibFlags: SetLibFlags::<Identity, OFFSET>,
            SaveAllChanges: SaveAllChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateTypeLib2_Impl: Sized + ICreateTypeLib_Impl {
    fn DeleteTypeInfo(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()>;
    fn SetHelpStringDll(&self, szfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICreateTypeLib2 {}
#[cfg(feature = "Win32_System_Com")]
impl ICreateTypeLib2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateTypeLib2_Vtbl
    where
        Identity: ICreateTypeLib2_Impl,
    {
        unsafe extern "system" fn DeleteTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib2_Impl::DeleteTypeInfo(this, core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn SetCustData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib2_Impl::SetCustData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvarval)).into()
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhelpstringcontext: u32) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib2_Impl::SetHelpStringContext(this, core::mem::transmute_copy(&dwhelpstringcontext)).into()
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szfilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICreateTypeLib2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateTypeLib2_Impl::SetHelpStringDll(this, core::mem::transmute(&szfilename)).into()
        }
        Self {
            base__: ICreateTypeLib_Vtbl::new::<Identity, OFFSET>(),
            DeleteTypeInfo: DeleteTypeInfo::<Identity, OFFSET>,
            SetCustData: SetCustData::<Identity, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateTypeLib2 as windows_core::Interface>::IID || iid == &<ICreateTypeLib as windows_core::Interface>::IID
    }
}
pub trait IDispError_Impl: Sized {
    fn QueryErrorInfo(&self, guiderrortype: &windows_core::GUID) -> windows_core::Result<IDispError>;
    fn GetNext(&self) -> windows_core::Result<IDispError>;
    fn GetHresult(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut u32) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IDispError {}
impl IDispError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDispError_Vtbl
    where
        Identity: IDispError_Impl,
    {
        unsafe extern "system" fn QueryErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guiderrortype: windows_core::GUID, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispError_Impl::QueryErrorInfo(this, core::mem::transmute(&guiderrortype)) {
                Ok(ok__) => {
                    ppde.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispError_Impl::GetNext(this) {
                Ok(ok__) => {
                    ppde.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHresult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispError_Impl::GetHresult(this) {
                Ok(ok__) => {
                    phr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispError_Impl::GetSource(this) {
                Ok(ok__) => {
                    pbstrsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwcontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispError_Impl::GetHelpInfo(this, core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pdwcontext)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDispError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispError_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryErrorInfo: QueryErrorInfo::<Identity, OFFSET>,
            GetNext: GetNext::<Identity, OFFSET>,
            GetHresult: GetHresult::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispatchEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDispID(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::Result<i32>;
    fn InvokeEx(&self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut windows_core::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: Option<&super::Com::IServiceProvider>) -> windows_core::Result<()>;
    fn DeleteMemberByName(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::Result<()>;
    fn DeleteMemberByDispID(&self, id: i32) -> windows_core::Result<()>;
    fn GetMemberProperties(&self, id: i32, grfdexfetch: u32) -> windows_core::Result<FDEX_PROP_FLAGS>;
    fn GetMemberName(&self, id: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetNextDispID(&self, grfdex: u32, id: i32) -> windows_core::Result<i32>;
    fn GetNameSpaceParent(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDispatchEx {}
#[cfg(feature = "Win32_System_Com")]
impl IDispatchEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDispatchEx_Vtbl
    where
        Identity: IDispatchEx_Impl,
    {
        unsafe extern "system" fn GetDispID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, grfdex: u32, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatchEx_Impl::GetDispID(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex)) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pei: *mut super::Com::EXCEPINFO, pspcaller: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispatchEx_Impl::InvokeEx(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdp), core::mem::transmute_copy(&pvarres), core::mem::transmute_copy(&pei), windows_core::from_raw_borrowed(&pspcaller)).into()
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, grfdex: u32) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispatchEx_Impl::DeleteMemberByName(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex)).into()
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispatchEx_Impl::DeleteMemberByDispID(this, core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetMemberProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut FDEX_PROP_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatchEx_Impl::GetMemberProperties(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&grfdexfetch)) {
                Ok(ok__) => {
                    pgrfdex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatchEx_Impl::GetMemberName(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextDispID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatchEx_Impl::GetNextDispID(this, core::mem::transmute_copy(&grfdex), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispatchEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispatchEx_Impl::GetNameSpaceParent(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDispID: GetDispID::<Identity, OFFSET>,
            InvokeEx: InvokeEx::<Identity, OFFSET>,
            DeleteMemberByName: DeleteMemberByName::<Identity, OFFSET>,
            DeleteMemberByDispID: DeleteMemberByDispID::<Identity, OFFSET>,
            GetMemberProperties: GetMemberProperties::<Identity, OFFSET>,
            GetMemberName: GetMemberName::<Identity, OFFSET>,
            GetNextDispID: GetNextDispID::<Identity, OFFSET>,
            GetNameSpaceParent: GetNameSpaceParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatchEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
pub trait IDropSource_Impl: Sized {
    fn QueryContinueDrag(&self, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> windows_core::HRESULT;
    fn GiveFeedback(&self, dweffect: DROPEFFECT) -> windows_core::HRESULT;
}
#[cfg(feature = "Win32_System_SystemServices")]
impl windows_core::RuntimeName for IDropSource {}
#[cfg(feature = "Win32_System_SystemServices")]
impl IDropSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDropSource_Vtbl
    where
        Identity: IDropSource_Impl,
    {
        unsafe extern "system" fn QueryContinueDrag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> windows_core::HRESULT
        where
            Identity: IDropSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropSource_Impl::QueryContinueDrag(this, core::mem::transmute_copy(&fescapepressed), core::mem::transmute_copy(&grfkeystate))
        }
        unsafe extern "system" fn GiveFeedback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffect: DROPEFFECT) -> windows_core::HRESULT
        where
            Identity: IDropSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropSource_Impl::GiveFeedback(this, core::mem::transmute_copy(&dweffect))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryContinueDrag: QueryContinueDrag::<Identity, OFFSET>,
            GiveFeedback: GiveFeedback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropSource as windows_core::Interface>::IID
    }
}
pub trait IDropSourceNotify_Impl: Sized {
    fn DragEnterTarget(&self, hwndtarget: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn DragLeaveTarget(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDropSourceNotify {}
impl IDropSourceNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDropSourceNotify_Vtbl
    where
        Identity: IDropSourceNotify_Impl,
    {
        unsafe extern "system" fn DragEnterTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDropSourceNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropSourceNotify_Impl::DragEnterTarget(this, core::mem::transmute_copy(&hwndtarget)).into()
        }
        unsafe extern "system" fn DragLeaveTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDropSourceNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropSourceNotify_Impl::DragLeaveTarget(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnterTarget: DragEnterTarget::<Identity, OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropSourceNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
pub trait IDropTarget_Impl: Sized {
    fn DragEnter(&self, pdataobj: Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
    fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
    fn DragLeave(&self) -> windows_core::Result<()>;
    fn Drop(&self, pdataobj: Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl windows_core::RuntimeName for IDropTarget {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl IDropTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDropTarget_Vtbl
    where
        Identity: IDropTarget_Impl,
    {
        unsafe extern "system" fn DragEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT
        where
            Identity: IDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropTarget_Impl::DragEnter(this, windows_core::from_raw_borrowed(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragOver<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT
        where
            Identity: IDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropTarget_Impl::DragOver(this, core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn DragLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropTarget_Impl::DragLeave(this).into()
        }
        unsafe extern "system" fn Drop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT
        where
            Identity: IDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDropTarget_Impl::Drop(this, windows_core::from_raw_borrowed(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragOver: DragOver::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            Drop: Drop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropTarget as windows_core::Interface>::IID
    }
}
pub trait IEnterpriseDropTarget_Impl: Sized {
    fn SetDropSourceEnterpriseId(&self, identity: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsEvaluatingEdpPolicy(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IEnterpriseDropTarget {}
impl IEnterpriseDropTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnterpriseDropTarget_Vtbl
    where
        Identity: IEnterpriseDropTarget_Impl,
    {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identity: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IEnterpriseDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnterpriseDropTarget_Impl::SetDropSourceEnterpriseId(this, core::mem::transmute(&identity)).into()
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEnterpriseDropTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnterpriseDropTarget_Impl::IsEvaluatingEdpPolicy(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Identity, OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnterpriseDropTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IEnumOLEVERB_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOLEVERB>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IEnumOLEVERB {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IEnumOLEVERB_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumOLEVERB_Vtbl
    where
        Identity: IEnumOLEVERB_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumOLEVERB_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOLEVERB_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumOLEVERB_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOLEVERB_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOLEVERB_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOLEVERB_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOLEVERB_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumOLEVERB_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOLEVERB as windows_core::Interface>::IID
    }
}
pub trait IEnumOleDocumentViews_Impl: Sized {
    fn Next(&self, cviews: u32, rgpview: *mut Option<IOleDocumentView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cviews: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOleDocumentViews>;
}
impl windows_core::RuntimeName for IEnumOleDocumentViews {}
impl IEnumOleDocumentViews_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumOleDocumentViews_Vtbl
    where
        Identity: IEnumOleDocumentViews_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cviews: u32, rgpview: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumOleDocumentViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleDocumentViews_Impl::Next(this, core::mem::transmute_copy(&cviews), core::mem::transmute_copy(&rgpview), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cviews: u32) -> windows_core::HRESULT
        where
            Identity: IEnumOleDocumentViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleDocumentViews_Impl::Skip(this, core::mem::transmute_copy(&cviews)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOleDocumentViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleDocumentViews_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOleDocumentViews_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumOleDocumentViews_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOleDocumentViews as windows_core::Interface>::IID
    }
}
pub trait IEnumOleUndoUnits_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOleUndoUnit>, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOleUndoUnits>;
}
impl windows_core::RuntimeName for IEnumOleUndoUnits {}
impl IEnumOleUndoUnits_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumOleUndoUnits_Vtbl
    where
        Identity: IEnumOleUndoUnits_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumOleUndoUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleUndoUnits_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumOleUndoUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleUndoUnits_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOleUndoUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumOleUndoUnits_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumOleUndoUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumOleUndoUnits_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOleUndoUnits as windows_core::Interface>::IID
    }
}
pub trait IEnumVARIANT_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut windows_core::VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVARIANT>;
}
impl windows_core::RuntimeName for IEnumVARIANT {}
impl IEnumVARIANT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumVARIANT_Vtbl
    where
        Identity: IEnumVARIANT_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumVARIANT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVARIANT_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumVARIANT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVARIANT_Impl::Skip(this, core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumVARIANT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVARIANT_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumVARIANT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumVARIANT_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumVARIANT as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IFont_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<super::Com::CY>;
    fn SetSize(&self, size: &super::Com::CY) -> windows_core::Result<()>;
    fn Bold(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBold(&self, bold: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Italic(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetItalic(&self, italic: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Underline(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUnderline(&self, underline: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Strikethrough(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetStrikethrough(&self, strikethrough: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Weight(&self) -> windows_core::Result<i16>;
    fn SetWeight(&self, weight: i16) -> windows_core::Result<()>;
    fn Charset(&self) -> windows_core::Result<i16>;
    fn SetCharset(&self, charset: i16) -> windows_core::Result<()>;
    fn hFont(&self) -> windows_core::Result<super::super::Graphics::Gdi::HFONT>;
    fn Clone(&self) -> windows_core::Result<IFont>;
    fn IsEqual(&self, pfontother: Option<&IFont>) -> windows_core::Result<()>;
    fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> windows_core::Result<()>;
    fn QueryTextMetrics(&self, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> windows_core::Result<()>;
    fn AddRefHfont(&self, hfont: super::super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ReleaseHfont(&self, hfont: super::super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn SetHdc(&self, hdc: super::super::Graphics::Gdi::HDC) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IFont {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IFont_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFont_Vtbl
    where
        Identity: IFont_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Name(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::Com::CY) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Size(this) {
                Ok(ok__) => {
                    psize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: super::Com::CY) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetSize(this, core::mem::transmute(&size)).into()
        }
        unsafe extern "system" fn Bold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Bold(this) {
                Ok(ok__) => {
                    pbold.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bold: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetBold(this, core::mem::transmute_copy(&bold)).into()
        }
        unsafe extern "system" fn Italic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Italic(this) {
                Ok(ok__) => {
                    pitalic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, italic: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetItalic(this, core::mem::transmute_copy(&italic)).into()
        }
        unsafe extern "system" fn Underline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Underline(this) {
                Ok(ok__) => {
                    punderline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, underline: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetUnderline(this, core::mem::transmute_copy(&underline)).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Strikethrough(this) {
                Ok(ok__) => {
                    pstrikethrough.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetStrikethrough(this, core::mem::transmute_copy(&strikethrough)).into()
        }
        unsafe extern "system" fn Weight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweight: *mut i16) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Weight(this) {
                Ok(ok__) => {
                    pweight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: i16) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetWeight(this, core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn Charset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcharset: *mut i16) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Charset(this) {
                Ok(ok__) => {
                    pcharset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: i16) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetCharset(this, core::mem::transmute_copy(&charset)).into()
        }
        unsafe extern "system" fn hFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::hFont(this) {
                Ok(ok__) => {
                    phfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFont_Impl::Clone(this) {
                Ok(ok__) => {
                    ppfont.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfontother: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pfontother)).into()
        }
        unsafe extern "system" fn SetRatio<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetRatio(this, core::mem::transmute_copy(&cylogical), core::mem::transmute_copy(&cyhimetric)).into()
        }
        unsafe extern "system" fn QueryTextMetrics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::QueryTextMetrics(this, core::mem::transmute_copy(&ptm)).into()
        }
        unsafe extern "system" fn AddRefHfont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::AddRefHfont(this, core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ReleaseHfont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::ReleaseHfont(this, core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn SetHdc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IFont_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFont_Impl::SetHdc(this, core::mem::transmute_copy(&hdc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            Bold: Bold::<Identity, OFFSET>,
            SetBold: SetBold::<Identity, OFFSET>,
            Italic: Italic::<Identity, OFFSET>,
            SetItalic: SetItalic::<Identity, OFFSET>,
            Underline: Underline::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            Strikethrough: Strikethrough::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            Weight: Weight::<Identity, OFFSET>,
            SetWeight: SetWeight::<Identity, OFFSET>,
            Charset: Charset::<Identity, OFFSET>,
            SetCharset: SetCharset::<Identity, OFFSET>,
            hFont: hFont::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            SetRatio: SetRatio::<Identity, OFFSET>,
            QueryTextMetrics: QueryTextMetrics::<Identity, OFFSET>,
            AddRefHfont: AddRefHfont::<Identity, OFFSET>,
            ReleaseHfont: ReleaseHfont::<Identity, OFFSET>,
            SetHdc: SetHdc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFont as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFontDisp {}
#[cfg(feature = "Win32_System_Com")]
impl IFontDisp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFontDisp_Vtbl
    where
        Identity: IFontDisp_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontDisp as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontEventsDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFontEventsDisp {}
#[cfg(feature = "Win32_System_Com")]
impl IFontEventsDisp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFontEventsDisp_Vtbl
    where
        Identity: IFontEventsDisp_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontEventsDisp as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IGetOleObject_Impl: Sized {
    fn GetOleObject(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGetOleObject {}
impl IGetOleObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetOleObject_Vtbl
    where
        Identity: IGetOleObject_Impl,
    {
        unsafe extern "system" fn GetOleObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetOleObject_Impl::GetOleObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOleObject: GetOleObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetOleObject as windows_core::Interface>::IID
    }
}
pub trait IGetVBAObject_Impl: Sized {
    fn GetObject(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGetVBAObject {}
impl IGetVBAObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetVBAObject_Vtbl
    where
        Identity: IGetVBAObject_Impl,
    {
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IGetVBAObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetVBAObject_Impl::GetObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetObject: GetObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetVBAObject as windows_core::Interface>::IID
    }
}
pub trait IObjectIdentity_Impl: Sized {
    fn IsEqualObject(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectIdentity {}
impl IObjectIdentity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectIdentity_Vtbl
    where
        Identity: IObjectIdentity_Impl,
    {
        unsafe extern "system" fn IsEqualObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectIdentity_Impl::IsEqualObject(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsEqualObject: IsEqualObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectIdentity as windows_core::Interface>::IID
    }
}
pub trait IObjectWithSite_Impl: Sized {
    fn SetSite(&self, punksite: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetSite(&self, riid: *const windows_core::GUID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectWithSite {}
impl IObjectWithSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectWithSite_Vtbl
    where
        Identity: IObjectWithSite_Impl,
    {
        unsafe extern "system" fn SetSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectWithSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectWithSite_Impl::SetSite(this, windows_core::from_raw_borrowed(&punksite)).into()
        }
        unsafe extern "system" fn GetSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectWithSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectWithSite_Impl::GetSite(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvsite)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSite: SetSite::<Identity, OFFSET>, GetSite: GetSite::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectWithSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleAdviseHolder_Impl: Sized {
    fn Advise(&self, padvise: Option<&super::Com::IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<super::Com::IEnumSTATDATA>;
    fn SendOnRename(&self, pmk: Option<&super::Com::IMoniker>) -> windows_core::Result<()>;
    fn SendOnSave(&self) -> windows_core::Result<()>;
    fn SendOnClose(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleAdviseHolder {}
#[cfg(feature = "Win32_System_Com")]
impl IOleAdviseHolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleAdviseHolder_Vtbl
    where
        Identity: IOleAdviseHolder_Impl,
    {
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvise: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleAdviseHolder_Impl::Advise(this, windows_core::from_raw_borrowed(&padvise)) {
                Ok(ok__) => {
                    pdwconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleAdviseHolder_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleAdviseHolder_Impl::EnumAdvise(this) {
                Ok(ok__) => {
                    ppenumadvise.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnRename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleAdviseHolder_Impl::SendOnRename(this, windows_core::from_raw_borrowed(&pmk)).into()
        }
        unsafe extern "system" fn SendOnSave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleAdviseHolder_Impl::SendOnSave(this).into()
        }
        unsafe extern "system" fn SendOnClose<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleAdviseHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleAdviseHolder_Impl::SendOnClose(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            SendOnRename: SendOnRename::<Identity, OFFSET>,
            SendOnSave: SendOnSave::<Identity, OFFSET>,
            SendOnClose: SendOnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleAdviseHolder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache_Impl: Sized {
    fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> windows_core::Result<u32>;
    fn Uncache(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumCache(&self) -> windows_core::Result<super::Com::IEnumSTATDATA>;
    fn InitCache(&self, pdataobject: Option<&super::Com::IDataObject>) -> windows_core::Result<()>;
    fn SetData(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IOleCache {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleCache_Vtbl
    where
        Identity: IOleCache_Impl,
    {
        unsafe extern "system" fn Cache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleCache_Impl::Cache(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&advf)) {
                Ok(ok__) => {
                    pdwconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uncache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT
        where
            Identity: IOleCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCache_Impl::Uncache(this, core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstatdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleCache_Impl::EnumCache(this) {
                Ok(ok__) => {
                    ppenumstatdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCache_Impl::InitCache(this, windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCache_Impl::SetData(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&frelease)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cache: Cache::<Identity, OFFSET>,
            Uncache: Uncache::<Identity, OFFSET>,
            EnumCache: EnumCache::<Identity, OFFSET>,
            InitCache: InitCache::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCache as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache2_Impl: Sized + IOleCache_Impl {
    fn UpdateCache(&self, pdataobject: Option<&super::Com::IDataObject>, grfupdf: UPDFCACHE_FLAGS, preserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DiscardCache(&self, dwdiscardoptions: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IOleCache2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IOleCache2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleCache2_Vtbl
    where
        Identity: IOleCache2_Impl,
    {
        unsafe extern "system" fn UpdateCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, grfupdf: UPDFCACHE_FLAGS, preserved: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleCache2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCache2_Impl::UpdateCache(this, windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&grfupdf), core::mem::transmute_copy(&preserved)).into()
        }
        unsafe extern "system" fn DiscardCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdiscardoptions: u32) -> windows_core::HRESULT
        where
            Identity: IOleCache2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCache2_Impl::DiscardCache(this, core::mem::transmute_copy(&dwdiscardoptions)).into()
        }
        Self { base__: IOleCache_Vtbl::new::<Identity, OFFSET>(), UpdateCache: UpdateCache::<Identity, OFFSET>, DiscardCache: DiscardCache::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCache2 as windows_core::Interface>::IID || iid == &<IOleCache as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControl_Impl: Sized {
    fn OnRun(&self, pdataobject: Option<&super::Com::IDataObject>) -> windows_core::Result<()>;
    fn OnStop(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleCacheControl {}
#[cfg(feature = "Win32_System_Com")]
impl IOleCacheControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleCacheControl_Vtbl
    where
        Identity: IOleCacheControl_Impl,
    {
        unsafe extern "system" fn OnRun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleCacheControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCacheControl_Impl::OnRun(this, windows_core::from_raw_borrowed(&pdataobject)).into()
        }
        unsafe extern "system" fn OnStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleCacheControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCacheControl_Impl::OnStop(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnRun: OnRun::<Identity, OFFSET>, OnStop: OnStop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCacheControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleClientSite_Impl: Sized {
    fn SaveObject(&self) -> windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> windows_core::Result<super::Com::IMoniker>;
    fn GetContainer(&self) -> windows_core::Result<IOleContainer>;
    fn ShowObject(&self) -> windows_core::Result<()>;
    fn OnShowWindow(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RequestNewObjectLayout(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleClientSite {}
#[cfg(feature = "Win32_System_Com")]
impl IOleClientSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleClientSite_Vtbl
    where
        Identity: IOleClientSite_Impl,
    {
        unsafe extern "system" fn SaveObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleClientSite_Impl::SaveObject(this).into()
        }
        unsafe extern "system" fn GetMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleClientSite_Impl::GetMoniker(this, core::mem::transmute(&dwassign), core::mem::transmute(&dwwhichmoniker)) {
                Ok(ok__) => {
                    ppmk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleClientSite_Impl::GetContainer(this) {
                Ok(ok__) => {
                    ppcontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleClientSite_Impl::ShowObject(this).into()
        }
        unsafe extern "system" fn OnShowWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleClientSite_Impl::OnShowWindow(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn RequestNewObjectLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleClientSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleClientSite_Impl::RequestNewObjectLayout(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveObject: SaveObject::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
            ShowObject: ShowObject::<Identity, OFFSET>,
            OnShowWindow: OnShowWindow::<Identity, OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleClientSite as windows_core::Interface>::IID
    }
}
pub trait IOleCommandTarget_Impl: Sized {
    fn QueryStatus(&self, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::Result<()>;
    fn Exec(&self, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const windows_core::VARIANT, pvaout: *mut windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleCommandTarget {}
impl IOleCommandTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleCommandTarget_Vtbl
    where
        Identity: IOleCommandTarget_Impl,
    {
        unsafe extern "system" fn QueryStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::HRESULT
        where
            Identity: IOleCommandTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCommandTarget_Impl::QueryStatus(this, core::mem::transmute_copy(&pguidcmdgroup), core::mem::transmute_copy(&ccmds), core::mem::transmute_copy(&prgcmds), core::mem::transmute_copy(&pcmdtext)).into()
        }
        unsafe extern "system" fn Exec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvaout: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IOleCommandTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleCommandTarget_Impl::Exec(this, core::mem::transmute_copy(&pguidcmdgroup), core::mem::transmute_copy(&ncmdid), core::mem::transmute_copy(&ncmdexecopt), core::mem::transmute_copy(&pvain), core::mem::transmute_copy(&pvaout)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryStatus: QueryStatus::<Identity, OFFSET>, Exec: Exec::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCommandTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleContainer_Impl: Sized + IParseDisplayName_Impl {
    fn EnumObjects(&self, grfflags: &OLECONTF) -> windows_core::Result<super::Com::IEnumUnknown>;
    fn LockContainer(&self, flock: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IOleContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleContainer_Vtbl
    where
        Identity: IOleContainer_Impl,
    {
        unsafe extern "system" fn EnumObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleContainer_Impl::EnumObjects(this, core::mem::transmute(&grfflags)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleContainer_Impl::LockContainer(this, core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base__: IParseDisplayName_Vtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            LockContainer: LockContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleContainer as windows_core::Interface>::IID || iid == &<IParseDisplayName as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOleControl_Impl: Sized {
    fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::Result<()>;
    fn OnMnemonic(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
    fn OnAmbientPropertyChange(&self, dispid: i32) -> windows_core::Result<()>;
    fn FreezeEvents(&self, bfreeze: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOleControl {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOleControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleControl_Vtbl
    where
        Identity: IOleControl_Impl,
    {
        unsafe extern "system" fn GetControlInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pci: *mut CONTROLINFO) -> windows_core::HRESULT
        where
            Identity: IOleControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControl_Impl::GetControlInfo(this, core::mem::transmute_copy(&pci)).into()
        }
        unsafe extern "system" fn OnMnemonic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT
        where
            Identity: IOleControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControl_Impl::OnMnemonic(this, core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32) -> windows_core::HRESULT
        where
            Identity: IOleControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControl_Impl::OnAmbientPropertyChange(this, core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn FreezeEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControl_Impl::FreezeEvents(this, core::mem::transmute_copy(&bfreeze)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetControlInfo: GetControlInfo::<Identity, OFFSET>,
            OnMnemonic: OnMnemonic::<Identity, OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Identity, OFFSET>,
            FreezeEvents: FreezeEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlSite_Impl: Sized {
    fn OnControlInfoChanged(&self) -> windows_core::Result<()>;
    fn LockInPlaceActive(&self, flock: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetExtendedControl(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransformCoords(&self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: &XFORMCOORDS) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> windows_core::Result<()>;
    fn OnFocus(&self, fgotfocus: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ShowPropertyFrame(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IOleControlSite {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleControlSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleControlSite_Vtbl
    where
        Identity: IOleControlSite_Impl,
    {
        unsafe extern "system" fn OnControlInfoChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::OnControlInfoChanged(this).into()
        }
        unsafe extern "system" fn LockInPlaceActive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::LockInPlaceActive(this, core::mem::transmute_copy(&flock)).into()
        }
        unsafe extern "system" fn GetExtendedControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleControlSite_Impl::GetExtendedControl(this) {
                Ok(ok__) => {
                    ppdisp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformCoords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::TransformCoords(this, core::mem::transmute_copy(&pptlhimetric), core::mem::transmute_copy(&pptfcontainer), core::mem::transmute(&dwflags)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&grfmodifiers)).into()
        }
        unsafe extern "system" fn OnFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::OnFocus(this, core::mem::transmute_copy(&fgotfocus)).into()
        }
        unsafe extern "system" fn ShowPropertyFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleControlSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleControlSite_Impl::ShowPropertyFrame(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnControlInfoChanged: OnControlInfoChanged::<Identity, OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Identity, OFFSET>,
            GetExtendedControl: GetExtendedControl::<Identity, OFFSET>,
            TransformCoords: TransformCoords::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
            OnFocus: OnFocus::<Identity, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleControlSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocument_Impl: Sized {
    fn CreateView(&self, pipsite: Option<&IOleInPlaceSite>, pstm: Option<&super::Com::IStream>, dwreserved: u32) -> windows_core::Result<IOleDocumentView>;
    fn GetDocMiscStatus(&self) -> windows_core::Result<u32>;
    fn EnumViews(&self, ppenum: *mut Option<IEnumOleDocumentViews>, ppview: *mut Option<IOleDocumentView>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IOleDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleDocument_Vtbl
    where
        Identity: IOleDocument_Impl,
    {
        unsafe extern "system" fn CreateView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsite: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, dwreserved: u32, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocument_Impl::CreateView(this, windows_core::from_raw_borrowed(&pipsite), windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocMiscStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocument_Impl::GetDocMiscStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumViews<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocument_Impl::EnumViews(this, core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&ppview)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateView: CreateView::<Identity, OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Identity, OFFSET>,
            EnumViews: EnumViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocument as windows_core::Interface>::IID
    }
}
pub trait IOleDocumentSite_Impl: Sized {
    fn ActivateMe(&self, pviewtoactivate: Option<&IOleDocumentView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleDocumentSite {}
impl IOleDocumentSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleDocumentSite_Vtbl
    where
        Identity: IOleDocumentSite_Impl,
    {
        unsafe extern "system" fn ActivateMe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewtoactivate: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentSite_Impl::ActivateMe(this, windows_core::from_raw_borrowed(&pviewtoactivate)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateMe: ActivateMe::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocumentSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocumentView_Impl: Sized {
    fn SetInPlaceSite(&self, pipsite: Option<&IOleInPlaceSite>) -> windows_core::Result<()>;
    fn GetInPlaceSite(&self) -> windows_core::Result<IOleInPlaceSite>;
    fn GetDocument(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetRect(&self, prcview: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetRectComplex(&self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn Show(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UIActivate(&self, fuiactivate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Open(&self) -> windows_core::Result<()>;
    fn CloseView(&self, dwreserved: u32) -> windows_core::Result<()>;
    fn SaveViewState(&self, pstm: Option<&super::Com::IStream>) -> windows_core::Result<()>;
    fn ApplyViewState(&self, pstm: Option<&super::Com::IStream>) -> windows_core::Result<()>;
    fn Clone(&self, pipsitenew: Option<&IOleInPlaceSite>) -> windows_core::Result<IOleDocumentView>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleDocumentView {}
#[cfg(feature = "Win32_System_Com")]
impl IOleDocumentView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleDocumentView_Vtbl
    where
        Identity: IOleDocumentView_Impl,
    {
        unsafe extern "system" fn SetInPlaceSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::SetInPlaceSite(this, windows_core::from_raw_borrowed(&pipsite)).into()
        }
        unsafe extern "system" fn GetInPlaceSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocumentView_Impl::GetInPlaceSite(this) {
                Ok(ok__) => {
                    ppipsite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocumentView_Impl::GetDocument(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::SetRect(this, core::mem::transmute_copy(&prcview)).into()
        }
        unsafe extern "system" fn GetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocumentView_Impl::GetRect(this) {
                Ok(ok__) => {
                    prcview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRectComplex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::SetRectComplex(this, core::mem::transmute_copy(&prcview), core::mem::transmute_copy(&prchscroll), core::mem::transmute_copy(&prcvscroll), core::mem::transmute_copy(&prcsizebox)).into()
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn UIActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::UIActivate(this, core::mem::transmute_copy(&fuiactivate)).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::Open(this).into()
        }
        unsafe extern "system" fn CloseView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::CloseView(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SaveViewState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::SaveViewState(this, windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn ApplyViewState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleDocumentView_Impl::ApplyViewState(this, windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipsitenew: *mut core::ffi::c_void, ppviewnew: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleDocumentView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleDocumentView_Impl::Clone(this, windows_core::from_raw_borrowed(&pipsitenew)) {
                Ok(ok__) => {
                    ppviewnew.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInPlaceSite: SetInPlaceSite::<Identity, OFFSET>,
            GetInPlaceSite: GetInPlaceSite::<Identity, OFFSET>,
            GetDocument: GetDocument::<Identity, OFFSET>,
            SetRect: SetRect::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            SetRectComplex: SetRectComplex::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            UIActivate: UIActivate::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            CloseView: CloseView::<Identity, OFFSET>,
            SaveViewState: SaveViewState::<Identity, OFFSET>,
            ApplyViewState: ApplyViewState::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleDocumentView as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOleInPlaceActiveObject_Impl: Sized + IOleWindow_Impl {
    fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
    fn OnFrameWindowActivate(&self, factivate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnDocWindowActivate(&self, factivate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ResizeBorder(&self, prcborder: *const super::super::Foundation::RECT, puiwindow: Option<&IOleInPlaceUIWindow>, fframewindow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnableModeless(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOleInPlaceActiveObject {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOleInPlaceActiveObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceActiveObject_Vtbl
    where
        Identity: IOleInPlaceActiveObject_Impl,
    {
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceActiveObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceActiveObject_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&lpmsg)).into()
        }
        unsafe extern "system" fn OnFrameWindowActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceActiveObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceActiveObject_Impl::OnFrameWindowActivate(this, core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn OnDocWindowActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceActiveObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceActiveObject_Impl::OnDocWindowActivate(this, core::mem::transmute_copy(&factivate)).into()
        }
        unsafe extern "system" fn ResizeBorder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: *mut core::ffi::c_void, fframewindow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceActiveObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceActiveObject_Impl::ResizeBorder(this, core::mem::transmute_copy(&prcborder), windows_core::from_raw_borrowed(&puiwindow), core::mem::transmute_copy(&fframewindow)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceActiveObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceActiveObject_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Identity, OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Identity, OFFSET>,
            ResizeBorder: ResizeBorder::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOleInPlaceFrame_Impl: Sized + IOleInPlaceUIWindow_Impl {
    fn InsertMenus(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::Result<()>;
    fn SetMenu(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn RemoveMenus(&self, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::Result<()>;
    fn SetStatusText(&self, pszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableModeless(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOleInPlaceFrame {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOleInPlaceFrame_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceFrame_Vtbl
    where
        Identity: IOleInPlaceFrame_Impl,
    {
        unsafe extern "system" fn InsertMenus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::InsertMenus(this, core::mem::transmute_copy(&hmenushared), core::mem::transmute_copy(&lpmenuwidths)).into()
        }
        unsafe extern "system" fn SetMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::SetMenu(this, core::mem::transmute_copy(&hmenushared), core::mem::transmute_copy(&holemenu), core::mem::transmute_copy(&hwndactiveobject)).into()
        }
        unsafe extern "system" fn RemoveMenus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::RemoveMenus(this, core::mem::transmute_copy(&hmenushared)).into()
        }
        unsafe extern "system" fn SetStatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::SetStatusText(this, core::mem::transmute(&pszstatustext)).into()
        }
        unsafe extern "system" fn EnableModeless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceFrame_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&lpmsg), core::mem::transmute_copy(&wid)).into()
        }
        Self {
            base__: IOleInPlaceUIWindow_Vtbl::new::<Identity, OFFSET>(),
            InsertMenus: InsertMenus::<Identity, OFFSET>,
            SetMenu: SetMenu::<Identity, OFFSET>,
            RemoveMenus: RemoveMenus::<Identity, OFFSET>,
            SetStatusText: SetStatusText::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID || iid == &<IOleInPlaceUIWindow as windows_core::Interface>::IID
    }
}
pub trait IOleInPlaceObject_Impl: Sized + IOleWindow_Impl {
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn UIDeactivate(&self) -> windows_core::Result<()>;
    fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn ReactivateAndUndo(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleInPlaceObject {}
impl IOleInPlaceObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceObject_Vtbl
    where
        Identity: IOleInPlaceObject_Impl,
    {
        unsafe extern "system" fn InPlaceDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceObject_Impl::InPlaceDeactivate(this).into()
        }
        unsafe extern "system" fn UIDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceObject_Impl::UIDeactivate(this).into()
        }
        unsafe extern "system" fn SetObjectRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceObject_Impl::SetObjectRects(this, core::mem::transmute_copy(&lprcposrect), core::mem::transmute_copy(&lprccliprect)).into()
        }
        unsafe extern "system" fn ReactivateAndUndo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceObject_Impl::ReactivateAndUndo(this).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            InPlaceDeactivate: InPlaceDeactivate::<Identity, OFFSET>,
            UIDeactivate: UIDeactivate::<Identity, OFFSET>,
            SetObjectRects: SetObjectRects::<Identity, OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObject as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
pub trait IOleInPlaceObjectWindowless_Impl: Sized + IOleInPlaceObject_Impl {
    fn OnWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::LRESULT>;
    fn GetDropTarget(&self) -> windows_core::Result<IDropTarget>;
}
impl windows_core::RuntimeName for IOleInPlaceObjectWindowless {}
impl IOleInPlaceObjectWindowless_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceObjectWindowless_Vtbl
    where
        Identity: IOleInPlaceObjectWindowless_Impl,
    {
        unsafe extern "system" fn OnWindowMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObjectWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleInPlaceObjectWindowless_Impl::OnWindowMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdroptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceObjectWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleInPlaceObjectWindowless_Impl::GetDropTarget(this) {
                Ok(ok__) => {
                    ppdroptarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleInPlaceObject_Vtbl::new::<Identity, OFFSET>(),
            OnWindowMessage: OnWindowMessage::<Identity, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObjectWindowless as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID || iid == &<IOleInPlaceObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOleInPlaceSite_Impl: Sized + IOleWindow_Impl {
    fn CanInPlaceActivate(&self) -> windows_core::Result<()>;
    fn OnInPlaceActivate(&self) -> windows_core::Result<()>;
    fn OnUIActivate(&self) -> windows_core::Result<()>;
    fn GetWindowContext(&self, ppframe: *mut Option<IOleInPlaceFrame>, ppdoc: *mut Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn Scroll(&self, scrollextant: &super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn OnUIDeactivate(&self, fundoable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnInPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn DiscardUndoState(&self) -> windows_core::Result<()>;
    fn DeactivateAndUndo(&self) -> windows_core::Result<()>;
    fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOleInPlaceSite {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOleInPlaceSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceSite_Vtbl
    where
        Identity: IOleInPlaceSite_Impl,
    {
        unsafe extern "system" fn CanInPlaceActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::CanInPlaceActivate(this).into()
        }
        unsafe extern "system" fn OnInPlaceActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::OnInPlaceActivate(this).into()
        }
        unsafe extern "system" fn OnUIActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::OnUIActivate(this).into()
        }
        unsafe extern "system" fn GetWindowContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppframe: *mut *mut core::ffi::c_void, ppdoc: *mut *mut core::ffi::c_void, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::GetWindowContext(this, core::mem::transmute_copy(&ppframe), core::mem::transmute_copy(&ppdoc), core::mem::transmute_copy(&lprcposrect), core::mem::transmute_copy(&lprccliprect), core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn Scroll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::Scroll(this, core::mem::transmute(&scrollextant)).into()
        }
        unsafe extern "system" fn OnUIDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::OnUIDeactivate(this, core::mem::transmute_copy(&fundoable)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::OnInPlaceDeactivate(this).into()
        }
        unsafe extern "system" fn DiscardUndoState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::DiscardUndoState(this).into()
        }
        unsafe extern "system" fn DeactivateAndUndo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::DeactivateAndUndo(this).into()
        }
        unsafe extern "system" fn OnPosRectChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSite_Impl::OnPosRectChange(this, core::mem::transmute_copy(&lprcposrect)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            CanInPlaceActivate: CanInPlaceActivate::<Identity, OFFSET>,
            OnInPlaceActivate: OnInPlaceActivate::<Identity, OFFSET>,
            OnUIActivate: OnUIActivate::<Identity, OFFSET>,
            GetWindowContext: GetWindowContext::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
            OnUIDeactivate: OnUIDeactivate::<Identity, OFFSET>,
            OnInPlaceDeactivate: OnInPlaceDeactivate::<Identity, OFFSET>,
            DiscardUndoState: DiscardUndoState::<Identity, OFFSET>,
            DeactivateAndUndo: DeactivateAndUndo::<Identity, OFFSET>,
            OnPosRectChange: OnPosRectChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSite as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOleInPlaceSiteEx_Impl: Sized + IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> windows_core::Result<()>;
    fn OnInPlaceDeactivateEx(&self, fnoredraw: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RequestUIActivate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOleInPlaceSiteEx {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOleInPlaceSiteEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceSiteEx_Vtbl
    where
        Identity: IOleInPlaceSiteEx_Impl,
    {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteEx_Impl::OnInPlaceActivateEx(this, core::mem::transmute_copy(&pfnoredraw), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteEx_Impl::OnInPlaceDeactivateEx(this, core::mem::transmute_copy(&fnoredraw)).into()
        }
        unsafe extern "system" fn RequestUIActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteEx_Impl::RequestUIActivate(this).into()
        }
        Self {
            base__: IOleInPlaceSite_Vtbl::new::<Identity, OFFSET>(),
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID || iid == &<IOleInPlaceSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteWindowless_Impl: Sized + IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(&self) -> windows_core::Result<()>;
    fn GetCapture(&self) -> windows_core::Result<()>;
    fn SetCapture(&self, fcapture: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFocus(&self) -> windows_core::Result<()>;
    fn SetFocus(&self, ffocus: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDC(&self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn ReleaseDC(&self, hdc: super::super::Graphics::Gdi::HDC) -> windows_core::Result<()>;
    fn InvalidateRect(&self, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn InvalidateRgn(&self, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn AdjustRect(&self, prc: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn OnDefWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IOleInPlaceSiteWindowless {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleInPlaceSiteWindowless_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceSiteWindowless_Vtbl
    where
        Identity: IOleInPlaceSiteWindowless_Impl,
    {
        unsafe extern "system" fn CanWindowlessActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::CanWindowlessActivate(this).into()
        }
        unsafe extern "system" fn GetCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::GetCapture(this).into()
        }
        unsafe extern "system" fn SetCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::SetCapture(this, core::mem::transmute_copy(&fcapture)).into()
        }
        unsafe extern "system" fn GetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::GetFocus(this).into()
        }
        unsafe extern "system" fn SetFocus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::SetFocus(this, core::mem::transmute_copy(&ffocus)).into()
        }
        unsafe extern "system" fn GetDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleInPlaceSiteWindowless_Impl::GetDC(this, core::mem::transmute_copy(&prect), core::mem::transmute_copy(&grfflags)) {
                Ok(ok__) => {
                    phdc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
        }
        unsafe extern "system" fn InvalidateRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::InvalidateRect(this, core::mem::transmute_copy(&prect), core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn InvalidateRgn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::InvalidateRgn(this, core::mem::transmute_copy(&hrgn), core::mem::transmute_copy(&ferase)).into()
        }
        unsafe extern "system" fn ScrollRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::ScrollRect(this, core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&prectscroll), core::mem::transmute_copy(&prectclip)).into()
        }
        unsafe extern "system" fn AdjustRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceSiteWindowless_Impl::AdjustRect(this, core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceSiteWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleInPlaceSiteWindowless_Impl::OnDefWindowMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                Ok(ok__) => {
                    plresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleInPlaceSiteEx_Vtbl::new::<Identity, OFFSET>(),
            CanWindowlessActivate: CanWindowlessActivate::<Identity, OFFSET>,
            GetCapture: GetCapture::<Identity, OFFSET>,
            SetCapture: SetCapture::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            InvalidateRect: InvalidateRect::<Identity, OFFSET>,
            InvalidateRgn: InvalidateRgn::<Identity, OFFSET>,
            ScrollRect: ScrollRect::<Identity, OFFSET>,
            AdjustRect: AdjustRect::<Identity, OFFSET>,
            OnDefWindowMessage: OnDefWindowMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSiteWindowless as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID || iid == &<IOleInPlaceSite as windows_core::Interface>::IID || iid == &<IOleInPlaceSiteEx as windows_core::Interface>::IID
    }
}
pub trait IOleInPlaceUIWindow_Impl: Sized + IOleWindow_Impl {
    fn GetBorder(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetActiveObject(&self, pactiveobject: Option<&IOleInPlaceActiveObject>, pszobjname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleInPlaceUIWindow {}
impl IOleInPlaceUIWindow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleInPlaceUIWindow_Vtbl
    where
        Identity: IOleInPlaceUIWindow_Impl,
    {
        unsafe extern "system" fn GetBorder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceUIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleInPlaceUIWindow_Impl::GetBorder(this) {
                Ok(ok__) => {
                    lprectborder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceUIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceUIWindow_Impl::RequestBorderSpace(this, core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetBorderSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceUIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceUIWindow_Impl::SetBorderSpace(this, core::mem::transmute_copy(&pborderwidths)).into()
        }
        unsafe extern "system" fn SetActiveObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactiveobject: *mut core::ffi::c_void, pszobjname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IOleInPlaceUIWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleInPlaceUIWindow_Impl::SetActiveObject(this, windows_core::from_raw_borrowed(&pactiveobject), core::mem::transmute(&pszobjname)).into()
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            GetBorder: GetBorder::<Identity, OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Identity, OFFSET>,
            SetBorderSpace: SetBorderSpace::<Identity, OFFSET>,
            SetActiveObject: SetActiveObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleItemContainer_Impl: Sized + IOleContainer_Impl {
    fn GetObject(&self, pszitem: &windows_core::PCWSTR, dwspeedneeded: u32, pbc: Option<&super::Com::IBindCtx>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectStorage(&self, pszitem: &windows_core::PCWSTR, pbc: Option<&super::Com::IBindCtx>, riid: *const windows_core::GUID, ppvstorage: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsRunning(&self, pszitem: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleItemContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IOleItemContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleItemContainer_Vtbl
    where
        Identity: IOleItemContainer_Impl,
    {
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR, dwspeedneeded: u32, pbc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleItemContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleItemContainer_Impl::GetObject(this, core::mem::transmute(&pszitem), core::mem::transmute_copy(&dwspeedneeded), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetObjectStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleItemContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleItemContainer_Impl::GetObjectStorage(this, core::mem::transmute(&pszitem), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvstorage)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IOleItemContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleItemContainer_Impl::IsRunning(this, core::mem::transmute(&pszitem)).into()
        }
        Self {
            base__: IOleContainer_Vtbl::new::<Identity, OFFSET>(),
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectStorage: GetObjectStorage::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleItemContainer as windows_core::Interface>::IID || iid == &<IParseDisplayName as windows_core::Interface>::IID || iid == &<IOleContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOleLink_Impl: Sized {
    fn SetUpdateOptions(&self, dwupdateopt: u32) -> windows_core::Result<()>;
    fn GetUpdateOptions(&self) -> windows_core::Result<u32>;
    fn SetSourceMoniker(&self, pmk: Option<&super::Com::IMoniker>, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetSourceMoniker(&self) -> windows_core::Result<super::Com::IMoniker>;
    fn SetSourceDisplayName(&self, pszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSourceDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn BindToSource(&self, bindflags: u32, pbc: Option<&super::Com::IBindCtx>) -> windows_core::Result<()>;
    fn BindIfRunning(&self) -> windows_core::Result<()>;
    fn GetBoundSource(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn UnbindSource(&self) -> windows_core::Result<()>;
    fn Update(&self, pbc: Option<&super::Com::IBindCtx>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOleLink {}
#[cfg(feature = "Win32_System_Com")]
impl IOleLink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleLink_Vtbl
    where
        Identity: IOleLink_Impl,
    {
        unsafe extern "system" fn SetUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwupdateopt: u32) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::SetUpdateOptions(this, core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwupdateopt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleLink_Impl::GetUpdateOptions(this) {
                Ok(ok__) => {
                    pdwupdateopt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::SetSourceMoniker(this, windows_core::from_raw_borrowed(&pmk), core::mem::transmute_copy(&rclsid)).into()
        }
        unsafe extern "system" fn GetSourceMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleLink_Impl::GetSourceMoniker(this) {
                Ok(ok__) => {
                    ppmk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::SetSourceDisplayName(this, core::mem::transmute(&pszstatustext)).into()
        }
        unsafe extern "system" fn GetSourceDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleLink_Impl::GetSourceDisplayName(this) {
                Ok(ok__) => {
                    ppszdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bindflags: u32, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::BindToSource(this, core::mem::transmute_copy(&bindflags), windows_core::from_raw_borrowed(&pbc)).into()
        }
        unsafe extern "system" fn BindIfRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::BindIfRunning(this).into()
        }
        unsafe extern "system" fn GetBoundSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleLink_Impl::GetBoundSource(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::UnbindSource(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleLink_Impl::Update(this, windows_core::from_raw_borrowed(&pbc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUpdateOptions: SetUpdateOptions::<Identity, OFFSET>,
            GetUpdateOptions: GetUpdateOptions::<Identity, OFFSET>,
            SetSourceMoniker: SetSourceMoniker::<Identity, OFFSET>,
            GetSourceMoniker: GetSourceMoniker::<Identity, OFFSET>,
            SetSourceDisplayName: SetSourceDisplayName::<Identity, OFFSET>,
            GetSourceDisplayName: GetSourceDisplayName::<Identity, OFFSET>,
            BindToSource: BindToSource::<Identity, OFFSET>,
            BindIfRunning: BindIfRunning::<Identity, OFFSET>,
            GetBoundSource: GetBoundSource::<Identity, OFFSET>,
            UnbindSource: UnbindSource::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleLink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleObject_Impl: Sized {
    fn SetClientSite(&self, pclientsite: Option<&IOleClientSite>) -> windows_core::Result<()>;
    fn GetClientSite(&self) -> windows_core::Result<IOleClientSite>;
    fn SetHostNames(&self, szcontainerapp: &windows_core::PCWSTR, szcontainerobj: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self, dwsaveoption: &OLECLOSE) -> windows_core::Result<()>;
    fn SetMoniker(&self, dwwhichmoniker: &OLEWHICHMK, pmk: Option<&super::Com::IMoniker>) -> windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> windows_core::Result<super::Com::IMoniker>;
    fn InitFromData(&self, pdataobject: Option<&super::Com::IDataObject>, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> windows_core::Result<()>;
    fn GetClipboardData(&self, dwreserved: u32) -> windows_core::Result<super::Com::IDataObject>;
    fn DoVerb(&self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: Option<&IOleClientSite>, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn EnumVerbs(&self) -> windows_core::Result<IEnumOLEVERB>;
    fn Update(&self) -> windows_core::Result<()>;
    fn IsUpToDate(&self) -> windows_core::Result<()>;
    fn GetUserClassID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetUserType(&self, dwformoftype: &USERCLASSTYPE) -> windows_core::Result<windows_core::PWSTR>;
    fn SetExtent(&self, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT) -> windows_core::Result<super::super::Foundation::SIZE>;
    fn Advise(&self, padvsink: Option<&super::Com::IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<super::Com::IEnumSTATDATA>;
    fn GetMiscStatus(&self, dwaspect: super::Com::DVASPECT) -> windows_core::Result<OLEMISC>;
    fn SetColorScheme(&self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IOleObject {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IOleObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleObject_Vtbl
    where
        Identity: IOleObject_Impl,
    {
        unsafe extern "system" fn SetClientSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientsite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::SetClientSite(this, windows_core::from_raw_borrowed(&pclientsite)).into()
        }
        unsafe extern "system" fn GetClientSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclientsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetClientSite(this) {
                Ok(ok__) => {
                    ppclientsite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcontainerapp: windows_core::PCWSTR, szcontainerobj: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::SetHostNames(this, core::mem::transmute(&szcontainerapp), core::mem::transmute(&szcontainerobj)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsaveoption: u32) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::Close(this, core::mem::transmute(&dwsaveoption)).into()
        }
        unsafe extern "system" fn SetMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhichmoniker: u32, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::SetMoniker(this, core::mem::transmute(&dwwhichmoniker), windows_core::from_raw_borrowed(&pmk)).into()
        }
        unsafe extern "system" fn GetMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetMoniker(this, core::mem::transmute(&dwassign), core::mem::transmute(&dwwhichmoniker)) {
                Ok(ok__) => {
                    ppmk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitFromData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::InitFromData(this, windows_core::from_raw_borrowed(&pdataobject), core::mem::transmute_copy(&fcreation), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetClipboardData(this, core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppdataobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoVerb<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: *mut core::ffi::c_void, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::DoVerb(this, core::mem::transmute_copy(&iverb), core::mem::transmute_copy(&lpmsg), windows_core::from_raw_borrowed(&pactivesite), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lprcposrect)).into()
        }
        unsafe extern "system" fn EnumVerbs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumoleverb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::EnumVerbs(this) {
                Ok(ok__) => {
                    ppenumoleverb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::Update(this).into()
        }
        unsafe extern "system" fn IsUpToDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::IsUpToDate(this).into()
        }
        unsafe extern "system" fn GetUserClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetUserClassID(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwformoftype: u32, pszusertype: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetUserType(this, core::mem::transmute(&dwformoftype)) {
                Ok(ok__) => {
                    pszusertype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::SetExtent(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetExtent(this, core::mem::transmute_copy(&dwdrawaspect)) {
                Ok(ok__) => {
                    psizel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvsink: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::Advise(this, windows_core::from_raw_borrowed(&padvsink)) {
                Ok(ok__) => {
                    pdwconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::EnumAdvise(this) {
                Ok(ok__) => {
                    ppenumadvise.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMiscStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::Com::DVASPECT, pdwstatus: *mut OLEMISC) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleObject_Impl::GetMiscStatus(this, core::mem::transmute_copy(&dwaspect)) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorScheme<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::HRESULT
        where
            Identity: IOleObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleObject_Impl::SetColorScheme(this, core::mem::transmute_copy(&plogpal)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientSite: SetClientSite::<Identity, OFFSET>,
            GetClientSite: GetClientSite::<Identity, OFFSET>,
            SetHostNames: SetHostNames::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetMoniker: SetMoniker::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            InitFromData: InitFromData::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            DoVerb: DoVerb::<Identity, OFFSET>,
            EnumVerbs: EnumVerbs::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            IsUpToDate: IsUpToDate::<Identity, OFFSET>,
            GetUserClassID: GetUserClassID::<Identity, OFFSET>,
            GetUserType: GetUserType::<Identity, OFFSET>,
            SetExtent: SetExtent::<Identity, OFFSET>,
            GetExtent: GetExtent::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            GetMiscStatus: GetMiscStatus::<Identity, OFFSET>,
            SetColorScheme: SetColorScheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleObject as windows_core::Interface>::IID
    }
}
pub trait IOleParentUndoUnit_Impl: Sized + IOleUndoUnit_Impl {
    fn Open(&self, ppuu: Option<&IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Add(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn FindUnit(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetParentState(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IOleParentUndoUnit {}
impl IOleParentUndoUnit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleParentUndoUnit_Vtbl
    where
        Identity: IOleParentUndoUnit_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleParentUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleParentUndoUnit_Impl::Open(this, windows_core::from_raw_borrowed(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleParentUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleParentUndoUnit_Impl::Close(this, windows_core::from_raw_borrowed(&ppuu), core::mem::transmute_copy(&fcommit)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleParentUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleParentUndoUnit_Impl::Add(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn FindUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleParentUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleParentUndoUnit_Impl::FindUnit(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn GetParentState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleParentUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleParentUndoUnit_Impl::GetParentState(this) {
                Ok(ok__) => {
                    pdwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IOleUndoUnit_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            FindUnit: FindUnit::<Identity, OFFSET>,
            GetParentState: GetParentState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleParentUndoUnit as windows_core::Interface>::IID || iid == &<IOleUndoUnit as windows_core::Interface>::IID
    }
}
pub trait IOleUILinkContainerA_Impl: Sized {
    fn GetNextLink(&self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> windows_core::Result<()>;
    fn GetLinkUpdateOptions(&self, dwlink: u32) -> windows_core::Result<u32>;
    fn SetLinkSource(&self, dwlink: u32, lpszdisplayname: &windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut windows_core::PSTR, lplpszshortlinktype: *mut windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OpenLinkSource(&self, dwlink: u32) -> windows_core::Result<()>;
    fn UpdateLink(&self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CancelLink(&self, dwlink: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUILinkContainerA {}
impl IOleUILinkContainerA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUILinkContainerA_Vtbl
    where
        Identity: IOleUILinkContainerA_Impl,
    {
        unsafe extern "system" fn GetNextLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> u32
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::GetNextLink(this, core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::SetLinkUpdateOptions(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUILinkContainerA_Impl::GetLinkUpdateOptions(this, core::mem::transmute_copy(&dwlink)) {
                Ok(ok__) => {
                    lpdwupdateopt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lpszdisplayname: windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::SetLinkSource(this, core::mem::transmute_copy(&dwlink), core::mem::transmute(&lpszdisplayname), core::mem::transmute_copy(&lenfilename), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut windows_core::PSTR, lplpszshortlinktype: *mut windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::GetLinkSource(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&lplpszdisplayname), core::mem::transmute_copy(&lplenfilename), core::mem::transmute_copy(&lplpszfulllinktype), core::mem::transmute_copy(&lplpszshortlinktype), core::mem::transmute_copy(&lpfsourceavailable), core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::OpenLinkSource(this, core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::UpdateLink(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&ferrormessage), core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerA_Impl::CancelLink(this, core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNextLink: GetNextLink::<Identity, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, OFFSET>,
            UpdateLink: UpdateLink::<Identity, OFFSET>,
            CancelLink: CancelLink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUILinkContainerA as windows_core::Interface>::IID
    }
}
pub trait IOleUILinkContainerW_Impl: Sized {
    fn GetNextLink(&self, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> windows_core::Result<()>;
    fn GetLinkUpdateOptions(&self, dwlink: u32) -> windows_core::Result<u32>;
    fn SetLinkSource(&self, dwlink: u32, lpszdisplayname: &windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: *mut windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut windows_core::PWSTR, lplpszshortlinktype: *mut windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OpenLinkSource(&self, dwlink: u32) -> windows_core::Result<()>;
    fn UpdateLink(&self, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CancelLink(&self, dwlink: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUILinkContainerW {}
impl IOleUILinkContainerW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUILinkContainerW_Vtbl
    where
        Identity: IOleUILinkContainerW_Impl,
    {
        unsafe extern "system" fn GetNextLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> u32
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::GetNextLink(this, core::mem::transmute_copy(&dwlink))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::SetLinkUpdateOptions(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&dwupdateopt)).into()
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUILinkContainerW_Impl::GetLinkUpdateOptions(this, core::mem::transmute_copy(&dwlink)) {
                Ok(ok__) => {
                    lpdwupdateopt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lpszdisplayname: windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::SetLinkSource(this, core::mem::transmute_copy(&dwlink), core::mem::transmute(&lpszdisplayname), core::mem::transmute_copy(&lenfilename), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&fvalidatesource)).into()
        }
        unsafe extern "system" fn GetLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut windows_core::PWSTR, lplpszshortlinktype: *mut windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::GetLinkSource(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&lplpszdisplayname), core::mem::transmute_copy(&lplenfilename), core::mem::transmute_copy(&lplpszfulllinktype), core::mem::transmute_copy(&lplpszshortlinktype), core::mem::transmute_copy(&lpfsourceavailable), core::mem::transmute_copy(&lpfisselected)).into()
        }
        unsafe extern "system" fn OpenLinkSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::OpenLinkSource(this, core::mem::transmute_copy(&dwlink)).into()
        }
        unsafe extern "system" fn UpdateLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::UpdateLink(this, core::mem::transmute_copy(&dwlink), core::mem::transmute_copy(&ferrormessage), core::mem::transmute_copy(&freserved)).into()
        }
        unsafe extern "system" fn CancelLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32) -> windows_core::HRESULT
        where
            Identity: IOleUILinkContainerW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUILinkContainerW_Impl::CancelLink(this, core::mem::transmute_copy(&dwlink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNextLink: GetNextLink::<Identity, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, OFFSET>,
            UpdateLink: UpdateLink::<Identity, OFFSET>,
            CancelLink: CancelLink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUILinkContainerW as windows_core::Interface>::IID
    }
}
pub trait IOleUILinkInfoA_Impl: Sized + IOleUILinkContainerA_Impl {
    fn GetLastUpdate(&self, dwlink: u32) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl windows_core::RuntimeName for IOleUILinkInfoA {}
impl IOleUILinkInfoA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUILinkInfoA_Vtbl
    where
        Identity: IOleUILinkInfoA_Impl,
    {
        unsafe extern "system" fn GetLastUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IOleUILinkInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUILinkInfoA_Impl::GetLastUpdate(this, core::mem::transmute_copy(&dwlink)) {
                Ok(ok__) => {
                    lplastupdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IOleUILinkContainerA_Vtbl::new::<Identity, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUILinkInfoA as windows_core::Interface>::IID || iid == &<IOleUILinkContainerA as windows_core::Interface>::IID
    }
}
pub trait IOleUILinkInfoW_Impl: Sized + IOleUILinkContainerW_Impl {
    fn GetLastUpdate(&self, dwlink: u32) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl windows_core::RuntimeName for IOleUILinkInfoW {}
impl IOleUILinkInfoW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUILinkInfoW_Vtbl
    where
        Identity: IOleUILinkInfoW_Impl,
    {
        unsafe extern "system" fn GetLastUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IOleUILinkInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUILinkInfoW_Impl::GetLastUpdate(this, core::mem::transmute_copy(&dwlink)) {
                Ok(ok__) => {
                    lplastupdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IOleUILinkContainerW_Vtbl::new::<Identity, OFFSET>(), GetLastUpdate: GetLastUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUILinkInfoW as windows_core::Interface>::IID || iid == &<IOleUILinkContainerW as windows_core::Interface>::IID
    }
}
pub trait IOleUIObjInfoA_Impl: Sized {
    fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut windows_core::PSTR, lplpsztype: *mut windows_core::PSTR, lplpszshorttype: *mut windows_core::PSTR, lplpszlocation: *mut windows_core::PSTR) -> windows_core::Result<()>;
    fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: *mut u32) -> windows_core::Result<()>;
    fn ConvertObject(&self, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetViewInfo(&self, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> windows_core::Result<()>;
    fn SetViewInfo(&self, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUIObjInfoA {}
impl IOleUIObjInfoA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUIObjInfoA_Vtbl
    where
        Identity: IOleUIObjInfoA_Impl,
    {
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut windows_core::PSTR, lplpsztype: *mut windows_core::PSTR, lplpszshorttype: *mut windows_core::PSTR, lplpszlocation: *mut windows_core::PSTR) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoA_Impl::GetObjectInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&lpdwobjsize), core::mem::transmute_copy(&lplpszlabel), core::mem::transmute_copy(&lplpsztype), core::mem::transmute_copy(&lplpszshorttype), core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoA_Impl::GetConvertInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&lpclassid), core::mem::transmute_copy(&lpwformat), core::mem::transmute_copy(&lpconvertdefaultclassid), core::mem::transmute_copy(&lplpclsidexclude), core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoA_Impl::ConvertObject(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoA_Impl::GetViewInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&phmetapict), core::mem::transmute_copy(&pdvaspect), core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoA_Impl::SetViewInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&hmetapict), core::mem::transmute_copy(&dvaspect), core::mem::transmute_copy(&ncurrentscale), core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, OFFSET>,
            ConvertObject: ConvertObject::<Identity, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUIObjInfoA as windows_core::Interface>::IID
    }
}
pub trait IOleUIObjInfoW_Impl: Sized {
    fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut windows_core::PWSTR, lplpsztype: *mut windows_core::PWSTR, lplpszshorttype: *mut windows_core::PWSTR, lplpszlocation: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: *mut u32) -> windows_core::Result<()>;
    fn ConvertObject(&self, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetViewInfo(&self, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> windows_core::Result<()>;
    fn SetViewInfo(&self, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUIObjInfoW {}
impl IOleUIObjInfoW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUIObjInfoW_Vtbl
    where
        Identity: IOleUIObjInfoW_Impl,
    {
        unsafe extern "system" fn GetObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut windows_core::PWSTR, lplpsztype: *mut windows_core::PWSTR, lplpszshorttype: *mut windows_core::PWSTR, lplpszlocation: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoW_Impl::GetObjectInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&lpdwobjsize), core::mem::transmute_copy(&lplpszlabel), core::mem::transmute_copy(&lplpsztype), core::mem::transmute_copy(&lplpszshorttype), core::mem::transmute_copy(&lplpszlocation)).into()
        }
        unsafe extern "system" fn GetConvertInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoW_Impl::GetConvertInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&lpclassid), core::mem::transmute_copy(&lpwformat), core::mem::transmute_copy(&lpconvertdefaultclassid), core::mem::transmute_copy(&lplpclsidexclude), core::mem::transmute_copy(&lpcclsidexclude)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoW_Impl::ConvertObject(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&clsidnew)).into()
        }
        unsafe extern "system" fn GetViewInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoW_Impl::GetViewInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&phmetapict), core::mem::transmute_copy(&pdvaspect), core::mem::transmute_copy(&pncurrentscale)).into()
        }
        unsafe extern "system" fn SetViewInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUIObjInfoW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUIObjInfoW_Impl::SetViewInfo(this, core::mem::transmute_copy(&dwobject), core::mem::transmute_copy(&hmetapict), core::mem::transmute_copy(&dvaspect), core::mem::transmute_copy(&ncurrentscale), core::mem::transmute_copy(&brelativetoorig)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, OFFSET>,
            ConvertObject: ConvertObject::<Identity, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUIObjInfoW as windows_core::Interface>::IID
    }
}
pub trait IOleUndoManager_Impl: Sized {
    fn Open(&self, ppuu: Option<&IOleParentUndoUnit>) -> windows_core::Result<()>;
    fn Close(&self, ppuu: Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> windows_core::HRESULT;
    fn Add(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn GetOpenParentState(&self) -> windows_core::Result<u32>;
    fn DiscardFrom(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn UndoTo(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn RedoTo(&self, puu: Option<&IOleUndoUnit>) -> windows_core::Result<()>;
    fn EnumUndoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLastRedoDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUndoManager {}
impl IOleUndoManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUndoManager_Vtbl
    where
        Identity: IOleUndoManager_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::Open(this, windows_core::from_raw_borrowed(&ppuu)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppuu: *mut core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::Close(this, windows_core::from_raw_borrowed(&ppuu), core::mem::transmute_copy(&fcommit))
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::Add(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn GetOpenParentState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoManager_Impl::GetOpenParentState(this) {
                Ok(ok__) => {
                    pdwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::DiscardFrom(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn UndoTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::UndoTo(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn RedoTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puu: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::RedoTo(this, windows_core::from_raw_borrowed(&puu)).into()
        }
        unsafe extern "system" fn EnumUndoable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoManager_Impl::EnumUndoable(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRedoable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoManager_Impl::EnumRedoable(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUndoDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoManager_Impl::GetLastUndoDescription(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastRedoDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoManager_Impl::GetLastRedoDescription(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleUndoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoManager_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            GetOpenParentState: GetOpenParentState::<Identity, OFFSET>,
            DiscardFrom: DiscardFrom::<Identity, OFFSET>,
            UndoTo: UndoTo::<Identity, OFFSET>,
            RedoTo: RedoTo::<Identity, OFFSET>,
            EnumUndoable: EnumUndoable::<Identity, OFFSET>,
            EnumRedoable: EnumRedoable::<Identity, OFFSET>,
            GetLastUndoDescription: GetLastUndoDescription::<Identity, OFFSET>,
            GetLastRedoDescription: GetLastRedoDescription::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUndoManager as windows_core::Interface>::IID
    }
}
pub trait IOleUndoUnit_Impl: Sized {
    fn Do(&self, pundomanager: Option<&IOleUndoManager>) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUnitType(&self, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::Result<()>;
    fn OnNextAdd(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleUndoUnit {}
impl IOleUndoUnit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleUndoUnit_Vtbl
    where
        Identity: IOleUndoUnit_Impl,
    {
        unsafe extern "system" fn Do<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pundomanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoUnit_Impl::Do(this, windows_core::from_raw_borrowed(&pundomanager)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IOleUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleUndoUnit_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnitType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IOleUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoUnit_Impl::GetUnitType(this, core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn OnNextAdd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOleUndoUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleUndoUnit_Impl::OnNextAdd(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Do: Do::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetUnitType: GetUnitType::<Identity, OFFSET>,
            OnNextAdd: OnNextAdd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleUndoUnit as windows_core::Interface>::IID
    }
}
pub trait IOleWindow_Impl: Sized {
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IOleWindow {}
impl IOleWindow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOleWindow_Vtbl
    where
        Identity: IOleWindow_Impl,
    {
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IOleWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOleWindow_Impl::GetWindow(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOleWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOleWindow_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IParseDisplayName_Impl: Sized {
    fn ParseDisplayName(&self, pbc: Option<&super::Com::IBindCtx>, pszdisplayname: &windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut Option<super::Com::IMoniker>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IParseDisplayName {}
#[cfg(feature = "Win32_System_Com")]
impl IParseDisplayName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IParseDisplayName_Vtbl
    where
        Identity: IParseDisplayName_Impl,
    {
        unsafe extern "system" fn ParseDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pszdisplayname: windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IParseDisplayName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IParseDisplayName_Impl::ParseDisplayName(this, windows_core::from_raw_borrowed(&pbc), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&ppmkout)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseDisplayName: ParseDisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IParseDisplayName as windows_core::Interface>::IID
    }
}
pub trait IPerPropertyBrowsing_Impl: Sized {
    fn GetDisplayString(&self, dispid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: i32) -> windows_core::Result<windows_core::GUID>;
    fn GetPredefinedStrings(&self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::Result<()>;
    fn GetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for IPerPropertyBrowsing {}
impl IPerPropertyBrowsing_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPerPropertyBrowsing_Vtbl
    where
        Identity: IPerPropertyBrowsing_Impl,
    {
        unsafe extern "system" fn GetDisplayString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerPropertyBrowsing_Impl::GetDisplayString(this, core::mem::transmute_copy(&dispid)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerPropertyBrowsing_Impl::MapPropertyToPage(this, core::mem::transmute_copy(&dispid)) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerPropertyBrowsing_Impl::GetPredefinedStrings(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pcastringsout), core::mem::transmute_copy(&pcacookiesout)).into()
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IPerPropertyBrowsing_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerPropertyBrowsing_Impl::GetPredefinedValue(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&dwcookie)) {
                Ok(ok__) => {
                    pvarout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IPersistPropertyBag_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: Option<&super::Com::StructuredStorage::IPropertyBag>, perrorlog: Option<&super::Com::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: Option<&super::Com::StructuredStorage::IPropertyBag>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IPersistPropertyBag {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IPersistPropertyBag_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistPropertyBag_Vtbl
    where
        Identity: IPersistPropertyBag_Impl,
    {
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag_Impl::InitNew(this).into()
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag_Impl::Load(this, windows_core::from_raw_borrowed(&ppropbag), windows_core::from_raw_borrowed(&perrorlog)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag_Impl::Save(this, windows_core::from_raw_borrowed(&ppropbag), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        Self {
            base__: super::Com::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag as windows_core::Interface>::IID || iid == &<super::Com::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IPersistPropertyBag2_Impl: Sized + super::Com::IPersist_Impl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn Load(&self, ppropbag: Option<&super::Com::StructuredStorage::IPropertyBag2>, perrlog: Option<&super::Com::IErrorLog>) -> windows_core::Result<()>;
    fn Save(&self, ppropbag: Option<&super::Com::StructuredStorage::IPropertyBag2>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsDirty(&self) -> windows_core::HRESULT;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IPersistPropertyBag2 {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IPersistPropertyBag2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistPropertyBag2_Vtbl
    where
        Identity: IPersistPropertyBag2_Impl,
    {
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag2_Impl::InitNew(this).into()
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, perrlog: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag2_Impl::Load(this, windows_core::from_raw_borrowed(&ppropbag), windows_core::from_raw_borrowed(&perrlog)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropbag: *mut core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag2_Impl::Save(this, windows_core::from_raw_borrowed(&ppropbag), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&fsaveallproperties)).into()
        }
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistPropertyBag2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistPropertyBag2_Impl::IsDirty(this)
        }
        Self {
            base__: super::Com::IPersist_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistPropertyBag2 as windows_core::Interface>::IID || iid == &<super::Com::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture_Impl: Sized {
    fn Handle(&self) -> windows_core::Result<OLE_HANDLE>;
    fn hPal(&self) -> windows_core::Result<OLE_HANDLE>;
    fn Type(&self) -> windows_core::Result<PICTYPE>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Render(&self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: OLE_HANDLE) -> windows_core::Result<()>;
    fn CurDC(&self) -> windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::Result<()>;
    fn KeepOriginalFormat(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&self, keep: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> windows_core::Result<i32>;
    fn Attributes(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPicture {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPicture_Vtbl
    where
        Identity: IPicture_Impl,
    {
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpal: *mut OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::hPal(this) {
                Ok(ok__) => {
                    phpal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut PICTYPE) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::Type(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::Width(this) {
                Ok(ok__) => {
                    pwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::Height(this) {
                Ok(ok__) => {
                    pheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture_Impl::Render(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&xsrc), core::mem::transmute_copy(&ysrc), core::mem::transmute_copy(&cxsrc), core::mem::transmute_copy(&cysrc), core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpal: OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture_Impl::set_hPal(this, core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::CurDC(this) {
                Ok(ok__) => {
                    phdc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture_Impl::SelectPicture(this, core::mem::transmute_copy(&hdcin), core::mem::transmute_copy(&phdcout), core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::KeepOriginalFormat(this) {
                Ok(ok__) => {
                    pkeep.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keep: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture_Impl::SetKeepOriginalFormat(this, core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture_Impl::PictureChanged(this).into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::SaveAsFile(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&fsavememcopy)) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture_Impl::Attributes(this) {
                Ok(ok__) => {
                    pdwattr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, OFFSET>,
            hPal: hPal::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            set_hPal: set_hPal::<Identity, OFFSET>,
            CurDC: CurDC::<Identity, OFFSET>,
            SelectPicture: SelectPicture::<Identity, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, OFFSET>,
            PictureChanged: PictureChanged::<Identity, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPicture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture2_Impl: Sized {
    fn Handle(&self) -> windows_core::Result<usize>;
    fn hPal(&self) -> windows_core::Result<usize>;
    fn Type(&self) -> windows_core::Result<i16>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Render(&self, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn set_hPal(&self, hpal: usize) -> windows_core::Result<()>;
    fn CurDC(&self) -> windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(&self, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> windows_core::Result<()>;
    fn KeepOriginalFormat(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(&self, keep: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn PictureChanged(&self) -> windows_core::Result<()>;
    fn SaveAsFile(&self, pstream: Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> windows_core::Result<i32>;
    fn Attributes(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IPicture2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IPicture2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPicture2_Vtbl
    where
        Identity: IPicture2_Impl,
    {
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hPal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phpal: *mut usize) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::hPal(this) {
                Ok(ok__) => {
                    phpal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i16) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::Type(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::Width(this) {
                Ok(ok__) => {
                    pwidth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::Height(this) {
                Ok(ok__) => {
                    pheight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Render<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture2_Impl::Render(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&xsrc), core::mem::transmute_copy(&ysrc), core::mem::transmute_copy(&cxsrc), core::mem::transmute_copy(&cysrc), core::mem::transmute_copy(&prcwbounds)).into()
        }
        unsafe extern "system" fn set_hPal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpal: usize) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture2_Impl::set_hPal(this, core::mem::transmute_copy(&hpal)).into()
        }
        unsafe extern "system" fn CurDC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::CurDC(this) {
                Ok(ok__) => {
                    phdc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectPicture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture2_Impl::SelectPicture(this, core::mem::transmute_copy(&hdcin), core::mem::transmute_copy(&phdcout), core::mem::transmute_copy(&phbmpout)).into()
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::KeepOriginalFormat(this) {
                Ok(ok__) => {
                    pkeep.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keep: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture2_Impl::SetKeepOriginalFormat(this, core::mem::transmute_copy(&keep)).into()
        }
        unsafe extern "system" fn PictureChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPicture2_Impl::PictureChanged(this).into()
        }
        unsafe extern "system" fn SaveAsFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::SaveAsFile(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&fsavememcopy)) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattr: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPicture2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPicture2_Impl::Attributes(this) {
                Ok(ok__) => {
                    pdwattr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Handle: Handle::<Identity, OFFSET>,
            hPal: hPal::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            set_hPal: set_hPal::<Identity, OFFSET>,
            CurDC: CurDC::<Identity, OFFSET>,
            SelectPicture: SelectPicture::<Identity, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, OFFSET>,
            PictureChanged: PictureChanged::<Identity, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPicture2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPictureDisp_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPictureDisp {}
#[cfg(feature = "Win32_System_Com")]
impl IPictureDisp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPictureDisp_Vtbl
    where
        Identity: IPictureDisp_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPictureDisp as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IPointerInactive_Impl: Sized {
    fn GetActivationPolicy(&self) -> windows_core::Result<POINTERINACTIVE>;
    fn OnInactiveMouseMove(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::Result<()>;
    fn OnInactiveSetCursor(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPointerInactive {}
impl IPointerInactive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPointerInactive_Vtbl
    where
        Identity: IPointerInactive_Impl,
    {
        unsafe extern "system" fn GetActivationPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpolicy: *mut POINTERINACTIVE) -> windows_core::HRESULT
        where
            Identity: IPointerInactive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPointerInactive_Impl::GetActivationPolicy(this) {
                Ok(ok__) => {
                    pdwpolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInactiveMouseMove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::HRESULT
        where
            Identity: IPointerInactive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPointerInactive_Impl::OnInactiveMouseMove(this, core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&grfkeystate)).into()
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPointerInactive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPointerInactive_Impl::OnInactiveSetCursor(this, core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&dwmousemsg), core::mem::transmute_copy(&fsetalways)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActivationPolicy: GetActivationPolicy::<Identity, OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Identity, OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerInactive as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPrint_Impl: Sized {
    fn SetInitialPageNum(&self, nfirstpage: i32) -> windows_core::Result<()>;
    fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::Result<()>;
    fn Print(&self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: Option<&IContinueCallback>, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IPrint {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IPrint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrint_Vtbl
    where
        Identity: IPrint_Impl,
    {
        unsafe extern "system" fn SetInitialPageNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nfirstpage: i32) -> windows_core::HRESULT
        where
            Identity: IPrint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrint_Impl::SetInitialPageNum(this, core::mem::transmute_copy(&nfirstpage)).into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrint_Impl::GetPageInfo(this, core::mem::transmute_copy(&pnfirstpage), core::mem::transmute_copy(&pcpages)).into()
        }
        unsafe extern "system" fn Print<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: *mut core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPrint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrint_Impl::Print(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&pptd), core::mem::transmute_copy(&pppageset), core::mem::transmute_copy(&pstgmoptions), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&nfirstpage), core::mem::transmute_copy(&pcpagesprinted), core::mem::transmute_copy(&pnlastpage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialPageNum: SetInitialPageNum::<Identity, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, OFFSET>,
            Print: Print::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrint as windows_core::Interface>::IID
    }
}
pub trait IPropertyNotifySink_Impl: Sized {
    fn OnChanged(&self, dispid: i32) -> windows_core::Result<()>;
    fn OnRequestEdit(&self, dispid: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyNotifySink {}
impl IPropertyNotifySink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertyNotifySink_Vtbl
    where
        Identity: IPropertyNotifySink_Impl,
    {
        unsafe extern "system" fn OnChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32) -> windows_core::HRESULT
        where
            Identity: IPropertyNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyNotifySink_Impl::OnChanged(this, core::mem::transmute_copy(&dispid)).into()
        }
        unsafe extern "system" fn OnRequestEdit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32) -> windows_core::HRESULT
        where
            Identity: IPropertyNotifySink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyNotifySink_Impl::OnRequestEdit(this, core::mem::transmute_copy(&dispid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnChanged: OnChanged::<Identity, OFFSET>,
            OnRequestEdit: OnRequestEdit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IPropertyPage_Impl: Sized {
    fn SetPageSite(&self, ppagesite: Option<&IPropertyPageSite>) -> windows_core::Result<()>;
    fn Activate(&self, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::Result<()>;
    fn SetObjects(&self, cobjects: u32, ppunk: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Show(&self, ncmdshow: u32) -> windows_core::Result<()>;
    fn Move(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn IsPageDirty(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Help(&self, pszhelpdir: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IPropertyPage {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IPropertyPage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertyPage_Vtbl
    where
        Identity: IPropertyPage_Impl,
    {
        unsafe extern "system" fn SetPageSite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppagesite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::SetPageSite(this, windows_core::from_raw_borrowed(&ppagesite)).into()
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Activate(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&bmodal)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn GetPageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::GetPageInfo(this, core::mem::transmute_copy(&ppageinfo)).into()
        }
        unsafe extern "system" fn SetObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjects: u32, ppunk: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::SetObjects(this, core::mem::transmute_copy(&cobjects), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Show<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncmdshow: u32) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Show(this, core::mem::transmute_copy(&ncmdshow)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Move(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn IsPageDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::IsPageDirty(this).into()
        }
        unsafe extern "system" fn Apply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Apply(this).into()
        }
        unsafe extern "system" fn Help<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelpdir: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::Help(this, core::mem::transmute(&pszhelpdir)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT
        where
            Identity: IPropertyPage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPageSite: SetPageSite::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, OFFSET>,
            SetObjects: SetObjects::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            IsPageDirty: IsPageDirty::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            Help: Help::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IPropertyPage2_Impl: Sized + IPropertyPage_Impl {
    fn EditProperty(&self, dispid: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IPropertyPage2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IPropertyPage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertyPage2_Vtbl
    where
        Identity: IPropertyPage2_Impl,
    {
        unsafe extern "system" fn EditProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: i32) -> windows_core::HRESULT
        where
            Identity: IPropertyPage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPage2_Impl::EditProperty(this, core::mem::transmute_copy(&dispid)).into()
        }
        Self { base__: IPropertyPage_Vtbl::new::<Identity, OFFSET>(), EditProperty: EditProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPage2 as windows_core::Interface>::IID || iid == &<IPropertyPage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IPropertyPageSite_Impl: Sized {
    fn OnStatusChange(&self, dwflags: &PROPPAGESTATUS) -> windows_core::Result<()>;
    fn GetLocaleID(&self) -> windows_core::Result<u32>;
    fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IPropertyPageSite {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IPropertyPageSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertyPageSite_Vtbl
    where
        Identity: IPropertyPageSite_Impl,
    {
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IPropertyPageSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPageSite_Impl::OnStatusChange(this, core::mem::transmute(&dwflags)).into()
        }
        unsafe extern "system" fn GetLocaleID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocaleid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPropertyPageSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyPageSite_Impl::GetLocaleID(this) {
                Ok(ok__) => {
                    plocaleid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertyPageSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyPageSite_Impl::GetPageContainer(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT
        where
            Identity: IPropertyPageSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyPageSite_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&pmsg)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyPageSite as windows_core::Interface>::IID
    }
}
pub trait IProtectFocus_Impl: Sized {
    fn AllowFocusChange(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IProtectFocus {}
impl IProtectFocus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProtectFocus_Vtbl
    where
        Identity: IProtectFocus_Impl,
    {
        unsafe extern "system" fn AllowFocusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IProtectFocus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtectFocus_Impl::AllowFocusChange(this) {
                Ok(ok__) => {
                    pfallow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowFocusChange: AllowFocusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectFocus as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IProtectedModeMenuServices_Impl: Sized {
    fn CreateMenu(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenu(&self, pszmodulename: &windows_core::PCWSTR, pszmenuname: &windows_core::PCWSTR) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenuID(&self, pszmodulename: &windows_core::PCWSTR, wresourceid: u16) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IProtectedModeMenuServices {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IProtectedModeMenuServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProtectedModeMenuServices_Vtbl
    where
        Identity: IProtectedModeMenuServices_Impl,
    {
        unsafe extern "system" fn CreateMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT
        where
            Identity: IProtectedModeMenuServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtectedModeMenuServices_Impl::CreateMenu(this) {
                Ok(ok__) => {
                    phmenu.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmodulename: windows_core::PCWSTR, pszmenuname: windows_core::PCWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT
        where
            Identity: IProtectedModeMenuServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtectedModeMenuServices_Impl::LoadMenu(this, core::mem::transmute(&pszmodulename), core::mem::transmute(&pszmenuname)) {
                Ok(ok__) => {
                    phmenu.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMenuID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmodulename: windows_core::PCWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT
        where
            Identity: IProtectedModeMenuServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtectedModeMenuServices_Impl::LoadMenuID(this, core::mem::transmute(&pszmodulename), core::mem::transmute_copy(&wresourceid)) {
                Ok(ok__) => {
                    phmenu.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMenu: CreateMenu::<Identity, OFFSET>,
            LoadMenu: LoadMenu::<Identity, OFFSET>,
            LoadMenuID: LoadMenuID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectedModeMenuServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo_Impl: Sized {
    fn GetClassInfo(&self) -> windows_core::Result<super::Com::ITypeInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideClassInfo_Vtbl
    where
        Identity: IProvideClassInfo_Impl,
    {
        unsafe extern "system" fn GetClassInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppti: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProvideClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvideClassInfo_Impl::GetClassInfo(this) {
                Ok(ok__) => {
                    ppti.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassInfo: GetClassInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo2_Impl: Sized + IProvideClassInfo_Impl {
    fn GetGUID(&self, dwguidkind: u32) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideClassInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideClassInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideClassInfo2_Vtbl
    where
        Identity: IProvideClassInfo2_Impl,
    {
        unsafe extern "system" fn GetGUID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwguidkind: u32, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IProvideClassInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvideClassInfo2_Impl::GetGUID(this, core::mem::transmute_copy(&dwguidkind)) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IProvideClassInfo_Vtbl::new::<Identity, OFFSET>(), GetGUID: GetGUID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideClassInfo2 as windows_core::Interface>::IID || iid == &<IProvideClassInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfo_Impl: Sized + IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetInfoOfIndex(&self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideMultipleClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMultipleClassInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideMultipleClassInfo_Vtbl
    where
        Identity: IProvideMultipleClassInfo_Impl,
    {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcti: *mut u32) -> windows_core::HRESULT
        where
            Identity: IProvideMultipleClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvideMultipleClassInfo_Impl::GetMultiTypeInfoCount(this) {
                Ok(ok__) => {
                    pcti.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoOfIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut *mut core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IProvideMultipleClassInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProvideMultipleClassInfo_Impl::GetInfoOfIndex(this, core::mem::transmute_copy(&iti), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppticoclass), core::mem::transmute_copy(&pdwtiflags), core::mem::transmute_copy(&pcdispidreserved), core::mem::transmute_copy(&piidprimary), core::mem::transmute_copy(&piidsource)).into()
        }
        Self {
            base__: IProvideClassInfo2_Vtbl::new::<Identity, OFFSET>(),
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Identity, OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideMultipleClassInfo as windows_core::Interface>::IID || iid == &<IProvideClassInfo as windows_core::Interface>::IID || iid == &<IProvideClassInfo2 as windows_core::Interface>::IID
    }
}
pub trait IProvideRuntimeContext_Impl: Sized {
    fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProvideRuntimeContext {}
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideRuntimeContext_Vtbl
    where
        Identity: IProvideRuntimeContext_Impl,
    {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IProvideRuntimeContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProvideRuntimeContext_Impl::GetCurrentSourceContext(this, core::mem::transmute_copy(&pdwcontext), core::mem::transmute_copy(&pfexecutingglobalcode)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrentSourceContext: GetCurrentSourceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideRuntimeContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IQuickActivate_Impl: Sized {
    fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::Result<()>;
    fn SetContentExtent(&self, psizel: *const super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn GetContentExtent(&self) -> windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IQuickActivate {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IQuickActivate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IQuickActivate_Vtbl
    where
        Identity: IQuickActivate_Impl,
    {
        unsafe extern "system" fn QuickActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::HRESULT
        where
            Identity: IQuickActivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQuickActivate_Impl::QuickActivate(this, core::mem::transmute_copy(&pqacontainer), core::mem::transmute_copy(&pqacontrol)).into()
        }
        unsafe extern "system" fn SetContentExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IQuickActivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQuickActivate_Impl::SetContentExtent(this, core::mem::transmute_copy(&psizel)).into()
        }
        unsafe extern "system" fn GetContentExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IQuickActivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQuickActivate_Impl::GetContentExtent(this) {
                Ok(ok__) => {
                    psizel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuickActivate: QuickActivate::<Identity, OFFSET>,
            SetContentExtent: SetContentExtent::<Identity, OFFSET>,
            GetContentExtent: GetContentExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQuickActivate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRecordInfo_Impl: Sized {
    fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordCopy(&self, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSize(&self) -> windows_core::Result<u32>;
    fn GetTypeInfo(&self) -> windows_core::Result<super::Com::ITypeInfo>;
    fn GetField(&self, pvdata: *const core::ffi::c_void, szfieldname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetFieldNoCopy(&self, pvdata: *const core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *mut windows_core::VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PutField(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn PutFieldNoCopy(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: &windows_core::PCWSTR, pvarfield: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsMatchingType(&self, precordinfo: Option<&IRecordInfo>) -> super::super::Foundation::BOOL;
    fn RecordCreate(&self) -> *mut core::ffi::c_void;
    fn RecordCreateCopy(&self, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRecordInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IRecordInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRecordInfo_Vtbl
    where
        Identity: IRecordInfo_Impl,
    {
        unsafe extern "system" fn RecordInit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordInit(this, core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn RecordClear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordClear(this, core::mem::transmute_copy(&pvexisting)).into()
        }
        unsafe extern "system" fn RecordCopy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordCopy(this, core::mem::transmute_copy(&pvexisting), core::mem::transmute_copy(&pvnew)).into()
        }
        unsafe extern "system" fn GetGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecordInfo_Impl::GetGuid(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecordInfo_Impl::GetName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecordInfo_Impl::GetSize(this) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecordInfo_Impl::GetTypeInfo(this) {
                Ok(ok__) => {
                    pptypeinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *const core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecordInfo_Impl::GetField(this, core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname)) {
                Ok(ok__) => {
                    pvarfield.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdata: *const core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::GetFieldNoCopy(this, core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield), core::mem::transmute_copy(&ppvdatacarray)).into()
        }
        unsafe extern "system" fn PutField<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::PutField(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: windows_core::PCWSTR, pvarfield: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::PutFieldNoCopy(this, core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pvdata), core::mem::transmute(&szfieldname), core::mem::transmute_copy(&pvarfield)).into()
        }
        unsafe extern "system" fn GetFieldNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::GetFieldNames(this, core::mem::transmute_copy(&pcnames), core::mem::transmute_copy(&rgbstrnames)).into()
        }
        unsafe extern "system" fn IsMatchingType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precordinfo: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::IsMatchingType(this, windows_core::from_raw_borrowed(&precordinfo))
        }
        unsafe extern "system" fn RecordCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordCreate(this)
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordCreateCopy(this, core::mem::transmute_copy(&pvsource), core::mem::transmute_copy(&ppvdest)).into()
        }
        unsafe extern "system" fn RecordDestroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvrecord: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecordInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecordInfo_Impl::RecordDestroy(this, core::mem::transmute_copy(&pvrecord)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RecordInit: RecordInit::<Identity, OFFSET>,
            RecordClear: RecordClear::<Identity, OFFSET>,
            RecordCopy: RecordCopy::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetField: GetField::<Identity, OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Identity, OFFSET>,
            PutField: PutField::<Identity, OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Identity, OFFSET>,
            GetFieldNames: GetFieldNames::<Identity, OFFSET>,
            IsMatchingType: IsMatchingType::<Identity, OFFSET>,
            RecordCreate: RecordCreate::<Identity, OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Identity, OFFSET>,
            RecordDestroy: RecordDestroy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecordInfo as windows_core::Interface>::IID
    }
}
pub trait ISimpleFrameSite_Impl: Sized {
    fn PreMessageFilter(&self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> windows_core::Result<()>;
    fn PostMessageFilter(&self, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISimpleFrameSite {}
impl ISimpleFrameSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISimpleFrameSite_Vtbl
    where
        Identity: ISimpleFrameSite_Impl,
    {
        unsafe extern "system" fn PreMessageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISimpleFrameSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleFrameSite_Impl::PreMessageFilter(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wp), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&plresult), core::mem::transmute_copy(&pdwcookie)).into()
        }
        unsafe extern "system" fn PostMessageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ISimpleFrameSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleFrameSite_Impl::PostMessageFilter(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wp), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&plresult), core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreMessageFilter: PreMessageFilter::<Identity, OFFSET>,
            PostMessageFilter: PostMessageFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleFrameSite as windows_core::Interface>::IID
    }
}
pub trait ISpecifyPropertyPages_Impl: Sized {
    fn GetPages(&self) -> windows_core::Result<CAUUID>;
}
impl windows_core::RuntimeName for ISpecifyPropertyPages {}
impl ISpecifyPropertyPages_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpecifyPropertyPages_Vtbl
    where
        Identity: ISpecifyPropertyPages_Impl,
    {
        unsafe extern "system" fn GetPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppages: *mut CAUUID) -> windows_core::HRESULT
        where
            Identity: ISpecifyPropertyPages_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpecifyPropertyPages_Impl::GetPages(this) {
                Ok(ok__) => {
                    ppages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPages: GetPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpecifyPropertyPages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeChangeEvents_Impl: Sized {
    fn RequestTypeChange(&self, changekind: CHANGEKIND, ptinfobefore: Option<&super::Com::ITypeInfo>, pstrname: &windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn AfterTypeChange(&self, changekind: CHANGEKIND, ptinfoafter: Option<&super::Com::ITypeInfo>, pstrname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITypeChangeEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ITypeChangeEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeChangeEvents_Vtbl
    where
        Identity: ITypeChangeEvents_Impl,
    {
        unsafe extern "system" fn RequestTypeChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, pfcancel: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITypeChangeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeChangeEvents_Impl::RequestTypeChange(this, core::mem::transmute_copy(&changekind), windows_core::from_raw_borrowed(&ptinfobefore), core::mem::transmute(&pstrname)) {
                Ok(ok__) => {
                    pfcancel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterTypeChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITypeChangeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeChangeEvents_Impl::AfterTypeChange(this, core::mem::transmute_copy(&changekind), windows_core::from_raw_borrowed(&ptinfoafter), core::mem::transmute(&pstrname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestTypeChange: RequestTypeChange::<Identity, OFFSET>,
            AfterTypeChange: AfterTypeChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeChangeEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeFactory_Impl: Sized {
    fn CreateFromTypeInfo(&self, ptypeinfo: Option<&super::Com::ITypeInfo>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITypeFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ITypeFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeFactory_Vtbl
    where
        Identity: ITypeFactory_Impl,
    {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeinfo: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeFactory_Impl::CreateFromTypeInfo(this, windows_core::from_raw_borrowed(&ptypeinfo), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateFromTypeInfo: CreateFromTypeInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeFactory as windows_core::Interface>::IID
    }
}
pub trait ITypeMarshal_Impl: Sized {
    fn Size(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn Marshal(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> windows_core::Result<()>;
    fn Unmarshal(&self, pvtype: *mut core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Free(&self, pvtype: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITypeMarshal {}
impl ITypeMarshal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeMarshal_Vtbl
    where
        Identity: ITypeMarshal_Impl,
    {
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, psize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeMarshal_Impl::Size(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext)) {
                Ok(ok__) => {
                    psize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeMarshal_Impl::Marshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Unmarshal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeMarshal_Impl::Unmarshal(this, core::mem::transmute_copy(&pvtype), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cbbufferlength), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Free<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeMarshal_Impl::Free(this, core::mem::transmute_copy(&pvtype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Size: Size::<Identity, OFFSET>,
            Marshal: Marshal::<Identity, OFFSET>,
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeMarshal as windows_core::Interface>::IID
    }
}
pub trait IVBFormat_Impl: Sized {
    fn Format(&self, vdata: *mut windows_core::VARIANT, bstrformat: &windows_core::BSTR, lpbuffer: *mut core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVBFormat {}
impl IVBFormat_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVBFormat_Vtbl
    where
        Identity: IVBFormat_Impl,
    {
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>, bstrformat: core::mem::MaybeUninit<windows_core::BSTR>, lpbuffer: *mut core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> windows_core::HRESULT
        where
            Identity: IVBFormat_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVBFormat_Impl::Format(this, core::mem::transmute_copy(&vdata), core::mem::transmute(&bstrformat), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&sfirstdayofweek), core::mem::transmute_copy(&sfirstweekofyear), core::mem::transmute_copy(&rcb)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Format: Format::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBFormat as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBGetControl_Impl: Sized {
    fn EnumControls(&self, dwolecontf: &OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> windows_core::Result<super::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBGetControl {}
#[cfg(feature = "Win32_System_Com")]
impl IVBGetControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVBGetControl_Vtbl
    where
        Identity: IVBGetControl_Impl,
    {
        unsafe extern "system" fn EnumControls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwolecontf: u32, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVBGetControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVBGetControl_Impl::EnumControls(this, core::mem::transmute(&dwolecontf), core::mem::transmute_copy(&dwwhich)) {
                Ok(ok__) => {
                    ppenumunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumControls: EnumControls::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBGetControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
pub trait IVariantChangeType_Impl: Sized {
    fn ChangeType(&self, pvardst: *mut windows_core::VARIANT, pvarsrc: *const windows_core::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for IVariantChangeType {}
#[cfg(feature = "Win32_System_Variant")]
impl IVariantChangeType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVariantChangeType_Vtbl
    where
        Identity: IVariantChangeType_Impl,
    {
        unsafe extern "system" fn ChangeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardst: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pvarsrc: *const core::mem::MaybeUninit<windows_core::VARIANT>, lcid: u32, vtnew: super::Variant::VARENUM) -> windows_core::HRESULT
        where
            Identity: IVariantChangeType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVariantChangeType_Impl::ChangeType(this, core::mem::transmute_copy(&pvardst), core::mem::transmute_copy(&pvarsrc), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&vtnew)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ChangeType: ChangeType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVariantChangeType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject_Impl: Sized {
    fn Draw(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> windows_core::Result<()>;
    fn GetColorSet(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::Result<()>;
    fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, pdwfreeze: *mut u32) -> windows_core::Result<()>;
    fn Unfreeze(&self, dwfreeze: u32) -> windows_core::Result<()>;
    fn SetAdvise(&self, aspects: super::Com::DVASPECT, advf: u32, padvsink: Option<&super::Com::IAdviseSink>) -> windows_core::Result<()>;
    fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut Option<super::Com::IAdviseSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IViewObject {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObject_Vtbl
    where
        Identity: IViewObject_Impl,
    {
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::Draw(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdctargetdev), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcwbounds), core::mem::transmute_copy(&pfncontinue), core::mem::transmute_copy(&dwcontinue)).into()
        }
        unsafe extern "system" fn GetColorSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::GetColorSet(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&ppcolorset)).into()
        }
        unsafe extern "system" fn Freeze<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, pdwfreeze: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::Freeze(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&pdwfreeze)).into()
        }
        unsafe extern "system" fn Unfreeze<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfreeze: u32) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::Unfreeze(this, core::mem::transmute_copy(&dwfreeze)).into()
        }
        unsafe extern "system" fn SetAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, aspects: super::Com::DVASPECT, advf: u32, padvsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::SetAdvise(this, core::mem::transmute_copy(&aspects), core::mem::transmute_copy(&advf), windows_core::from_raw_borrowed(&padvsink)).into()
        }
        unsafe extern "system" fn GetAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewObject_Impl::GetAdvise(this, core::mem::transmute_copy(&paspects), core::mem::transmute_copy(&padvf), core::mem::transmute_copy(&ppadvsink)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, OFFSET>,
            GetColorSet: GetColorSet::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            Unfreeze: Unfreeze::<Identity, OFFSET>,
            SetAdvise: SetAdvise::<Identity, OFFSET>,
            GetAdvise: GetAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject2_Impl: Sized + IViewObject_Impl {
    fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IViewObject2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObject2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObject2_Vtbl
    where
        Identity: IViewObject2_Impl,
    {
        unsafe extern "system" fn GetExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IViewObject2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObject2_Impl::GetExtent(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ptd)) {
                Ok(ok__) => {
                    lpsizel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IViewObject_Vtbl::new::<Identity, OFFSET>(), GetExtent: GetExtent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObject2 as windows_core::Interface>::IID || iid == &<IViewObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectEx_Impl: Sized + IViewObject2_Impl {
    fn GetRect(&self, dwaspect: u32) -> windows_core::Result<super::super::Foundation::RECTL>;
    fn GetViewStatus(&self) -> windows_core::Result<u32>;
    fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: &super::super::Foundation::POINT, lclosehint: i32) -> windows_core::Result<u32>;
    fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> windows_core::Result<u32>;
    fn GetNaturalExtent(&self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IViewObjectEx {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IViewObjectEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewObjectEx_Vtbl
    where
        Identity: IViewObjectEx_Impl,
    {
        unsafe extern "system" fn GetRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> windows_core::HRESULT
        where
            Identity: IViewObjectEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectEx_Impl::GetRect(this, core::mem::transmute_copy(&dwaspect)) {
                Ok(ok__) => {
                    prect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewObjectEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectEx_Impl::GetViewStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewObjectEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectEx_Impl::QueryHitPoint(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&prectbounds), core::mem::transmute(&ptlloc), core::mem::transmute_copy(&lclosehint)) {
                Ok(ok__) => {
                    phitresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHitRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewObjectEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectEx_Impl::QueryHitRect(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&prectbounds), core::mem::transmute_copy(&prectloc), core::mem::transmute_copy(&lclosehint)) {
                Ok(ok__) => {
                    phitresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNaturalExtent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO, psizel: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IViewObjectEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewObjectEx_Impl::GetNaturalExtent(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hictargetdev), core::mem::transmute_copy(&pextentinfo)) {
                Ok(ok__) => {
                    psizel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IViewObject2_Vtbl::new::<Identity, OFFSET>(),
            GetRect: GetRect::<Identity, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectEx as windows_core::Interface>::IID || iid == &<IViewObject as windows_core::Interface>::IID || iid == &<IViewObject2 as windows_core::Interface>::IID
    }
}
pub trait IZoomEvents_Impl: Sized {
    fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IZoomEvents {}
impl IZoomEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IZoomEvents_Vtbl
    where
        Identity: IZoomEvents_Impl,
    {
        unsafe extern "system" fn OnZoomPercentChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulzoompercent: u32) -> windows_core::HRESULT
        where
            Identity: IZoomEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoomEvents_Impl::OnZoomPercentChanged(this, core::mem::transmute_copy(&ulzoompercent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnZoomPercentChanged: OnZoomPercentChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoomEvents as windows_core::Interface>::IID
    }
}
