pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
windows_core::imp::define_interface!(IItemEnumerator, IItemEnumerator_Vtbl, 0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(IItemEnumerator, windows_core::IUnknown);
impl IItemEnumerator {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Current(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Current)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Current: usize,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IItemEnumerator_Impl: windows_core::IUnknownImpl {
    fn Current(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IItemEnumerator_Vtbl {
    pub const fn new<Identity: IItemEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Current<Identity: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IItemEnumerator_Impl::Current(this) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemvalid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IItemEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        itemvalid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IItemEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IItemEnumerator_Impl::Reset(this).into()
            }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IItemEnumerator {}
windows_core::imp::define_interface!(ISettingsContext, ISettingsContext_Vtbl, 0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsContext, windows_core::IUnknown);
impl ISettingsContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1>(&self, pstream: P0, ptarget: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<ITargetInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), ptarget.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<P0, P1>(&self, pstream: P0, ptarget: P1, pppresults: *mut *mut Option<ISettingsResult>) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<ITargetInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), ptarget.param().abi(), pppresults as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUserData(&self, puserdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserData)(windows_core::Interface::as_raw(self), puserdata).ok() }
    }
    pub unsafe fn GetUserData(&self) -> windows_core::Result<*mut core::ffi::c_void> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNamespaces(&self) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoredSettings<P0>(&self, pidentity: P0, ppaddedsettings: *mut Option<IItemEnumerator>, ppmodifiedsettings: *mut Option<IItemEnumerator>, ppdeletedsettings: *mut Option<IItemEnumerator>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetStoredSettings)(windows_core::Interface::as_raw(self), pidentity.param().abi(), core::mem::transmute(ppaddedsettings), core::mem::transmute(ppmodifiedsettings), core::mem::transmute(ppdeletedsettings)).ok() }
    }
    pub unsafe fn RevertSetting<P0, P1>(&self, pidentity: P0, pwzsetting: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RevertSetting)(windows_core::Interface::as_raw(self), pidentity.param().abi(), pwzsetting.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISettingsContext_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pstream: windows_core::Ref<super::Com::IStream>, ptarget: windows_core::Ref<ITargetInfo>) -> windows_core::Result<()>;
    fn Deserialize(&self, pstream: windows_core::Ref<super::Com::IStream>, ptarget: windows_core::Ref<ITargetInfo>, pppresults: *mut *mut Option<ISettingsResult>) -> windows_core::Result<usize>;
    fn SetUserData(&self, puserdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetUserData(&self) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetNamespaces(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetStoredSettings(&self, pidentity: windows_core::Ref<ISettingsIdentity>, ppaddedsettings: windows_core::OutRef<IItemEnumerator>, ppmodifiedsettings: windows_core::OutRef<IItemEnumerator>, ppdeletedsettings: windows_core::OutRef<IItemEnumerator>) -> windows_core::Result<()>;
    fn RevertSetting(&self, pidentity: windows_core::Ref<ISettingsIdentity>, pwzsetting: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISettingsContext_Vtbl {
    pub const fn new<Identity: ISettingsContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsContext_Impl::Serialize(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn Deserialize<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, pppresults: *mut *mut *mut core::ffi::c_void, pcresultcount: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsContext_Impl::Deserialize(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&ptarget), core::mem::transmute_copy(&pppresults)) {
                    Ok(ok__) => {
                        pcresultcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserData<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsContext_Impl::SetUserData(this, core::mem::transmute_copy(&puserdata)).into()
            }
        }
        unsafe extern "system" fn GetUserData<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puserdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsContext_Impl::GetUserData(this) {
                    Ok(ok__) => {
                        puserdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNamespaces<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamespaceids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsContext_Impl::GetNamespaces(this) {
                    Ok(ok__) => {
                        ppnamespaceids.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoredSettings<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, ppaddedsettings: *mut *mut core::ffi::c_void, ppmodifiedsettings: *mut *mut core::ffi::c_void, ppdeletedsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsContext_Impl::GetStoredSettings(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&ppaddedsettings), core::mem::transmute_copy(&ppmodifiedsettings), core::mem::transmute_copy(&ppdeletedsettings)).into()
            }
        }
        unsafe extern "system" fn RevertSetting<Identity: ISettingsContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *mut core::ffi::c_void, pwzsetting: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsContext_Impl::RevertSetting(this, core::mem::transmute_copy(&pidentity), core::mem::transmute(&pwzsetting)).into()
            }
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
impl windows_core::RuntimeName for ISettingsContext {}
windows_core::imp::define_interface!(ISettingsEngine, ISettingsEngine_Vtbl, 0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsEngine, windows_core::IUnknown);
impl ISettingsEngine {
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), flags, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNamespace<P0>(&self, settingsid: P0, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsNamespace>
    where
        P0: windows_core::Param<ISettingsIdentity>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), access, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hresult, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateSettingsIdentity(&self) -> windows_core::Result<ISettingsIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSettingsIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStoreStatus(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<WcmUserStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStoreStatus)(windows_core::Interface::as_raw(self), reserved, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LoadStore(&self, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadStore)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn UnloadStore(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnloadStore)(windows_core::Interface::as_raw(self), reserved).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterNamespace<P0, P1>(&self, settingsid: P0, stream: P1, pushsettings: bool) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<ISettingsIdentity>,
        P1: windows_core::Param<super::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), stream.param().abi(), pushsettings.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UnregisterNamespace<P0>(&self, settingsid: P0, removesettings: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), removesettings.into()).ok() }
    }
    pub unsafe fn CreateTargetInfo(&self) -> windows_core::Result<ITargetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTargetInfo(&self) -> windows_core::Result<ITargetInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTargetInfo<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITargetInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetInfo)(windows_core::Interface::as_raw(self), target.param().abi()).ok() }
    }
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSettingsContext)(windows_core::Interface::as_raw(self), flags, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSettingsContext<P0>(&self, settingscontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSettingsContext)(windows_core::Interface::as_raw(self), settingscontext.param().abi()).ok() }
    }
    pub unsafe fn ApplySettingsContext<P0>(&self, settingscontext: P0, pppwzidentities: *mut *mut windows_core::PWSTR) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<ISettingsContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplySettingsContext)(windows_core::Interface::as_raw(self), settingscontext.param().abi(), pppwzidentities as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSettingsContext(&self) -> windows_core::Result<ISettingsContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettingsContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, WcmNamespaceEnumerationFlags, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WcmNamespaceAccess, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingsIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut WcmUserStatus) -> windows_core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterNamespace: usize,
    pub UnregisterNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsEngine_Impl: windows_core::IUnknownImpl {
    fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void) -> windows_core::Result<IItemEnumerator>;
    fn GetNamespace(&self, settingsid: windows_core::Ref<ISettingsIdentity>, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsNamespace>;
    fn GetErrorDescription(&self, hresult: i32) -> windows_core::Result<windows_core::BSTR>;
    fn CreateSettingsIdentity(&self) -> windows_core::Result<ISettingsIdentity>;
    fn GetStoreStatus(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<WcmUserStatus>;
    fn LoadStore(&self, flags: u32) -> windows_core::Result<()>;
    fn UnloadStore(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn RegisterNamespace(&self, settingsid: windows_core::Ref<ISettingsIdentity>, stream: windows_core::Ref<super::Com::IStream>, pushsettings: windows_core::BOOL) -> windows_core::Result<super::Variant::VARIANT>;
    fn UnregisterNamespace(&self, settingsid: windows_core::Ref<ISettingsIdentity>, removesettings: windows_core::BOOL) -> windows_core::Result<()>;
    fn CreateTargetInfo(&self) -> windows_core::Result<ITargetInfo>;
    fn GetTargetInfo(&self) -> windows_core::Result<ITargetInfo>;
    fn SetTargetInfo(&self, target: windows_core::Ref<ITargetInfo>) -> windows_core::Result<()>;
    fn CreateSettingsContext(&self, flags: u32, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsContext>;
    fn SetSettingsContext(&self, settingscontext: windows_core::Ref<ISettingsContext>) -> windows_core::Result<()>;
    fn ApplySettingsContext(&self, settingscontext: windows_core::Ref<ISettingsContext>, pppwzidentities: *mut *mut windows_core::PWSTR) -> windows_core::Result<usize>;
    fn GetSettingsContext(&self) -> windows_core::Result<ISettingsContext>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISettingsEngine_Vtbl {
    pub const fn new<Identity: ISettingsEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamespaces<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void, namespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetNamespaces(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        namespaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNamespace<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void, namespaceitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetNamespace(this, core::mem::transmute_copy(&settingsid), core::mem::transmute_copy(&access), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        namespaceitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: i32, message: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult)) {
                    Ok(ok__) => {
                        message.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSettingsIdentity<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::CreateSettingsIdentity(this) {
                    Ok(ok__) => {
                        settingsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStoreStatus<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, status: *mut WcmUserStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetStoreStatus(this, core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadStore<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsEngine_Impl::LoadStore(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnloadStore<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsEngine_Impl::UnloadStore(this, core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn RegisterNamespace<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, pushsettings: windows_core::BOOL, results: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::RegisterNamespace(this, core::mem::transmute_copy(&settingsid), core::mem::transmute_copy(&stream), core::mem::transmute_copy(&pushsettings)) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterNamespace<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut core::ffi::c_void, removesettings: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsEngine_Impl::UnregisterNamespace(this, core::mem::transmute_copy(&settingsid), core::mem::transmute_copy(&removesettings)).into()
            }
        }
        unsafe extern "system" fn CreateTargetInfo<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::CreateTargetInfo(this) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetInfo<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetTargetInfo(this) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetInfo<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsEngine_Impl::SetTargetInfo(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn CreateSettingsContext<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, reserved: *const core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::CreateSettingsContext(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        settingscontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSettingsContext<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsEngine_Impl::SetSettingsContext(this, core::mem::transmute_copy(&settingscontext)).into()
            }
        }
        unsafe extern "system" fn ApplySettingsContext<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut core::ffi::c_void, pppwzidentities: *mut *mut windows_core::PWSTR, pcidentities: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::ApplySettingsContext(this, core::mem::transmute_copy(&settingscontext), core::mem::transmute_copy(&pppwzidentities)) {
                    Ok(ok__) => {
                        pcidentities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSettingsContext<Identity: ISettingsEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingscontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsEngine_Impl::GetSettingsContext(this) {
                    Ok(ok__) => {
                        settingscontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISettingsEngine {}
windows_core::imp::define_interface!(ISettingsIdentity, ISettingsIdentity_Vtbl, 0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsIdentity, windows_core::IUnknown);
impl ISettingsIdentity {
    pub unsafe fn GetAttribute<P1>(&self, reserved: *const core::ffi::c_void, name: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), reserved, name.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAttribute<P1, P2>(&self, reserved: *const core::ffi::c_void, name: P1, value: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttribute)(windows_core::Interface::as_raw(self), reserved, name.param().abi(), value.param().abi()).ok() }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISettingsIdentity_Impl: windows_core::IUnknownImpl {
    fn GetAttribute(&self, reserved: *const core::ffi::c_void, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetAttribute(&self, reserved: *const core::ffi::c_void, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn SetFlags(&self, flags: u32) -> windows_core::Result<()>;
}
impl ISettingsIdentity_Vtbl {
    pub const fn new<Identity: ISettingsIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttribute<Identity: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsIdentity_Impl::GetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttribute<Identity: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const core::ffi::c_void, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsIdentity_Impl::SetAttribute(this, core::mem::transmute_copy(&reserved), core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsIdentity_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ISettingsIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsIdentity_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
            }
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
impl windows_core::RuntimeName for ISettingsIdentity {}
windows_core::imp::define_interface!(ISettingsItem, ISettingsItem_Vtbl, 0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsItem, windows_core::IUnknown);
impl ISettingsItem {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, value: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value)).ok() }
    }
    pub unsafe fn GetSettingType(&self) -> windows_core::Result<WcmSettingType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettingType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataType(&self) -> windows_core::Result<WcmDataType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueRaw)(windows_core::Interface::as_raw(self), data as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValueRaw)(windows_core::Interface::as_raw(self), datatype, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn HasChild(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasChild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Children(&self) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetChild<P0>(&self, name: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi()).ok() }
    }
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut windows_core::BSTR) -> windows_core::Result<WcmDataType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetListKeyInformation)(windows_core::Interface::as_raw(self), core::mem::transmute(keyname), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Variant::VARIANT) -> windows_core::Result<ISettingsItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateListElement)(windows_core::Interface::as_raw(self), core::mem::transmute(keydata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveListElement<P0>(&self, elementname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveListElement)(windows_core::Interface::as_raw(self), elementname.param().abi()).ok() }
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRestrictionFacets(&self) -> windows_core::Result<WcmRestrictionFacets> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictionFacets)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestriction)(windows_core::Interface::as_raw(self), restrictionfacet, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetKeyValue(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeyValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmSettingType) -> windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmDataType) -> windows_core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, u32) -> windows_core::HRESULT,
    pub HasChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetListKeyInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut WcmDataType) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateListElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttribute: usize,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictionFacets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmRestrictionFacets) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, WcmRestrictionFacets, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetKeyValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsItem_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetValue(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, value: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetSettingType(&self) -> windows_core::Result<WcmSettingType>;
    fn GetDataType(&self) -> windows_core::Result<WcmDataType>;
    fn GetValueRaw(&self, data: *mut *mut u8) -> windows_core::Result<u32>;
    fn SetValueRaw(&self, datatype: i32, data: *const u8, datasize: u32) -> windows_core::Result<()>;
    fn HasChild(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Children(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetChild(&self, name: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn GetSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetListKeyInformation(&self, keyname: *mut windows_core::BSTR) -> windows_core::Result<WcmDataType>;
    fn CreateListElement(&self, keydata: *const super::Variant::VARIANT) -> windows_core::Result<ISettingsItem>;
    fn RemoveListElement(&self, elementname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Attributes(&self) -> windows_core::Result<IItemEnumerator>;
    fn GetAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRestrictionFacets(&self) -> windows_core::Result<WcmRestrictionFacets>;
    fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetKeyValue(&self) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISettingsItem_Vtbl {
    pub const fn new<Identity: ISettingsItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetValue(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsItem_Impl::SetValue(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSettingType<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmSettingType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetSettingType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataType<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut WcmDataType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetDataType(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueRaw<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetValueRaw(this, core::mem::transmute_copy(&data)) {
                    Ok(ok__) => {
                        datasize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueRaw<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsItem_Impl::SetValueRaw(this, core::mem::transmute_copy(&datatype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn HasChild<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemhaschild: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::HasChild(this) {
                    Ok(ok__) => {
                        itemhaschild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Children<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::Children(this) {
                    Ok(ok__) => {
                        children.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChild<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetChild(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        child.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        setting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        setting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsItem_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn GetListKeyInformation<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keyname: *mut *mut core::ffi::c_void, datatype: *mut WcmDataType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetListKeyInformation(this, core::mem::transmute_copy(&keyname)) {
                    Ok(ok__) => {
                        datatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateListElement<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keydata: *const super::Variant::VARIANT, child: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::CreateListElement(this, core::mem::transmute_copy(&keydata)) {
                    Ok(ok__) => {
                        child.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveListElement<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsItem_Impl::RemoveListElement(this, core::mem::transmute(&elementname)).into()
            }
        }
        unsafe extern "system" fn Attributes<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::Attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPath<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetPath(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictionFacets<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetRestrictionFacets(this) {
                    Ok(ok__) => {
                        restrictionfacets.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestriction<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetRestriction(this, core::mem::transmute_copy(&restrictionfacet)) {
                    Ok(ok__) => {
                        facetdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: ISettingsItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsItem_Impl::GetKeyValue(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISettingsItem {}
windows_core::imp::define_interface!(ISettingsNamespace, ISettingsNamespace_Vtbl, 0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsNamespace, windows_core::IUnknown);
impl ISettingsNamespace {
    pub unsafe fn GetIdentity(&self) -> windows_core::Result<ISettingsIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Settings(&self) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Save(&self, pushsettings: bool) -> windows_core::Result<ISettingsResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pushsettings.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAttribute: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISettingsNamespace_Impl: windows_core::IUnknownImpl {
    fn GetIdentity(&self) -> windows_core::Result<ISettingsIdentity>;
    fn Settings(&self) -> windows_core::Result<IItemEnumerator>;
    fn Save(&self, pushsettings: windows_core::BOOL) -> windows_core::Result<ISettingsResult>;
    fn GetSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn CreateSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<ISettingsItem>;
    fn RemoveSettingByPath(&self, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISettingsNamespace_Vtbl {
    pub const fn new<Identity: ISettingsNamespace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentity<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::GetIdentity(this) {
                    Ok(ok__) => {
                        settingsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settings<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::Settings(this) {
                    Ok(ok__) => {
                        settings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Save<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pushsettings: windows_core::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::Save(this, core::mem::transmute_copy(&pushsettings)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSettingByPath<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::GetSettingByPath(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        setting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSettingByPath<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, setting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::CreateSettingByPath(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        setting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSettingByPath<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISettingsNamespace_Impl::RemoveSettingByPath(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ISettingsNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsNamespace_Impl::GetAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISettingsNamespace {}
windows_core::imp::define_interface!(ISettingsResult, ISettingsResult_Vtbl, 0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ISettingsResult, windows_core::IUnknown);
impl ISettingsResult {
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLine(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColumn(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISettingsResult_Impl: windows_core::IUnknownImpl {
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLine(&self) -> windows_core::Result<u32>;
    fn GetColumn(&self) -> windows_core::Result<u32>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ISettingsResult_Vtbl {
    pub const fn new<Identity: ISettingsResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrout: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetErrorCode(this) {
                    Ok(ok__) => {
                        hrout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetContextDescription(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLine<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwline: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetLine(this) {
                    Ok(ok__) => {
                        dwline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcolumn: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetColumn(this) {
                    Ok(ok__) => {
                        dwcolumn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: ISettingsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISettingsResult_Impl::GetSource(this) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISettingsResult {}
windows_core::imp::define_interface!(ITargetInfo, ITargetInfo_Vtbl, 0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
windows_core::imp::interface_hierarchy!(ITargetInfo, windows_core::IUnknown);
impl ITargetInfo {
    pub unsafe fn GetTargetMode(&self) -> windows_core::Result<WcmTargetMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetMode)(windows_core::Interface::as_raw(self), targetmode).ok() }
    }
    pub unsafe fn GetTemporaryStoreLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemporaryStoreLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTemporaryStoreLocation<P0>(&self, temporarystorelocation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTemporaryStoreLocation)(windows_core::Interface::as_raw(self), temporarystorelocation.param().abi()).ok() }
    }
    pub unsafe fn GetTargetID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetID(&self, targetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetID)(windows_core::Interface::as_raw(self), core::mem::transmute(targetid)).ok() }
    }
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetProcessorArchitecture)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetProcessorArchitecture<P0>(&self, processorarchitecture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetProcessorArchitecture)(windows_core::Interface::as_raw(self), processorarchitecture.param().abi()).ok() }
    }
    pub unsafe fn GetProperty<P1>(&self, offline: bool, property: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), offline.into(), property.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProperty<P1, P2>(&self, offline: bool, property: P1, value: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), offline.into(), property.param().abi(), value.param().abi()).ok() }
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IItemEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExpandTarget<P1>(&self, offline: bool, location: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpandTarget)(windows_core::Interface::as_raw(self), offline.into(), location.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExpandTargetPath<P1>(&self, offline: bool, location: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpandTargetPath)(windows_core::Interface::as_raw(self), offline.into(), location.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModulePath<P0, P1>(&self, module: P0, path: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetModulePath)(windows_core::Interface::as_raw(self), module.param().abi(), path.param().abi()).ok() }
    }
    pub unsafe fn LoadModule<P0>(&self, module: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadModule)(windows_core::Interface::as_raw(self), module.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWow64Context<P0>(&self, installermodule: P0, wow64context: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWow64Context)(windows_core::Interface::as_raw(self), installermodule.param().abi(), wow64context).ok() }
    }
    pub unsafe fn TranslateWow64<P0, P1>(&self, clientarchitecture: P0, value: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslateWow64)(windows_core::Interface::as_raw(self), clientarchitecture.param().abi(), value.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSchemaHiveLocation<P0>(&self, pwzhivedir: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchemaHiveLocation)(windows_core::Interface::as_raw(self), pwzhivedir.param().abi()).ok() }
    }
    pub unsafe fn GetSchemaHiveLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemaHiveLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSchemaHiveMountName<P0>(&self, pwzmountname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchemaHiveMountName)(windows_core::Interface::as_raw(self), pwzmountname.param().abi()).ok() }
    }
    pub unsafe fn GetSchemaHiveMountName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemaHiveMountName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmTargetMode) -> windows_core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(*mut core::ffi::c_void, WcmTargetMode) -> windows_core::HRESULT,
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTargetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpandTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpandTargetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModulePath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LoadModule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
    pub SetWow64Context: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8) -> windows_core::HRESULT,
    pub TranslateWow64: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemaHiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemaHiveMountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITargetInfo_Impl: windows_core::IUnknownImpl {
    fn GetTargetMode(&self) -> windows_core::Result<WcmTargetMode>;
    fn SetTargetMode(&self, targetmode: WcmTargetMode) -> windows_core::Result<()>;
    fn GetTemporaryStoreLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTemporaryStoreLocation(&self, temporarystorelocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTargetID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetID(&self, targetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetTargetProcessorArchitecture(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetProcessorArchitecture(&self, processorarchitecture: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetProperty(&self, offline: windows_core::BOOL, property: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetProperty(&self, offline: windows_core::BOOL, property: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetEnumerator(&self) -> windows_core::Result<IItemEnumerator>;
    fn ExpandTarget(&self, offline: windows_core::BOOL, location: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn ExpandTargetPath(&self, offline: windows_core::BOOL, location: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetModulePath(&self, module: &windows_core::PCWSTR, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LoadModule(&self, module: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn SetWow64Context(&self, installermodule: &windows_core::PCWSTR, wow64context: *const u8) -> windows_core::Result<()>;
    fn TranslateWow64(&self, clientarchitecture: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetSchemaHiveLocation(&self, pwzhivedir: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemaHiveLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSchemaHiveMountName(&self, pwzmountname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemaHiveMountName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITargetInfo_Vtbl {
    pub const fn new<Identity: ITargetInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTargetMode<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: *mut WcmTargetMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetTargetMode(this) {
                    Ok(ok__) => {
                        targetmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetMode<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetmode: WcmTargetMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetTargetMode(this, core::mem::transmute_copy(&targetmode)).into()
            }
        }
        unsafe extern "system" fn GetTemporaryStoreLocation<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetTemporaryStoreLocation(this) {
                    Ok(ok__) => {
                        temporarystorelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTemporaryStoreLocation<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, temporarystorelocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetTemporaryStoreLocation(this, core::mem::transmute(&temporarystorelocation)).into()
            }
        }
        unsafe extern "system" fn GetTargetID<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetTargetID(this) {
                    Ok(ok__) => {
                        targetid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetID<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetTargetID(this, core::mem::transmute(&targetid)).into()
            }
        }
        unsafe extern "system" fn GetTargetProcessorArchitecture<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetTargetProcessorArchitecture(this) {
                    Ok(ok__) => {
                        processorarchitecture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetProcessorArchitecture<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processorarchitecture: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetTargetProcessorArchitecture(this, core::mem::transmute(&processorarchitecture)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: windows_core::BOOL, property: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: windows_core::BOOL, property: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetProperty(this, core::mem::transmute_copy(&offline), core::mem::transmute(&property), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetEnumerator<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetEnumerator(this) {
                    Ok(ok__) => {
                        enumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpandTarget<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: windows_core::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::ExpandTarget(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                    Ok(ok__) => {
                        expandedlocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpandTargetPath<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offline: windows_core::BOOL, location: windows_core::PCWSTR, expandedlocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::ExpandTargetPath(this, core::mem::transmute_copy(&offline), core::mem::transmute(&location)) {
                    Ok(ok__) => {
                        expandedlocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModulePath<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, path: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetModulePath(this, core::mem::transmute(&module), core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn LoadModule<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::LoadModule(this, core::mem::transmute(&module)) {
                    Ok(ok__) => {
                        modulehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWow64Context<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installermodule: windows_core::PCWSTR, wow64context: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetWow64Context(this, core::mem::transmute(&installermodule), core::mem::transmute_copy(&wow64context)).into()
            }
        }
        unsafe extern "system" fn TranslateWow64<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientarchitecture: windows_core::PCWSTR, value: windows_core::PCWSTR, translatedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::TranslateWow64(this, core::mem::transmute(&clientarchitecture), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        translatedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSchemaHiveLocation<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzhivedir: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetSchemaHiveLocation(this, core::mem::transmute(&pwzhivedir)).into()
            }
        }
        unsafe extern "system" fn GetSchemaHiveLocation<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phivelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetSchemaHiveLocation(this) {
                    Ok(ok__) => {
                        phivelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSchemaHiveMountName<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzmountname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetInfo_Impl::SetSchemaHiveMountName(this, core::mem::transmute(&pwzmountname)).into()
            }
        }
        unsafe extern "system" fn GetSchemaHiveMountName<Identity: ITargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmountname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetInfo_Impl::GetSchemaHiveMountName(this) {
                    Ok(ok__) => {
                        pmountname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ITargetInfo {}
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
pub const SettingsEngine: windows_core::GUID = windows_core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
pub const WCM_E_ABORTOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80220028_u32 as _);
pub const WCM_E_ASSERTIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8022001A_u32 as _);
pub const WCM_E_ATTRIBUTENOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80220004_u32 as _);
pub const WCM_E_ATTRIBUTENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220003_u32 as _);
pub const WCM_E_CONFLICTINGASSERTION: windows_core::HRESULT = windows_core::HRESULT(0x80220019_u32 as _);
pub const WCM_E_CYCLICREFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80220023_u32 as _);
pub const WCM_E_DUPLICATENAME: windows_core::HRESULT = windows_core::HRESULT(0x8022001B_u32 as _);
pub const WCM_E_EXPRESSIONNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220010_u32 as _);
pub const WCM_E_HANDLERNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x8022001E_u32 as _);
pub const WCM_E_INTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x80220000_u32 as _);
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x80220027_u32 as _);
pub const WCM_E_INVALIDDATATYPE: windows_core::HRESULT = windows_core::HRESULT(0x80220008_u32 as _);
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x80220017_u32 as _);
pub const WCM_E_INVALIDHANDLERSYNTAX: windows_core::HRESULT = windows_core::HRESULT(0x8022001F_u32 as _);
pub const WCM_E_INVALIDKEY: windows_core::HRESULT = windows_core::HRESULT(0x8022001C_u32 as _);
pub const WCM_E_INVALIDLANGUAGEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8022000E_u32 as _);
pub const WCM_E_INVALIDPATH: windows_core::HRESULT = windows_core::HRESULT(0x8022000B_u32 as _);
pub const WCM_E_INVALIDPROCESSORFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8022002A_u32 as _);
pub const WCM_E_INVALIDSTREAM: windows_core::HRESULT = windows_core::HRESULT(0x8022001D_u32 as _);
pub const WCM_E_INVALIDVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80220005_u32 as _);
pub const WCM_E_INVALIDVALUEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80220006_u32 as _);
pub const WCM_E_INVALIDVERSIONFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8022000D_u32 as _);
pub const WCM_E_KEYNOTCHANGEABLE: windows_core::HRESULT = windows_core::HRESULT(0x8022000F_u32 as _);
pub const WCM_E_MANIFESTCOMPILATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80220022_u32 as _);
pub const WCM_E_MISSINGCONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x80220029_u32 as _);
pub const WCM_E_MIXTYPEASSERTION: windows_core::HRESULT = windows_core::HRESULT(0x80220024_u32 as _);
pub const WCM_E_NAMESPACEALREADYREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80220015_u32 as _);
pub const WCM_E_NAMESPACENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220014_u32 as _);
pub const WCM_E_NOTIFICATIONNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220018_u32 as _);
pub const WCM_E_NOTPOSITIONED: windows_core::HRESULT = windows_core::HRESULT(0x80220009_u32 as _);
pub const WCM_E_NOTSUPPORTEDFUNCTION: windows_core::HRESULT = windows_core::HRESULT(0x80220025_u32 as _);
pub const WCM_E_READONLYITEM: windows_core::HRESULT = windows_core::HRESULT(0x8022000A_u32 as _);
pub const WCM_E_RESTRICTIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80220021_u32 as _);
pub const WCM_E_SOURCEMANEMPTYVALUE: windows_core::HRESULT = windows_core::HRESULT(0x8022002B_u32 as _);
pub const WCM_E_STATENODENOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80220002_u32 as _);
pub const WCM_E_STATENODENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220001_u32 as _);
pub const WCM_E_STORECORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80220016_u32 as _);
pub const WCM_E_SUBSTITUTIONNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220011_u32 as _);
pub const WCM_E_TYPENOTSPECIFIED: windows_core::HRESULT = windows_core::HRESULT(0x80220007_u32 as _);
pub const WCM_E_UNKNOWNRESULT: windows_core::HRESULT = windows_core::HRESULT(0x80221003_u32 as _);
pub const WCM_E_USERALREADYREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80220012_u32 as _);
pub const WCM_E_USERNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80220013_u32 as _);
pub const WCM_E_VALIDATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80220020_u32 as _);
pub const WCM_E_VALUETOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x80220026_u32 as _);
pub const WCM_E_WRONGESCAPESTRING: windows_core::HRESULT = windows_core::HRESULT(0x8022000C_u32 as _);
pub const WCM_SETTINGS_ID_ARCHITECTURE: windows_core::PCWSTR = windows_core::w!("architecture");
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
pub const WCM_SETTINGS_ID_LANGUAGE: windows_core::PCWSTR = windows_core::w!("language");
pub const WCM_SETTINGS_ID_NAME: windows_core::PCWSTR = windows_core::w!("name");
pub const WCM_SETTINGS_ID_TOKEN: windows_core::PCWSTR = windows_core::w!("token");
pub const WCM_SETTINGS_ID_URI: windows_core::PCWSTR = windows_core::w!("uri");
pub const WCM_SETTINGS_ID_VERSION: windows_core::PCWSTR = windows_core::w!("version");
pub const WCM_SETTINGS_ID_VERSION_SCOPE: windows_core::PCWSTR = windows_core::w!("versionScope");
pub const WCM_S_ATTRIBUTENOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x221005_u32 as _);
pub const WCM_S_ATTRIBUTENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x221001_u32 as _);
pub const WCM_S_INTERNALERROR: windows_core::HRESULT = windows_core::HRESULT(0x221000_u32 as _);
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x221004_u32 as _);
pub const WCM_S_LEGACYSETTINGWARNING: windows_core::HRESULT = windows_core::HRESULT(0x221002_u32 as _);
pub const WCM_S_NAMESPACENOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x221006_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmDataType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmNamespaceAccess(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmRestrictionFacets(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmSettingType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmTargetMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WcmUserStatus(pub i32);
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
