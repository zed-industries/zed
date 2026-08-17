#[inline]
pub unsafe fn ProtectFileToEnterpriseIdentity<P0, P1>(fileorfolderpath: P0, identity: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("efswrt.dll" "system" fn ProtectFileToEnterpriseIdentity(fileorfolderpath : windows_core::PCWSTR, identity : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ProtectFileToEnterpriseIdentity(fileorfolderpath.param().abi(), identity.param().abi()).ok() }
}
#[inline]
pub unsafe fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("srpapi.dll" "system" fn SrpCloseThreadNetworkContext(threadnetworkcontext : *mut HTHREAD_NETWORK_CONTEXT) -> windows_core::HRESULT);
    unsafe { SrpCloseThreadNetworkContext(threadnetworkcontext as _).ok() }
}
#[inline]
pub unsafe fn SrpCreateThreadNetworkContext<P0>(enterpriseid: P0) -> windows_core::Result<HTHREAD_NETWORK_CONTEXT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("srpapi.dll" "system" fn SrpCreateThreadNetworkContext(enterpriseid : windows_core::PCWSTR, threadnetworkcontext : *mut HTHREAD_NETWORK_CONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SrpCreateThreadNetworkContext(enterpriseid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SrpDisablePermissiveModeFileEncryption() -> windows_core::Result<()> {
    windows_core::link!("srpapi.dll" "system" fn SrpDisablePermissiveModeFileEncryption() -> windows_core::HRESULT);
    unsafe { SrpDisablePermissiveModeFileEncryption().ok() }
}
#[cfg(feature = "Win32_Storage_Packaging_Appx")]
#[inline]
pub unsafe fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("srpapi.dll" "system" fn SrpDoesPolicyAllowAppExecution(packageid : *const super::super::Storage::Packaging::Appx:: PACKAGE_ID, isallowed : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SrpDoesPolicyAllowAppExecution(packageid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SrpEnablePermissiveModeFileEncryption<P0>(enterpriseid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("srpapi.dll" "system" fn SrpEnablePermissiveModeFileEncryption(enterpriseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SrpEnablePermissiveModeFileEncryption(enterpriseid.param().abi()).ok() }
}
#[inline]
pub unsafe fn SrpGetEnterpriseIds(tokenhandle: super::super::Foundation::HANDLE, numberofbytes: Option<*mut u32>, enterpriseids: Option<*mut windows_core::PCWSTR>, enterpriseidcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("srpapi.dll" "system" fn SrpGetEnterpriseIds(tokenhandle : super::super::Foundation:: HANDLE, numberofbytes : *mut u32, enterpriseids : *mut windows_core::PCWSTR, enterpriseidcount : *mut u32) -> windows_core::HRESULT);
    unsafe { SrpGetEnterpriseIds(tokenhandle, numberofbytes.unwrap_or(core::mem::zeroed()) as _, enterpriseids.unwrap_or(core::mem::zeroed()) as _, enterpriseidcount as _).ok() }
}
#[inline]
pub unsafe fn SrpGetEnterprisePolicy(tokenhandle: super::super::Foundation::HANDLE) -> windows_core::Result<ENTERPRISE_DATA_POLICIES> {
    windows_core::link!("srpapi.dll" "system" fn SrpGetEnterprisePolicy(tokenhandle : super::super::Foundation:: HANDLE, policyflags : *mut ENTERPRISE_DATA_POLICIES) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SrpGetEnterprisePolicy(tokenhandle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const core::ffi::c_void, cbdata: u32) -> windows_core::Result<()> {
    windows_core::link!("srpapi.dll" "system" fn SrpHostingInitialize(version : SRPHOSTING_VERSION, r#type : SRPHOSTING_TYPE, pvdata : *const core::ffi::c_void, cbdata : u32) -> windows_core::HRESULT);
    unsafe { SrpHostingInitialize(version, r#type, pvdata, cbdata).ok() }
}
#[inline]
pub unsafe fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE) {
    windows_core::link!("srpapi.dll" "system" fn SrpHostingTerminate(r#type : SRPHOSTING_TYPE));
    unsafe { SrpHostingTerminate(r#type) }
}
#[inline]
pub unsafe fn SrpIsTokenService(tokenhandle: super::super::Foundation::HANDLE, istokenservice: *mut u8) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("srpapi.dll" "system" fn SrpIsTokenService(tokenhandle : super::super::Foundation:: HANDLE, istokenservice : *mut u8) -> super::super::Foundation:: NTSTATUS);
    unsafe { SrpIsTokenService(tokenhandle, istokenservice as _) }
}
#[inline]
pub unsafe fn SrpSetTokenEnterpriseId<P1>(tokenhandle: super::super::Foundation::HANDLE, enterpriseid: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("srpapi.dll" "system" fn SrpSetTokenEnterpriseId(tokenhandle : super::super::Foundation:: HANDLE, enterpriseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SrpSetTokenEnterpriseId(tokenhandle, enterpriseid.param().abi()).ok() }
}
#[inline]
pub unsafe fn UnprotectFile<P0>(fileorfolderpath: P0, options: Option<*const FILE_UNPROTECT_OPTIONS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("efswrt.dll" "system" fn UnprotectFile(fileorfolderpath : windows_core::PCWSTR, options : *const FILE_UNPROTECT_OPTIONS) -> windows_core::HRESULT);
    unsafe { UnprotectFile(fileorfolderpath.param().abi(), options.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENTERPRISE_DATA_POLICIES(pub i32);
impl ENTERPRISE_DATA_POLICIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ENTERPRISE_DATA_POLICIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ENTERPRISE_DATA_POLICIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1i32);
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2i32);
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4i32);
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(IProtectionPolicyManagerInterop, IProtectionPolicyManagerInterop_Vtbl, 0x4652651d_c1fe_4ba1_9f0a_c0f56596f721);
windows_core::imp::interface_hierarchy!(IProtectionPolicyManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IProtectionPolicyManagerInterop {
    pub unsafe fn RequestAccessForWindowAsync<T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(targetidentity), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProtectionPolicyManagerInterop_Impl: windows_core::IUnknownImpl {
    fn RequestAccessForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IProtectionPolicyManagerInterop_Vtbl {
    pub const fn new<Identity: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestAccessForWindowAsync<Identity: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, targetidentity: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop_Impl::RequestAccessForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn GetForWindow<Identity: IProtectionPolicyManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop, OFFSET>(),
            RequestAccessForWindowAsync: RequestAccessForWindowAsync::<Identity, OFFSET>,
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop {}
windows_core::imp::define_interface!(IProtectionPolicyManagerInterop2, IProtectionPolicyManagerInterop2_Vtbl, 0x157cfbe4_a78d_4156_b384_61fdac41e686);
windows_core::imp::interface_hierarchy!(IProtectionPolicyManagerInterop2, windows_core::IUnknown, windows_core::IInspectable);
impl IProtectionPolicyManagerInterop2 {
    pub unsafe fn RequestAccessForAppWithWindowAsync<T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessForAppWithWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(apppackagefamilyname), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessWithAuditingInfoForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: P3) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessWithAuditingInfoForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(targetidentity), auditinfounk.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessWithMessageForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: P3, messagefromapp: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessWithMessageForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(targetidentity), auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessForAppWithAuditingInfoForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: P3) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessForAppWithAuditingInfoForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(apppackagefamilyname), auditinfounk.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessForAppWithMessageForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: P3, messagefromapp: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessForAppWithMessageForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(apppackagefamilyname), auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessForAppWithWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessWithAuditingInfoForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessWithMessageForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessForAppWithAuditingInfoForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessForAppWithMessageForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProtectionPolicyManagerInterop2_Impl: windows_core::IUnknownImpl {
    fn RequestAccessForAppWithWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessWithAuditingInfoForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessWithMessageForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithAuditingInfoForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithMessageForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IProtectionPolicyManagerInterop2_Vtbl {
    pub const fn new<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestAccessForAppWithWindowAsync<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessWithAuditingInfoForWindowAsync<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, targetidentity: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop2_Impl::RequestAccessWithAuditingInfoForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), core::mem::transmute_copy(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessWithMessageForWindowAsync<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, targetidentity: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop2_Impl::RequestAccessWithMessageForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithAuditingInfoForWindowAsync<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithAuditingInfoForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithMessageForWindowAsync<Identity: IProtectionPolicyManagerInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop2_Impl::RequestAccessForAppWithMessageForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop2, OFFSET>(),
            RequestAccessForAppWithWindowAsync: RequestAccessForAppWithWindowAsync::<Identity, OFFSET>,
            RequestAccessWithAuditingInfoForWindowAsync: RequestAccessWithAuditingInfoForWindowAsync::<Identity, OFFSET>,
            RequestAccessWithMessageForWindowAsync: RequestAccessWithMessageForWindowAsync::<Identity, OFFSET>,
            RequestAccessForAppWithAuditingInfoForWindowAsync: RequestAccessForAppWithAuditingInfoForWindowAsync::<Identity, OFFSET>,
            RequestAccessForAppWithMessageForWindowAsync: RequestAccessForAppWithMessageForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop2 {}
windows_core::imp::define_interface!(IProtectionPolicyManagerInterop3, IProtectionPolicyManagerInterop3_Vtbl, 0xc1c03933_b398_4d93_b0fd_2972adf802c2);
windows_core::imp::interface_hierarchy!(IProtectionPolicyManagerInterop3, windows_core::IUnknown, windows_core::IInspectable);
impl IProtectionPolicyManagerInterop3 {
    pub unsafe fn RequestAccessWithBehaviorForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: P3, messagefromapp: &windows_core::HSTRING, behavior: u32) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessWithBehaviorForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(targetidentity), auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), behavior, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessForAppWithBehaviorForWindowAsync<P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: P3, messagefromapp: &windows_core::HSTRING, behavior: u32) -> windows_core::Result<T>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessForAppWithBehaviorForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(sourceidentity), core::mem::transmute_copy(apppackagefamilyname), auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), behavior, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessToFilesForAppForWindowAsync<P1, P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: P1, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: P3) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessToFilesForAppForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, sourceitemlistunk.param().abi(), core::mem::transmute_copy(apppackagefamilyname), auditinfounk.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<P1, P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: P1, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: P3, messagefromapp: &windows_core::HSTRING, behavior: u32) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, sourceitemlistunk.param().abi(), core::mem::transmute_copy(apppackagefamilyname), auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), behavior, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessToFilesForProcessForWindowAsync<P1, P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: P1, processid: u32, auditinfounk: P3) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessToFilesForProcessForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, sourceitemlistunk.param().abi(), processid, auditinfounk.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<P1, P3, T>(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: P1, processid: u32, auditinfounk: P3, messagefromapp: &windows_core::HSTRING, behavior: u32) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, sourceitemlistunk.param().abi(), processid, auditinfounk.param().abi(), core::mem::transmute_copy(messagefromapp), behavior, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessWithBehaviorForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessForAppWithBehaviorForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessToFilesForAppForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessToFilesForProcessForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProtectionPolicyManagerInterop3_Impl: windows_core::IUnknownImpl {
    fn RequestAccessWithBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, targetidentity: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessForAppWithBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceidentity: &windows_core::HSTRING, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForAppForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: windows_core::Ref<windows_core::IUnknown>, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: windows_core::Ref<windows_core::IUnknown>, apppackagefamilyname: &windows_core::HSTRING, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForProcessForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: windows_core::Ref<windows_core::IUnknown>, processid: u32, auditinfounk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(&self, appwindow: super::super::Foundation::HWND, sourceitemlistunk: windows_core::Ref<windows_core::IUnknown>, processid: u32, auditinfounk: windows_core::Ref<windows_core::IUnknown>, messagefromapp: &windows_core::HSTRING, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IProtectionPolicyManagerInterop3_Vtbl {
    pub const fn new<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestAccessWithBehaviorForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, targetidentity: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessWithBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&targetidentity), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessForAppWithBehaviorForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessForAppWithBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&sourceidentity), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForAppForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForAppForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&sourceitemlistunk), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, apppackagefamilyname: *mut core::ffi::c_void, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&sourceitemlistunk), core::mem::transmute(&apppackagefamilyname), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, processid: u32, auditinfounk: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForProcessForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&sourceitemlistunk), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&auditinfounk), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        unsafe extern "system" fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<Identity: IProtectionPolicyManagerInterop3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut core::ffi::c_void, processid: u32, auditinfounk: *mut core::ffi::c_void, messagefromapp: *mut core::ffi::c_void, behavior: u32, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProtectionPolicyManagerInterop3_Impl::RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&sourceitemlistunk), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&auditinfounk), core::mem::transmute(&messagefromapp), core::mem::transmute_copy(&behavior), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProtectionPolicyManagerInterop3, OFFSET>(),
            RequestAccessWithBehaviorForWindowAsync: RequestAccessWithBehaviorForWindowAsync::<Identity, OFFSET>,
            RequestAccessForAppWithBehaviorForWindowAsync: RequestAccessForAppWithBehaviorForWindowAsync::<Identity, OFFSET>,
            RequestAccessToFilesForAppForWindowAsync: RequestAccessToFilesForAppForWindowAsync::<Identity, OFFSET>,
            RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync::<Identity, OFFSET>,
            RequestAccessToFilesForProcessForWindowAsync: RequestAccessToFilesForProcessForWindowAsync::<Identity, OFFSET>,
            RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtectionPolicyManagerInterop3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProtectionPolicyManagerInterop3 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SRPHOSTING_TYPE(pub i32);
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SRPHOSTING_VERSION(pub i32);
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
