#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDialBranding_Impl: Sized {
    fn Initialize(&self, pwzconnectoid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBitmap(&self, dwindex: u32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDialBranding {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDialBranding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDialBranding_Vtbl
    where
        Identity: IDialBranding_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzconnectoid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDialBranding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialBranding_Impl::Initialize(this, core::mem::transmute(&pwzconnectoid)).into()
        }
        unsafe extern "system" fn GetBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT
        where
            Identity: IDialBranding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDialBranding_Impl::GetBitmap(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    phbitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetBitmap: GetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialBranding as windows_core::Interface>::IID
    }
}
pub trait IDialEngine_Impl: Sized {
    fn Initialize(&self, pwzconnectoid: &windows_core::PCWSTR, pides: Option<&IDialEventSink>) -> windows_core::Result<()>;
    fn GetProperty(&self, pwzproperty: &windows_core::PCWSTR, pwzvalue: &windows_core::PCWSTR, dwbufsize: u32) -> windows_core::Result<()>;
    fn SetProperty(&self, pwzproperty: &windows_core::PCWSTR, pwzvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Dial(&self) -> windows_core::Result<()>;
    fn HangUp(&self) -> windows_core::Result<()>;
    fn GetConnectedState(&self) -> windows_core::Result<u32>;
    fn GetConnectHandle(&self) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IDialEngine {}
impl IDialEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDialEngine_Vtbl
    where
        Identity: IDialEngine_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzconnectoid: windows_core::PCWSTR, pides: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEngine_Impl::Initialize(this, core::mem::transmute(&pwzconnectoid), windows_core::from_raw_borrowed(&pides)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzproperty: windows_core::PCWSTR, pwzvalue: windows_core::PCWSTR, dwbufsize: u32) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEngine_Impl::GetProperty(this, core::mem::transmute(&pwzproperty), core::mem::transmute(&pwzvalue), core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzproperty: windows_core::PCWSTR, pwzvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEngine_Impl::SetProperty(this, core::mem::transmute(&pwzproperty), core::mem::transmute(&pwzvalue)).into()
        }
        unsafe extern "system" fn Dial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEngine_Impl::Dial(this).into()
        }
        unsafe extern "system" fn HangUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEngine_Impl::HangUp(this).into()
        }
        unsafe extern "system" fn GetConnectedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDialEngine_Impl::GetConnectedState(this) {
                Ok(ok__) => {
                    pdwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDialEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDialEngine_Impl::GetConnectHandle(this) {
                Ok(ok__) => {
                    pdwhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Dial: Dial::<Identity, OFFSET>,
            HangUp: HangUp::<Identity, OFFSET>,
            GetConnectedState: GetConnectedState::<Identity, OFFSET>,
            GetConnectHandle: GetConnectHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialEngine as windows_core::Interface>::IID
    }
}
pub trait IDialEventSink_Impl: Sized {
    fn OnEvent(&self, dwevent: u32, dwstatus: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDialEventSink {}
impl IDialEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDialEventSink_Vtbl
    where
        Identity: IDialEventSink_Impl,
    {
        unsafe extern "system" fn OnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwevent: u32, dwstatus: u32) -> windows_core::HRESULT
        where
            Identity: IDialEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDialEventSink_Impl::OnEvent(this, core::mem::transmute_copy(&dwevent), core::mem::transmute_copy(&dwstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialEventSink as windows_core::Interface>::IID
    }
}
pub trait IProofOfPossessionCookieInfoManager_Impl: Sized {
    fn GetCookieInfoForUri(&self, uri: &windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProofOfPossessionCookieInfoManager {}
impl IProofOfPossessionCookieInfoManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProofOfPossessionCookieInfoManager_Vtbl
    where
        Identity: IProofOfPossessionCookieInfoManager_Impl,
    {
        unsafe extern "system" fn GetCookieInfoForUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT
        where
            Identity: IProofOfPossessionCookieInfoManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProofOfPossessionCookieInfoManager_Impl::GetCookieInfoForUri(this, core::mem::transmute(&uri), core::mem::transmute_copy(&cookieinfocount), core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCookieInfoForUri: GetCookieInfoForUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager as windows_core::Interface>::IID
    }
}
pub trait IProofOfPossessionCookieInfoManager2_Impl: Sized {
    fn GetCookieInfoWithUriForAccount(&self, webaccount: Option<&windows_core::IInspectable>, uri: &windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProofOfPossessionCookieInfoManager2 {}
impl IProofOfPossessionCookieInfoManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProofOfPossessionCookieInfoManager2_Vtbl
    where
        Identity: IProofOfPossessionCookieInfoManager2_Impl,
    {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, webaccount: *mut core::ffi::c_void, uri: windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT
        where
            Identity: IProofOfPossessionCookieInfoManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProofOfPossessionCookieInfoManager2_Impl::GetCookieInfoWithUriForAccount(this, windows_core::from_raw_borrowed(&webaccount), core::mem::transmute(&uri), core::mem::transmute_copy(&cookieinfocount), core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCookieInfoWithUriForAccount: GetCookieInfoWithUriForAccount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager2 as windows_core::Interface>::IID
    }
}
