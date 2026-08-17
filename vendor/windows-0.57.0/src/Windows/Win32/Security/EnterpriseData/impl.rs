pub trait IProtectionPolicyManagerInterop_Impl: Sized {
    fn RequestAccessForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop {}
impl IProtectionPolicyManagerInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>() -> IProtectionPolicyManagerInterop_Vtbl {
        unsafe extern "system" fn RequestAccessForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, targetidentity: core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop_Impl::RequestAccessForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn GetForWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop, OFFSET>(),
            RequestAccessForWindowAsync: RequestAccessForWindowAsync::<Identity, Impl, OFFSET>,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop as windows_core::Interface>::IID
    }
}
pub trait IProtectionPolicyManagerInterop2_Impl: Sized {
    fn RequestAccessForAppWithWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessWithAuditingInfoForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessWithMessageForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithAuditingInfoForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithMessageForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop2 {}
impl IProtectionPolicyManagerInterop2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>() -> IProtectionPolicyManagerInterop2_Vtbl {
        unsafe extern "system" fn RequestAccessForAppWithWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessWithAuditingInfoForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, targetidentity: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop2_Impl::RequestAccessWithAuditingInfoForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessWithMessageForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, targetidentity: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop2_Impl::RequestAccessWithMessageForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithAuditingInfoForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithMessageForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop2, OFFSET>(),
            RequestAccessForAppWithWindowAsync: RequestAccessForAppWithWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessWithAuditingInfoForWindowAsync: RequestAccessWithAuditingInfoForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessWithMessageForWindowAsync: RequestAccessWithMessageForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithAuditingInfoForWindowAsync: RequestAccessForAppWithAuditingInfoForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithMessageForWindowAsync: RequestAccessForAppWithMessageForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop2 as windows_core::Interface>::IID
    }
}
pub trait IProtectionPolicyManagerInterop3_Impl: Sized {
    fn RequestAccessWithBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForAppForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: Option<&windows_core::IUnknown>, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: Option<&windows_core::IUnknown>, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForProcessForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: Option<&windows_core::IUnknown>, processid: u32, auditinfounk: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: Option<&windows_core::IUnknown>, processid: u32, auditinfounk: Option<&windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop3 {}
impl IProtectionPolicyManagerInterop3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>() -> IProtectionPolicyManagerInterop3_Vtbl {
        unsafe extern "system" fn RequestAccessWithBehaviorForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, targetidentity: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessWithBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: core::mem::MaybeUninit<windows_core::HSTRING>, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessForAppWithBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForAppForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&sourceitemlistunk), core::mem::transmute(&apppackagefamilyname), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, apppackagefamilyname: core::mem::MaybeUninit<windows_core::HSTRING>, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&sourceitemlistunk), core::mem::transmute(&apppackagefamilyname), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, processid: u32, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForProcessForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&sourceitemlistunk), core::mem::transmute_copy(&processid), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, processid: u32, auditinfounk: *mut core::ffi::c_void, messagefromapp: core::mem::MaybeUninit<windows_core::HSTRING>, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), windows_core::from_raw_borrowed(&sourceitemlistunk), core::mem::transmute_copy(&processid), windows_core::from_raw_borrowed(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop3, OFFSET>(),
            RequestAccessWithBehaviorForWindowAsync: RequestAccessWithBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessForAppWithBehaviorForWindowAsync: RequestAccessForAppWithBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForAppForWindowAsync: RequestAccessToFilesForAppForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForProcessForWindowAsync: RequestAccessToFilesForProcessForWindowAsync::<Identity, Impl, OFFSET>,
            RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop3 as windows_core::Interface>::IID
    }
}
