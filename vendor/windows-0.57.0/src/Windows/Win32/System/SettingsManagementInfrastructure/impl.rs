pub trait IItemEnumerator_Impl: Sized {
    fn Current(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IItemEnumerator {}
impl IItemEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>() -> IItemEnumerator_Vtbl {
        unsafe extern "system" fn Current<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IItemEnumerator_Impl::Current(this) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IItemEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    core::ptr::write(itemvalid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IItemEnumerator_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Current: Current::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IItemEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISettingsContext_Impl: Sized {
    fn Serialize(&self, pstream: Option<&super::Com::IStream>, ptarget: Option<&ITargetInfo>) -> windows_core::Result<()>;
    fn Deserialize(&self, pstream: Option<&super::Com::IStream>, ptarget: Option<&ITargetInfo>, pppresults: *mut *mut Option<ISettingsResult>) -> windows_core::Result<usize>;
    fn SetUserData(&self, puserdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetUserData(&self) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetNamespaces(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetStoredSettings(&self, pidentity: Option<&ISettingsIdentity>, ppaddedsettings: *mut Option<IItemEnumerator>, ppmodifiedsettings: *mut Option<IItemEnumerator>, ppdeletedsettings: *mut Option<IItemEnumerator>) -> windows_core::Result<()>;
    fn RevertSetting(&self, pidentity: Option<&ISettingsIdentity>, pwzsetting: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISettingsContext {}
#[cfg(feature = "Win32_System_Com")]
impl ISettingsContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>() -> ISettingsContext_Vtbl {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsContext_Impl::Serialize(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, pppresults: *mut *mut Option<ISettingsResult>, pcresultcount: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsContext_Impl::Deserialize(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ptarget), core::mem::transmute_copy(&pppresults)) {
                Ok(ok__) => {
                    core::ptr::write(pcresultcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsContext_Impl::SetUserData(this, core::mem::transmute_copy(&puserdata)).into()
        }
        unsafe extern "system" fn GetUserData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsContext_Impl::GetUserData(this) {
                Ok(ok__) => {
                    core::ptr::write(puserdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamespaceids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsContext_Impl::GetNamespaces(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnamespaceids, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoredSettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, ppaddedsettings: *mut *mut core::ffi::c_void, ppmodifiedsettings: *mut *mut core::ffi::c_void, ppdeletedsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsContext_Impl::GetStoredSettings(this, windows_core::from_raw_borrowed(&pidentity), core::mem::transmute_copy(&ppaddedsettings), core::mem::transmute_copy(&ppmodifiedsettings), core::mem::transmute_copy(&ppdeletedsettings)).into()
        }
        unsafe extern "system" fn RevertSetting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, pwzsetting: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsContext_Impl::RevertSetting(this, windows_core::from_raw_borrowed(&pidentity), core::mem::transmute(&pwzsetting)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
            SetUserData: SetUserData::<Identity, Impl, OFFSET>,
            GetUserData: GetUserData::<Identity, Impl, OFFSET>,
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetStoredSettings: GetStoredSettings::<Identity, Impl, OFFSET>,
            RevertSetting: RevertSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISettingsEngine_Impl: Sized {
    fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void) -> windows_core::Result<IItemEnumerator>;
    fn GetNamespace(&self, settingsid: Option<&ISettingsIdentity>, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsNamespace>;
    fn GetErrorDescription(&self, hresult: i32) -> windows_core::Result<windows_core::BSTR>;
    fn CreateSettingsIdentity(&self) -> windows_core::Result<ISettingsIdentity>;
    fn GetStoreStatus(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<WcmUserStatus>;
    fn LoadStore(&self, flags: u32) -> windows_core::Result<()>;
    fn UnloadStore(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn RegisterNamespace(&self, settingsid: Option<&ISettingsIdentity>, stream: Option<&super::Com::IStream>, pushsettings: super::super::Foundation::BOOL) -> windows_core::Result<windows_core::VARIANT>;
    fn UnregisterNamespace(&self, settingsid: Option<&ISettingsIdentity>, removesettings: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateTargetInfo(&self) -> windows_core::Result<ITargetInfo>;
    fn GetTargetInfo(&self) -> windows_core::Result<ITargetInfo>;
    fn SetTargetInfo(&self, target: Option<&ITargetInfo>) -> windows_core::Result<()>;
    fn CreateSettingsContext(&self, flags: u32, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsContext>;
    fn SetSettingsContext(&self, settingscontext: Option<&ISettingsContext>) -> windows_core::Result<()>;
    fn ApplySettingsContext(&self, settingscontext: Option<&ISettingsContext>, pppwzidentities: *mut *mut windows_core::PWSTR) -> windows_core::Result<usize>;
    fn GetSettingsContext(&self) -> windows_core::Result<ISettingsContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISettingsEngine {}
#[cfg(feature = "Win32_System_Com")]
impl ISettingsEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>() -> ISettingsEngine_Vtbl {
        unsafe extern "system" fn GetNamespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void, namespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetNamespaces(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(namespaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void, namespaceitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetNamespace(this, windows_core::from_raw_borrowed(&settingsid), core::mem::transmute_copy(&access), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(namespaceitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: i32, message: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult)) {
                Ok(ok__) => {
                    core::ptr::write(message, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingsIdentity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::CreateSettingsIdentity(this) {
                Ok(ok__) => {
                    core::ptr::write(settingsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, status: *mut WcmUserStatus) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetStoreStatus(this, core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsEngine_Impl::LoadStore(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnloadStore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsEngine_Impl::UnloadStore(this, core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn RegisterNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::RegisterNamespace(this, windows_core::from_raw_borrowed(&settingsid), windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&pushsettings)) {
                Ok(ok__) => {
                    core::ptr::write(results, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsEngine_Impl::UnregisterNamespace(this, windows_core::from_raw_borrowed(&settingsid), core::mem::transmute_copy(&removesettings)).into()
        }
        unsafe extern "system" fn CreateTargetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::CreateTargetInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(target, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetTargetInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(target, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsEngine_Impl::SetTargetInfo(this, windows_core::from_raw_borrowed(&target)).into()
        }
        unsafe extern "system" fn CreateSettingsContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, reserved: *const core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::CreateSettingsContext(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    core::ptr::write(settingscontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsEngine_Impl::SetSettingsContext(this, windows_core::from_raw_borrowed(&settingscontext)).into()
        }
        unsafe extern "system" fn ApplySettingsContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void, pppwzidentities: *mut *mut windows_core::PWSTR, pcidentities: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::ApplySettingsContext(this, windows_core::from_raw_borrowed(&settingscontext), core::mem::transmute_copy(&pppwzidentities)) {
                Ok(ok__) => {
                    core::ptr::write(pcidentities, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingsContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsEngine_Impl::GetSettingsContext(this) {
                Ok(ok__) => {
                    core::ptr::write(settingscontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, Impl, OFFSET>,
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            CreateSettingsIdentity: CreateSettingsIdentity::<Identity, Impl, OFFSET>,
            GetStoreStatus: GetStoreStatus::<Identity, Impl, OFFSET>,
            LoadStore: LoadStore::<Identity, Impl, OFFSET>,
            UnloadStore: UnloadStore::<Identity, Impl, OFFSET>,
            RegisterNamespace: RegisterNamespace::<Identity, Impl, OFFSET>,
            UnregisterNamespace: UnregisterNamespace::<Identity, Impl, OFFSET>,
            CreateTargetInfo: CreateTargetInfo::<Identity, Impl, OFFSET>,
            GetTargetInfo: GetTargetInfo::<Identity, Impl, OFFSET>,
            SetTargetInfo: SetTargetInfo::<Identity, Impl, OFFSET>,
            CreateSettingsContext: CreateSettingsContext::<Identity, Impl, OFFSET>,
            SetSettingsContext: SetSettingsContext::<Identity, Impl, OFFSET>,
            ApplySettingsContext: ApplySettingsContext::<Identity, Impl, OFFSET>,
            GetSettingsContext: GetSettingsContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsEngine as windows_core::Interface>::IID
    }
}
pub trait ISettingsIdentity_Impl: Sized {
    fn GetAttribute(&self, reserved: *const core::ffi::c_void, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetAttribute(&self, reserved: *const core::ffi::c_void, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn SetFlags(&self, flags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISettingsIdentity {}
impl ISettingsIdentity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>() -> ISettingsIdentity_Vtbl {
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsIdentity_Impl::GetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsIdentity_Impl::SetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsIdentity_Impl::GetFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(flags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsIdentity_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            SetAttribute: SetAttribute::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsIdentity as windows_core::Interface>::IID
    }
}
pub trait ISettingsItem_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, value: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetSettingType(&self) -> windows_core::Result<WcmSettingType>;
    fn GetDataType(&self) -> windows_core::Result<WcmDataType>;
    fn GetValueRaw(&self, data: *mut *mut u8) -> windows_core::Result<u32>;
    fn SetValueRaw(&self, datatype: i32, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn HasChild(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Children(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetChild(&self, name: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn GetSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetListKeyInformation(&self, keyname: *mut windows_core::BSTR) -> windows_core::Result<WcmDataType>;
    fn CreateListElement(&self, keydata: *const windows_core::VARIANT) -> windows_core::Result<ISettingsItem>;
    fn RemoveListElement(&self, elementname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Attributes(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRestrictionFacets(&self) -> windows_core::Result<WcmRestrictionFacets>;
    fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> windows_core::Result<windows_core::VARIANT>;
    fn GetKeyValue(&self) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ISettingsItem {}
impl ISettingsItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>() -> ISettingsItem_Vtbl {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsItem_Impl::SetValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSettingType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmSettingType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetSettingType(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmDataType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetDataType(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueRaw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetValueRaw(this, core::mem::transmute_copy(&data)) {
                Ok(ok__) => {
                    core::ptr::write(datasize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueRaw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsItem_Impl::SetValueRaw(this, core::mem::transmute_copy(&datatype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn HasChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::HasChild(this) {
                Ok(ok__) => {
                    core::ptr::write(itemhaschild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::Children(this) {
                Ok(ok__) => {
                    core::ptr::write(children, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetChild(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(child, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(setting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(setting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsItem_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetListKeyInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keyname: *mut core::mem::MaybeUninit<windows_core::BSTR>, datatype: *mut WcmDataType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetListKeyInformation(this, core::mem::transmute_copy(&keyname)) {
                Ok(ok__) => {
                    core::ptr::write(datatype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateListElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keydata: *const core::mem::MaybeUninit<windows_core::VARIANT>, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::CreateListElement(this, core::mem::transmute_copy(&keydata)) {
                Ok(ok__) => {
                    core::ptr::write(child, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsItem_Impl::RemoveListElement(this, core::mem::transmute(&elementname)).into()
        }
        unsafe extern "system" fn Attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::Attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetPath(this) {
                Ok(ok__) => {
                    core::ptr::write(path, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictionFacets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetRestrictionFacets(this) {
                Ok(ok__) => {
                    core::ptr::write(restrictionfacets, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestriction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetRestriction(this, core::mem::transmute_copy(&restrictionfacet)) {
                Ok(ok__) => {
                    core::ptr::write(facetdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsItem_Impl::GetKeyValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetSettingType: GetSettingType::<Identity, Impl, OFFSET>,
            GetDataType: GetDataType::<Identity, Impl, OFFSET>,
            GetValueRaw: GetValueRaw::<Identity, Impl, OFFSET>,
            SetValueRaw: SetValueRaw::<Identity, Impl, OFFSET>,
            HasChild: HasChild::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetListKeyInformation: GetListKeyInformation::<Identity, Impl, OFFSET>,
            CreateListElement: CreateListElement::<Identity, Impl, OFFSET>,
            RemoveListElement: RemoveListElement::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRestrictionFacets: GetRestrictionFacets::<Identity, Impl, OFFSET>,
            GetRestriction: GetRestriction::<Identity, Impl, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsItem as windows_core::Interface>::IID
    }
}
pub trait ISettingsNamespace_Impl: Sized {
    fn GetIdentity(&self) -> windows_core::Result<ISettingsIdentity>;
    fn Settings(&self) -> windows_core::Result<IItemEnumerator>;
    fn Save(&self, pushsettings: super::super::Foundation::BOOL) -> windows_core::Result<ISettingsResult>;
    fn GetSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ISettingsNamespace {}
impl ISettingsNamespace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>() -> ISettingsNamespace_Vtbl {
        unsafe extern "system" fn GetIdentity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::GetIdentity(this) {
                Ok(ok__) => {
                    core::ptr::write(settingsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::Settings(this) {
                Ok(ok__) => {
                    core::ptr::write(settings, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::Save(this, core::mem::transmute_copy(&pushsettings)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(setting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    core::ptr::write(setting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISettingsNamespace_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsNamespace_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdentity: GetIdentity::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, Impl, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, Impl, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsNamespace as windows_core::Interface>::IID
    }
}
pub trait ISettingsResult_Impl: Sized {
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLine(&self) -> windows_core::Result<u32>;
    fn GetColumn(&self) -> windows_core::Result<u32>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ISettingsResult {}
impl ISettingsResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>() -> ISettingsResult_Vtbl {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(description, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrout: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetErrorCode(this) {
                Ok(ok__) => {
                    core::ptr::write(hrout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetContextDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(description, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwline: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetLine(this) {
                Ok(ok__) => {
                    core::ptr::write(dwline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcolumn: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetColumn(this) {
                Ok(ok__) => {
                    core::ptr::write(dwcolumn, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISettingsResult_Impl::GetSource(this) {
                Ok(ok__) => {
                    core::ptr::write(file, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, Impl, OFFSET>,
            GetLine: GetLine::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISettingsResult as windows_core::Interface>::IID
    }
}
pub trait ITargetInfo_Impl: Sized {
    fn GetTargetMode(&self) -> windows_core::Result<WcmTargetMode>;
    fn SetTargetMode(&self, targetmode: WcmTargetMode) -> windows_core::Result<()>;
    fn GetTemporaryStoreLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTemporaryStoreLocation(&self, temporarystorelocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTargetID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetID(&self, targetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetTargetProcessorArchitecture(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetProcessorArchitecture(&self, processorarchitecture: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProperty(&self, offline: super::super::Foundation::BOOL, property: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetProperty(&self, offline: super::super::Foundation::BOOL, property: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IItemEnumerator>;
    fn ExpandTarget(&self, offline: super::super::Foundation::BOOL, location: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn ExpandTargetPath(&self, offline: super::super::Foundation::BOOL, location: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetModulePath(&self, module: &windows_core::PCWSTR, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LoadModule(&self, module: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn SetWow64Context(&self, installermodule: &windows_core::PCWSTR, wow64context: *const u8) -> windows_core::Result<()>;
    fn TranslateWow64(&self, clientarchitecture: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetSchemaHiveLocation(&self, pwzhivedir: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemaHiveLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSchemaHiveMountName(&self, pwzmountname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemaHiveMountName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITargetInfo {}
impl ITargetInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>() -> ITargetInfo_Vtbl {
        unsafe extern "system" fn GetTargetMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: *mut WcmTargetMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetTargetMode(this) {
                Ok(ok__) => {
                    core::ptr::write(targetmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: WcmTargetMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetTargetMode(this, core::mem::transmute_copy(&targetmode)).into()
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetTemporaryStoreLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(temporarystorelocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetTemporaryStoreLocation(this, core::mem::transmute(&temporarystorelocation)).into()
        }
        unsafe extern "system" fn GetTargetID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetTargetID(this) {
                Ok(ok__) => {
                    core::ptr::write(targetid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetTargetID(this, core::mem::transmute(&targetid)).into()
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetTargetProcessorArchitecture(this) {
                Ok(ok__) => {
                    core::ptr::write(processorarchitecture, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetTargetProcessorArchitecture(this, core::mem::transmute(&processorarchitecture)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, property: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, property: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    core::ptr::write(enumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTarget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::ExpandTarget(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                Ok(ok__) => {
                    core::ptr::write(expandedlocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTargetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::ExpandTargetPath(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                Ok(ok__) => {
                    core::ptr::write(expandedlocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModulePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetModulePath(this, core::mem::transmute(&module), core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn LoadModule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::LoadModule(this, core::mem::transmute(&module)) {
                Ok(ok__) => {
                    core::ptr::write(modulehandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWow64Context<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installermodule: windows_core::PCWSTR, wow64context: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetWow64Context(this, core::mem::transmute(&installermodule), core::mem::transmute_copy(&wow64context)).into()
        }
        unsafe extern "system" fn TranslateWow64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientarchitecture: windows_core::PCWSTR, value: windows_core::PCWSTR, translatedvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::TranslateWow64(this, core::mem::transmute(&clientarchitecture), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    core::ptr::write(translatedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzhivedir: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetSchemaHiveLocation(this, core::mem::transmute(&pwzhivedir)).into()
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phivelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetSchemaHiveLocation(this) {
                Ok(ok__) => {
                    core::ptr::write(phivelocation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzmountname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITargetInfo_Impl::SetSchemaHiveMountName(this, core::mem::transmute(&pwzmountname)).into()
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmountname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITargetInfo_Impl::GetSchemaHiveMountName(this) {
                Ok(ok__) => {
                    core::ptr::write(pmountname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTargetMode: GetTargetMode::<Identity, Impl, OFFSET>,
            SetTargetMode: SetTargetMode::<Identity, Impl, OFFSET>,
            GetTemporaryStoreLocation: GetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            SetTemporaryStoreLocation: SetTemporaryStoreLocation::<Identity, Impl, OFFSET>,
            GetTargetID: GetTargetID::<Identity, Impl, OFFSET>,
            SetTargetID: SetTargetID::<Identity, Impl, OFFSET>,
            GetTargetProcessorArchitecture: GetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            SetTargetProcessorArchitecture: SetTargetProcessorArchitecture::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
            ExpandTarget: ExpandTarget::<Identity, Impl, OFFSET>,
            ExpandTargetPath: ExpandTargetPath::<Identity, Impl, OFFSET>,
            SetModulePath: SetModulePath::<Identity, Impl, OFFSET>,
            LoadModule: LoadModule::<Identity, Impl, OFFSET>,
            SetWow64Context: SetWow64Context::<Identity, Impl, OFFSET>,
            TranslateWow64: TranslateWow64::<Identity, Impl, OFFSET>,
            SetSchemaHiveLocation: SetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            GetSchemaHiveLocation: GetSchemaHiveLocation::<Identity, Impl, OFFSET>,
            SetSchemaHiveMountName: SetSchemaHiveMountName::<Identity, Impl, OFFSET>,
            GetSchemaHiveMountName: GetSchemaHiveMountName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetInfo as windows_core::Interface>::IID
    }
}
