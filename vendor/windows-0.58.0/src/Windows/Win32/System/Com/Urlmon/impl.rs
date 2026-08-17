pub trait IBindCallbackRedirect_Impl: Sized {
    fn Redirect(&self, lpcurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
impl windows_core::RuntimeName for IBindCallbackRedirect {}
impl IBindCallbackRedirect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindCallbackRedirect_Vtbl
    where
        Identity: IBindCallbackRedirect_Impl,
    {
        unsafe extern "system" fn Redirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcurl: windows_core::PCWSTR, vbcancel: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IBindCallbackRedirect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindCallbackRedirect_Impl::Redirect(this, core::mem::transmute(&lpcurl)) {
                Ok(ok__) => {
                    vbcancel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Redirect: Redirect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindCallbackRedirect as windows_core::Interface>::IID
    }
}
pub trait IBindHttpSecurity_Impl: Sized {
    fn GetIgnoreCertMask(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBindHttpSecurity {}
impl IBindHttpSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindHttpSecurity_Vtbl
    where
        Identity: IBindHttpSecurity_Impl,
    {
        unsafe extern "system" fn GetIgnoreCertMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwignorecertmask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBindHttpSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindHttpSecurity_Impl::GetIgnoreCertMask(this) {
                Ok(ok__) => {
                    pdwignorecertmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIgnoreCertMask: GetIgnoreCertMask::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindHttpSecurity as windows_core::Interface>::IID
    }
}
pub trait IBindProtocol_Impl: Sized {
    fn CreateBinding(&self, szurl: &windows_core::PCWSTR, pbc: Option<&super::IBindCtx>) -> windows_core::Result<super::IBinding>;
}
impl windows_core::RuntimeName for IBindProtocol {}
impl IBindProtocol_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindProtocol_Vtbl
    where
        Identity: IBindProtocol_Impl,
    {
        unsafe extern "system" fn CreateBinding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, ppb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBindProtocol_Impl::CreateBinding(this, core::mem::transmute(&szurl), windows_core::from_raw_borrowed(&pbc)) {
                Ok(ok__) => {
                    ppb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateBinding: CreateBinding::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindProtocol as windows_core::Interface>::IID
    }
}
pub trait ICatalogFileInfo_Impl: Sized {
    fn GetCatalogFile(&self) -> windows_core::Result<windows_core::PSTR>;
    fn GetJavaTrust(&self, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICatalogFileInfo {}
impl ICatalogFileInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatalogFileInfo_Vtbl
    where
        Identity: ICatalogFileInfo_Impl,
    {
        unsafe extern "system" fn GetCatalogFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcatalogfile: *mut windows_core::PSTR) -> windows_core::HRESULT
        where
            Identity: ICatalogFileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogFileInfo_Impl::GetCatalogFile(this) {
                Ok(ok__) => {
                    ppszcatalogfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJavaTrust<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogFileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogFileInfo_Impl::GetJavaTrust(this, core::mem::transmute_copy(&ppjavatrust)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCatalogFile: GetCatalogFile::<Identity, OFFSET>,
            GetJavaTrust: GetJavaTrust::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogFileInfo as windows_core::Interface>::IID
    }
}
pub trait ICodeInstall_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnCodeInstallProblem(&self, ulstatuscode: u32, szdestination: &windows_core::PCWSTR, szsource: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICodeInstall {}
impl ICodeInstall_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICodeInstall_Vtbl
    where
        Identity: ICodeInstall_Impl,
    {
        unsafe extern "system" fn OnCodeInstallProblem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstatuscode: u32, szdestination: windows_core::PCWSTR, szsource: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: ICodeInstall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICodeInstall_Impl::OnCodeInstallProblem(this, core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szdestination), core::mem::transmute(&szsource), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, OFFSET>(), OnCodeInstallProblem: OnCodeInstallProblem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICodeInstall as windows_core::Interface>::IID || iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
pub trait IDataFilter_Impl: Sized {
    fn DoEncode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()>;
    fn DoDecode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetEncodingLevel(&self, dwenclevel: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDataFilter {}
impl IDataFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataFilter_Vtbl
    where
        Identity: IDataFilter_Impl,
    {
        unsafe extern "system" fn DoEncode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IDataFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataFilter_Impl::DoEncode(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&linbuffersize), core::mem::transmute_copy(&pbinbuffer), core::mem::transmute_copy(&loutbuffersize), core::mem::transmute_copy(&pboutbuffer), core::mem::transmute_copy(&linbytesavailable), core::mem::transmute_copy(&plinbytesread), core::mem::transmute_copy(&ploutbyteswritten), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn DoDecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IDataFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataFilter_Impl::DoDecode(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&linbuffersize), core::mem::transmute_copy(&pbinbuffer), core::mem::transmute_copy(&loutbuffersize), core::mem::transmute_copy(&pboutbuffer), core::mem::transmute_copy(&linbytesavailable), core::mem::transmute_copy(&plinbytesread), core::mem::transmute_copy(&ploutbyteswritten), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetEncodingLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenclevel: u32) -> windows_core::HRESULT
        where
            Identity: IDataFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataFilter_Impl::SetEncodingLevel(this, core::mem::transmute_copy(&dwenclevel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DoEncode: DoEncode::<Identity, OFFSET>,
            DoDecode: DoDecode::<Identity, OFFSET>,
            SetEncodingLevel: SetEncodingLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataFilter as windows_core::Interface>::IID
    }
}
pub trait IEncodingFilterFactory_Impl: Sized {
    fn FindBestFilter(&self, pwzcodein: &windows_core::PCWSTR, pwzcodeout: &windows_core::PCWSTR, info: &DATAINFO) -> windows_core::Result<IDataFilter>;
    fn GetDefaultFilter(&self, pwzcodein: &windows_core::PCWSTR, pwzcodeout: &windows_core::PCWSTR) -> windows_core::Result<IDataFilter>;
}
impl windows_core::RuntimeName for IEncodingFilterFactory {}
impl IEncodingFilterFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEncodingFilterFactory_Vtbl
    where
        Identity: IEncodingFilterFactory_Impl,
    {
        unsafe extern "system" fn FindBestFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzcodein: windows_core::PCWSTR, pwzcodeout: windows_core::PCWSTR, info: DATAINFO, ppdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEncodingFilterFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEncodingFilterFactory_Impl::FindBestFilter(this, core::mem::transmute(&pwzcodein), core::mem::transmute(&pwzcodeout), core::mem::transmute(&info)) {
                Ok(ok__) => {
                    ppdf.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzcodein: windows_core::PCWSTR, pwzcodeout: windows_core::PCWSTR, ppdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEncodingFilterFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEncodingFilterFactory_Impl::GetDefaultFilter(this, core::mem::transmute(&pwzcodein), core::mem::transmute(&pwzcodeout)) {
                Ok(ok__) => {
                    ppdf.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindBestFilter: FindBestFilter::<Identity, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEncodingFilterFactory as windows_core::Interface>::IID
    }
}
pub trait IGetBindHandle_Impl: Sized {
    fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for IGetBindHandle {}
impl IGetBindHandle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetBindHandle_Vtbl
    where
        Identity: IGetBindHandle_Impl,
    {
        unsafe extern "system" fn GetBindHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IGetBindHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetBindHandle_Impl::GetBindHandle(this, core::mem::transmute_copy(&enumrequestedhandle)) {
                Ok(ok__) => {
                    prethandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBindHandle: GetBindHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetBindHandle as windows_core::Interface>::IID
    }
}
pub trait IHttpNegotiate_Impl: Sized {
    fn BeginningTransaction(&self, szurl: &windows_core::PCWSTR, szheaders: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn OnResponse(&self, dwresponsecode: u32, szresponseheaders: &windows_core::PCWSTR, szrequestheaders: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IHttpNegotiate {}
impl IHttpNegotiate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHttpNegotiate_Vtbl
    where
        Identity: IHttpNegotiate_Impl,
    {
        unsafe extern "system" fn BeginningTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, szheaders: windows_core::PCWSTR, dwreserved: u32, pszadditionalheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IHttpNegotiate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpNegotiate_Impl::BeginningTransaction(this, core::mem::transmute(&szurl), core::mem::transmute(&szheaders), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    pszadditionalheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwresponsecode: u32, szresponseheaders: windows_core::PCWSTR, szrequestheaders: windows_core::PCWSTR, pszadditionalrequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IHttpNegotiate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHttpNegotiate_Impl::OnResponse(this, core::mem::transmute_copy(&dwresponsecode), core::mem::transmute(&szresponseheaders), core::mem::transmute(&szrequestheaders)) {
                Ok(ok__) => {
                    pszadditionalrequestheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginningTransaction: BeginningTransaction::<Identity, OFFSET>,
            OnResponse: OnResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate as windows_core::Interface>::IID
    }
}
pub trait IHttpNegotiate2_Impl: Sized + IHttpNegotiate_Impl {
    fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHttpNegotiate2 {}
impl IHttpNegotiate2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHttpNegotiate2_Vtbl
    where
        Identity: IHttpNegotiate2_Impl,
    {
        unsafe extern "system" fn GetRootSecurityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IHttpNegotiate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHttpNegotiate2_Impl::GetRootSecurityId(this, core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IHttpNegotiate_Vtbl::new::<Identity, OFFSET>(), GetRootSecurityId: GetRootSecurityId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate2 as windows_core::Interface>::IID || iid == &<IHttpNegotiate as windows_core::Interface>::IID
    }
}
pub trait IHttpNegotiate3_Impl: Sized + IHttpNegotiate2_Impl {
    fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHttpNegotiate3 {}
impl IHttpNegotiate3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHttpNegotiate3_Vtbl
    where
        Identity: IHttpNegotiate3_Impl,
    {
        unsafe extern "system" fn GetSerializedClientCertContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHttpNegotiate3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHttpNegotiate3_Impl::GetSerializedClientCertContext(this, core::mem::transmute_copy(&ppbcert), core::mem::transmute_copy(&pcbcert)).into()
        }
        Self { base__: IHttpNegotiate2_Vtbl::new::<Identity, OFFSET>(), GetSerializedClientCertContext: GetSerializedClientCertContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpNegotiate3 as windows_core::Interface>::IID || iid == &<IHttpNegotiate as windows_core::Interface>::IID || iid == &<IHttpNegotiate2 as windows_core::Interface>::IID
    }
}
pub trait IHttpSecurity_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnSecurityProblem(&self, dwproblem: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHttpSecurity {}
impl IHttpSecurity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHttpSecurity_Vtbl
    where
        Identity: IHttpSecurity_Impl,
    {
        unsafe extern "system" fn OnSecurityProblem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproblem: u32) -> windows_core::HRESULT
        where
            Identity: IHttpSecurity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHttpSecurity_Impl::OnSecurityProblem(this, core::mem::transmute_copy(&dwproblem)).into()
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, OFFSET>(), OnSecurityProblem: OnSecurityProblem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHttpSecurity as windows_core::Interface>::IID || iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
pub trait IInternet_Impl: Sized {}
impl windows_core::RuntimeName for IInternet {}
impl IInternet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternet_Vtbl
    where
        Identity: IInternet_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfo_Impl: Sized {
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> windows_core::Result<()>;
    fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IInternetBindInfo {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetBindInfo_Vtbl
    where
        Identity: IInternetBindInfo_Impl,
    {
        unsafe extern "system" fn GetBindInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> windows_core::HRESULT
        where
            Identity: IInternetBindInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetBindInfo_Impl::GetBindInfo(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn GetBindString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetBindInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetBindInfo_Impl::GetBindString(this, core::mem::transmute_copy(&ulstringtype), core::mem::transmute_copy(&ppwzstr), core::mem::transmute_copy(&cel), core::mem::transmute_copy(&pcelfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBindInfo: GetBindInfo::<Identity, OFFSET>,
            GetBindString: GetBindString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetBindInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfoEx_Impl: Sized + IInternetBindInfo_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IInternetBindInfoEx {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfoEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetBindInfoEx_Vtbl
    where
        Identity: IInternetBindInfoEx_Impl,
    {
        unsafe extern "system" fn GetBindInfoEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetBindInfoEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetBindInfoEx_Impl::GetBindInfoEx(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&grfbindf2), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IInternetBindInfo_Vtbl::new::<Identity, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetBindInfoEx as windows_core::Interface>::IID || iid == &<IInternetBindInfo as windows_core::Interface>::IID
    }
}
pub trait IInternetHostSecurityManager_Impl: Sized {
    fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn ProcessUrlAction(&self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn QueryCustomPolicy(&self, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetHostSecurityManager {}
impl IInternetHostSecurityManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetHostSecurityManager_Vtbl
    where
        Identity: IInternetHostSecurityManager_Impl,
    {
        unsafe extern "system" fn GetSecurityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IInternetHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetHostSecurityManager_Impl::GetSecurityId(this, core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetHostSecurityManager_Impl::ProcessUrlAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetHostSecurityManager_Impl::QueryCustomPolicy(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSecurityId: GetSecurityId::<Identity, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetHostSecurityManager as windows_core::Interface>::IID
    }
}
pub trait IInternetPriority_Impl: Sized {
    fn SetPriority(&self, npriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IInternetPriority {}
impl IInternetPriority_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetPriority_Vtbl
    where
        Identity: IInternetPriority_Impl,
    {
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, npriority: i32) -> windows_core::HRESULT
        where
            Identity: IInternetPriority_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetPriority_Impl::SetPriority(this, core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInternetPriority_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetPriority_Impl::GetPriority(this) {
                Ok(ok__) => {
                    pnpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetPriority as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocol_Impl: Sized + IInternetProtocolRoot_Impl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Seek(&self, dlibmove: i64, dworigin: u32) -> windows_core::Result<u64>;
    fn LockRequest(&self, dwoptions: u32) -> windows_core::Result<()>;
    fn UnlockRequest(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocol {}
impl IInternetProtocol_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocol_Vtbl
    where
        Identity: IInternetProtocol_Impl,
    {
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocol_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IInternetProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetProtocol_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin)) {
                Ok(ok__) => {
                    plibnewposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocol_Impl::LockRequest(this, core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn UnlockRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocol_Impl::UnlockRequest(this).into()
        }
        Self {
            base__: IInternetProtocolRoot_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            LockRequest: LockRequest::<Identity, OFFSET>,
            UnlockRequest: UnlockRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocol as windows_core::Interface>::IID || iid == &<IInternetProtocolRoot as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocolEx_Impl: Sized + IInternetProtocol_Impl {
    fn StartEx(&self, puri: Option<&super::IUri>, poiprotsink: Option<&IInternetProtocolSink>, poibindinfo: Option<&IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocolEx {}
impl IInternetProtocolEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocolEx_Vtbl
    where
        Identity: IInternetProtocolEx_Impl,
    {
        unsafe extern "system" fn StartEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, poiprotsink: *mut core::ffi::c_void, poibindinfo: *mut core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolEx_Impl::StartEx(this, windows_core::from_raw_borrowed(&puri), windows_core::from_raw_borrowed(&poiprotsink), windows_core::from_raw_borrowed(&poibindinfo), core::mem::transmute_copy(&grfpi), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IInternetProtocol_Vtbl::new::<Identity, OFFSET>(), StartEx: StartEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolEx as windows_core::Interface>::IID || iid == &<IInternetProtocolRoot as windows_core::Interface>::IID || iid == &<IInternetProtocol as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocolInfo_Impl: Sized {
    fn ParseUrl(&self, pwzurl: &windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
    fn CombineUrl(&self, pwzbaseurl: &windows_core::PCWSTR, pwzrelativeurl: &windows_core::PCWSTR, dwcombineflags: u32, pwzresult: &windows_core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
    fn CompareUrl(&self, pwzurl1: &windows_core::PCWSTR, pwzurl2: &windows_core::PCWSTR, dwcompareflags: u32) -> windows_core::Result<()>;
    fn QueryInfo(&self, pwzurl: &windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocolInfo {}
impl IInternetProtocolInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocolInfo_Vtbl
    where
        Identity: IInternetProtocolInfo_Impl,
    {
        unsafe extern "system" fn ParseUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl: windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolInfo_Impl::ParseUrl(this, core::mem::transmute(&pwzurl), core::mem::transmute_copy(&parseaction), core::mem::transmute_copy(&dwparseflags), core::mem::transmute_copy(&pwzresult), core::mem::transmute_copy(&cchresult), core::mem::transmute_copy(&pcchresult), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CombineUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbaseurl: windows_core::PCWSTR, pwzrelativeurl: windows_core::PCWSTR, dwcombineflags: u32, pwzresult: windows_core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolInfo_Impl::CombineUrl(this, core::mem::transmute(&pwzbaseurl), core::mem::transmute(&pwzrelativeurl), core::mem::transmute_copy(&dwcombineflags), core::mem::transmute(&pwzresult), core::mem::transmute_copy(&cchresult), core::mem::transmute_copy(&pcchresult), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CompareUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl1: windows_core::PCWSTR, pwzurl2: windows_core::PCWSTR, dwcompareflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolInfo_Impl::CompareUrl(this, core::mem::transmute(&pwzurl1), core::mem::transmute(&pwzurl2), core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn QueryInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzurl: windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolInfo_Impl::QueryInfo(this, core::mem::transmute(&pwzurl), core::mem::transmute_copy(&oueryoption), core::mem::transmute_copy(&dwqueryflags), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcbbuf), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseUrl: ParseUrl::<Identity, OFFSET>,
            CombineUrl: CombineUrl::<Identity, OFFSET>,
            CompareUrl: CompareUrl::<Identity, OFFSET>,
            QueryInfo: QueryInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolInfo as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocolRoot_Impl: Sized {
    fn Start(&self, szurl: &windows_core::PCWSTR, poiprotsink: Option<&IInternetProtocolSink>, poibindinfo: Option<&IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()>;
    fn Abort(&self, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::Result<()>;
    fn Terminate(&self, dwoptions: u32) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocolRoot {}
impl IInternetProtocolRoot_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocolRoot_Vtbl
    where
        Identity: IInternetProtocolRoot_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, poiprotsink: *mut core::ffi::c_void, poibindinfo: *mut core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Start(this, core::mem::transmute(&szurl), windows_core::from_raw_borrowed(&poiprotsink), windows_core::from_raw_borrowed(&poibindinfo), core::mem::transmute_copy(&grfpi), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Continue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Continue(this, core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Abort(this, core::mem::transmute_copy(&hrreason), core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Terminate(this, core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Suspend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Suspend(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolRoot_Impl::Resume(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Continue: Continue::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolRoot as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocolSink_Impl: Sized {
    fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()>;
    fn ReportProgress(&self, ulstatuscode: u32, szstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()>;
    fn ReportResult(&self, hrresult: windows_core::HRESULT, dwerror: u32, szresult: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocolSink {}
impl IInternetProtocolSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocolSink_Vtbl
    where
        Identity: IInternetProtocolSink_Impl,
    {
        unsafe extern "system" fn Switch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSink_Impl::Switch(this, core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn ReportProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstatuscode: u32, szstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSink_Impl::ReportProgress(this, core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szstatustext)).into()
        }
        unsafe extern "system" fn ReportData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSink_Impl::ReportData(this, core::mem::transmute_copy(&grfbscf), core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax)).into()
        }
        unsafe extern "system" fn ReportResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, dwerror: u32, szresult: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSink_Impl::ReportResult(this, core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&dwerror), core::mem::transmute(&szresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Switch: Switch::<Identity, OFFSET>,
            ReportProgress: ReportProgress::<Identity, OFFSET>,
            ReportData: ReportData::<Identity, OFFSET>,
            ReportResult: ReportResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolSink as windows_core::Interface>::IID
    }
}
pub trait IInternetProtocolSinkStackable_Impl: Sized {
    fn SwitchSink(&self, poiprotsink: Option<&IInternetProtocolSink>) -> windows_core::Result<()>;
    fn CommitSwitch(&self) -> windows_core::Result<()>;
    fn RollbackSwitch(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetProtocolSinkStackable {}
impl IInternetProtocolSinkStackable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetProtocolSinkStackable_Vtbl
    where
        Identity: IInternetProtocolSinkStackable_Impl,
    {
        unsafe extern "system" fn SwitchSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poiprotsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSinkStackable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSinkStackable_Impl::SwitchSink(this, windows_core::from_raw_borrowed(&poiprotsink)).into()
        }
        unsafe extern "system" fn CommitSwitch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSinkStackable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSinkStackable_Impl::CommitSwitch(this).into()
        }
        unsafe extern "system" fn RollbackSwitch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetProtocolSinkStackable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetProtocolSinkStackable_Impl::RollbackSwitch(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SwitchSink: SwitchSink::<Identity, OFFSET>,
            CommitSwitch: CommitSwitch::<Identity, OFFSET>,
            RollbackSwitch: RollbackSwitch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetProtocolSinkStackable as windows_core::Interface>::IID
    }
}
pub trait IInternetSecurityManager_Impl: Sized {
    fn SetSecuritySite(&self, psite: Option<&IInternetSecurityMgrSite>) -> windows_core::Result<()>;
    fn GetSecuritySite(&self) -> windows_core::Result<IInternetSecurityMgrSite>;
    fn MapUrlToZone(&self, pwszurl: &windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetSecurityId(&self, pwszurl: &windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn ProcessUrlAction(&self, pwszurl: &windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn QueryCustomPolicy(&self, pwszurl: &windows_core::PCWSTR, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetZoneMapping(&self, dwzone: u32, lpszpattern: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut Option<super::IEnumString>, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetSecurityManager {}
impl IInternetSecurityManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetSecurityManager_Vtbl
    where
        Identity: IInternetSecurityManager_Impl,
    {
        unsafe extern "system" fn SetSecuritySite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::SetSecuritySite(this, windows_core::from_raw_borrowed(&psite)).into()
        }
        unsafe extern "system" fn GetSecuritySite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetSecurityManager_Impl::GetSecuritySite(this) {
                Ok(ok__) => {
                    ppsite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapUrlToZone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::MapUrlToZone(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pdwzone), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetSecurityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::GetSecurityId(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::ProcessUrlAction(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::QueryCustomPolicy(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetZoneMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, lpszpattern: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::SetZoneMapping(this, core::mem::transmute_copy(&dwzone), core::mem::transmute(&lpszpattern), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, ppenumstring: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManager_Impl::GetZoneMappings(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&ppenumstring), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSecuritySite: SetSecuritySite::<Identity, OFFSET>,
            GetSecuritySite: GetSecuritySite::<Identity, OFFSET>,
            MapUrlToZone: MapUrlToZone::<Identity, OFFSET>,
            GetSecurityId: GetSecurityId::<Identity, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, OFFSET>,
            SetZoneMapping: SetZoneMapping::<Identity, OFFSET>,
            GetZoneMappings: GetZoneMappings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManager as windows_core::Interface>::IID
    }
}
pub trait IInternetSecurityManagerEx_Impl: Sized + IInternetSecurityManager_Impl {
    fn ProcessUrlActionEx(&self, pwszurl: &windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetSecurityManagerEx {}
impl IInternetSecurityManagerEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetSecurityManagerEx_Vtbl
    where
        Identity: IInternetSecurityManagerEx_Impl,
    {
        unsafe extern "system" fn ProcessUrlActionEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManagerEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManagerEx_Impl::ProcessUrlActionEx(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pdwoutflags)).into()
        }
        Self { base__: IInternetSecurityManager_Vtbl::new::<Identity, OFFSET>(), ProcessUrlActionEx: ProcessUrlActionEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx as windows_core::Interface>::IID || iid == &<IInternetSecurityManager as windows_core::Interface>::IID
    }
}
pub trait IInternetSecurityManagerEx2_Impl: Sized + IInternetSecurityManagerEx_Impl {
    fn MapUrlToZoneEx2(&self, puri: Option<&super::IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut windows_core::PWSTR, pdwoutflags: *mut u32) -> windows_core::Result<()>;
    fn ProcessUrlActionEx2(&self, puri: Option<&super::IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityIdEx2(&self, puri: Option<&super::IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>;
    fn QueryCustomPolicyEx2(&self, puri: Option<&super::IUri>, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetSecurityManagerEx2 {}
impl IInternetSecurityManagerEx2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetSecurityManagerEx2_Vtbl
    where
        Identity: IInternetSecurityManagerEx2_Impl,
    {
        unsafe extern "system" fn MapUrlToZoneEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut windows_core::PWSTR, pdwoutflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManagerEx2_Impl::MapUrlToZoneEx2(this, windows_core::from_raw_borrowed(&puri), core::mem::transmute_copy(&pdwzone), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppwszmappedurl), core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManagerEx2_Impl::ProcessUrlActionEx2(this, windows_core::from_raw_borrowed(&puri), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn GetSecurityIdEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManagerEx2_Impl::GetSecurityIdEx2(this, windows_core::from_raw_borrowed(&puri), core::mem::transmute_copy(&pbsecurityid), core::mem::transmute_copy(&pcbsecurityid), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityManagerEx2_Impl::QueryCustomPolicyEx2(this, windows_core::from_raw_borrowed(&puri), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&cbcontext), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: IInternetSecurityManagerEx_Vtbl::new::<Identity, OFFSET>(),
            MapUrlToZoneEx2: MapUrlToZoneEx2::<Identity, OFFSET>,
            ProcessUrlActionEx2: ProcessUrlActionEx2::<Identity, OFFSET>,
            GetSecurityIdEx2: GetSecurityIdEx2::<Identity, OFFSET>,
            QueryCustomPolicyEx2: QueryCustomPolicyEx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx2 as windows_core::Interface>::IID || iid == &<IInternetSecurityManager as windows_core::Interface>::IID || iid == &<IInternetSecurityManagerEx as windows_core::Interface>::IID
    }
}
pub trait IInternetSecurityMgrSite_Impl: Sized {
    fn GetWindow(&self) -> windows_core::Result<super::super::super::Foundation::HWND>;
    fn EnableModeless(&self, fenable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetSecurityMgrSite {}
impl IInternetSecurityMgrSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetSecurityMgrSite_Vtbl
    where
        Identity: IInternetSecurityMgrSite_Impl,
    {
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityMgrSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetSecurityMgrSite_Impl::GetWindow(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IInternetSecurityMgrSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSecurityMgrSite_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSecurityMgrSite as windows_core::Interface>::IID
    }
}
pub trait IInternetSession_Impl: Sized {
    fn RegisterNameSpace(&self, pcf: Option<&super::IClassFactory>, rclsid: *const windows_core::GUID, pwzprotocol: &windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn UnregisterNameSpace(&self, pcf: Option<&super::IClassFactory>, pszprotocol: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RegisterMimeFilter(&self, pcf: Option<&super::IClassFactory>, rclsid: *const windows_core::GUID, pwztype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterMimeFilter(&self, pcf: Option<&super::IClassFactory>, pwztype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateBinding(&self, pbc: Option<&super::IBindCtx>, szurl: &windows_core::PCWSTR, punkouter: Option<&windows_core::IUnknown>, ppunk: *mut Option<windows_core::IUnknown>, ppoinetprot: *mut Option<IInternetProtocol>, dwoption: u32) -> windows_core::Result<()>;
    fn SetSessionOption(&self, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::Result<()>;
    fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetSession {}
impl IInternetSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetSession_Vtbl
    where
        Identity: IInternetSession_Impl,
    {
        unsafe extern "system" fn RegisterNameSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pwzprotocol: windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::RegisterNameSpace(this, windows_core::from_raw_borrowed(&pcf), core::mem::transmute_copy(&rclsid), core::mem::transmute(&pwzprotocol), core::mem::transmute_copy(&cpatterns), core::mem::transmute_copy(&ppwzpatterns), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn UnregisterNameSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, pszprotocol: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::UnregisterNameSpace(this, windows_core::from_raw_borrowed(&pcf), core::mem::transmute(&pszprotocol)).into()
        }
        unsafe extern "system" fn RegisterMimeFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, pwztype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::RegisterMimeFilter(this, windows_core::from_raw_borrowed(&pcf), core::mem::transmute_copy(&rclsid), core::mem::transmute(&pwztype)).into()
        }
        unsafe extern "system" fn UnregisterMimeFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcf: *mut core::ffi::c_void, pwztype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::UnregisterMimeFilter(this, windows_core::from_raw_borrowed(&pcf), core::mem::transmute(&pwztype)).into()
        }
        unsafe extern "system" fn CreateBinding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, szurl: windows_core::PCWSTR, punkouter: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void, ppoinetprot: *mut *mut core::ffi::c_void, dwoption: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::CreateBinding(this, windows_core::from_raw_borrowed(&pbc), core::mem::transmute(&szurl), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ppunk), core::mem::transmute_copy(&ppoinetprot), core::mem::transmute_copy(&dwoption)).into()
        }
        unsafe extern "system" fn SetSessionOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::SetSessionOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbufferlength), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetSessionOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetSession_Impl::GetSessionOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pdwbufferlength), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterNameSpace: RegisterNameSpace::<Identity, OFFSET>,
            UnregisterNameSpace: UnregisterNameSpace::<Identity, OFFSET>,
            RegisterMimeFilter: RegisterMimeFilter::<Identity, OFFSET>,
            UnregisterMimeFilter: UnregisterMimeFilter::<Identity, OFFSET>,
            CreateBinding: CreateBinding::<Identity, OFFSET>,
            SetSessionOption: SetSessionOption::<Identity, OFFSET>,
            GetSessionOption: GetSessionOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetSession as windows_core::Interface>::IID
    }
}
pub trait IInternetThreadSwitch_Impl: Sized {
    fn Prepare(&self) -> windows_core::Result<()>;
    fn Continue(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetThreadSwitch {}
impl IInternetThreadSwitch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetThreadSwitch_Vtbl
    where
        Identity: IInternetThreadSwitch_Impl,
    {
        unsafe extern "system" fn Prepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetThreadSwitch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetThreadSwitch_Impl::Prepare(this).into()
        }
        unsafe extern "system" fn Continue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetThreadSwitch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetThreadSwitch_Impl::Continue(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Prepare: Prepare::<Identity, OFFSET>, Continue: Continue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetThreadSwitch as windows_core::Interface>::IID
    }
}
pub trait IInternetZoneManager_Impl: Sized {
    fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::Result<()>;
    fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::Result<()>;
    fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::Result<()>;
    fn PromptAction(&self, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR, dwpromptflags: u32) -> windows_core::Result<()>;
    fn LogAction(&self, dwaction: u32, pwszurl: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR, dwlogflags: u32) -> windows_core::Result<()>;
    fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> windows_core::Result<u32>;
    fn DestroyZoneEnumerator(&self, dwenum: u32) -> windows_core::Result<()>;
    fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetZoneManager {}
impl IInternetZoneManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetZoneManager_Vtbl
    where
        Identity: IInternetZoneManager_Impl,
    {
        unsafe extern "system" fn GetZoneAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::GetZoneAttributes(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn SetZoneAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::SetZoneAttributes(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::GetZoneCustomPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&pcbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::SetZoneCustomPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn GetZoneActionPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::GetZoneActionPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::SetZoneActionPolicy(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn PromptAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR, dwpromptflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::PromptAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztext), core::mem::transmute_copy(&dwpromptflags)).into()
        }
        unsafe extern "system" fn LogAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaction: u32, pwszurl: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR, dwlogflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::LogAction(this, core::mem::transmute_copy(&dwaction), core::mem::transmute(&pwszurl), core::mem::transmute(&pwsztext), core::mem::transmute_copy(&dwlogflags)).into()
        }
        unsafe extern "system" fn CreateZoneEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::CreateZoneEnumerator(this, core::mem::transmute_copy(&pdwenum), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInternetZoneManager_Impl::GetZoneAt(this, core::mem::transmute_copy(&dwenum), core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pdwzone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenum: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::DestroyZoneEnumerator(this, core::mem::transmute_copy(&dwenum)).into()
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManager_Impl::CopyTemplatePoliciesToZone(this, core::mem::transmute_copy(&dwtemplate), core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetZoneAttributes: GetZoneAttributes::<Identity, OFFSET>,
            SetZoneAttributes: SetZoneAttributes::<Identity, OFFSET>,
            GetZoneCustomPolicy: GetZoneCustomPolicy::<Identity, OFFSET>,
            SetZoneCustomPolicy: SetZoneCustomPolicy::<Identity, OFFSET>,
            GetZoneActionPolicy: GetZoneActionPolicy::<Identity, OFFSET>,
            SetZoneActionPolicy: SetZoneActionPolicy::<Identity, OFFSET>,
            PromptAction: PromptAction::<Identity, OFFSET>,
            LogAction: LogAction::<Identity, OFFSET>,
            CreateZoneEnumerator: CreateZoneEnumerator::<Identity, OFFSET>,
            GetZoneAt: GetZoneAt::<Identity, OFFSET>,
            DestroyZoneEnumerator: DestroyZoneEnumerator::<Identity, OFFSET>,
            CopyTemplatePoliciesToZone: CopyTemplatePoliciesToZone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManager as windows_core::Interface>::IID
    }
}
pub trait IInternetZoneManagerEx_Impl: Sized + IInternetZoneManager_Impl {
    fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()>;
    fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetZoneManagerEx {}
impl IInternetZoneManagerEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetZoneManagerEx_Vtbl
    where
        Identity: IInternetZoneManagerEx_Impl,
    {
        unsafe extern "system" fn GetZoneActionPolicyEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx_Impl::GetZoneActionPolicyEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx_Impl::SetZoneActionPolicyEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&cbpolicy), core::mem::transmute_copy(&urlzonereg), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IInternetZoneManager_Vtbl::new::<Identity, OFFSET>(),
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Identity, OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx as windows_core::Interface>::IID || iid == &<IInternetZoneManager as windows_core::Interface>::IID
    }
}
pub trait IInternetZoneManagerEx2_Impl: Sized + IInternetZoneManagerEx_Impl {
    fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::Result<()>;
    fn GetZoneSecurityState(&self, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetIESecurityState(&self, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FixUnsecureSettings(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInternetZoneManagerEx2 {}
impl IInternetZoneManagerEx2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInternetZoneManagerEx2_Vtbl
    where
        Identity: IInternetZoneManagerEx2_Impl,
    {
        unsafe extern "system" fn GetZoneAttributesEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx2_Impl::GetZoneAttributesEx(this, core::mem::transmute_copy(&dwzone), core::mem::transmute_copy(&pzoneattributes), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneSecurityState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx2_Impl::GetZoneSecurityState(this, core::mem::transmute_copy(&dwzoneindex), core::mem::transmute_copy(&frespectpolicy), core::mem::transmute_copy(&pdwstate), core::mem::transmute_copy(&pfpolicyencountered)).into()
        }
        unsafe extern "system" fn GetIESecurityState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx2_Impl::GetIESecurityState(this, core::mem::transmute_copy(&frespectpolicy), core::mem::transmute_copy(&pdwstate), core::mem::transmute_copy(&pfpolicyencountered), core::mem::transmute_copy(&fnocache)).into()
        }
        unsafe extern "system" fn FixUnsecureSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInternetZoneManagerEx2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInternetZoneManagerEx2_Impl::FixUnsecureSettings(this).into()
        }
        Self {
            base__: IInternetZoneManagerEx_Vtbl::new::<Identity, OFFSET>(),
            GetZoneAttributesEx: GetZoneAttributesEx::<Identity, OFFSET>,
            GetZoneSecurityState: GetZoneSecurityState::<Identity, OFFSET>,
            GetIESecurityState: GetIESecurityState::<Identity, OFFSET>,
            FixUnsecureSettings: FixUnsecureSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx2 as windows_core::Interface>::IID || iid == &<IInternetZoneManager as windows_core::Interface>::IID || iid == &<IInternetZoneManagerEx as windows_core::Interface>::IID
    }
}
pub trait IMonikerProp_Impl: Sized {
    fn PutProperty(&self, mkp: MONIKERPROPERTY, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMonikerProp {}
impl IMonikerProp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMonikerProp_Vtbl
    where
        Identity: IMonikerProp_Impl,
    {
        unsafe extern "system" fn PutProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mkp: MONIKERPROPERTY, val: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMonikerProp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMonikerProp_Impl::PutProperty(this, core::mem::transmute_copy(&mkp), core::mem::transmute(&val)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PutProperty: PutProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonikerProp as windows_core::Interface>::IID
    }
}
pub trait IPersistMoniker_Impl: Sized {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: Option<&super::IMoniker>, pibc: Option<&super::IBindCtx>, grfmode: u32) -> windows_core::Result<()>;
    fn Save(&self, pimkname: Option<&super::IMoniker>, pbc: Option<&super::IBindCtx>, fremember: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pimkname: Option<&super::IMoniker>, pibc: Option<&super::IBindCtx>) -> windows_core::Result<()>;
    fn GetCurMoniker(&self) -> windows_core::Result<super::IMoniker>;
}
impl windows_core::RuntimeName for IPersistMoniker {}
impl IPersistMoniker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPersistMoniker_Vtbl
    where
        Identity: IPersistMoniker_Impl,
    {
        unsafe extern "system" fn GetClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistMoniker_Impl::GetClassID(this) {
                Ok(ok__) => {
                    pclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMoniker_Impl::IsDirty(this)
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: *mut core::ffi::c_void, pibc: *mut core::ffi::c_void, grfmode: u32) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMoniker_Impl::Load(this, core::mem::transmute_copy(&ffullyavailable), windows_core::from_raw_borrowed(&pimkname), windows_core::from_raw_borrowed(&pibc), core::mem::transmute_copy(&grfmode)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimkname: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, fremember: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMoniker_Impl::Save(this, windows_core::from_raw_borrowed(&pimkname), windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimkname: *mut core::ffi::c_void, pibc: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistMoniker_Impl::SaveCompleted(this, windows_core::from_raw_borrowed(&pimkname), windows_core::from_raw_borrowed(&pibc)).into()
        }
        unsafe extern "system" fn GetCurMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimkname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPersistMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistMoniker_Impl::GetCurMoniker(this) {
                Ok(ok__) => {
                    ppimkname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassID: GetClassID::<Identity, OFFSET>,
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            GetCurMoniker: GetCurMoniker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMoniker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
pub trait ISoftDistExt_Impl: Sized {
    fn ProcessSoftDist(&self, szcdfurl: &windows_core::PCWSTR, psoftdistelement: Option<&super::super::super::Data::Xml::MsXml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> windows_core::Result<()>;
    fn GetFirstCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()>;
    fn GetNextCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()>;
    fn AsyncInstallDistributionUnit(&self, pbc: Option<&super::IBindCtx>, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl windows_core::RuntimeName for ISoftDistExt {}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl ISoftDistExt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISoftDistExt_Vtbl
    where
        Identity: ISoftDistExt_Impl,
    {
        unsafe extern "system" fn ProcessSoftDist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcdfurl: windows_core::PCWSTR, psoftdistelement: *mut core::ffi::c_void, lpsdi: *mut SOFTDISTINFO) -> windows_core::HRESULT
        where
            Identity: ISoftDistExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftDistExt_Impl::ProcessSoftDist(this, core::mem::transmute(&szcdfurl), windows_core::from_raw_borrowed(&psoftdistelement), core::mem::transmute_copy(&lpsdi)).into()
        }
        unsafe extern "system" fn GetFirstCodeBase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT
        where
            Identity: ISoftDistExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftDistExt_Impl::GetFirstCodeBase(this, core::mem::transmute_copy(&szcodebase), core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn GetNextCodeBase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::HRESULT
        where
            Identity: ISoftDistExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftDistExt_Impl::GetNextCodeBase(this, core::mem::transmute_copy(&szcodebase), core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::HRESULT
        where
            Identity: ISoftDistExt_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftDistExt_Impl::AsyncInstallDistributionUnit(this, windows_core::from_raw_borrowed(&pbc), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&lpcbh)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProcessSoftDist: ProcessSoftDist::<Identity, OFFSET>,
            GetFirstCodeBase: GetFirstCodeBase::<Identity, OFFSET>,
            GetNextCodeBase: GetNextCodeBase::<Identity, OFFSET>,
            AsyncInstallDistributionUnit: AsyncInstallDistributionUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISoftDistExt as windows_core::Interface>::IID
    }
}
pub trait IUriBuilderFactory_Impl: Sized {
    fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<super::IUriBuilder>;
    fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<super::IUriBuilder>;
}
impl windows_core::RuntimeName for IUriBuilderFactory {}
impl IUriBuilderFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUriBuilderFactory_Vtbl
    where
        Identity: IUriBuilderFactory_Impl,
    {
        unsafe extern "system" fn CreateIUriBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilderFactory_Impl::CreateIUriBuilder(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppiuribuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriBuilderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriBuilderFactory_Impl::CreateInitializedIUriBuilder(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    ppiuribuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIUriBuilder: CreateIUriBuilder::<Identity, OFFSET>,
            CreateInitializedIUriBuilder: CreateInitializedIUriBuilder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriBuilderFactory as windows_core::Interface>::IID
    }
}
pub trait IUriContainer_Impl: Sized {
    fn GetIUri(&self) -> windows_core::Result<super::IUri>;
}
impl windows_core::RuntimeName for IUriContainer {}
impl IUriContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUriContainer_Vtbl
    where
        Identity: IUriContainer_Impl,
    {
        unsafe extern "system" fn GetIUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUriContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUriContainer_Impl::GetIUri(this) {
                Ok(ok__) => {
                    ppiuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIUri: GetIUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriContainer as windows_core::Interface>::IID
    }
}
pub trait IWinInetCacheHints_Impl: Sized {
    fn SetCacheExtension(&self, pwzext: &windows_core::PCWSTR, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetCacheHints {}
impl IWinInetCacheHints_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetCacheHints_Vtbl
    where
        Identity: IWinInetCacheHints_Impl,
    {
        unsafe extern "system" fn SetCacheExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzext: windows_core::PCWSTR, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWinInetCacheHints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetCacheHints_Impl::SetCacheExtension(this, core::mem::transmute(&pwzext), core::mem::transmute_copy(&pszcachefile), core::mem::transmute_copy(&pcbcachefile), core::mem::transmute_copy(&pdwwinineterror), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetCacheExtension: SetCacheExtension::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetCacheHints as windows_core::Interface>::IID
    }
}
pub trait IWinInetCacheHints2_Impl: Sized + IWinInetCacheHints_Impl {
    fn SetCacheExtension2(&self, pwzext: &windows_core::PCWSTR, pwzcachefile: windows_core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetCacheHints2 {}
impl IWinInetCacheHints2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetCacheHints2_Vtbl
    where
        Identity: IWinInetCacheHints2_Impl,
    {
        unsafe extern "system" fn SetCacheExtension2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzext: windows_core::PCWSTR, pwzcachefile: windows_core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWinInetCacheHints2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetCacheHints2_Impl::SetCacheExtension2(this, core::mem::transmute(&pwzext), core::mem::transmute_copy(&pwzcachefile), core::mem::transmute_copy(&pcchcachefile), core::mem::transmute_copy(&pdwwinineterror), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IWinInetCacheHints_Vtbl::new::<Identity, OFFSET>(), SetCacheExtension2: SetCacheExtension2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetCacheHints2 as windows_core::Interface>::IID || iid == &<IWinInetCacheHints as windows_core::Interface>::IID
    }
}
pub trait IWinInetFileStream_Impl: Sized {
    fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::Result<()>;
    fn SetDeleteFile(&self, dwreserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetFileStream {}
impl IWinInetFileStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetFileStream_Vtbl
    where
        Identity: IWinInetFileStream_Impl,
    {
        unsafe extern "system" fn SetHandleForUnlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IWinInetFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetFileStream_Impl::SetHandleForUnlock(this, core::mem::transmute_copy(&hwininetlockhandle), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetDeleteFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IWinInetFileStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetFileStream_Impl::SetDeleteFile(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHandleForUnlock: SetHandleForUnlock::<Identity, OFFSET>,
            SetDeleteFile: SetDeleteFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetFileStream as windows_core::Interface>::IID
    }
}
pub trait IWinInetHttpInfo_Impl: Sized + IWinInetInfo_Impl {
    fn QueryInfo(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetHttpInfo {}
impl IWinInetHttpInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetHttpInfo_Vtbl
    where
        Identity: IWinInetHttpInfo_Impl,
    {
        unsafe extern "system" fn QueryInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWinInetHttpInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetHttpInfo_Impl::QueryInfo(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbbuf), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IWinInetInfo_Vtbl::new::<Identity, OFFSET>(), QueryInfo: QueryInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetHttpInfo as windows_core::Interface>::IID || iid == &<IWinInetInfo as windows_core::Interface>::IID
    }
}
pub trait IWinInetHttpTimeouts_Impl: Sized {
    fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetHttpTimeouts {}
impl IWinInetHttpTimeouts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetHttpTimeouts_Vtbl
    where
        Identity: IWinInetHttpTimeouts_Impl,
    {
        unsafe extern "system" fn GetRequestTimeouts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWinInetHttpTimeouts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetHttpTimeouts_Impl::GetRequestTimeouts(this, core::mem::transmute_copy(&pdwconnecttimeout), core::mem::transmute_copy(&pdwsendtimeout), core::mem::transmute_copy(&pdwreceivetimeout)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestTimeouts: GetRequestTimeouts::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetHttpTimeouts as windows_core::Interface>::IID
    }
}
pub trait IWinInetInfo_Impl: Sized {
    fn QueryOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWinInetInfo {}
impl IWinInetInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWinInetInfo_Vtbl
    where
        Identity: IWinInetInfo_Impl,
    {
        unsafe extern "system" fn QueryOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWinInetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWinInetInfo_Impl::QueryOption(this, core::mem::transmute_copy(&dwoption), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcbbuf)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryOption: QueryOption::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWinInetInfo as windows_core::Interface>::IID
    }
}
pub trait IWindowForBindingUI_Impl: Sized {
    fn GetWindow(&self, rguidreason: *const windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for IWindowForBindingUI {}
impl IWindowForBindingUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowForBindingUI_Vtbl
    where
        Identity: IWindowForBindingUI_Impl,
    {
        unsafe extern "system" fn GetWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidreason: *const windows_core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWindowForBindingUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowForBindingUI_Impl::GetWindow(this, core::mem::transmute_copy(&rguidreason)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowForBindingUI as windows_core::Interface>::IID
    }
}
pub trait IWrappedProtocol_Impl: Sized {
    fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWrappedProtocol {}
impl IWrappedProtocol_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWrappedProtocol_Vtbl
    where
        Identity: IWrappedProtocol_Impl,
    {
        unsafe extern "system" fn GetWrapperCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IWrappedProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWrappedProtocol_Impl::GetWrapperCode(this, core::mem::transmute_copy(&pncode), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWrapperCode: GetWrapperCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWrappedProtocol as windows_core::Interface>::IID
    }
}
pub trait IZoneIdentifier_Impl: Sized {
    fn GetId(&self) -> windows_core::Result<u32>;
    fn SetId(&self, dwzone: u32) -> windows_core::Result<()>;
    fn Remove(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IZoneIdentifier {}
impl IZoneIdentifier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IZoneIdentifier_Vtbl
    where
        Identity: IZoneIdentifier_Impl,
    {
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwzone: *mut u32) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IZoneIdentifier_Impl::GetId(this) {
                Ok(ok__) => {
                    pdwzone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwzone: u32) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier_Impl::SetId(this, core::mem::transmute_copy(&dwzone)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier_Impl::Remove(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoneIdentifier as windows_core::Interface>::IID
    }
}
pub trait IZoneIdentifier2_Impl: Sized + IZoneIdentifier_Impl {
    fn GetLastWriterPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLastWriterPackageFamilyName(&self, packagefamilyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(&self) -> windows_core::Result<()>;
    fn GetAppZoneId(&self) -> windows_core::Result<u32>;
    fn SetAppZoneId(&self, zone: u32) -> windows_core::Result<()>;
    fn RemoveAppZoneId(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IZoneIdentifier2 {}
impl IZoneIdentifier2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IZoneIdentifier2_Vtbl
    where
        Identity: IZoneIdentifier2_Impl,
    {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IZoneIdentifier2_Impl::GetLastWriterPackageFamilyName(this) {
                Ok(ok__) => {
                    packagefamilyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier2_Impl::SetLastWriterPackageFamilyName(this, core::mem::transmute(&packagefamilyname)).into()
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier2_Impl::RemoveLastWriterPackageFamilyName(this).into()
        }
        unsafe extern "system" fn GetAppZoneId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, zone: *mut u32) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IZoneIdentifier2_Impl::GetAppZoneId(this) {
                Ok(ok__) => {
                    zone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppZoneId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, zone: u32) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier2_Impl::SetAppZoneId(this, core::mem::transmute_copy(&zone)).into()
        }
        unsafe extern "system" fn RemoveAppZoneId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IZoneIdentifier2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IZoneIdentifier2_Impl::RemoveAppZoneId(this).into()
        }
        Self {
            base__: IZoneIdentifier_Vtbl::new::<Identity, OFFSET>(),
            GetLastWriterPackageFamilyName: GetLastWriterPackageFamilyName::<Identity, OFFSET>,
            SetLastWriterPackageFamilyName: SetLastWriterPackageFamilyName::<Identity, OFFSET>,
            RemoveLastWriterPackageFamilyName: RemoveLastWriterPackageFamilyName::<Identity, OFFSET>,
            GetAppZoneId: GetAppZoneId::<Identity, OFFSET>,
            SetAppZoneId: SetAppZoneId::<Identity, OFFSET>,
            RemoveAppZoneId: RemoveAppZoneId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IZoneIdentifier2 as windows_core::Interface>::IID || iid == &<IZoneIdentifier as windows_core::Interface>::IID
    }
}
