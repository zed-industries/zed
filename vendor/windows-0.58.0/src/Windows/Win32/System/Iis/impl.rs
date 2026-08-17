pub trait AsyncIFtpAuthenticationProvider_Impl: Sized {
    fn Begin_AuthenticateUser(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_AuthenticateUser(&self, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIFtpAuthenticationProvider {}
impl AsyncIFtpAuthenticationProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpAuthenticationProvider_Vtbl
    where
        Identity: AsyncIFtpAuthenticationProvider_Impl,
    {
        unsafe extern "system" fn Begin_AuthenticateUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszpassword: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpAuthenticationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpAuthenticationProvider_Impl::Begin_AuthenticateUser(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername), core::mem::transmute(&pszpassword)).into()
        }
        unsafe extern "system" fn Finish_AuthenticateUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpAuthenticationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpAuthenticationProvider_Impl::Finish_AuthenticateUser(this, core::mem::transmute_copy(&ppszcanonicalusername), core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_AuthenticateUser: Begin_AuthenticateUser::<Identity, OFFSET>,
            Finish_AuthenticateUser: Finish_AuthenticateUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpAuthenticationProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpAuthorizationProvider_Impl: Sized {
    fn Begin_GetUserAccessPermission(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszvirtualpath: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_GetUserAccessPermission(&self) -> windows_core::Result<FTP_ACCESS>;
}
impl windows_core::RuntimeName for AsyncIFtpAuthorizationProvider {}
impl AsyncIFtpAuthorizationProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpAuthorizationProvider_Vtbl
    where
        Identity: AsyncIFtpAuthorizationProvider_Impl,
    {
        unsafe extern "system" fn Begin_GetUserAccessPermission<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszvirtualpath: windows_core::PCWSTR, pszusername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpAuthorizationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpAuthorizationProvider_Impl::Begin_GetUserAccessPermission(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszvirtualpath), core::mem::transmute(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserAccessPermission<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpAuthorizationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIFtpAuthorizationProvider_Impl::Finish_GetUserAccessPermission(this) {
                Ok(ok__) => {
                    pftpaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetUserAccessPermission: Begin_GetUserAccessPermission::<Identity, OFFSET>,
            Finish_GetUserAccessPermission: Finish_GetUserAccessPermission::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpAuthorizationProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpHomeDirectoryProvider_Impl: Sized {
    fn Begin_GetUserHomeDirectoryData(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_GetUserHomeDirectoryData(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for AsyncIFtpHomeDirectoryProvider {}
impl AsyncIFtpHomeDirectoryProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpHomeDirectoryProvider_Vtbl
    where
        Identity: AsyncIFtpHomeDirectoryProvider_Impl,
    {
        unsafe extern "system" fn Begin_GetUserHomeDirectoryData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpHomeDirectoryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpHomeDirectoryProvider_Impl::Begin_GetUserHomeDirectoryData(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername)).into()
        }
        unsafe extern "system" fn Finish_GetUserHomeDirectoryData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszhomedirectorydata: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpHomeDirectoryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIFtpHomeDirectoryProvider_Impl::Finish_GetUserHomeDirectoryData(this) {
                Ok(ok__) => {
                    ppszhomedirectorydata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetUserHomeDirectoryData: Begin_GetUserHomeDirectoryData::<Identity, OFFSET>,
            Finish_GetUserHomeDirectoryData: Finish_GetUserHomeDirectoryData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpHomeDirectoryProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpLogProvider_Impl: Sized {
    fn Begin_Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::Result<()>;
    fn Finish_Log(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIFtpLogProvider {}
impl AsyncIFtpLogProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpLogProvider_Vtbl
    where
        Identity: AsyncIFtpLogProvider_Impl,
    {
        unsafe extern "system" fn Begin_Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpLogProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpLogProvider_Impl::Begin_Log(this, core::mem::transmute_copy(&ploggingparameters)).into()
        }
        unsafe extern "system" fn Finish_Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpLogProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpLogProvider_Impl::Finish_Log(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Log: Begin_Log::<Identity, OFFSET>,
            Finish_Log: Finish_Log::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpLogProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpPostprocessProvider_Impl: Sized {
    fn Begin_HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> windows_core::Result<()>;
    fn Finish_HandlePostprocess(&self) -> windows_core::Result<FTP_PROCESS_STATUS>;
}
impl windows_core::RuntimeName for AsyncIFtpPostprocessProvider {}
impl AsyncIFtpPostprocessProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpPostprocessProvider_Vtbl
    where
        Identity: AsyncIFtpPostprocessProvider_Impl,
    {
        unsafe extern "system" fn Begin_HandlePostprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpPostprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpPostprocessProvider_Impl::Begin_HandlePostprocess(this, core::mem::transmute_copy(&ppostprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePostprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpPostprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIFtpPostprocessProvider_Impl::Finish_HandlePostprocess(this) {
                Ok(ok__) => {
                    pftpprocessstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_HandlePostprocess: Begin_HandlePostprocess::<Identity, OFFSET>,
            Finish_HandlePostprocess: Finish_HandlePostprocess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpPostprocessProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpPreprocessProvider_Impl: Sized {
    fn Begin_HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> windows_core::Result<()>;
    fn Finish_HandlePreprocess(&self) -> windows_core::Result<FTP_PROCESS_STATUS>;
}
impl windows_core::RuntimeName for AsyncIFtpPreprocessProvider {}
impl AsyncIFtpPreprocessProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpPreprocessProvider_Vtbl
    where
        Identity: AsyncIFtpPreprocessProvider_Impl,
    {
        unsafe extern "system" fn Begin_HandlePreprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpPreprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpPreprocessProvider_Impl::Begin_HandlePreprocess(this, core::mem::transmute_copy(&ppreprocessparameters)).into()
        }
        unsafe extern "system" fn Finish_HandlePreprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpPreprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIFtpPreprocessProvider_Impl::Finish_HandlePreprocess(this) {
                Ok(ok__) => {
                    pftpprocessstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_HandlePreprocess: Begin_HandlePreprocess::<Identity, OFFSET>,
            Finish_HandlePreprocess: Finish_HandlePreprocess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpPreprocessProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIFtpRoleProvider_Impl: Sized {
    fn Begin_IsUserInRole(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszrole: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_IsUserInRole(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for AsyncIFtpRoleProvider {}
impl AsyncIFtpRoleProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIFtpRoleProvider_Vtbl
    where
        Identity: AsyncIFtpRoleProvider_Impl,
    {
        unsafe extern "system" fn Begin_IsUserInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszrole: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpRoleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIFtpRoleProvider_Impl::Begin_IsUserInRole(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername), core::mem::transmute(&pszrole)).into()
        }
        unsafe extern "system" fn Finish_IsUserInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: AsyncIFtpRoleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIFtpRoleProvider_Impl::Finish_IsUserInRole(this) {
                Ok(ok__) => {
                    pfisinrole.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_IsUserInRole: Begin_IsUserInRole::<Identity, OFFSET>,
            Finish_IsUserInRole: Finish_IsUserInRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIFtpRoleProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIMSAdminBaseSinkW_Impl: Sized {
    fn Begin_SinkNotify(&self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> windows_core::Result<()>;
    fn Finish_SinkNotify(&self) -> windows_core::Result<()>;
    fn Begin_ShutdownNotify(&self) -> windows_core::Result<()>;
    fn Finish_ShutdownNotify(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIMSAdminBaseSinkW {}
impl AsyncIMSAdminBaseSinkW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIMSAdminBaseSinkW_Vtbl
    where
        Identity: AsyncIMSAdminBaseSinkW_Impl,
    {
        unsafe extern "system" fn Begin_SinkNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> windows_core::HRESULT
        where
            Identity: AsyncIMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMSAdminBaseSinkW_Impl::Begin_SinkNotify(this, core::mem::transmute_copy(&dwmdnumelements), core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn Finish_SinkNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMSAdminBaseSinkW_Impl::Finish_SinkNotify(this).into()
        }
        unsafe extern "system" fn Begin_ShutdownNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMSAdminBaseSinkW_Impl::Begin_ShutdownNotify(this).into()
        }
        unsafe extern "system" fn Finish_ShutdownNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIMSAdminBaseSinkW_Impl::Finish_ShutdownNotify(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_SinkNotify: Begin_SinkNotify::<Identity, OFFSET>,
            Finish_SinkNotify: Finish_SinkNotify::<Identity, OFFSET>,
            Begin_ShutdownNotify: Begin_ShutdownNotify::<Identity, OFFSET>,
            Finish_ShutdownNotify: Finish_ShutdownNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIMSAdminBaseSinkW as windows_core::Interface>::IID
    }
}
pub trait IADMEXT_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn EnumDcomCLSIDs(&self, pclsiddcom: *mut windows_core::GUID, dwenumindex: u32) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IADMEXT {}
impl IADMEXT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IADMEXT_Vtbl
    where
        Identity: IADMEXT_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IADMEXT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADMEXT_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn EnumDcomCLSIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsiddcom: *mut windows_core::GUID, dwenumindex: u32) -> windows_core::HRESULT
        where
            Identity: IADMEXT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADMEXT_Impl::EnumDcomCLSIDs(this, core::mem::transmute_copy(&pclsiddcom), core::mem::transmute_copy(&dwenumindex)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IADMEXT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IADMEXT_Impl::Terminate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            EnumDcomCLSIDs: EnumDcomCLSIDs::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADMEXT as windows_core::Interface>::IID
    }
}
pub trait IFtpAuthenticationProvider_Impl: Sized {
    fn AuthenticateUser(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszpassword: &windows_core::PCWSTR, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFtpAuthenticationProvider {}
impl IFtpAuthenticationProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpAuthenticationProvider_Vtbl
    where
        Identity: IFtpAuthenticationProvider_Impl,
    {
        unsafe extern "system" fn AuthenticateUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszpassword: windows_core::PCWSTR, ppszcanonicalusername: *mut windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFtpAuthenticationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFtpAuthenticationProvider_Impl::AuthenticateUser(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername), core::mem::transmute(&pszpassword), core::mem::transmute_copy(&ppszcanonicalusername), core::mem::transmute_copy(&pfauthenticated)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AuthenticateUser: AuthenticateUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpAuthenticationProvider as windows_core::Interface>::IID
    }
}
pub trait IFtpAuthorizationProvider_Impl: Sized {
    fn GetUserAccessPermission(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszvirtualpath: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR) -> windows_core::Result<FTP_ACCESS>;
}
impl windows_core::RuntimeName for IFtpAuthorizationProvider {}
impl IFtpAuthorizationProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpAuthorizationProvider_Vtbl
    where
        Identity: IFtpAuthorizationProvider_Impl,
    {
        unsafe extern "system" fn GetUserAccessPermission<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszvirtualpath: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pftpaccess: *mut FTP_ACCESS) -> windows_core::HRESULT
        where
            Identity: IFtpAuthorizationProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFtpAuthorizationProvider_Impl::GetUserAccessPermission(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszvirtualpath), core::mem::transmute(&pszusername)) {
                Ok(ok__) => {
                    pftpaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUserAccessPermission: GetUserAccessPermission::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpAuthorizationProvider as windows_core::Interface>::IID
    }
}
pub trait IFtpHomeDirectoryProvider_Impl: Sized {
    fn GetUserHomeDirectoryData(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IFtpHomeDirectoryProvider {}
impl IFtpHomeDirectoryProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpHomeDirectoryProvider_Vtbl
    where
        Identity: IFtpHomeDirectoryProvider_Impl,
    {
        unsafe extern "system" fn GetUserHomeDirectoryData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, ppszhomedirectorydata: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IFtpHomeDirectoryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFtpHomeDirectoryProvider_Impl::GetUserHomeDirectoryData(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername)) {
                Ok(ok__) => {
                    ppszhomedirectorydata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUserHomeDirectoryData: GetUserHomeDirectoryData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpHomeDirectoryProvider as windows_core::Interface>::IID
    }
}
pub trait IFtpLogProvider_Impl: Sized {
    fn Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFtpLogProvider {}
impl IFtpLogProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpLogProvider_Vtbl
    where
        Identity: IFtpLogProvider_Impl,
    {
        unsafe extern "system" fn Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: IFtpLogProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFtpLogProvider_Impl::Log(this, core::mem::transmute_copy(&ploggingparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Log: Log::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpLogProvider as windows_core::Interface>::IID
    }
}
pub trait IFtpPostprocessProvider_Impl: Sized {
    fn HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> windows_core::Result<FTP_PROCESS_STATUS>;
}
impl windows_core::RuntimeName for IFtpPostprocessProvider {}
impl IFtpPostprocessProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpPostprocessProvider_Vtbl
    where
        Identity: IFtpPostprocessProvider_Impl,
    {
        unsafe extern "system" fn HandlePostprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT
        where
            Identity: IFtpPostprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFtpPostprocessProvider_Impl::HandlePostprocess(this, core::mem::transmute_copy(&ppostprocessparameters)) {
                Ok(ok__) => {
                    pftpprocessstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePostprocess: HandlePostprocess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpPostprocessProvider as windows_core::Interface>::IID
    }
}
pub trait IFtpPreprocessProvider_Impl: Sized {
    fn HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> windows_core::Result<FTP_PROCESS_STATUS>;
}
impl windows_core::RuntimeName for IFtpPreprocessProvider {}
impl IFtpPreprocessProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpPreprocessProvider_Vtbl
    where
        Identity: IFtpPreprocessProvider_Impl,
    {
        unsafe extern "system" fn HandlePreprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> windows_core::HRESULT
        where
            Identity: IFtpPreprocessProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFtpPreprocessProvider_Impl::HandlePreprocess(this, core::mem::transmute_copy(&ppreprocessparameters)) {
                Ok(ok__) => {
                    pftpprocessstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePreprocess: HandlePreprocess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpPreprocessProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFtpProviderConstruct_Impl: Sized {
    fn Construct(&self, configurationentries: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFtpProviderConstruct {}
#[cfg(feature = "Win32_System_Com")]
impl IFtpProviderConstruct_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpProviderConstruct_Vtbl
    where
        Identity: IFtpProviderConstruct_Impl,
    {
        unsafe extern "system" fn Construct<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IFtpProviderConstruct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFtpProviderConstruct_Impl::Construct(this, core::mem::transmute_copy(&configurationentries)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpProviderConstruct as windows_core::Interface>::IID
    }
}
pub trait IFtpRoleProvider_Impl: Sized {
    fn IsUserInRole(&self, pszsessionid: &windows_core::PCWSTR, pszsitename: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszrole: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IFtpRoleProvider {}
impl IFtpRoleProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFtpRoleProvider_Vtbl
    where
        Identity: IFtpRoleProvider_Impl,
    {
        unsafe extern "system" fn IsUserInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsessionid: windows_core::PCWSTR, pszsitename: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszrole: windows_core::PCWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFtpRoleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFtpRoleProvider_Impl::IsUserInRole(this, core::mem::transmute(&pszsessionid), core::mem::transmute(&pszsitename), core::mem::transmute(&pszusername), core::mem::transmute(&pszrole)) {
                Ok(ok__) => {
                    pfisinrole.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsUserInRole: IsUserInRole::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFtpRoleProvider as windows_core::Interface>::IID
    }
}
pub trait IMSAdminBase2W_Impl: Sized + IMSAdminBaseW_Impl {
    fn BackupWithPasswd(&self, pszmdbackuplocation: &windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RestoreWithPasswd(&self, pszmdbackuplocation: &windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Export(&self, pszpasswd: &windows_core::PCWSTR, pszfilename: &windows_core::PCWSTR, pszsourcepath: &windows_core::PCWSTR, dwmdflags: u32) -> windows_core::Result<()>;
    fn Import(&self, pszpasswd: &windows_core::PCWSTR, pszfilename: &windows_core::PCWSTR, pszsourcepath: &windows_core::PCWSTR, pszdestpath: &windows_core::PCWSTR, dwmdflags: u32) -> windows_core::Result<()>;
    fn RestoreHistory(&self, pszmdhistorylocation: &windows_core::PCWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> windows_core::Result<()>;
    fn EnumHistory(&self, pszmdhistorylocation: &windows_core::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMSAdminBase2W {}
impl IMSAdminBase2W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSAdminBase2W_Vtbl
    where
        Identity: IMSAdminBase2W_Impl,
    {
        unsafe extern "system" fn BackupWithPasswd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::BackupWithPasswd(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&dwmdversion), core::mem::transmute_copy(&dwmdflags), core::mem::transmute(&pszpasswd)).into()
        }
        unsafe extern "system" fn RestoreWithPasswd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::RestoreWithPasswd(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&dwmdversion), core::mem::transmute_copy(&dwmdflags), core::mem::transmute(&pszpasswd)).into()
        }
        unsafe extern "system" fn Export<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpasswd: windows_core::PCWSTR, pszfilename: windows_core::PCWSTR, pszsourcepath: windows_core::PCWSTR, dwmdflags: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::Export(this, core::mem::transmute(&pszpasswd), core::mem::transmute(&pszfilename), core::mem::transmute(&pszsourcepath), core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpasswd: windows_core::PCWSTR, pszfilename: windows_core::PCWSTR, pszsourcepath: windows_core::PCWSTR, pszdestpath: windows_core::PCWSTR, dwmdflags: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::Import(this, core::mem::transmute(&pszpasswd), core::mem::transmute(&pszfilename), core::mem::transmute(&pszsourcepath), core::mem::transmute(&pszdestpath), core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn RestoreHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdhistorylocation: windows_core::PCWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::RestoreHistory(this, core::mem::transmute(&pszmdhistorylocation), core::mem::transmute_copy(&dwmdmajorversion), core::mem::transmute_copy(&dwmdminorversion), core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdhistorylocation: windows_core::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase2W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase2W_Impl::EnumHistory(this, core::mem::transmute(&pszmdhistorylocation), core::mem::transmute_copy(&pdwmdmajorversion), core::mem::transmute_copy(&pdwmdminorversion), core::mem::transmute_copy(&pftmdhistorytime), core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        Self {
            base__: IMSAdminBaseW_Vtbl::new::<Identity, OFFSET>(),
            BackupWithPasswd: BackupWithPasswd::<Identity, OFFSET>,
            RestoreWithPasswd: RestoreWithPasswd::<Identity, OFFSET>,
            Export: Export::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            RestoreHistory: RestoreHistory::<Identity, OFFSET>,
            EnumHistory: EnumHistory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSAdminBase2W as windows_core::Interface>::IID || iid == &<IMSAdminBaseW as windows_core::Interface>::IID
    }
}
pub trait IMSAdminBase3W_Impl: Sized + IMSAdminBase2W_Impl {
    fn GetChildPaths(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, cchmdbuffersize: u32, pszbuffer: &windows_core::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMSAdminBase3W {}
impl IMSAdminBase3W_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSAdminBase3W_Vtbl
    where
        Identity: IMSAdminBase3W_Impl,
    {
        unsafe extern "system" fn GetChildPaths<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, cchmdbuffersize: u32, pszbuffer: windows_core::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBase3W_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBase3W_Impl::GetChildPaths(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&cchmdbuffersize), core::mem::transmute(&pszbuffer), core::mem::transmute_copy(&pcchmdrequiredbuffersize)).into()
        }
        Self { base__: IMSAdminBase2W_Vtbl::new::<Identity, OFFSET>(), GetChildPaths: GetChildPaths::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSAdminBase3W as windows_core::Interface>::IID || iid == &<IMSAdminBaseW as windows_core::Interface>::IID || iid == &<IMSAdminBase2W as windows_core::Interface>::IID
    }
}
pub trait IMSAdminBaseSinkW_Impl: Sized {
    fn SinkNotify(&self, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> windows_core::Result<()>;
    fn ShutdownNotify(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMSAdminBaseSinkW {}
impl IMSAdminBaseSinkW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSAdminBaseSinkW_Vtbl
    where
        Identity: IMSAdminBaseSinkW_Impl,
    {
        unsafe extern "system" fn SinkNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseSinkW_Impl::SinkNotify(this, core::mem::transmute_copy(&dwmdnumelements), core::mem::transmute_copy(&pcochangelist)).into()
        }
        unsafe extern "system" fn ShutdownNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseSinkW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseSinkW_Impl::ShutdownNotify(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SinkNotify: SinkNotify::<Identity, OFFSET>,
            ShutdownNotify: ShutdownNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSAdminBaseSinkW as windows_core::Interface>::IID
    }
}
pub trait IMSAdminBaseW_Impl: Sized {
    fn AddKey(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteKey(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteChildKeys(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumKeys(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pszmdname: windows_core::PWSTR, dwmdenumobjectindex: u32) -> windows_core::Result<()>;
    fn CopyKey(&self, hmdsourcehandle: u32, pszmdsourcepath: &windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: &windows_core::PCWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RenameKey(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pszmdnewname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD) -> windows_core::Result<()>;
    fn GetData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> windows_core::Result<()>;
    fn DeleteData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> windows_core::Result<()>;
    fn EnumData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> windows_core::Result<()>;
    fn GetAllData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>;
    fn DeleteAllData(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, dwmdusertype: u32, dwmddatatype: u32) -> windows_core::Result<()>;
    fn CopyData(&self, hmdsourcehandle: u32, pszmdsourcepath: &windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: &windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDataPaths(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>;
    fn OpenKey(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32) -> windows_core::Result<u32>;
    fn CloseKey(&self, hmdhandle: u32) -> windows_core::Result<()>;
    fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> windows_core::Result<()>;
    fn SaveData(&self) -> windows_core::Result<()>;
    fn GetHandleInfo(&self, hmdhandle: u32) -> windows_core::Result<METADATA_HANDLE_INFO>;
    fn GetSystemChangeNumber(&self) -> windows_core::Result<u32>;
    fn GetDataSetNumber(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn SetLastChangeTime(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLastChangeTime(&self, hmdhandle: u32, pszmdpath: &windows_core::PCWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn KeyExchangePhase1(&self) -> windows_core::Result<()>;
    fn KeyExchangePhase2(&self) -> windows_core::Result<()>;
    fn Backup(&self, pszmdbackuplocation: &windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> windows_core::Result<()>;
    fn Restore(&self, pszmdbackuplocation: &windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> windows_core::Result<()>;
    fn EnumBackups(&self, pszmdbackuplocation: &windows_core::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::Result<()>;
    fn DeleteBackup(&self, pszmdbackuplocation: &windows_core::PCWSTR, dwmdversion: u32) -> windows_core::Result<()>;
    fn UnmarshalInterface(&self) -> windows_core::Result<IMSAdminBaseW>;
    fn GetServerGuid(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMSAdminBaseW {}
impl IMSAdminBaseW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSAdminBaseW_Vtbl
    where
        Identity: IMSAdminBaseW_Impl,
    {
        unsafe extern "system" fn AddKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::AddKey(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::DeleteKey(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath)).into()
        }
        unsafe extern "system" fn DeleteChildKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::DeleteChildKeys(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pszmdname: windows_core::PWSTR, dwmdenumobjectindex: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::EnumKeys(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pszmdname), core::mem::transmute_copy(&dwmdenumobjectindex)).into()
        }
        unsafe extern "system" fn CopyKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: windows_core::PCWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::CopyKey(this, core::mem::transmute_copy(&hmdsourcehandle), core::mem::transmute(&pszmdsourcepath), core::mem::transmute_copy(&hmddesthandle), core::mem::transmute(&pszmddestpath), core::mem::transmute_copy(&bmdoverwriteflag), core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn RenameKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pszmdnewname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::RenameKey(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute(&pszmdnewname)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::SetData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pmdrmddata)).into()
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::GetData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pmdrmddata), core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn DeleteData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::DeleteData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&dwmdidentifier), core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn EnumData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::EnumData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pmdrmddata), core::mem::transmute_copy(&dwmdenumdataindex), core::mem::transmute_copy(&pdwmdrequireddatalen)).into()
        }
        unsafe extern "system" fn GetAllData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::GetAllData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&dwmdattributes), core::mem::transmute_copy(&dwmdusertype), core::mem::transmute_copy(&dwmddatatype), core::mem::transmute_copy(&pdwmdnumdataentries), core::mem::transmute_copy(&pdwmddatasetnumber), core::mem::transmute_copy(&dwmdbuffersize), core::mem::transmute_copy(&pbmdbuffer), core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        unsafe extern "system" fn DeleteAllData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, dwmdusertype: u32, dwmddatatype: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::DeleteAllData(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&dwmdusertype), core::mem::transmute_copy(&dwmddatatype)).into()
        }
        unsafe extern "system" fn CopyData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::CopyData(this, core::mem::transmute_copy(&hmdsourcehandle), core::mem::transmute(&pszmdsourcepath), core::mem::transmute_copy(&hmddesthandle), core::mem::transmute(&pszmddestpath), core::mem::transmute_copy(&dwmdattributes), core::mem::transmute_copy(&dwmdusertype), core::mem::transmute_copy(&dwmddatatype), core::mem::transmute_copy(&bmdcopyflag)).into()
        }
        unsafe extern "system" fn GetDataPaths<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::GetDataPaths(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&dwmdidentifier), core::mem::transmute_copy(&dwmddatatype), core::mem::transmute_copy(&dwmdbuffersize), core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        unsafe extern "system" fn OpenKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSAdminBaseW_Impl::OpenKey(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&dwmdaccessrequested), core::mem::transmute_copy(&dwmdtimeout)) {
                Ok(ok__) => {
                    phmdnewhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::CloseKey(this, core::mem::transmute_copy(&hmdhandle)).into()
        }
        unsafe extern "system" fn ChangePermissions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::ChangePermissions(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute_copy(&dwmdtimeout), core::mem::transmute_copy(&dwmdaccessrequested)).into()
        }
        unsafe extern "system" fn SaveData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::SaveData(this).into()
        }
        unsafe extern "system" fn GetHandleInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSAdminBaseW_Impl::GetHandleInfo(this, core::mem::transmute_copy(&hmdhandle)) {
                Ok(ok__) => {
                    pmdhiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSystemChangeNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSAdminBaseW_Impl::GetSystemChangeNumber(this) {
                Ok(ok__) => {
                    pdwsystemchangenumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataSetNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pdwmddatasetnumber: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSAdminBaseW_Impl::GetDataSetNumber(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath)) {
                Ok(ok__) => {
                    pdwmddatasetnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::SetLastChangeTime(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pftmdlastchangetime), core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn GetLastChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmdhandle: u32, pszmdpath: windows_core::PCWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::GetLastChangeTime(this, core::mem::transmute_copy(&hmdhandle), core::mem::transmute(&pszmdpath), core::mem::transmute_copy(&pftmdlastchangetime), core::mem::transmute_copy(&blocaltime)).into()
        }
        unsafe extern "system" fn KeyExchangePhase1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::KeyExchangePhase1(this).into()
        }
        unsafe extern "system" fn KeyExchangePhase2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::KeyExchangePhase2(this).into()
        }
        unsafe extern "system" fn Backup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::Backup(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&dwmdversion), core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::Restore(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&dwmdversion), core::mem::transmute_copy(&dwmdflags)).into()
        }
        unsafe extern "system" fn EnumBackups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::EnumBackups(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&pdwmdversion), core::mem::transmute_copy(&pftmdbackuptime), core::mem::transmute_copy(&dwmdenumindex)).into()
        }
        unsafe extern "system" fn DeleteBackup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmdbackuplocation: windows_core::PCWSTR, dwmdversion: u32) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::DeleteBackup(this, core::mem::transmute(&pszmdbackuplocation), core::mem::transmute_copy(&dwmdversion)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piadmbwinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSAdminBaseW_Impl::UnmarshalInterface(this) {
                Ok(ok__) => {
                    piadmbwinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSAdminBaseW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSAdminBaseW_Impl::GetServerGuid(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddKey: AddKey::<Identity, OFFSET>,
            DeleteKey: DeleteKey::<Identity, OFFSET>,
            DeleteChildKeys: DeleteChildKeys::<Identity, OFFSET>,
            EnumKeys: EnumKeys::<Identity, OFFSET>,
            CopyKey: CopyKey::<Identity, OFFSET>,
            RenameKey: RenameKey::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            DeleteData: DeleteData::<Identity, OFFSET>,
            EnumData: EnumData::<Identity, OFFSET>,
            GetAllData: GetAllData::<Identity, OFFSET>,
            DeleteAllData: DeleteAllData::<Identity, OFFSET>,
            CopyData: CopyData::<Identity, OFFSET>,
            GetDataPaths: GetDataPaths::<Identity, OFFSET>,
            OpenKey: OpenKey::<Identity, OFFSET>,
            CloseKey: CloseKey::<Identity, OFFSET>,
            ChangePermissions: ChangePermissions::<Identity, OFFSET>,
            SaveData: SaveData::<Identity, OFFSET>,
            GetHandleInfo: GetHandleInfo::<Identity, OFFSET>,
            GetSystemChangeNumber: GetSystemChangeNumber::<Identity, OFFSET>,
            GetDataSetNumber: GetDataSetNumber::<Identity, OFFSET>,
            SetLastChangeTime: SetLastChangeTime::<Identity, OFFSET>,
            GetLastChangeTime: GetLastChangeTime::<Identity, OFFSET>,
            KeyExchangePhase1: KeyExchangePhase1::<Identity, OFFSET>,
            KeyExchangePhase2: KeyExchangePhase2::<Identity, OFFSET>,
            Backup: Backup::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            EnumBackups: EnumBackups::<Identity, OFFSET>,
            DeleteBackup: DeleteBackup::<Identity, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, OFFSET>,
            GetServerGuid: GetServerGuid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSAdminBaseW as windows_core::Interface>::IID
    }
}
pub trait IMSImpExpHelpW_Impl: Sized {
    fn EnumeratePathsInFile(&self, pszfilename: &windows_core::PCWSTR, pszkeytype: &windows_core::PCWSTR, dwmdbuffersize: u32, pszbuffer: &windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMSImpExpHelpW {}
impl IMSImpExpHelpW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSImpExpHelpW_Vtbl
    where
        Identity: IMSImpExpHelpW_Impl,
    {
        unsafe extern "system" fn EnumeratePathsInFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, pszkeytype: windows_core::PCWSTR, dwmdbuffersize: u32, pszbuffer: windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMSImpExpHelpW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSImpExpHelpW_Impl::EnumeratePathsInFile(this, core::mem::transmute(&pszfilename), core::mem::transmute(&pszkeytype), core::mem::transmute_copy(&dwmdbuffersize), core::mem::transmute(&pszbuffer), core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumeratePathsInFile: EnumeratePathsInFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSImpExpHelpW as windows_core::Interface>::IID
    }
}
