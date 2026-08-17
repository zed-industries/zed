windows_core::imp::define_interface!(INetworkTransportSettings, INetworkTransportSettings_Vtbl, 0x5e7abb2c_f2c1_4a61_bd35_deb7a08ab0f1);
impl core::ops::Deref for INetworkTransportSettings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkTransportSettings, windows_core::IUnknown);
impl INetworkTransportSettings {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplySetting)(windows_core::Interface::as_raw(self), settingid, valuein.len().try_into().unwrap(), core::mem::transmute(valuein.as_ptr()), lengthout, valueout).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QuerySetting)(windows_core::Interface::as_raw(self), settingid, valuein.len().try_into().unwrap(), core::mem::transmute(valuein.as_ptr()), lengthout, valueout).ok()
    }
}
#[repr(C)]
pub struct INetworkTransportSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
windows_core::imp::define_interface!(INotificationTransportSync, INotificationTransportSync_Vtbl, 0x79eb1402_0ab8_49c0_9e14_a1ae4ba93058);
impl core::ops::Deref for INotificationTransportSync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INotificationTransportSync, windows_core::IUnknown);
impl INotificationTransportSync {
    pub unsafe fn CompleteDelivery(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteDelivery)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INotificationTransportSync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteDelivery: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCBuddy, IRTCBuddy_Vtbl, 0xfcb136c8_7b90_4e0c_befe_56edf0ba6f1c);
impl core::ops::Deref for IRTCBuddy {
    type Target = IRTCPresenceContact;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCBuddy, windows_core::IUnknown, IRTCPresenceContact);
impl IRTCBuddy {
    pub unsafe fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Notes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Notes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCBuddy_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCBuddy2, IRTCBuddy2_Vtbl, 0x102f9588_23e7_40e3_954d_cd7a1d5c0361);
impl core::ops::Deref for IRTCBuddy2 {
    type Target = IRTCBuddy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCBuddy2, windows_core::IUnknown, IRTCPresenceContact, IRTCBuddy);
impl IRTCBuddy2 {
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePresenceDevices(&self) -> windows_core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePresenceDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PresenceDevices(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PresenceDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SubscriptionType(&self) -> windows_core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriptionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCBuddy2_Vtbl {
    pub base__: IRTCBuddy_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumeratePresenceDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PresenceDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PresenceDevices: usize,
    pub SubscriptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCBuddyEvent, IRTCBuddyEvent_Vtbl, 0xf36d755d_17e6_404e_954f_0fc07574c78d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCBuddyEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCBuddyEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent {
    pub unsafe fn Buddy(&self) -> windows_core::Result<IRTCBuddy> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Buddy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCBuddyEvent2, IRTCBuddyEvent2_Vtbl, 0x484a7f1e_73f0_4990_bfc2_60bc3978a720);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCBuddyEvent2 {
    type Target = IRTCBuddyEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCBuddyEvent2, windows_core::IUnknown, super::Com::IDispatch, IRTCBuddyEvent);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent2 {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_BUDDY_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyEvent2_Vtbl {
    pub base__: IRTCBuddyEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_BUDDY_EVENT_TYPE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCBuddyGroup, IRTCBuddyGroup_Vtbl, 0x60361e68_9164_4389_a4c6_d0b3925bda5e);
impl core::ops::Deref for IRTCBuddyGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCBuddyGroup, windows_core::IUnknown);
impl IRTCBuddyGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrgroupname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi()).ok()
    }
    pub unsafe fn AddBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        (windows_core::Interface::vtable(self).AddBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok()
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        (windows_core::Interface::vtable(self).RemoveBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateBuddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Buddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Data(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), bstrdata.param().abi()).ok()
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCBuddyGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCBuddyGroupEvent, IRTCBuddyGroupEvent_Vtbl, 0x3a79e1d1_b736_4414_96f8_bbc7f08863e4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCBuddyGroupEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCBuddyGroupEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroupEvent {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_GROUP_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Group(&self) -> windows_core::Result<IRTCBuddyGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Buddy(&self) -> windows_core::Result<IRTCBuddy2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Buddy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCBuddyGroupEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_GROUP_EVENT_TYPE) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClient, IRTCClient_Vtbl, 0x07829e45_9a34_408e_a011_bddf13487cd1);
impl core::ops::Deref for IRTCClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClient, windows_core::IUnknown);
impl IRTCClient {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrepareForShutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventFilter)(windows_core::Interface::as_raw(self), lfilter).ok()
    }
    pub unsafe fn EventFilter(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPreferredMediaTypes<P0>(&self, lmediatypes: i32, fpersistent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPreferredMediaTypes)(windows_core::Interface::as_raw(self), lmediatypes, fpersistent.param().abi()).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateSession<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> windows_core::Result<IRTCSession>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), entype, bstrlocalphoneuri.param().abi(), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetListenForIncomingSessions)(windows_core::Interface::as_raw(self), enlisten).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> windows_core::Result<RTC_LISTEN_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ListenForIncomingSessions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_NetworkAddresses<P0, P1>(&self, ftcp: P0, fexternal: P1) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_NetworkAddresses)(windows_core::Interface::as_raw(self), ftcp.param().abi(), fexternal.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_Volume)(windows_core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Volume)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
    }
    pub unsafe fn put_AudioMuted<P0>(&self, endevice: RTC_AUDIO_DEVICE, fmuted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_AudioMuted)(windows_core::Interface::as_raw(self), endevice, fmuted.param().abi()).ok()
    }
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AudioMuted)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> windows_core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IVideoWindow)(windows_core::Interface::as_raw(self), endevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_PreferredAudioDevice<P0>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_PreferredAudioDevice)(windows_core::Interface::as_raw(self), endevice, bstrdevicename.param().abi()).ok()
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PreferredAudioDevice)(windows_core::Interface::as_raw(self), endevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_PreferredVolume)(windows_core::Interface::as_raw(self), endevice, lvolume).ok()
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PreferredVolume)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
    }
    pub unsafe fn SetPreferredAEC<P0>(&self, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPreferredAEC)(windows_core::Interface::as_raw(self), benable.param().abi()).ok()
    }
    pub unsafe fn PreferredAEC(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredAEC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPreferredVideoDevice<P0>(&self, bstrdevicename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPreferredVideoDevice)(windows_core::Interface::as_raw(self), bstrdevicename.param().abi()).ok()
    }
    pub unsafe fn PreferredVideoDevice(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredVideoDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActiveMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxBitrate)(windows_core::Interface::as_raw(self), lmaxbitrate).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxBitrate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTemporalSpatialTradeOff)(windows_core::Interface::as_raw(self), lvalue).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TemporalSpatialTradeOff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NetworkQuality(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetworkQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartT120Applet)(windows_core::Interface::as_raw(self), enapplet).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopT120Applets)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IsT120AppletRunning)(windows_core::Interface::as_raw(self), enapplet, &mut result__).map(|| result__)
    }
    pub unsafe fn LocalUserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalUserURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalUserURI<P0>(&self, bstruseruri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalUserURI)(windows_core::Interface::as_raw(self), bstruseruri.param().abi()).ok()
    }
    pub unsafe fn LocalUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalUserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalUserName<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalUserName)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn PlayRing<P0>(&self, entype: RTC_RING_TYPE, bplay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).PlayRing)(windows_core::Interface::as_raw(self), entype, bplay.param().abi()).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendDTMF)(windows_core::Interface::as_raw(self), endtmf).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvokeTuningWizard)(windows_core::Interface::as_raw(self), hwndparent).ok()
    }
    pub unsafe fn IsTuned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTuned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareForShutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPreferredMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PreferredMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MediaCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetListenForIncomingSessions: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_LISTEN_MODE) -> windows_core::HRESULT,
    pub ListenForIncomingSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_LISTEN_MODE) -> windows_core::HRESULT,
    pub get_NetworkAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, i32) -> windows_core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut i32) -> windows_core::HRESULT,
    pub put_AudioMuted: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_AudioMuted: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub get_IVideoWindow: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_VIDEO_DEVICE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com")))]
    get_IVideoWindow: usize,
    pub put_PreferredAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_PreferredAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_PreferredVolume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, i32) -> windows_core::HRESULT,
    pub get_PreferredVolume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut i32) -> windows_core::HRESULT,
    pub SetPreferredAEC: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PreferredAEC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPreferredVideoDevice: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PreferredVideoDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ActiveMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTemporalSpatialTradeOff: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TemporalSpatialTradeOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NetworkQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartT120Applet: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_T120_APPLET) -> windows_core::HRESULT,
    pub StopT120Applets: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IsT120AppletRunning: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_T120_APPLET, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LocalUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PlayRing: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_RING_TYPE, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SendDTMF: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_DTMF) -> windows_core::HRESULT,
    pub InvokeTuningWizard: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub IsTuned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClient2, IRTCClient2_Vtbl, 0x0c91d71d_1064_42da_bfa5_572beb8eea84);
impl core::ops::Deref for IRTCClient2 {
    type Target = IRTCClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClient2, windows_core::IUnknown, IRTCClient);
impl IRTCClient2 {
    pub unsafe fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_AnswerMode)(windows_core::Interface::as_raw(self), entype, enmode).ok()
    }
    pub unsafe fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> windows_core::Result<RTC_ANSWER_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AnswerMode)(windows_core::Interface::as_raw(self), entype, &mut result__).map(|| result__)
    }
    pub unsafe fn InvokeTuningWizardEx<P0, P1>(&self, hwndparent: isize, fallowaudio: P0, fallowvideo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).InvokeTuningWizardEx)(windows_core::Interface::as_raw(self), hwndparent, fallowaudio.param().abi(), fallowvideo.param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientName<P0>(&self, bstrclientname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), bstrclientname.param().abi()).ok()
    }
    pub unsafe fn SetClientCurVer<P0>(&self, bstrclientcurver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientCurVer)(windows_core::Interface::as_raw(self), bstrclientcurver.param().abi()).ok()
    }
    pub unsafe fn InitializeEx(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeEx)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn CreateSessionWithDescription<P0, P1, P2>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, pprofile: P2, lflags: i32) -> windows_core::Result<IRTCSession2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSessionWithDescription)(windows_core::Interface::as_raw(self), bstrcontenttype.param().abi(), bstrsessiondescription.param().abi(), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSessionDescriptionManager<P0>(&self, psessiondescriptionmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCSessionDescriptionManager>,
    {
        (windows_core::Interface::vtable(self).SetSessionDescriptionManager)(windows_core::Interface::as_raw(self), psessiondescriptionmanager.param().abi()).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_AllowedPorts)(windows_core::Interface::as_raw(self), ltransport, enlistenmode).ok()
    }
    pub unsafe fn get_AllowedPorts(&self, ltransport: i32) -> windows_core::Result<RTC_LISTEN_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AllowedPorts)(windows_core::Interface::as_raw(self), ltransport, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCClient2_Vtbl {
    pub base__: IRTCClient_Vtbl,
    pub put_AnswerMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, RTC_ANSWER_MODE) -> windows_core::HRESULT,
    pub get_AnswerMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, *mut RTC_ANSWER_MODE) -> windows_core::HRESULT,
    pub InvokeTuningWizardEx: unsafe extern "system" fn(*mut core::ffi::c_void, isize, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientCurVer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitializeEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateSessionWithDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSessionDescriptionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub put_AllowedPorts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, RTC_LISTEN_MODE) -> windows_core::HRESULT,
    pub get_AllowedPorts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut RTC_LISTEN_MODE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCClientEvent, IRTCClientEvent_Vtbl, 0x2b493b7a_3cba_4170_9c8b_76a9dacdd644);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCClientEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCClientEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientEvent {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_CLIENT_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Client(&self) -> windows_core::Result<IRTCClient> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Client)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCClientEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_CLIENT_EVENT_TYPE) -> windows_core::HRESULT,
    pub Client: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClientPortManagement, IRTCClientPortManagement_Vtbl, 0xd5df3f03_4bde_4417_aefe_71177bdaea66);
impl core::ops::Deref for IRTCClientPortManagement {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientPortManagement, windows_core::IUnknown);
impl IRTCClientPortManagement {
    pub unsafe fn StartListenAddressAndPort<P0>(&self, bstrinternallocaladdress: P0, linternallocalport: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartListenAddressAndPort)(windows_core::Interface::as_raw(self), bstrinternallocaladdress.param().abi(), linternallocalport).ok()
    }
    pub unsafe fn StopListenAddressAndPort<P0>(&self, bstrinternallocaladdress: P0, linternallocalport: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StopListenAddressAndPort)(windows_core::Interface::as_raw(self), bstrinternallocaladdress.param().abi(), linternallocalport).ok()
    }
    pub unsafe fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPortRange)(windows_core::Interface::as_raw(self), enporttype, plminvalue, plmaxvalue).ok()
    }
}
#[repr(C)]
pub struct IRTCClientPortManagement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListenAddressAndPort: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub StopListenAddressAndPort: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub GetPortRange: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PORT_TYPE, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClientPresence, IRTCClientPresence_Vtbl, 0x11c3cbcc_0744_42d1_968a_51aa1bb274c6);
impl core::ops::Deref for IRTCClientPresence {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientPresence, windows_core::IUnknown);
impl IRTCClientPresence {
    pub unsafe fn EnablePresence<P0, P1>(&self, fusestorage: P0, varstorage: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).EnablePresence)(windows_core::Interface::as_raw(self), fusestorage.param().abi(), varstorage.param().abi()).ok()
    }
    pub unsafe fn Export<P0>(&self, varstorage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), varstorage.param().abi()).ok()
    }
    pub unsafe fn Import<P0, P1>(&self, varstorage: P0, freplaceall: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), varstorage.param().abi(), freplaceall.param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateBuddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Buddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Buddy<P0>(&self, bstrpresentityuri: P0) -> windows_core::Result<IRTCBuddy>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Buddy)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddBuddy<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fpersistent: P3, pprofile: P4, lflags: i32) -> windows_core::Result<IRTCBuddy>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddBuddy)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), bstrusername.param().abi(), bstrdata.param().abi(), fpersistent.param().abi(), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        (windows_core::Interface::vtable(self).RemoveBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> windows_core::Result<IRTCEnumWatchers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateWatchers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Watchers(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Watchers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Watcher<P0>(&self, bstrpresentityuri: P0) -> windows_core::Result<IRTCWatcher>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Watcher)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddWatcher<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fblocked: P3, fpersistent: P4) -> windows_core::Result<IRTCWatcher>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddWatcher)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), bstrusername.param().abi(), bstrdata.param().abi(), fblocked.param().abi(), fpersistent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveWatcher<P0>(&self, pwatcher: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCWatcher>,
    {
        (windows_core::Interface::vtable(self).RemoveWatcher)(windows_core::Interface::as_raw(self), pwatcher.param().abi()).ok()
    }
    pub unsafe fn SetLocalPresenceInfo<P0>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalPresenceInfo)(windows_core::Interface::as_raw(self), enstatus, bstrnotes.param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> windows_core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OfferWatcherMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOfferWatcherMode)(windows_core::Interface::as_raw(self), enmode).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> windows_core::Result<RTC_PRIVACY_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivacyMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivacyMode)(windows_core::Interface::as_raw(self), enmode).ok()
    }
}
#[repr(C)]
pub struct IRTCClientPresence_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnablePresence: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub get_Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateWatchers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Watchers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Watchers: usize,
    pub get_Watcher: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_STATUS, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OfferWatcherMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT,
    pub SetOfferWatcherMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT,
    pub PrivacyMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRIVACY_MODE) -> windows_core::HRESULT,
    pub SetPrivacyMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRIVACY_MODE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClientPresence2, IRTCClientPresence2_Vtbl, 0xad1809e8_62f7_4783_909a_29c9d2cb1d34);
impl core::ops::Deref for IRTCClientPresence2 {
    type Target = IRTCClientPresence;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientPresence2, windows_core::IUnknown, IRTCClientPresence);
impl IRTCClientPresence2 {
    pub unsafe fn EnablePresenceEx<P0, P1>(&self, pprofile: P0, varstorage: P1, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).EnablePresenceEx)(windows_core::Interface::as_raw(self), pprofile.param().abi(), varstorage.param().abi(), lflags).ok()
    }
    pub unsafe fn DisablePresence(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisablePresence)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddGroup<P0, P1, P2>(&self, bstrgroupname: P0, bstrdata: P1, pprofile: P2, lflags: i32) -> windows_core::Result<IRTCBuddyGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddGroup)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), bstrdata.param().abi(), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveGroup<P0>(&self, pgroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddyGroup>,
    {
        (windows_core::Interface::vtable(self).RemoveGroup)(windows_core::Interface::as_raw(self), pgroup.param().abi()).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Group<P0>(&self, bstrgroupname: P0) -> windows_core::Result<IRTCBuddyGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Group)(windows_core::Interface::as_raw(self), bstrgroupname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddWatcherEx<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, enstate: RTC_WATCHER_STATE, fpersistent: P3, enscope: RTC_ACE_SCOPE, pprofile: P4, lflags: i32) -> windows_core::Result<IRTCWatcher2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddWatcherEx)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), bstrusername.param().abi(), bstrdata.param().abi(), enstate, fpersistent.param().abi(), enscope, pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_WatcherEx<P0>(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: P0) -> windows_core::Result<IRTCWatcher2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_WatcherEx)(windows_core::Interface::as_raw(self), enmode, bstrpresentityuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_PresenceProperty<P0>(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, bstrproperty.param().abi()).ok()
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPresenceData<P0, P1>(&self, bstrnamespace: P0, bstrdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPresenceData)(windows_core::Interface::as_raw(self), bstrnamespace.param().abi(), bstrdata.param().abi()).ok()
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocalPresenceInfo)(windows_core::Interface::as_raw(self), penstatus, core::mem::transmute(pbstrnotes)).ok()
    }
    pub unsafe fn AddBuddyEx<P0, P1, P2, P3, P4>(&self, bstrpresentityuri: P0, bstrusername: P1, bstrdata: P2, fpersistent: P3, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: P4, lflags: i32) -> windows_core::Result<IRTCBuddy2>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<IRTCProfile>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddBuddyEx)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi(), bstrusername.param().abi(), bstrdata.param().abi(), fpersistent.param().abi(), ensubscriptiontype, pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCClientPresence2_Vtbl {
    pub base__: IRTCClientPresence_Vtbl,
    pub EnablePresenceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    pub DisablePresence: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_Group: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWatcherEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, RTC_WATCHER_STATE, super::super::Foundation::VARIANT_BOOL, RTC_ACE_SCOPE, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_WatcherEx: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_WATCHER_MATCH_MODE, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddBuddyEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, RTC_BUDDY_SUBSCRIPTION_TYPE, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClientProvisioning, IRTCClientProvisioning_Vtbl, 0xb9f5cf06_65b9_4a80_a0e6_73cae3ef3822);
impl core::ops::Deref for IRTCClientProvisioning {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientProvisioning, windows_core::IUnknown);
impl IRTCClientProvisioning {
    pub unsafe fn CreateProfile<P0>(&self, bstrprofilexml: P0) -> windows_core::Result<IRTCProfile>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProfile)(windows_core::Interface::as_raw(self), bstrprofilexml.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableProfile<P0>(&self, pprofile: P0, lregisterflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        (windows_core::Interface::vtable(self).EnableProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi(), lregisterflags).ok()
    }
    pub unsafe fn DisableProfile<P0>(&self, pprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        (windows_core::Interface::vtable(self).DisableProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> windows_core::Result<IRTCEnumProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateProfiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profiles(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProfile<P0, P1, P2, P3>(&self, bstruseraccount: P0, bstruserpassword: P1, bstruseruri: P2, bstrserver: P3, ltransport: i32, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), bstruseraccount.param().abi(), bstruserpassword.param().abi(), bstruseruri.param().abi(), bstrserver.param().abi(), ltransport, lcookie).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCClientProvisioning_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisableProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profiles: usize,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, isize) -> windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCClientProvisioning2, IRTCClientProvisioning2_Vtbl, 0xa70909b5_f40e_4587_bb75_e6bc0845023e);
impl core::ops::Deref for IRTCClientProvisioning2 {
    type Target = IRTCClientProvisioning;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientProvisioning2, windows_core::IUnknown, IRTCClientProvisioning);
impl IRTCClientProvisioning2 {
    pub unsafe fn EnableProfileEx<P0>(&self, pprofile: P0, lregisterflags: i32, lroamingflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        (windows_core::Interface::vtable(self).EnableProfileEx)(windows_core::Interface::as_raw(self), pprofile.param().abi(), lregisterflags, lroamingflags).ok()
    }
}
#[repr(C)]
pub struct IRTCClientProvisioning2_Vtbl {
    pub base__: IRTCClientProvisioning_Vtbl,
    pub EnableProfileEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCCollection, IRTCCollection_Vtbl, 0xec7c8096_b918_4044_94f1_e4fba0361d5c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCDispatchEventNotification, IRTCDispatchEventNotification_Vtbl, 0x176ddfbe_fec0_4d55_bc87_84cff1ef7f91);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCDispatchEventNotification {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCDispatchEventNotification, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCDispatchEventNotification_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IRTCEnumBuddies, IRTCEnumBuddies_Vtbl, 0xf7296917_5569_4b3b_b3af_98d1144b2b87);
impl core::ops::Deref for IRTCEnumBuddies {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumBuddies, windows_core::IUnknown);
impl IRTCEnumBuddies {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCBuddy>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumBuddies> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumBuddies_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumGroups, IRTCEnumGroups_Vtbl, 0x742378d6_a141_4415_8f27_35d99076cf5d);
impl core::ops::Deref for IRTCEnumGroups {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumGroups, windows_core::IUnknown);
impl IRTCEnumGroups {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCBuddyGroup>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumGroups> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumGroups_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumParticipants, IRTCEnumParticipants_Vtbl, 0xfcd56f29_4a4f_41b2_ba5c_f5bccc060bf6);
impl core::ops::Deref for IRTCEnumParticipants {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumParticipants, windows_core::IUnknown);
impl IRTCEnumParticipants {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCParticipant>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumParticipants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumParticipants_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumPresenceDevices, IRTCEnumPresenceDevices_Vtbl, 0x708c2ab7_8bf8_42f8_8c7d_635197ad5539);
impl core::ops::Deref for IRTCEnumPresenceDevices {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumPresenceDevices, windows_core::IUnknown);
impl IRTCEnumPresenceDevices {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCPresenceDevice>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumPresenceDevices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumPresenceDevices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumProfiles, IRTCEnumProfiles_Vtbl, 0x29b7c41c_ed82_4bca_84ad_39d5101b58e3);
impl core::ops::Deref for IRTCEnumProfiles {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumProfiles, windows_core::IUnknown);
impl IRTCEnumProfiles {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCProfile>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumProfiles> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumUserSearchResults, IRTCEnumUserSearchResults_Vtbl, 0x83d4d877_aa5d_4a5b_8d0e_002a8067e0e8);
impl core::ops::Deref for IRTCEnumUserSearchResults {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumUserSearchResults, windows_core::IUnknown);
impl IRTCEnumUserSearchResults {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCUserSearchResult>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumUserSearchResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEnumWatchers, IRTCEnumWatchers_Vtbl, 0xa87d55d7_db74_4ed1_9ca4_77a0e41b413e);
impl core::ops::Deref for IRTCEnumWatchers {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEnumWatchers, windows_core::IUnknown);
impl IRTCEnumWatchers {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCWatcher>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumWatchers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCEnumWatchers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCEventNotification, IRTCEventNotification_Vtbl, 0x13fa24c7_5748_4b21_91f5_7397609ce747);
impl core::ops::Deref for IRTCEventNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCEventNotification, windows_core::IUnknown);
impl IRTCEventNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<P0>(&self, rtcevent: RTC_EVENT, pevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), rtcevent, pevent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRTCEventNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_EVENT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCInfoEvent, IRTCInfoEvent_Vtbl, 0x4e1d68ae_1912_4f49_b2c3_594fadfd425f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCInfoEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCInfoEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCInfoEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Info(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Info)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InfoHeader(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InfoHeader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCInfoEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InfoHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCIntensityEvent, IRTCIntensityEvent_Vtbl, 0x4c23bf51_390c_4992_a41d_41eec05b2a4b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCIntensityEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCIntensityEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCIntensityEvent {
    pub unsafe fn Level(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Min(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Min)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Max(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Max)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<RTC_AUDIO_DEVICE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCIntensityEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_AUDIO_DEVICE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCMediaEvent, IRTCMediaEvent_Vtbl, 0x099944fb_bcda_453e_8c41_e13da2adf7f3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCMediaEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCMediaEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaEvent {
    pub unsafe fn MediaType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_MEDIA_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EventReason(&self) -> windows_core::Result<RTC_MEDIA_EVENT_REASON> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMediaEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MEDIA_EVENT_TYPE) -> windows_core::HRESULT,
    pub EventReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MEDIA_EVENT_REASON) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCMediaRequestEvent, IRTCMediaRequestEvent_Vtbl, 0x52572d15_148c_4d97_a36c_2da55c289d63);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCMediaRequestEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCMediaRequestEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaRequestEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProposedMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProposedMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentMedia(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Accept(&self, lmediatypes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self), lmediatypes).ok()
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMediaRequestEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProposedMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REINVITE_STATE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCMessagingEvent, IRTCMessagingEvent_Vtbl, 0xd3609541_1b29_4de5_a4ad_5aebaf319512);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCMessagingEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCMessagingEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCMessagingEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_MESSAGING_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MessageHeader(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageHeader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserStatus(&self) -> windows_core::Result<RTC_MESSAGING_USER_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCMessagingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MESSAGING_EVENT_TYPE) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MessageHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MESSAGING_USER_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCParticipant, IRTCParticipant_Vtbl, 0xae86add5_26b1_4414_af1d_b94cd938d739);
impl core::ops::Deref for IRTCParticipant {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCParticipant, windows_core::IUnknown);
impl IRTCParticipant {
    pub unsafe fn UserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Removable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Removable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCParticipant_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Removable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCParticipantStateChangeEvent, IRTCParticipantStateChangeEvent_Vtbl, 0x09bcb597_f0fa_48f9_b420_468cea7fde04);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCParticipantStateChangeEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCParticipantStateChangeEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCParticipantStateChangeEvent {
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCParticipantStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCPortManager, IRTCPortManager_Vtbl, 0xda77c14b_6208_43ca_8ddf_5b60a0a69fac);
impl core::ops::Deref for IRTCPortManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCPortManager, windows_core::IUnknown);
impl IRTCPortManager {
    pub unsafe fn GetMapping<P0>(&self, bstrremoteaddress: P0, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut windows_core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut windows_core::BSTR, plexternallocalport: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetMapping)(windows_core::Interface::as_raw(self), bstrremoteaddress.param().abi(), enporttype, core::mem::transmute(pbstrinternallocaladdress), plinternallocalport, core::mem::transmute(pbstrexternallocaladdress), plexternallocalport).ok()
    }
    pub unsafe fn UpdateRemoteAddress<P0, P1, P2>(&self, bstrremoteaddress: P0, bstrinternallocaladdress: P1, linternallocalport: i32, bstrexternallocaladdress: P2, lexternallocalport: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UpdateRemoteAddress)(windows_core::Interface::as_raw(self), bstrremoteaddress.param().abi(), bstrinternallocaladdress.param().abi(), linternallocalport, bstrexternallocaladdress.param().abi(), lexternallocalport).ok()
    }
    pub unsafe fn ReleaseMapping<P0, P1>(&self, bstrinternallocaladdress: P0, linternallocalport: i32, bstrexternallocaladdress: P1, lexternallocaladdress: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ReleaseMapping)(windows_core::Interface::as_raw(self), bstrinternallocaladdress.param().abi(), linternallocalport, bstrexternallocaladdress.param().abi(), lexternallocaladdress).ok()
    }
}
#[repr(C)]
pub struct IRTCPortManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMapping: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, RTC_PORT_TYPE, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub UpdateRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ReleaseMapping: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCPresenceContact, IRTCPresenceContact_Vtbl, 0x8b22f92c_cd90_42db_a733_212205c3e3df);
impl core::ops::Deref for IRTCPresenceContact {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCPresenceContact, windows_core::IUnknown);
impl IRTCPresenceContact {
    pub unsafe fn PresentityURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PresentityURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPresentityURI<P0>(&self, bstrpresentityuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPresentityURI)(windows_core::Interface::as_raw(self), bstrpresentityuri.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn Data(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetData<P0>(&self, bstrdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), bstrdata.param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Persistent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPersistent<P0>(&self, fpersistent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPersistent)(windows_core::Interface::as_raw(self), fpersistent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRTCPresenceContact_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PresentityURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPresentityURI: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Persistent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPersistent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCPresenceDataEvent, IRTCPresenceDataEvent_Vtbl, 0x38f0e78c_8b87_4c04_a82d_aedd83c909bb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCPresenceDataEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCPresenceDataEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceDataEvent {
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresenceDataEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCPresenceDevice, IRTCPresenceDevice_Vtbl, 0xbc6a90dd_ad9a_48da_9b0c_2515e38521ad);
impl core::ops::Deref for IRTCPresenceDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCPresenceDevice, windows_core::IUnknown);
impl IRTCPresenceDevice {
    pub unsafe fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Notes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Notes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok()
    }
}
#[repr(C)]
pub struct IRTCPresenceDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCPresencePropertyEvent, IRTCPresencePropertyEvent_Vtbl, 0xf777f570_a820_49d5_86bd_e099493f1518);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCPresencePropertyEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCPresencePropertyEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresencePropertyEvent {
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PresenceProperty(&self) -> windows_core::Result<RTC_PRESENCE_PROPERTY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PresenceProperty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresencePropertyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_PROPERTY) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCPresenceStatusEvent, IRTCPresenceStatusEvent_Vtbl, 0x78673f32_4a0f_462c_89aa_ee7706707678);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCPresenceStatusEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCPresenceStatusEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceStatusEvent {
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLocalPresenceInfo)(windows_core::Interface::as_raw(self), penstatus, core::mem::transmute(pbstrnotes)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCPresenceStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCProfile, IRTCProfile_Vtbl, 0xd07eca9e_4062_4dd4_9e7d_722a49ba7303);
impl core::ops::Deref for IRTCProfile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCProfile, windows_core::IUnknown);
impl IRTCProfile {
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn XML(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).XML)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ProviderURI)(windows_core::Interface::as_raw(self), enuri, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProviderData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProviderData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClientBanner(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientBanner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClientMinVer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientMinVer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClientCurVer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientCurVer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClientUpdateURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientUpdateURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClientData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCredentials<P0, P1, P2>(&self, bstruseruri: P0, bstruseraccount: P1, bstrpassword: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), bstruseruri.param().abi(), bstruseraccount.param().abi(), bstrpassword.param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub XML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_ProviderURI: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PROVIDER_URI, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProviderData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientBanner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClientMinVer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientCurVer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientUpdateURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCProfile2, IRTCProfile2_Vtbl, 0x4b81f84e_bdc7_4184_9154_3cb2dd7917fb);
impl core::ops::Deref for IRTCProfile2 {
    type Target = IRTCProfile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCProfile2, windows_core::IUnknown, IRTCProfile);
impl IRTCProfile2 {
    pub unsafe fn Realm(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Realm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRealm<P0>(&self, bstrrealm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRealm)(windows_core::Interface::as_raw(self), bstrrealm.param().abi()).ok()
    }
    pub unsafe fn AllowedAuth(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowedAuth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowedAuth(&self, lallowedauth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllowedAuth)(windows_core::Interface::as_raw(self), lallowedauth).ok()
    }
}
#[repr(C)]
pub struct IRTCProfile2_Vtbl {
    pub base__: IRTCProfile_Vtbl,
    pub Realm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRealm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AllowedAuth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAllowedAuth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCProfileEvent, IRTCProfileEvent_Vtbl, 0xd6d5ab3b_770e_43e8_800a_79b062395fca);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCProfileEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCProfileEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent {
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCProfileEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCProfileEvent2, IRTCProfileEvent2_Vtbl, 0x62e56edc_03fa_4121_94fb_23493fd0ae64);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCProfileEvent2 {
    type Target = IRTCProfileEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCProfileEvent2, windows_core::IUnknown, super::Com::IDispatch, IRTCProfileEvent);
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent2 {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_PROFILE_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCProfileEvent2_Vtbl {
    pub base__: IRTCProfileEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PROFILE_EVENT_TYPE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCReInviteEvent, IRTCReInviteEvent_Vtbl, 0x11558d84_204c_43e7_99b0_2034e9417f7d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCReInviteEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCReInviteEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCReInviteEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Accept<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self), bstrcontenttype.param().abi(), bstrsessiondescription.param().abi()).ok()
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCReInviteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REINVITE_STATE) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCRegistrationStateChangeEvent, IRTCRegistrationStateChangeEvent_Vtbl, 0x62d0991b_50ab_4f02_b948_ca94f26f8f95);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCRegistrationStateChangeEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCRegistrationStateChangeEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCRegistrationStateChangeEvent {
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCRegistrationStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCRoamingEvent, IRTCRoamingEvent_Vtbl, 0x79960a6b_0cb1_4dc8_a805_7318e99902e8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCRoamingEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCRoamingEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCRoamingEvent {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_ROAMING_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCRoamingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_ROAMING_EVENT_TYPE) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCSession, IRTCSession_Vtbl, 0x387c8086_99be_42fb_9973_7c0fc0ca9fa8);
impl core::ops::Deref for IRTCSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSession, windows_core::IUnknown);
impl IRTCSession {
    pub unsafe fn Client(&self) -> windows_core::Result<IRTCClient> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Client)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Type(&self) -> windows_core::Result<RTC_SESSION_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Participants(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Participants)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Answer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Answer)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), enreason).ok()
    }
    pub unsafe fn Redirect<P0, P1>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: P0, pprofile: P1, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IRTCProfile>,
    {
        (windows_core::Interface::vtable(self).Redirect)(windows_core::Interface::as_raw(self), entype, bstrlocalphoneuri.param().abi(), pprofile.param().abi(), lflags).ok()
    }
    pub unsafe fn AddParticipant<P0, P1>(&self, bstraddress: P0, bstrname: P1) -> windows_core::Result<IRTCParticipant>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddParticipant)(windows_core::Interface::as_raw(self), bstraddress.param().abi(), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveParticipant<P0>(&self, pparticipant: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCParticipant>,
    {
        (windows_core::Interface::vtable(self).RemoveParticipant)(windows_core::Interface::as_raw(self), pparticipant.param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> windows_core::Result<IRTCEnumParticipants> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateParticipants)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CanAddParticipants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanAddParticipants)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RedirectedUserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectedUserURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RedirectedUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectedUserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NextRedirectedUser(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextRedirectedUser)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendMessage<P0, P1>(&self, bstrmessageheader: P0, bstrmessage: P1, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SendMessage)(windows_core::Interface::as_raw(self), bstrmessageheader.param().abi(), bstrmessage.param().abi(), lcookie).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendMessageStatus)(windows_core::Interface::as_raw(self), enuserstatus, lcookie).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), lmediatype, lcookie).ok()
    }
    pub unsafe fn put_EncryptionKey<P0>(&self, lmediatype: i32, encryptionkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_EncryptionKey)(windows_core::Interface::as_raw(self), lmediatype, encryptionkey.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRTCSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Client: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_STATE) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_TYPE) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Participants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Participants: usize,
    pub Answer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_TERMINATE_REASON) -> windows_core::HRESULT,
    pub Redirect: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddParticipant: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveParticipant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanAddParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RedirectedUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RedirectedUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NextRedirectedUser: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, isize) -> windows_core::HRESULT,
    pub SendMessageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_MESSAGING_USER_STATUS, isize) -> windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize) -> windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize) -> windows_core::HRESULT,
    pub put_EncryptionKey: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCSession2, IRTCSession2_Vtbl, 0x17d7cdfc_b007_484c_99d2_86a8a820991d);
impl core::ops::Deref for IRTCSession2 {
    type Target = IRTCSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSession2, windows_core::IUnknown, IRTCSession);
impl IRTCSession2 {
    pub unsafe fn SendInfo<P0, P1>(&self, bstrinfoheader: P0, bstrinfo: P1, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SendInfo)(windows_core::Interface::as_raw(self), bstrinfoheader.param().abi(), bstrinfo.param().abi(), lcookie).ok()
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok()
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
    }
    pub unsafe fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
    }
    pub unsafe fn AnswerWithSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AnswerWithSessionDescription)(windows_core::Interface::as_raw(self), bstrcontenttype.param().abi(), bstrsessiondescription.param().abi()).ok()
    }
    pub unsafe fn ReInviteWithSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ReInviteWithSessionDescription)(windows_core::Interface::as_raw(self), bstrcontenttype.param().abi(), bstrsessiondescription.param().abi(), lcookie).ok()
    }
}
#[repr(C)]
pub struct IRTCSession2_Vtbl {
    pub base__: IRTCSession_Vtbl,
    pub SendInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, isize) -> windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AnswerWithSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReInviteWithSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, isize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCSessionCallControl, IRTCSessionCallControl_Vtbl, 0xe9a50d94_190b_4f82_9530_3b8ebf60758a);
impl core::ops::Deref for IRTCSessionCallControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSessionCallControl, windows_core::IUnknown);
impl IRTCSessionCallControl {
    pub unsafe fn Hold(&self, lcookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Hold)(windows_core::Interface::as_raw(self), lcookie).ok()
    }
    pub unsafe fn UnHold(&self, lcookie: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnHold)(windows_core::Interface::as_raw(self), lcookie).ok()
    }
    pub unsafe fn Forward<P0>(&self, bstrforwardtouri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Forward)(windows_core::Interface::as_raw(self), bstrforwardtouri.param().abi()).ok()
    }
    pub unsafe fn Refer<P0, P1>(&self, bstrrefertouri: P0, bstrrefercookie: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Refer)(windows_core::Interface::as_raw(self), bstrrefertouri.param().abi(), bstrrefercookie.param().abi()).ok()
    }
    pub unsafe fn SetReferredByURI<P0>(&self, bstrreferredbyuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReferredByURI)(windows_core::Interface::as_raw(self), bstrreferredbyuri.param().abi()).ok()
    }
    pub unsafe fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferredByURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReferCookie<P0>(&self, bstrrefercookie: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReferCookie)(windows_core::Interface::as_raw(self), bstrrefercookie.param().abi()).ok()
    }
    pub unsafe fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferCookie)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsReferred(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsReferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCSessionCallControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Hold: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub UnHold: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub Forward: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Refer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsReferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCSessionDescriptionManager, IRTCSessionDescriptionManager_Vtbl, 0xba7f518e_d336_4070_93a6_865395c843f9);
impl core::ops::Deref for IRTCSessionDescriptionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSessionDescriptionManager, windows_core::IUnknown);
impl IRTCSessionDescriptionManager {
    pub unsafe fn EvaluateSessionDescription<P0, P1>(&self, bstrcontenttype: P0, bstrsessiondescription: P1, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EvaluateSessionDescription)(windows_core::Interface::as_raw(self), bstrcontenttype.param().abi(), bstrsessiondescription.param().abi(), pfapplicationsession).ok()
    }
}
#[repr(C)]
pub struct IRTCSessionDescriptionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EvaluateSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionOperationCompleteEvent, IRTCSessionOperationCompleteEvent_Vtbl, 0xa6bff4c0_f7c8_4d3c_9a41_3550f78a95b0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionOperationCompleteEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionOperationCompleteEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionOperationCompleteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionOperationCompleteEvent2, IRTCSessionOperationCompleteEvent2_Vtbl, 0xf6fc2a9b_d5bc_4241_b436_1b8460c13832);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionOperationCompleteEvent2 {
    type Target = IRTCSessionOperationCompleteEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionOperationCompleteEvent2, windows_core::IUnknown, super::Com::IDispatch, IRTCSessionOperationCompleteEvent);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent2 {
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionOperationCompleteEvent2_Vtbl {
    pub base__: IRTCSessionOperationCompleteEvent_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCSessionPortManagement, IRTCSessionPortManagement_Vtbl, 0xa072f1d6_0286_4e1f_85f2_17a2948456ec);
impl core::ops::Deref for IRTCSessionPortManagement {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSessionPortManagement, windows_core::IUnknown);
impl IRTCSessionPortManagement {
    pub unsafe fn SetPortManager<P0>(&self, pportmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCPortManager>,
    {
        (windows_core::Interface::vtable(self).SetPortManager)(windows_core::Interface::as_raw(self), pportmanager.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRTCSessionPortManagement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPortManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionReferStatusEvent, IRTCSessionReferStatusEvent_Vtbl, 0x3d8fc2cd_5d76_44ab_bb68_2a80353b34a2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionReferStatusEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionReferStatusEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferStatusEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReferStatus(&self) -> windows_core::Result<RTC_SESSION_REFER_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionReferStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_REFER_STATUS) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionReferredEvent, IRTCSessionReferredEvent_Vtbl, 0x176a6828_4fcc_4f28_a862_04597a6cf1c4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionReferredEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionReferredEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferredEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferredByURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReferToURI(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferToURI)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReferCookie)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Accept(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReferredSessionState)(windows_core::Interface::as_raw(self), enstate).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionReferredEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReferToURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferredSessionState: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_STATE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionStateChangeEvent, IRTCSessionStateChangeEvent_Vtbl, 0xb5bad703_5952_48b3_9321_7f4500521506);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionStateChangeEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionStateChangeEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent {
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_SESSION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCSessionStateChangeEvent2, IRTCSessionStateChangeEvent2_Vtbl, 0x4f933171_6f95_4880_80d9_2ec8d495d261);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCSessionStateChangeEvent2 {
    type Target = IRTCSessionStateChangeEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCSessionStateChangeEvent2, windows_core::IUnknown, super::Com::IDispatch, IRTCSessionStateChangeEvent);
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent2 {
    pub unsafe fn MediaTypes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
    }
    pub unsafe fn IsForked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsForked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCSessionStateChangeEvent2_Vtbl {
    pub base__: IRTCSessionStateChangeEvent_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub IsForked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCUserSearch, IRTCUserSearch_Vtbl, 0xb619882b_860c_4db4_be1b_693b6505bbe5);
impl core::ops::Deref for IRTCUserSearch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCUserSearch, windows_core::IUnknown);
impl IRTCUserSearch {
    pub unsafe fn CreateQuery(&self) -> windows_core::Result<IRTCUserSearchQuery> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExecuteSearch<P0, P1>(&self, pquery: P0, pprofile: P1, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCUserSearchQuery>,
        P1: windows_core::Param<IRTCProfile>,
    {
        (windows_core::Interface::vtable(self).ExecuteSearch)(windows_core::Interface::as_raw(self), pquery.param().abi(), pprofile.param().abi(), lcookie).ok()
    }
}
#[repr(C)]
pub struct IRTCUserSearch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCUserSearchQuery, IRTCUserSearchQuery_Vtbl, 0x288300f5_d23a_4365_9a73_9985c98c2881);
impl core::ops::Deref for IRTCUserSearchQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCUserSearchQuery, windows_core::IUnknown);
impl IRTCUserSearchQuery {
    pub unsafe fn put_SearchTerm<P0, P1>(&self, bstrname: P0, bstrvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_SearchTerm)(windows_core::Interface::as_raw(self), bstrname.param().abi(), bstrvalue.param().abi()).ok()
    }
    pub unsafe fn get_SearchTerm<P0>(&self, bstrname: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SearchTerm)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SearchTerms(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SearchTerms)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_SearchPreference)(windows_core::Interface::as_raw(self), enpreference, lvalue).ok()
    }
    pub unsafe fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_SearchPreference)(windows_core::Interface::as_raw(self), enpreference, &mut result__).map(|| result__)
    }
    pub unsafe fn SetSearchDomain<P0>(&self, bstrdomain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSearchDomain)(windows_core::Interface::as_raw(self), bstrdomain.param().abi()).ok()
    }
    pub unsafe fn SearchDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SearchDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCUserSearchQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_SearchTerm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_SearchTerm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SearchTerms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_SearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_PREFERENCE, i32) -> windows_core::HRESULT,
    pub get_SearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_PREFERENCE, *mut i32) -> windows_core::HRESULT,
    pub SetSearchDomain: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SearchDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCUserSearchResult, IRTCUserSearchResult_Vtbl, 0x851278b2_9592_480f_8db5_2de86b26b54d);
impl core::ops::Deref for IRTCUserSearchResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCUserSearchResult, windows_core::IUnknown);
impl IRTCUserSearchResult {
    pub unsafe fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), encolumn, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRTCUserSearchResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_COLUMN, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCUserSearchResultsEvent, IRTCUserSearchResultsEvent_Vtbl, 0xd8c8c3cd_7fac_4088_81c5_c24cbc0938e3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCUserSearchResultsEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCUserSearchResultsEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCUserSearchResultsEvent {
    pub unsafe fn EnumerateResults(&self) -> windows_core::Result<IRTCEnumUserSearchResults> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> windows_core::Result<IRTCCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Query(&self) -> windows_core::Result<IRTCUserSearchQuery> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoreAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoreAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCUserSearchResultsEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EnumerateResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MoreAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCWatcher, IRTCWatcher_Vtbl, 0xc7cedad8_346b_4d1b_ac02_a2088df9be4f);
impl core::ops::Deref for IRTCWatcher {
    type Target = IRTCPresenceContact;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCWatcher, windows_core::IUnknown, IRTCPresenceContact);
impl IRTCWatcher {
    pub unsafe fn State(&self) -> windows_core::Result<RTC_WATCHER_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), enstate).ok()
    }
}
#[repr(C)]
pub struct IRTCWatcher_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_WATCHER_STATE) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_WATCHER_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRTCWatcher2, IRTCWatcher2_Vtbl, 0xd4d9967f_d011_4b1d_91e3_aba78f96393d);
impl core::ops::Deref for IRTCWatcher2 {
    type Target = IRTCWatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCWatcher2, windows_core::IUnknown, IRTCPresenceContact, IRTCWatcher);
impl IRTCWatcher2 {
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<RTC_ACE_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRTCWatcher2_Vtbl {
    pub base__: IRTCWatcher_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_ACE_SCOPE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCWatcherEvent, IRTCWatcherEvent_Vtbl, 0xf30d7261_587a_424f_822c_312788f43548);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCWatcherEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCWatcherEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent {
    pub unsafe fn Watcher(&self) -> windows_core::Result<IRTCWatcher> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Watcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCWatcherEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Watcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRTCWatcherEvent2, IRTCWatcherEvent2_Vtbl, 0xe52891e8_188c_49af_b005_98ed13f83f9c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRTCWatcherEvent2 {
    type Target = IRTCWatcherEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRTCWatcherEvent2, windows_core::IUnknown, super::Com::IDispatch, IRTCWatcherEvent);
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent2 {
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_WATCHER_EVENT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRTCWatcherEvent2_Vtbl {
    pub base__: IRTCWatcherEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_WATCHER_EVENT_TYPE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITransportSettingsInternal, ITransportSettingsInternal_Vtbl, 0x5123e076_29e3_4bfd_84fe_0192d411e3e8);
impl core::ops::Deref for ITransportSettingsInternal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransportSettingsInternal, windows_core::IUnknown);
impl ITransportSettingsInternal {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplySetting)(windows_core::Interface::as_raw(self), setting).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QuerySetting)(windows_core::Interface::as_raw(self), setting).ok()
    }
}
#[repr(C)]
pub struct ITransportSettingsInternal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub ApplySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TRANSPORT_SETTING) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    ApplySetting: usize,
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub QuerySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TRANSPORT_SETTING) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))]
    QuerySetting: usize,
}
pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(1i32);
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(0i32);
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(1i32);
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(2i32);
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = RTC_ANSWER_MODE(3i32);
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(0i32);
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = RTC_ACE_SCOPE(2i32);
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = RTC_ACE_SCOPE(1i32);
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = RTC_ACE_SCOPE(0i32);
pub const RTCAU_BASIC: u32 = 1u32;
pub const RTCAU_DIGEST: u32 = 2u32;
pub const RTCAU_KERBEROS: u32 = 8u32;
pub const RTCAU_NTLM: u32 = 4u32;
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(0i32);
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(1i32);
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(4i32);
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(3i32);
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(5i32);
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(2i32);
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(1i32);
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(2i32);
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(3i32);
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(0i32);
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(3i32);
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(1i32);
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(2i32);
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(0i32);
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub const RTCEF_ALL: u32 = 33554431u32;
pub const RTCEF_BUDDY: u32 = 256u32;
pub const RTCEF_BUDDY2: u32 = 262144u32;
pub const RTCEF_CLIENT: u32 = 1u32;
pub const RTCEF_GROUP: u32 = 8192u32;
pub const RTCEF_INFO: u32 = 4096u32;
pub const RTCEF_INTENSITY: u32 = 64u32;
pub const RTCEF_MEDIA: u32 = 32u32;
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
pub const RTCEF_MESSAGING: u32 = 128u32;
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
pub const RTCEF_PROFILE: u32 = 1024u32;
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
pub const RTCEF_REINVITE: u32 = 4194304u32;
pub const RTCEF_ROAMING: u32 = 65536u32;
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
pub const RTCEF_USERSEARCH: u32 = 2048u32;
pub const RTCEF_WATCHER: u32 = 512u32;
pub const RTCEF_WATCHER2: u32 = 524288u32;
pub const RTCE_BUDDY: RTC_EVENT = RTC_EVENT(8i32);
pub const RTCE_CLIENT: RTC_EVENT = RTC_EVENT(0i32);
pub const RTCE_GROUP: RTC_EVENT = RTC_EVENT(13i32);
pub const RTCE_INFO: RTC_EVENT = RTC_EVENT(12i32);
pub const RTCE_INTENSITY: RTC_EVENT = RTC_EVENT(6i32);
pub const RTCE_MEDIA: RTC_EVENT = RTC_EVENT(5i32);
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = RTC_EVENT(14i32);
pub const RTCE_MESSAGING: RTC_EVENT = RTC_EVENT(7i32);
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = RTC_EVENT(4i32);
pub const RTCE_PRESENCE_DATA: RTC_EVENT = RTC_EVENT(17i32);
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = RTC_EVENT(16i32);
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = RTC_EVENT(18i32);
pub const RTCE_PROFILE: RTC_EVENT = RTC_EVENT(10i32);
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(1i32);
pub const RTCE_REINVITE: RTC_EVENT = RTC_EVENT(21i32);
pub const RTCE_ROAMING: RTC_EVENT = RTC_EVENT(15i32);
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = RTC_EVENT(3i32);
pub const RTCE_SESSION_REFERRED: RTC_EVENT = RTC_EVENT(20i32);
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = RTC_EVENT(19i32);
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(2i32);
pub const RTCE_USERSEARCH: RTC_EVENT = RTC_EVENT(11i32);
pub const RTCE_WATCHER: RTC_EVENT = RTC_EVENT(9i32);
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(0i32);
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(3i32);
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(4i32);
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(1i32);
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(5i32);
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(2i32);
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
pub const RTCLM_BOTH: RTC_LISTEN_MODE = RTC_LISTEN_MODE(2i32);
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = RTC_LISTEN_MODE(1i32);
pub const RTCLM_NONE: RTC_LISTEN_MODE = RTC_LISTEN_MODE(0i32);
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(3i32);
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(1i32);
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(0i32);
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(4i32);
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(5i32);
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(6i32);
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(2i32);
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(2i32);
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(1i32);
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(0i32);
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(0i32);
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(1i32);
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(0i32);
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(1i32);
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(1i32);
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(0i32);
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(0i32);
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(1i32);
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(1i32);
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(0i32);
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(3i32);
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(1i32);
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(2i32);
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(4i32);
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(0i32);
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(5i32);
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(3i32);
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(6i32);
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(8i32);
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(7i32);
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(0i32);
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(2i32);
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(4i32);
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(1i32);
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(1i32);
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(0i32);
pub const RTCPT_SIP: RTC_PORT_TYPE = RTC_PORT_TYPE(4i32);
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(3i32);
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(2i32);
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = RTC_PROVIDER_URI(3i32);
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(4i32);
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = RTC_PROVIDER_URI(1i32);
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(0i32);
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = RTC_PROVIDER_URI(2i32);
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(0i32);
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(2i32);
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(3i32);
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(1i32);
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(4i32);
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = RTC_REINVITE_STATE(2i32);
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = RTC_REINVITE_STATE(0i32);
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = RTC_REINVITE_STATE(1i32);
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(5i32);
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(7i32);
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(6i32);
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(0i32);
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(2i32);
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(1i32);
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(3i32);
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(8i32);
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(4i32);
pub const RTCRT_MESSAGE: RTC_RING_TYPE = RTC_RING_TYPE(1i32);
pub const RTCRT_PHONE: RTC_RING_TYPE = RTC_RING_TYPE(0i32);
pub const RTCRT_RINGBACK: RTC_RING_TYPE = RTC_RING_TYPE(2i32);
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(3i32);
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(2i32);
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(1i32);
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(0i32);
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(1i32);
pub const RTCSI_APPLICATION: u32 = 32u32;
pub const RTCSI_IM: u32 = 8u32;
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
pub const RTCSI_PC_TO_PC: u32 = 1u32;
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(1i32);
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(5i32);
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(4i32);
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(2i32);
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(0i32);
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(3i32);
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = RTC_SESSION_STATE(2i32);
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(4i32);
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(5i32);
pub const RTCSS_HOLD: RTC_SESSION_STATE = RTC_SESSION_STATE(6i32);
pub const RTCSS_IDLE: RTC_SESSION_STATE = RTC_SESSION_STATE(0i32);
pub const RTCSS_INCOMING: RTC_SESSION_STATE = RTC_SESSION_STATE(1i32);
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = RTC_SESSION_STATE(3i32);
pub const RTCSS_REFER: RTC_SESSION_STATE = RTC_SESSION_STATE(7i32);
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = RTC_SESSION_TYPE(5i32);
pub const RTCST_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(3i32);
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(4i32);
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = RTC_SESSION_TYPE(0i32);
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(1i32);
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(2i32);
pub const RTCTA_APPSHARING: RTC_T120_APPLET = RTC_T120_APPLET(1i32);
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = RTC_T120_APPLET(0i32);
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(2i32);
pub const RTCTR_DND: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(1i32);
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(6i32);
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(0i32);
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(7i32);
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(3i32);
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(5i32);
pub const RTCTR_TCP: u32 = 2u32;
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(4i32);
pub const RTCTR_TLS: u32 = 4u32;
pub const RTCTR_UDP: u32 = 1u32;
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(6i32);
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(5i32);
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(8i32);
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(1i32);
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(9i32);
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(3i32);
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(4i32);
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(7i32);
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(2i32);
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(0i32);
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(0i32);
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(1i32);
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(1i32);
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(0i32);
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(0i32);
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(3i32);
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(1i32);
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(4i32);
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(2i32);
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(1i32);
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(0i32);
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(2i32);
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(3i32);
pub const RTCWS_DENIED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(4i32);
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = RTC_WATCHER_STATE(1i32);
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = RTC_WATCHER_STATE(5i32);
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = RTC_WATCHER_STATE(0i32);
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(2i32);
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(5i32);
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(4i32);
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(3i32);
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(0i32);
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(1i32);
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(6i32);
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(7i32);
pub const RTC_DTMF_0: RTC_DTMF = RTC_DTMF(0i32);
pub const RTC_DTMF_1: RTC_DTMF = RTC_DTMF(1i32);
pub const RTC_DTMF_2: RTC_DTMF = RTC_DTMF(2i32);
pub const RTC_DTMF_3: RTC_DTMF = RTC_DTMF(3i32);
pub const RTC_DTMF_4: RTC_DTMF = RTC_DTMF(4i32);
pub const RTC_DTMF_5: RTC_DTMF = RTC_DTMF(5i32);
pub const RTC_DTMF_6: RTC_DTMF = RTC_DTMF(6i32);
pub const RTC_DTMF_7: RTC_DTMF = RTC_DTMF(7i32);
pub const RTC_DTMF_8: RTC_DTMF = RTC_DTMF(8i32);
pub const RTC_DTMF_9: RTC_DTMF = RTC_DTMF(9i32);
pub const RTC_DTMF_A: RTC_DTMF = RTC_DTMF(12i32);
pub const RTC_DTMF_B: RTC_DTMF = RTC_DTMF(13i32);
pub const RTC_DTMF_C: RTC_DTMF = RTC_DTMF(14i32);
pub const RTC_DTMF_D: RTC_DTMF = RTC_DTMF(15i32);
pub const RTC_DTMF_FLASH: RTC_DTMF = RTC_DTMF(16i32);
pub const RTC_DTMF_POUND: RTC_DTMF = RTC_DTMF(11i32);
pub const RTC_DTMF_STAR: RTC_DTMF = RTC_DTMF(10i32);
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0077_u32 as _);
pub const RTC_E_BASIC_AUTH_SET_TLS: windows_core::HRESULT = windows_core::HRESULT(0x80EE003F_u32 as _);
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0026_u32 as _);
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: windows_core::HRESULT = windows_core::HRESULT(0x80EE0027_u32 as _);
pub const RTC_E_CLIENT_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0025_u32 as _);
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0013_u32 as _);
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0015_u32 as _);
pub const RTC_E_DUPLICATE_BUDDY: windows_core::HRESULT = windows_core::HRESULT(0x80EE004A_u32 as _);
pub const RTC_E_DUPLICATE_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80EE0052_u32 as _);
pub const RTC_E_DUPLICATE_REALM: windows_core::HRESULT = windows_core::HRESULT(0x80EE0043_u32 as _);
pub const RTC_E_DUPLICATE_WATCHER: windows_core::HRESULT = windows_core::HRESULT(0x80EE004B_u32 as _);
pub const RTC_E_INVALID_ACL_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0050_u32 as _);
pub const RTC_E_INVALID_ADDRESS_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0014_u32 as _);
pub const RTC_E_INVALID_BUDDY_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE004F_u32 as _);
pub const RTC_E_INVALID_LISTEN_SOCKET: windows_core::HRESULT = windows_core::HRESULT(0x80EE007B_u32 as _);
pub const RTC_E_INVALID_OBJECT_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0061_u32 as _);
pub const RTC_E_INVALID_PORTRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80EE005C_u32 as _);
pub const RTC_E_INVALID_PREFERENCE_LIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0059_u32 as _);
pub const RTC_E_INVALID_PROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80EE002E_u32 as _);
pub const RTC_E_INVALID_PROXY_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0016_u32 as _);
pub const RTC_E_INVALID_REGISTRATION_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80EE006D_u32 as _);
pub const RTC_E_INVALID_SESSION_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80EE002A_u32 as _);
pub const RTC_E_INVALID_SESSION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0029_u32 as _);
pub const RTC_E_INVALID_SIP_URL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0012_u32 as _);
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE007A_u32 as _);
pub const RTC_E_LOCAL_PHONE_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80EE002C_u32 as _);
pub const RTC_E_MALFORMED_XML: windows_core::HRESULT = windows_core::HRESULT(0x80EE004C_u32 as _);
pub const RTC_E_MAX_PENDING_OPERATIONS: windows_core::HRESULT = windows_core::HRESULT(0x80EE005A_u32 as _);
pub const RTC_E_MAX_REDIRECTS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0078_u32 as _);
pub const RTC_E_MEDIA_AEC: windows_core::HRESULT = windows_core::HRESULT(0x80EE0024_u32 as _);
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0021_u32 as _);
pub const RTC_E_MEDIA_CONTROLLER_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80EE001F_u32 as _);
pub const RTC_E_MEDIA_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80EE006E_u32 as _);
pub const RTC_E_MEDIA_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80EE006F_u32 as _);
pub const RTC_E_MEDIA_NEED_TERMINAL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0020_u32 as _);
pub const RTC_E_MEDIA_SESSION_IN_HOLD: windows_core::HRESULT = windows_core::HRESULT(0x80EE0076_u32 as _);
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0075_u32 as _);
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0022_u32 as _);
pub const RTC_E_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0082_u32 as _);
pub const RTC_E_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0058_u32 as _);
pub const RTC_E_NOT_PRESENCE_PROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80EE006A_u32 as _);
pub const RTC_E_NO_BUDDY: windows_core::HRESULT = windows_core::HRESULT(0x80EE0054_u32 as _);
pub const RTC_E_NO_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x80EE002D_u32 as _);
pub const RTC_E_NO_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80EE0051_u32 as _);
pub const RTC_E_NO_PROFILE: windows_core::HRESULT = windows_core::HRESULT(0x80EE002B_u32 as _);
pub const RTC_E_NO_REALM: windows_core::HRESULT = windows_core::HRESULT(0x80EE0056_u32 as _);
pub const RTC_E_NO_TRANSPORT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0057_u32 as _);
pub const RTC_E_NO_WATCHER: windows_core::HRESULT = windows_core::HRESULT(0x80EE0055_u32 as _);
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: windows_core::HRESULT = windows_core::HRESULT(0x80EE003E_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80F00007_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: windows_core::HRESULT = windows_core::HRESULT(0x80F0000B_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80F00005_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80F0000A_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: windows_core::HRESULT = windows_core::HRESULT(0x80F00006_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80F00008_u32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80F00009_u32 as _);
pub const RTC_E_PLATFORM_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0080_u32 as _);
pub const RTC_E_POLICY_NOT_ALLOW: windows_core::HRESULT = windows_core::HRESULT(0x80EE0044_u32 as _);
pub const RTC_E_PORT_MANAGER_ALREADY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80EE007C_u32 as _);
pub const RTC_E_PORT_MAPPING_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0046_u32 as _);
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0045_u32 as _);
pub const RTC_E_PRESENCE_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0062_u32 as _);
pub const RTC_E_PRESENCE_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0028_u32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: windows_core::HRESULT = windows_core::HRESULT(0x80EE0038_u32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0037_u32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0039_u32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x80EE003B_u32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: windows_core::HRESULT = windows_core::HRESULT(0x80EE003C_u32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80EE003D_u32 as _);
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: windows_core::HRESULT = windows_core::HRESULT(0x80EE003A_u32 as _);
pub const RTC_E_PROFILE_NO_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80EE0030_u32 as _);
pub const RTC_E_PROFILE_NO_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80EE0031_u32 as _);
pub const RTC_E_PROFILE_NO_PROVISION: windows_core::HRESULT = windows_core::HRESULT(0x80EE002F_u32 as _);
pub const RTC_E_PROFILE_NO_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80EE0034_u32 as _);
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0035_u32 as _);
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0036_u32 as _);
pub const RTC_E_PROFILE_NO_USER: windows_core::HRESULT = windows_core::HRESULT(0x80EE0032_u32 as _);
pub const RTC_E_PROFILE_NO_USER_URI: windows_core::HRESULT = windows_core::HRESULT(0x80EE0033_u32 as _);
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0042_u32 as _);
pub const RTC_E_REDIRECT_PROCESSING_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0079_u32 as _);
pub const RTC_E_REFER_NOT_ACCEPTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0070_u32 as _);
pub const RTC_E_REFER_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0071_u32 as _);
pub const RTC_E_REFER_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0072_u32 as _);
pub const RTC_E_REGISTRATION_DEACTIVATED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0083_u32 as _);
pub const RTC_E_REGISTRATION_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0084_u32 as _);
pub const RTC_E_REGISTRATION_UNREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0085_u32 as _);
pub const RTC_E_ROAMING_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0063_u32 as _);
pub const RTC_E_ROAMING_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE004E_u32 as _);
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE004D_u32 as _);
pub const RTC_E_SDP_CONNECTION_ADDR: windows_core::HRESULT = windows_core::HRESULT(0x80EE000A_u32 as _);
pub const RTC_E_SDP_FAILED_TO_BUILD: windows_core::HRESULT = windows_core::HRESULT(0x80EE000D_u32 as _);
pub const RTC_E_SDP_MULTICAST: windows_core::HRESULT = windows_core::HRESULT(0x80EE0009_u32 as _);
pub const RTC_E_SDP_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0006_u32 as _);
pub const RTC_E_SDP_NO_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x80EE000B_u32 as _);
pub const RTC_E_SDP_PARSE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0007_u32 as _);
pub const RTC_E_SDP_UPDATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0008_u32 as _);
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: windows_core::HRESULT = windows_core::HRESULT(0x80EE007D_u32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0047_u32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0048_u32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0049_u32 as _);
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x80EE005E_u32 as _);
pub const RTC_E_SIP_AUTH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0011_u32 as _);
pub const RTC_E_SIP_AUTH_HEADER_SENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE000F_u32 as _);
pub const RTC_E_SIP_AUTH_TIME_SKEW: windows_core::HRESULT = windows_core::HRESULT(0x80EE006C_u32 as _);
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0010_u32 as _);
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: windows_core::HRESULT = windows_core::HRESULT(0x80EE005D_u32 as _);
pub const RTC_E_SIP_CALL_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0019_u32 as _);
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: windows_core::HRESULT = windows_core::HRESULT(0x80EE0000_u32 as _);
pub const RTC_E_SIP_DNS_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0066_u32 as _);
pub const RTC_E_SIP_HEADER_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0005_u32 as _);
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0040_u32 as _);
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80EE0073_u32 as _);
pub const RTC_E_SIP_INVALID_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0065_u32 as _);
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80EE006B_u32 as _);
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80EE000E_u32 as _);
pub const RTC_E_SIP_NEED_MORE_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80EE0018_u32 as _);
pub const RTC_E_SIP_NO_STREAM: windows_core::HRESULT = windows_core::HRESULT(0x80EE0003_u32 as _);
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0060_u32 as _);
pub const RTC_E_SIP_PARSE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0004_u32 as _);
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x80EE005F_u32 as _);
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x80EE0081_u32 as _);
pub const RTC_E_SIP_REFER_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80EE007F_u32 as _);
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE001A_u32 as _);
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80EE001D_u32 as _);
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80EE001C_u32 as _);
pub const RTC_E_SIP_STACK_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80EE001E_u32 as _);
pub const RTC_E_SIP_STREAM_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0002_u32 as _);
pub const RTC_E_SIP_STREAM_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80EE0001_u32 as _);
pub const RTC_E_SIP_TCP_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0067_u32 as _);
pub const RTC_E_SIP_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80EE000C_u32 as _);
pub const RTC_E_SIP_TLS_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80EE0069_u32 as _);
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: windows_core::HRESULT = windows_core::HRESULT(0x80EE0064_u32 as _);
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE0017_u32 as _);
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80EE001B_u32 as _);
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x80EE0074_u32 as _);
pub const RTC_E_START_STREAM: windows_core::HRESULT = windows_core::HRESULT(0x80EE0023_u32 as _);
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E4_u32 as _);
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E5_u32 as _);
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: windows_core::HRESULT = windows_core::HRESULT(0x80EF01A4_u32 as _);
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80EF0190_u32 as _);
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E6_u32 as _);
pub const RTC_E_STATUS_CLIENT_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80EF0199_u32 as _);
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80EF0193_u32 as _);
pub const RTC_E_STATUS_CLIENT_GONE: windows_core::HRESULT = windows_core::HRESULT(0x80EF019A_u32 as _);
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80EF019B_u32 as _);
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E2_u32 as _);
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80EF0195_u32 as _);
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EF0196_u32 as _);
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80EF0194_u32 as _);
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80EF0192_u32 as _);
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80EF0197_u32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80EF019D_u32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80EF0198_u32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80EF019E_u32 as _);
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E0_u32 as _);
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E3_u32 as _);
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E1_u32 as _);
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x80EF0191_u32 as _);
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80EF019F_u32 as _);
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: windows_core::HRESULT = windows_core::HRESULT(0x80EF0258_u32 as _);
pub const RTC_E_STATUS_GLOBAL_DECLINE: windows_core::HRESULT = windows_core::HRESULT(0x80EF025B_u32 as _);
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: windows_core::HRESULT = windows_core::HRESULT(0x80EF025C_u32 as _);
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EF025E_u32 as _);
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: windows_core::HRESULT = windows_core::HRESULT(0xEF00B5_u32 as _);
pub const RTC_E_STATUS_INFO_QUEUED: windows_core::HRESULT = windows_core::HRESULT(0xEF00B6_u32 as _);
pub const RTC_E_STATUS_INFO_RINGING: windows_core::HRESULT = windows_core::HRESULT(0xEF00B4_u32 as _);
pub const RTC_E_STATUS_INFO_TRYING: windows_core::HRESULT = windows_core::HRESULT(0xEF0064_u32 as _);
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E8_u32 as _);
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80EF017C_u32 as _);
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: windows_core::HRESULT = windows_core::HRESULT(0x80EF012D_u32 as _);
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: windows_core::HRESULT = windows_core::HRESULT(0x80EF012E_u32 as _);
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: windows_core::HRESULT = windows_core::HRESULT(0x80EF012C_u32 as _);
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: windows_core::HRESULT = windows_core::HRESULT(0x80EF012F_u32 as _);
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: windows_core::HRESULT = windows_core::HRESULT(0x80EF0131_u32 as _);
pub const RTC_E_STATUS_REQUEST_TERMINATED: windows_core::HRESULT = windows_core::HRESULT(0x80EF01E7_u32 as _);
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F6_u32 as _);
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F4_u32 as _);
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F5_u32 as _);
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F8_u32 as _);
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F7_u32 as _);
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80EF01F9_u32 as _);
pub const RTC_E_STATUS_SESSION_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xEF00B7_u32 as _);
pub const RTC_E_STATUS_SUCCESS: windows_core::HRESULT = windows_core::HRESULT(0xEF00C8_u32 as _);
pub const RTC_E_TOO_MANY_GROUPS: windows_core::HRESULT = windows_core::HRESULT(0x80EE0053_u32 as _);
pub const RTC_E_TOO_MANY_RETRIES: windows_core::HRESULT = windows_core::HRESULT(0x80EE005B_u32 as _);
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80EE0068_u32 as _);
pub const RTC_E_UDP_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80EE007E_u32 as _);
pub const RTC_S_ROAMING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xEE0041_u32 as _);
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_ACE_SCOPE(pub i32);
impl windows_core::TypeKind for RTC_ACE_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_ACE_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_ACE_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_ANSWER_MODE(pub i32);
impl windows_core::TypeKind for RTC_ANSWER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_ANSWER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_ANSWER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_AUDIO_DEVICE(pub i32);
impl windows_core::TypeKind for RTC_AUDIO_DEVICE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_AUDIO_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_AUDIO_DEVICE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_BUDDY_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_BUDDY_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_BUDDY_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(pub i32);
impl windows_core::TypeKind for RTC_BUDDY_SUBSCRIPTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_BUDDY_SUBSCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_CLIENT_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_CLIENT_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_CLIENT_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_CLIENT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_DTMF(pub i32);
impl windows_core::TypeKind for RTC_DTMF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_DTMF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_DTMF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_EVENT(pub i32);
impl windows_core::TypeKind for RTC_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_GROUP_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_GROUP_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_LISTEN_MODE(pub i32);
impl windows_core::TypeKind for RTC_LISTEN_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_LISTEN_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_LISTEN_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_MEDIA_EVENT_REASON(pub i32);
impl windows_core::TypeKind for RTC_MEDIA_EVENT_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_MEDIA_EVENT_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_MEDIA_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_MEDIA_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_MEDIA_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_MEDIA_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_MESSAGING_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_MESSAGING_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_MESSAGING_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_MESSAGING_USER_STATUS(pub i32);
impl windows_core::TypeKind for RTC_MESSAGING_USER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_MESSAGING_USER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_MESSAGING_USER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_OFFER_WATCHER_MODE(pub i32);
impl windows_core::TypeKind for RTC_OFFER_WATCHER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_OFFER_WATCHER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_OFFER_WATCHER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PARTICIPANT_STATE(pub i32);
impl windows_core::TypeKind for RTC_PARTICIPANT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PARTICIPANT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PARTICIPANT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PORT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_PORT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PORT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PRESENCE_PROPERTY(pub i32);
impl windows_core::TypeKind for RTC_PRESENCE_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PRESENCE_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PRESENCE_STATUS(pub i32);
impl windows_core::TypeKind for RTC_PRESENCE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PRESENCE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PRESENCE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PRIVACY_MODE(pub i32);
impl windows_core::TypeKind for RTC_PRIVACY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PRIVACY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PRIVACY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PROFILE_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_PROFILE_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PROFILE_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PROFILE_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_PROVIDER_URI(pub i32);
impl windows_core::TypeKind for RTC_PROVIDER_URI {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_PROVIDER_URI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_PROVIDER_URI").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_REGISTRATION_STATE(pub i32);
impl windows_core::TypeKind for RTC_REGISTRATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_REGISTRATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_REGISTRATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_REINVITE_STATE(pub i32);
impl windows_core::TypeKind for RTC_REINVITE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_REINVITE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_REINVITE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_RING_TYPE(pub i32);
impl windows_core::TypeKind for RTC_RING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_RING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_RING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_ROAMING_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_ROAMING_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_ROAMING_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_ROAMING_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_SECURITY_LEVEL(pub i32);
impl windows_core::TypeKind for RTC_SECURITY_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_SECURITY_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_SECURITY_TYPE(pub i32);
impl windows_core::TypeKind for RTC_SECURITY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_SECURITY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_SECURITY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_SESSION_REFER_STATUS(pub i32);
impl windows_core::TypeKind for RTC_SESSION_REFER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_SESSION_REFER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_SESSION_REFER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_SESSION_STATE(pub i32);
impl windows_core::TypeKind for RTC_SESSION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_SESSION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_SESSION_TYPE(pub i32);
impl windows_core::TypeKind for RTC_SESSION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_SESSION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_SESSION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_T120_APPLET(pub i32);
impl windows_core::TypeKind for RTC_T120_APPLET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_T120_APPLET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_T120_APPLET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_TERMINATE_REASON(pub i32);
impl windows_core::TypeKind for RTC_TERMINATE_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_TERMINATE_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_TERMINATE_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_USER_SEARCH_COLUMN(pub i32);
impl windows_core::TypeKind for RTC_USER_SEARCH_COLUMN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_USER_SEARCH_COLUMN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_COLUMN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_USER_SEARCH_PREFERENCE(pub i32);
impl windows_core::TypeKind for RTC_USER_SEARCH_PREFERENCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_USER_SEARCH_PREFERENCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_USER_SEARCH_PREFERENCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_VIDEO_DEVICE(pub i32);
impl windows_core::TypeKind for RTC_VIDEO_DEVICE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_VIDEO_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_VIDEO_DEVICE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_WATCHER_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTC_WATCHER_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_WATCHER_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_WATCHER_MATCH_MODE(pub i32);
impl windows_core::TypeKind for RTC_WATCHER_MATCH_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_WATCHER_MATCH_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_MATCH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTC_WATCHER_STATE(pub i32);
impl windows_core::TypeKind for RTC_WATCHER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTC_WATCHER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTC_WATCHER_STATE").field(&self.0).finish()
    }
}
pub const RTCClient: windows_core::GUID = windows_core::GUID::from_u128(0x7a42ea29_a2b7_40c4_b091_f6f024aa89be);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSPORT_SETTING {
    pub SettingId: super::super::Networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for TRANSPORT_SETTING {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
