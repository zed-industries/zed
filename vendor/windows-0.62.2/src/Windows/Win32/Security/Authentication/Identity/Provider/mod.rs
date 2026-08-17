#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCOUNT_STATE(pub i32);
windows_core::imp::define_interface!(AsyncIAssociatedIdentityProvider, AsyncIAssociatedIdentityProvider_Vtbl, 0x2834d6ed_297e_4e72_8a51_961e86f05152);
windows_core::imp::interface_hierarchy!(AsyncIAssociatedIdentityProvider, windows_core::IUnknown);
impl AsyncIAssociatedIdentityProvider {
    pub unsafe fn Begin_AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_AssociateIdentity)(windows_core::Interface::as_raw(self), hwndparent).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_AssociateIdentity(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_AssociateIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Begin_DisassociateIdentity<P1>(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_DisassociateIdentity)(windows_core::Interface::as_raw(self), hwndparent, lpszuniqueid.param().abi()).ok() }
    }
    pub unsafe fn Finish_DisassociateIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_DisassociateIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_ChangeCredential<P1>(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_ChangeCredential)(windows_core::Interface::as_raw(self), hwndparent, lpszuniqueid.param().abi()).ok() }
    }
    pub unsafe fn Finish_ChangeCredential(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_ChangeCredential)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAssociatedIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_AssociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_AssociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_AssociateIdentity: usize,
    pub Begin_DisassociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_DisassociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_ChangeCredential: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_ChangeCredential: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIAssociatedIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn Begin_AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Finish_AssociateIdentity(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_DisassociateIdentity(&self) -> windows_core::Result<()>;
    fn Begin_ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_ChangeCredential(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_AssociateIdentity<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAssociatedIdentityProvider_Impl::Begin_AssociateIdentity(this, core::mem::transmute_copy(&hwndparent)).into()
            }
        }
        unsafe extern "system" fn Finish_AssociateIdentity<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIAssociatedIdentityProvider_Impl::Finish_AssociateIdentity(this) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_DisassociateIdentity<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAssociatedIdentityProvider_Impl::Begin_DisassociateIdentity(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        unsafe extern "system" fn Finish_DisassociateIdentity<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAssociatedIdentityProvider_Impl::Finish_DisassociateIdentity(this).into()
            }
        }
        unsafe extern "system" fn Begin_ChangeCredential<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAssociatedIdentityProvider_Impl::Begin_ChangeCredential(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        unsafe extern "system" fn Finish_ChangeCredential<Identity: AsyncIAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAssociatedIdentityProvider_Impl::Finish_ChangeCredential(this).into()
            }
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for AsyncIAssociatedIdentityProvider {}
windows_core::imp::define_interface!(AsyncIConnectedIdentityProvider, AsyncIConnectedIdentityProvider_Vtbl, 0x9ce55141_bce9_4e15_824d_43d79f512f93);
windows_core::imp::interface_hierarchy!(AsyncIConnectedIdentityProvider, windows_core::IUnknown);
impl AsyncIConnectedIdentityProvider {
    pub unsafe fn Begin_ConnectIdentity(&self, authbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_ConnectIdentity)(windows_core::Interface::as_raw(self), core::mem::transmute(authbuffer.as_ptr()), authbuffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Finish_ConnectIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_ConnectIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_DisconnectIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_DisconnectIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_DisconnectIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_DisconnectIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_IsConnected(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_IsConnected)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_IsConnected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin_GetUrl<P1>(&self, identifier: IDENTITY_URL, context: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::super::System::Com::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetUrl)(windows_core::Interface::as_raw(self), identifier, context.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_GetUrl)(windows_core::Interface::as_raw(self), core::mem::transmute(postdata), url as _).ok() }
    }
    pub unsafe fn Begin_GetAccountState(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetAccountState)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_GetAccountState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIConnectedIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_ConnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Finish_ConnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_DisconnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_DisconnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin_GetUrl: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_URL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin_GetUrl: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Finish_GetUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::System::Variant::VARIANT, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Finish_GetUrl: usize,
    pub Begin_GetAccountState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_GetAccountState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ACCOUNT_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AsyncIConnectedIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn Begin_ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> windows_core::Result<()>;
    fn Finish_ConnectIdentity(&self) -> windows_core::Result<()>;
    fn Begin_DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn Finish_DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn Begin_IsConnected(&self) -> windows_core::Result<()>;
    fn Finish_IsConnected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Begin_GetUrl(&self, identifier: IDENTITY_URL, context: windows_core::Ref<super::super::super::super::System::Com::IBindCtx>) -> windows_core::Result<()>;
    fn Finish_GetUrl(&self, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Begin_GetAccountState(&self) -> windows_core::Result<()>;
    fn Finish_GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl AsyncIConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_ConnectIdentity<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Begin_ConnectIdentity(this, core::mem::transmute_copy(&authbuffer), core::mem::transmute_copy(&authbuffersize)).into()
            }
        }
        unsafe extern "system" fn Finish_ConnectIdentity<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Finish_ConnectIdentity(this).into()
            }
        }
        unsafe extern "system" fn Begin_DisconnectIdentity<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Begin_DisconnectIdentity(this).into()
            }
        }
        unsafe extern "system" fn Finish_DisconnectIdentity<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Finish_DisconnectIdentity(this).into()
            }
        }
        unsafe extern "system" fn Begin_IsConnected<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Begin_IsConnected(this).into()
            }
        }
        unsafe extern "system" fn Finish_IsConnected<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIConnectedIdentityProvider_Impl::Finish_IsConnected(this) {
                    Ok(ok__) => {
                        connected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_GetUrl<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: IDENTITY_URL, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Begin_GetUrl(this, core::mem::transmute_copy(&identifier), core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn Finish_GetUrl<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Finish_GetUrl(this, core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&url)).into()
            }
        }
        unsafe extern "system" fn Begin_GetAccountState<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIConnectedIdentityProvider_Impl::Begin_GetAccountState(this).into()
            }
        }
        unsafe extern "system" fn Finish_GetAccountState<Identity: AsyncIConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIConnectedIdentityProvider_Impl::Finish_GetAccountState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for AsyncIConnectedIdentityProvider {}
windows_core::imp::define_interface!(AsyncIIdentityAdvise, AsyncIIdentityAdvise_Vtbl, 0x3ab4c8da_d038_4830_8dd9_3253c55a127f);
windows_core::imp::interface_hierarchy!(AsyncIIdentityAdvise, windows_core::IUnknown);
impl AsyncIIdentityAdvise {
    pub unsafe fn Begin_IdentityUpdated<P1>(&self, dwidentityupdateevents: u32, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_IdentityUpdated)(windows_core::Interface::as_raw(self), dwidentityupdateevents, lpszuniqueid.param().abi()).ok() }
    }
    pub unsafe fn Finish_IdentityUpdated(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_IdentityUpdated)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAdvise_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_IdentityUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Finish_IdentityUpdated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIIdentityAdvise_Impl: windows_core::IUnknownImpl {
    fn Begin_IdentityUpdated(&self, dwidentityupdateevents: u32, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_IdentityUpdated(&self) -> windows_core::Result<()>;
}
impl AsyncIIdentityAdvise_Vtbl {
    pub const fn new<Identity: AsyncIIdentityAdvise_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_IdentityUpdated<Identity: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAdvise_Impl::Begin_IdentityUpdated(this, core::mem::transmute_copy(&dwidentityupdateevents), core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        unsafe extern "system" fn Finish_IdentityUpdated<Identity: AsyncIIdentityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAdvise_Impl::Finish_IdentityUpdated(this).into()
            }
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
impl windows_core::RuntimeName for AsyncIIdentityAdvise {}
windows_core::imp::define_interface!(AsyncIIdentityAuthentication, AsyncIIdentityAuthentication_Vtbl, 0xf9a2f918_feca_4e9c_9633_61cbf13ed34d);
windows_core::imp::interface_hierarchy!(AsyncIIdentityAuthentication, windows_core::IUnknown);
impl AsyncIIdentityAuthentication {
    pub unsafe fn Begin_SetIdentityCredential(&self, credbuffer: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_SetIdentityCredential)(windows_core::Interface::as_raw(self), core::mem::transmute(credbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn Finish_SetIdentityCredential(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_SetIdentityCredential)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: Option<*mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_ValidateIdentityCredential)(windows_core::Interface::as_raw(self), core::mem::transmute(credbuffer.as_ptr()), credbuffer.len().try_into().unwrap(), ppidentityproperties.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: Option<*mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_ValidateIdentityCredential)(windows_core::Interface::as_raw(self), ppidentityproperties.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityAuthentication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_SetIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Finish_SetIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_ValidateIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_ValidateIdentityCredential: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_ValidateIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_ValidateIdentityCredential: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait AsyncIIdentityAuthentication_Impl: windows_core::IUnknownImpl {
    fn Begin_SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> windows_core::Result<()>;
    fn Finish_SetIdentityCredential(&self) -> windows_core::Result<()>;
    fn Begin_ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: windows_core::OutRef<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Finish_ValidateIdentityCredential(&self, ppidentityproperties: windows_core::OutRef<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AsyncIIdentityAuthentication_Vtbl {
    pub const fn new<Identity: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_SetIdentityCredential<Identity: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAuthentication_Impl::Begin_SetIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength)).into()
            }
        }
        unsafe extern "system" fn Finish_SetIdentityCredential<Identity: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAuthentication_Impl::Finish_SetIdentityCredential(this).into()
            }
        }
        unsafe extern "system" fn Begin_ValidateIdentityCredential<Identity: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAuthentication_Impl::Begin_ValidateIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength), core::mem::transmute_copy(&ppidentityproperties)).into()
            }
        }
        unsafe extern "system" fn Finish_ValidateIdentityCredential<Identity: AsyncIIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityAuthentication_Impl::Finish_ValidateIdentityCredential(this, core::mem::transmute_copy(&ppidentityproperties)).into()
            }
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for AsyncIIdentityAuthentication {}
windows_core::imp::define_interface!(AsyncIIdentityProvider, AsyncIIdentityProvider_Vtbl, 0xc6fc9901_c433_4646_8f48_4e4687aae2a0);
windows_core::imp::interface_hierarchy!(AsyncIIdentityProvider, windows_core::IUnknown);
impl AsyncIIdentityProvider {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: Option<*const super::super::super::super::Foundation::PROPERTYKEY>, pfilterpropvarvalue: Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetIdentityEnum)(windows_core::Interface::as_raw(self), eidentitytype, pfilterkey.unwrap_or(core::mem::zeroed()) as _, pfilterpropvarvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_GetIdentityEnum(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_GetIdentityEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_Create<P0>(&self, lpszusername: P0, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_Create)(windows_core::Interface::as_raw(self), lpszusername.param().abi(), core::mem::transmute(pkeywordstoadd)).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_Create(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_Create)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Begin_Import<P0>(&self, ppropertystore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_Import)(windows_core::Interface::as_raw(self), ppropertystore.param().abi()).ok() }
    }
    pub unsafe fn Finish_Import(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Import)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_Delete)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), core::mem::transmute(pkeywordstodelete)).ok() }
    }
    pub unsafe fn Finish_Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_FindByUniqueID)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_FindByUniqueID(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_FindByUniqueID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Begin_GetProviderPropertyStore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetProviderPropertyStore)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Finish_GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_GetProviderPropertyStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Begin_Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IIdentityAdvise>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_Advise)(windows_core::Interface::as_raw(self), pidentityadvise.param().abi(), dwidentityupdateevents).ok() }
    }
    pub unsafe fn Finish_Advise(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_Advise)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Begin_UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_UnAdvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn Finish_UnAdvise(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_UnAdvise)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_GetIdentityEnum: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_TYPE, *const super::super::super::super::Foundation::PROPERTYKEY, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_GetIdentityEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_GetIdentityEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_Import: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_Import: usize,
    pub Finish_Import: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_Delete: usize,
    pub Finish_Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_FindByUniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_FindByUniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_FindByUniqueID: usize,
    pub Begin_GetProviderPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_GetProviderPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_GetProviderPropertyStore: usize,
    pub Begin_Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Begin_UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait AsyncIIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn Begin_GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_GetIdentityEnum(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Create(&self, lpszusername: &windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_Create(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Import(&self, ppropertystore: windows_core::Ref<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Finish_Import(&self) -> windows_core::Result<()>;
    fn Begin_Delete(&self, lpszuniqueid: &windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_Delete(&self) -> windows_core::Result<()>;
    fn Begin_FindByUniqueID(&self, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Finish_FindByUniqueID(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_GetProviderPropertyStore(&self) -> windows_core::Result<()>;
    fn Finish_GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Begin_Advise(&self, pidentityadvise: windows_core::Ref<IIdentityAdvise>, dwidentityupdateevents: u32) -> windows_core::Result<()>;
    fn Finish_Advise(&self) -> windows_core::Result<u32>;
    fn Begin_UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn Finish_UnAdvise(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AsyncIIdentityProvider_Vtbl {
    pub const fn new<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_GetIdentityEnum<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_GetIdentityEnum(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)).into()
            }
        }
        unsafe extern "system" fn Finish_GetIdentityEnum<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityProvider_Impl::Finish_GetIdentityEnum(this) {
                    Ok(ok__) => {
                        ppidentityenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_Create<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: windows_core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_Create(this, core::mem::transmute(&lpszusername), core::mem::transmute_copy(&pkeywordstoadd)).into()
            }
        }
        unsafe extern "system" fn Finish_Create<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityProvider_Impl::Finish_Create(this) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_Import<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_Import(this, core::mem::transmute_copy(&ppropertystore)).into()
            }
        }
        unsafe extern "system" fn Finish_Import<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Finish_Import(this).into()
            }
        }
        unsafe extern "system" fn Begin_Delete<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_Delete(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&pkeywordstodelete)).into()
            }
        }
        unsafe extern "system" fn Finish_Delete<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Finish_Delete(this).into()
            }
        }
        unsafe extern "system" fn Begin_FindByUniqueID<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_FindByUniqueID(this, core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        unsafe extern "system" fn Finish_FindByUniqueID<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityProvider_Impl::Finish_FindByUniqueID(this) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_GetProviderPropertyStore<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_GetProviderPropertyStore(this).into()
            }
        }
        unsafe extern "system" fn Finish_GetProviderPropertyStore<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityProvider_Impl::Finish_GetProviderPropertyStore(this) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_Advise<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentityadvise: *mut core::ffi::c_void, dwidentityupdateevents: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_Advise(this, core::mem::transmute_copy(&pidentityadvise), core::mem::transmute_copy(&dwidentityupdateevents)).into()
            }
        }
        unsafe extern "system" fn Finish_Advise<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityProvider_Impl::Finish_Advise(this) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_UnAdvise<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Begin_UnAdvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn Finish_UnAdvise<Identity: AsyncIIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityProvider_Impl::Finish_UnAdvise(this).into()
            }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for AsyncIIdentityProvider {}
windows_core::imp::define_interface!(AsyncIIdentityStore, AsyncIIdentityStore_Vtbl, 0xeefa1616_48de_4872_aa64_6e6206535a51);
windows_core::imp::interface_hierarchy!(AsyncIIdentityStore, windows_core::IUnknown);
impl AsyncIIdentityStore {
    pub unsafe fn Begin_GetCount(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetCount)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Begin_GetAt(&self, dwprovider: u32, pprovguid: Option<*mut windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_GetAt)(windows_core::Interface::as_raw(self), dwprovider, pprovguid.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Finish_GetAt(&self, pprovguid: Option<*mut windows_core::GUID>, ppidentityprovider: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_GetAt)(windows_core::Interface::as_raw(self), pprovguid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppidentityprovider)).ok() }
    }
    pub unsafe fn Begin_AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_AddToCache)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), providerguid).ok() }
    }
    pub unsafe fn Finish_AddToCache(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_AddToCache)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const windows_core::GUID, cbsid: u16, psid: Option<*mut u8>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_ConvertToSid)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), providerguid, cbsid, psid.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Finish_ConvertToSid(&self, psid: Option<*mut u8>, pcbrequiredsid: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_ConvertToSid)(windows_core::Interface::as_raw(self), psid.unwrap_or(core::mem::zeroed()) as _, pcbrequiredsid as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: Option<*const super::super::super::super::Foundation::PROPERTYKEY>, pfilterpropvarvalue: Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_EnumerateIdentities)(windows_core::Interface::as_raw(self), eidentitytype, pfilterkey.unwrap_or(core::mem::zeroed()) as _, pfilterpropvarvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Finish_EnumerateIdentities(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Finish_EnumerateIdentities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Begin_Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Begin_GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_AddToCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_AddToCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_ConvertToSid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, u16, *mut u8) -> windows_core::HRESULT,
    pub Finish_ConvertToSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Begin_EnumerateIdentities: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_TYPE, *const super::super::super::super::Foundation::PROPERTYKEY, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Begin_EnumerateIdentities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_EnumerateIdentities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_EnumerateIdentities: usize,
    pub Begin_Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait AsyncIIdentityStore_Impl: windows_core::IUnknownImpl {
    fn Begin_GetCount(&self) -> windows_core::Result<()>;
    fn Finish_GetCount(&self) -> windows_core::Result<u32>;
    fn Begin_GetAt(&self, dwprovider: u32, pprovguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_GetAt(&self, pprovguid: *mut windows_core::GUID, ppidentityprovider: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Begin_AddToCache(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_AddToCache(&self) -> windows_core::Result<()>;
    fn Begin_ConvertToSid(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8) -> windows_core::Result<()>;
    fn Finish_ConvertToSid(&self, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::Result<()>;
    fn Begin_EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Finish_EnumerateIdentities(&self) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Begin_Reset(&self) -> windows_core::Result<()>;
    fn Finish_Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl AsyncIIdentityStore_Vtbl {
    pub const fn new<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_GetCount<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_GetCount(this).into()
            }
        }
        unsafe extern "system" fn Finish_GetCount<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproviders: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityStore_Impl::Finish_GetCount(this) {
                    Ok(ok__) => {
                        pdwproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_GetAt<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprovider: u32, pprovguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_GetAt(this, core::mem::transmute_copy(&dwprovider), core::mem::transmute_copy(&pprovguid)).into()
            }
        }
        unsafe extern "system" fn Finish_GetAt<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Finish_GetAt(this, core::mem::transmute_copy(&pprovguid), core::mem::transmute_copy(&ppidentityprovider)).into()
            }
        }
        unsafe extern "system" fn Begin_AddToCache<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_AddToCache(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid)).into()
            }
        }
        unsafe extern "system" fn Finish_AddToCache<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Finish_AddToCache(this).into()
            }
        }
        unsafe extern "system" fn Begin_ConvertToSid<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_ConvertToSid(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid), core::mem::transmute_copy(&cbsid), core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn Finish_ConvertToSid<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Finish_ConvertToSid(this, core::mem::transmute_copy(&psid), core::mem::transmute_copy(&pcbrequiredsid)).into()
            }
        }
        unsafe extern "system" fn Begin_EnumerateIdentities<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_EnumerateIdentities(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)).into()
            }
        }
        unsafe extern "system" fn Finish_EnumerateIdentities<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match AsyncIIdentityStore_Impl::Finish_EnumerateIdentities(this) {
                    Ok(ok__) => {
                        ppidentityenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Begin_Reset<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Begin_Reset(this).into()
            }
        }
        unsafe extern "system" fn Finish_Reset<Identity: AsyncIIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStore_Impl::Finish_Reset(this).into()
            }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for AsyncIIdentityStore {}
windows_core::imp::define_interface!(AsyncIIdentityStoreEx, AsyncIIdentityStoreEx_Vtbl, 0xfca3af9a_8a07_4eae_8632_ec3de658a36a);
windows_core::imp::interface_hierarchy!(AsyncIIdentityStoreEx, windows_core::IUnknown);
impl AsyncIIdentityStoreEx {
    pub unsafe fn Begin_CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_CreateConnectedIdentity)(windows_core::Interface::as_raw(self), localname.param().abi(), connectedname.param().abi(), providerguid).ok() }
    }
    pub unsafe fn Finish_CreateConnectedIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_CreateConnectedIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_DeleteConnectedIdentity)(windows_core::Interface::as_raw(self), connectedname.param().abi(), providerguid).ok() }
    }
    pub unsafe fn Finish_DeleteConnectedIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_DeleteConnectedIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIIdentityStoreEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_CreateConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_CreateConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_DeleteConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_DeleteConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIIdentityStoreEx_Impl: windows_core::IUnknownImpl {
    fn Begin_CreateConnectedIdentity(&self, localname: &windows_core::PCWSTR, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_CreateConnectedIdentity(&self) -> windows_core::Result<()>;
    fn Begin_DeleteConnectedIdentity(&self, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_DeleteConnectedIdentity(&self) -> windows_core::Result<()>;
}
impl AsyncIIdentityStoreEx_Vtbl {
    pub const fn new<Identity: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_CreateConnectedIdentity<Identity: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localname: windows_core::PCWSTR, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStoreEx_Impl::Begin_CreateConnectedIdentity(this, core::mem::transmute(&localname), core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
            }
        }
        unsafe extern "system" fn Finish_CreateConnectedIdentity<Identity: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStoreEx_Impl::Finish_CreateConnectedIdentity(this).into()
            }
        }
        unsafe extern "system" fn Begin_DeleteConnectedIdentity<Identity: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStoreEx_Impl::Begin_DeleteConnectedIdentity(this, core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
            }
        }
        unsafe extern "system" fn Finish_DeleteConnectedIdentity<Identity: AsyncIIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIIdentityStoreEx_Impl::Finish_DeleteConnectedIdentity(this).into()
            }
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
impl windows_core::RuntimeName for AsyncIIdentityStoreEx {}
pub const CIdentityProfileHandler: windows_core::GUID = windows_core::GUID::from_u128(0xecf5bf46_e3b6_449a_b56b_43f58f867814);
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
pub const CoClassIdentityStore: windows_core::GUID = windows_core::GUID::from_u128(0x30d49246_d217_465f_b00b_ac9ddd652eb7);
windows_core::imp::define_interface!(IAssociatedIdentityProvider, IAssociatedIdentityProvider_Vtbl, 0x2af066b3_4cbb_4cba_a798_204b6af68cc0);
windows_core::imp::interface_hierarchy!(IAssociatedIdentityProvider, windows_core::IUnknown);
impl IAssociatedIdentityProvider {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociateIdentity)(windows_core::Interface::as_raw(self), hwndparent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DisassociateIdentity<P1>(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DisassociateIdentity)(windows_core::Interface::as_raw(self), hwndparent, lpszuniqueid.param().abi()).ok() }
    }
    pub unsafe fn ChangeCredential<P1>(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChangeCredential)(windows_core::Interface::as_raw(self), hwndparent, lpszuniqueid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssociatedIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub AssociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    AssociateIdentity: usize,
    pub DisassociateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ChangeCredential: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAssociatedIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn AssociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn DisassociateIdentity(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ChangeCredential(&self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAssociatedIdentityProvider_Vtbl {
    pub const fn new<Identity: IAssociatedIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssociateIdentity<Identity: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAssociatedIdentityProvider_Impl::AssociateIdentity(this, core::mem::transmute_copy(&hwndparent)) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisassociateIdentity<Identity: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssociatedIdentityProvider_Impl::DisassociateIdentity(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        unsafe extern "system" fn ChangeCredential<Identity: IAssociatedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssociatedIdentityProvider_Impl::ChangeCredential(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&lpszuniqueid)).into()
            }
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAssociatedIdentityProvider {}
windows_core::imp::define_interface!(IConnectedIdentityProvider, IConnectedIdentityProvider_Vtbl, 0xb7417b54_e08c_429b_96c8_678d1369ecb1);
windows_core::imp::interface_hierarchy!(IConnectedIdentityProvider, windows_core::IUnknown);
impl IConnectedIdentityProvider {
    pub unsafe fn ConnectIdentity(&self, authbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectIdentity)(windows_core::Interface::as_raw(self), core::mem::transmute(authbuffer.as_ptr()), authbuffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn DisconnectIdentity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectIdentity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetUrl<P1>(&self, identifier: IDENTITY_URL, context: P1, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::super::System::Com::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetUrl)(windows_core::Interface::as_raw(self), identifier, context.param().abi(), core::mem::transmute(postdata), url as _).ok() }
    }
    pub unsafe fn GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccountState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub DisconnectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetUrl: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_URL, *mut core::ffi::c_void, *mut super::super::super::super::System::Variant::VARIANT, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetUrl: usize,
    pub GetAccountState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ACCOUNT_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IConnectedIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn ConnectIdentity(&self, authbuffer: *const u8, authbuffersize: u32) -> windows_core::Result<()>;
    fn DisconnectIdentity(&self) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetUrl(&self, identifier: IDENTITY_URL, context: windows_core::Ref<super::super::super::super::System::Com::IBindCtx>, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetAccountState(&self) -> windows_core::Result<ACCOUNT_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IConnectedIdentityProvider_Vtbl {
    pub const fn new<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectIdentity<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authbuffer: *const u8, authbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectedIdentityProvider_Impl::ConnectIdentity(this, core::mem::transmute_copy(&authbuffer), core::mem::transmute_copy(&authbuffersize)).into()
            }
        }
        unsafe extern "system" fn DisconnectIdentity<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectedIdentityProvider_Impl::DisconnectIdentity(this).into()
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectedIdentityProvider_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        connected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUrl<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identifier: IDENTITY_URL, context: *mut core::ffi::c_void, postdata: *mut super::super::super::super::System::Variant::VARIANT, url: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectedIdentityProvider_Impl::GetUrl(this, core::mem::transmute_copy(&identifier), core::mem::transmute_copy(&context), core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&url)).into()
            }
        }
        unsafe extern "system" fn GetAccountState<Identity: IConnectedIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut ACCOUNT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectedIdentityProvider_Impl::GetAccountState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IConnectedIdentityProvider {}
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1i32);
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64i32);
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4i32);
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16i32);
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2i32);
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128i32);
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8i32);
pub const IDENTITY_KEYWORD_ASSOCIATED: windows_core::PCWSTR = windows_core::w!("associated");
pub const IDENTITY_KEYWORD_CONNECTED: windows_core::PCWSTR = windows_core::w!("connected");
pub const IDENTITY_KEYWORD_HOMEGROUP: windows_core::PCWSTR = windows_core::w!("homegroup");
pub const IDENTITY_KEYWORD_LOCAL: windows_core::PCWSTR = windows_core::w!("local");
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDENTITY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDENTITY_URL(pub i32);
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
windows_core::imp::define_interface!(IIdentityAdvise, IIdentityAdvise_Vtbl, 0x4e982fed_d14b_440c_b8d6_bb386453d386);
windows_core::imp::interface_hierarchy!(IIdentityAdvise, windows_core::IUnknown);
impl IIdentityAdvise {
    pub unsafe fn IdentityUpdated<P1>(&self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IdentityUpdated)(windows_core::Interface::as_raw(self), dwidentityupdateevents.0 as _, lpszuniqueid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAdvise_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IdentityUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IIdentityAdvise_Impl: windows_core::IUnknownImpl {
    fn IdentityUpdated(&self, dwidentityupdateevents: &IdentityUpdateEvent, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IIdentityAdvise_Vtbl {
    pub const fn new<Identity: IIdentityAdvise_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IdentityUpdated<Identity: IIdentityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwidentityupdateevents: u32, lpszuniqueid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityAdvise_Impl::IdentityUpdated(this, core::mem::transmute(&dwidentityupdateevents), core::mem::transmute(&lpszuniqueid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IdentityUpdated: IdentityUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdentityAdvise as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IIdentityAdvise {}
windows_core::imp::define_interface!(IIdentityAuthentication, IIdentityAuthentication_Vtbl, 0x5e7ef254_979f_43b5_b74e_06e4eb7df0f9);
windows_core::imp::interface_hierarchy!(IIdentityAuthentication, windows_core::IUnknown);
impl IIdentityAuthentication {
    pub unsafe fn SetIdentityCredential(&self, credbuffer: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIdentityCredential)(windows_core::Interface::as_raw(self), core::mem::transmute(credbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), credbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ValidateIdentityCredential(&self, credbuffer: &[u8], ppidentityproperties: Option<*mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ValidateIdentityCredential)(windows_core::Interface::as_raw(self), core::mem::transmute(credbuffer.as_ptr()), credbuffer.len().try_into().unwrap(), ppidentityproperties.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityAuthentication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ValidateIdentityCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ValidateIdentityCredential: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IIdentityAuthentication_Impl: windows_core::IUnknownImpl {
    fn SetIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32) -> windows_core::Result<()>;
    fn ValidateIdentityCredential(&self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: windows_core::OutRef<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IIdentityAuthentication_Vtbl {
    pub const fn new<Identity: IIdentityAuthentication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIdentityCredential<Identity: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityAuthentication_Impl::SetIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength)).into()
            }
        }
        unsafe extern "system" fn ValidateIdentityCredential<Identity: IIdentityAuthentication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityAuthentication_Impl::ValidateIdentityCredential(this, core::mem::transmute_copy(&credbuffer), core::mem::transmute_copy(&credbufferlength), core::mem::transmute_copy(&ppidentityproperties)).into()
            }
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IIdentityAuthentication {}
windows_core::imp::define_interface!(IIdentityProvider, IIdentityProvider_Vtbl, 0x0d1b9e0c_e8ba_4f55_a81b_bce934b948f5);
windows_core::imp::interface_hierarchy!(IIdentityProvider, windows_core::IUnknown);
impl IIdentityProvider {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: Option<*const super::super::super::super::Foundation::PROPERTYKEY>, pfilterpropvarvalue: Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIdentityEnum)(windows_core::Interface::as_raw(self), eidentitytype, pfilterkey.unwrap_or(core::mem::zeroed()) as _, pfilterpropvarvalue.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Create<P0>(&self, lpszusername: P0, pppropertystore: *mut Option<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), lpszusername.param().abi(), core::mem::transmute(pppropertystore), core::mem::transmute(pkeywordstoadd)).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Import<P0>(&self, ppropertystore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), ppropertystore.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Delete<P0>(&self, lpszuniqueid: P0, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), core::mem::transmute(pkeywordstodelete)).ok() }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn FindByUniqueID<P0>(&self, lpszuniqueid: P0) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindByUniqueID)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderPropertyStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Advise<P0>(&self, pidentityadvise: P0, dwidentityupdateevents: IdentityUpdateEvent) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IIdentityAdvise>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pidentityadvise.param().abi(), dwidentityupdateevents.0 as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetIdentityEnum: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_TYPE, *const super::super::super::super::Foundation::PROPERTYKEY, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Import: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Delete: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub FindByUniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    FindByUniqueID: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProviderPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProviderPropertyStore: usize,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IIdentityProvider_Impl: windows_core::IUnknownImpl {
    fn GetIdentityEnum(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Create(&self, lpszusername: &windows_core::PCWSTR, pppropertystore: windows_core::OutRef<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Import(&self, ppropertystore: windows_core::Ref<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn Delete(&self, lpszuniqueid: &windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn FindByUniqueID(&self, lpszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetProviderPropertyStore(&self) -> windows_core::Result<super::super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Advise(&self, pidentityadvise: windows_core::Ref<IIdentityAdvise>, dwidentityupdateevents: &IdentityUpdateEvent) -> windows_core::Result<u32>;
    fn UnAdvise(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IIdentityProvider_Vtbl {
    pub const fn new<Identity: IIdentityProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentityEnum<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityProvider_Impl::GetIdentityEnum(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)) {
                    Ok(ok__) => {
                        ppidentityenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: windows_core::PCWSTR, pppropertystore: *mut *mut core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityProvider_Impl::Create(this, core::mem::transmute(&lpszusername), core::mem::transmute_copy(&pppropertystore), core::mem::transmute_copy(&pkeywordstoadd)).into()
            }
        }
        unsafe extern "system" fn Import<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityProvider_Impl::Import(this, core::mem::transmute_copy(&ppropertystore)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityProvider_Impl::Delete(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&pkeywordstodelete)).into()
            }
        }
        unsafe extern "system" fn FindByUniqueID<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityProvider_Impl::FindByUniqueID(this, core::mem::transmute(&lpszuniqueid)) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderPropertyStore<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityProvider_Impl::GetProviderPropertyStore(this) {
                    Ok(ok__) => {
                        pppropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentityadvise: *mut core::ffi::c_void, dwidentityupdateevents: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityProvider_Impl::Advise(this, core::mem::transmute_copy(&pidentityadvise), core::mem::transmute(&dwidentityupdateevents)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: IIdentityProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityProvider_Impl::UnAdvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IIdentityProvider {}
windows_core::imp::define_interface!(IIdentityStore, IIdentityStore_Vtbl, 0xdf586fa5_6f35_44f1_b209_b38e169772eb);
windows_core::imp::interface_hierarchy!(IIdentityStore, windows_core::IUnknown);
impl IIdentityStore {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, dwprovider: u32, pprovguid: Option<*mut windows_core::GUID>, ppidentityprovider: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwprovider, pprovguid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppidentityprovider)).ok() }
    }
    pub unsafe fn AddToCache<P0>(&self, lpszuniqueid: P0, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddToCache)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), providerguid).ok() }
    }
    pub unsafe fn ConvertToSid<P0>(&self, lpszuniqueid: P0, providerguid: *const windows_core::GUID, psid: Option<&mut [u8]>, pcbrequiredsid: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertToSid)(windows_core::Interface::as_raw(self), lpszuniqueid.param().abi(), providerguid, psid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(psid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcbrequiredsid as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: Option<*const super::super::super::super::Foundation::PROPERTYKEY>, pfilterpropvarvalue: Option<*const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateIdentities)(windows_core::Interface::as_raw(self), eidentitytype, pfilterkey.unwrap_or(core::mem::zeroed()) as _, pfilterpropvarvalue.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub ConvertToSid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, u16, *mut u8, *mut u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub EnumerateIdentities: unsafe extern "system" fn(*mut core::ffi::c_void, IDENTITY_TYPE, *const super::super::super::super::Foundation::PROPERTYKEY, *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    EnumerateIdentities: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IIdentityStore_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, dwprovider: u32, pprovguid: *mut windows_core::GUID, ppidentityprovider: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddToCache(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ConvertToSid(&self, lpszuniqueid: &windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::Result<()>;
    fn EnumerateIdentities(&self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<super::super::super::super::System::Com::IEnumUnknown>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IIdentityStore_Vtbl {
    pub const fn new<Identity: IIdentityStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproviders: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityStore_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprovider: u32, pprovguid: *mut windows_core::GUID, ppidentityprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStore_Impl::GetAt(this, core::mem::transmute_copy(&dwprovider), core::mem::transmute_copy(&pprovguid), core::mem::transmute_copy(&ppidentityprovider)).into()
            }
        }
        unsafe extern "system" fn AddToCache<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStore_Impl::AddToCache(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid)).into()
            }
        }
        unsafe extern "system" fn ConvertToSid<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszuniqueid: windows_core::PCWSTR, providerguid: *const windows_core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStore_Impl::ConvertToSid(this, core::mem::transmute(&lpszuniqueid), core::mem::transmute_copy(&providerguid), core::mem::transmute_copy(&cbsid), core::mem::transmute_copy(&psid), core::mem::transmute_copy(&pcbrequiredsid)).into()
            }
        }
        unsafe extern "system" fn EnumerateIdentities<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::Foundation::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdentityStore_Impl::EnumerateIdentities(this, core::mem::transmute_copy(&eidentitytype), core::mem::transmute_copy(&pfilterkey), core::mem::transmute_copy(&pfilterpropvarvalue)) {
                    Ok(ok__) => {
                        ppidentityenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IIdentityStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStore_Impl::Reset(this).into()
            }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIdentityStore {}
windows_core::imp::define_interface!(IIdentityStoreEx, IIdentityStoreEx_Vtbl, 0xf9f9eb98_8f7f_4e38_9577_6980114ce32b);
windows_core::imp::interface_hierarchy!(IIdentityStoreEx, windows_core::IUnknown);
impl IIdentityStoreEx {
    pub unsafe fn CreateConnectedIdentity<P0, P1>(&self, localname: P0, connectedname: P1, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateConnectedIdentity)(windows_core::Interface::as_raw(self), localname.param().abi(), connectedname.param().abi(), providerguid).ok() }
    }
    pub unsafe fn DeleteConnectedIdentity<P0>(&self, connectedname: P0, providerguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteConnectedIdentity)(windows_core::Interface::as_raw(self), connectedname.param().abi(), providerguid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdentityStoreEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteConnectedIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IIdentityStoreEx_Impl: windows_core::IUnknownImpl {
    fn CreateConnectedIdentity(&self, localname: &windows_core::PCWSTR, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteConnectedIdentity(&self, connectedname: &windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IIdentityStoreEx_Vtbl {
    pub const fn new<Identity: IIdentityStoreEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateConnectedIdentity<Identity: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localname: windows_core::PCWSTR, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStoreEx_Impl::CreateConnectedIdentity(this, core::mem::transmute(&localname), core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
            }
        }
        unsafe extern "system" fn DeleteConnectedIdentity<Identity: IIdentityStoreEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectedname: windows_core::PCWSTR, providerguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdentityStoreEx_Impl::DeleteConnectedIdentity(this, core::mem::transmute(&connectedname), core::mem::transmute_copy(&providerguid)).into()
            }
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
impl windows_core::RuntimeName for IIdentityStoreEx {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdentityUpdateEvent(pub i32);
impl IdentityUpdateEvent {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IdentityUpdateEvent {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IdentityUpdateEvent {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IdentityUpdateEvent {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IdentityUpdateEvent {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IdentityUpdateEvent {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
pub const OID_OAssociatedIdentityProviderObject: windows_core::GUID = windows_core::GUID::from_u128(0x98c5a3dd_db68_4f1a_8d2b_9079cdfeaf61);
pub const STR_COMPLETE_ACCOUNT: windows_core::PCWSTR = windows_core::w!("CompleteAccount");
pub const STR_MODERN_SETTINGS_ADD_USER: windows_core::PCWSTR = windows_core::w!("ModernSettingsAddUser");
pub const STR_NTH_USER_FIRST_AUTH: windows_core::PCWSTR = windows_core::w!("NthUserFirstAuth");
pub const STR_OUT_OF_BOX_EXPERIENCE: windows_core::PCWSTR = windows_core::w!("OutOfBoxExperience");
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: windows_core::PCWSTR = windows_core::w!("OutOfBoxUpgradeExperience");
pub const STR_PROPERTY_STORE: windows_core::PCWSTR = windows_core::w!("PropertyStore");
pub const STR_USER_NAME: windows_core::PCWSTR = windows_core::w!("Username");
