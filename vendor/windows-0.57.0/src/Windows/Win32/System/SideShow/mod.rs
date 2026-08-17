windows_core::imp::define_interface!(ISideShowBulkCapabilities, ISideShowBulkCapabilities_Vtbl, 0x3a2b7fbc_3ad5_48bd_bbf1_0e6cfbd10807);
impl core::ops::Deref for ISideShowBulkCapabilities {
    type Target = ISideShowCapabilities;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowBulkCapabilities, windows_core::IUnknown, ISideShowCapabilities);
impl ISideShowBulkCapabilities {
    pub unsafe fn GetCapabilities<P0>(&self, in_keycollection: P0, inout_pvalues: *mut Option<ISideShowPropVariantCollection>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowKeyCollection>,
    {
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), in_keycollection.param().abi(), core::mem::transmute(inout_pvalues)).ok()
    }
}
#[repr(C)]
pub struct ISideShowBulkCapabilities_Vtbl {
    pub base__: ISideShowCapabilities_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowCapabilities, ISideShowCapabilities_Vtbl, 0x535e1379_c09e_4a54_a511_597bab3a72b8);
impl core::ops::Deref for ISideShowCapabilities {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowCapabilities, windows_core::IUnknown);
impl ISideShowCapabilities {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCapability)(windows_core::Interface::as_raw(self), in_keycapability, core::mem::transmute(inout_pvalue)).ok()
    }
}
#[repr(C)]
pub struct ISideShowCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetCapability: usize,
}
windows_core::imp::define_interface!(ISideShowCapabilitiesCollection, ISideShowCapabilitiesCollection_Vtbl, 0x50305597_5e0d_4ff7_b3af_33d0d9bd52dd);
impl core::ops::Deref for ISideShowCapabilitiesCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowCapabilitiesCollection, windows_core::IUnknown);
impl ISideShowCapabilitiesCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAt(&self, in_dwindex: u32) -> windows_core::Result<ISideShowCapabilities> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), in_dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISideShowCapabilitiesCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowContent, ISideShowContent_Vtbl, 0xc18552ed_74ff_4fec_be07_4cfed29d4887);
impl core::ops::Deref for ISideShowContent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowContent, windows_core::IUnknown);
impl ISideShowContent {
    pub unsafe fn GetContent<P0>(&self, in_picapabilities: P0, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        (windows_core::Interface::vtable(self).GetContent)(windows_core::Interface::as_raw(self), in_picapabilities.param().abi(), out_pdwsize, out_ppbdata).ok()
    }
    pub unsafe fn ContentId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContentId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DifferentiateContent(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DifferentiateContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISideShowContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DifferentiateContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowContentManager, ISideShowContentManager_Vtbl, 0xa5d5b66b_eef9_41db_8d7e_e17c33ab10b0);
impl core::ops::Deref for ISideShowContentManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowContentManager, windows_core::IUnknown);
impl ISideShowContentManager {
    pub unsafe fn Add<P0>(&self, in_picontent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowContent>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), in_picontent.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, in_contentid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), in_contentid).ok()
    }
    pub unsafe fn RemoveAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventSink<P0>(&self, in_pievents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowEvents>,
    {
        (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), in_pievents.param().abi()).ok()
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> windows_core::Result<ISideShowCapabilitiesCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISideShowContentManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowEvents, ISideShowEvents_Vtbl, 0x61feca4c_deb4_4a7e_8d75_51f1132d615b);
impl core::ops::Deref for ISideShowEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowEvents, windows_core::IUnknown);
impl ISideShowEvents {
    pub unsafe fn ContentMissing(&self, in_contentid: u32) -> windows_core::Result<ISideShowContent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContentMissing)(windows_core::Interface::as_raw(self), in_contentid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ApplicationEvent<P0>(&self, in_picapabilities: P0, in_dweventid: u32, in_pbeventdata: Option<&[u8]>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        (windows_core::Interface::vtable(self).ApplicationEvent)(windows_core::Interface::as_raw(self), in_picapabilities.param().abi(), in_dweventid, in_pbeventdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(in_pbeventdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn DeviceAdded<P0>(&self, in_pidevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        (windows_core::Interface::vtable(self).DeviceAdded)(windows_core::Interface::as_raw(self), in_pidevice.param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<P0>(&self, in_pidevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowCapabilities>,
    {
        (windows_core::Interface::vtable(self).DeviceRemoved)(windows_core::Interface::as_raw(self), in_pidevice.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISideShowEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContentMissing: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u8) -> windows_core::HRESULT,
    pub DeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowKeyCollection, ISideShowKeyCollection_Vtbl, 0x045473bc_a37b_4957_b144_68105411ed8e);
impl core::ops::Deref for ISideShowKeyCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowKeyCollection, windows_core::IUnknown);
impl ISideShowKeyCollection {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, pkey).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok()
    }
}
#[repr(C)]
pub struct ISideShowKeyCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowNotification, ISideShowNotification_Vtbl, 0x03c93300_8ab2_41c5_9b79_46127a30e148);
impl core::ops::Deref for ISideShowNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowNotification, windows_core::IUnknown);
impl ISideShowNotification {
    pub unsafe fn NotificationId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotificationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotificationId(&self, in_notificationid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotificationId)(windows_core::Interface::as_raw(self), in_notificationid).ok()
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTitle<P0>(&self, in_pwsztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), in_pwsztitle.param().abi()).ok()
    }
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMessage<P0>(&self, in_pwszmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMessage)(windows_core::Interface::as_raw(self), in_pwszmessage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Image(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetImage<P0>(&self, in_hicon: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
    {
        (windows_core::Interface::vtable(self).SetImage)(windows_core::Interface::as_raw(self), in_hicon.param().abi()).ok()
    }
    pub unsafe fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpirationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExpirationTime(&self, in_ptime: Option<*const super::super::Foundation::SYSTEMTIME>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExpirationTime)(windows_core::Interface::as_raw(self), core::mem::transmute(in_ptime.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct ISideShowNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotificationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNotificationId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Image: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetImage: usize,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowNotificationManager, ISideShowNotificationManager_Vtbl, 0x63cea909_f2b9_4302_b5e1_c68e6d9ab833);
impl core::ops::Deref for ISideShowNotificationManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowNotificationManager, windows_core::IUnknown);
impl ISideShowNotificationManager {
    pub unsafe fn Show<P0>(&self, in_pinotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISideShowNotification>,
    {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), in_pinotification.param().abi()).ok()
    }
    pub unsafe fn Revoke(&self, in_notificationid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), in_notificationid).ok()
    }
    pub unsafe fn RevokeAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevokeAll)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISideShowNotificationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RevokeAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowPropVariantCollection, ISideShowPropVariantCollection_Vtbl, 0x2ea7a549_7bff_4aae_bab0_22d43111de49);
impl core::ops::Deref for ISideShowPropVariantCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowPropVariantCollection, windows_core::IUnknown);
impl ISideShowPropVariantCollection {
    pub unsafe fn Add(&self, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelems).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), dwindex).ok()
    }
}
#[repr(C)]
pub struct ISideShowPropVariantCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISideShowSession, ISideShowSession_Vtbl, 0xe22331ee_9e7d_4922_9fc2_ab7aa41ce491);
impl core::ops::Deref for ISideShowSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISideShowSession, windows_core::IUnknown);
impl ISideShowSession {
    pub unsafe fn RegisterContent(&self, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID) -> windows_core::Result<ISideShowContentManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterContent)(windows_core::Interface::as_raw(self), in_applicationid, in_endpointid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterNotifications(&self, in_applicationid: *const windows_core::GUID) -> windows_core::Result<ISideShowNotificationManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterNotifications)(windows_core::Interface::as_raw(self), in_applicationid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISideShowSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterContent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CONTENT_ID_GLANCE: u32 = 0u32;
pub const CONTENT_ID_HOME: u32 = 1u32;
pub const GUID_DEVINTERFACE_SIDESHOW: windows_core::GUID = windows_core::GUID::from_u128(0x152e5811_feb9_4b00_90f4_d32947ae1681);
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = SCF_BUTTON_IDS(65280i32);
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = SCF_BUTTON_IDS(4i32);
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = SCF_BUTTON_IDS(9i32);
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(5i32);
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = SCF_BUTTON_IDS(1i32);
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = SCF_BUTTON_IDS(8i32);
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = SCF_BUTTON_IDS(7i32);
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = SCF_BUTTON_IDS(10i32);
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(6i32);
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(2i32);
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(11i32);
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(3i32);
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = SCF_EVENT_IDS(3i32);
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = SCF_EVENT_IDS(2i32);
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = SCF_EVENT_IDS(1i32);
pub const SIDESHOW_APPLICATION_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x4cb572fa_1d3b_49b3_a17a_2e6bff052854);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 1 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: windows_core::GUID = windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 10 };
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(2i32);
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(0i32);
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(1i32);
pub const SIDESHOW_CONTENT_MISSING_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x5007fba8_d313_439f_bea2_a50201d3e9a8);
pub const SIDESHOW_ENDPOINT_ICAL: windows_core::GUID = windows_core::GUID::from_u128(0x4dff36b5_9dde_4f76_9a2a_96435047063d);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0xa9a5353f_2d4b_47ce_93ee_759f3a7dda4f);
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: windows_core::GUID = windows_core::GUID::from_u128(0x57813854_2fc1_411c_a59f_f24927608804);
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(0i32);
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(1i32);
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x5009673c_3f7d_4c7e_9971_eaa2e91f1575);
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCF_BUTTON_IDS(pub i32);
impl windows_core::TypeKind for SCF_BUTTON_IDS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCF_BUTTON_IDS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCF_BUTTON_IDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCF_EVENT_IDS(pub i32);
impl windows_core::TypeKind for SCF_EVENT_IDS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCF_EVENT_IDS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCF_EVENT_IDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SIDESHOW_COLOR_TYPE(pub i32);
impl windows_core::TypeKind for SIDESHOW_COLOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SIDESHOW_COLOR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SIDESHOW_COLOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SIDESHOW_SCREEN_TYPE(pub i32);
impl windows_core::TypeKind for SIDESHOW_SCREEN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SIDESHOW_SCREEN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SIDESHOW_SCREEN_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: windows_core::GUID,
    pub EndpointId: windows_core::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl windows_core::TypeKind for APPLICATION_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: windows_core::GUID,
    pub EndpointId: windows_core::GUID,
    pub ContentId: u32,
}
impl windows_core::TypeKind for CONTENT_MISSING_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONTENT_MISSING_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
impl windows_core::TypeKind for DEVICE_USER_CHANGE_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DEVICE_USER_CHANGE_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: windows_core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl windows_core::TypeKind for EVENT_DATA_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for EVENT_DATA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
impl windows_core::TypeKind for NEW_EVENT_DATA_AVAILABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for NEW_EVENT_DATA_AVAILABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
impl windows_core::TypeKind for SCF_CONTEXTMENU_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCF_CONTEXTMENU_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
impl windows_core::TypeKind for SCF_EVENT_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCF_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
impl windows_core::TypeKind for SCF_MENUACTION_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCF_MENUACTION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
impl windows_core::TypeKind for SCF_NAVIGATION_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCF_NAVIGATION_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SideShowKeyCollection: windows_core::GUID = windows_core::GUID::from_u128(0xdfbbdbf8_18de_49b8_83dc_ebc727c62d94);
pub const SideShowNotification: windows_core::GUID = windows_core::GUID::from_u128(0x0ce3e86f_d5cd_4525_a766_1abab1a752f5);
pub const SideShowPropVariantCollection: windows_core::GUID = windows_core::GUID::from_u128(0xe640f415_539e_4923_96cd_5f093bc250cd);
pub const SideShowSession: windows_core::GUID = windows_core::GUID::from_u128(0xe20543b9_f785_4ea2_981e_c4ffa76bbc7c);
#[cfg(feature = "implement")]
core::include!("impl.rs");
