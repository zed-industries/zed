pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
windows_core::imp::define_interface!(INetworkTransportSettings, INetworkTransportSettings_Vtbl, 0x5e7abb2c_f2c1_4a61_bd35_deb7a08ab0f1);
windows_core::imp::interface_hierarchy!(INetworkTransportSettings, windows_core::IUnknown);
impl INetworkTransportSettings {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplySetting)(windows_core::Interface::as_raw(self), settingid, valuein.len().try_into().unwrap(), core::mem::transmute(valuein.as_ptr()), lengthout as _, valueout as _).ok() }
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, valuein: &[u8], lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QuerySetting)(windows_core::Interface::as_raw(self), settingid, valuein.len().try_into().unwrap(), core::mem::transmute(valuein.as_ptr()), lengthout as _, valueout as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettings_Impl: windows_core::IUnknownImpl {
    fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()>;
    fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettings_Vtbl {
    pub const fn new<Identity: INetworkTransportSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplySetting<Identity: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkTransportSettings_Impl::ApplySetting(this, core::mem::transmute_copy(&settingid), core::mem::transmute_copy(&lengthin), core::mem::transmute_copy(&valuein), core::mem::transmute_copy(&lengthout), core::mem::transmute_copy(&valueout)).into()
            }
        }
        unsafe extern "system" fn QuerySetting<Identity: INetworkTransportSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkTransportSettings_Impl::QuerySetting(this, core::mem::transmute_copy(&settingid), core::mem::transmute_copy(&lengthin), core::mem::transmute_copy(&valuein), core::mem::transmute_copy(&lengthout), core::mem::transmute_copy(&valueout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplySetting: ApplySetting::<Identity, OFFSET>,
            QuerySetting: QuerySetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkTransportSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::RuntimeName for INetworkTransportSettings {}
windows_core::imp::define_interface!(INotificationTransportSync, INotificationTransportSync_Vtbl, 0x79eb1402_0ab8_49c0_9e14_a1ae4ba93058);
windows_core::imp::interface_hierarchy!(INotificationTransportSync, windows_core::IUnknown);
impl INotificationTransportSync {
    pub unsafe fn CompleteDelivery(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteDelivery)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationTransportSync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteDelivery: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait INotificationTransportSync_Impl: windows_core::IUnknownImpl {
    fn CompleteDelivery(&self) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl INotificationTransportSync_Vtbl {
    pub const fn new<Identity: INotificationTransportSync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteDelivery<Identity: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INotificationTransportSync_Impl::CompleteDelivery(this).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: INotificationTransportSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INotificationTransportSync_Impl::Flush(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompleteDelivery: CompleteDelivery::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INotificationTransportSync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INotificationTransportSync {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Notes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Notes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCBuddy_Impl: IRTCPresenceContact_Impl {
    fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IRTCBuddy_Vtbl {
    pub const fn new<Identity: IRTCBuddy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy_Impl::Status(this) {
                    Ok(ok__) => {
                        penstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notes<Identity: IRTCBuddy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnotes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy_Impl::Notes(this) {
                    Ok(ok__) => {
                        pbstrnotes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRTCPresenceContact_Vtbl::new::<Identity, OFFSET>(), Status: Status::<Identity, OFFSET>, Notes: Notes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddy as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCBuddy {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumeratePresenceDevices(&self) -> windows_core::Result<IRTCEnumPresenceDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumeratePresenceDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PresenceDevices(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PresenceDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SubscriptionType(&self) -> windows_core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubscriptionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy2_Vtbl {
    pub base__: IRTCBuddy_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumeratePresenceDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PresenceDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PresenceDevices: usize,
    pub SubscriptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddy2_Impl: IRTCBuddy_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups>;
    fn Groups(&self) -> windows_core::Result<IRTCCollection>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR>;
    fn EnumeratePresenceDevices(&self) -> windows_core::Result<IRTCEnumPresenceDevices>;
    fn PresenceDevices(&self) -> windows_core::Result<IRTCCollection>;
    fn SubscriptionType(&self) -> windows_core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddy2_Vtbl {
    pub const fn new<Identity: IRTCBuddy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Profile<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCBuddy2_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn EnumerateGroups<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::EnumerateGroups(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Groups<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::Groups(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                    Ok(ok__) => {
                        pbstrproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::EnumeratePresenceDevices(this) {
                    Ok(ok__) => {
                        ppenumdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PresenceDevices<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevicescollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::PresenceDevices(this) {
                    Ok(ok__) => {
                        ppdevicescollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubscriptionType<Identity: IRTCBuddy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddy2_Impl::SubscriptionType(this) {
                    Ok(ok__) => {
                        pensubscriptiontype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRTCBuddy_Vtbl::new::<Identity, OFFSET>(),
            Profile: Profile::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, OFFSET>,
            EnumeratePresenceDevices: EnumeratePresenceDevices::<Identity, OFFSET>,
            PresenceDevices: PresenceDevices::<Identity, OFFSET>,
            SubscriptionType: SubscriptionType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddy2 as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID || iid == &<IRTCBuddy as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddy2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buddy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyEvent_Impl: super::Com::IDispatch_Impl {
    fn Buddy(&self) -> windows_core::Result<IRTCBuddy>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCBuddyEvent_Vtbl {
    pub const fn new<Identity: IRTCBuddyEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buddy<Identity: IRTCBuddyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyEvent_Impl::Buddy(this) {
                    Ok(ok__) => {
                        ppbuddy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Buddy: Buddy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCBuddyEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent2_Vtbl {
    pub base__: IRTCBuddyEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_BUDDY_EVENT_TYPE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyEvent2_Impl: IRTCBuddyEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCBuddyEvent2_Vtbl {
    pub const fn new<Identity: IRTCBuddyEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyEvent2_Impl::EventType(this) {
                    Ok(ok__) => {
                        peventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyEvent2_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCBuddyEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyEvent2_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRTCBuddyEvent_Vtbl::new::<Identity, OFFSET>(),
            EventType: EventType::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddyEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCBuddyEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCBuddyEvent2 {}
windows_core::imp::define_interface!(IRTCBuddyGroup, IRTCBuddyGroup_Vtbl, 0x60361e68_9164_4389_a4c6_d0b3925bda5e);
windows_core::imp::interface_hierarchy!(IRTCBuddyGroup, windows_core::IUnknown);
impl IRTCBuddyGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname)).ok() }
    }
    pub unsafe fn AddBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok() }
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok() }
    }
    pub unsafe fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateBuddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdata)).ok() }
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateBuddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroup_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddBuddy(&self, pbuddy: windows_core::Ref<IRTCBuddy>) -> windows_core::Result<()>;
    fn RemoveBuddy(&self, pbuddy: windows_core::Ref<IRTCBuddy>) -> windows_core::Result<()>;
    fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> windows_core::Result<IRTCCollection>;
    fn Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroup_Vtbl {
    pub const fn new<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgroupname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrgroupname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCBuddyGroup_Impl::SetName(this, core::mem::transmute(&bstrgroupname)).into()
            }
        }
        unsafe extern "system" fn AddBuddy<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCBuddyGroup_Impl::AddBuddy(this, core::mem::transmute_copy(&pbuddy)).into()
            }
        }
        unsafe extern "system" fn RemoveBuddy<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCBuddyGroup_Impl::RemoveBuddy(this, core::mem::transmute_copy(&pbuddy)).into()
            }
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroup_Impl::EnumerateBuddies(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Buddies<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroup_Impl::Buddies(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroup_Impl::Data(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCBuddyGroup_Impl::SetData(this, core::mem::transmute(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn Profile<Identity: IRTCBuddyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroup_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            AddBuddy: AddBuddy::<Identity, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, OFFSET>,
            Buddies: Buddies::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddyGroup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddyGroup {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Group(&self) -> windows_core::Result<IRTCBuddyGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Buddy(&self) -> windows_core::Result<IRTCBuddy2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buddy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroupEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_GROUP_EVENT_TYPE) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyGroupEvent_Impl: super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(&self) -> windows_core::Result<IRTCBuddyGroup>;
    fn Buddy(&self) -> windows_core::Result<IRTCBuddy2>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCBuddyGroupEvent_Vtbl {
    pub const fn new<Identity: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroupEvent_Impl::EventType(this) {
                    Ok(ok__) => {
                        peventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Group<Identity: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroupEvent_Impl::Group(this) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Buddy<Identity: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroupEvent_Impl::Buddy(this) {
                    Ok(ok__) => {
                        ppbuddy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCBuddyGroupEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCBuddyGroupEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EventType: EventType::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            Buddy: Buddy::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddyGroupEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCBuddyGroupEvent {}
windows_core::imp::define_interface!(IRTCClient, IRTCClient_Vtbl, 0x07829e45_9a34_408e_a011_bddf13487cd1);
windows_core::imp::interface_hierarchy!(IRTCClient, windows_core::IUnknown);
impl IRTCClient {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PrepareForShutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareForShutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventFilter)(windows_core::Interface::as_raw(self), lfilter).ok() }
    }
    pub unsafe fn EventFilter(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredMediaTypes)(windows_core::Interface::as_raw(self), lmediatypes, fpersistent).ok() }
    }
    pub unsafe fn PreferredMediaTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MediaCapabilities(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateSession<P2>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: P2, lflags: i32) -> windows_core::Result<IRTCSession>
    where
        P2: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), entype, core::mem::transmute_copy(bstrlocalphoneuri), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetListenForIncomingSessions)(windows_core::Interface::as_raw(self), enlisten).ok() }
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> windows_core::Result<RTC_LISTEN_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ListenForIncomingSessions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_NetworkAddresses(&self, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NetworkAddresses)(windows_core::Interface::as_raw(self), ftcp, fexternal, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_Volume)(windows_core::Interface::as_raw(self), endevice, lvolume).ok() }
    }
    pub unsafe fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Volume)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_AudioMuted)(windows_core::Interface::as_raw(self), endevice, fmuted).ok() }
    }
    pub unsafe fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AudioMuted)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub unsafe fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> windows_core::Result<super::super::Media::DirectShow::IVideoWindow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IVideoWindow)(windows_core::Interface::as_raw(self), endevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_PreferredAudioDevice)(windows_core::Interface::as_raw(self), endevice, core::mem::transmute_copy(bstrdevicename)).ok() }
    }
    pub unsafe fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreferredAudioDevice)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_PreferredVolume)(windows_core::Interface::as_raw(self), endevice, lvolume).ok() }
    }
    pub unsafe fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreferredVolume)(windows_core::Interface::as_raw(self), endevice, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreferredAEC(&self, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredAEC)(windows_core::Interface::as_raw(self), benable).ok() }
    }
    pub unsafe fn PreferredAEC(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredAEC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreferredVideoDevice(&self, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredVideoDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdevicename)).ok() }
    }
    pub unsafe fn PreferredVideoDevice(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredVideoDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ActiveMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxBitrate)(windows_core::Interface::as_raw(self), lmaxbitrate).ok() }
    }
    pub unsafe fn MaxBitrate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxBitrate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTemporalSpatialTradeOff)(windows_core::Interface::as_raw(self), lvalue).ok() }
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TemporalSpatialTradeOff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NetworkQuality(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartT120Applet)(windows_core::Interface::as_raw(self), enapplet).ok() }
    }
    pub unsafe fn StopT120Applets(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopT120Applets)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsT120AppletRunning)(windows_core::Interface::as_raw(self), enapplet, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalUserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserURI(&self, bstruseruri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstruseruri)).ok() }
    }
    pub unsafe fn LocalUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserName(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PlayRing)(windows_core::Interface::as_raw(self), entype, bplay).ok() }
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDTMF)(windows_core::Interface::as_raw(self), endtmf).ok() }
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvokeTuningWizard)(windows_core::Interface::as_raw(self), hwndparent).ok() }
    }
    pub unsafe fn IsTuned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTuned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetListenForIncomingSessions: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_LISTEN_MODE) -> windows_core::HRESULT,
    pub ListenForIncomingSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_LISTEN_MODE) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_NetworkAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_NetworkAddresses: usize,
    pub put_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, i32) -> windows_core::HRESULT,
    pub get_Volume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut i32) -> windows_core::HRESULT,
    pub put_AudioMuted: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_AudioMuted: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
    pub get_IVideoWindow: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_VIDEO_DEVICE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com")))]
    get_IVideoWindow: usize,
    pub put_PreferredAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PreferredAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PreferredVolume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, i32) -> windows_core::HRESULT,
    pub get_PreferredVolume: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_AUDIO_DEVICE, *mut i32) -> windows_core::HRESULT,
    pub SetPreferredAEC: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PreferredAEC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPreferredVideoDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreferredVideoDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTemporalSpatialTradeOff: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TemporalSpatialTradeOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NetworkQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartT120Applet: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_T120_APPLET) -> windows_core::HRESULT,
    pub StopT120Applets: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IsT120AppletRunning: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_T120_APPLET, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LocalUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlayRing: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_RING_TYPE, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SendDTMF: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_DTMF) -> windows_core::HRESULT,
    pub InvokeTuningWizard: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub IsTuned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClient_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn PrepareForShutdown(&self) -> windows_core::Result<()>;
    fn SetEventFilter(&self, lfilter: i32) -> windows_core::Result<()>;
    fn EventFilter(&self) -> windows_core::Result<i32>;
    fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PreferredMediaTypes(&self) -> windows_core::Result<i32>;
    fn MediaCapabilities(&self) -> windows_core::Result<i32>;
    fn CreateSession(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> windows_core::Result<()>;
    fn ListenForIncomingSessions(&self) -> windows_core::Result<RTC_LISTEN_MODE>;
    fn get_NetworkAddresses(&self, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::Variant::VARIANT>;
    fn put_Volume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()>;
    fn get_Volume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32>;
    fn put_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> windows_core::Result<super::super::Media::DirectShow::IVideoWindow>;
    fn put_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<windows_core::BSTR>;
    fn put_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::Result<()>;
    fn get_PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> windows_core::Result<i32>;
    fn SetPreferredAEC(&self, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PreferredAEC(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPreferredVideoDevice(&self, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PreferredVideoDevice(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ActiveMedia(&self) -> windows_core::Result<i32>;
    fn SetMaxBitrate(&self, lmaxbitrate: i32) -> windows_core::Result<()>;
    fn MaxBitrate(&self) -> windows_core::Result<i32>;
    fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> windows_core::Result<()>;
    fn TemporalSpatialTradeOff(&self) -> windows_core::Result<i32>;
    fn NetworkQuality(&self) -> windows_core::Result<i32>;
    fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<()>;
    fn StopT120Applets(&self) -> windows_core::Result<()>;
    fn get_IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalUserURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserURI(&self, bstruseruri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserName(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SendDTMF(&self, endtmf: RTC_DTMF) -> windows_core::Result<()>;
    fn InvokeTuningWizard(&self, hwndparent: isize) -> windows_core::Result<()>;
    fn IsTuned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCClient_Vtbl {
    pub const fn new<Identity: IRTCClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::Initialize(this).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn PrepareForShutdown<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::PrepareForShutdown(this).into()
            }
        }
        unsafe extern "system" fn SetEventFilter<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfilter: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetEventFilter(this, core::mem::transmute_copy(&lfilter)).into()
            }
        }
        unsafe extern "system" fn EventFilter<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfilter: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::EventFilter(this) {
                    Ok(ok__) => {
                        plfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetPreferredMediaTypes(this, core::mem::transmute_copy(&lmediatypes), core::mem::transmute_copy(&fpersistent)).into()
            }
        }
        unsafe extern "system" fn PreferredMediaTypes<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::PreferredMediaTypes(this) {
                    Ok(ok__) => {
                        plmediatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaCapabilities<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::MediaCapabilities(this) {
                    Ok(ok__) => {
                        plmediatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSession<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lflags: i32, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::CreateSession(this, core::mem::transmute_copy(&entype), core::mem::transmute(&bstrlocalphoneuri), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetListenForIncomingSessions(this, core::mem::transmute_copy(&enlisten)).into()
            }
        }
        unsafe extern "system" fn ListenForIncomingSessions<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::ListenForIncomingSessions(this) {
                    Ok(ok__) => {
                        penlisten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NetworkAddresses<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL, pvaddresses: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_NetworkAddresses(this, core::mem::transmute_copy(&ftcp), core::mem::transmute_copy(&fexternal)) {
                    Ok(ok__) => {
                        pvaddresses.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Volume<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::put_Volume(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn get_Volume<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_Volume(this, core::mem::transmute_copy(&endevice)) {
                    Ok(ok__) => {
                        plvolume.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AudioMuted<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::put_AudioMuted(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&fmuted)).into()
            }
        }
        unsafe extern "system" fn get_AudioMuted<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_AudioMuted(this, core::mem::transmute_copy(&endevice)) {
                    Ok(ok__) => {
                        pfmuted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IVideoWindow<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_IVideoWindow(this, core::mem::transmute_copy(&endevice)) {
                    Ok(ok__) => {
                        ppivideowindow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PreferredAudioDevice<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::put_PreferredAudioDevice(this, core::mem::transmute_copy(&endevice), core::mem::transmute(&bstrdevicename)).into()
            }
        }
        unsafe extern "system" fn get_PreferredAudioDevice<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_PreferredAudioDevice(this, core::mem::transmute_copy(&endevice)) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PreferredVolume<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::put_PreferredVolume(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn get_PreferredVolume<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_PreferredVolume(this, core::mem::transmute_copy(&endevice)) {
                    Ok(ok__) => {
                        plvolume.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetPreferredAEC(this, core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn PreferredAEC<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::PreferredAEC(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetPreferredVideoDevice(this, core::mem::transmute(&bstrdevicename)).into()
            }
        }
        unsafe extern "system" fn PreferredVideoDevice<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::PreferredVideoDevice(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActiveMedia<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::ActiveMedia(this) {
                    Ok(ok__) => {
                        plmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbitrate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetMaxBitrate(this, core::mem::transmute_copy(&lmaxbitrate)).into()
            }
        }
        unsafe extern "system" fn MaxBitrate<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxbitrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::MaxBitrate(this) {
                    Ok(ok__) => {
                        plmaxbitrate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetTemporalSpatialTradeOff(this, core::mem::transmute_copy(&lvalue)).into()
            }
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::TemporalSpatialTradeOff(this) {
                    Ok(ok__) => {
                        plvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetworkQuality<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnetworkquality: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::NetworkQuality(this) {
                    Ok(ok__) => {
                        plnetworkquality.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartT120Applet<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enapplet: RTC_T120_APPLET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::StartT120Applet(this, core::mem::transmute_copy(&enapplet)).into()
            }
        }
        unsafe extern "system" fn StopT120Applets<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::StopT120Applets(this).into()
            }
        }
        unsafe extern "system" fn get_IsT120AppletRunning<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::get_IsT120AppletRunning(this, core::mem::transmute_copy(&enapplet)) {
                    Ok(ok__) => {
                        pfrunning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalUserURI<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::LocalUserURI(this) {
                    Ok(ok__) => {
                        pbstruseruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseruri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetLocalUserURI(this, core::mem::transmute(&bstruseruri)).into()
            }
        }
        unsafe extern "system" fn LocalUserName<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::LocalUserName(this) {
                    Ok(ok__) => {
                        pbstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserName<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SetLocalUserName(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn PlayRing<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::PlayRing(this, core::mem::transmute_copy(&entype), core::mem::transmute_copy(&bplay)).into()
            }
        }
        unsafe extern "system" fn SendDTMF<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endtmf: RTC_DTMF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::SendDTMF(this, core::mem::transmute_copy(&endtmf)).into()
            }
        }
        unsafe extern "system" fn InvokeTuningWizard<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient_Impl::InvokeTuningWizard(this, core::mem::transmute_copy(&hwndparent)).into()
            }
        }
        unsafe extern "system" fn IsTuned<Identity: IRTCClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftuned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient_Impl::IsTuned(this) {
                    Ok(ok__) => {
                        pftuned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            PrepareForShutdown: PrepareForShutdown::<Identity, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, OFFSET>,
            EventFilter: EventFilter::<Identity, OFFSET>,
            SetPreferredMediaTypes: SetPreferredMediaTypes::<Identity, OFFSET>,
            PreferredMediaTypes: PreferredMediaTypes::<Identity, OFFSET>,
            MediaCapabilities: MediaCapabilities::<Identity, OFFSET>,
            CreateSession: CreateSession::<Identity, OFFSET>,
            SetListenForIncomingSessions: SetListenForIncomingSessions::<Identity, OFFSET>,
            ListenForIncomingSessions: ListenForIncomingSessions::<Identity, OFFSET>,
            get_NetworkAddresses: get_NetworkAddresses::<Identity, OFFSET>,
            put_Volume: put_Volume::<Identity, OFFSET>,
            get_Volume: get_Volume::<Identity, OFFSET>,
            put_AudioMuted: put_AudioMuted::<Identity, OFFSET>,
            get_AudioMuted: get_AudioMuted::<Identity, OFFSET>,
            get_IVideoWindow: get_IVideoWindow::<Identity, OFFSET>,
            put_PreferredAudioDevice: put_PreferredAudioDevice::<Identity, OFFSET>,
            get_PreferredAudioDevice: get_PreferredAudioDevice::<Identity, OFFSET>,
            put_PreferredVolume: put_PreferredVolume::<Identity, OFFSET>,
            get_PreferredVolume: get_PreferredVolume::<Identity, OFFSET>,
            SetPreferredAEC: SetPreferredAEC::<Identity, OFFSET>,
            PreferredAEC: PreferredAEC::<Identity, OFFSET>,
            SetPreferredVideoDevice: SetPreferredVideoDevice::<Identity, OFFSET>,
            PreferredVideoDevice: PreferredVideoDevice::<Identity, OFFSET>,
            ActiveMedia: ActiveMedia::<Identity, OFFSET>,
            SetMaxBitrate: SetMaxBitrate::<Identity, OFFSET>,
            MaxBitrate: MaxBitrate::<Identity, OFFSET>,
            SetTemporalSpatialTradeOff: SetTemporalSpatialTradeOff::<Identity, OFFSET>,
            TemporalSpatialTradeOff: TemporalSpatialTradeOff::<Identity, OFFSET>,
            NetworkQuality: NetworkQuality::<Identity, OFFSET>,
            StartT120Applet: StartT120Applet::<Identity, OFFSET>,
            StopT120Applets: StopT120Applets::<Identity, OFFSET>,
            get_IsT120AppletRunning: get_IsT120AppletRunning::<Identity, OFFSET>,
            LocalUserURI: LocalUserURI::<Identity, OFFSET>,
            SetLocalUserURI: SetLocalUserURI::<Identity, OFFSET>,
            LocalUserName: LocalUserName::<Identity, OFFSET>,
            SetLocalUserName: SetLocalUserName::<Identity, OFFSET>,
            PlayRing: PlayRing::<Identity, OFFSET>,
            SendDTMF: SendDTMF::<Identity, OFFSET>,
            InvokeTuningWizard: InvokeTuningWizard::<Identity, OFFSET>,
            IsTuned: IsTuned::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCClient {}
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
        unsafe { (windows_core::Interface::vtable(self).put_AnswerMode)(windows_core::Interface::as_raw(self), entype, enmode).ok() }
    }
    pub unsafe fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> windows_core::Result<RTC_ANSWER_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AnswerMode)(windows_core::Interface::as_raw(self), entype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvokeTuningWizardEx)(windows_core::Interface::as_raw(self), hwndparent, fallowaudio, fallowvideo).ok() }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, bstrclientname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclientname)).ok() }
    }
    pub unsafe fn SetClientCurVer(&self, bstrclientcurver: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientCurVer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclientcurver)).ok() }
    }
    pub unsafe fn InitializeEx(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeEx)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn CreateSessionWithDescription<P2>(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pprofile: P2, lflags: i32) -> windows_core::Result<IRTCSession2>
    where
        P2: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSessionWithDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcontenttype), core::mem::transmute_copy(bstrsessiondescription), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSessionDescriptionManager<P0>(&self, psessiondescriptionmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCSessionDescriptionManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSessionDescriptionManager)(windows_core::Interface::as_raw(self), psessiondescriptionmanager.param().abi()).ok() }
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok() }
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_AllowedPorts)(windows_core::Interface::as_raw(self), ltransport, enlistenmode).ok() }
    }
    pub unsafe fn get_AllowedPorts(&self, ltransport: i32) -> windows_core::Result<RTC_LISTEN_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowedPorts)(windows_core::Interface::as_raw(self), ltransport, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient2_Vtbl {
    pub base__: IRTCClient_Vtbl,
    pub put_AnswerMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, RTC_ANSWER_MODE) -> windows_core::HRESULT,
    pub get_AnswerMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, *mut RTC_ANSWER_MODE) -> windows_core::HRESULT,
    pub InvokeTuningWizardEx: unsafe extern "system" fn(*mut core::ffi::c_void, isize, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientCurVer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateSessionWithDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSessionDescriptionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub put_AllowedPorts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, RTC_LISTEN_MODE) -> windows_core::HRESULT,
    pub get_AllowedPorts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut RTC_LISTEN_MODE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClient2_Impl: IRTCClient_Impl {
    fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> windows_core::Result<()>;
    fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> windows_core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<i32>;
    fn SetClientName(&self, bstrclientname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCurVer(&self, bstrclientcurver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeEx(&self, lflags: i32) -> windows_core::Result<()>;
    fn CreateSessionWithDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(&self, psessiondescriptionmanager: windows_core::Ref<IRTCSessionDescriptionManager>) -> windows_core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::Result<()>;
    fn get_AllowedPorts(&self, ltransport: i32) -> windows_core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCClient2_Vtbl {
    pub const fn new<Identity: IRTCClient2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_AnswerMode<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::put_AnswerMode(this, core::mem::transmute_copy(&entype), core::mem::transmute_copy(&enmode)).into()
            }
        }
        unsafe extern "system" fn get_AnswerMode<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient2_Impl::get_AnswerMode(this, core::mem::transmute_copy(&entype)) {
                    Ok(ok__) => {
                        penmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::InvokeTuningWizardEx(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&fallowaudio), core::mem::transmute_copy(&fallowvideo)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient2_Impl::Version(this) {
                    Ok(ok__) => {
                        plversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclientname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::SetClientName(this, core::mem::transmute(&bstrclientname)).into()
            }
        }
        unsafe extern "system" fn SetClientCurVer<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclientcurver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::SetClientCurVer(this, core::mem::transmute(&bstrclientcurver)).into()
            }
        }
        unsafe extern "system" fn InitializeEx<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::InitializeEx(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateSessionWithDescription<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: *mut core::ffi::c_void, bstrsessiondescription: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lflags: i32, ppsession2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient2_Impl::CreateSessionWithDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppsession2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessiondescriptionmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::SetSessionDescriptionManager(this, core::mem::transmute_copy(&psessiondescriptionmanager)).into()
            }
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::put_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype), core::mem::transmute_copy(&ensecuritylevel)).into()
            }
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient2_Impl::get_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                    Ok(ok__) => {
                        pensecuritylevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowedPorts<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClient2_Impl::put_AllowedPorts(this, core::mem::transmute_copy(&ltransport), core::mem::transmute_copy(&enlistenmode)).into()
            }
        }
        unsafe extern "system" fn get_AllowedPorts<Identity: IRTCClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClient2_Impl::get_AllowedPorts(this, core::mem::transmute_copy(&ltransport)) {
                    Ok(ok__) => {
                        penlistenmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRTCClient_Vtbl::new::<Identity, OFFSET>(),
            put_AnswerMode: put_AnswerMode::<Identity, OFFSET>,
            get_AnswerMode: get_AnswerMode::<Identity, OFFSET>,
            InvokeTuningWizardEx: InvokeTuningWizardEx::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            SetClientCurVer: SetClientCurVer::<Identity, OFFSET>,
            InitializeEx: InitializeEx::<Identity, OFFSET>,
            CreateSessionWithDescription: CreateSessionWithDescription::<Identity, OFFSET>,
            SetSessionDescriptionManager: SetSessionDescriptionManager::<Identity, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, OFFSET>,
            put_AllowedPorts: put_AllowedPorts::<Identity, OFFSET>,
            get_AllowedPorts: get_AllowedPorts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClient2 as windows_core::Interface>::IID || iid == &<IRTCClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCClient2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Client(&self) -> windows_core::Result<IRTCClient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Client)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_CLIENT_EVENT_TYPE) -> windows_core::HRESULT,
    pub Client: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientEvent_Impl: super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(&self) -> windows_core::Result<IRTCClient>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCClientEvent_Vtbl {
    pub const fn new<Identity: IRTCClientEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientEvent_Impl::EventType(this) {
                    Ok(ok__) => {
                        peneventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Client<Identity: IRTCClientEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientEvent_Impl::Client(this) {
                    Ok(ok__) => {
                        ppclient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET>, Client: Client::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCClientEvent {}
windows_core::imp::define_interface!(IRTCClientPortManagement, IRTCClientPortManagement_Vtbl, 0xd5df3f03_4bde_4417_aefe_71177bdaea66);
windows_core::imp::interface_hierarchy!(IRTCClientPortManagement, windows_core::IUnknown);
impl IRTCClientPortManagement {
    pub unsafe fn StartListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartListenAddressAndPort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinternallocaladdress), linternallocalport).ok() }
    }
    pub unsafe fn StopListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopListenAddressAndPort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinternallocaladdress), linternallocalport).ok() }
    }
    pub unsafe fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPortRange)(windows_core::Interface::as_raw(self), enporttype, plminvalue as _, plmaxvalue as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPortManagement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListenAddressAndPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StopListenAddressAndPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPortRange: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PORT_TYPE, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IRTCClientPortManagement_Impl: windows_core::IUnknownImpl {
    fn StartListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()>;
    fn StopListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()>;
    fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::Result<()>;
}
impl IRTCClientPortManagement_Vtbl {
    pub const fn new<Identity: IRTCClientPortManagement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartListenAddressAndPort<Identity: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: *mut core::ffi::c_void, linternallocalport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPortManagement_Impl::StartListenAddressAndPort(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport)).into()
            }
        }
        unsafe extern "system" fn StopListenAddressAndPort<Identity: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: *mut core::ffi::c_void, linternallocalport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPortManagement_Impl::StopListenAddressAndPort(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport)).into()
            }
        }
        unsafe extern "system" fn GetPortRange<Identity: IRTCClientPortManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPortManagement_Impl::GetPortRange(this, core::mem::transmute_copy(&enporttype), core::mem::transmute_copy(&plminvalue), core::mem::transmute_copy(&plmaxvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListenAddressAndPort: StartListenAddressAndPort::<Identity, OFFSET>,
            StopListenAddressAndPort: StopListenAddressAndPort::<Identity, OFFSET>,
            GetPortRange: GetPortRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientPortManagement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCClientPortManagement {}
windows_core::imp::define_interface!(IRTCClientPresence, IRTCClientPresence_Vtbl, 0x11c3cbcc_0744_42d1_968a_51aa1bb274c6);
windows_core::imp::interface_hierarchy!(IRTCClientPresence, windows_core::IUnknown);
impl IRTCClientPresence {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnablePresence(&self, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnablePresence)(windows_core::Interface::as_raw(self), fusestorage, core::mem::transmute_copy(varstorage)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Export(&self, varstorage: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varstorage)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Import(&self, varstorage: &super::Variant::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varstorage), freplaceall).ok() }
    }
    pub unsafe fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateBuddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Buddies(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buddies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Buddy(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCBuddy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Buddy)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddBuddy<P4>(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: P4, lflags: i32) -> windows_core::Result<IRTCBuddy>
    where
        P4: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddBuddy)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrdata), fpersistent, pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveBuddy<P0>(&self, pbuddy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddy>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveBuddy)(windows_core::Interface::as_raw(self), pbuddy.param().abi()).ok() }
    }
    pub unsafe fn EnumerateWatchers(&self) -> windows_core::Result<IRTCEnumWatchers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateWatchers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Watchers(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Watchers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Watcher(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Watcher)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddWatcher(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IRTCWatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddWatcher)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrdata), fblocked, fpersistent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveWatcher<P0>(&self, pwatcher: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCWatcher>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveWatcher)(windows_core::Interface::as_raw(self), pwatcher.param().abi()).ok() }
    }
    pub unsafe fn SetLocalPresenceInfo(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalPresenceInfo)(windows_core::Interface::as_raw(self), enstatus, core::mem::transmute_copy(bstrnotes)).ok() }
    }
    pub unsafe fn OfferWatcherMode(&self) -> windows_core::Result<RTC_OFFER_WATCHER_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfferWatcherMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOfferWatcherMode)(windows_core::Interface::as_raw(self), enmode).ok() }
    }
    pub unsafe fn PrivacyMode(&self) -> windows_core::Result<RTC_PRIVACY_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivacyMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivacyMode)(windows_core::Interface::as_raw(self), enmode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnablePresence: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnablePresence: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Export: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Import: usize,
    pub EnumerateBuddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Buddies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buddies: usize,
    pub get_Buddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBuddy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateWatchers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Watchers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Watchers: usize,
    pub get_Watcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_STATUS, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OfferWatcherMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT,
    pub SetOfferWatcherMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT,
    pub PrivacyMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRIVACY_MODE) -> windows_core::HRESULT,
    pub SetPrivacyMode: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRIVACY_MODE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientPresence_Impl: windows_core::IUnknownImpl {
    fn EnablePresence(&self, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Export(&self, varstorage: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Import(&self, varstorage: &super::Variant::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Buddy(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCBuddy>;
    fn AddBuddy(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddy>;
    fn RemoveBuddy(&self, pbuddy: windows_core::Ref<IRTCBuddy>) -> windows_core::Result<()>;
    fn EnumerateWatchers(&self) -> windows_core::Result<IRTCEnumWatchers>;
    fn Watchers(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Watcher(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher>;
    fn AddWatcher(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IRTCWatcher>;
    fn RemoveWatcher(&self, pwatcher: windows_core::Ref<IRTCWatcher>) -> windows_core::Result<()>;
    fn SetLocalPresenceInfo(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfferWatcherMode(&self) -> windows_core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::Result<()>;
    fn PrivacyMode(&self) -> windows_core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCClientPresence_Vtbl {
    pub const fn new<Identity: IRTCClientPresence_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnablePresence<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::EnablePresence(this, core::mem::transmute_copy(&fusestorage), core::mem::transmute(&varstorage)).into()
            }
        }
        unsafe extern "system" fn Export<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varstorage: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::Export(this, core::mem::transmute(&varstorage)).into()
            }
        }
        unsafe extern "system" fn Import<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varstorage: super::Variant::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::Import(this, core::mem::transmute(&varstorage), core::mem::transmute_copy(&freplaceall)).into()
            }
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::EnumerateBuddies(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Buddies<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::Buddies(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Buddy<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::get_Buddy(this, core::mem::transmute(&bstrpresentityuri)) {
                    Ok(ok__) => {
                        ppbuddy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddBuddy<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: *mut core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::AddBuddy(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fpersistent), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppbuddy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveBuddy<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::RemoveBuddy(this, core::mem::transmute_copy(&pbuddy)).into()
            }
        }
        unsafe extern "system" fn EnumerateWatchers<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::EnumerateWatchers(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Watchers<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::Watchers(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Watcher<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::get_Watcher(this, core::mem::transmute(&bstrpresentityuri)) {
                    Ok(ok__) => {
                        ppwatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddWatcher<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::AddWatcher(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fblocked), core::mem::transmute_copy(&fpersistent)) {
                    Ok(ok__) => {
                        ppwatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveWatcher<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwatcher: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::RemoveWatcher(this, core::mem::transmute_copy(&pwatcher)).into()
            }
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::SetLocalPresenceInfo(this, core::mem::transmute_copy(&enstatus), core::mem::transmute(&bstrnotes)).into()
            }
        }
        unsafe extern "system" fn OfferWatcherMode<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::OfferWatcherMode(this) {
                    Ok(ok__) => {
                        penmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::SetOfferWatcherMode(this, core::mem::transmute_copy(&enmode)).into()
            }
        }
        unsafe extern "system" fn PrivacyMode<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence_Impl::PrivacyMode(this) {
                    Ok(ok__) => {
                        penmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Identity: IRTCClientPresence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence_Impl::SetPrivacyMode(this, core::mem::transmute_copy(&enmode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnablePresence: EnablePresence::<Identity, OFFSET>,
            Export: Export::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, OFFSET>,
            Buddies: Buddies::<Identity, OFFSET>,
            get_Buddy: get_Buddy::<Identity, OFFSET>,
            AddBuddy: AddBuddy::<Identity, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, OFFSET>,
            EnumerateWatchers: EnumerateWatchers::<Identity, OFFSET>,
            Watchers: Watchers::<Identity, OFFSET>,
            get_Watcher: get_Watcher::<Identity, OFFSET>,
            AddWatcher: AddWatcher::<Identity, OFFSET>,
            RemoveWatcher: RemoveWatcher::<Identity, OFFSET>,
            SetLocalPresenceInfo: SetLocalPresenceInfo::<Identity, OFFSET>,
            OfferWatcherMode: OfferWatcherMode::<Identity, OFFSET>,
            SetOfferWatcherMode: SetOfferWatcherMode::<Identity, OFFSET>,
            PrivacyMode: PrivacyMode::<Identity, OFFSET>,
            SetPrivacyMode: SetPrivacyMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientPresence as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCClientPresence {}
windows_core::imp::define_interface!(IRTCClientPresence2, IRTCClientPresence2_Vtbl, 0xad1809e8_62f7_4783_909a_29c9d2cb1d34);
impl core::ops::Deref for IRTCClientPresence2 {
    type Target = IRTCClientPresence;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCClientPresence2, windows_core::IUnknown, IRTCClientPresence);
impl IRTCClientPresence2 {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnablePresenceEx<P0>(&self, pprofile: P0, varstorage: &super::Variant::VARIANT, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnablePresenceEx)(windows_core::Interface::as_raw(self), pprofile.param().abi(), core::mem::transmute_copy(varstorage), lflags).ok() }
    }
    pub unsafe fn DisablePresence(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisablePresence)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddGroup<P2>(&self, bstrgroupname: &windows_core::BSTR, bstrdata: &windows_core::BSTR, pprofile: P2, lflags: i32) -> windows_core::Result<IRTCBuddyGroup>
    where
        P2: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), core::mem::transmute_copy(bstrdata), pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveGroup<P0>(&self, pgroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCBuddyGroup>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveGroup)(windows_core::Interface::as_raw(self), pgroup.param().abi()).ok() }
    }
    pub unsafe fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Group(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<IRTCBuddyGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Group)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddWatcherEx<P6>(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: P6, lflags: i32) -> windows_core::Result<IRTCWatcher2>
    where
        P6: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddWatcherEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrdata), enstate, fpersistent, enscope, pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_WatcherEx(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WatcherEx)(windows_core::Interface::as_raw(self), enmode, core::mem::transmute_copy(bstrpresentityuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, core::mem::transmute_copy(bstrproperty)).ok() }
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPresenceData(&self, bstrnamespace: &windows_core::BSTR, bstrdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnamespace), core::mem::transmute_copy(bstrdata)).ok() }
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok() }
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocalPresenceInfo)(windows_core::Interface::as_raw(self), penstatus as _, core::mem::transmute(pbstrnotes)).ok() }
    }
    pub unsafe fn AddBuddyEx<P5>(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: P5, lflags: i32) -> windows_core::Result<IRTCBuddy2>
    where
        P5: windows_core::Param<IRTCProfile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddBuddyEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrdata), fpersistent, ensubscriptiontype, pprofile.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence2_Vtbl {
    pub base__: IRTCClientPresence_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnablePresenceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnablePresenceEx: usize,
    pub DisablePresence: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub get_Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWatcherEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, RTC_WATCHER_STATE, super::super::Foundation::VARIANT_BOOL, RTC_ACE_SCOPE, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_WatcherEx: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_WATCHER_MATCH_MODE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBuddyEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, RTC_BUDDY_SUBSCRIPTION_TYPE, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientPresence2_Impl: IRTCClientPresence_Impl {
    fn EnablePresenceEx(&self, pprofile: windows_core::Ref<IRTCProfile>, varstorage: &super::Variant::VARIANT, lflags: i32) -> windows_core::Result<()>;
    fn DisablePresence(&self) -> windows_core::Result<()>;
    fn AddGroup(&self, bstrgroupname: &windows_core::BSTR, bstrdata: &windows_core::BSTR, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(&self, pgroup: windows_core::Ref<IRTCBuddyGroup>) -> windows_core::Result<()>;
    fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups>;
    fn Groups(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Group(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCWatcher2>;
    fn get_WatcherEx(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher2>;
    fn put_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR>;
    fn SetPresenceData(&self, bstrnamespace: &windows_core::BSTR, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddBuddyEx(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddy2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCClientPresence2_Vtbl {
    pub const fn new<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnablePresenceEx<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, varstorage: super::Variant::VARIANT, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::EnablePresenceEx(this, core::mem::transmute_copy(&pprofile), core::mem::transmute(&varstorage), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn DisablePresence<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::DisablePresence(this).into()
            }
        }
        unsafe extern "system" fn AddGroup<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lflags: i32, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::AddGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGroup<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::RemoveGroup(this, core::mem::transmute_copy(&pgroup)).into()
            }
        }
        unsafe extern "system" fn EnumerateGroups<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::EnumerateGroups(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Groups<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::Groups(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Group<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::get_Group(this, core::mem::transmute(&bstrgroupname)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddWatcherEx<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: *mut core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::AddWatcherEx(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&enstate), core::mem::transmute_copy(&fpersistent), core::mem::transmute_copy(&enscope), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppwatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WatcherEx<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: *mut core::ffi::c_void, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::get_WatcherEx(this, core::mem::transmute_copy(&enmode), core::mem::transmute(&bstrpresentityuri)) {
                    Ok(ok__) => {
                        ppwatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PresenceProperty<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::put_PresenceProperty(this, core::mem::transmute_copy(&enproperty), core::mem::transmute(&bstrproperty)).into()
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                    Ok(ok__) => {
                        pbstrproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPresenceData<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnamespace: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::SetPresenceData(this, core::mem::transmute(&bstrnamespace), core::mem::transmute(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientPresence2_Impl::GetLocalPresenceInfo(this, core::mem::transmute_copy(&penstatus), core::mem::transmute_copy(&pbstrnotes)).into()
            }
        }
        unsafe extern "system" fn AddBuddyEx<Identity: IRTCClientPresence2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientPresence2_Impl::AddBuddyEx(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fpersistent), core::mem::transmute_copy(&ensubscriptiontype), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppbuddy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRTCClientPresence_Vtbl::new::<Identity, OFFSET>(),
            EnablePresenceEx: EnablePresenceEx::<Identity, OFFSET>,
            DisablePresence: DisablePresence::<Identity, OFFSET>,
            AddGroup: AddGroup::<Identity, OFFSET>,
            RemoveGroup: RemoveGroup::<Identity, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            get_Group: get_Group::<Identity, OFFSET>,
            AddWatcherEx: AddWatcherEx::<Identity, OFFSET>,
            get_WatcherEx: get_WatcherEx::<Identity, OFFSET>,
            put_PresenceProperty: put_PresenceProperty::<Identity, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, OFFSET>,
            SetPresenceData: SetPresenceData::<Identity, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, OFFSET>,
            AddBuddyEx: AddBuddyEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientPresence2 as windows_core::Interface>::IID || iid == &<IRTCClientPresence as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCClientPresence2 {}
windows_core::imp::define_interface!(IRTCClientProvisioning, IRTCClientProvisioning_Vtbl, 0xb9f5cf06_65b9_4a80_a0e6_73cae3ef3822);
windows_core::imp::interface_hierarchy!(IRTCClientProvisioning, windows_core::IUnknown);
impl IRTCClientProvisioning {
    pub unsafe fn CreateProfile(&self, bstrprofilexml: &windows_core::BSTR) -> windows_core::Result<IRTCProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProfile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprofilexml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableProfile<P0>(&self, pprofile: P0, lregisterflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi(), lregisterflags).ok() }
    }
    pub unsafe fn DisableProfile<P0>(&self, pprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).DisableProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()).ok() }
    }
    pub unsafe fn EnumerateProfiles(&self) -> windows_core::Result<IRTCEnumProfiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateProfiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profiles(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetProfile(&self, bstruseraccount: &windows_core::BSTR, bstruserpassword: &windows_core::BSTR, bstruseruri: &windows_core::BSTR, bstrserver: &windows_core::BSTR, ltransport: i32, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstruseraccount), core::mem::transmute_copy(bstruserpassword), core::mem::transmute_copy(bstruseruri), core::mem::transmute_copy(bstrserver), ltransport, lcookie).ok() }
    }
    pub unsafe fn SessionCapabilities(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisableProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profiles: usize,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, isize) -> windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning_Impl: windows_core::IUnknownImpl {
    fn CreateProfile(&self, bstrprofilexml: &windows_core::BSTR) -> windows_core::Result<IRTCProfile>;
    fn EnableProfile(&self, pprofile: windows_core::Ref<IRTCProfile>, lregisterflags: i32) -> windows_core::Result<()>;
    fn DisableProfile(&self, pprofile: windows_core::Ref<IRTCProfile>) -> windows_core::Result<()>;
    fn EnumerateProfiles(&self) -> windows_core::Result<IRTCEnumProfiles>;
    fn Profiles(&self) -> windows_core::Result<IRTCCollection>;
    fn GetProfile(&self, bstruseraccount: &windows_core::BSTR, bstruserpassword: &windows_core::BSTR, bstruseruri: &windows_core::BSTR, bstrserver: &windows_core::BSTR, ltransport: i32, lcookie: isize) -> windows_core::Result<()>;
    fn SessionCapabilities(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning_Vtbl {
    pub const fn new<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProfile<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprofilexml: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientProvisioning_Impl::CreateProfile(this, core::mem::transmute(&bstrprofilexml)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableProfile<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lregisterflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientProvisioning_Impl::EnableProfile(this, core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lregisterflags)).into()
            }
        }
        unsafe extern "system" fn DisableProfile<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientProvisioning_Impl::DisableProfile(this, core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn EnumerateProfiles<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientProvisioning_Impl::EnumerateProfiles(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Profiles<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientProvisioning_Impl::Profiles(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProfile<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseraccount: *mut core::ffi::c_void, bstruserpassword: *mut core::ffi::c_void, bstruseruri: *mut core::ffi::c_void, bstrserver: *mut core::ffi::c_void, ltransport: i32, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientProvisioning_Impl::GetProfile(this, core::mem::transmute(&bstruseraccount), core::mem::transmute(&bstruserpassword), core::mem::transmute(&bstruseruri), core::mem::transmute(&bstrserver), core::mem::transmute_copy(&ltransport), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn SessionCapabilities<Identity: IRTCClientProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsupportedsessions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCClientProvisioning_Impl::SessionCapabilities(this) {
                    Ok(ok__) => {
                        plsupportedsessions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProfile: CreateProfile::<Identity, OFFSET>,
            EnableProfile: EnableProfile::<Identity, OFFSET>,
            DisableProfile: DisableProfile::<Identity, OFFSET>,
            EnumerateProfiles: EnumerateProfiles::<Identity, OFFSET>,
            Profiles: Profiles::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientProvisioning as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientProvisioning {}
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
        unsafe { (windows_core::Interface::vtable(self).EnableProfileEx)(windows_core::Interface::as_raw(self), pprofile.param().abi(), lregisterflags, lroamingflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning2_Vtbl {
    pub base__: IRTCClientProvisioning_Vtbl,
    pub EnableProfileEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning2_Impl: IRTCClientProvisioning_Impl {
    fn EnableProfileEx(&self, pprofile: windows_core::Ref<IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning2_Vtbl {
    pub const fn new<Identity: IRTCClientProvisioning2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableProfileEx<Identity: IRTCClientProvisioning2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCClientProvisioning2_Impl::EnableProfileEx(this, core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lregisterflags), core::mem::transmute_copy(&lroamingflags)).into()
            }
        }
        Self { base__: IRTCClientProvisioning_Vtbl::new::<Identity, OFFSET>(), EnableProfileEx: EnableProfileEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as windows_core::Interface>::IID || iid == &<IRTCClientProvisioning as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientProvisioning2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCCollection_Vtbl {
    pub const fn new<Identity: IRTCCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IRTCCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        lcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRTCCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IRTCCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppnewenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCCollection {}
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
#[repr(C)]
#[doc(hidden)]
pub struct IRTCDispatchEventNotification_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCDispatchEventNotification_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCDispatchEventNotification_Vtbl {
    pub const fn new<Identity: IRTCDispatchEventNotification_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCDispatchEventNotification as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCDispatchEventNotification {}
windows_core::imp::define_interface!(IRTCEnumBuddies, IRTCEnumBuddies_Vtbl, 0xf7296917_5569_4b3b_b3af_98d1144b2b87);
windows_core::imp::interface_hierarchy!(IRTCEnumBuddies, windows_core::IUnknown);
impl IRTCEnumBuddies {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCBuddy>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumBuddies> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumBuddies_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumBuddies_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCBuddy>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumBuddies>;
}
impl IRTCEnumBuddies_Vtbl {
    pub const fn new<Identity: IRTCEnumBuddies_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumBuddies_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumBuddies_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumBuddies_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumBuddies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumBuddies_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumBuddies as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumBuddies {}
windows_core::imp::define_interface!(IRTCEnumGroups, IRTCEnumGroups_Vtbl, 0x742378d6_a141_4415_8f27_35d99076cf5d);
windows_core::imp::interface_hierarchy!(IRTCEnumGroups, windows_core::IUnknown);
impl IRTCEnumGroups {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCBuddyGroup>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumGroups_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumGroups_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumGroups>;
}
impl IRTCEnumGroups_Vtbl {
    pub const fn new<Identity: IRTCEnumGroups_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumGroups_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumGroups_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumGroups_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumGroups_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumGroups as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumGroups {}
windows_core::imp::define_interface!(IRTCEnumParticipants, IRTCEnumParticipants_Vtbl, 0xfcd56f29_4a4f_41b2_ba5c_f5bccc060bf6);
windows_core::imp::interface_hierarchy!(IRTCEnumParticipants, windows_core::IUnknown);
impl IRTCEnumParticipants {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCParticipant>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumParticipants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumParticipants_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumParticipants_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCParticipant>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumParticipants>;
}
impl IRTCEnumParticipants_Vtbl {
    pub const fn new<Identity: IRTCEnumParticipants_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumParticipants_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumParticipants_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumParticipants_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumParticipants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumParticipants_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumParticipants as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumParticipants {}
windows_core::imp::define_interface!(IRTCEnumPresenceDevices, IRTCEnumPresenceDevices_Vtbl, 0x708c2ab7_8bf8_42f8_8c7d_635197ad5539);
windows_core::imp::interface_hierarchy!(IRTCEnumPresenceDevices, windows_core::IUnknown);
impl IRTCEnumPresenceDevices {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCPresenceDevice>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumPresenceDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumPresenceDevices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumPresenceDevices_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumPresenceDevices>;
}
impl IRTCEnumPresenceDevices_Vtbl {
    pub const fn new<Identity: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumPresenceDevices_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumPresenceDevices_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumPresenceDevices_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumPresenceDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumPresenceDevices_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumPresenceDevices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumPresenceDevices {}
windows_core::imp::define_interface!(IRTCEnumProfiles, IRTCEnumProfiles_Vtbl, 0x29b7c41c_ed82_4bca_84ad_39d5101b58e3);
windows_core::imp::interface_hierarchy!(IRTCEnumProfiles, windows_core::IUnknown);
impl IRTCEnumProfiles {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCProfile>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumProfiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumProfiles_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCProfile>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumProfiles>;
}
impl IRTCEnumProfiles_Vtbl {
    pub const fn new<Identity: IRTCEnumProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumProfiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumProfiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumProfiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumProfiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumProfiles as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumProfiles {}
windows_core::imp::define_interface!(IRTCEnumUserSearchResults, IRTCEnumUserSearchResults_Vtbl, 0x83d4d877_aa5d_4a5b_8d0e_002a8067e0e8);
windows_core::imp::interface_hierarchy!(IRTCEnumUserSearchResults, windows_core::IUnknown);
impl IRTCEnumUserSearchResults {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCUserSearchResult>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumUserSearchResults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumUserSearchResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumUserSearchResults_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumUserSearchResults>;
}
impl IRTCEnumUserSearchResults_Vtbl {
    pub const fn new<Identity: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumUserSearchResults_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumUserSearchResults_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumUserSearchResults_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumUserSearchResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumUserSearchResults_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumUserSearchResults as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumUserSearchResults {}
windows_core::imp::define_interface!(IRTCEnumWatchers, IRTCEnumWatchers_Vtbl, 0xa87d55d7_db74_4ed1_9ca4_77a0e41b413e);
windows_core::imp::interface_hierarchy!(IRTCEnumWatchers, windows_core::IUnknown);
impl IRTCEnumWatchers {
    pub unsafe fn Next(&self, ppelements: &mut [Option<IRTCWatcher>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IRTCEnumWatchers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumWatchers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCEnumWatchers_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCWatcher>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumWatchers>;
}
impl IRTCEnumWatchers_Vtbl {
    pub const fn new<Identity: IRTCEnumWatchers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumWatchers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumWatchers_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEnumWatchers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IRTCEnumWatchers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCEnumWatchers_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEnumWatchers as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCEnumWatchers {}
windows_core::imp::define_interface!(IRTCEventNotification, IRTCEventNotification_Vtbl, 0x13fa24c7_5748_4b21_91f5_7397609ce747);
windows_core::imp::interface_hierarchy!(IRTCEventNotification, windows_core::IUnknown);
impl IRTCEventNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<P1>(&self, rtcevent: RTC_EVENT, pevent: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), rtcevent, pevent.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEventNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_EVENT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCEventNotification_Impl: windows_core::IUnknownImpl {
    fn Event(&self, rtcevent: RTC_EVENT, pevent: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotification_Vtbl {
    pub const fn new<Identity: IRTCEventNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Event<Identity: IRTCEventNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtcevent: RTC_EVENT, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCEventNotification_Impl::Event(this, core::mem::transmute_copy(&rtcevent), core::mem::transmute_copy(&pevent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEventNotification as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCEventNotification {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Info(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Info)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InfoHeader(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InfoHeader)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCInfoEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InfoHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCInfoEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn Info(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InfoHeader(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCInfoEvent_Vtbl {
    pub const fn new<Identity: IRTCInfoEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCInfoEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Participant<Identity: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCInfoEvent_Impl::Participant(this) {
                    Ok(ok__) => {
                        ppparticipant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Info<Identity: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCInfoEvent_Impl::Info(this) {
                    Ok(ok__) => {
                        pbstrinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InfoHeader<Identity: IRTCInfoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinfoheader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCInfoEvent_Impl::InfoHeader(this) {
                    Ok(ok__) => {
                        pbstrinfoheader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Participant: Participant::<Identity, OFFSET>,
            Info: Info::<Identity, OFFSET>,
            InfoHeader: InfoHeader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCInfoEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCInfoEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Min(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Min)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Max(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Max)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<RTC_AUDIO_DEVICE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCIntensityEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_AUDIO_DEVICE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCIntensityEvent_Impl: super::Com::IDispatch_Impl {
    fn Level(&self) -> windows_core::Result<i32>;
    fn Min(&self) -> windows_core::Result<i32>;
    fn Max(&self) -> windows_core::Result<i32>;
    fn Direction(&self) -> windows_core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCIntensityEvent_Vtbl {
    pub const fn new<Identity: IRTCIntensityEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Level<Identity: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCIntensityEvent_Impl::Level(this) {
                    Ok(ok__) => {
                        pllevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Min<Identity: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCIntensityEvent_Impl::Min(this) {
                    Ok(ok__) => {
                        plmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Max<Identity: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmax: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCIntensityEvent_Impl::Max(this) {
                    Ok(ok__) => {
                        plmax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Direction<Identity: IRTCIntensityEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCIntensityEvent_Impl::Direction(this) {
                    Ok(ok__) => {
                        pendirection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Level: Level::<Identity, OFFSET>,
            Min: Min::<Identity, OFFSET>,
            Max: Max::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCIntensityEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCIntensityEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_MEDIA_EVENT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EventReason(&self) -> windows_core::Result<RTC_MEDIA_EVENT_REASON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventReason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MEDIA_EVENT_TYPE) -> windows_core::HRESULT,
    pub EventReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MEDIA_EVENT_REASON) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMediaEvent_Impl: super::Com::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<i32>;
    fn EventType(&self) -> windows_core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(&self) -> windows_core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCMediaEvent_Vtbl {
    pub const fn new<Identity: IRTCMediaEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MediaType<Identity: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaEvent_Impl::MediaType(this) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventType<Identity: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaEvent_Impl::EventType(this) {
                    Ok(ok__) => {
                        peneventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventReason<Identity: IRTCMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaEvent_Impl::EventReason(this) {
                    Ok(ok__) => {
                        peneventreason.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaType: MediaType::<Identity, OFFSET>,
            EventType: EventType::<Identity, OFFSET>,
            EventReason: EventReason::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCMediaEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCMediaEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProposedMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProposedMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Accept(&self, lmediatypes: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self), lmediatypes).ok() }
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMediaRequestEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ProposedMedia(&self) -> windows_core::Result<i32>;
    fn CurrentMedia(&self) -> windows_core::Result<i32>;
    fn Accept(&self, lmediatypes: i32) -> windows_core::Result<()>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCMediaRequestEvent_Vtbl {
    pub const fn new<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaRequestEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProposedMedia<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaRequestEvent_Impl::ProposedMedia(this) {
                    Ok(ok__) => {
                        plmediatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentMedia<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaRequestEvent_Impl::CurrentMedia(this) {
                    Ok(ok__) => {
                        plmediatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Accept<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatypes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCMediaRequestEvent_Impl::Accept(this, core::mem::transmute_copy(&lmediatypes)).into()
            }
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaRequestEvent_Impl::get_RemotePreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                    Ok(ok__) => {
                        pensecuritylevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reject<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCMediaRequestEvent_Impl::Reject(this).into()
            }
        }
        unsafe extern "system" fn State<Identity: IRTCMediaRequestEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMediaRequestEvent_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            ProposedMedia: ProposedMedia::<Identity, OFFSET>,
            CurrentMedia: CurrentMedia::<Identity, OFFSET>,
            Accept: Accept::<Identity, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, OFFSET>,
            Reject: Reject::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCMediaRequestEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCMediaRequestEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Participant(&self) -> windows_core::Result<IRTCParticipant> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EventType(&self) -> windows_core::Result<RTC_MESSAGING_EVENT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MessageHeader(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageHeader)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserStatus(&self) -> windows_core::Result<RTC_MESSAGING_USER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMessagingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MESSAGING_EVENT_TYPE) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_MESSAGING_USER_STATUS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMessagingEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn EventType(&self) -> windows_core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MessageHeader(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserStatus(&self) -> windows_core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCMessagingEvent_Vtbl {
    pub const fn new<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Participant<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::Participant(this) {
                    Ok(ok__) => {
                        ppparticipant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventType<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::EventType(this) {
                    Ok(ok__) => {
                        peneventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Message<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::Message(this) {
                    Ok(ok__) => {
                        pbstrmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MessageHeader<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmessageheader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::MessageHeader(this) {
                    Ok(ok__) => {
                        pbstrmessageheader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserStatus<Identity: IRTCMessagingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCMessagingEvent_Impl::UserStatus(this) {
                    Ok(ok__) => {
                        penuserstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Participant: Participant::<Identity, OFFSET>,
            EventType: EventType::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
            MessageHeader: MessageHeader::<Identity, OFFSET>,
            UserStatus: UserStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCMessagingEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCMessagingEvent {}
windows_core::imp::define_interface!(IRTCParticipant, IRTCParticipant_Vtbl, 0xae86add5_26b1_4414_af1d_b94cd938d739);
windows_core::imp::interface_hierarchy!(IRTCParticipant, windows_core::IUnknown);
impl IRTCParticipant {
    pub unsafe fn UserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Removable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Removable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Session(&self) -> windows_core::Result<IRTCSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipant_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Removable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCParticipant_Impl: windows_core::IUnknownImpl {
    fn UserURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Removable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(&self) -> windows_core::Result<IRTCSession>;
}
impl IRTCParticipant_Vtbl {
    pub const fn new<Identity: IRTCParticipant_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserURI<Identity: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipant_Impl::UserURI(this) {
                    Ok(ok__) => {
                        pbstruseruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipant_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Removable<Identity: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfremovable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipant_Impl::Removable(this) {
                    Ok(ok__) => {
                        pfremovable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipant_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Session<Identity: IRTCParticipant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipant_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UserURI: UserURI::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Removable: Removable::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Session: Session::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCParticipant as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCParticipant {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipantStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCParticipantStateChangeEvent_Impl: super::Com::IDispatch_Impl {
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCParticipantStateChangeEvent_Vtbl {
    pub const fn new<Identity: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Participant<Identity: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipantStateChangeEvent_Impl::Participant(this) {
                    Ok(ok__) => {
                        ppparticipant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipantStateChangeEvent_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCParticipantStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCParticipantStateChangeEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Participant: Participant::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCParticipantStateChangeEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCParticipantStateChangeEvent {}
windows_core::imp::define_interface!(IRTCPortManager, IRTCPortManager_Vtbl, 0xda77c14b_6208_43ca_8ddf_5b60a0a69fac);
windows_core::imp::interface_hierarchy!(IRTCPortManager, windows_core::IUnknown);
impl IRTCPortManager {
    pub unsafe fn GetMapping(&self, bstrremoteaddress: &windows_core::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut windows_core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut windows_core::BSTR, plexternallocalport: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMapping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrremoteaddress), enporttype, core::mem::transmute(pbstrinternallocaladdress), plinternallocalport as _, core::mem::transmute(pbstrexternallocaladdress), plexternallocalport as _).ok() }
    }
    pub unsafe fn UpdateRemoteAddress(&self, bstrremoteaddress: &windows_core::BSTR, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocalport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateRemoteAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrremoteaddress), core::mem::transmute_copy(bstrinternallocaladdress), linternallocalport, core::mem::transmute_copy(bstrexternallocaladdress), lexternallocalport).ok() }
    }
    pub unsafe fn ReleaseMapping(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocaladdress: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMapping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinternallocaladdress), linternallocalport, core::mem::transmute_copy(bstrexternallocaladdress), lexternallocaladdress).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPortManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RTC_PORT_TYPE, *mut *mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReleaseMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IRTCPortManager_Impl: windows_core::IUnknownImpl {
    fn GetMapping(&self, bstrremoteaddress: &windows_core::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut windows_core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut windows_core::BSTR, plexternallocalport: *mut i32) -> windows_core::Result<()>;
    fn UpdateRemoteAddress(&self, bstrremoteaddress: &windows_core::BSTR, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocalport: i32) -> windows_core::Result<()>;
    fn ReleaseMapping(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocaladdress: i32) -> windows_core::Result<()>;
}
impl IRTCPortManager_Vtbl {
    pub const fn new<Identity: IRTCPortManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMapping<Identity: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremoteaddress: *mut core::ffi::c_void, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut *mut core::ffi::c_void, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut *mut core::ffi::c_void, plexternallocalport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPortManager_Impl::GetMapping(this, core::mem::transmute(&bstrremoteaddress), core::mem::transmute_copy(&enporttype), core::mem::transmute_copy(&pbstrinternallocaladdress), core::mem::transmute_copy(&plinternallocalport), core::mem::transmute_copy(&pbstrexternallocaladdress), core::mem::transmute_copy(&plexternallocalport)).into()
            }
        }
        unsafe extern "system" fn UpdateRemoteAddress<Identity: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremoteaddress: *mut core::ffi::c_void, bstrinternallocaladdress: *mut core::ffi::c_void, linternallocalport: i32, bstrexternallocaladdress: *mut core::ffi::c_void, lexternallocalport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPortManager_Impl::UpdateRemoteAddress(this, core::mem::transmute(&bstrremoteaddress), core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport), core::mem::transmute(&bstrexternallocaladdress), core::mem::transmute_copy(&lexternallocalport)).into()
            }
        }
        unsafe extern "system" fn ReleaseMapping<Identity: IRTCPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: *mut core::ffi::c_void, linternallocalport: i32, bstrexternallocaladdress: *mut core::ffi::c_void, lexternallocaladdress: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPortManager_Impl::ReleaseMapping(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport), core::mem::transmute(&bstrexternallocaladdress), core::mem::transmute_copy(&lexternallocaladdress)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMapping: GetMapping::<Identity, OFFSET>,
            UpdateRemoteAddress: UpdateRemoteAddress::<Identity, OFFSET>,
            ReleaseMapping: ReleaseMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPortManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCPortManager {}
windows_core::imp::define_interface!(IRTCPresenceContact, IRTCPresenceContact_Vtbl, 0x8b22f92c_cd90_42db_a733_212205c3e3df);
windows_core::imp::interface_hierarchy!(IRTCPresenceContact, windows_core::IUnknown);
impl IRTCPresenceContact {
    pub unsafe fn PresentityURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PresentityURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPresentityURI(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPresentityURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpresentityuri)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)).ok() }
    }
    pub unsafe fn Data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdata)).ok() }
    }
    pub unsafe fn Persistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Persistent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPersistent(&self, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPersistent)(windows_core::Interface::as_raw(self), fpersistent).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceContact_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PresentityURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresentityURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Persistent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPersistent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IRTCPresenceContact_Impl: windows_core::IUnknownImpl {
    fn PresentityURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPresentityURI(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Persistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPersistent(&self, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl IRTCPresenceContact_Vtbl {
    pub const fn new<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PresentityURI<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpresentityuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceContact_Impl::PresentityURI(this) {
                    Ok(ok__) => {
                        pbstrpresentityuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPresentityURI<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceContact_Impl::SetPresentityURI(this, core::mem::transmute(&bstrpresentityuri)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceContact_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceContact_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn Data<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceContact_Impl::Data(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceContact_Impl::SetData(this, core::mem::transmute(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn Persistent<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpersistent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceContact_Impl::Persistent(this) {
                    Ok(ok__) => {
                        pfpersistent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPersistent<Identity: IRTCPresenceContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceContact_Impl::SetPersistent(this, core::mem::transmute_copy(&fpersistent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentityURI: PresentityURI::<Identity, OFFSET>,
            SetPresentityURI: SetPresentityURI::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            Persistent: Persistent::<Identity, OFFSET>,
            SetPersistent: SetPersistent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPresenceContact as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCPresenceContact {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDataEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresenceDataEvent_Impl: super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCPresenceDataEvent_Vtbl {
    pub const fn new<Identity: IRTCPresenceDataEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StatusCode<Identity: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceDataEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceDataEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: IRTCPresenceDataEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceDataEvent_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPresenceDataEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCPresenceDataEvent {}
windows_core::imp::define_interface!(IRTCPresenceDevice, IRTCPresenceDevice_Vtbl, 0xbc6a90dd_ad9a_48da_9b0c_2515e38521ad);
windows_core::imp::interface_hierarchy!(IRTCPresenceDevice, windows_core::IUnknown);
impl IRTCPresenceDevice {
    pub unsafe fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Notes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Notes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PresenceProperty)(windows_core::Interface::as_raw(self), enproperty, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPresenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrnamespace), core::mem::transmute(pbstrdata)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT,
    pub Notes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PRESENCE_PROPERTY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCPresenceDevice_Impl: windows_core::IUnknownImpl {
    fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IRTCPresenceDevice_Vtbl {
    pub const fn new<Identity: IRTCPresenceDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceDevice_Impl::Status(this) {
                    Ok(ok__) => {
                        penstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notes<Identity: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnotes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceDevice_Impl::Notes(this) {
                    Ok(ok__) => {
                        pbstrnotes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceDevice_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                    Ok(ok__) => {
                        pbstrproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: IRTCPresenceDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceDevice_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            Notes: Notes::<Identity, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPresenceDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCPresenceDevice {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PresenceProperty(&self) -> windows_core::Result<RTC_PRESENCE_PROPERTY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PresenceProperty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresencePropertyEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PresenceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_PROPERTY) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresencePropertyEvent_Impl: super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PresenceProperty(&self) -> windows_core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCPresencePropertyEvent_Vtbl {
    pub const fn new<Identity: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StatusCode<Identity: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresencePropertyEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresencePropertyEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PresenceProperty<Identity: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresencePropertyEvent_Impl::PresenceProperty(this) {
                    Ok(ok__) => {
                        penpresprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: IRTCPresencePropertyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresencePropertyEvent_Impl::Value(this) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
            PresenceProperty: PresenceProperty::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPresencePropertyEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCPresencePropertyEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocalPresenceInfo)(windows_core::Interface::as_raw(self), penstatus as _, core::mem::transmute(pbstrnotes)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalPresenceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PRESENCE_STATUS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresenceStatusEvent_Impl: super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCPresenceStatusEvent_Vtbl {
    pub const fn new<Identity: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StatusCode<Identity: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceStatusEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCPresenceStatusEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: IRTCPresenceStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCPresenceStatusEvent_Impl::GetLocalPresenceInfo(this, core::mem::transmute_copy(&penstatus), core::mem::transmute_copy(&pbstrnotes)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCPresenceStatusEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCPresenceStatusEvent {}
windows_core::imp::define_interface!(IRTCProfile, IRTCProfile_Vtbl, 0xd07eca9e_4062_4dd4_9e7d_722a49ba7303);
windows_core::imp::interface_hierarchy!(IRTCProfile, windows_core::IUnknown);
impl IRTCProfile {
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn XML(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XML)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProviderURI)(windows_core::Interface::as_raw(self), enuri, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProviderData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClientBanner(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientBanner)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClientMinVer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientMinVer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClientCurVer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientCurVer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClientUpdateURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientUpdateURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClientData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCredentials(&self, bstruseruri: &windows_core::BSTR, bstruseraccount: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstruseruri), core::mem::transmute_copy(bstruseraccount), core::mem::transmute_copy(bstrpassword)).ok() }
    }
    pub unsafe fn SessionCapabilities(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProviderURI: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_PROVIDER_URI, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientBanner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClientMinVer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientCurVer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientUpdateURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT,
}
pub trait IRTCProfile_Impl: windows_core::IUnknownImpl {
    fn Key(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn XML(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> windows_core::Result<windows_core::BSTR>;
    fn ProviderData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientBanner(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClientMinVer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientCurVer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientUpdateURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCredentials(&self, bstruseruri: &windows_core::BSTR, bstruseraccount: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SessionCapabilities(&self) -> windows_core::Result<i32>;
    fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE>;
}
impl IRTCProfile_Vtbl {
    pub const fn new<Identity: IRTCProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::Key(this) {
                    Ok(ok__) => {
                        pbstrkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn XML<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::XML(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProviderName<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ProviderName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProviderURI<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::get_ProviderURI(this, core::mem::transmute_copy(&enuri)) {
                    Ok(ok__) => {
                        pbstruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProviderData<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ProviderData(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientName<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientBanner<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbanner: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientBanner(this) {
                    Ok(ok__) => {
                        pfbanner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientMinVer<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrminver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientMinVer(this) {
                    Ok(ok__) => {
                        pbstrminver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientCurVer<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcurver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientCurVer(this) {
                    Ok(ok__) => {
                        pbstrcurver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrupdateuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientUpdateURI(this) {
                    Ok(ok__) => {
                        pbstrupdateuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientData<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::ClientData(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserURI<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::UserURI(this) {
                    Ok(ok__) => {
                        pbstruseruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserName<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::UserName(this) {
                    Ok(ok__) => {
                        pbstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserAccount<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::UserAccount(this) {
                    Ok(ok__) => {
                        pbstruseraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseruri: *mut core::ffi::c_void, bstruseraccount: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCProfile_Impl::SetCredentials(this, core::mem::transmute(&bstruseruri), core::mem::transmute(&bstruseraccount), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn SessionCapabilities<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsupportedsessions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::SessionCapabilities(this) {
                    Ok(ok__) => {
                        plsupportedsessions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Key: Key::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            XML: XML::<Identity, OFFSET>,
            ProviderName: ProviderName::<Identity, OFFSET>,
            get_ProviderURI: get_ProviderURI::<Identity, OFFSET>,
            ProviderData: ProviderData::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            ClientBanner: ClientBanner::<Identity, OFFSET>,
            ClientMinVer: ClientMinVer::<Identity, OFFSET>,
            ClientCurVer: ClientCurVer::<Identity, OFFSET>,
            ClientUpdateURI: ClientUpdateURI::<Identity, OFFSET>,
            ClientData: ClientData::<Identity, OFFSET>,
            UserURI: UserURI::<Identity, OFFSET>,
            UserName: UserName::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCProfile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCProfile {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Realm)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRealm(&self, bstrrealm: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRealm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrealm)).ok() }
    }
    pub unsafe fn AllowedAuth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowedAuth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowedAuth(&self, lallowedauth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowedAuth)(windows_core::Interface::as_raw(self), lallowedauth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile2_Vtbl {
    pub base__: IRTCProfile_Vtbl,
    pub Realm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRealm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowedAuth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAllowedAuth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IRTCProfile2_Impl: IRTCProfile_Impl {
    fn Realm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRealm(&self, bstrrealm: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowedAuth(&self) -> windows_core::Result<i32>;
    fn SetAllowedAuth(&self, lallowedauth: i32) -> windows_core::Result<()>;
}
impl IRTCProfile2_Vtbl {
    pub const fn new<Identity: IRTCProfile2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Realm<Identity: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrealm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile2_Impl::Realm(this) {
                    Ok(ok__) => {
                        pbstrrealm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRealm<Identity: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCProfile2_Impl::SetRealm(this, core::mem::transmute(&bstrrealm)).into()
            }
        }
        unsafe extern "system" fn AllowedAuth<Identity: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plallowedauth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfile2_Impl::AllowedAuth(this) {
                    Ok(ok__) => {
                        plallowedauth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Identity: IRTCProfile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lallowedauth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCProfile2_Impl::SetAllowedAuth(this, core::mem::transmute_copy(&lallowedauth)).into()
            }
        }
        Self {
            base__: IRTCProfile_Vtbl::new::<Identity, OFFSET>(),
            Realm: Realm::<Identity, OFFSET>,
            SetRealm: SetRealm::<Identity, OFFSET>,
            AllowedAuth: AllowedAuth::<Identity, OFFSET>,
            SetAllowedAuth: SetAllowedAuth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCProfile2 as windows_core::Interface>::IID || iid == &<IRTCProfile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCProfile2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCProfileEvent_Impl: super::Com::IDispatch_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCProfileEvent_Vtbl {
    pub const fn new<Identity: IRTCProfileEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Profile<Identity: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfileEvent_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cookie<Identity: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfileEvent_Impl::Cookie(this) {
                    Ok(ok__) => {
                        plcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCProfileEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfileEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Profile: Profile::<Identity, OFFSET>,
            Cookie: Cookie::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCProfileEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCProfileEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent2_Vtbl {
    pub base__: IRTCProfileEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_PROFILE_EVENT_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCProfileEvent2_Impl: IRTCProfileEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCProfileEvent2_Vtbl {
    pub const fn new<Identity: IRTCProfileEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCProfileEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCProfileEvent2_Impl::EventType(this) {
                    Ok(ok__) => {
                        peventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRTCProfileEvent_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCProfileEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCProfileEvent2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Accept(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcontenttype), core::mem::transmute_copy(bstrsessiondescription)).ok() }
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCReInviteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REINVITE_STATE) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCReInviteEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn Accept(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCReInviteEvent_Vtbl {
    pub const fn new<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCReInviteEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Accept<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: *mut core::ffi::c_void, bstrsessiondescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCReInviteEvent_Impl::Accept(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription)).into()
            }
        }
        unsafe extern "system" fn Reject<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCReInviteEvent_Impl::Reject(this).into()
            }
        }
        unsafe extern "system" fn State<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCReInviteEvent_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: IRTCReInviteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut *mut core::ffi::c_void, pbstrsessiondescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCReInviteEvent_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Accept: Accept::<Identity, OFFSET>,
            Reject: Reject::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCReInviteEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCReInviteEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRegistrationStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCRegistrationStateChangeEvent_Impl: super::Com::IDispatch_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCRegistrationStateChangeEvent_Vtbl {
    pub const fn new<Identity: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Profile<Identity: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRegistrationStateChangeEvent_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRegistrationStateChangeEvent_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRegistrationStateChangeEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRegistrationStateChangeEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Profile: Profile::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCRegistrationStateChangeEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCRegistrationStateChangeEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRoamingEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_ROAMING_EVENT_TYPE) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCRoamingEvent_Impl: super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCRoamingEvent_Vtbl {
    pub const fn new<Identity: IRTCRoamingEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRoamingEvent_Impl::EventType(this) {
                    Ok(ok__) => {
                        peventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Profile<Identity: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRoamingEvent_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRoamingEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCRoamingEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCRoamingEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EventType: EventType::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCRoamingEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCRoamingEvent {}
windows_core::imp::define_interface!(IRTCSession, IRTCSession_Vtbl, 0x387c8086_99be_42fb_9973_7c0fc0ca9fa8);
windows_core::imp::interface_hierarchy!(IRTCSession, windows_core::IUnknown);
impl IRTCSession {
    pub unsafe fn Client(&self) -> windows_core::Result<IRTCClient> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Client)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_SESSION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<RTC_SESSION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Participants(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Participants)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Answer(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Answer)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), enreason).ok() }
    }
    pub unsafe fn Redirect<P2>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: P2, lflags: i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IRTCProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).Redirect)(windows_core::Interface::as_raw(self), entype, core::mem::transmute_copy(bstrlocalphoneuri), pprofile.param().abi(), lflags).ok() }
    }
    pub unsafe fn AddParticipant(&self, bstraddress: &windows_core::BSTR, bstrname: &windows_core::BSTR) -> windows_core::Result<IRTCParticipant> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddParticipant)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraddress), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveParticipant<P0>(&self, pparticipant: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCParticipant>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveParticipant)(windows_core::Interface::as_raw(self), pparticipant.param().abi()).ok() }
    }
    pub unsafe fn EnumerateParticipants(&self) -> windows_core::Result<IRTCEnumParticipants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateParticipants)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CanAddParticipants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanAddParticipants)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectedUserURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectedUserURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RedirectedUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectedUserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NextRedirectedUser(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NextRedirectedUser)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendMessage(&self, bstrmessageheader: &windows_core::BSTR, bstrmessage: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessageheader), core::mem::transmute_copy(bstrmessage), lcookie).ok() }
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMessageStatus)(windows_core::Interface::as_raw(self), enuserstatus, lcookie).ok() }
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), lmediatype, lcookie).ok() }
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), lmediatype, lcookie).ok() }
    }
    pub unsafe fn put_EncryptionKey(&self, lmediatype: i32, encryptionkey: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_EncryptionKey)(windows_core::Interface::as_raw(self), lmediatype, core::mem::transmute_copy(encryptionkey)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub Redirect: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddParticipant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveParticipant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanAddParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RedirectedUserURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedirectedUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextRedirectedUser: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub SendMessageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_MESSAGING_USER_STATUS, isize) -> windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize) -> windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, isize) -> windows_core::HRESULT,
    pub put_EncryptionKey: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSession_Impl: windows_core::IUnknownImpl {
    fn Client(&self) -> windows_core::Result<IRTCClient>;
    fn State(&self) -> windows_core::Result<RTC_SESSION_STATE>;
    fn Type(&self) -> windows_core::Result<RTC_SESSION_TYPE>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn Participants(&self) -> windows_core::Result<IRTCCollection>;
    fn Answer(&self) -> windows_core::Result<()>;
    fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> windows_core::Result<()>;
    fn Redirect(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: windows_core::Ref<IRTCProfile>, lflags: i32) -> windows_core::Result<()>;
    fn AddParticipant(&self, bstraddress: &windows_core::BSTR, bstrname: &windows_core::BSTR) -> windows_core::Result<IRTCParticipant>;
    fn RemoveParticipant(&self, pparticipant: windows_core::Ref<IRTCParticipant>) -> windows_core::Result<()>;
    fn EnumerateParticipants(&self) -> windows_core::Result<IRTCEnumParticipants>;
    fn CanAddParticipants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectedUserURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RedirectedUserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NextRedirectedUser(&self) -> windows_core::Result<()>;
    fn SendMessage(&self, bstrmessageheader: &windows_core::BSTR, bstrmessage: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()>;
    fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> windows_core::Result<()>;
    fn AddStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()>;
    fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> windows_core::Result<()>;
    fn put_EncryptionKey(&self, lmediatype: i32, encryptionkey: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSession_Vtbl {
    pub const fn new<Identity: IRTCSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Client<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::Client(this) {
                    Ok(ok__) => {
                        ppclient.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::Type(this) {
                    Ok(ok__) => {
                        pentype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Profile<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Participants<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::Participants(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Answer<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::Answer(this).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::Terminate(this, core::mem::transmute_copy(&enreason)).into()
            }
        }
        unsafe extern "system" fn Redirect<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::Redirect(this, core::mem::transmute_copy(&entype), core::mem::transmute(&bstrlocalphoneuri), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn AddParticipant<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraddress: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::AddParticipant(this, core::mem::transmute(&bstraddress), core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppparticipant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveParticipant<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparticipant: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::RemoveParticipant(this, core::mem::transmute_copy(&pparticipant)).into()
            }
        }
        unsafe extern "system" fn EnumerateParticipants<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::EnumerateParticipants(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanAddParticipants<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcanadd: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::CanAddParticipants(this) {
                    Ok(ok__) => {
                        pfcanadd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::RedirectedUserURI(this) {
                    Ok(ok__) => {
                        pbstruseruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectedUserName<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession_Impl::RedirectedUserName(this) {
                    Ok(ok__) => {
                        pbstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::NextRedirectedUser(this).into()
            }
        }
        unsafe extern "system" fn SendMessage<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageheader: *mut core::ffi::c_void, bstrmessage: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::SendMessage(this, core::mem::transmute(&bstrmessageheader), core::mem::transmute(&bstrmessage), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn SendMessageStatus<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::SendMessageStatus(this, core::mem::transmute_copy(&enuserstatus), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn AddStream<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::AddStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::RemoveStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn put_EncryptionKey<Identity: IRTCSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, encryptionkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession_Impl::put_EncryptionKey(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute(&encryptionkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Client: Client::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
            Participants: Participants::<Identity, OFFSET>,
            Answer: Answer::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            Redirect: Redirect::<Identity, OFFSET>,
            AddParticipant: AddParticipant::<Identity, OFFSET>,
            RemoveParticipant: RemoveParticipant::<Identity, OFFSET>,
            EnumerateParticipants: EnumerateParticipants::<Identity, OFFSET>,
            CanAddParticipants: CanAddParticipants::<Identity, OFFSET>,
            RedirectedUserURI: RedirectedUserURI::<Identity, OFFSET>,
            RedirectedUserName: RedirectedUserName::<Identity, OFFSET>,
            NextRedirectedUser: NextRedirectedUser::<Identity, OFFSET>,
            SendMessage: SendMessage::<Identity, OFFSET>,
            SendMessageStatus: SendMessageStatus::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            put_EncryptionKey: put_EncryptionKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSession {}
windows_core::imp::define_interface!(IRTCSession2, IRTCSession2_Vtbl, 0x17d7cdfc_b007_484c_99d2_86a8a820991d);
impl core::ops::Deref for IRTCSession2 {
    type Target = IRTCSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRTCSession2, windows_core::IUnknown, IRTCSession);
impl IRTCSession2 {
    pub unsafe fn SendInfo(&self, bstrinfoheader: &windows_core::BSTR, bstrinfo: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinfoheader), core::mem::transmute_copy(bstrinfo), lcookie).ok() }
    }
    pub unsafe fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, ensecuritylevel).ok() }
    }
    pub unsafe fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AnswerWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AnswerWithSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcontenttype), core::mem::transmute_copy(bstrsessiondescription)).ok() }
    }
    pub unsafe fn ReInviteWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReInviteWithSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcontenttype), core::mem::transmute_copy(bstrsessiondescription), lcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession2_Vtbl {
    pub base__: IRTCSession_Vtbl,
    pub SendInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub put_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub get_PreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AnswerWithSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReInviteWithSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSession2_Impl: IRTCSession_Impl {
    fn SendInfo(&self, bstrinfoheader: &windows_core::BSTR, bstrinfo: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReInviteWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSession2_Vtbl {
    pub const fn new<Identity: IRTCSession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendInfo<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinfoheader: *mut core::ffi::c_void, bstrinfo: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession2_Impl::SendInfo(this, core::mem::transmute(&bstrinfoheader), core::mem::transmute(&bstrinfo), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession2_Impl::put_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype), core::mem::transmute_copy(&ensecuritylevel)).into()
            }
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession2_Impl::get_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                    Ok(ok__) => {
                        pensecuritylevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSession2_Impl::IsSecurityEnabled(this, core::mem::transmute_copy(&ensecuritytype)) {
                    Ok(ok__) => {
                        pfsecurityenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: *mut core::ffi::c_void, bstrsessiondescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession2_Impl::AnswerWithSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription)).into()
            }
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Identity: IRTCSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: *mut core::ffi::c_void, bstrsessiondescription: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSession2_Impl::ReInviteWithSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        Self {
            base__: IRTCSession_Vtbl::new::<Identity, OFFSET>(),
            SendInfo: SendInfo::<Identity, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            AnswerWithSessionDescription: AnswerWithSessionDescription::<Identity, OFFSET>,
            ReInviteWithSessionDescription: ReInviteWithSessionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSession2 as windows_core::Interface>::IID || iid == &<IRTCSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSession2 {}
windows_core::imp::define_interface!(IRTCSessionCallControl, IRTCSessionCallControl_Vtbl, 0xe9a50d94_190b_4f82_9530_3b8ebf60758a);
windows_core::imp::interface_hierarchy!(IRTCSessionCallControl, windows_core::IUnknown);
impl IRTCSessionCallControl {
    pub unsafe fn Hold(&self, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Hold)(windows_core::Interface::as_raw(self), lcookie).ok() }
    }
    pub unsafe fn UnHold(&self, lcookie: isize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnHold)(windows_core::Interface::as_raw(self), lcookie).ok() }
    }
    pub unsafe fn Forward(&self, bstrforwardtouri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Forward)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrforwardtouri)).ok() }
    }
    pub unsafe fn Refer(&self, bstrrefertouri: &windows_core::BSTR, bstrrefercookie: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrefertouri), core::mem::transmute_copy(bstrrefercookie)).ok() }
    }
    pub unsafe fn SetReferredByURI(&self, bstrreferredbyuri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReferredByURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrreferredbyuri)).ok() }
    }
    pub unsafe fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferredByURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReferCookie(&self, bstrrefercookie: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReferCookie)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrefercookie)).ok() }
    }
    pub unsafe fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsReferred(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsReferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionCallControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Hold: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub UnHold: unsafe extern "system" fn(*mut core::ffi::c_void, isize) -> windows_core::HRESULT,
    pub Forward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsReferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IRTCSessionCallControl_Impl: windows_core::IUnknownImpl {
    fn Hold(&self, lcookie: isize) -> windows_core::Result<()>;
    fn UnHold(&self, lcookie: isize) -> windows_core::Result<()>;
    fn Forward(&self, bstrforwardtouri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Refer(&self, bstrrefertouri: &windows_core::BSTR, bstrrefercookie: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetReferredByURI(&self, bstrreferredbyuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReferCookie(&self, bstrrefercookie: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsReferred(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl IRTCSessionCallControl_Vtbl {
    pub const fn new<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Hold<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::Hold(this, core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn UnHold<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::UnHold(this, core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn Forward<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrforwardtouri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::Forward(this, core::mem::transmute(&bstrforwardtouri)).into()
            }
        }
        unsafe extern "system" fn Refer<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrefertouri: *mut core::ffi::c_void, bstrrefercookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::Refer(this, core::mem::transmute(&bstrrefertouri), core::mem::transmute(&bstrrefercookie)).into()
            }
        }
        unsafe extern "system" fn SetReferredByURI<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreferredbyuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::SetReferredByURI(this, core::mem::transmute(&bstrreferredbyuri)).into()
            }
        }
        unsafe extern "system" fn ReferredByURI<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferredbyuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionCallControl_Impl::ReferredByURI(this) {
                    Ok(ok__) => {
                        pbstrreferredbyuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReferCookie<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrefercookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionCallControl_Impl::SetReferCookie(this, core::mem::transmute(&bstrrefercookie)).into()
            }
        }
        unsafe extern "system" fn ReferCookie<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrefercookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionCallControl_Impl::ReferCookie(this) {
                    Ok(ok__) => {
                        pbstrrefercookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsReferred<Identity: IRTCSessionCallControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisreferred: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionCallControl_Impl::IsReferred(this) {
                    Ok(ok__) => {
                        pfisreferred.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Hold: Hold::<Identity, OFFSET>,
            UnHold: UnHold::<Identity, OFFSET>,
            Forward: Forward::<Identity, OFFSET>,
            Refer: Refer::<Identity, OFFSET>,
            SetReferredByURI: SetReferredByURI::<Identity, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, OFFSET>,
            SetReferCookie: SetReferCookie::<Identity, OFFSET>,
            ReferCookie: ReferCookie::<Identity, OFFSET>,
            IsReferred: IsReferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionCallControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCSessionCallControl {}
windows_core::imp::define_interface!(IRTCSessionDescriptionManager, IRTCSessionDescriptionManager_Vtbl, 0xba7f518e_d336_4070_93a6_865395c843f9);
windows_core::imp::interface_hierarchy!(IRTCSessionDescriptionManager, windows_core::IUnknown);
impl IRTCSessionDescriptionManager {
    pub unsafe fn EvaluateSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EvaluateSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcontenttype), core::mem::transmute_copy(bstrsessiondescription), pfapplicationsession as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionDescriptionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EvaluateSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IRTCSessionDescriptionManager_Impl: windows_core::IUnknownImpl {
    fn EvaluateSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl IRTCSessionDescriptionManager_Vtbl {
    pub const fn new<Identity: IRTCSessionDescriptionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EvaluateSessionDescription<Identity: IRTCSessionDescriptionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: *mut core::ffi::c_void, bstrsessiondescription: *mut core::ffi::c_void, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionDescriptionManager_Impl::EvaluateSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), core::mem::transmute_copy(&pfapplicationsession)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EvaluateSessionDescription: EvaluateSessionDescription::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionDescriptionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCSessionDescriptionManager {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionOperationCompleteEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionOperationCompleteEvent_Vtbl {
    pub const fn new<Identity: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionOperationCompleteEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cookie<Identity: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionOperationCompleteEvent_Impl::Cookie(this) {
                    Ok(ok__) => {
                        plcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionOperationCompleteEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionOperationCompleteEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Cookie: Cookie::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionOperationCompleteEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Participant)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent2_Vtbl {
    pub base__: IRTCSessionOperationCompleteEvent_Vtbl,
    pub Participant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionOperationCompleteEvent2_Impl: IRTCSessionOperationCompleteEvent_Impl {
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionOperationCompleteEvent2_Vtbl {
    pub const fn new<Identity: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Participant<Identity: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionOperationCompleteEvent2_Impl::Participant(this) {
                    Ok(ok__) => {
                        ppparticipant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut *mut core::ffi::c_void, pbstrsessiondescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionOperationCompleteEvent2_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
            }
        }
        Self {
            base__: IRTCSessionOperationCompleteEvent_Vtbl::new::<Identity, OFFSET>(),
            Participant: Participant::<Identity, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionOperationCompleteEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCSessionOperationCompleteEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionOperationCompleteEvent2 {}
windows_core::imp::define_interface!(IRTCSessionPortManagement, IRTCSessionPortManagement_Vtbl, 0xa072f1d6_0286_4e1f_85f2_17a2948456ec);
windows_core::imp::interface_hierarchy!(IRTCSessionPortManagement, windows_core::IUnknown);
impl IRTCSessionPortManagement {
    pub unsafe fn SetPortManager<P0>(&self, pportmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCPortManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPortManager)(windows_core::Interface::as_raw(self), pportmanager.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionPortManagement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPortManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCSessionPortManagement_Impl: windows_core::IUnknownImpl {
    fn SetPortManager(&self, pportmanager: windows_core::Ref<IRTCPortManager>) -> windows_core::Result<()>;
}
impl IRTCSessionPortManagement_Vtbl {
    pub const fn new<Identity: IRTCSessionPortManagement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPortManager<Identity: IRTCSessionPortManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionPortManagement_Impl::SetPortManager(this, core::mem::transmute_copy(&pportmanager)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetPortManager: SetPortManager::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionPortManagement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCSessionPortManagement {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReferStatus(&self) -> windows_core::Result<RTC_SESSION_REFER_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferStatusEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_REFER_STATUS) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionReferStatusEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ReferStatus(&self) -> windows_core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionReferStatusEvent_Vtbl {
    pub const fn new<Identity: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferStatusEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferStatus<Identity: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferStatusEvent_Impl::ReferStatus(this) {
                    Ok(ok__) => {
                        penreferstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferStatusEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCSessionReferStatusEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferStatusEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            ReferStatus: ReferStatus::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionReferStatusEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionReferStatusEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferredByURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ReferToURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferToURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Accept(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Accept)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reject(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reject)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReferredSessionState)(windows_core::Interface::as_raw(self), enstate).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferredEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferredByURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferToURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReferredSessionState: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SESSION_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionReferredEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReferToURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Accept(&self) -> windows_core::Result<()>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionReferredEvent_Vtbl {
    pub const fn new<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferredEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferredByURI<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferredbyuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferredEvent_Impl::ReferredByURI(this) {
                    Ok(ok__) => {
                        pbstrreferredbyuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferToURI<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferouri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferredEvent_Impl::ReferToURI(this) {
                    Ok(ok__) => {
                        pbstrreferouri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferCookie<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrefercookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionReferredEvent_Impl::ReferCookie(this) {
                    Ok(ok__) => {
                        pbstrrefercookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Accept<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionReferredEvent_Impl::Accept(this).into()
            }
        }
        unsafe extern "system" fn Reject<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionReferredEvent_Impl::Reject(this).into()
            }
        }
        unsafe extern "system" fn SetReferredSessionState<Identity: IRTCSessionReferredEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstate: RTC_SESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionReferredEvent_Impl::SetReferredSessionState(this, core::mem::transmute_copy(&enstate)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, OFFSET>,
            ReferToURI: ReferToURI::<Identity, OFFSET>,
            ReferCookie: ReferCookie::<Identity, OFFSET>,
            Accept: Accept::<Identity, OFFSET>,
            Reject: Reject::<Identity, OFFSET>,
            SetReferredSessionState: SetReferredSessionState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionReferredEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionReferredEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<RTC_SESSION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_SESSION_STATE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionStateChangeEvent_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn State(&self) -> windows_core::Result<RTC_SESSION_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionStateChangeEvent_Vtbl {
    pub const fn new<Identity: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent_Impl::Session(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusText<Identity: IRTCSessionStateChangeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent_Impl::StatusText(this) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionStateChangeEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemotePreferredSecurityLevel)(windows_core::Interface::as_raw(self), ensecuritytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsForked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsForked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRemoteSessionDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcontenttype), core::mem::transmute(pbstrsessiondescription)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent2_Vtbl {
    pub base__: IRTCSessionStateChangeEvent_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RemotePreferredSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_SECURITY_TYPE, *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT,
    pub IsForked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetRemoteSessionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionStateChangeEvent2_Impl: IRTCSessionStateChangeEvent_Impl {
    fn MediaTypes(&self) -> windows_core::Result<i32>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCSessionStateChangeEvent2_Vtbl {
    pub const fn new<Identity: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MediaTypes<Identity: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent2_Impl::MediaTypes(this) {
                    Ok(ok__) => {
                        pmediatypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent2_Impl::get_RemotePreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                    Ok(ok__) => {
                        pensecuritylevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsForked<Identity: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisforked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCSessionStateChangeEvent2_Impl::IsForked(this) {
                    Ok(ok__) => {
                        pfisforked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: IRTCSessionStateChangeEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut *mut core::ffi::c_void, pbstrsessiondescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCSessionStateChangeEvent2_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
            }
        }
        Self {
            base__: IRTCSessionStateChangeEvent_Vtbl::new::<Identity, OFFSET>(),
            MediaTypes: MediaTypes::<Identity, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, OFFSET>,
            IsForked: IsForked::<Identity, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionStateChangeEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCSessionStateChangeEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCSessionStateChangeEvent2 {}
windows_core::imp::define_interface!(IRTCUserSearch, IRTCUserSearch_Vtbl, 0xb619882b_860c_4db4_be1b_693b6505bbe5);
windows_core::imp::interface_hierarchy!(IRTCUserSearch, windows_core::IUnknown);
impl IRTCUserSearch {
    pub unsafe fn CreateQuery(&self) -> windows_core::Result<IRTCUserSearchQuery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecuteSearch<P0, P1>(&self, pquery: P0, pprofile: P1, lcookie: isize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRTCUserSearchQuery>,
        P1: windows_core::Param<IRTCProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecuteSearch)(windows_core::Interface::as_raw(self), pquery.param().abi(), pprofile.param().abi(), lcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, isize) -> windows_core::HRESULT,
}
pub trait IRTCUserSearch_Impl: windows_core::IUnknownImpl {
    fn CreateQuery(&self) -> windows_core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(&self, pquery: windows_core::Ref<IRTCUserSearchQuery>, pprofile: windows_core::Ref<IRTCProfile>, lcookie: isize) -> windows_core::Result<()>;
}
impl IRTCUserSearch_Vtbl {
    pub const fn new<Identity: IRTCUserSearch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateQuery<Identity: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearch_Impl::CreateQuery(this) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecuteSearch<Identity: IRTCUserSearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCUserSearch_Impl::ExecuteSearch(this, core::mem::transmute_copy(&pquery), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&lcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCUserSearch as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCUserSearch {}
windows_core::imp::define_interface!(IRTCUserSearchQuery, IRTCUserSearchQuery_Vtbl, 0x288300f5_d23a_4365_9a73_9985c98c2881);
windows_core::imp::interface_hierarchy!(IRTCUserSearchQuery, windows_core::IUnknown);
impl IRTCUserSearchQuery {
    pub unsafe fn put_SearchTerm(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_SearchTerm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue)).ok() }
    }
    pub unsafe fn get_SearchTerm(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SearchTerm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SearchTerms(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchTerms)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_SearchPreference)(windows_core::Interface::as_raw(self), enpreference, lvalue).ok() }
    }
    pub unsafe fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SearchPreference)(windows_core::Interface::as_raw(self), enpreference, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSearchDomain(&self, bstrdomain: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSearchDomain)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdomain)).ok() }
    }
    pub unsafe fn SearchDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_SearchTerm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SearchTerm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchTerms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_PREFERENCE, i32) -> windows_core::HRESULT,
    pub get_SearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_PREFERENCE, *mut i32) -> windows_core::HRESULT,
    pub SetSearchDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCUserSearchQuery_Impl: windows_core::IUnknownImpl {
    fn put_SearchTerm(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SearchTerm(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SearchTerms(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::Result<()>;
    fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> windows_core::Result<i32>;
    fn SetSearchDomain(&self, bstrdomain: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SearchDomain(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IRTCUserSearchQuery_Vtbl {
    pub const fn new<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_SearchTerm<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCUserSearchQuery_Impl::put_SearchTerm(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn get_SearchTerm<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchQuery_Impl::get_SearchTerm(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchTerms<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnames: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchQuery_Impl::SearchTerms(this) {
                    Ok(ok__) => {
                        pbstrnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SearchPreference<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCUserSearchQuery_Impl::put_SearchPreference(this, core::mem::transmute_copy(&enpreference), core::mem::transmute_copy(&lvalue)).into()
            }
        }
        unsafe extern "system" fn get_SearchPreference<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchQuery_Impl::get_SearchPreference(this, core::mem::transmute_copy(&enpreference)) {
                    Ok(ok__) => {
                        plvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSearchDomain<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCUserSearchQuery_Impl::SetSearchDomain(this, core::mem::transmute(&bstrdomain)).into()
            }
        }
        unsafe extern "system" fn SearchDomain<Identity: IRTCUserSearchQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchQuery_Impl::SearchDomain(this) {
                    Ok(ok__) => {
                        pbstrdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_SearchTerm: put_SearchTerm::<Identity, OFFSET>,
            get_SearchTerm: get_SearchTerm::<Identity, OFFSET>,
            SearchTerms: SearchTerms::<Identity, OFFSET>,
            put_SearchPreference: put_SearchPreference::<Identity, OFFSET>,
            get_SearchPreference: get_SearchPreference::<Identity, OFFSET>,
            SetSearchDomain: SetSearchDomain::<Identity, OFFSET>,
            SearchDomain: SearchDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCUserSearchQuery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCUserSearchQuery {}
windows_core::imp::define_interface!(IRTCUserSearchResult, IRTCUserSearchResult_Vtbl, 0x851278b2_9592_480f_8db5_2de86b26b54d);
windows_core::imp::interface_hierarchy!(IRTCUserSearchResult, windows_core::IUnknown);
impl IRTCUserSearchResult {
    pub unsafe fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), encolumn, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_USER_SEARCH_COLUMN, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRTCUserSearchResult_Impl: windows_core::IUnknownImpl {
    fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> windows_core::Result<windows_core::BSTR>;
}
impl IRTCUserSearchResult_Vtbl {
    pub const fn new<Identity: IRTCUserSearchResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: IRTCUserSearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResult_Impl::get_Value(this, core::mem::transmute_copy(&encolumn)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_Value: get_Value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCUserSearchResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCUserSearchResult {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Results(&self) -> windows_core::Result<IRTCCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<IRTCProfile2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Query(&self) -> windows_core::Result<IRTCUserSearchQuery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cookie(&self) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoreAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoreAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResultsEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub EnumerateResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut isize) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MoreAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCUserSearchResultsEvent_Impl: super::Com::IDispatch_Impl {
    fn EnumerateResults(&self) -> windows_core::Result<IRTCEnumUserSearchResults>;
    fn Results(&self) -> windows_core::Result<IRTCCollection>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn Query(&self) -> windows_core::Result<IRTCUserSearchQuery>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn MoreAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCUserSearchResultsEvent_Vtbl {
    pub const fn new<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumerateResults<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::EnumerateResults(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Results<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::Results(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Profile<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Query<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::Query(this) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cookie<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::Cookie(this) {
                    Ok(ok__) => {
                        plcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoreAvailable<Identity: IRTCUserSearchResultsEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmoreavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCUserSearchResultsEvent_Impl::MoreAvailable(this) {
                    Ok(ok__) => {
                        pfmoreavailable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumerateResults: EnumerateResults::<Identity, OFFSET>,
            Results: Results::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Cookie: Cookie::<Identity, OFFSET>,
            StatusCode: StatusCode::<Identity, OFFSET>,
            MoreAvailable: MoreAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCUserSearchResultsEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCUserSearchResultsEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), enstate).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher_Vtbl {
    pub base__: IRTCPresenceContact_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_WATCHER_STATE) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, RTC_WATCHER_STATE) -> windows_core::HRESULT,
}
pub trait IRTCWatcher_Impl: IRTCPresenceContact_Impl {
    fn State(&self) -> windows_core::Result<RTC_WATCHER_STATE>;
    fn SetState(&self, enstate: RTC_WATCHER_STATE) -> windows_core::Result<()>;
}
impl IRTCWatcher_Vtbl {
    pub const fn new<Identity: IRTCWatcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn State<Identity: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcher_Impl::State(this) {
                    Ok(ok__) => {
                        penstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetState<Identity: IRTCWatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRTCWatcher_Impl::SetState(this, core::mem::transmute_copy(&enstate)).into()
            }
        }
        Self { base__: IRTCPresenceContact_Vtbl::new::<Identity, OFFSET>(), State: State::<Identity, OFFSET>, SetState: SetState::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcher as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCWatcher {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<RTC_ACE_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher2_Vtbl {
    pub base__: IRTCWatcher_Vtbl,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_ACE_SCOPE) -> windows_core::HRESULT,
}
pub trait IRTCWatcher2_Impl: IRTCWatcher_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn Scope(&self) -> windows_core::Result<RTC_ACE_SCOPE>;
}
impl IRTCWatcher2_Vtbl {
    pub const fn new<Identity: IRTCWatcher2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Profile<Identity: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcher2_Impl::Profile(this) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Scope<Identity: IRTCWatcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcher2_Impl::Scope(this) {
                    Ok(ok__) => {
                        penscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRTCWatcher_Vtbl::new::<Identity, OFFSET>(), Profile: Profile::<Identity, OFFSET>, Scope: Scope::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcher2 as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID || iid == &<IRTCWatcher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRTCWatcher2 {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Watcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Watcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCWatcherEvent_Impl: super::Com::IDispatch_Impl {
    fn Watcher(&self) -> windows_core::Result<IRTCWatcher>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCWatcherEvent_Vtbl {
    pub const fn new<Identity: IRTCWatcherEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Watcher<Identity: IRTCWatcherEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcherEvent_Impl::Watcher(this) {
                    Ok(ok__) => {
                        ppwatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Watcher: Watcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCWatcherEvent {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StatusCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent2_Vtbl {
    pub base__: IRTCWatcherEvent_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RTC_WATCHER_EVENT_TYPE) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCWatcherEvent2_Impl: IRTCWatcherEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRTCWatcherEvent2_Vtbl {
    pub const fn new<Identity: IRTCWatcherEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcherEvent2_Impl::EventType(this) {
                    Ok(ok__) => {
                        peventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StatusCode<Identity: IRTCWatcherEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRTCWatcherEvent2_Impl::StatusCode(this) {
                    Ok(ok__) => {
                        plstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRTCWatcherEvent_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET>, StatusCode: StatusCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcherEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCWatcherEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRTCWatcherEvent2 {}
windows_core::imp::define_interface!(ITransportSettingsInternal, ITransportSettingsInternal_Vtbl, 0x5123e076_29e3_4bfd_84fe_0192d411e3e8);
windows_core::imp::interface_hierarchy!(ITransportSettingsInternal, windows_core::IUnknown);
impl ITransportSettingsInternal {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplySetting)(windows_core::Interface::as_raw(self), setting as _).ok() }
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QuerySetting)(windows_core::Interface::as_raw(self), setting as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternal_Impl: windows_core::IUnknownImpl {
    fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()>;
    fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternal_Vtbl {
    pub const fn new<Identity: ITransportSettingsInternal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplySetting<Identity: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransportSettingsInternal_Impl::ApplySetting(this, core::mem::transmute_copy(&setting)).into()
            }
        }
        unsafe extern "system" fn QuerySetting<Identity: ITransportSettingsInternal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransportSettingsInternal_Impl::QuerySetting(this, core::mem::transmute_copy(&setting)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ApplySetting: ApplySetting::<Identity, OFFSET>,
            QuerySetting: QuerySetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransportSettingsInternal as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::RuntimeName for ITransportSettingsInternal {}
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
pub const RTCClient: windows_core::GUID = windows_core::GUID::from_u128(0x7a42ea29_a2b7_40c4_b091_f6f024aa89be);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_ACE_SCOPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_ANSWER_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_AUDIO_DEVICE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_BUDDY_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_CLIENT_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_DTMF(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_EVENT(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_GROUP_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_LISTEN_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_MEDIA_EVENT_REASON(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_MEDIA_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_MESSAGING_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_MESSAGING_USER_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_OFFER_WATCHER_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PARTICIPANT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PORT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PRESENCE_PROPERTY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PRESENCE_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PRIVACY_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PROFILE_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_PROVIDER_URI(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_REGISTRATION_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_REINVITE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_RING_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_ROAMING_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_SECURITY_LEVEL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_SECURITY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_SESSION_REFER_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_SESSION_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_SESSION_TYPE(pub i32);
pub const RTC_S_ROAMING_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xEE0041_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_T120_APPLET(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_TERMINATE_REASON(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_USER_SEARCH_COLUMN(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_USER_SEARCH_PREFERENCE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_VIDEO_DEVICE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_WATCHER_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_WATCHER_MATCH_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTC_WATCHER_STATE(pub i32);
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSPORT_SETTING {
    pub SettingId: super::super::Networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
