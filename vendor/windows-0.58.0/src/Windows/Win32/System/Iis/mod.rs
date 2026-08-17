#[inline]
pub unsafe fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL {
    windows_targets::link!("rpcproxy.dll" "system" fn GetExtensionVersion(pver : *mut HSE_VERSION_INFO) -> super::super::Foundation:: BOOL);
    GetExtensionVersion(pver)
}
#[inline]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL {
    windows_targets::link!("rpcproxy.dll" "system" fn GetFilterVersion(pver : *mut HTTP_FILTER_VERSION) -> super::super::Foundation:: BOOL);
    GetFilterVersion(pver)
}
#[inline]
pub unsafe fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32 {
    windows_targets::link!("rpcproxy.dll" "system" fn HttpExtensionProc(pecb : *const EXTENSION_CONTROL_BLOCK) -> u32);
    HttpExtensionProc(pecb)
}
#[inline]
pub unsafe fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("rpcproxy.dll" "system" fn HttpFilterProc(pfc : *mut HTTP_FILTER_CONTEXT, notificationtype : u32, pvnotification : *mut core::ffi::c_void) -> u32);
    HttpFilterProc(pfc, notificationtype, pvnotification)
}
windows_core::imp::define_interface!(AsyncIFtpAuthenticationProvider, AsyncIFtpAuthenticationProvider_Vtbl, 0xc24efb65_9f3e_4996_8fb1_ce166916bab5);
impl core::ops::Deref for AsyncIFtpAuthenticationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpAuthenticationProvider, windows_core::IUnknown);
impl AsyncIFtpAuthenticationProvider {
    pub unsafe fn Begin_AuthenticateUser<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszpassword: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Begin_AuthenticateUser)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi(), pszpassword.param().abi()).ok()
    }
    pub unsafe fn Finish_AuthenticateUser(&self, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_AuthenticateUser)(windows_core::Interface::as_raw(self), ppszcanonicalusername, pfauthenticated).ok()
    }
}
#[repr(C)]
pub struct AsyncIFtpAuthenticationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_AuthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_AuthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpAuthorizationProvider, AsyncIFtpAuthorizationProvider_Vtbl, 0x860dc339_07e5_4a5c_9c61_8820cea012bc);
impl core::ops::Deref for AsyncIFtpAuthorizationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpAuthorizationProvider, windows_core::IUnknown);
impl AsyncIFtpAuthorizationProvider {
    pub unsafe fn Begin_GetUserAccessPermission<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszvirtualpath: P2, pszusername: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Begin_GetUserAccessPermission)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszvirtualpath.param().abi(), pszusername.param().abi()).ok()
    }
    pub unsafe fn Finish_GetUserAccessPermission(&self) -> windows_core::Result<FTP_ACCESS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Finish_GetUserAccessPermission)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct AsyncIFtpAuthorizationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_GetUserAccessPermission: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_GetUserAccessPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FTP_ACCESS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpHomeDirectoryProvider, AsyncIFtpHomeDirectoryProvider_Vtbl, 0x73f81638_6295_42bd_a2be_4a657f7c479c);
impl core::ops::Deref for AsyncIFtpHomeDirectoryProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpHomeDirectoryProvider, windows_core::IUnknown);
impl AsyncIFtpHomeDirectoryProvider {
    pub unsafe fn Begin_GetUserHomeDirectoryData<P0, P1, P2>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Begin_GetUserHomeDirectoryData)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi()).ok()
    }
    pub unsafe fn Finish_GetUserHomeDirectoryData(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Finish_GetUserHomeDirectoryData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct AsyncIFtpHomeDirectoryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_GetUserHomeDirectoryData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_GetUserHomeDirectoryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpLogProvider, AsyncIFtpLogProvider_Vtbl, 0x00a0ae46_2498_48b2_95e6_df678ed7d49f);
impl core::ops::Deref for AsyncIFtpLogProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpLogProvider, windows_core::IUnknown);
impl AsyncIFtpLogProvider {
    pub unsafe fn Begin_Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Log)(windows_core::Interface::as_raw(self), ploggingparameters).ok()
    }
    pub unsafe fn Finish_Log(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Log)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIFtpLogProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Log: unsafe extern "system" fn(*mut core::ffi::c_void, *const LOGGING_PARAMETERS) -> windows_core::HRESULT,
    pub Finish_Log: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpPostprocessProvider, AsyncIFtpPostprocessProvider_Vtbl, 0xa16b2542_9694_4eb1_a564_6c2e91fdc133);
impl core::ops::Deref for AsyncIFtpPostprocessProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpPostprocessProvider, windows_core::IUnknown);
impl AsyncIFtpPostprocessProvider {
    pub unsafe fn Begin_HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_HandlePostprocess)(windows_core::Interface::as_raw(self), ppostprocessparameters).ok()
    }
    pub unsafe fn Finish_HandlePostprocess(&self) -> windows_core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Finish_HandlePostprocess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct AsyncIFtpPostprocessProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_HandlePostprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *const POST_PROCESS_PARAMETERS) -> windows_core::HRESULT,
    pub Finish_HandlePostprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpPreprocessProvider, AsyncIFtpPreprocessProvider_Vtbl, 0x6ff5fd8f_fd8e_48b1_a3e0_bf7073db4db5);
impl core::ops::Deref for AsyncIFtpPreprocessProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpPreprocessProvider, windows_core::IUnknown);
impl AsyncIFtpPreprocessProvider {
    pub unsafe fn Begin_HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_HandlePreprocess)(windows_core::Interface::as_raw(self), ppreprocessparameters).ok()
    }
    pub unsafe fn Finish_HandlePreprocess(&self) -> windows_core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Finish_HandlePreprocess)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct AsyncIFtpPreprocessProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_HandlePreprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *const PRE_PROCESS_PARAMETERS) -> windows_core::HRESULT,
    pub Finish_HandlePreprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIFtpRoleProvider, AsyncIFtpRoleProvider_Vtbl, 0x3e83bf99_70ec_41ca_84b6_aca7c7a62caf);
impl core::ops::Deref for AsyncIFtpRoleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIFtpRoleProvider, windows_core::IUnknown);
impl AsyncIFtpRoleProvider {
    pub unsafe fn Begin_IsUserInRole<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszrole: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Begin_IsUserInRole)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi(), pszrole.param().abi()).ok()
    }
    pub unsafe fn Finish_IsUserInRole(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Finish_IsUserInRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct AsyncIFtpRoleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_IsUserInRole: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_IsUserInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIMSAdminBaseSinkW, AsyncIMSAdminBaseSinkW_Vtbl, 0xa9e69613_b80d_11d0_b9b9_00a0c922e750);
impl core::ops::Deref for AsyncIMSAdminBaseSinkW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIMSAdminBaseSinkW, windows_core::IUnknown);
impl AsyncIMSAdminBaseSinkW {
    pub unsafe fn Begin_SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_SinkNotify)(windows_core::Interface::as_raw(self), pcochangelist.len().try_into().unwrap(), core::mem::transmute(pcochangelist.as_ptr())).ok()
    }
    pub unsafe fn Finish_SinkNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_SinkNotify)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_ShutdownNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_ShutdownNotify)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_ShutdownNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_ShutdownNotify)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIMSAdminBaseSinkW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_SinkNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const MD_CHANGE_OBJECT_W) -> windows_core::HRESULT,
    pub Finish_SinkNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_ShutdownNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_ShutdownNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IADMEXT, IADMEXT_Vtbl, 0x51dfe970_f6f2_11d0_b9bd_00a0c922e750);
impl core::ops::Deref for IADMEXT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IADMEXT, windows_core::IUnknown);
impl IADMEXT {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDcomCLSIDs(&self, pclsiddcom: *mut windows_core::GUID, dwenumindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumDcomCLSIDs)(windows_core::Interface::as_raw(self), pclsiddcom, dwenumindex).ok()
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IADMEXT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDcomCLSIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpAuthenticationProvider, IFtpAuthenticationProvider_Vtbl, 0x4659f95c_d5a8_4707_b2fc_6fd5794246cf);
impl core::ops::Deref for IFtpAuthenticationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpAuthenticationProvider, windows_core::IUnknown);
impl IFtpAuthenticationProvider {
    pub unsafe fn AuthenticateUser<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszpassword: P3, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AuthenticateUser)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi(), pszpassword.param().abi(), ppszcanonicalusername, pfauthenticated).ok()
    }
}
#[repr(C)]
pub struct IFtpAuthenticationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AuthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpAuthorizationProvider, IFtpAuthorizationProvider_Vtbl, 0xa50ae7a1_a35a_42b4_a4f3_f4f7057a05d1);
impl core::ops::Deref for IFtpAuthorizationProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpAuthorizationProvider, windows_core::IUnknown);
impl IFtpAuthorizationProvider {
    pub unsafe fn GetUserAccessPermission<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszvirtualpath: P2, pszusername: P3) -> windows_core::Result<FTP_ACCESS>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserAccessPermission)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszvirtualpath.param().abi(), pszusername.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFtpAuthorizationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserAccessPermission: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut FTP_ACCESS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpHomeDirectoryProvider, IFtpHomeDirectoryProvider_Vtbl, 0x0933b392_18dd_4097_8b9c_83325c35d9a6);
impl core::ops::Deref for IFtpHomeDirectoryProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpHomeDirectoryProvider, windows_core::IUnknown);
impl IFtpHomeDirectoryProvider {
    pub unsafe fn GetUserHomeDirectoryData<P0, P1, P2>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserHomeDirectoryData)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFtpHomeDirectoryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserHomeDirectoryData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpLogProvider, IFtpLogProvider_Vtbl, 0xa18a94cc_8299_4408_816c_7c3baca1a40e);
impl core::ops::Deref for IFtpLogProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpLogProvider, windows_core::IUnknown);
impl IFtpLogProvider {
    pub unsafe fn Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), ploggingparameters).ok()
    }
}
#[repr(C)]
pub struct IFtpLogProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, *const LOGGING_PARAMETERS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpPostprocessProvider, IFtpPostprocessProvider_Vtbl, 0x4522cbc6_16cd_49ad_8653_9a2c579e4280);
impl core::ops::Deref for IFtpPostprocessProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpPostprocessProvider, windows_core::IUnknown);
impl IFtpPostprocessProvider {
    pub unsafe fn HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> windows_core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HandlePostprocess)(windows_core::Interface::as_raw(self), ppostprocessparameters, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFtpPostprocessProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandlePostprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *const POST_PROCESS_PARAMETERS, *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpPreprocessProvider, IFtpPreprocessProvider_Vtbl, 0xa3c19b60_5a28_471a_8f93_ab30411cee82);
impl core::ops::Deref for IFtpPreprocessProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpPreprocessProvider, windows_core::IUnknown);
impl IFtpPreprocessProvider {
    pub unsafe fn HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> windows_core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HandlePreprocess)(windows_core::Interface::as_raw(self), ppreprocessparameters, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFtpPreprocessProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandlePreprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *const PRE_PROCESS_PARAMETERS, *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFtpProviderConstruct, IFtpProviderConstruct_Vtbl, 0x4d1a3f7b_412d_447c_b199_64f967e9a2da);
impl core::ops::Deref for IFtpProviderConstruct {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpProviderConstruct, windows_core::IUnknown);
impl IFtpProviderConstruct {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct(&self, configurationentries: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Construct)(windows_core::Interface::as_raw(self), configurationentries).ok()
    }
}
#[repr(C)]
pub struct IFtpProviderConstruct_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
windows_core::imp::define_interface!(IFtpRoleProvider, IFtpRoleProvider_Vtbl, 0x909c850d_8ca0_4674_96b8_cc2941535725);
impl core::ops::Deref for IFtpRoleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFtpRoleProvider, windows_core::IUnknown);
impl IFtpRoleProvider {
    pub unsafe fn IsUserInRole<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszrole: P3) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUserInRole)(windows_core::Interface::as_raw(self), pszsessionid.param().abi(), pszsitename.param().abi(), pszusername.param().abi(), pszrole.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFtpRoleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsUserInRole: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMSAdminBase2W, IMSAdminBase2W_Vtbl, 0x8298d101_f992_43b7_8eca_5052d885b995);
impl core::ops::Deref for IMSAdminBase2W {
    type Target = IMSAdminBaseW;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMSAdminBase2W, windows_core::IUnknown, IMSAdminBaseW);
impl IMSAdminBase2W {
    pub unsafe fn BackupWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BackupWithPasswd)(windows_core::Interface::as_raw(self), pszmdbackuplocation.param().abi(), dwmdversion, dwmdflags, pszpasswd.param().abi()).ok()
    }
    pub unsafe fn RestoreWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RestoreWithPasswd)(windows_core::Interface::as_raw(self), pszmdbackuplocation.param().abi(), dwmdversion, dwmdflags, pszpasswd.param().abi()).ok()
    }
    pub unsafe fn Export<P0, P1, P2>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, dwmdflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), pszpasswd.param().abi(), pszfilename.param().abi(), pszsourcepath.param().abi(), dwmdflags).ok()
    }
    pub unsafe fn Import<P0, P1, P2, P3>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, pszdestpath: P3, dwmdflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), pszpasswd.param().abi(), pszfilename.param().abi(), pszsourcepath.param().abi(), pszdestpath.param().abi(), dwmdflags).ok()
    }
    pub unsafe fn RestoreHistory<P0>(&self, pszmdhistorylocation: P0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RestoreHistory)(windows_core::Interface::as_raw(self), pszmdhistorylocation.param().abi(), dwmdmajorversion, dwmdminorversion, dwmdflags).ok()
    }
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumHistory)(windows_core::Interface::as_raw(self), core::mem::transmute(pszmdhistorylocation.as_ptr()), pdwmdmajorversion, pdwmdminorversion, pftmdhistorytime, dwmdenumindex).ok()
    }
}
#[repr(C)]
pub struct IMSAdminBase2W_Vtbl {
    pub base__: IMSAdminBaseW_Vtbl,
    pub BackupWithPasswd: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RestoreWithPasswd: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub RestoreHistory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32) -> windows_core::HRESULT,
    pub EnumHistory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, *mut u32, *mut super::super::Foundation::FILETIME, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMSAdminBase3W, IMSAdminBase3W_Vtbl, 0xf612954d_3b0b_4c56_9563_227b7be624b4);
impl core::ops::Deref for IMSAdminBase3W {
    type Target = IMSAdminBase2W;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMSAdminBase3W, windows_core::IUnknown, IMSAdminBaseW, IMSAdminBase2W);
impl IMSAdminBase3W {
    pub unsafe fn GetChildPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszbuffer: Option<&mut [u16]>, pcchmdrequiredbuffersize: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetChildPaths)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pszbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pszbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(pcchmdrequiredbuffersize.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IMSAdminBase3W_Vtbl {
    pub base__: IMSAdminBase2W_Vtbl,
    pub GetChildPaths: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMSAdminBaseSinkW, IMSAdminBaseSinkW_Vtbl, 0xa9e69612_b80d_11d0_b9b9_00a0c922e750);
impl core::ops::Deref for IMSAdminBaseSinkW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMSAdminBaseSinkW, windows_core::IUnknown);
impl IMSAdminBaseSinkW {
    pub unsafe fn SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SinkNotify)(windows_core::Interface::as_raw(self), pcochangelist.len().try_into().unwrap(), core::mem::transmute(pcochangelist.as_ptr())).ok()
    }
    pub unsafe fn ShutdownNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownNotify)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMSAdminBaseSinkW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SinkNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const MD_CHANGE_OBJECT_W) -> windows_core::HRESULT,
    pub ShutdownNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMSAdminBaseW, IMSAdminBaseW_Vtbl, 0x70b51430_b6ca_11d0_b9b9_00a0c922e750);
impl core::ops::Deref for IMSAdminBaseW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMSAdminBaseW, windows_core::IUnknown);
impl IMSAdminBaseW {
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddKey)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteKey)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteChildKeys)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumKeys)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CopyKey)(windows_core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.param().abi(), hmddesthandle, pszmddestpath.param().abi(), bmdoverwriteflag.param().abi(), bmdcopyflag.param().abi()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RenameKey)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pszmdnewname.param().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAllData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteAllData)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), dwmdusertype, dwmddatatype).ok()
    }
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CopyData)(windows_core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.param().abi(), hmddesthandle, pszmddestpath.param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.param().abi()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDataPaths)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len().try_into().unwrap(), core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenKey)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), dwmdaccessrequested, dwmdtimeout, &mut result__).map(|| result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseKey)(windows_core::Interface::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangePermissions)(windows_core::Interface::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveData)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> windows_core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHandleInfo)(windows_core::Interface::as_raw(self), hmdhandle, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSystemChangeNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataSetNumber)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLastChangeTime)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pftmdlastchangetime, blocaltime.param().abi()).ok()
    }
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetLastChangeTime)(windows_core::Interface::as_raw(self), hmdhandle, pszmdpath.param().abi(), pftmdlastchangetime, blocaltime.param().abi()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KeyExchangePhase1)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KeyExchangePhase2)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), pszmdbackuplocation.param().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), pszmdbackuplocation.param().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumBackups)(windows_core::Interface::as_raw(self), core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteBackup)(windows_core::Interface::as_raw(self), pszmdbackuplocation.param().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> windows_core::Result<IMSAdminBaseW> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UnmarshalInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetServerGuid(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetServerGuid)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMSAdminBaseW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteChildKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub CopyKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RenameKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut METADATA_RECORD) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut METADATA_RECORD, *mut u32) -> windows_core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub EnumData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut METADATA_RECORD, u32, *mut u32) -> windows_core::HRESULT,
    pub GetAllData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, u32, *mut u32, *mut u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub DeleteAllData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub CopyData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, u32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDataPaths: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub OpenKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub CloseKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ChangePermissions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub SaveData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHandleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut METADATA_HANDLE_INFO) -> windows_core::HRESULT,
    pub GetSystemChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataSetNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetLastChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLastChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut super::super::Foundation::FILETIME, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub KeyExchangePhase1: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyExchangePhase2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub EnumBackups: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, *mut super::super::Foundation::FILETIME, u32) -> windows_core::HRESULT,
    pub DeleteBackup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServerGuid: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMSImpExpHelpW, IMSImpExpHelpW_Vtbl, 0x29ff67ff_8050_480f_9f30_cc41635f2f9d);
impl core::ops::Deref for IMSImpExpHelpW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMSImpExpHelpW, windows_core::IUnknown);
impl IMSImpExpHelpW {
    pub unsafe fn EnumeratePathsInFile<P0, P1>(&self, pszfilename: P0, pszkeytype: P1, pszbuffer: Option<&mut [u16]>, pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumeratePathsInFile)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), pszkeytype.param().abi(), pszbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pszbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwmdrequiredbuffersize).ok()
    }
}
#[repr(C)]
pub struct IMSImpExpHelpW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumeratePathsInFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub const ADMINDATA_MAX_NAME_LEN: u32 = 256u32;
pub const ALL_METADATA: METADATATYPES = METADATATYPES(0i32);
pub const APPCTR_MD_ID_BEGIN_RESERVED: u32 = 57344u32;
pub const APPCTR_MD_ID_END_RESERVED: u32 = 61439u32;
pub const APPSTATUS_NOTDEFINED: u32 = 2u32;
pub const APPSTATUS_RUNNING: u32 = 1u32;
pub const APPSTATUS_STOPPED: u32 = 0u32;
pub const ASP_MD_ID_BEGIN_RESERVED: u32 = 28672u32;
pub const ASP_MD_ID_END_RESERVED: u32 = 29951u32;
pub const ASP_MD_SERVER_BASE: u32 = 7000u32;
pub const ASP_MD_UT_APP: u32 = 101u32;
pub const BINARY_METADATA: METADATATYPES = METADATATYPES(3i32);
pub const CLSID_IImgCtx: windows_core::GUID = windows_core::GUID::from_u128(0x3050f3d6_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_IisServiceControl: windows_core::GUID = windows_core::GUID::from_u128(0xe8fb8621_588f_11d2_9d61_00c04f79c5fe);
pub const CLSID_MSAdminBase_W: windows_core::GUID = windows_core::GUID::from_u128(0xa9e69610_b80d_11d0_b9b9_00a0c922e750);
pub const CLSID_Request: windows_core::GUID = windows_core::GUID::from_u128(0x920c25d0_25d9_11d0_a55f_00a0c90c2091);
pub const CLSID_Response: windows_core::GUID = windows_core::GUID::from_u128(0x46e19ba0_25dd_11d0_a55f_00a0c90c2091);
pub const CLSID_ScriptingContext: windows_core::GUID = windows_core::GUID::from_u128(0xd97a6da0_a868_11cf_83ae_11b0c90c2bd8);
pub const CLSID_Server: windows_core::GUID = windows_core::GUID::from_u128(0xa506d160_25e0_11d0_a55f_00a0c90c2091);
pub const CLSID_Session: windows_core::GUID = windows_core::GUID::from_u128(0x509f8f20_25de_11d0_a55f_00a0c90c2091);
pub const CLSID_WamAdmin: windows_core::GUID = windows_core::GUID::from_u128(0x61738644_f196_11d0_9953_00c04fd919c1);
pub const DISPID_HTTPREQUEST_ABORT: u32 = 12u32;
pub const DISPID_HTTPREQUEST_BASE: u32 = 1u32;
pub const DISPID_HTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 4u32;
pub const DISPID_HTTPREQUEST_GETRESPONSEHEADER: u32 = 3u32;
pub const DISPID_HTTPREQUEST_OPEN: u32 = 1u32;
pub const DISPID_HTTPREQUEST_OPTION: u32 = 6u32;
pub const DISPID_HTTPREQUEST_RESPONSEBODY: u32 = 10u32;
pub const DISPID_HTTPREQUEST_RESPONSESTREAM: u32 = 11u32;
pub const DISPID_HTTPREQUEST_RESPONSETEXT: u32 = 9u32;
pub const DISPID_HTTPREQUEST_SEND: u32 = 5u32;
pub const DISPID_HTTPREQUEST_SETAUTOLOGONPOLICY: u32 = 18u32;
pub const DISPID_HTTPREQUEST_SETCLIENTCERTIFICATE: u32 = 17u32;
pub const DISPID_HTTPREQUEST_SETCREDENTIALS: u32 = 14u32;
pub const DISPID_HTTPREQUEST_SETPROXY: u32 = 13u32;
pub const DISPID_HTTPREQUEST_SETREQUESTHEADER: u32 = 2u32;
pub const DISPID_HTTPREQUEST_SETTIMEOUTS: u32 = 16u32;
pub const DISPID_HTTPREQUEST_STATUS: u32 = 7u32;
pub const DISPID_HTTPREQUEST_STATUSTEXT: u32 = 8u32;
pub const DISPID_HTTPREQUEST_WAITFORRESPONSE: u32 = 15u32;
pub const DWN_COLORMODE: u32 = 63u32;
pub const DWN_DOWNLOADONLY: u32 = 64u32;
pub const DWN_FORCEDITHER: u32 = 128u32;
pub const DWN_MIRRORIMAGE: u32 = 512u32;
pub const DWN_RAWIMAGE: u32 = 256u32;
pub const DWORD_METADATA: METADATATYPES = METADATATYPES(1i32);
pub const EXPANDSZ_METADATA: METADATATYPES = METADATATYPES(4i32);
pub const FP_MD_ID_BEGIN_RESERVED: u32 = 32768u32;
pub const FP_MD_ID_END_RESERVED: u32 = 36863u32;
pub const FTP_ACCESS_NONE: FTP_ACCESS = FTP_ACCESS(0i32);
pub const FTP_ACCESS_READ: FTP_ACCESS = FTP_ACCESS(1i32);
pub const FTP_ACCESS_READ_WRITE: FTP_ACCESS = FTP_ACCESS(3i32);
pub const FTP_ACCESS_WRITE: FTP_ACCESS = FTP_ACCESS(2i32);
pub const FTP_PROCESS_CLOSE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(1i32);
pub const FTP_PROCESS_CONTINUE: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(0i32);
pub const FTP_PROCESS_REJECT_COMMAND: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(3i32);
pub const FTP_PROCESS_TERMINATE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(2i32);
pub const GUID_IIS_ALL_TRACE_PROVIDERS: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_IIS_ASPNET_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xaff081fe_0247_4275_9c4e_021f3dc1da35);
pub const GUID_IIS_ASP_TRACE_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x06b94d9a_b15e_456e_a4ef_37c984a2cb4b);
pub const GUID_IIS_ISAPI_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xa1c2040e_8840_4c31_ba11_9871031a19ea);
pub const GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xd55d3bc9_cba9_44df_827e_132d3a4596c2);
pub const GUID_IIS_WWW_SERVER_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x3a2a4e84_4c21_4981_ae10_3fda0d9b0f83);
pub const GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xde4649c9_15e8_4fea_9d85_1cdda520c334);
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003u32;
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0u32;
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1u32;
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2u32;
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32u32;
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: u32 = 128u32;
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: u32 = 4u32;
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: u32 = 16u32;
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2u32;
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64u32;
pub const HSE_IO_ASYNC: u32 = 2u32;
pub const HSE_IO_CACHE_RESPONSE: u32 = 32u32;
pub const HSE_IO_DISCONNECT_AFTER_SEND: u32 = 4u32;
pub const HSE_IO_FINAL_SEND: u32 = 16u32;
pub const HSE_IO_NODELAY: u32 = 4096u32;
pub const HSE_IO_SEND_HEADERS: u32 = 8u32;
pub const HSE_IO_SYNC: u32 = 1u32;
pub const HSE_IO_TRY_SKIP_CUSTOM_ERRORS: u32 = 64u32;
pub const HSE_LOG_BUFFER_LEN: u32 = 80u32;
pub const HSE_MAX_EXT_DLL_NAME_LEN: u32 = 256u32;
pub const HSE_REQ_ABORTIVE_CLOSE: u32 = 1014u32;
pub const HSE_REQ_ASYNC_READ_CLIENT: u32 = 1010u32;
pub const HSE_REQ_BASE: u32 = 0u32;
pub const HSE_REQ_CANCEL_IO: u32 = 1049u32;
pub const HSE_REQ_CLOSE_CONNECTION: u32 = 1017u32;
pub const HSE_REQ_DONE_WITH_SESSION: u32 = 4u32;
pub const HSE_REQ_END_RESERVED: u32 = 1000u32;
pub const HSE_REQ_EXEC_UNICODE_URL: u32 = 1025u32;
pub const HSE_REQ_EXEC_URL: u32 = 1026u32;
pub const HSE_REQ_GET_ANONYMOUS_TOKEN: u32 = 1038u32;
pub const HSE_REQ_GET_CACHE_INVALIDATION_CALLBACK: u32 = 1040u32;
pub const HSE_REQ_GET_CERT_INFO_EX: u32 = 1015u32;
pub const HSE_REQ_GET_CHANNEL_BINDING_TOKEN: u32 = 1050u32;
pub const HSE_REQ_GET_CONFIG_OBJECT: u32 = 1046u32;
pub const HSE_REQ_GET_EXEC_URL_STATUS: u32 = 1027u32;
pub const HSE_REQ_GET_IMPERSONATION_TOKEN: u32 = 1011u32;
pub const HSE_REQ_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK: u32 = 1048u32;
pub const HSE_REQ_GET_SSPI_INFO: u32 = 1002u32;
pub const HSE_REQ_GET_TRACE_INFO: u32 = 1042u32;
pub const HSE_REQ_GET_TRACE_INFO_EX: u32 = 1044u32;
pub const HSE_REQ_GET_UNICODE_ANONYMOUS_TOKEN: u32 = 1041u32;
pub const HSE_REQ_GET_WORKER_PROCESS_SETTINGS: u32 = 1047u32;
pub const HSE_REQ_IO_COMPLETION: u32 = 1005u32;
pub const HSE_REQ_IS_CONNECTED: u32 = 1018u32;
pub const HSE_REQ_IS_IN_PROCESS: u32 = 1030u32;
pub const HSE_REQ_IS_KEEP_CONN: u32 = 1008u32;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH: u32 = 1023u32;
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH_EX: u32 = 1024u32;
pub const HSE_REQ_MAP_URL_TO_PATH: u32 = 1001u32;
pub const HSE_REQ_MAP_URL_TO_PATH_EX: u32 = 1012u32;
pub const HSE_REQ_NORMALIZE_URL: u32 = 1033u32;
pub const HSE_REQ_RAISE_TRACE_EVENT: u32 = 1045u32;
pub const HSE_REQ_REFRESH_ISAPI_ACL: u32 = 1007u32;
pub const HSE_REQ_REPORT_UNHEALTHY: u32 = 1032u32;
pub const HSE_REQ_SEND_CUSTOM_ERROR: u32 = 1028u32;
pub const HSE_REQ_SEND_RESPONSE_HEADER: u32 = 3u32;
pub const HSE_REQ_SEND_RESPONSE_HEADER_EX: u32 = 1016u32;
pub const HSE_REQ_SEND_URL: u32 = 2u32;
pub const HSE_REQ_SEND_URL_REDIRECT_RESP: u32 = 1u32;
pub const HSE_REQ_SET_FLUSH_FLAG: u32 = 1043u32;
pub const HSE_REQ_TRANSMIT_FILE: u32 = 1006u32;
pub const HSE_REQ_VECTOR_SEND: u32 = 1037u32;
pub const HSE_STATUS_ERROR: u32 = 4u32;
pub const HSE_STATUS_PENDING: u32 = 3u32;
pub const HSE_STATUS_SUCCESS: u32 = 1u32;
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: u32 = 2u32;
pub const HSE_TERM_ADVISORY_UNLOAD: u32 = 1u32;
pub const HSE_TERM_MUST_UNLOAD: u32 = 2u32;
pub const HSE_URL_FLAGS_DONT_CACHE: u32 = 16u32;
pub const HSE_URL_FLAGS_EXECUTE: u32 = 4u32;
pub const HSE_URL_FLAGS_MAP_CERT: u32 = 128u32;
pub const HSE_URL_FLAGS_MASK: u32 = 1023u32;
pub const HSE_URL_FLAGS_NEGO_CERT: u32 = 32u32;
pub const HSE_URL_FLAGS_READ: u32 = 1u32;
pub const HSE_URL_FLAGS_REQUIRE_CERT: u32 = 64u32;
pub const HSE_URL_FLAGS_SCRIPT: u32 = 512u32;
pub const HSE_URL_FLAGS_SSL: u32 = 8u32;
pub const HSE_URL_FLAGS_SSL128: u32 = 256u32;
pub const HSE_URL_FLAGS_WRITE: u32 = 2u32;
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: u32 = 1u32;
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: u32 = 0u32;
pub const HSE_VERSION_MAJOR: u32 = 8u32;
pub const HSE_VERSION_MINOR: u32 = 0u32;
pub const HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS: u32 = 1u32;
pub const HTTP_TRACE_LEVEL_END: u32 = 7u32;
pub const HTTP_TRACE_LEVEL_START: u32 = 6u32;
pub const HTTP_TRACE_TYPE_BOOL: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(11i32);
pub const HTTP_TRACE_TYPE_BYTE: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(17i32);
pub const HTTP_TRACE_TYPE_CHAR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(16i32);
pub const HTTP_TRACE_TYPE_LONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(3i32);
pub const HTTP_TRACE_TYPE_LONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(20i32);
pub const HTTP_TRACE_TYPE_LPCGUID: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(72i32);
pub const HTTP_TRACE_TYPE_LPCSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(30i32);
pub const HTTP_TRACE_TYPE_LPCWSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(31i32);
pub const HTTP_TRACE_TYPE_SHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(2i32);
pub const HTTP_TRACE_TYPE_ULONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(19i32);
pub const HTTP_TRACE_TYPE_ULONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(21i32);
pub const HTTP_TRACE_TYPE_USHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(18i32);
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEY: windows_core::PCWSTR = windows_core::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYA: windows_core::PCSTR = windows_core::s!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYW: windows_core::PCWSTR = windows_core::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
pub const IISADMIN_EXTENSIONS_REG_KEY: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IISADMIN_EXTENSIONS_REG_KEYA: windows_core::PCSTR = windows_core::s!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IISADMIN_EXTENSIONS_REG_KEYW: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
pub const IIS_CLASS_CERTMAPPER: windows_core::PCSTR = windows_core::s!("IIsCertMapper");
pub const IIS_CLASS_CERTMAPPER_W: windows_core::PCWSTR = windows_core::w!("IIsCertMapper");
pub const IIS_CLASS_COMPRESS_SCHEME: windows_core::PCSTR = windows_core::s!("IIsCompressionScheme");
pub const IIS_CLASS_COMPRESS_SCHEMES: windows_core::PCSTR = windows_core::s!("IIsCompressionSchemes");
pub const IIS_CLASS_COMPRESS_SCHEMES_W: windows_core::PCWSTR = windows_core::w!("IIsCompressionSchemes");
pub const IIS_CLASS_COMPRESS_SCHEME_W: windows_core::PCWSTR = windows_core::w!("IIsCompressionScheme");
pub const IIS_CLASS_COMPUTER: windows_core::PCSTR = windows_core::s!("IIsComputer");
pub const IIS_CLASS_COMPUTER_W: windows_core::PCWSTR = windows_core::w!("IIsComputer");
pub const IIS_CLASS_FILTER: windows_core::PCSTR = windows_core::s!("IIsFilter");
pub const IIS_CLASS_FILTERS: windows_core::PCSTR = windows_core::s!("IIsFilters");
pub const IIS_CLASS_FILTERS_W: windows_core::PCWSTR = windows_core::w!("IIsFilters");
pub const IIS_CLASS_FILTER_W: windows_core::PCWSTR = windows_core::w!("IIsFilter");
pub const IIS_CLASS_FTP_INFO: windows_core::PCSTR = windows_core::s!("IIsFtpInfo");
pub const IIS_CLASS_FTP_INFO_W: windows_core::PCWSTR = windows_core::w!("IIsFtpInfo");
pub const IIS_CLASS_FTP_SERVER: windows_core::PCSTR = windows_core::s!("IIsFtpServer");
pub const IIS_CLASS_FTP_SERVER_W: windows_core::PCWSTR = windows_core::w!("IIsFtpServer");
pub const IIS_CLASS_FTP_SERVICE: windows_core::PCSTR = windows_core::s!("IIsFtpService");
pub const IIS_CLASS_FTP_SERVICE_W: windows_core::PCWSTR = windows_core::w!("IIsFtpService");
pub const IIS_CLASS_FTP_VDIR: windows_core::PCSTR = windows_core::s!("IIsFtpVirtualDir");
pub const IIS_CLASS_FTP_VDIR_W: windows_core::PCWSTR = windows_core::w!("IIsFtpVirtualDir");
pub const IIS_CLASS_LOG_MODULE: windows_core::PCSTR = windows_core::s!("IIsLogModule");
pub const IIS_CLASS_LOG_MODULES: windows_core::PCSTR = windows_core::s!("IIsLogModules");
pub const IIS_CLASS_LOG_MODULES_W: windows_core::PCWSTR = windows_core::w!("IIsLogModules");
pub const IIS_CLASS_LOG_MODULE_W: windows_core::PCWSTR = windows_core::w!("IIsLogModule");
pub const IIS_CLASS_MIMEMAP: windows_core::PCSTR = windows_core::s!("IIsMimeMap");
pub const IIS_CLASS_MIMEMAP_W: windows_core::PCWSTR = windows_core::w!("IIsMimeMap");
pub const IIS_CLASS_WEB_DIR: windows_core::PCSTR = windows_core::s!("IIsWebDirectory");
pub const IIS_CLASS_WEB_DIR_W: windows_core::PCWSTR = windows_core::w!("IIsWebDirectory");
pub const IIS_CLASS_WEB_FILE: windows_core::PCSTR = windows_core::s!("IIsWebFile");
pub const IIS_CLASS_WEB_FILE_W: windows_core::PCWSTR = windows_core::w!("IIsWebFile");
pub const IIS_CLASS_WEB_INFO: windows_core::PCSTR = windows_core::s!("IIsWebInfo");
pub const IIS_CLASS_WEB_INFO_W: windows_core::PCWSTR = windows_core::w!("IIsWebInfo");
pub const IIS_CLASS_WEB_SERVER: windows_core::PCSTR = windows_core::s!("IIsWebServer");
pub const IIS_CLASS_WEB_SERVER_W: windows_core::PCWSTR = windows_core::w!("IIsWebServer");
pub const IIS_CLASS_WEB_SERVICE: windows_core::PCSTR = windows_core::s!("IIsWebService");
pub const IIS_CLASS_WEB_SERVICE_W: windows_core::PCWSTR = windows_core::w!("IIsWebService");
pub const IIS_CLASS_WEB_VDIR: windows_core::PCSTR = windows_core::s!("IIsWebVirtualDir");
pub const IIS_CLASS_WEB_VDIR_W: windows_core::PCWSTR = windows_core::w!("IIsWebVirtualDir");
pub const IIS_MD_ADSI_METAID_BEGIN: u32 = 130000u32;
pub const IIS_MD_ADSI_SCHEMA_PATH_A: windows_core::PCSTR = windows_core::s!("/Schema");
pub const IIS_MD_ADSI_SCHEMA_PATH_W: windows_core::PCWSTR = windows_core::w!("/Schema");
pub const IIS_MD_APPPOOL_BASE: u32 = 9000u32;
pub const IIS_MD_APP_BASE: u32 = 9100u32;
pub const IIS_MD_FILE_PROP_BASE: u32 = 6000u32;
pub const IIS_MD_FTP_BASE: u32 = 5000u32;
pub const IIS_MD_GLOBAL_BASE: u32 = 9200u32;
pub const IIS_MD_HTTP_BASE: u32 = 2000u32;
pub const IIS_MD_ID_BEGIN_RESERVED: u32 = 1u32;
pub const IIS_MD_ID_END_RESERVED: u32 = 32767u32;
pub const IIS_MD_INSTANCE_ROOT: windows_core::PCSTR = windows_core::s!("Root");
pub const IIS_MD_ISAPI_FILTERS: windows_core::PCSTR = windows_core::s!("/Filters");
pub const IIS_MD_LOCAL_MACHINE_PATH: windows_core::PCSTR = windows_core::s!("LM");
pub const IIS_MD_LOGCUSTOM_BASE: u32 = 4500u32;
pub const IIS_MD_LOGCUSTOM_LAST: u32 = 4508u32;
pub const IIS_MD_LOG_BASE: u32 = 4000u32;
pub const IIS_MD_LOG_LAST: u32 = 4015u32;
pub const IIS_MD_SERVER_BASE: u32 = 1000u32;
pub const IIS_MD_SSL_BASE: u32 = 5500u32;
pub const IIS_MD_SVC_INFO_PATH: windows_core::PCSTR = windows_core::s!("Info");
pub const IIS_MD_UT_END_RESERVED: u32 = 2000u32;
pub const IIS_MD_UT_FILE: u32 = 2u32;
pub const IIS_MD_UT_SERVER: u32 = 1u32;
pub const IIS_MD_UT_WAM: u32 = 100u32;
pub const IIS_MD_VR_BASE: u32 = 3000u32;
pub const IIS_WEBSOCKET: windows_core::PCWSTR = windows_core::w!("websockets");
pub const IIS_WEBSOCKET_SERVER_VARIABLE: windows_core::PCSTR = windows_core::s!("IIS_WEBSOCK");
pub const IMAP_MD_ID_BEGIN_RESERVED: u32 = 49152u32;
pub const IMAP_MD_ID_END_RESERVED: u32 = 53247u32;
pub const IMGANIM_ANIMATED: u32 = 268435456u32;
pub const IMGANIM_MASK: u32 = 268435456u32;
pub const IMGBITS_MASK: u32 = 234881024u32;
pub const IMGBITS_NONE: u32 = 33554432u32;
pub const IMGBITS_PARTIAL: u32 = 67108864u32;
pub const IMGBITS_TOTAL: u32 = 134217728u32;
pub const IMGCHG_ANIMATE: u32 = 8u32;
pub const IMGCHG_COMPLETE: u32 = 4u32;
pub const IMGCHG_MASK: u32 = 15u32;
pub const IMGCHG_SIZE: u32 = 1u32;
pub const IMGCHG_VIEW: u32 = 2u32;
pub const IMGLOAD_COMPLETE: u32 = 16777216u32;
pub const IMGLOAD_ERROR: u32 = 8388608u32;
pub const IMGLOAD_LOADING: u32 = 2097152u32;
pub const IMGLOAD_MASK: u32 = 32505856u32;
pub const IMGLOAD_NOTLOADED: u32 = 1048576u32;
pub const IMGLOAD_STOPPED: u32 = 4194304u32;
pub const IMGTRANS_MASK: u32 = 536870912u32;
pub const IMGTRANS_OPAQUE: u32 = 536870912u32;
pub const INVALID_END_METADATA: METADATATYPES = METADATATYPES(6i32);
pub const LIBID_ASPTypeLibrary: windows_core::GUID = windows_core::GUID::from_u128(0xd97a6da0_a85c_11cf_83ae_00a0c90c2bd8);
pub const LIBID_IISRSTALib: windows_core::GUID = windows_core::GUID::from_u128(0xe8fb8614_588f_11d2_9d61_00c04f79c5fe);
pub const LIBID_WAMREGLib: windows_core::GUID = windows_core::GUID::from_u128(0x29822aa8_f302_11d0_9953_00c04fd919c1);
pub const MB_DONT_IMPERSONATE: u32 = 9033u32;
pub const MD_ACCESS_EXECUTE: u32 = 4u32;
pub const MD_ACCESS_MAP_CERT: u32 = 128u32;
pub const MD_ACCESS_MASK: u32 = 65535u32;
pub const MD_ACCESS_NEGO_CERT: u32 = 32u32;
pub const MD_ACCESS_NO_PHYSICAL_DIR: u32 = 32768u32;
pub const MD_ACCESS_NO_REMOTE_EXECUTE: u32 = 8192u32;
pub const MD_ACCESS_NO_REMOTE_READ: u32 = 4096u32;
pub const MD_ACCESS_NO_REMOTE_SCRIPT: u32 = 16384u32;
pub const MD_ACCESS_NO_REMOTE_WRITE: u32 = 1024u32;
pub const MD_ACCESS_PERM: u32 = 6016u32;
pub const MD_ACCESS_READ: u32 = 1u32;
pub const MD_ACCESS_REQUIRE_CERT: u32 = 64u32;
pub const MD_ACCESS_SCRIPT: u32 = 512u32;
pub const MD_ACCESS_SOURCE: u32 = 16u32;
pub const MD_ACCESS_SSL: u32 = 8u32;
pub const MD_ACCESS_SSL128: u32 = 256u32;
pub const MD_ACCESS_WRITE: u32 = 2u32;
pub const MD_ACR_ENUM_KEYS: u32 = 8u32;
pub const MD_ACR_READ: u32 = 1u32;
pub const MD_ACR_RESTRICTED_WRITE: u32 = 32u32;
pub const MD_ACR_UNSECURE_PROPS_READ: u32 = 128u32;
pub const MD_ACR_WRITE: u32 = 2u32;
pub const MD_ACR_WRITE_DAC: u32 = 262144u32;
pub const MD_ADMIN_ACL: u32 = 6027u32;
pub const MD_ADMIN_INSTANCE: u32 = 2115u32;
pub const MD_ADV_CACHE_TTL: u32 = 2064u32;
pub const MD_ADV_NOTIFY_PWD_EXP_IN_DAYS: u32 = 2063u32;
pub const MD_AD_CONNECTIONS_PASSWORD: u32 = 5015u32;
pub const MD_AD_CONNECTIONS_USERNAME: u32 = 5014u32;
pub const MD_ALLOW_ANONYMOUS: u32 = 5005u32;
pub const MD_ALLOW_KEEPALIVES: u32 = 6038u32;
pub const MD_ALLOW_PATH_INFO_FOR_SCRIPT_MAPPINGS: u32 = 2095u32;
pub const MD_ALLOW_REPLACE_ON_RENAME: u32 = 5009u32;
pub const MD_ANONYMOUS_ONLY: u32 = 5006u32;
pub const MD_ANONYMOUS_PWD: u32 = 6021u32;
pub const MD_ANONYMOUS_USER_NAME: u32 = 6020u32;
pub const MD_ANONYMOUS_USE_SUBAUTH: u32 = 6022u32;
pub const MD_APPPOOL_32_BIT_APP_ON_WIN64: u32 = 9040u32;
pub const MD_APPPOOL_ALLOW_TRANSIENT_REGISTRATION: u32 = 9202u32;
pub const MD_APPPOOL_APPPOOL_ID: u32 = 9201u32;
pub const MD_APPPOOL_AUTO_SHUTDOWN_EXE: u32 = 9035u32;
pub const MD_APPPOOL_AUTO_SHUTDOWN_PARAMS: u32 = 9036u32;
pub const MD_APPPOOL_AUTO_START: u32 = 9028u32;
pub const MD_APPPOOL_COMMAND: u32 = 9026u32;
pub const MD_APPPOOL_COMMAND_START: u32 = 1u32;
pub const MD_APPPOOL_COMMAND_STOP: u32 = 2u32;
pub const MD_APPPOOL_DISALLOW_OVERLAPPING_ROTATION: u32 = 9015u32;
pub const MD_APPPOOL_DISALLOW_ROTATION_ON_CONFIG_CHANGE: u32 = 9018u32;
pub const MD_APPPOOL_EMULATION_ON_WINARM64: u32 = 9043u32;
pub const MD_APPPOOL_IDENTITY_TYPE: u32 = 9021u32;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSERVICE: u32 = 1u32;
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSYSTEM: u32 = 0u32;
pub const MD_APPPOOL_IDENTITY_TYPE_NETWORKSERVICE: u32 = 2u32;
pub const MD_APPPOOL_IDENTITY_TYPE_SPECIFICUSER: u32 = 3u32;
pub const MD_APPPOOL_IDLE_TIMEOUT: u32 = 9005u32;
pub const MD_APPPOOL_MANAGED_PIPELINE_MODE: u32 = 9041u32;
pub const MD_APPPOOL_MANAGED_RUNTIME_VERSION: u32 = 9039u32;
pub const MD_APPPOOL_MAX_PROCESS_COUNT: u32 = 9003u32;
pub const MD_APPPOOL_ORPHAN_ACTION_EXE: u32 = 9031u32;
pub const MD_APPPOOL_ORPHAN_ACTION_PARAMS: u32 = 9032u32;
pub const MD_APPPOOL_ORPHAN_PROCESSES_FOR_DEBUGGING: u32 = 9009u32;
pub const MD_APPPOOL_PERIODIC_RESTART_CONNECTIONS: u32 = 9104u32;
pub const MD_APPPOOL_PERIODIC_RESTART_MEMORY: u32 = 9024u32;
pub const MD_APPPOOL_PERIODIC_RESTART_PRIVATE_MEMORY: u32 = 9038u32;
pub const MD_APPPOOL_PERIODIC_RESTART_REQUEST_COUNT: u32 = 9002u32;
pub const MD_APPPOOL_PERIODIC_RESTART_SCHEDULE: u32 = 9020u32;
pub const MD_APPPOOL_PERIODIC_RESTART_TIME: u32 = 9001u32;
pub const MD_APPPOOL_PINGING_ENABLED: u32 = 9004u32;
pub const MD_APPPOOL_PING_INTERVAL: u32 = 9013u32;
pub const MD_APPPOOL_PING_RESPONSE_TIMELIMIT: u32 = 9014u32;
pub const MD_APPPOOL_RAPID_FAIL_PROTECTION_ENABLED: u32 = 9006u32;
pub const MD_APPPOOL_SHUTDOWN_TIMELIMIT: u32 = 9012u32;
pub const MD_APPPOOL_SMP_AFFINITIZED: u32 = 9007u32;
pub const MD_APPPOOL_SMP_AFFINITIZED_PROCESSOR_MASK: u32 = 9008u32;
pub const MD_APPPOOL_STARTUP_TIMELIMIT: u32 = 9011u32;
pub const MD_APPPOOL_STATE: u32 = 9027u32;
pub const MD_APPPOOL_STATE_STARTED: u32 = 2u32;
pub const MD_APPPOOL_STATE_STARTING: u32 = 1u32;
pub const MD_APPPOOL_STATE_STOPPED: u32 = 4u32;
pub const MD_APPPOOL_STATE_STOPPING: u32 = 3u32;
pub const MD_APPPOOL_UL_APPPOOL_QUEUE_LENGTH: u32 = 9017u32;
pub const MD_APP_ALLOW_TRANSIENT_REGISTRATION: u32 = 9102u32;
pub const MD_APP_APPPOOL_ID: u32 = 9101u32;
pub const MD_APP_AUTO_START: u32 = 9103u32;
pub const MD_APP_DEPENDENCIES: u32 = 2167u32;
pub const MD_APP_FRIENDLY_NAME: u32 = 2102u32;
pub const MD_APP_ISOLATED: u32 = 2104u32;
pub const MD_APP_OOP_RECOVER_LIMIT: u32 = 2110u32;
pub const MD_APP_PACKAGE_ID: u32 = 2106u32;
pub const MD_APP_PACKAGE_NAME: u32 = 2107u32;
pub const MD_APP_PERIODIC_RESTART_REQUESTS: u32 = 2112u32;
pub const MD_APP_PERIODIC_RESTART_SCHEDULE: u32 = 2113u32;
pub const MD_APP_PERIODIC_RESTART_TIME: u32 = 2111u32;
pub const MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL: u32 = 9042u32;
pub const MD_APP_POOL_LOG_EVENT_ON_RECYCLE: u32 = 9037u32;
pub const MD_APP_POOL_PROCESSMODEL_IDLE_TIMEOUT: u32 = 1u32;
pub const MD_APP_POOL_RECYCLE_CONFIG_CHANGE: u32 = 64u32;
pub const MD_APP_POOL_RECYCLE_ISAPI_UNHEALTHY: u32 = 16u32;
pub const MD_APP_POOL_RECYCLE_MEMORY: u32 = 8u32;
pub const MD_APP_POOL_RECYCLE_ON_DEMAND: u32 = 32u32;
pub const MD_APP_POOL_RECYCLE_PRIVATE_MEMORY: u32 = 128u32;
pub const MD_APP_POOL_RECYCLE_REQUESTS: u32 = 2u32;
pub const MD_APP_POOL_RECYCLE_SCHEDULE: u32 = 4u32;
pub const MD_APP_POOL_RECYCLE_TIME: u32 = 1u32;
pub const MD_APP_ROOT: u32 = 2103u32;
pub const MD_APP_SHUTDOWN_TIME_LIMIT: u32 = 2114u32;
pub const MD_APP_TRACE_URL_LIST: u32 = 2118u32;
pub const MD_APP_WAM_CLSID: u32 = 2105u32;
pub const MD_ASP_ALLOWOUTOFPROCCMPNTS: u32 = 7014u32;
pub const MD_ASP_ALLOWOUTOFPROCCOMPONENTS: u32 = 7014u32;
pub const MD_ASP_ALLOWSESSIONSTATE: u32 = 7011u32;
pub const MD_ASP_BUFFERINGON: u32 = 7000u32;
pub const MD_ASP_BUFFER_LIMIT: u32 = 7052u32;
pub const MD_ASP_CALCLINENUMBER: u32 = 7050u32;
pub const MD_ASP_CODEPAGE: u32 = 7016u32;
pub const MD_ASP_DISKTEMPLATECACHEDIRECTORY: u32 = 7036u32;
pub const MD_ASP_ENABLEAPPLICATIONRESTART: u32 = 7027u32;
pub const MD_ASP_ENABLEASPHTMLFALLBACK: u32 = 7021u32;
pub const MD_ASP_ENABLECHUNKEDENCODING: u32 = 7022u32;
pub const MD_ASP_ENABLECLIENTDEBUG: u32 = 7019u32;
pub const MD_ASP_ENABLEPARENTPATHS: u32 = 7008u32;
pub const MD_ASP_ENABLESERVERDEBUG: u32 = 7018u32;
pub const MD_ASP_ENABLETYPELIBCACHE: u32 = 7023u32;
pub const MD_ASP_ERRORSTONTLOG: u32 = 7024u32;
pub const MD_ASP_EXCEPTIONCATCHENABLE: u32 = 7015u32;
pub const MD_ASP_EXECUTEINMTA: u32 = 7041u32;
pub const MD_ASP_ID_LAST: u32 = 7053u32;
pub const MD_ASP_KEEPSESSIONIDSECURE: u32 = 7043u32;
pub const MD_ASP_LCID: u32 = 7042u32;
pub const MD_ASP_LOGERRORREQUESTS: u32 = 7001u32;
pub const MD_ASP_MAXDISKTEMPLATECACHEFILES: u32 = 7040u32;
pub const MD_ASP_MAXREQUESTENTITY: u32 = 7053u32;
pub const MD_ASP_MAX_REQUEST_ENTITY_ALLOWED: u32 = 7053u32;
pub const MD_ASP_MEMFREEFACTOR: u32 = 7009u32;
pub const MD_ASP_MINUSEDBLOCKS: u32 = 7010u32;
pub const MD_ASP_PROCESSORTHREADMAX: u32 = 7025u32;
pub const MD_ASP_QUEUECONNECTIONTESTTIME: u32 = 7028u32;
pub const MD_ASP_QUEUETIMEOUT: u32 = 7013u32;
pub const MD_ASP_REQEUSTQUEUEMAX: u32 = 7026u32;
pub const MD_ASP_RUN_ONEND_ANON: u32 = 7051u32;
pub const MD_ASP_SCRIPTENGINECACHEMAX: u32 = 7005u32;
pub const MD_ASP_SCRIPTERRORMESSAGE: u32 = 7003u32;
pub const MD_ASP_SCRIPTERRORSSENTTOBROWSER: u32 = 7002u32;
pub const MD_ASP_SCRIPTFILECACHESIZE: u32 = 7004u32;
pub const MD_ASP_SCRIPTLANGUAGE: u32 = 7012u32;
pub const MD_ASP_SCRIPTLANGUAGELIST: u32 = 7017u32;
pub const MD_ASP_SCRIPTTIMEOUT: u32 = 7006u32;
pub const MD_ASP_SERVICE_ENABLE_SXS: u32 = 2u32;
pub const MD_ASP_SERVICE_ENABLE_TRACKER: u32 = 1u32;
pub const MD_ASP_SERVICE_FLAGS: u32 = 7044u32;
pub const MD_ASP_SERVICE_FLAG_FUSION: u32 = 7046u32;
pub const MD_ASP_SERVICE_FLAG_PARTITIONS: u32 = 7047u32;
pub const MD_ASP_SERVICE_FLAG_TRACKER: u32 = 7045u32;
pub const MD_ASP_SERVICE_PARTITION_ID: u32 = 7048u32;
pub const MD_ASP_SERVICE_SXS_NAME: u32 = 7049u32;
pub const MD_ASP_SERVICE_USE_PARTITION: u32 = 4u32;
pub const MD_ASP_SESSIONMAX: u32 = 7029u32;
pub const MD_ASP_SESSIONTIMEOUT: u32 = 7007u32;
pub const MD_ASP_THREADGATEENABLED: u32 = 7030u32;
pub const MD_ASP_THREADGATELOADHIGH: u32 = 7035u32;
pub const MD_ASP_THREADGATELOADLOW: u32 = 7034u32;
pub const MD_ASP_THREADGATESLEEPDELAY: u32 = 7032u32;
pub const MD_ASP_THREADGATESLEEPMAX: u32 = 7033u32;
pub const MD_ASP_THREADGATETIMESLICE: u32 = 7031u32;
pub const MD_ASP_TRACKTHREADINGMODEL: u32 = 7020u32;
pub const MD_AUTHORIZATION: u32 = 6000u32;
pub const MD_AUTHORIZATION_PERSISTENCE: u32 = 6031u32;
pub const MD_AUTH_ADVNOTIFY_DISABLE: u32 = 4u32;
pub const MD_AUTH_ANONYMOUS: u32 = 1u32;
pub const MD_AUTH_BASIC: u32 = 2u32;
pub const MD_AUTH_CHANGE_DISABLE: u32 = 2u32;
pub const MD_AUTH_CHANGE_FLAGS: u32 = 2068u32;
pub const MD_AUTH_CHANGE_UNSECURE: u32 = 1u32;
pub const MD_AUTH_CHANGE_URL: u32 = 2060u32;
pub const MD_AUTH_EXPIRED_UNSECUREURL: u32 = 2067u32;
pub const MD_AUTH_EXPIRED_URL: u32 = 2061u32;
pub const MD_AUTH_MD5: u32 = 16u32;
pub const MD_AUTH_NT: u32 = 4u32;
pub const MD_AUTH_PASSPORT: u32 = 64u32;
pub const MD_AUTH_SINGLEREQUEST: u32 = 64u32;
pub const MD_AUTH_SINGLEREQUESTALWAYSIFPROXY: u32 = 256u32;
pub const MD_AUTH_SINGLEREQUESTIFPROXY: u32 = 128u32;
pub const MD_BACKUP_FORCE_BACKUP: u32 = 4u32;
pub const MD_BACKUP_HIGHEST_VERSION: u32 = 4294967294u32;
pub const MD_BACKUP_MAX_LEN: u32 = 100u32;
pub const MD_BACKUP_MAX_VERSION: u32 = 9999u32;
pub const MD_BACKUP_NEXT_VERSION: u32 = 4294967295u32;
pub const MD_BACKUP_OVERWRITE: u32 = 1u32;
pub const MD_BACKUP_SAVE_FIRST: u32 = 2u32;
pub const MD_BANNER_MESSAGE: u32 = 5011u32;
pub const MD_BINDINGS: u32 = 2022u32;
pub const MD_CACHE_EXTENSIONS: u32 = 6034u32;
pub const MD_CAL_AUTH_RESERVE_TIMEOUT: u32 = 2131u32;
pub const MD_CAL_SSL_RESERVE_TIMEOUT: u32 = 2132u32;
pub const MD_CAL_VC_PER_CONNECT: u32 = 2130u32;
pub const MD_CAL_W3_ERROR: u32 = 2133u32;
pub const MD_CC_MAX_AGE: u32 = 6042u32;
pub const MD_CC_NO_CACHE: u32 = 6041u32;
pub const MD_CC_OTHER: u32 = 6043u32;
pub const MD_CENTRAL_W3C_LOGGING_ENABLED: u32 = 2119u32;
pub const MD_CERT_CACHE_RETRIEVAL_ONLY: u32 = 2u32;
pub const MD_CERT_CHECK_REVOCATION_FRESHNESS_TIME: u32 = 4u32;
pub const MD_CERT_NO_REVOC_CHECK: u32 = 1u32;
pub const MD_CERT_NO_USAGE_CHECK: u32 = 65536u32;
pub const MD_CGI_RESTRICTION_LIST: u32 = 2164u32;
pub const MD_CHANGE_TYPE_ADD_OBJECT: u32 = 2u32;
pub const MD_CHANGE_TYPE_DELETE_DATA: u32 = 8u32;
pub const MD_CHANGE_TYPE_DELETE_OBJECT: u32 = 1u32;
pub const MD_CHANGE_TYPE_RENAME_OBJECT: u32 = 16u32;
pub const MD_CHANGE_TYPE_RESTORE: u32 = 32u32;
pub const MD_CHANGE_TYPE_SET_DATA: u32 = 4u32;
pub const MD_COMMENTS: u32 = 9990u32;
pub const MD_CONNECTION_TIMEOUT: u32 = 1013u32;
pub const MD_CPU_ACTION: u32 = 9022u32;
pub const MD_CPU_APP_ENABLED: u32 = 2141u32;
pub const MD_CPU_CGI_ENABLED: u32 = 2140u32;
pub const MD_CPU_CGI_LIMIT: u32 = 2148u32;
pub const MD_CPU_DISABLE_ALL_LOGGING: u32 = 0u32;
pub const MD_CPU_ENABLE_ACTIVE_PROCS: u32 = 64u32;
pub const MD_CPU_ENABLE_ALL_PROC_LOGGING: u32 = 1u32;
pub const MD_CPU_ENABLE_APP_LOGGING: u32 = 4u32;
pub const MD_CPU_ENABLE_CGI_LOGGING: u32 = 2u32;
pub const MD_CPU_ENABLE_EVENT: u32 = 1u32;
pub const MD_CPU_ENABLE_KERNEL_TIME: u32 = 8u32;
pub const MD_CPU_ENABLE_LOGGING: u32 = 2147483648u32;
pub const MD_CPU_ENABLE_PAGE_FAULTS: u32 = 16u32;
pub const MD_CPU_ENABLE_PROC_TYPE: u32 = 2u32;
pub const MD_CPU_ENABLE_TERMINATED_PROCS: u32 = 128u32;
pub const MD_CPU_ENABLE_TOTAL_PROCS: u32 = 32u32;
pub const MD_CPU_ENABLE_USER_TIME: u32 = 4u32;
pub const MD_CPU_KILL_W3WP: u32 = 1u32;
pub const MD_CPU_LIMIT: u32 = 9023u32;
pub const MD_CPU_LIMITS_ENABLED: u32 = 2143u32;
pub const MD_CPU_LIMIT_LOGEVENT: u32 = 2149u32;
pub const MD_CPU_LIMIT_PAUSE: u32 = 2152u32;
pub const MD_CPU_LIMIT_PRIORITY: u32 = 2150u32;
pub const MD_CPU_LIMIT_PROCSTOP: u32 = 2151u32;
pub const MD_CPU_LOGGING_INTERVAL: u32 = 2145u32;
pub const MD_CPU_LOGGING_MASK: u32 = 4507u32;
pub const MD_CPU_LOGGING_OPTIONS: u32 = 2146u32;
pub const MD_CPU_NO_ACTION: u32 = 0u32;
pub const MD_CPU_RESET_INTERVAL: u32 = 2144u32;
pub const MD_CPU_THROTTLE: u32 = 3u32;
pub const MD_CPU_TRACE: u32 = 2u32;
pub const MD_CREATE_PROCESS_AS_USER: u32 = 6035u32;
pub const MD_CREATE_PROC_NEW_CONSOLE: u32 = 6036u32;
pub const MD_CUSTOM_DEPLOYMENT_DATA: u32 = 6055u32;
pub const MD_CUSTOM_ERROR: u32 = 6008u32;
pub const MD_CUSTOM_ERROR_DESC: u32 = 2120u32;
pub const MD_DEFAULT_BACKUP_LOCATION: windows_core::PCWSTR = windows_core::w!("MDBackUp");
pub const MD_DEFAULT_LOAD_FILE: u32 = 6006u32;
pub const MD_DEFAULT_LOGON_DOMAIN: u32 = 6012u32;
pub const MD_DEMAND_START_THRESHOLD: u32 = 9207u32;
pub const MD_DIRBROW_ENABLED: u32 = 2147483648u32;
pub const MD_DIRBROW_LOADDEFAULT: u32 = 1073741824u32;
pub const MD_DIRBROW_LONG_DATE: u32 = 32u32;
pub const MD_DIRBROW_SHOW_DATE: u32 = 2u32;
pub const MD_DIRBROW_SHOW_EXTENSION: u32 = 16u32;
pub const MD_DIRBROW_SHOW_SIZE: u32 = 8u32;
pub const MD_DIRBROW_SHOW_TIME: u32 = 4u32;
pub const MD_DIRECTORY_BROWSING: u32 = 6005u32;
pub const MD_DISABLE_SOCKET_POOLING: u32 = 1029u32;
pub const MD_DONT_LOG: u32 = 6023u32;
pub const MD_DOWNLEVEL_ADMIN_INSTANCE: u32 = 1021u32;
pub const MD_DO_REVERSE_DNS: u32 = 6029u32;
pub const MD_ENABLEDPROTOCOLS: u32 = 2023u32;
pub const MD_ENABLE_URL_AUTHORIZATION: u32 = 6048u32;
pub const MD_ERROR_CANNOT_REMOVE_SECURE_ATTRIBUTE: i32 = -2146646008i32;
pub const MD_ERROR_DATA_NOT_FOUND: i32 = -2146646015i32;
pub const MD_ERROR_IISAO_INVALID_SCHEMA: i32 = -2146646000i32;
pub const MD_ERROR_INVALID_VERSION: i32 = -2146646014i32;
pub const MD_ERROR_NOT_INITIALIZED: i32 = -2146646016i32;
pub const MD_ERROR_NO_SESSION_KEY: i32 = -2146645987i32;
pub const MD_ERROR_READ_METABASE_FILE: i32 = -2146645991i32;
pub const MD_ERROR_SECURE_CHANNEL_FAILURE: i32 = -2146646010i32;
pub const MD_ERROR_SUB400_INVALID_CONTENT_LENGTH: u32 = 7u32;
pub const MD_ERROR_SUB400_INVALID_DEPTH: u32 = 2u32;
pub const MD_ERROR_SUB400_INVALID_DESTINATION: u32 = 1u32;
pub const MD_ERROR_SUB400_INVALID_IF: u32 = 3u32;
pub const MD_ERROR_SUB400_INVALID_LOCK_TOKEN: u32 = 9u32;
pub const MD_ERROR_SUB400_INVALID_OVERWRITE: u32 = 4u32;
pub const MD_ERROR_SUB400_INVALID_REQUEST_BODY: u32 = 6u32;
pub const MD_ERROR_SUB400_INVALID_TIMEOUT: u32 = 8u32;
pub const MD_ERROR_SUB400_INVALID_TRANSLATE: u32 = 5u32;
pub const MD_ERROR_SUB400_INVALID_WEBSOCKET_REQUEST: u32 = 11u32;
pub const MD_ERROR_SUB400_INVALID_XFF_HEADER: u32 = 10u32;
pub const MD_ERROR_SUB401_APPLICATION: u32 = 5u32;
pub const MD_ERROR_SUB401_FILTER: u32 = 4u32;
pub const MD_ERROR_SUB401_LOGON: u32 = 1u32;
pub const MD_ERROR_SUB401_LOGON_ACL: u32 = 3u32;
pub const MD_ERROR_SUB401_LOGON_CONFIG: u32 = 2u32;
pub const MD_ERROR_SUB401_URLAUTH_POLICY: u32 = 7u32;
pub const MD_ERROR_SUB403_ADDR_REJECT: u32 = 6u32;
pub const MD_ERROR_SUB403_APPPOOL_DENIED: u32 = 18u32;
pub const MD_ERROR_SUB403_CAL_EXCEEDED: u32 = 15u32;
pub const MD_ERROR_SUB403_CERT_BAD: u32 = 16u32;
pub const MD_ERROR_SUB403_CERT_REQUIRED: u32 = 7u32;
pub const MD_ERROR_SUB403_CERT_REVOKED: u32 = 13u32;
pub const MD_ERROR_SUB403_CERT_TIME_INVALID: u32 = 17u32;
pub const MD_ERROR_SUB403_DIR_LIST_DENIED: u32 = 14u32;
pub const MD_ERROR_SUB403_EXECUTE_ACCESS_DENIED: u32 = 1u32;
pub const MD_ERROR_SUB403_INFINITE_DEPTH_DENIED: u32 = 22u32;
pub const MD_ERROR_SUB403_INSUFFICIENT_PRIVILEGE_FOR_CGI: u32 = 19u32;
pub const MD_ERROR_SUB403_INVALID_CNFG: u32 = 10u32;
pub const MD_ERROR_SUB403_LOCK_TOKEN_REQUIRED: u32 = 23u32;
pub const MD_ERROR_SUB403_MAPPER_DENY_ACCESS: u32 = 12u32;
pub const MD_ERROR_SUB403_PASSPORT_LOGIN_FAILURE: u32 = 20u32;
pub const MD_ERROR_SUB403_PWD_CHANGE: u32 = 11u32;
pub const MD_ERROR_SUB403_READ_ACCESS_DENIED: u32 = 2u32;
pub const MD_ERROR_SUB403_SITE_ACCESS_DENIED: u32 = 8u32;
pub const MD_ERROR_SUB403_SOURCE_ACCESS_DENIED: u32 = 21u32;
pub const MD_ERROR_SUB403_SSL128_REQUIRED: u32 = 5u32;
pub const MD_ERROR_SUB403_SSL_REQUIRED: u32 = 4u32;
pub const MD_ERROR_SUB403_TOO_MANY_USERS: u32 = 9u32;
pub const MD_ERROR_SUB403_VALIDATION_FAILURE: u32 = 24u32;
pub const MD_ERROR_SUB403_WRITE_ACCESS_DENIED: u32 = 3u32;
pub const MD_ERROR_SUB404_DENIED_BY_FILTERING_RULE: u32 = 19u32;
pub const MD_ERROR_SUB404_DENIED_BY_MIMEMAP: u32 = 3u32;
pub const MD_ERROR_SUB404_DENIED_BY_POLICY: u32 = 2u32;
pub const MD_ERROR_SUB404_FILE_ATTRIBUTE_HIDDEN: u32 = 9u32;
pub const MD_ERROR_SUB404_FILE_EXTENSION_DENIED: u32 = 7u32;
pub const MD_ERROR_SUB404_HIDDEN_SEGMENT: u32 = 8u32;
pub const MD_ERROR_SUB404_NO_HANDLER: u32 = 4u32;
pub const MD_ERROR_SUB404_PRECONDITIONED_HANDLER: u32 = 17u32;
pub const MD_ERROR_SUB404_QUERY_STRING_SEQUENCE_DENIED: u32 = 18u32;
pub const MD_ERROR_SUB404_QUERY_STRING_TOO_LONG: u32 = 15u32;
pub const MD_ERROR_SUB404_SITE_NOT_FOUND: u32 = 1u32;
pub const MD_ERROR_SUB404_STATICFILE_DAV: u32 = 16u32;
pub const MD_ERROR_SUB404_TOO_MANY_URL_SEGMENTS: u32 = 20u32;
pub const MD_ERROR_SUB404_URL_DOUBLE_ESCAPED: u32 = 11u32;
pub const MD_ERROR_SUB404_URL_HAS_HIGH_BIT_CHARS: u32 = 12u32;
pub const MD_ERROR_SUB404_URL_SEQUENCE_DENIED: u32 = 5u32;
pub const MD_ERROR_SUB404_URL_TOO_LONG: u32 = 14u32;
pub const MD_ERROR_SUB404_VERB_DENIED: u32 = 6u32;
pub const MD_ERROR_SUB413_CONTENT_LENGTH_TOO_LARGE: u32 = 1u32;
pub const MD_ERROR_SUB423_LOCK_TOKEN_SUBMITTED: u32 = 1u32;
pub const MD_ERROR_SUB423_NO_CONFLICTING_LOCK: u32 = 2u32;
pub const MD_ERROR_SUB500_ASPNET_HANDLERS: u32 = 23u32;
pub const MD_ERROR_SUB500_ASPNET_IMPERSONATION: u32 = 24u32;
pub const MD_ERROR_SUB500_ASPNET_MODULES: u32 = 22u32;
pub const MD_ERROR_SUB500_BAD_METADATA: u32 = 19u32;
pub const MD_ERROR_SUB500_HANDLERS_MODULE: u32 = 21u32;
pub const MD_ERROR_SUB500_UNC_ACCESS: u32 = 16u32;
pub const MD_ERROR_SUB500_URLAUTH_NO_SCOPE: u32 = 20u32;
pub const MD_ERROR_SUB500_URLAUTH_NO_STORE: u32 = 17u32;
pub const MD_ERROR_SUB500_URLAUTH_STORE_ERROR: u32 = 18u32;
pub const MD_ERROR_SUB502_ARR_CONNECTION_ERROR: u32 = 3u32;
pub const MD_ERROR_SUB502_ARR_NO_SERVER: u32 = 4u32;
pub const MD_ERROR_SUB502_PREMATURE_EXIT: u32 = 2u32;
pub const MD_ERROR_SUB502_TIMEOUT: u32 = 1u32;
pub const MD_ERROR_SUB503_APP_CONCURRENT: u32 = 2u32;
pub const MD_ERROR_SUB503_ASPNET_QUEUE_FULL: u32 = 3u32;
pub const MD_ERROR_SUB503_CONNECTION_LIMIT: u32 = 5u32;
pub const MD_ERROR_SUB503_CPU_LIMIT: u32 = 1u32;
pub const MD_ERROR_SUB503_FASTCGI_QUEUE_FULL: u32 = 4u32;
pub const MD_EXIT_MESSAGE: u32 = 5001u32;
pub const MD_EXPORT_INHERITED: u32 = 1u32;
pub const MD_EXPORT_NODE_ONLY: u32 = 2u32;
pub const MD_EXTLOG_BYTES_RECV: u32 = 8192u32;
pub const MD_EXTLOG_BYTES_SENT: u32 = 4096u32;
pub const MD_EXTLOG_CLIENT_IP: u32 = 4u32;
pub const MD_EXTLOG_COMPUTER_NAME: u32 = 32u32;
pub const MD_EXTLOG_COOKIE: u32 = 131072u32;
pub const MD_EXTLOG_DATE: u32 = 1u32;
pub const MD_EXTLOG_HOST: u32 = 1048576u32;
pub const MD_EXTLOG_HTTP_STATUS: u32 = 1024u32;
pub const MD_EXTLOG_HTTP_SUB_STATUS: u32 = 2097152u32;
pub const MD_EXTLOG_METHOD: u32 = 128u32;
pub const MD_EXTLOG_PROTOCOL_VERSION: u32 = 524288u32;
pub const MD_EXTLOG_REFERER: u32 = 262144u32;
pub const MD_EXTLOG_SERVER_IP: u32 = 64u32;
pub const MD_EXTLOG_SERVER_PORT: u32 = 32768u32;
pub const MD_EXTLOG_SITE_NAME: u32 = 16u32;
pub const MD_EXTLOG_TIME: u32 = 2u32;
pub const MD_EXTLOG_TIME_TAKEN: u32 = 16384u32;
pub const MD_EXTLOG_URI_QUERY: u32 = 512u32;
pub const MD_EXTLOG_URI_STEM: u32 = 256u32;
pub const MD_EXTLOG_USERNAME: u32 = 8u32;
pub const MD_EXTLOG_USER_AGENT: u32 = 65536u32;
pub const MD_EXTLOG_WIN32_STATUS: u32 = 2048u32;
pub const MD_FILTER_DESCRIPTION: u32 = 2045u32;
pub const MD_FILTER_ENABLED: u32 = 2043u32;
pub const MD_FILTER_ENABLE_CACHE: u32 = 2046u32;
pub const MD_FILTER_FLAGS: u32 = 2044u32;
pub const MD_FILTER_IMAGE_PATH: u32 = 2041u32;
pub const MD_FILTER_LOAD_ORDER: u32 = 2040u32;
pub const MD_FILTER_STATE: u32 = 2042u32;
pub const MD_FILTER_STATE_LOADED: u32 = 1u32;
pub const MD_FILTER_STATE_UNLOADED: u32 = 4u32;
pub const MD_FOOTER_DOCUMENT: u32 = 6009u32;
pub const MD_FOOTER_ENABLED: u32 = 6010u32;
pub const MD_FRONTPAGE_WEB: u32 = 2072u32;
pub const MD_FTPS_128_BITS: u32 = 5053u32;
pub const MD_FTPS_ALLOW_CCC: u32 = 5054u32;
pub const MD_FTPS_SECURE_ANONYMOUS: u32 = 5052u32;
pub const MD_FTPS_SECURE_CONTROL_CHANNEL: u32 = 5050u32;
pub const MD_FTPS_SECURE_DATA_CHANNEL: u32 = 5051u32;
pub const MD_FTP_KEEP_PARTIAL_UPLOADS: u32 = 5019u32;
pub const MD_FTP_LOG_IN_UTF_8: u32 = 5013u32;
pub const MD_FTP_PASV_RESPONSE_IP: u32 = 5018u32;
pub const MD_FTP_UTF8_FILE_NAMES: u32 = 5020u32;
pub const MD_GLOBAL_BINARY_LOGGING_ENABLED: u32 = 4016u32;
pub const MD_GLOBAL_BINSCHEMATIMESTAMP: u32 = 9991u32;
pub const MD_GLOBAL_CHANGE_NUMBER: u32 = 9997u32;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MAJOR_VERSION_NUMBER: u32 = 9994u32;
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MINOR_VERSION_NUMBER: u32 = 9993u32;
pub const MD_GLOBAL_LOG_IN_UTF_8: u32 = 9206u32;
pub const MD_GLOBAL_SESSIONKEY: u32 = 9999u32;
pub const MD_GLOBAL_STANDARD_APP_MODE_ENABLED: u32 = 9203u32;
pub const MD_GLOBAL_XMLSCHEMATIMESTAMP: u32 = 9992u32;
pub const MD_GREETING_MESSAGE: u32 = 5002u32;
pub const MD_HC_CACHE_CONTROL_HEADER: u32 = 2211u32;
pub const MD_HC_COMPRESSION_BUFFER_SIZE: u32 = 2223u32;
pub const MD_HC_COMPRESSION_DIRECTORY: u32 = 2210u32;
pub const MD_HC_COMPRESSION_DLL: u32 = 2237u32;
pub const MD_HC_CREATE_FLAGS: u32 = 2243u32;
pub const MD_HC_DO_DISK_SPACE_LIMITING: u32 = 2216u32;
pub const MD_HC_DO_DYNAMIC_COMPRESSION: u32 = 2213u32;
pub const MD_HC_DO_NAMESPACE_DYNAMIC_COMPRESSION: u32 = 2255u32;
pub const MD_HC_DO_NAMESPACE_STATIC_COMPRESSION: u32 = 2256u32;
pub const MD_HC_DO_ON_DEMAND_COMPRESSION: u32 = 2215u32;
pub const MD_HC_DO_STATIC_COMPRESSION: u32 = 2214u32;
pub const MD_HC_DYNAMIC_COMPRESSION_LEVEL: u32 = 2241u32;
pub const MD_HC_EXPIRES_HEADER: u32 = 2212u32;
pub const MD_HC_FILES_DELETED_PER_DISK_FREE: u32 = 2225u32;
pub const MD_HC_FILE_EXTENSIONS: u32 = 2238u32;
pub const MD_HC_IO_BUFFER_SIZE: u32 = 2222u32;
pub const MD_HC_MAX_DISK_SPACE_USAGE: u32 = 2221u32;
pub const MD_HC_MAX_QUEUE_LENGTH: u32 = 2224u32;
pub const MD_HC_MIME_TYPE: u32 = 2239u32;
pub const MD_HC_MIN_FILE_SIZE_FOR_COMP: u32 = 2226u32;
pub const MD_HC_NO_COMPRESSION_FOR_HTTP_10: u32 = 2217u32;
pub const MD_HC_NO_COMPRESSION_FOR_PROXIES: u32 = 2218u32;
pub const MD_HC_NO_COMPRESSION_FOR_RANGE: u32 = 2219u32;
pub const MD_HC_ON_DEMAND_COMP_LEVEL: u32 = 2242u32;
pub const MD_HC_PRIORITY: u32 = 2240u32;
pub const MD_HC_SCRIPT_FILE_EXTENSIONS: u32 = 2244u32;
pub const MD_HC_SEND_CACHE_HEADERS: u32 = 2220u32;
pub const MD_HEADER_WAIT_TIMEOUT: u32 = 9204u32;
pub const MD_HISTORY_LATEST: u32 = 1u32;
pub const MD_HTTPERRORS_EXISTING_RESPONSE: u32 = 6056u32;
pub const MD_HTTP_CUSTOM: u32 = 6004u32;
pub const MD_HTTP_EXPIRES: u32 = 6002u32;
pub const MD_HTTP_FORWARDER_CUSTOM: u32 = 6054u32;
pub const MD_HTTP_PICS: u32 = 6003u32;
pub const MD_HTTP_REDIRECT: u32 = 6011u32;
pub const MD_IISADMIN_EXTENSIONS: u32 = 1028u32;
pub const MD_IMPORT_INHERITED: u32 = 1u32;
pub const MD_IMPORT_MERGE: u32 = 4u32;
pub const MD_IMPORT_NODE_ONLY: u32 = 2u32;
pub const MD_INSERT_PATH_STRING: windows_core::PCWSTR = windows_core::w!("<%INSERT_PATH%>");
pub const MD_INSERT_PATH_STRINGA: windows_core::PCSTR = windows_core::s!("<%INSERT_PATH%>");
pub const MD_IN_PROCESS_ISAPI_APPS: u32 = 2073u32;
pub const MD_IP_SEC: u32 = 6019u32;
pub const MD_ISAPI_RESTRICTION_LIST: u32 = 2163u32;
pub const MD_IS_CONTENT_INDEXED: u32 = 6039u32;
pub const MD_KEY_TYPE: u32 = 1002u32;
pub const MD_LEVELS_TO_SCAN: u32 = 1022u32;
pub const MD_LOAD_BALANCER_CAPABILITIES: u32 = 9034u32;
pub const MD_LOAD_BALANCER_CAPABILITIES_BASIC: u32 = 1u32;
pub const MD_LOAD_BALANCER_CAPABILITIES_SOPHISTICATED: u32 = 2u32;
pub const MD_LOCATION: u32 = 9989u32;
pub const MD_LOGCUSTOM_DATATYPE_DOUBLE: u32 = 5u32;
pub const MD_LOGCUSTOM_DATATYPE_FLOAT: u32 = 4u32;
pub const MD_LOGCUSTOM_DATATYPE_INT: u32 = 0u32;
pub const MD_LOGCUSTOM_DATATYPE_LONG: u32 = 2u32;
pub const MD_LOGCUSTOM_DATATYPE_LPSTR: u32 = 6u32;
pub const MD_LOGCUSTOM_DATATYPE_LPWSTR: u32 = 7u32;
pub const MD_LOGCUSTOM_DATATYPE_UINT: u32 = 1u32;
pub const MD_LOGCUSTOM_DATATYPE_ULONG: u32 = 3u32;
pub const MD_LOGCUSTOM_PROPERTY_DATATYPE: u32 = 4505u32;
pub const MD_LOGCUSTOM_PROPERTY_HEADER: u32 = 4502u32;
pub const MD_LOGCUSTOM_PROPERTY_ID: u32 = 4503u32;
pub const MD_LOGCUSTOM_PROPERTY_MASK: u32 = 4504u32;
pub const MD_LOGCUSTOM_PROPERTY_NAME: u32 = 4501u32;
pub const MD_LOGCUSTOM_PROPERTY_NODE_ID: u32 = 4508u32;
pub const MD_LOGCUSTOM_SERVICES_STRING: u32 = 4506u32;
pub const MD_LOGEXT_FIELD_MASK: u32 = 4013u32;
pub const MD_LOGEXT_FIELD_MASK2: u32 = 4014u32;
pub const MD_LOGFILE_DIRECTORY: u32 = 4001u32;
pub const MD_LOGFILE_LOCALTIME_ROLLOVER: u32 = 4015u32;
pub const MD_LOGFILE_PERIOD: u32 = 4003u32;
pub const MD_LOGFILE_PERIOD_DAILY: u32 = 1u32;
pub const MD_LOGFILE_PERIOD_HOURLY: u32 = 4u32;
pub const MD_LOGFILE_PERIOD_MAXSIZE: u32 = 0u32;
pub const MD_LOGFILE_PERIOD_MONTHLY: u32 = 3u32;
pub const MD_LOGFILE_PERIOD_NONE: u32 = 0u32;
pub const MD_LOGFILE_PERIOD_WEEKLY: u32 = 2u32;
pub const MD_LOGFILE_TRUNCATE_SIZE: u32 = 4004u32;
pub const MD_LOGON_BATCH: u32 = 1u32;
pub const MD_LOGON_INTERACTIVE: u32 = 0u32;
pub const MD_LOGON_METHOD: u32 = 6013u32;
pub const MD_LOGON_NETWORK: u32 = 2u32;
pub const MD_LOGON_NETWORK_CLEARTEXT: u32 = 3u32;
pub const MD_LOGSQL_DATA_SOURCES: u32 = 4007u32;
pub const MD_LOGSQL_PASSWORD: u32 = 4010u32;
pub const MD_LOGSQL_TABLE_NAME: u32 = 4008u32;
pub const MD_LOGSQL_USER_NAME: u32 = 4009u32;
pub const MD_LOG_ANONYMOUS: u32 = 5007u32;
pub const MD_LOG_NONANONYMOUS: u32 = 5008u32;
pub const MD_LOG_PLUGINS_AVAILABLE: u32 = 4012u32;
pub const MD_LOG_PLUGIN_MOD_ID: u32 = 4005u32;
pub const MD_LOG_PLUGIN_ORDER: u32 = 4011u32;
pub const MD_LOG_PLUGIN_UI_ID: u32 = 4006u32;
pub const MD_LOG_TYPE: u32 = 4000u32;
pub const MD_LOG_TYPE_DISABLED: u32 = 0u32;
pub const MD_LOG_TYPE_ENABLED: u32 = 1u32;
pub const MD_LOG_UNUSED1: u32 = 4002u32;
pub const MD_MAX_BANDWIDTH: u32 = 1000u32;
pub const MD_MAX_BANDWIDTH_BLOCKED: u32 = 1003u32;
pub const MD_MAX_CHANGE_ENTRIES: u32 = 100u32;
pub const MD_MAX_CLIENTS_MESSAGE: u32 = 5003u32;
pub const MD_MAX_CONNECTIONS: u32 = 1014u32;
pub const MD_MAX_ENDPOINT_CONNECTIONS: u32 = 1024u32;
pub const MD_MAX_ERROR_FILES: u32 = 9988u32;
pub const MD_MAX_GLOBAL_BANDWIDTH: u32 = 9201u32;
pub const MD_MAX_GLOBAL_CONNECTIONS: u32 = 9202u32;
pub const MD_MAX_REQUEST_ENTITY_ALLOWED: u32 = 6051u32;
pub const MD_MD_SERVER_SS_AUTH_MAPPING: u32 = 2200u32;
pub const MD_METADATA_ID_REGISTRATION: u32 = 1030u32;
pub const MD_MIME_MAP: u32 = 6015u32;
pub const MD_MIN_FILE_BYTES_PER_SEC: u32 = 9205u32;
pub const MD_MSDOS_DIR_OUTPUT: u32 = 5004u32;
pub const MD_NETLOGON_WKS_DNS: u32 = 2u32;
pub const MD_NETLOGON_WKS_IP: u32 = 1u32;
pub const MD_NETLOGON_WKS_NONE: u32 = 0u32;
pub const MD_NET_LOGON_WKS: u32 = 2065u32;
pub const MD_NOTIFEXAUTH_NTLMSSL: u32 = 1u32;
pub const MD_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
pub const MD_NOTIFY_AUTHENTICATION: u32 = 8192u32;
pub const MD_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
pub const MD_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
pub const MD_NOTIFY_END_OF_REQUEST: u32 = 128u32;
pub const MD_NOTIFY_LOG: u32 = 512u32;
pub const MD_NOTIFY_NONSECURE_PORT: u32 = 2u32;
pub const MD_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
pub const MD_NOTIFY_ORDER_HIGH: u32 = 524288u32;
pub const MD_NOTIFY_ORDER_LOW: u32 = 131072u32;
pub const MD_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
pub const MD_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
pub const MD_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
pub const MD_NOTIFY_SECURE_PORT: u32 = 1u32;
pub const MD_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
pub const MD_NOTIFY_SEND_RESPONSE: u32 = 64u32;
pub const MD_NOTIFY_URL_MAP: u32 = 4096u32;
pub const MD_NOT_DELETABLE: u32 = 2116u32;
pub const MD_NTAUTHENTICATION_PROVIDERS: u32 = 6032u32;
pub const MD_PASSIVE_PORT_RANGE: u32 = 5016u32;
pub const MD_PASSPORT_NEED_MAPPING: u32 = 2u32;
pub const MD_PASSPORT_NO_MAPPING: u32 = 0u32;
pub const MD_PASSPORT_REQUIRE_AD_MAPPING: u32 = 6052u32;
pub const MD_PASSPORT_TRY_MAPPING: u32 = 1u32;
pub const MD_POOL_IDC_TIMEOUT: u32 = 6037u32;
pub const MD_PROCESS_NTCR_IF_LOGGED_ON: u32 = 2070u32;
pub const MD_PUT_READ_SIZE: u32 = 6046u32;
pub const MD_RAPID_FAIL_PROTECTION_INTERVAL: u32 = 9029u32;
pub const MD_RAPID_FAIL_PROTECTION_MAX_CRASHES: u32 = 9030u32;
pub const MD_REALM: u32 = 6001u32;
pub const MD_REDIRECT_HEADERS: u32 = 6044u32;
pub const MD_RESTRICTION_LIST_CUSTOM_DESC: u32 = 2165u32;
pub const MD_ROOT_ENABLE_EDIT_WHILE_RUNNING: u32 = 9998u32;
pub const MD_ROOT_ENABLE_HISTORY: u32 = 9996u32;
pub const MD_ROOT_MAX_HISTORY_FILES: u32 = 9995u32;
pub const MD_SCHEMA_METAID: u32 = 1004u32;
pub const MD_SCRIPTMAPFLAG_ALLOWED_ON_READ_DIR: u32 = 1u32;
pub const MD_SCRIPTMAPFLAG_CHECK_PATH_INFO: u32 = 4u32;
pub const MD_SCRIPTMAPFLAG_SCRIPT: u32 = 1u32;
pub const MD_SCRIPT_MAPS: u32 = 6014u32;
pub const MD_SCRIPT_TIMEOUT: u32 = 6033u32;
pub const MD_SECURE_BINDINGS: u32 = 2021u32;
pub const MD_SECURITY_SETUP_REQUIRED: u32 = 2166u32;
pub const MD_SERVER_AUTOSTART: u32 = 1017u32;
pub const MD_SERVER_BINDINGS: u32 = 1023u32;
pub const MD_SERVER_COMMAND: u32 = 1012u32;
pub const MD_SERVER_COMMAND_CONTINUE: u32 = 4u32;
pub const MD_SERVER_COMMAND_PAUSE: u32 = 3u32;
pub const MD_SERVER_COMMAND_START: u32 = 1u32;
pub const MD_SERVER_COMMAND_STOP: u32 = 2u32;
pub const MD_SERVER_COMMENT: u32 = 1015u32;
pub const MD_SERVER_CONFIGURATION_INFO: u32 = 1027u32;
pub const MD_SERVER_CONFIG_ALLOW_ENCRYPT: u32 = 4u32;
pub const MD_SERVER_CONFIG_AUTO_PW_SYNC: u32 = 8u32;
pub const MD_SERVER_CONFIG_SSL_128: u32 = 2u32;
pub const MD_SERVER_CONFIG_SSL_40: u32 = 1u32;
pub const MD_SERVER_LISTEN_BACKLOG: u32 = 1019u32;
pub const MD_SERVER_LISTEN_TIMEOUT: u32 = 1020u32;
pub const MD_SERVER_SIZE: u32 = 1018u32;
pub const MD_SERVER_SIZE_LARGE: u32 = 2u32;
pub const MD_SERVER_SIZE_MEDIUM: u32 = 1u32;
pub const MD_SERVER_SIZE_SMALL: u32 = 0u32;
pub const MD_SERVER_STATE: u32 = 1016u32;
pub const MD_SERVER_STATE_CONTINUING: u32 = 7u32;
pub const MD_SERVER_STATE_PAUSED: u32 = 6u32;
pub const MD_SERVER_STATE_PAUSING: u32 = 5u32;
pub const MD_SERVER_STATE_STARTED: u32 = 2u32;
pub const MD_SERVER_STATE_STARTING: u32 = 1u32;
pub const MD_SERVER_STATE_STOPPED: u32 = 4u32;
pub const MD_SERVER_STATE_STOPPING: u32 = 3u32;
pub const MD_SET_HOST_NAME: u32 = 2154u32;
pub const MD_SHOW_4_DIGIT_YEAR: u32 = 5010u32;
pub const MD_SSI_EXEC_DISABLED: u32 = 6028u32;
pub const MD_SSL_ACCESS_PERM: u32 = 6030u32;
pub const MD_SSL_ALWAYS_NEGO_CLIENT_CERT: u32 = 5521u32;
pub const MD_SSL_KEY_PASSWORD: u32 = 5502u32;
pub const MD_SSL_KEY_REQUEST: u32 = 5503u32;
pub const MD_SSL_PRIVATE_KEY: u32 = 5501u32;
pub const MD_SSL_PUBLIC_KEY: u32 = 5500u32;
pub const MD_SSL_USE_DS_MAPPER: u32 = 5519u32;
pub const MD_STOP_LISTENING: u32 = 9987u32;
pub const MD_SUPPRESS_DEFAULT_BANNER: u32 = 5017u32;
pub const MD_UPLOAD_READAHEAD_SIZE: u32 = 6045u32;
pub const MD_URL_AUTHORIZATION_IMPERSONATION_LEVEL: u32 = 6053u32;
pub const MD_URL_AUTHORIZATION_SCOPE_NAME: u32 = 6050u32;
pub const MD_URL_AUTHORIZATION_STORE_NAME: u32 = 6049u32;
pub const MD_USER_ISOLATION: u32 = 5012u32;
pub const MD_USER_ISOLATION_AD: u32 = 2u32;
pub const MD_USER_ISOLATION_BASIC: u32 = 1u32;
pub const MD_USER_ISOLATION_LAST: u32 = 2u32;
pub const MD_USER_ISOLATION_NONE: u32 = 0u32;
pub const MD_USE_DIGEST_SSP: u32 = 6047u32;
pub const MD_USE_HOST_NAME: u32 = 2066u32;
pub const MD_VR_IGNORE_TRANSLATE: u32 = 3008u32;
pub const MD_VR_NO_CACHE: u32 = 3007u32;
pub const MD_VR_PASSTHROUGH: u32 = 3006u32;
pub const MD_VR_PASSWORD: u32 = 3003u32;
pub const MD_VR_PATH: u32 = 3001u32;
pub const MD_VR_USERNAME: u32 = 3002u32;
pub const MD_WAM_PWD: u32 = 7502u32;
pub const MD_WAM_USER_NAME: u32 = 7501u32;
pub const MD_WARNING_DUP_NAME: i32 = 837636i32;
pub const MD_WARNING_INVALID_DATA: i32 = 837637i32;
pub const MD_WARNING_PATH_NOT_FOUND: i32 = 837635i32;
pub const MD_WARNING_PATH_NOT_INSERTED: i32 = 837639i32;
pub const MD_WARNING_SAVE_FAILED: i32 = 837641i32;
pub const MD_WEBDAV_MAX_ATTRIBUTES_PER_ELEMENT: u32 = 8501u32;
pub const MD_WEB_SVC_EXT_RESTRICTION_LIST: u32 = 2168u32;
pub const MD_WIN32_ERROR: u32 = 1099u32;
pub const METADATA_DONT_EXPAND: u32 = 512u32;
pub const METADATA_INHERIT: u32 = 1u32;
pub const METADATA_INSERT_PATH: u32 = 64u32;
pub const METADATA_ISINHERITED: u32 = 32u32;
pub const METADATA_LOCAL_MACHINE_ONLY: u32 = 128u32;
pub const METADATA_MASTER_ROOT_HANDLE: u32 = 0u32;
pub const METADATA_MAX_NAME_LEN: u32 = 256u32;
pub const METADATA_NON_SECURE_ONLY: u32 = 256u32;
pub const METADATA_NO_ATTRIBUTES: u32 = 0u32;
pub const METADATA_PARTIAL_PATH: u32 = 2u32;
pub const METADATA_PERMISSION_READ: u32 = 1u32;
pub const METADATA_PERMISSION_WRITE: u32 = 2u32;
pub const METADATA_REFERENCE: u32 = 8u32;
pub const METADATA_SECURE: u32 = 4u32;
pub const METADATA_VOLATILE: u32 = 16u32;
pub const MSCS_MD_ID_BEGIN_RESERVED: u32 = 53248u32;
pub const MSCS_MD_ID_END_RESERVED: u32 = 57343u32;
pub const MULTISZ_METADATA: METADATATYPES = METADATATYPES(5i32);
pub const NNTP_MD_ID_BEGIN_RESERVED: u32 = 45056u32;
pub const NNTP_MD_ID_END_RESERVED: u32 = 49151u32;
pub const POP3_MD_ID_BEGIN_RESERVED: u32 = 40960u32;
pub const POP3_MD_ID_END_RESERVED: u32 = 45055u32;
pub const SF_DENIED_APPLICATION: u32 = 8u32;
pub const SF_DENIED_BY_CONFIG: u32 = 65536u32;
pub const SF_DENIED_FILTER: u32 = 4u32;
pub const SF_DENIED_LOGON: u32 = 1u32;
pub const SF_DENIED_RESOURCE: u32 = 2u32;
pub const SF_MAX_AUTH_TYPE: u32 = 33u32;
pub const SF_MAX_FILTER_DESC_LEN: u32 = 257u32;
pub const SF_MAX_PASSWORD: u32 = 257u32;
pub const SF_MAX_USERNAME: u32 = 257u32;
pub const SF_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
pub const SF_NOTIFY_AUTHENTICATION: u32 = 8192u32;
pub const SF_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
pub const SF_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
pub const SF_NOTIFY_END_OF_REQUEST: u32 = 128u32;
pub const SF_NOTIFY_LOG: u32 = 512u32;
pub const SF_NOTIFY_NONSECURE_PORT: u32 = 2u32;
pub const SF_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
pub const SF_NOTIFY_ORDER_HIGH: u32 = 524288u32;
pub const SF_NOTIFY_ORDER_LOW: u32 = 131072u32;
pub const SF_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
pub const SF_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
pub const SF_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
pub const SF_NOTIFY_SECURE_PORT: u32 = 1u32;
pub const SF_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
pub const SF_NOTIFY_SEND_RESPONSE: u32 = 64u32;
pub const SF_NOTIFY_URL_MAP: u32 = 4096u32;
pub const SF_PROPERTY_INSTANCE_NUM_ID: SF_PROPERTY_IIS = SF_PROPERTY_IIS(1i32);
pub const SF_PROPERTY_SSL_CTXT: SF_PROPERTY_IIS = SF_PROPERTY_IIS(0i32);
pub const SF_REQ_ADD_HEADERS_ON_DENIAL: SF_REQ_TYPE = SF_REQ_TYPE(1i32);
pub const SF_REQ_DISABLE_NOTIFICATIONS: SF_REQ_TYPE = SF_REQ_TYPE(8i32);
pub const SF_REQ_GET_CONNID: SF_REQ_TYPE = SF_REQ_TYPE(4i32);
pub const SF_REQ_GET_PROPERTY: SF_REQ_TYPE = SF_REQ_TYPE(6i32);
pub const SF_REQ_NORMALIZE_URL: SF_REQ_TYPE = SF_REQ_TYPE(7i32);
pub const SF_REQ_SEND_RESPONSE_HEADER: SF_REQ_TYPE = SF_REQ_TYPE(0i32);
pub const SF_REQ_SET_CERTIFICATE_INFO: SF_REQ_TYPE = SF_REQ_TYPE(5i32);
pub const SF_REQ_SET_NEXT_READ_SIZE: SF_REQ_TYPE = SF_REQ_TYPE(2i32);
pub const SF_REQ_SET_PROXY_INFO: SF_REQ_TYPE = SF_REQ_TYPE(3i32);
pub const SF_STATUS_REQ_ERROR: SF_STATUS_TYPE = SF_STATUS_TYPE(134217732i32);
pub const SF_STATUS_REQ_FINISHED: SF_STATUS_TYPE = SF_STATUS_TYPE(134217728i32);
pub const SF_STATUS_REQ_FINISHED_KEEP_CONN: SF_STATUS_TYPE = SF_STATUS_TYPE(134217729i32);
pub const SF_STATUS_REQ_HANDLED_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217731i32);
pub const SF_STATUS_REQ_NEXT_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217730i32);
pub const SF_STATUS_REQ_READ_NEXT: SF_STATUS_TYPE = SF_STATUS_TYPE(134217733i32);
pub const SMTP_MD_ID_BEGIN_RESERVED: u32 = 36864u32;
pub const SMTP_MD_ID_END_RESERVED: u32 = 40959u32;
pub const STRING_METADATA: METADATATYPES = METADATATYPES(2i32);
pub const USER_MD_ID_BASE_RESERVED: u32 = 65535u32;
pub const WAM_MD_ID_BEGIN_RESERVED: u32 = 29952u32;
pub const WAM_MD_ID_END_RESERVED: u32 = 32767u32;
pub const WAM_MD_SERVER_BASE: u32 = 7500u32;
pub const WEBDAV_MD_SERVER_BASE: u32 = 8500u32;
pub const WEB_CORE_ACTIVATE_DLL_ENTRY: windows_core::PCSTR = windows_core::s!("WebCoreActivate");
pub const WEB_CORE_DLL_NAME: windows_core::PCWSTR = windows_core::w!("hwebcore.dll");
pub const WEB_CORE_SET_METADATA_DLL_ENTRY: windows_core::PCSTR = windows_core::s!("WebCoreSetMetadata");
pub const WEB_CORE_SHUTDOWN_DLL_ENTRY: windows_core::PCSTR = windows_core::s!("WebCoreShutdown");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FTP_ACCESS(pub i32);
impl windows_core::TypeKind for FTP_ACCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FTP_ACCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FTP_ACCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FTP_PROCESS_STATUS(pub i32);
impl windows_core::TypeKind for FTP_PROCESS_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FTP_PROCESS_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FTP_PROCESS_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_TRACE_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_TRACE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_TRACE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_TRACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct METADATATYPES(pub i32);
impl windows_core::TypeKind for METADATATYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for METADATATYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("METADATATYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SF_PROPERTY_IIS(pub i32);
impl windows_core::TypeKind for SF_PROPERTY_IIS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SF_PROPERTY_IIS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SF_PROPERTY_IIS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SF_REQ_TYPE(pub i32);
impl windows_core::TypeKind for SF_REQ_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SF_REQ_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SF_REQ_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SF_STATUS_TYPE(pub i32);
impl windows_core::TypeKind for SF_STATUS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SF_STATUS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SF_STATUS_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::super::Security::Cryptography::CERT_CONTEXT,
    pub cbAllocated: u32,
    pub dwCertificateFlags: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for CERT_CONTEXT_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for CERT_CONTEXT_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CONFIGURATION_ENTRY {
    pub bstrKey: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrValue: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Clone for CONFIGURATION_ENTRY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CONFIGURATION_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: HCONN,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [i8; 80],
    pub lpszMethod: windows_core::PSTR,
    pub lpszQueryString: windows_core::PSTR,
    pub lpszPathInfo: windows_core::PSTR,
    pub lpszPathTranslated: windows_core::PSTR,
    pub cbTotalBytes: u32,
    pub cbAvailable: u32,
    pub lpbData: *mut u8,
    pub lpszContentType: windows_core::PSTR,
    pub GetServerVariable: PFN_IIS_GETSERVERVARIABLE,
    pub WriteClient: PFN_IIS_WRITECLIENT,
    pub ReadClient: PFN_IIS_READCLIENT,
    pub ServerSupportFunction: PFN_IIS_SERVERSUPPORTFUNCTION,
}
impl windows_core::TypeKind for EXTENSION_CONTROL_BLOCK {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FtpProvider: windows_core::GUID = windows_core::GUID::from_u128(0x70bdc667_33b2_45f0_ac52_c3ca46f7a656);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCONN(pub *mut core::ffi::c_void);
impl HCONN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCONN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: windows_core::PSTR,
    pub uHttpSubError: u16,
    pub fAsync: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for HSE_CUSTOM_ERROR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: windows_core::PWSTR,
    pub pszMethod: windows_core::PSTR,
    pub pszChildHeaders: windows_core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
impl windows_core::TypeKind for HSE_EXEC_UNICODE_URL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_UNICODE_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: windows_core::PWSTR,
    pub pszCustomAuthType: windows_core::PSTR,
}
impl windows_core::TypeKind for HSE_EXEC_UNICODE_URL_USER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_URL_ENTITY_INFO {
    pub cbAvailable: u32,
    pub lpbData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HSE_EXEC_URL_ENTITY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: windows_core::PSTR,
    pub pszMethod: windows_core::PSTR,
    pub pszChildHeaders: windows_core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
impl windows_core::TypeKind for HSE_EXEC_URL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_URL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: u16,
    pub uHttpSubStatus: u16,
    pub dwWin32Error: u32,
}
impl windows_core::TypeKind for HSE_EXEC_URL_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_URL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: windows_core::PSTR,
    pub pszCustomAuthType: windows_core::PSTR,
}
impl windows_core::TypeKind for HSE_EXEC_URL_USER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_EXEC_URL_USER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: u32,
    pub pszStatus: windows_core::PSTR,
    pub pszHeaders: windows_core::PSTR,
    pub nElementCount: u32,
    pub lpElementArray: *mut HSE_VECTOR_ELEMENT,
}
impl windows_core::TypeKind for HSE_RESPONSE_VECTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_RESPONSE_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: windows_core::PCSTR,
    pub pszHeader: windows_core::PCSTR,
    pub cchStatus: u32,
    pub cchHeader: u32,
    pub fKeepConn: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for HSE_SEND_HEADER_EX_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_SEND_HEADER_EX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HSE_TF_INFO {
    pub pfnHseIO: PFN_HSE_IO_COMPLETION,
    pub pContext: *mut core::ffi::c_void,
    pub hFile: super::super::Foundation::HANDLE,
    pub pszStatusCode: windows_core::PCSTR,
    pub BytesToWrite: u32,
    pub Offset: u32,
    pub pHead: *mut core::ffi::c_void,
    pub HeadLength: u32,
    pub pTail: *mut core::ffi::c_void,
    pub TailLength: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for HSE_TF_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: super::super::Foundation::BOOL,
    pub TraceContextId: [u8; 16],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for HSE_TRACE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_UNICODE_URL_MAPEX_INFO {
    pub lpszPath: [u16; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
}
impl windows_core::TypeKind for HSE_UNICODE_URL_MAPEX_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [i8; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for HSE_URL_MAPEX_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_VECTOR_ELEMENT {
    pub ElementType: u32,
    pub pvContext: *mut core::ffi::c_void,
    pub cbOffset: u64,
    pub cbSize: u64,
}
impl windows_core::TypeKind for HSE_VECTOR_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: u32,
    pub lpszExtensionDesc: [i8; 256],
}
impl windows_core::TypeKind for HSE_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: windows_core::PCSTR,
    pub pszPhysicalPath: windows_core::PCSTR,
    pub dwReason: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_ACCESS_DENIED {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: windows_core::PSTR,
    pub cbUserBuff: u32,
    pub pszPassword: windows_core::PSTR,
    pub cbPasswordBuff: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_AUTHENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub GetUserToken: isize,
    pub HttpStatus: u32,
    pub fResetAuth: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_AUTH_COMPLETE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: u32,
    pub Revision: u32,
    pub ServerContext: *mut core::ffi::c_void,
    pub ulReserved: u32,
    pub fIsSecurePort: super::super::Foundation::BOOL,
    pub pFilterContext: *mut core::ffi::c_void,
    pub GetServerVariable: isize,
    pub AddResponseHeaders: isize,
    pub WriteClient: isize,
    pub AllocMem: isize,
    pub ServerSupportFunction: isize,
}
impl windows_core::TypeKind for HTTP_FILTER_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: windows_core::PCSTR,
    pub pszClientUserName: windows_core::PCSTR,
    pub pszServerName: windows_core::PCSTR,
    pub pszOperation: windows_core::PCSTR,
    pub pszTarget: windows_core::PCSTR,
    pub pszParameters: windows_core::PCSTR,
    pub dwHttpStatus: u32,
    pub dwWin32Status: u32,
    pub dwBytesSent: u32,
    pub dwBytesRecvd: u32,
    pub msTimeForProcessing: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_LOG {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub HttpStatus: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_PREPROC_HEADERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_RAW_DATA {
    pub pvInData: *mut core::ffi::c_void,
    pub cbInData: u32,
    pub cbInBuffer: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_RAW_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: windows_core::PCSTR,
    pub pszPhysicalPath: windows_core::PSTR,
    pub cbPathBuff: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_URL_MAP {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: windows_core::PCSTR,
    pub pszPhysicalPath: windows_core::PSTR,
    pub cbPathBuff: u32,
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub pszScriptMapEntry: windows_core::PCSTR,
}
impl windows_core::TypeKind for HTTP_FILTER_URL_MAP_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: u32,
    pub dwFilterVersion: u32,
    pub lpszFilterDesc: [i8; 257],
    pub dwFlags: u32,
}
impl windows_core::TypeKind for HTTP_FILTER_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TRACE_CONFIGURATION {
    pub pProviderGuid: *const windows_core::GUID,
    pub dwAreas: u32,
    pub dwVerbosity: u32,
    pub fProviderEnabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for HTTP_TRACE_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TRACE_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TRACE_EVENT {
    pub pProviderGuid: *const windows_core::GUID,
    pub dwArea: u32,
    pub pAreaGuid: *const windows_core::GUID,
    pub dwEvent: u32,
    pub pszEventName: windows_core::PCWSTR,
    pub dwEventVersion: u32,
    pub dwVerbosity: u32,
    pub pActivityGuid: *const windows_core::GUID,
    pub pRelatedActivityGuid: *const windows_core::GUID,
    pub dwTimeStamp: u32,
    pub dwFlags: u32,
    pub cEventItems: u32,
    pub pEventItems: *mut HTTP_TRACE_EVENT_ITEM,
}
impl windows_core::TypeKind for HTTP_TRACE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TRACE_EVENT_ITEM {
    pub pszName: windows_core::PCWSTR,
    pub dwDataType: HTTP_TRACE_TYPE,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pszDataDescription: windows_core::PCWSTR,
}
impl windows_core::TypeKind for HTTP_TRACE_EVENT_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOGGING_PARAMETERS {
    pub pszSessionId: windows_core::PCWSTR,
    pub pszSiteName: windows_core::PCWSTR,
    pub pszUserName: windows_core::PCWSTR,
    pub pszHostName: windows_core::PCWSTR,
    pub pszRemoteIpAddress: windows_core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: windows_core::PCWSTR,
    pub pszCommandParameters: windows_core::PCWSTR,
    pub pszFullPath: windows_core::PCWSTR,
    pub dwElapsedMilliseconds: u32,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: windows_core::HRESULT,
    pub pszInformation: windows_core::PCWSTR,
}
impl windows_core::TypeKind for LOGGING_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MD_CHANGE_OBJECT_W {
    pub pszMDPath: windows_core::PWSTR,
    pub dwMDChangeType: u32,
    pub dwMDNumDataIDs: u32,
    pub pdwMDDataIDs: *mut u32,
}
impl windows_core::TypeKind for MD_CHANGE_OBJECT_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct METADATA_GETALL_INTERNAL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub Anonymous: METADATA_GETALL_INTERNAL_RECORD_0,
    pub dwMDDataTag: u32,
}
impl windows_core::TypeKind for METADATA_GETALL_INTERNAL_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for METADATA_GETALL_INTERNAL_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union METADATA_GETALL_INTERNAL_RECORD_0 {
    pub dwMDDataOffset: usize,
    pub pbMDData: *mut u8,
}
impl windows_core::TypeKind for METADATA_GETALL_INTERNAL_RECORD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct METADATA_GETALL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub dwMDDataOffset: u32,
    pub dwMDDataTag: u32,
}
impl windows_core::TypeKind for METADATA_GETALL_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for METADATA_GETALL_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct METADATA_HANDLE_INFO {
    pub dwMDPermissions: u32,
    pub dwMDSystemChangeNumber: u32,
}
impl windows_core::TypeKind for METADATA_HANDLE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for METADATA_HANDLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct METADATA_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub pbMDData: *mut u8,
    pub dwMDDataTag: u32,
}
impl windows_core::TypeKind for METADATA_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct POST_PROCESS_PARAMETERS {
    pub pszSessionId: windows_core::PCWSTR,
    pub pszSiteName: windows_core::PCWSTR,
    pub pszUserName: windows_core::PCWSTR,
    pub pszHostName: windows_core::PCWSTR,
    pub pszRemoteIpAddress: windows_core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: windows_core::PCWSTR,
    pub pszCommandParameters: windows_core::PCWSTR,
    pub pszFullPath: windows_core::PCWSTR,
    pub pszPhysicalPath: windows_core::PCWSTR,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: windows_core::HRESULT,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
impl windows_core::TypeKind for POST_PROCESS_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for POST_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRE_PROCESS_PARAMETERS {
    pub pszSessionId: windows_core::PCWSTR,
    pub pszSiteName: windows_core::PCWSTR,
    pub pszUserName: windows_core::PCWSTR,
    pub pszHostName: windows_core::PCWSTR,
    pub pszRemoteIpAddress: windows_core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: windows_core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub pszCommand: windows_core::PCWSTR,
    pub pszCommandParameters: windows_core::PCWSTR,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
impl windows_core::TypeKind for PRE_PROCESS_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRE_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_GETEXTENSIONVERSION = Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL>;
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = Option<unsafe extern "system" fn(pszurl: windows_core::PCWSTR) -> windows_core::HRESULT>;
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = Option<unsafe extern "system" fn(pszprotocolmanagerdll: windows_core::PCWSTR, pszprotocolmanagerdllinitfunction: windows_core::PCWSTR, dwcustominterfaceid: u32, ppcustominterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFN_HSE_IO_COMPLETION = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: *mut core::ffi::c_void, cbio: u32, dwerror: u32)>;
pub type PFN_HTTPEXTENSIONPROC = Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> u32>;
pub type PFN_IIS_GETSERVERVARIABLE = Option<unsafe extern "system" fn(param0: HCONN, param1: windows_core::PCSTR, param2: *mut core::ffi::c_void, param3: *mut u32) -> super::super::Foundation::BOOL>;
pub type PFN_IIS_READCLIENT = Option<unsafe extern "system" fn(param0: HCONN, param1: *mut core::ffi::c_void, param2: *mut u32) -> super::super::Foundation::BOOL>;
pub type PFN_IIS_SERVERSUPPORTFUNCTION = Option<unsafe extern "system" fn(param0: HCONN, param1: u32, param2: *mut core::ffi::c_void, param3: *mut u32, param4: *mut u32) -> super::super::Foundation::BOOL>;
pub type PFN_IIS_WRITECLIENT = Option<unsafe extern "system" fn(param0: HCONN, param1: *mut core::ffi::c_void, param2: *mut u32, param3: u32) -> super::super::Foundation::BOOL>;
pub type PFN_TERMINATEEXTENSION = Option<unsafe extern "system" fn(dwflags: u32) -> super::super::Foundation::BOOL>;
pub type PFN_WEB_CORE_ACTIVATE = Option<unsafe extern "system" fn(pszapphostconfigfile: windows_core::PCWSTR, pszrootwebconfigfile: windows_core::PCWSTR, pszinstancename: windows_core::PCWSTR) -> windows_core::HRESULT>;
pub type PFN_WEB_CORE_SET_METADATA_DLL_ENTRY = Option<unsafe extern "system" fn(pszmetadatatype: windows_core::PCWSTR, pszvalue: windows_core::PCWSTR) -> windows_core::HRESULT>;
pub type PFN_WEB_CORE_SHUTDOWN = Option<unsafe extern "system" fn(fimmediate: u32) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
