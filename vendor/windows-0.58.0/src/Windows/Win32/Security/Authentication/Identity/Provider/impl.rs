#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIAssociatedIdentityProvider_Impl: Sized {
    fn Begin_AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Finish_AssociateIdentity(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_DisassociateIdentity(&self) -> windows_core::Result<()>;
    fn Begin_ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_ChangeCredential(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for AsyncIAssociatedIdentityProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIAssociatedIdentityProvider_Vtbl
    where
        Identity: AsyncIAssociatedIdentityProvider_Impl,
    {
        unsafe extern "system" fn Begin_AssociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAssociatedIdentityProvider_Impl::Begin_AssociateIdentity(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIAssociatedIdentityProvider_Impl::Finish_AssociateIdentity(this) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAssociatedIdentityProvider_Impl::Begin_DisassociateIdentity(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAssociatedIdentityProvider_Impl::Finish_DisassociateIdentity(this).into()
        }
        unsafe extern "system" fn Begin_ChangeCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAssociatedIdentityProvider_Impl::Begin_ChangeCredential(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_ChangeCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIAssociatedIdentityProvider_Impl::Finish_ChangeCredential(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_AssociateIdentity: Begin_AssociateIdentity::<Identity, OFFSET>,
            Finish_AssociateIdentity: Finish_AssociateIdentity::<Identity, OFFSET>,
            Begin_DisassociateIdentity: Begin_DisassociateIdentity::<Identity, OFFSET>,
            Finish_DisassociateIdentity: Finish_DisassociateIdentity::<Identity, OFFSET>,
            Begin_ChangeCredential: Begin_ChangeCredential::<Identity, OFFSET>,
            Finish_ChangeCredential: Finish_ChangeCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAssociatedIdentityProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait AsyncIConnectedIdentityProvider_Impl: Sized {
    fn Begin_ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> windows_core::Result<()>;
    fn Finish_ConnectIdentity(&self) -> windows_core::Result<()>;
    fn Begin_DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn Finish_DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn Begin_IsConnected(&self) -> windows_core::Result<()>;
    fn Finish_IsConnected(&self) -> windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn Begin_GetUrl(&self, identifier: IDENTITY_URL, context: Option<&super::super::super::super::System::Com::IBindCtx>) -> windows_core::Result<()>;
    fn Finish_GetUrl(&self, postdata: *mut windows_core::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Begin_GetAccountState(&self) -> windows_core::Result<()>;
    fn Finish_GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for AsyncIConnectedIdentityProvider {}
#[cfg(feature = "Win32_System_Com")]
impl AsyncIConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIConnectedIdentityProvider_Vtbl
    where
        Identity: AsyncIConnectedIdentityProvider_Impl,
    {
        unsafe extern "system" fn Begin_ConnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Begin_ConnectIdentity(this, core::mem::transmute_copy(&authbuffer), core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Finish_ConnectIdentity(this).into()
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Begin_DisconnectIdentity(this).into()
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Finish_DisconnectIdentity(this).into()
        }
        unsafe extern "system" fn Begin_IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Begin_IsConnected(this).into()
        }
        unsafe extern "system" fn Finish_IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIConnectedIdentityProvider_Impl::Finish_IsConnected(this) {
                Ok(ok__) => {
                    connected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: IDENTITY_URL, context: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Begin_GetUrl(this, core::mem::transmute_copy(&identifier), windows_core::from_raw_borrowed(&context)).into()
        }
        unsafe extern "system" fn Finish_GetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, postdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>, url: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Finish_GetUrl(this, core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn Begin_GetAccountState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIConnectedIdentityProvider_Impl::Begin_GetAccountState(this).into()
        }
        unsafe extern "system" fn Finish_GetAccountState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> windows_core::HRESULT
        where
            Identity: AsyncIConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIConnectedIdentityProvider_Impl::Finish_GetAccountState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_ConnectIdentity: Begin_ConnectIdentity::<Identity, OFFSET>,
            Finish_ConnectIdentity: Finish_ConnectIdentity::<Identity, OFFSET>,
            Begin_DisconnectIdentity: Begin_DisconnectIdentity::<Identity, OFFSET>,
            Finish_DisconnectIdentity: Finish_DisconnectIdentity::<Identity, OFFSET>,
            Begin_IsConnected: Begin_IsConnected::<Identity, OFFSET>,
            Finish_IsConnected: Finish_IsConnected::<Identity, OFFSET>,
            Begin_GetUrl: Begin_GetUrl::<Identity, OFFSET>,
            Finish_GetUrl: Finish_GetUrl::<Identity, OFFSET>,
            Begin_GetAccountState: Begin_GetAccountState::<Identity, OFFSET>,
            Finish_GetAccountState: Finish_GetAccountState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIConnectedIdentityProvider as windows_core::Interface>::IID
    }
}
pub trait AsyncIIdentityAdvise_Impl: Sized {
    fn Begin_IdentityUpdated(&self, dwidentityupdateevents: u32, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_IdentityUpdated(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIIdentityAdvise {}
impl AsyncIIdentityAdvise_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIIdentityAdvise_Vtbl
    where
        Identity: AsyncIIdentityAdvise_Impl,
    {
        unsafe extern "system" fn Begin_IdentityUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAdvise_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAdvise_Impl::Begin_IdentityUpdated(this, core::mem::transmute_copy(&dwidentityupdateevents), core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAdvise_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAdvise_Impl::Finish_IdentityUpdated(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_IdentityUpdated: Begin_IdentityUpdated::<Identity, OFFSET>,
            Finish_IdentityUpdated: Finish_IdentityUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityAdvise as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthentication_Impl: Sized {
    fn Begin_SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> windows_core::Result<()>;
    fn Finish_SetIdentityCredential(&self) -> windows_core::Result<()>;
    fn Begin_ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: *mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for AsyncIIdentityAuthentication {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthentication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIIdentityAuthentication_Vtbl
    where
        Identity: AsyncIIdentityAuthentication_Impl,
    {
        unsafe extern "system" fn Begin_SetIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAuthentication_Impl::Begin_SetIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAuthentication_Impl::Finish_SetIdentityCredential(this).into()
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAuthentication_Impl::Begin_ValidateIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength), core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityAuthentication_Impl::Finish_ValidateIdentityCredential(this, core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_SetIdentityCredential: Begin_SetIdentityCredential::<Identity, OFFSET>,
            Finish_SetIdentityCredential: Finish_SetIdentityCredential::<Identity, OFFSET>,
            Begin_ValidateIdentityCredential: Begin_ValidateIdentityCredential::<Identity, OFFSET>,
            Finish_ValidateIdentityCredential: Finish_ValidateIdentityCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityAuthentication as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProvider_Impl: Sized {
    fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_GetIdentityEnum(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(&self, lpszusername: &windows_core::PCWSTR, pkeywordstoadd: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_Create(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(&self, ppropertystore: Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Finish_Import(&self) -> windows_core::Result<()>;
    fn Begin_Delete(&self, lpszuniqueid: &windows_core::PCWSTR, pkeywordstodelete: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_Delete(&self) -> windows_core::Result<()>;
    fn Begin_FindByUniqueID(&self, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_FindByUniqueID(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(&self) -> windows_core::Result<()>;
    fn Finish_GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(&self, pidentityadvise: Option<&IIdentityAdvise>, dwidentityupdateevents: u32) -> windows_core::Result<()>;
    fn Finish_Advise(&self) -> windows_core::Result<u32>;
    fn Begin_UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn Finish_UnAdvise(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for AsyncIIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIIdentityProvider_Vtbl
    where
        Identity: AsyncIIdentityProvider_Impl,
    {
        unsafe extern "system" fn Begin_GetIdentityEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_GetIdentityEnum(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityProvider_Impl::Finish_GetIdentityEnum(this) {
                Ok(ok__) => {
                    ppidentityenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: windows_core::PCWSTR, pkeywordstoadd: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_Create(this, core::mem::transmute(&lpszusername), core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Finish_Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityProvider_Impl::Finish_Create(this) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_Import(this, windows_core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn Finish_Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Finish_Import(this).into()
        }
        unsafe extern "system" fn Begin_Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pkeywordstodelete: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_Delete(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn Finish_Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Finish_Delete(this).into()
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_FindByUniqueID(this, core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityProvider_Impl::Finish_FindByUniqueID(this) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_GetProviderPropertyStore(this).into()
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityProvider_Impl::Finish_GetProviderPropertyStore(this) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentityadvise: *mut core::ffi::c_void, dwidentityupdateevents: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_Advise(this, windows_core::from_raw_borrowed(&pidentityadvise), core::mem::transmute_copy(&dwidentityupdateevents)).into()
        }
        unsafe extern "system" fn Finish_Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityProvider_Impl::Finish_Advise(this) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Begin_UnAdvise(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Finish_UnAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityProvider_Impl::Finish_UnAdvise(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetIdentityEnum: Begin_GetIdentityEnum::<Identity, OFFSET>,
            Finish_GetIdentityEnum: Finish_GetIdentityEnum::<Identity, OFFSET>,
            Begin_Create: Begin_Create::<Identity, OFFSET>,
            Finish_Create: Finish_Create::<Identity, OFFSET>,
            Begin_Import: Begin_Import::<Identity, OFFSET>,
            Finish_Import: Finish_Import::<Identity, OFFSET>,
            Begin_Delete: Begin_Delete::<Identity, OFFSET>,
            Finish_Delete: Finish_Delete::<Identity, OFFSET>,
            Begin_FindByUniqueID: Begin_FindByUniqueID::<Identity, OFFSET>,
            Finish_FindByUniqueID: Finish_FindByUniqueID::<Identity, OFFSET>,
            Begin_GetProviderPropertyStore: Begin_GetProviderPropertyStore::<Identity, OFFSET>,
            Finish_GetProviderPropertyStore: Finish_GetProviderPropertyStore::<Identity, OFFSET>,
            Begin_Advise: Begin_Advise::<Identity, OFFSET>,
            Finish_Advise: Finish_Advise::<Identity, OFFSET>,
            Begin_UnAdvise: Begin_UnAdvise::<Identity, OFFSET>,
            Finish_UnAdvise: Finish_UnAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityStore_Impl: Sized {
    fn Begin_GetCount(&self) -> windows_core::Result<()>;
    fn Finish_GetCount(&self) -> windows_core::Result<u32>;
    fn Begin_GetAt(&self, dwprovider: u32, pprovguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_GetAt(&self, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Begin_AddToCache(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_AddToCache(&self) -> windows_core::Result<()>;
    fn Begin_ConvertToSid(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8) -> windows_core::Result<()>;
    fn Finish_ConvertToSid(&self, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::Result<()>;
    fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_EnumerateIdentities(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Reset(&self) -> windows_core::Result<()>;
    fn Finish_Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for AsyncIIdentityStore {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIIdentityStore_Vtbl
    where
        Identity: AsyncIIdentityStore_Impl,
    {
        unsafe extern "system" fn Begin_GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_GetCount(this).into()
        }
        unsafe extern "system" fn Finish_GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproviders: *mut u32) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityStore_Impl::Finish_GetCount(this) {
                Ok(ok__) => {
                    pdwproviders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprovider: u32, pprovguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_GetAt(this, core::mem::transmute_copy(&dwprovider), core::mem::transmute_copy(&pprovguid)).into()
        }
        unsafe extern "system" fn Finish_GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Finish_GetAt(this, core::mem::transmute_copy(&pprovguid), core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn Begin_AddToCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_AddToCache(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_AddToCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Finish_AddToCache(this).into()
        }
        unsafe extern "system" fn Begin_ConvertToSid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_ConvertToSid(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid), core::mem::transmute_copy(&cbsid), core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn Finish_ConvertToSid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Finish_ConvertToSid(this, core::mem::transmute_copy(&psid), core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_EnumerateIdentities(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)).into()
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match AsyncIIdentityStore_Impl::Finish_EnumerateIdentities(this) {
                Ok(ok__) => {
                    ppidentityenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Begin_Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Begin_Reset(this).into()
        }
        unsafe extern "system" fn Finish_Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStore_Impl::Finish_Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_GetCount: Begin_GetCount::<Identity, OFFSET>,
            Finish_GetCount: Finish_GetCount::<Identity, OFFSET>,
            Begin_GetAt: Begin_GetAt::<Identity, OFFSET>,
            Finish_GetAt: Finish_GetAt::<Identity, OFFSET>,
            Begin_AddToCache: Begin_AddToCache::<Identity, OFFSET>,
            Finish_AddToCache: Finish_AddToCache::<Identity, OFFSET>,
            Begin_ConvertToSid: Begin_ConvertToSid::<Identity, OFFSET>,
            Finish_ConvertToSid: Finish_ConvertToSid::<Identity, OFFSET>,
            Begin_EnumerateIdentities: Begin_EnumerateIdentities::<Identity, OFFSET>,
            Finish_EnumerateIdentities: Finish_EnumerateIdentities::<Identity, OFFSET>,
            Begin_Reset: Begin_Reset::<Identity, OFFSET>,
            Finish_Reset: Finish_Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityStore as windows_core::Interface>::IID
    }
}
pub trait AsyncIIdentityStoreEx_Impl: Sized {
    fn Begin_CreateConnectedIdentity(&self, localname: &windows_core::PCWSTR, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_CreateConnectedIdentity(&self) -> windows_core::Result<()>;
    fn Begin_DeleteConnectedIdentity(&self, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_DeleteConnectedIdentity(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for AsyncIIdentityStoreEx {}
impl AsyncIIdentityStoreEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> AsyncIIdentityStoreEx_Vtbl
    where
        Identity: AsyncIIdentityStoreEx_Impl,
    {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localname: windows_core::PCWSTR, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStoreEx_Impl::Begin_CreateConnectedIdentity(this, core::mem::transmute(&localname), core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStoreEx_Impl::Finish_CreateConnectedIdentity(this).into()
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStoreEx_Impl::Begin_DeleteConnectedIdentity(this, core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: AsyncIIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            AsyncIIdentityStoreEx_Impl::Finish_DeleteConnectedIdentity(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_CreateConnectedIdentity: Begin_CreateConnectedIdentity::<Identity, OFFSET>,
            Finish_CreateConnectedIdentity: Finish_CreateConnectedIdentity::<Identity, OFFSET>,
            Begin_DeleteConnectedIdentity: Begin_DeleteConnectedIdentity::<Identity, OFFSET>,
            Finish_DeleteConnectedIdentity: Finish_DeleteConnectedIdentity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIIdentityStoreEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAssociatedIdentityProvider_Impl: Sized {
    fn AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAssociatedIdentityProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAssociatedIdentityProvider_Vtbl
    where
        Identity: IAssociatedIdentityProvider_Impl,
    {
        unsafe extern "system" fn AssociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAssociatedIdentityProvider_Impl::AssociateIdentity(this, core::mem::transmute_copy(&hwndparent)) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssociatedIdentityProvider_Impl::DisassociateIdentity(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
        }
        unsafe extern "system" fn ChangeCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAssociatedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssociatedIdentityProvider_Impl::ChangeCredential(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateIdentity: AssociateIdentity::<Identity, OFFSET>,
            DisassociateIdentity: DisassociateIdentity::<Identity, OFFSET>,
            ChangeCredential: ChangeCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssociatedIdentityProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConnectedIdentityProvider_Impl: Sized {
    fn ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> windows_core::Result<()>;
    fn DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn GetUrl(&self, identifier: IDENTITY_URL, context: Option<&super::super::super::super::System::Com::IBindCtx>, postdata: *mut windows_core::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IConnectedIdentityProvider {}
#[cfg(feature = "Win32_System_Com")]
impl IConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConnectedIdentityProvider_Vtbl
    where
        Identity: IConnectedIdentityProvider_Impl,
    {
        unsafe extern "system" fn ConnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> windows_core::HRESULT
        where
            Identity: IConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnectedIdentityProvider_Impl::ConnectIdentity(this, core::mem::transmute_copy(&authbuffer), core::mem::transmute_copy(&authbuffersize)).into()
        }
        unsafe extern "system" fn DisconnectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnectedIdentityProvider_Impl::DisconnectIdentity(this).into()
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, connected: *mut super::super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectedIdentityProvider_Impl::IsConnected(this) {
                Ok(ok__) => {
                    connected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: IDENTITY_URL, context: *mut core::ffi::c_void, postdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>, url: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnectedIdentityProvider_Impl::GetUrl(this, core::mem::transmute_copy(&identifier), windows_core::from_raw_borrowed(&context), core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn GetAccountState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> windows_core::HRESULT
        where
            Identity: IConnectedIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnectedIdentityProvider_Impl::GetAccountState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectIdentity: ConnectIdentity::<Identity, OFFSET>,
            DisconnectIdentity: DisconnectIdentity::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetUrl: GetUrl::<Identity, OFFSET>,
            GetAccountState: GetAccountState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectedIdentityProvider as windows_core::Interface>::IID
    }
}
pub trait IIdentityAdvise_Impl: Sized {
    fn IdentityUpdated(&self, dwidentityupdateevents: &IdentityUpdateEvent, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IIdentityAdvise {}
impl IIdentityAdvise_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdentityAdvise_Vtbl
    where
        Identity: IIdentityAdvise_Impl,
    {
        unsafe extern "system" fn IdentityUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IIdentityAdvise_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityAdvise_Impl::IdentityUpdated(this, core::mem::transmute(&dwidentityupdateevents), core::mem::transmute(&lpszuniqueid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IdentityUpdated: IdentityUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityAdvise as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthentication_Impl: Sized {
    fn SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> windows_core::Result<()>;
    fn ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IIdentityAuthentication {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthentication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdentityAuthentication_Vtbl
    where
        Identity: IIdentityAuthentication_Impl,
    {
        unsafe extern "system" fn SetIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> windows_core::HRESULT
        where
            Identity: IIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityAuthentication_Impl::SetIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength)).into()
        }
        unsafe extern "system" fn ValidateIdentityCredential<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityAuthentication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityAuthentication_Impl::ValidateIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength), core::mem::transmute_copy(&ppidentityproperties)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIdentityCredential: SetIdentityCredential::<Identity, OFFSET>,
            ValidateIdentityCredential: ValidateIdentityCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityAuthentication as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProvider_Impl: Sized {
    fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(&self, lpszusername: &windows_core::PCWSTR, pppropertystore: *mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Import(&self, ppropertystore: Option<&super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Delete(&self, lpszuniqueid: &windows_core::PCWSTR, pkeywordstodelete: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn FindByUniqueID(&self, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(&self, pidentityadvise: Option<&IIdentityAdvise>, dwidentityupdateevents: &IdentityUpdateEvent) -> windows_core::Result<u32>;
    fn UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IIdentityProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdentityProvider_Vtbl
    where
        Identity: IIdentityProvider_Impl,
    {
        unsafe extern "system" fn GetIdentityEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityProvider_Impl::GetIdentityEnum(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)) {
                Ok(ok__) => {
                    ppidentityenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: windows_core::PCWSTR, pppropertystore: *mut *mut core::ffi::c_void, pkeywordstoadd: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityProvider_Impl::Create(this, core::mem::transmute(&lpszusername), core::mem::transmute_copy(&pppropertystore), core::mem::transmute_copy(&pkeywordstoadd)).into()
        }
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityProvider_Impl::Import(this, windows_core::from_raw_borrowed(&ppropertystore)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pkeywordstodelete: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityProvider_Impl::Delete(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&pkeywordstodelete)).into()
        }
        unsafe extern "system" fn FindByUniqueID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityProvider_Impl::FindByUniqueID(this, core::mem::transmute(&lpszuniqueid)) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityProvider_Impl::GetProviderPropertyStore(this) {
                Ok(ok__) => {
                    pppropertystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentityadvise: *mut core::ffi::c_void, dwidentityupdateevents: u32, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityProvider_Impl::Advise(this, windows_core::from_raw_borrowed(&pidentityadvise), core::mem::transmute(&dwidentityupdateevents)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IIdentityProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityProvider_Impl::UnAdvise(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdentityEnum: GetIdentityEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            FindByUniqueID: FindByUniqueID::<Identity, OFFSET>,
            GetProviderPropertyStore: GetProviderPropertyStore::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityStore_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, dwprovider: u32, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddToCache(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ConvertToSid(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::Result<()>;
    fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IIdentityStore {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdentityStore_Vtbl
    where
        Identity: IIdentityStore_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproviders: *mut u32) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityStore_Impl::GetCount(this) {
                Ok(ok__) => {
                    pdwproviders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprovider: u32, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStore_Impl::GetAt(this, core::mem::transmute_copy(&dwprovider), core::mem::transmute_copy(&pprovguid), core::mem::transmute_copy(&ppidentityprovider)).into()
        }
        unsafe extern "system" fn AddToCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStore_Impl::AddToCache(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn ConvertToSid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStore_Impl::ConvertToSid(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid), core::mem::transmute_copy(&cbsid), core::mem::transmute_copy(&psid), core::mem::transmute_copy(&pcbrequiredsid)).into()
        }
        unsafe extern "system" fn EnumerateIdentities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIdentityStore_Impl::EnumerateIdentities(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)) {
                Ok(ok__) => {
                    ppidentityenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIdentityStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStore_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            AddToCache: AddToCache::<Identity, OFFSET>,
            ConvertToSid: ConvertToSid::<Identity, OFFSET>,
            EnumerateIdentities: EnumerateIdentities::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityStore as windows_core::Interface>::IID
    }
}
pub trait IIdentityStoreEx_Impl: Sized {
    fn CreateConnectedIdentity(&self, localname: &windows_core::PCWSTR, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteConnectedIdentity(&self, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IIdentityStoreEx {}
impl IIdentityStoreEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIdentityStoreEx_Vtbl
    where
        Identity: IIdentityStoreEx_Impl,
    {
        unsafe extern "system" fn CreateConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localname: windows_core::PCWSTR, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStoreEx_Impl::CreateConnectedIdentity(this, core::mem::transmute(&localname), core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IIdentityStoreEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIdentityStoreEx_Impl::DeleteConnectedIdentity(this, core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateConnectedIdentity: CreateConnectedIdentity::<Identity, OFFSET>,
            DeleteConnectedIdentity: DeleteConnectedIdentity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityStoreEx as windows_core::Interface>::IID
    }
}
