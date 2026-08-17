pub trait IItemEnumerator_Impl: Sized {
    fn Current(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IItemEnumerator {}
impl IItemEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IItemEnumerator_Vtbl
    where
        Identity: IItemEnumerator_Impl,
    {
        unsafe extern "system" fn Current<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IItemEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IItemEnumerator_Impl::Current(this) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IItemEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IItemEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    itemvalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IItemEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IItemEnumerator_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Current: Current::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsContext_Vtbl
    where
        Identity: ISettingsContext_Impl,
    {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsContext_Impl::Serialize(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, pppresults: *mut *mut Option<ISettingsResult>, pcresultcount: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsContext_Impl::Deserialize(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&ptarget), core::mem::transmute_copy(&pppresults)) {
                Ok(ok__) => {
                    pcresultcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsContext_Impl::SetUserData(this, core::mem::transmute_copy(&puserdata)).into()
        }
        unsafe extern "system" fn GetUserData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsContext_Impl::GetUserData(this) {
                Ok(ok__) => {
                    puserdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamespaceids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsContext_Impl::GetNamespaces(this) {
                Ok(ok__) => {
                    ppnamespaceids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoredSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, ppaddedsettings: *mut *mut core::ffi::c_void, ppmodifiedsettings: *mut *mut core::ffi::c_void, ppdeletedsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsContext_Impl::GetStoredSettings(this, windows_core::from_raw_borrowed(&pidentity), core::mem::transmute_copy(&ppaddedsettings), core::mem::transmute_copy(&ppmodifiedsettings), core::mem::transmute_copy(&ppdeletedsettings)).into()
        }
        unsafe extern "system" fn RevertSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, pwzsetting: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISettingsContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsContext_Impl::RevertSetting(this, windows_core::from_raw_borrowed(&pidentity), core::mem::transmute(&pwzsetting)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
            SetUserData: SetUserData::<Identity, OFFSET>,
            GetUserData: GetUserData::<Identity, OFFSET>,
            GetNamespaces: GetNamespaces::<Identity, OFFSET>,
            GetStoredSettings: GetStoredSettings::<Identity, OFFSET>,
            RevertSetting: RevertSetting::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsEngine_Vtbl
    where
        Identity: ISettingsEngine_Impl,
    {
        unsafe extern "system" fn GetNamespaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void, namespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetNamespaces(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    namespaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void, namespaceitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetNamespace(this, windows_core::from_raw_borrowed(&settingsid), core::mem::transmute_copy(&access), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    namespaceitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: i32, message: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult)) {
                Ok(ok__) => {
                    message.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingsIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::CreateSettingsIdentity(this) {
                Ok(ok__) => {
                    settingsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoreStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, status: *mut WcmUserStatus) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetStoreStatus(this, core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsEngine_Impl::LoadStore(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn UnloadStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsEngine_Impl::UnloadStore(this, core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn RegisterNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, results: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::RegisterNamespace(this, windows_core::from_raw_borrowed(&settingsid), windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&pushsettings)) {
                Ok(ok__) => {
                    results.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNamespace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, removesettings: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsEngine_Impl::UnregisterNamespace(this, windows_core::from_raw_borrowed(&settingsid), core::mem::transmute_copy(&removesettings)).into()
        }
        unsafe extern "system" fn CreateTargetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::CreateTargetInfo(this) {
                Ok(ok__) => {
                    target.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetTargetInfo(this) {
                Ok(ok__) => {
                    target.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsEngine_Impl::SetTargetInfo(this, windows_core::from_raw_borrowed(&target)).into()
        }
        unsafe extern "system" fn CreateSettingsContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, reserved: *const core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::CreateSettingsContext(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    settingscontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettingsContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsEngine_Impl::SetSettingsContext(this, windows_core::from_raw_borrowed(&settingscontext)).into()
        }
        unsafe extern "system" fn ApplySettingsContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void, pppwzidentities: *mut *mut windows_core::PWSTR, pcidentities: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::ApplySettingsContext(this, windows_core::from_raw_borrowed(&settingscontext), core::mem::transmute_copy(&pppwzidentities)) {
                Ok(ok__) => {
                    pcidentities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingsContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsEngine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsEngine_Impl::GetSettingsContext(this) {
                Ok(ok__) => {
                    settingscontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespaces: GetNamespaces::<Identity, OFFSET>,
            GetNamespace: GetNamespace::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            CreateSettingsIdentity: CreateSettingsIdentity::<Identity, OFFSET>,
            GetStoreStatus: GetStoreStatus::<Identity, OFFSET>,
            LoadStore: LoadStore::<Identity, OFFSET>,
            UnloadStore: UnloadStore::<Identity, OFFSET>,
            RegisterNamespace: RegisterNamespace::<Identity, OFFSET>,
            UnregisterNamespace: UnregisterNamespace::<Identity, OFFSET>,
            CreateTargetInfo: CreateTargetInfo::<Identity, OFFSET>,
            GetTargetInfo: GetTargetInfo::<Identity, OFFSET>,
            SetTargetInfo: SetTargetInfo::<Identity, OFFSET>,
            CreateSettingsContext: CreateSettingsContext::<Identity, OFFSET>,
            SetSettingsContext: SetSettingsContext::<Identity, OFFSET>,
            ApplySettingsContext: ApplySettingsContext::<Identity, OFFSET>,
            GetSettingsContext: GetSettingsContext::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsIdentity_Vtbl
    where
        Identity: ISettingsIdentity_Impl,
    {
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsIdentity_Impl::GetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISettingsIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsIdentity_Impl::SetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISettingsIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsIdentity_Impl::GetFlags(this) {
                Ok(ok__) => {
                    flags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: ISettingsIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsIdentity_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttribute: GetAttribute::<Identity, OFFSET>,
            SetAttribute: SetAttribute::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsItem_Vtbl
    where
        Identity: ISettingsItem_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsItem_Impl::SetValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSettingType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmSettingType) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetSettingType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmDataType) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetDataType(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueRaw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetValueRaw(this, core::mem::transmute_copy(&data)) {
                Ok(ok__) => {
                    datasize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueRaw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsItem_Impl::SetValueRaw(this, core::mem::transmute_copy(&datatype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn HasChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::HasChild(this) {
                Ok(ok__) => {
                    itemhaschild.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::Children(this) {
                Ok(ok__) => {
                    children.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetChild(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    child.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    setting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    setting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsItem_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetListKeyInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keyname: *mut core::mem::MaybeUninit<windows_core::BSTR>, datatype: *mut WcmDataType) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetListKeyInformation(this, core::mem::transmute_copy(&keyname)) {
                Ok(ok__) => {
                    datatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateListElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, keydata: *const core::mem::MaybeUninit<windows_core::VARIANT>, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::CreateListElement(this, core::mem::transmute_copy(&keydata)) {
                Ok(ok__) => {
                    child.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveListElement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsItem_Impl::RemoveListElement(this, core::mem::transmute(&elementname)).into()
        }
        unsafe extern "system" fn Attributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::Attributes(this) {
                Ok(ok__) => {
                    attributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetPath(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictionFacets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetRestrictionFacets(this) {
                Ok(ok__) => {
                    restrictionfacets.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestriction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetRestriction(this, core::mem::transmute_copy(&restrictionfacet)) {
                Ok(ok__) => {
                    facetdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsItem_Impl::GetKeyValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetSettingType: GetSettingType::<Identity, OFFSET>,
            GetDataType: GetDataType::<Identity, OFFSET>,
            GetValueRaw: GetValueRaw::<Identity, OFFSET>,
            SetValueRaw: SetValueRaw::<Identity, OFFSET>,
            HasChild: HasChild::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            GetChild: GetChild::<Identity, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, OFFSET>,
            GetListKeyInformation: GetListKeyInformation::<Identity, OFFSET>,
            CreateListElement: CreateListElement::<Identity, OFFSET>,
            RemoveListElement: RemoveListElement::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetRestrictionFacets: GetRestrictionFacets::<Identity, OFFSET>,
            GetRestriction: GetRestriction::<Identity, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsNamespace_Vtbl
    where
        Identity: ISettingsNamespace_Impl,
    {
        unsafe extern "system" fn GetIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::GetIdentity(this) {
                Ok(ok__) => {
                    settingsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::Settings(this) {
                Ok(ok__) => {
                    settings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::Save(this, core::mem::transmute_copy(&pushsettings)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    setting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    setting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISettingsNamespace_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISettingsNamespace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsNamespace_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdentity: GetIdentity::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSettingByPath: GetSettingByPath::<Identity, OFFSET>,
            CreateSettingByPath: CreateSettingByPath::<Identity, OFFSET>,
            RemoveSettingByPath: RemoveSettingByPath::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISettingsResult_Vtbl
    where
        Identity: ISettingsResult_Impl,
    {
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetDescription(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrout: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetErrorCode(this) {
                Ok(ok__) => {
                    hrout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetContextDescription(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwline: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetLine(this) {
                Ok(ok__) => {
                    dwline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcolumn: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetColumn(this) {
                Ok(ok__) => {
                    dwcolumn.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISettingsResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISettingsResult_Impl::GetSource(this) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, OFFSET>,
            GetLine: GetLine::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITargetInfo_Vtbl
    where
        Identity: ITargetInfo_Impl,
    {
        unsafe extern "system" fn GetTargetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: *mut WcmTargetMode) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetTargetMode(this) {
                Ok(ok__) => {
                    targetmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: WcmTargetMode) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetTargetMode(this, core::mem::transmute_copy(&targetmode)).into()
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetTemporaryStoreLocation(this) {
                Ok(ok__) => {
                    temporarystorelocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetTemporaryStoreLocation(this, core::mem::transmute(&temporarystorelocation)).into()
        }
        unsafe extern "system" fn GetTargetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetTargetID(this) {
                Ok(ok__) => {
                    targetid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetTargetID(this, core::mem::transmute(&targetid)).into()
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetTargetProcessorArchitecture(this) {
                Ok(ok__) => {
                    processorarchitecture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetTargetProcessorArchitecture(this, core::mem::transmute(&processorarchitecture)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, property: windows_core::PCWSTR, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, property: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetEnumerator(this) {
                Ok(ok__) => {
                    enumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::ExpandTarget(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                Ok(ok__) => {
                    expandedlocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandTargetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: super::super::Foundation::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::ExpandTargetPath(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                Ok(ok__) => {
                    expandedlocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModulePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, path: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetModulePath(this, core::mem::transmute(&module), core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn LoadModule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::LoadModule(this, core::mem::transmute(&module)) {
                Ok(ok__) => {
                    modulehandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWow64Context<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, installermodule: windows_core::PCWSTR, wow64context: *const u8) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetWow64Context(this, core::mem::transmute(&installermodule), core::mem::transmute_copy(&wow64context)).into()
        }
        unsafe extern "system" fn TranslateWow64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientarchitecture: windows_core::PCWSTR, value: windows_core::PCWSTR, translatedvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::TranslateWow64(this, core::mem::transmute(&clientarchitecture), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    translatedvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzhivedir: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetSchemaHiveLocation(this, core::mem::transmute(&pwzhivedir)).into()
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phivelocation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetSchemaHiveLocation(this) {
                Ok(ok__) => {
                    phivelocation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzmountname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITargetInfo_Impl::SetSchemaHiveMountName(this, core::mem::transmute(&pwzmountname)).into()
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmountname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITargetInfo_Impl::GetSchemaHiveMountName(this) {
                Ok(ok__) => {
                    pmountname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTargetMode: GetTargetMode::<Identity, OFFSET>,
            SetTargetMode: SetTargetMode::<Identity, OFFSET>,
            GetTemporaryStoreLocation: GetTemporaryStoreLocation::<Identity, OFFSET>,
            SetTemporaryStoreLocation: SetTemporaryStoreLocation::<Identity, OFFSET>,
            GetTargetID: GetTargetID::<Identity, OFFSET>,
            SetTargetID: SetTargetID::<Identity, OFFSET>,
            GetTargetProcessorArchitecture: GetTargetProcessorArchitecture::<Identity, OFFSET>,
            SetTargetProcessorArchitecture: SetTargetProcessorArchitecture::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, OFFSET>,
            ExpandTarget: ExpandTarget::<Identity, OFFSET>,
            ExpandTargetPath: ExpandTargetPath::<Identity, OFFSET>,
            SetModulePath: SetModulePath::<Identity, OFFSET>,
            LoadModule: LoadModule::<Identity, OFFSET>,
            SetWow64Context: SetWow64Context::<Identity, OFFSET>,
            TranslateWow64: TranslateWow64::<Identity, OFFSET>,
            SetSchemaHiveLocation: SetSchemaHiveLocation::<Identity, OFFSET>,
            GetSchemaHiveLocation: GetSchemaHiveLocation::<Identity, OFFSET>,
            SetSchemaHiveMountName: SetSchemaHiveMountName::<Identity, OFFSET>,
            GetSchemaHiveMountName: GetSchemaHiveMountName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetInfo as windows_core::Interface>::IID
    }
}
