windows_core::imp::define_interface!(IItemEnumerator, IItemEnumerator_Vtbl, 0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for IItemEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IItemEnumerator, windows_core::IUnknown);
impl IItemEnumerator {
    pub unsafe fn Current(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Current)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IItemEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsContext, ISettingsContext_Vtbl, 0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsContext, windows_core::IUnknown);
impl ISettingsContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1>(&self, pstream: P0, ptarget: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<ITargetInfo>,
    {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), ptarget.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<P0, P1>(&self, pstream: P0, ptarget: P1, pppresults: *mut *mut Option<ISettingsResult>) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<ITargetInfo>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), ptarget.param().abi(), pppresults, &mut result__).map(|| result__)
    }
    pub unsafe fn SetUserData(&self, puserdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUserData)(windows_core::Interface::as_raw(self), puserdata).ok()
    }
    pub unsafe fn GetUserData(&self) -> windows_core::Result<*mut core::ffi::c_void> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNamespaces(&self) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStoredSettings<P0>(&self, pidentity: P0, ppaddedsettings: *mut Option<IItemEnumerator>, ppmodifiedsettings: *mut Option<IItemEnumerator>, ppdeletedsettings: *mut Option<IItemEnumerator>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
    {
        (windows_core::Interface::vtable(self).GetStoredSettings)(windows_core::Interface::as_raw(self), pidentity.param().abi(), core::mem::transmute(ppaddedsettings), core::mem::transmute(ppmodifiedsettings), core::mem::transmute(ppdeletedsettings)).ok()
    }
    pub unsafe fn RevertSetting<P0, P1>(&self, pidentity: P0, pwzsetting: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RevertSetting)(windows_core::Interface::as_raw(self), pidentity.param().abi(), pwzsetting.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISettingsContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut Option<ISettingsResult>, *mut usize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsEngine, ISettingsEngine_Vtbl, 0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsEngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsEngine, windows_core::IUnknown);
impl ISettingsEngine {
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const core::ffi::c_void) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamespaces)(windows_core::Interface::as_raw(self), flags, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNamespace<P0>(&self, settingsid: P0, access: WcmNamespaceAccess, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsNamespace>
    where
        P0: windows_core::Param<ISettingsIdentity>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), access, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hresult, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSettingsIdentity(&self) -> windows_core::Result<ISettingsIdentity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSettingsIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStoreStatus(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<WcmUserStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStoreStatus)(windows_core::Interface::as_raw(self), reserved, &mut result__).map(|| result__)
    }
    pub unsafe fn LoadStore(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadStore)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn UnloadStore(&self, reserved: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnloadStore)(windows_core::Interface::as_raw(self), reserved).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterNamespace<P0, P1, P2>(&self, settingsid: P0, stream: P1, pushsettings: P2) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<ISettingsIdentity>,
        P1: windows_core::Param<super::Com::IStream>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), stream.param().abi(), pushsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterNamespace<P0, P1>(&self, settingsid: P0, removesettings: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsIdentity>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UnregisterNamespace)(windows_core::Interface::as_raw(self), settingsid.param().abi(), removesettings.param().abi()).ok()
    }
    pub unsafe fn CreateTargetInfo(&self) -> windows_core::Result<ITargetInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTargetInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTargetInfo(&self) -> windows_core::Result<ITargetInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetInfo<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITargetInfo>,
    {
        (windows_core::Interface::vtable(self).SetTargetInfo)(windows_core::Interface::as_raw(self), target.param().abi()).ok()
    }
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const core::ffi::c_void) -> windows_core::Result<ISettingsContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSettingsContext)(windows_core::Interface::as_raw(self), flags, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSettingsContext<P0>(&self, settingscontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISettingsContext>,
    {
        (windows_core::Interface::vtable(self).SetSettingsContext)(windows_core::Interface::as_raw(self), settingscontext.param().abi()).ok()
    }
    pub unsafe fn ApplySettingsContext<P0>(&self, settingscontext: P0, pppwzidentities: *mut *mut windows_core::PWSTR) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<ISettingsContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplySettingsContext)(windows_core::Interface::as_raw(self), settingscontext.param().abi(), pppwzidentities, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSettingsContext(&self) -> windows_core::Result<ISettingsContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSettingsContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISettingsEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, WcmNamespaceEnumerationFlags, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WcmNamespaceAccess, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateSettingsIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut WcmUserStatus) -> windows_core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterNamespace: usize,
    pub UnregisterNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CreateTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsIdentity, ISettingsIdentity_Vtbl, 0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsIdentity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsIdentity, windows_core::IUnknown);
impl ISettingsIdentity {
    pub unsafe fn GetAttribute<P0>(&self, reserved: *const core::ffi::c_void, name: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), reserved, name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAttribute<P0, P1>(&self, reserved: *const core::ffi::c_void, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttribute)(windows_core::Interface::as_raw(self), reserved, name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
}
#[repr(C)]
pub struct ISettingsIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsItem, ISettingsItem_Vtbl, 0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsItem, windows_core::IUnknown);
impl ISettingsItem {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetValue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue(&self, value: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetSettingType(&self) -> windows_core::Result<WcmSettingType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSettingType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataType(&self) -> windows_core::Result<WcmDataType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValueRaw)(windows_core::Interface::as_raw(self), data, &mut result__).map(|| result__)
    }
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueRaw)(windows_core::Interface::as_raw(self), datatype, core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok()
    }
    pub unsafe fn HasChild(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasChild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Children(&self) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetChild<P0>(&self, name: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut windows_core::BSTR) -> windows_core::Result<WcmDataType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetListKeyInformation)(windows_core::Interface::as_raw(self), core::mem::transmute(keyname), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateListElement(&self, keydata: *const windows_core::VARIANT) -> windows_core::Result<ISettingsItem> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateListElement)(windows_core::Interface::as_raw(self), core::mem::transmute(keydata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveListElement<P0>(&self, elementname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveListElement)(windows_core::Interface::as_raw(self), elementname.param().abi()).ok()
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRestrictionFacets(&self) -> windows_core::Result<WcmRestrictionFacets> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestrictionFacets)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestriction)(windows_core::Interface::as_raw(self), restrictionfacet, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetKeyValue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKeyValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISettingsItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetSettingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmSettingType) -> windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmDataType) -> windows_core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, u32) -> windows_core::HRESULT,
    pub HasChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetListKeyInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut WcmDataType) -> windows_core::HRESULT,
    pub CreateListElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveListElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRestrictionFacets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmRestrictionFacets) -> windows_core::HRESULT,
    pub GetRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, WcmRestrictionFacets, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetKeyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsNamespace, ISettingsNamespace_Vtbl, 0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsNamespace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsNamespace, windows_core::IUnknown);
impl ISettingsNamespace {
    pub unsafe fn GetIdentity(&self) -> windows_core::Result<ISettingsIdentity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Settings(&self) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Save<P0>(&self, pushsettings: P0) -> windows_core::Result<ISettingsResult>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pushsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSettingByPath<P0>(&self, path: P0) -> windows_core::Result<ISettingsItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveSettingByPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveSettingByPath)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn GetAttribute<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISettingsNamespace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsResult, ISettingsResult_Vtbl, 0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ISettingsResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISettingsResult, windows_core::IUnknown);
impl ISettingsResult {
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContextDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLine(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetColumn(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISettingsResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetInfo, ITargetInfo_Vtbl, 0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
impl core::ops::Deref for ITargetInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetInfo, windows_core::IUnknown);
impl ITargetInfo {
    pub unsafe fn GetTargetMode(&self) -> windows_core::Result<WcmTargetMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTargetMode)(windows_core::Interface::as_raw(self), targetmode).ok()
    }
    pub unsafe fn GetTemporaryStoreLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTemporaryStoreLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTemporaryStoreLocation<P0>(&self, temporarystorelocation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTemporaryStoreLocation)(windows_core::Interface::as_raw(self), temporarystorelocation.param().abi()).ok()
    }
    pub unsafe fn GetTargetID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetID(&self, targetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTargetID)(windows_core::Interface::as_raw(self), core::mem::transmute(targetid)).ok()
    }
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetProcessorArchitecture)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetProcessorArchitecture<P0>(&self, processorarchitecture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetProcessorArchitecture)(windows_core::Interface::as_raw(self), processorarchitecture.param().abi()).ok()
    }
    pub unsafe fn GetProperty<P0, P1>(&self, offline: P0, property: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), offline.param().abi(), property.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0, P1, P2>(&self, offline: P0, property: P1, value: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), offline.param().abi(), property.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> windows_core::Result<IItemEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExpandTarget<P0, P1>(&self, offline: P0, location: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpandTarget)(windows_core::Interface::as_raw(self), offline.param().abi(), location.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExpandTargetPath<P0, P1>(&self, offline: P0, location: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpandTargetPath)(windows_core::Interface::as_raw(self), offline.param().abi(), location.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetModulePath<P0, P1>(&self, module: P0, path: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetModulePath)(windows_core::Interface::as_raw(self), module.param().abi(), path.param().abi()).ok()
    }
    pub unsafe fn LoadModule<P0>(&self, module: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadModule)(windows_core::Interface::as_raw(self), module.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWow64Context<P0>(&self, installermodule: P0, wow64context: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetWow64Context)(windows_core::Interface::as_raw(self), installermodule.param().abi(), wow64context).ok()
    }
    pub unsafe fn TranslateWow64<P0, P1>(&self, clientarchitecture: P0, value: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslateWow64)(windows_core::Interface::as_raw(self), clientarchitecture.param().abi(), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSchemaHiveLocation<P0>(&self, pwzhivedir: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSchemaHiveLocation)(windows_core::Interface::as_raw(self), pwzhivedir.param().abi()).ok()
    }
    pub unsafe fn GetSchemaHiveLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSchemaHiveLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSchemaHiveMountName<P0>(&self, pwzmountname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSchemaHiveMountName)(windows_core::Interface::as_raw(self), pwzmountname.param().abi()).ok()
    }
    pub unsafe fn GetSchemaHiveMountName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSchemaHiveMountName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITargetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WcmTargetMode) -> windows_core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(*mut core::ffi::c_void, WcmTargetMode) -> windows_core::HRESULT,
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTargetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpandTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ExpandTargetPath: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetModulePath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LoadModule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
    pub SetWow64Context: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8) -> windows_core::HRESULT,
    pub TranslateWow64: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemaHiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemaHiveMountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmDataType(pub i32);
impl windows_core::TypeKind for WcmDataType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmDataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmNamespaceAccess(pub i32);
impl windows_core::TypeKind for WcmNamespaceAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmNamespaceAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmNamespaceAccess").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
impl windows_core::TypeKind for WcmNamespaceEnumerationFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmNamespaceEnumerationFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmNamespaceEnumerationFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmRestrictionFacets(pub i32);
impl windows_core::TypeKind for WcmRestrictionFacets {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmRestrictionFacets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmRestrictionFacets").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmSettingType(pub i32);
impl windows_core::TypeKind for WcmSettingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmSettingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmSettingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmTargetMode(pub i32);
impl windows_core::TypeKind for WcmTargetMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmTargetMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmTargetMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WcmUserStatus(pub i32);
impl windows_core::TypeKind for WcmUserStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WcmUserStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WcmUserStatus").field(&self.0).finish()
    }
}
pub const SettingsEngine: windows_core::GUID = windows_core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
#[cfg(feature = "implement")]
core::include!("impl.rs");
