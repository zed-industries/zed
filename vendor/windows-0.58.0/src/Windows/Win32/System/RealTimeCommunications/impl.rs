#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettings_Impl: Sized {
    fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()>;
    fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::RuntimeName for INetworkTransportSettings {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl INetworkTransportSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetworkTransportSettings_Vtbl
    where
        Identity: INetworkTransportSettings_Impl,
    {
        unsafe extern "system" fn ApplySetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: INetworkTransportSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkTransportSettings_Impl::ApplySetting(this, core::mem::transmute_copy(&settingid), core::mem::transmute_copy(&lengthin), core::mem::transmute_copy(&valuein), core::mem::transmute_copy(&lengthout), core::mem::transmute_copy(&valueout)).into()
        }
        unsafe extern "system" fn QuerySetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: INetworkTransportSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetworkTransportSettings_Impl::QuerySetting(this, core::mem::transmute_copy(&settingid), core::mem::transmute_copy(&lengthin), core::mem::transmute_copy(&valuein), core::mem::transmute_copy(&lengthout), core::mem::transmute_copy(&valueout)).into()
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
pub trait INotificationTransportSync_Impl: Sized {
    fn CompleteDelivery(&self) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INotificationTransportSync {}
impl INotificationTransportSync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INotificationTransportSync_Vtbl
    where
        Identity: INotificationTransportSync_Impl,
    {
        unsafe extern "system" fn CompleteDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INotificationTransportSync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INotificationTransportSync_Impl::CompleteDelivery(this).into()
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INotificationTransportSync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INotificationTransportSync_Impl::Flush(this).into()
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
pub trait IRTCBuddy_Impl: Sized + IRTCPresenceContact_Impl {
    fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IRTCBuddy {}
impl IRTCBuddy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddy_Vtbl
    where
        Identity: IRTCBuddy_Impl,
    {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy_Impl::Status(this) {
                Ok(ok__) => {
                    penstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnotes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy_Impl::Notes(this) {
                Ok(ok__) => {
                    pbstrnotes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IRTCPresenceContact_Vtbl::new::<Identity, OFFSET>(), Status: Status::<Identity, OFFSET>, Notes: Notes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddy as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddy2_Impl: Sized + IRTCBuddy_Impl {
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
impl windows_core::RuntimeName for IRTCBuddy2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddy2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddy2_Vtbl
    where
        Identity: IRTCBuddy2_Impl,
    {
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCBuddy2_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn EnumerateGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::EnumerateGroups(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::Groups(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                Ok(ok__) => {
                    pbstrproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::EnumeratePresenceDevices(this) {
                Ok(ok__) => {
                    ppenumdevices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevicescollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::PresenceDevices(this) {
                Ok(ok__) => {
                    ppdevicescollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCBuddy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddy2_Impl::SubscriptionType(this) {
                Ok(ok__) => {
                    pensubscriptiontype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCBuddyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Buddy(&self) -> windows_core::Result<IRTCBuddy>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddyEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddyEvent_Vtbl
    where
        Identity: IRTCBuddyEvent_Impl,
    {
        unsafe extern "system" fn Buddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyEvent_Impl::Buddy(this) {
                Ok(ok__) => {
                    ppbuddy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Buddy: Buddy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCBuddyEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyEvent2_Impl: Sized + IRTCBuddyEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddyEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddyEvent2_Vtbl
    where
        Identity: IRTCBuddyEvent2_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyEvent2_Impl::EventType(this) {
                Ok(ok__) => {
                    peventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyEvent2_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyEvent2_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroup_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddBuddy(&self, pbuddy: Option<&IRTCBuddy>) -> windows_core::Result<()>;
    fn RemoveBuddy(&self, pbuddy: Option<&IRTCBuddy>) -> windows_core::Result<()>;
    fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> windows_core::Result<IRTCCollection>;
    fn Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddyGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddyGroup_Vtbl
    where
        Identity: IRTCBuddyGroup_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgroupname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroup_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrgroupname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCBuddyGroup_Impl::SetName(this, core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn AddBuddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCBuddyGroup_Impl::AddBuddy(this, windows_core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn RemoveBuddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCBuddyGroup_Impl::RemoveBuddy(this, windows_core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroup_Impl::EnumerateBuddies(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroup_Impl::Buddies(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroup_Impl::Data(this) {
                Ok(ok__) => {
                    pbstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCBuddyGroup_Impl::SetData(this, core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroup_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCBuddyGroupEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(&self) -> windows_core::Result<IRTCBuddyGroup>;
    fn Buddy(&self) -> windows_core::Result<IRTCBuddy2>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCBuddyGroupEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCBuddyGroupEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCBuddyGroupEvent_Vtbl
    where
        Identity: IRTCBuddyGroupEvent_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroupEvent_Impl::EventType(this) {
                Ok(ok__) => {
                    peventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroupEvent_Impl::Group(this) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroupEvent_Impl::Buddy(this) {
                Ok(ok__) => {
                    ppbuddy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCBuddyGroupEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCBuddyGroupEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
pub trait IRTCClient_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn PrepareForShutdown(&self) -> windows_core::Result<()>;
    fn SetEventFilter(&self, lfilter: i32) -> windows_core::Result<()>;
    fn EventFilter(&self) -> windows_core::Result<i32>;
    fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PreferredMediaTypes(&self) -> windows_core::Result<i32>;
    fn MediaCapabilities(&self) -> windows_core::Result<i32>;
    fn CreateSession(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> windows_core::Result<()>;
    fn ListenForIncomingSessions(&self) -> windows_core::Result<RTC_LISTEN_MODE>;
    fn get_NetworkAddresses(&self, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::VARIANT>;
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
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IRTCClient {}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
impl IRTCClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClient_Vtbl
    where
        Identity: IRTCClient_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn PrepareForShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::PrepareForShutdown(this).into()
        }
        unsafe extern "system" fn SetEventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfilter: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetEventFilter(this, core::mem::transmute_copy(&lfilter)).into()
        }
        unsafe extern "system" fn EventFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfilter: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::EventFilter(this) {
                Ok(ok__) => {
                    plfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetPreferredMediaTypes(this, core::mem::transmute_copy(&lmediatypes), core::mem::transmute_copy(&fpersistent)).into()
        }
        unsafe extern "system" fn PreferredMediaTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::PreferredMediaTypes(this) {
                Ok(ok__) => {
                    plmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::MediaCapabilities(this) {
                Ok(ok__) => {
                    plmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: core::mem::MaybeUninit<windows_core::BSTR>, pprofile: *mut core::ffi::c_void, lflags: i32, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::CreateSession(this, core::mem::transmute_copy(&entype), core::mem::transmute(&bstrlocalphoneuri), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetListenForIncomingSessions(this, core::mem::transmute_copy(&enlisten)).into()
        }
        unsafe extern "system" fn ListenForIncomingSessions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::ListenForIncomingSessions(this) {
                Ok(ok__) => {
                    penlisten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NetworkAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL, pvaddresses: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_NetworkAddresses(this, core::mem::transmute_copy(&ftcp), core::mem::transmute_copy(&fexternal)) {
                Ok(ok__) => {
                    pvaddresses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Volume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::put_Volume(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn get_Volume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_Volume(this, core::mem::transmute_copy(&endevice)) {
                Ok(ok__) => {
                    plvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AudioMuted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::put_AudioMuted(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&fmuted)).into()
        }
        unsafe extern "system" fn get_AudioMuted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_AudioMuted(this, core::mem::transmute_copy(&endevice)) {
                Ok(ok__) => {
                    pfmuted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_IVideoWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_IVideoWindow(this, core::mem::transmute_copy(&endevice)) {
                Ok(ok__) => {
                    ppivideowindow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PreferredAudioDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::put_PreferredAudioDevice(this, core::mem::transmute_copy(&endevice), core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn get_PreferredAudioDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_PreferredAudioDevice(this, core::mem::transmute_copy(&endevice)) {
                Ok(ok__) => {
                    pbstrdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PreferredVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::put_PreferredVolume(this, core::mem::transmute_copy(&endevice), core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn get_PreferredVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_PreferredVolume(this, core::mem::transmute_copy(&endevice)) {
                Ok(ok__) => {
                    plvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredAEC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetPreferredAEC(this, core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn PreferredAEC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::PreferredAEC(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetPreferredVideoDevice(this, core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn PreferredVideoDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::PreferredVideoDevice(this) {
                Ok(ok__) => {
                    pbstrdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::ActiveMedia(this) {
                Ok(ok__) => {
                    plmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBitrate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbitrate: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetMaxBitrate(this, core::mem::transmute_copy(&lmaxbitrate)).into()
        }
        unsafe extern "system" fn MaxBitrate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxbitrate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::MaxBitrate(this) {
                Ok(ok__) => {
                    plmaxbitrate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvalue: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetTemporalSpatialTradeOff(this, core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::TemporalSpatialTradeOff(this) {
                Ok(ok__) => {
                    plvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnetworkquality: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::NetworkQuality(this) {
                Ok(ok__) => {
                    plnetworkquality.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartT120Applet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enapplet: RTC_T120_APPLET) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::StartT120Applet(this, core::mem::transmute_copy(&enapplet)).into()
        }
        unsafe extern "system" fn StopT120Applets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::StopT120Applets(this).into()
        }
        unsafe extern "system" fn get_IsT120AppletRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::get_IsT120AppletRunning(this, core::mem::transmute_copy(&enapplet)) {
                Ok(ok__) => {
                    pfrunning.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::LocalUserURI(this) {
                Ok(ok__) => {
                    pbstruseruri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseruri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetLocalUserURI(this, core::mem::transmute(&bstruseruri)).into()
        }
        unsafe extern "system" fn LocalUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::LocalUserName(this) {
                Ok(ok__) => {
                    pbstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SetLocalUserName(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn PlayRing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::PlayRing(this, core::mem::transmute_copy(&entype), core::mem::transmute_copy(&bplay)).into()
        }
        unsafe extern "system" fn SendDTMF<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endtmf: RTC_DTMF) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::SendDTMF(this, core::mem::transmute_copy(&endtmf)).into()
        }
        unsafe extern "system" fn InvokeTuningWizard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: isize) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient_Impl::InvokeTuningWizard(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn IsTuned<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftuned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient_Impl::IsTuned(this) {
                Ok(ok__) => {
                    pftuned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
pub trait IRTCClient2_Impl: Sized + IRTCClient_Impl {
    fn put_AnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> windows_core::Result<()>;
    fn get_AnswerMode(&self, entype: RTC_SESSION_TYPE) -> windows_core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<i32>;
    fn SetClientName(&self, bstrclientname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCurVer(&self, bstrclientcurver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeEx(&self, lflags: i32) -> windows_core::Result<()>;
    fn CreateSessionWithDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(&self, psessiondescriptionmanager: Option<&IRTCSessionDescriptionManager>) -> windows_core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn put_AllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::Result<()>;
    fn get_AllowedPorts(&self, ltransport: i32) -> windows_core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IRTCClient2 {}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_System_Com"))]
impl IRTCClient2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClient2_Vtbl
    where
        Identity: IRTCClient2_Impl,
    {
        unsafe extern "system" fn put_AnswerMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::put_AnswerMode(this, core::mem::transmute_copy(&entype), core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn get_AnswerMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient2_Impl::get_AnswerMode(this, core::mem::transmute_copy(&entype)) {
                Ok(ok__) => {
                    penmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::InvokeTuningWizardEx(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&fallowaudio), core::mem::transmute_copy(&fallowvideo)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient2_Impl::Version(this) {
                Ok(ok__) => {
                    plversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclientname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::SetClientName(this, core::mem::transmute(&bstrclientname)).into()
        }
        unsafe extern "system" fn SetClientCurVer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclientcurver: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::SetClientCurVer(this, core::mem::transmute(&bstrclientcurver)).into()
        }
        unsafe extern "system" fn InitializeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::InitializeEx(this, core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateSessionWithDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsessiondescription: core::mem::MaybeUninit<windows_core::BSTR>, pprofile: *mut core::ffi::c_void, lflags: i32, ppsession2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient2_Impl::CreateSessionWithDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppsession2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessiondescriptionmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::SetSessionDescriptionManager(this, windows_core::from_raw_borrowed(&psessiondescriptionmanager)).into()
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::put_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype), core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient2_Impl::get_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                Ok(ok__) => {
                    pensecuritylevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AllowedPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClient2_Impl::put_AllowedPorts(this, core::mem::transmute_copy(&ltransport), core::mem::transmute_copy(&enlistenmode)).into()
        }
        unsafe extern "system" fn get_AllowedPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClient2_Impl::get_AllowedPorts(this, core::mem::transmute_copy(&ltransport)) {
                Ok(ok__) => {
                    penlistenmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(&self) -> windows_core::Result<IRTCClient>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientEvent_Vtbl
    where
        Identity: IRTCClientEvent_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCClientEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientEvent_Impl::EventType(this) {
                Ok(ok__) => {
                    peneventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Client<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientEvent_Impl::Client(this) {
                Ok(ok__) => {
                    ppclient.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET>, Client: Client::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRTCClientPortManagement_Impl: Sized {
    fn StartListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()>;
    fn StopListenAddressAndPort(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32) -> windows_core::Result<()>;
    fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCClientPortManagement {}
impl IRTCClientPortManagement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientPortManagement_Vtbl
    where
        Identity: IRTCClientPortManagement_Impl,
    {
        unsafe extern "system" fn StartListenAddressAndPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, linternallocalport: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientPortManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPortManagement_Impl::StartListenAddressAndPort(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn StopListenAddressAndPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, linternallocalport: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientPortManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPortManagement_Impl::StopListenAddressAndPort(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport)).into()
        }
        unsafe extern "system" fn GetPortRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientPortManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPortManagement_Impl::GetPortRange(this, core::mem::transmute_copy(&enporttype), core::mem::transmute_copy(&plminvalue), core::mem::transmute_copy(&plmaxvalue)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientPresence_Impl: Sized {
    fn EnablePresence(&self, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Export(&self, varstorage: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Import(&self, varstorage: &windows_core::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnumerateBuddies(&self) -> windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Buddy(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCBuddy>;
    fn AddBuddy(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddy>;
    fn RemoveBuddy(&self, pbuddy: Option<&IRTCBuddy>) -> windows_core::Result<()>;
    fn EnumerateWatchers(&self) -> windows_core::Result<IRTCEnumWatchers>;
    fn Watchers(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Watcher(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher>;
    fn AddWatcher(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IRTCWatcher>;
    fn RemoveWatcher(&self, pwatcher: Option<&IRTCWatcher>) -> windows_core::Result<()>;
    fn SetLocalPresenceInfo(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OfferWatcherMode(&self) -> windows_core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::Result<()>;
    fn PrivacyMode(&self) -> windows_core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientPresence {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientPresence_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientPresence_Vtbl
    where
        Identity: IRTCClientPresence_Impl,
    {
        unsafe extern "system" fn EnablePresence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::EnablePresence(this, core::mem::transmute_copy(&fusestorage), core::mem::transmute(&varstorage)).into()
        }
        unsafe extern "system" fn Export<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varstorage: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::Export(this, core::mem::transmute(&varstorage)).into()
        }
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varstorage: core::mem::MaybeUninit<windows_core::VARIANT>, freplaceall: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::Import(this, core::mem::transmute(&varstorage), core::mem::transmute_copy(&freplaceall)).into()
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::EnumerateBuddies(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buddies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::Buddies(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Buddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::get_Buddy(this, core::mem::transmute(&bstrpresentityuri)) {
                Ok(ok__) => {
                    ppbuddy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddBuddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: *mut core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::AddBuddy(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fpersistent), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppbuddy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBuddy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuddy: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::RemoveBuddy(this, windows_core::from_raw_borrowed(&pbuddy)).into()
        }
        unsafe extern "system" fn EnumerateWatchers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::EnumerateWatchers(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watchers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::Watchers(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Watcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::get_Watcher(this, core::mem::transmute(&bstrpresentityuri)) {
                Ok(ok__) => {
                    ppwatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::AddWatcher(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fblocked), core::mem::transmute_copy(&fpersistent)) {
                Ok(ok__) => {
                    ppwatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwatcher: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::RemoveWatcher(this, windows_core::from_raw_borrowed(&pwatcher)).into()
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::SetLocalPresenceInfo(this, core::mem::transmute_copy(&enstatus), core::mem::transmute(&bstrnotes)).into()
        }
        unsafe extern "system" fn OfferWatcherMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::OfferWatcherMode(this) {
                Ok(ok__) => {
                    penmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfferWatcherMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::SetOfferWatcherMode(this, core::mem::transmute_copy(&enmode)).into()
        }
        unsafe extern "system" fn PrivacyMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence_Impl::PrivacyMode(this) {
                Ok(ok__) => {
                    penmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivacyMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence_Impl::SetPrivacyMode(this, core::mem::transmute_copy(&enmode)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientPresence2_Impl: Sized + IRTCClientPresence_Impl {
    fn EnablePresenceEx(&self, pprofile: Option<&IRTCProfile>, varstorage: &windows_core::VARIANT, lflags: i32) -> windows_core::Result<()>;
    fn DisablePresence(&self) -> windows_core::Result<()>;
    fn AddGroup(&self, bstrgroupname: &windows_core::BSTR, bstrdata: &windows_core::BSTR, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(&self, pgroup: Option<&IRTCBuddyGroup>) -> windows_core::Result<()>;
    fn EnumerateGroups(&self) -> windows_core::Result<IRTCEnumGroups>;
    fn Groups(&self) -> windows_core::Result<IRTCCollection>;
    fn get_Group(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCWatcher2>;
    fn get_WatcherEx(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<IRTCWatcher2>;
    fn put_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR>;
    fn SetPresenceData(&self, bstrnamespace: &windows_core::BSTR, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddBuddyEx(&self, bstrpresentityuri: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrdata: &windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<IRTCBuddy2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientPresence2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientPresence2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientPresence2_Vtbl
    where
        Identity: IRTCClientPresence2_Impl,
    {
        unsafe extern "system" fn EnablePresenceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, varstorage: core::mem::MaybeUninit<windows_core::VARIANT>, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::EnablePresenceEx(this, windows_core::from_raw_borrowed(&pprofile), core::mem::transmute(&varstorage), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DisablePresence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::DisablePresence(this).into()
        }
        unsafe extern "system" fn AddGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, pprofile: *mut core::ffi::c_void, lflags: i32, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::AddGroup(this, core::mem::transmute(&bstrgroupname), core::mem::transmute(&bstrdata), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::RemoveGroup(this, windows_core::from_raw_borrowed(&pgroup)).into()
        }
        unsafe extern "system" fn EnumerateGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::EnumerateGroups(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Groups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::Groups(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::get_Group(this, core::mem::transmute(&bstrgroupname)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWatcherEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: *mut core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::AddWatcherEx(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&enstate), core::mem::transmute_copy(&fpersistent), core::mem::transmute_copy(&enscope), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppwatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_WatcherEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::get_WatcherEx(this, core::mem::transmute_copy(&enmode), core::mem::transmute(&bstrpresentityuri)) {
                Ok(ok__) => {
                    ppwatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_PresenceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::put_PresenceProperty(this, core::mem::transmute_copy(&enproperty), core::mem::transmute(&bstrproperty)).into()
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                Ok(ok__) => {
                    pbstrproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresenceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnamespace: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::SetPresenceData(this, core::mem::transmute(&bstrnamespace), core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn GetPresenceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientPresence2_Impl::GetLocalPresenceInfo(this, core::mem::transmute_copy(&penstatus), core::mem::transmute_copy(&pbstrnotes)).into()
        }
        unsafe extern "system" fn AddBuddyEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientPresence2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientPresence2_Impl::AddBuddyEx(this, core::mem::transmute(&bstrpresentityuri), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrdata), core::mem::transmute_copy(&fpersistent), core::mem::transmute_copy(&ensubscriptiontype), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    ppbuddy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning_Impl: Sized {
    fn CreateProfile(&self, bstrprofilexml: &windows_core::BSTR) -> windows_core::Result<IRTCProfile>;
    fn EnableProfile(&self, pprofile: Option<&IRTCProfile>, lregisterflags: i32) -> windows_core::Result<()>;
    fn DisableProfile(&self, pprofile: Option<&IRTCProfile>) -> windows_core::Result<()>;
    fn EnumerateProfiles(&self) -> windows_core::Result<IRTCEnumProfiles>;
    fn Profiles(&self) -> windows_core::Result<IRTCCollection>;
    fn GetProfile(&self, bstruseraccount: &windows_core::BSTR, bstruserpassword: &windows_core::BSTR, bstruseruri: &windows_core::BSTR, bstrserver: &windows_core::BSTR, ltransport: i32, lcookie: isize) -> windows_core::Result<()>;
    fn SessionCapabilities(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientProvisioning {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientProvisioning_Vtbl
    where
        Identity: IRTCClientProvisioning_Impl,
    {
        unsafe extern "system" fn CreateProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprofilexml: core::mem::MaybeUninit<windows_core::BSTR>, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientProvisioning_Impl::CreateProfile(this, core::mem::transmute(&bstrprofilexml)) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lregisterflags: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientProvisioning_Impl::EnableProfile(this, windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lregisterflags)).into()
        }
        unsafe extern "system" fn DisableProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientProvisioning_Impl::DisableProfile(this, windows_core::from_raw_borrowed(&pprofile)).into()
        }
        unsafe extern "system" fn EnumerateProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientProvisioning_Impl::EnumerateProfiles(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientProvisioning_Impl::Profiles(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseraccount: core::mem::MaybeUninit<windows_core::BSTR>, bstruserpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstruseruri: core::mem::MaybeUninit<windows_core::BSTR>, bstrserver: core::mem::MaybeUninit<windows_core::BSTR>, ltransport: i32, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientProvisioning_Impl::GetProfile(this, core::mem::transmute(&bstruseraccount), core::mem::transmute(&bstruserpassword), core::mem::transmute(&bstruseruri), core::mem::transmute(&bstrserver), core::mem::transmute_copy(&ltransport), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsupportedsessions: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCClientProvisioning_Impl::SessionCapabilities(this) {
                Ok(ok__) => {
                    plsupportedsessions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCClientProvisioning2_Impl: Sized + IRTCClientProvisioning_Impl {
    fn EnableProfileEx(&self, pprofile: Option<&IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCClientProvisioning2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCClientProvisioning2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCClientProvisioning2_Vtbl
    where
        Identity: IRTCClientProvisioning2_Impl,
    {
        unsafe extern "system" fn EnableProfileEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> windows_core::HRESULT
        where
            Identity: IRTCClientProvisioning2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCClientProvisioning2_Impl::EnableProfileEx(this, windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lregisterflags), core::mem::transmute_copy(&lroamingflags)).into()
        }
        Self { base__: IRTCClientProvisioning_Vtbl::new::<Identity, OFFSET>(), EnableProfileEx: EnableProfileEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCClientProvisioning2 as windows_core::Interface>::IID || iid == &<IRTCClientProvisioning as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCCollection_Vtbl
    where
        Identity: IRTCCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCCollection_Impl::Count(this) {
                Ok(ok__) => {
                    lcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IRTCCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppnewenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCDispatchEventNotification_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCDispatchEventNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCDispatchEventNotification_Vtbl
    where
        Identity: IRTCDispatchEventNotification_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCDispatchEventNotification as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRTCEnumBuddies_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCBuddy>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumBuddies>;
}
impl windows_core::RuntimeName for IRTCEnumBuddies {}
impl IRTCEnumBuddies_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumBuddies_Vtbl
    where
        Identity: IRTCEnumBuddies_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumBuddies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumBuddies_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumBuddies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumBuddies_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumBuddies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumBuddies_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumBuddies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumBuddies_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumGroups_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumGroups>;
}
impl windows_core::RuntimeName for IRTCEnumGroups {}
impl IRTCEnumGroups_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumGroups_Vtbl
    where
        Identity: IRTCEnumGroups_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumGroups_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumGroups_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumGroups_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumGroups_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumParticipants_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCParticipant>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumParticipants>;
}
impl windows_core::RuntimeName for IRTCEnumParticipants {}
impl IRTCEnumParticipants_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumParticipants_Vtbl
    where
        Identity: IRTCEnumParticipants_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumParticipants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumParticipants_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumParticipants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumParticipants_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumParticipants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumParticipants_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumParticipants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumParticipants_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumPresenceDevices_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumPresenceDevices>;
}
impl windows_core::RuntimeName for IRTCEnumPresenceDevices {}
impl IRTCEnumPresenceDevices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumPresenceDevices_Vtbl
    where
        Identity: IRTCEnumPresenceDevices_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumPresenceDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumPresenceDevices_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumPresenceDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumPresenceDevices_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumPresenceDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumPresenceDevices_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumPresenceDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumPresenceDevices_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumProfiles_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCProfile>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumProfiles>;
}
impl windows_core::RuntimeName for IRTCEnumProfiles {}
impl IRTCEnumProfiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumProfiles_Vtbl
    where
        Identity: IRTCEnumProfiles_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumProfiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumProfiles_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumProfiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumProfiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumProfiles_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumUserSearchResults_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumUserSearchResults>;
}
impl windows_core::RuntimeName for IRTCEnumUserSearchResults {}
impl IRTCEnumUserSearchResults_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumUserSearchResults_Vtbl
    where
        Identity: IRTCEnumUserSearchResults_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumUserSearchResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumUserSearchResults_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumUserSearchResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumUserSearchResults_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumUserSearchResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumUserSearchResults_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumUserSearchResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumUserSearchResults_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCEnumWatchers_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut Option<IRTCWatcher>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IRTCEnumWatchers>;
}
impl windows_core::RuntimeName for IRTCEnumWatchers {}
impl IRTCEnumWatchers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEnumWatchers_Vtbl
    where
        Identity: IRTCEnumWatchers_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppelements: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumWatchers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumWatchers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppelements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumWatchers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumWatchers_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IRTCEnumWatchers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEnumWatchers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEnumWatchers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCEnumWatchers_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCEventNotification_Impl: Sized {
    fn Event(&self, rtcevent: RTC_EVENT, pevent: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCEventNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCEventNotification_Vtbl
    where
        Identity: IRTCEventNotification_Impl,
    {
        unsafe extern "system" fn Event<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtcevent: RTC_EVENT, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCEventNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCEventNotification_Impl::Event(this, core::mem::transmute_copy(&rtcevent), windows_core::from_raw_borrowed(&pevent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCEventNotification as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCInfoEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn Info(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InfoHeader(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCInfoEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCInfoEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCInfoEvent_Vtbl
    where
        Identity: IRTCInfoEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCInfoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCInfoEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCInfoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCInfoEvent_Impl::Participant(this) {
                Ok(ok__) => {
                    ppparticipant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinfo: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCInfoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCInfoEvent_Impl::Info(this) {
                Ok(ok__) => {
                    pbstrinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinfoheader: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCInfoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCInfoEvent_Impl::InfoHeader(this) {
                Ok(ok__) => {
                    pbstrinfoheader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCIntensityEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Level(&self) -> windows_core::Result<i32>;
    fn Min(&self) -> windows_core::Result<i32>;
    fn Max(&self) -> windows_core::Result<i32>;
    fn Direction(&self) -> windows_core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCIntensityEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCIntensityEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCIntensityEvent_Vtbl
    where
        Identity: IRTCIntensityEvent_Impl,
    {
        unsafe extern "system" fn Level<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCIntensityEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCIntensityEvent_Impl::Level(this) {
                Ok(ok__) => {
                    pllevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCIntensityEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCIntensityEvent_Impl::Min(this) {
                Ok(ok__) => {
                    plmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmax: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCIntensityEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCIntensityEvent_Impl::Max(this) {
                Ok(ok__) => {
                    plmax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> windows_core::HRESULT
        where
            Identity: IRTCIntensityEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCIntensityEvent_Impl::Direction(this) {
                Ok(ok__) => {
                    pendirection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<i32>;
    fn EventType(&self) -> windows_core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(&self) -> windows_core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCMediaEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCMediaEvent_Vtbl
    where
        Identity: IRTCMediaEvent_Impl,
    {
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaEvent_Impl::MediaType(this) {
                Ok(ok__) => {
                    pmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaEvent_Impl::EventType(this) {
                Ok(ok__) => {
                    peneventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventReason<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> windows_core::HRESULT
        where
            Identity: IRTCMediaEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaEvent_Impl::EventReason(this) {
                Ok(ok__) => {
                    peneventreason.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMediaRequestEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ProposedMedia(&self) -> windows_core::Result<i32>;
    fn CurrentMedia(&self) -> windows_core::Result<i32>;
    fn Accept(&self, lmediatypes: i32) -> windows_core::Result<()>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCMediaRequestEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMediaRequestEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCMediaRequestEvent_Vtbl
    where
        Identity: IRTCMediaRequestEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaRequestEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProposedMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaRequestEvent_Impl::ProposedMedia(this) {
                Ok(ok__) => {
                    plmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaRequestEvent_Impl::CurrentMedia(this) {
                Ok(ok__) => {
                    plmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatypes: i32) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCMediaRequestEvent_Impl::Accept(this, core::mem::transmute_copy(&lmediatypes)).into()
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaRequestEvent_Impl::get_RemotePreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                Ok(ok__) => {
                    pensecuritylevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCMediaRequestEvent_Impl::Reject(this).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCMediaRequestEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMediaRequestEvent_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCMessagingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn EventType(&self) -> windows_core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MessageHeader(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserStatus(&self) -> windows_core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCMessagingEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCMessagingEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCMessagingEvent_Vtbl
    where
        Identity: IRTCMessagingEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::Participant(this) {
                Ok(ok__) => {
                    ppparticipant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::EventType(this) {
                Ok(ok__) => {
                    peneventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmessage: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::Message(this) {
                Ok(ok__) => {
                    pbstrmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmessageheader: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::MessageHeader(this) {
                Ok(ok__) => {
                    pbstrmessageheader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> windows_core::HRESULT
        where
            Identity: IRTCMessagingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCMessagingEvent_Impl::UserStatus(this) {
                Ok(ok__) => {
                    penuserstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCParticipant_Impl: Sized {
    fn UserURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Removable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(&self) -> windows_core::Result<IRTCSession>;
}
impl windows_core::RuntimeName for IRTCParticipant {}
impl IRTCParticipant_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCParticipant_Vtbl
    where
        Identity: IRTCParticipant_Impl,
    {
        unsafe extern "system" fn UserURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCParticipant_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipant_Impl::UserURI(this) {
                Ok(ok__) => {
                    pbstruseruri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCParticipant_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipant_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfremovable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCParticipant_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipant_Impl::Removable(this) {
                Ok(ok__) => {
                    pfremovable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCParticipant_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipant_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCParticipant_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipant_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCParticipantStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn State(&self) -> windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCParticipantStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCParticipantStateChangeEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCParticipantStateChangeEvent_Vtbl
    where
        Identity: IRTCParticipantStateChangeEvent_Impl,
    {
        unsafe extern "system" fn Participant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCParticipantStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipantStateChangeEvent_Impl::Participant(this) {
                Ok(ok__) => {
                    ppparticipant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCParticipantStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipantStateChangeEvent_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCParticipantStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCParticipantStateChangeEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCPortManager_Impl: Sized {
    fn GetMapping(&self, bstrremoteaddress: &windows_core::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut windows_core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut windows_core::BSTR, plexternallocalport: *mut i32) -> windows_core::Result<()>;
    fn UpdateRemoteAddress(&self, bstrremoteaddress: &windows_core::BSTR, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocalport: i32) -> windows_core::Result<()>;
    fn ReleaseMapping(&self, bstrinternallocaladdress: &windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &windows_core::BSTR, lexternallocaladdress: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCPortManager {}
impl IRTCPortManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPortManager_Vtbl
    where
        Identity: IRTCPortManager_Impl,
    {
        unsafe extern "system" fn GetMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremoteaddress: core::mem::MaybeUninit<windows_core::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut core::mem::MaybeUninit<windows_core::BSTR>, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut core::mem::MaybeUninit<windows_core::BSTR>, plexternallocalport: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPortManager_Impl::GetMapping(this, core::mem::transmute(&bstrremoteaddress), core::mem::transmute_copy(&enporttype), core::mem::transmute_copy(&pbstrinternallocaladdress), core::mem::transmute_copy(&plinternallocalport), core::mem::transmute_copy(&pbstrexternallocaladdress), core::mem::transmute_copy(&plexternallocalport)).into()
        }
        unsafe extern "system" fn UpdateRemoteAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremoteaddress: core::mem::MaybeUninit<windows_core::BSTR>, bstrinternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, lexternallocalport: i32) -> windows_core::HRESULT
        where
            Identity: IRTCPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPortManager_Impl::UpdateRemoteAddress(this, core::mem::transmute(&bstrremoteaddress), core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport), core::mem::transmute(&bstrexternallocaladdress), core::mem::transmute_copy(&lexternallocalport)).into()
        }
        unsafe extern "system" fn ReleaseMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: core::mem::MaybeUninit<windows_core::BSTR>, lexternallocaladdress: i32) -> windows_core::HRESULT
        where
            Identity: IRTCPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPortManager_Impl::ReleaseMapping(this, core::mem::transmute(&bstrinternallocaladdress), core::mem::transmute_copy(&linternallocalport), core::mem::transmute(&bstrexternallocaladdress), core::mem::transmute_copy(&lexternallocaladdress)).into()
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
pub trait IRTCPresenceContact_Impl: Sized {
    fn PresentityURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPresentityURI(&self, bstrpresentityuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Persistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPersistent(&self, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCPresenceContact {}
impl IRTCPresenceContact_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPresenceContact_Vtbl
    where
        Identity: IRTCPresenceContact_Impl,
    {
        unsafe extern "system" fn PresentityURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpresentityuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceContact_Impl::PresentityURI(this) {
                Ok(ok__) => {
                    pbstrpresentityuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentityURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpresentityuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceContact_Impl::SetPresentityURI(this, core::mem::transmute(&bstrpresentityuri)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceContact_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceContact_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceContact_Impl::Data(this) {
                Ok(ok__) => {
                    pbstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceContact_Impl::SetData(this, core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Persistent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpersistent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceContact_Impl::Persistent(this) {
                Ok(ok__) => {
                    pfpersistent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceContact_Impl::SetPersistent(this, core::mem::transmute_copy(&fpersistent)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceDataEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCPresenceDataEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceDataEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPresenceDataEvent_Vtbl
    where
        Identity: IRTCPresenceDataEvent_Impl,
    {
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDataEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceDataEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDataEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceDataEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDataEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceDataEvent_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
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
pub trait IRTCPresenceDevice_Impl: Sized {
    fn Status(&self) -> windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> windows_core::Result<windows_core::BSTR>;
    fn GetPresenceData(&self, pbstrnamespace: *mut windows_core::BSTR, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCPresenceDevice {}
impl IRTCPresenceDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPresenceDevice_Vtbl
    where
        Identity: IRTCPresenceDevice_Impl,
    {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceDevice_Impl::Status(this) {
                Ok(ok__) => {
                    penstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnotes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceDevice_Impl::Notes(this) {
                Ok(ok__) => {
                    pbstrnotes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceDevice_Impl::get_PresenceProperty(this, core::mem::transmute_copy(&enproperty)) {
                Ok(ok__) => {
                    pbstrproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresenceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceDevice_Impl::GetPresenceData(this, core::mem::transmute_copy(&pbstrnamespace), core::mem::transmute_copy(&pbstrdata)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresencePropertyEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PresenceProperty(&self) -> windows_core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCPresencePropertyEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresencePropertyEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPresencePropertyEvent_Vtbl
    where
        Identity: IRTCPresencePropertyEvent_Impl,
    {
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCPresencePropertyEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresencePropertyEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresencePropertyEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresencePropertyEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresenceProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> windows_core::HRESULT
        where
            Identity: IRTCPresencePropertyEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresencePropertyEvent_Impl::PresenceProperty(this) {
                Ok(ok__) => {
                    penpresprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresencePropertyEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresencePropertyEvent_Impl::Value(this) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCPresenceStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCPresenceStatusEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCPresenceStatusEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCPresenceStatusEvent_Vtbl
    where
        Identity: IRTCPresenceStatusEvent_Impl,
    {
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceStatusEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCPresenceStatusEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCPresenceStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCPresenceStatusEvent_Impl::GetLocalPresenceInfo(this, core::mem::transmute_copy(&penstatus), core::mem::transmute_copy(&pbstrnotes)).into()
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
pub trait IRTCProfile_Impl: Sized {
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
impl windows_core::RuntimeName for IRTCProfile {}
impl IRTCProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCProfile_Vtbl
    where
        Identity: IRTCProfile_Impl,
    {
        unsafe extern "system" fn Key<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrkey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::Key(this) {
                Ok(ok__) => {
                    pbstrkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::XML(this) {
                Ok(ok__) => {
                    pbstrxml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ProviderURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::get_ProviderURI(this, core::mem::transmute_copy(&enuri)) {
                Ok(ok__) => {
                    pbstruri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ProviderData(this) {
                Ok(ok__) => {
                    pbstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientBanner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbanner: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientBanner(this) {
                Ok(ok__) => {
                    pfbanner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientMinVer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrminver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientMinVer(this) {
                Ok(ok__) => {
                    pbstrminver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCurVer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcurver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientCurVer(this) {
                Ok(ok__) => {
                    pbstrcurver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientUpdateURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrupdateuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientUpdateURI(this) {
                Ok(ok__) => {
                    pbstrupdateuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::ClientData(this) {
                Ok(ok__) => {
                    pbstrdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::UserURI(this) {
                Ok(ok__) => {
                    pbstruseruri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::UserName(this) {
                Ok(ok__) => {
                    pbstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseraccount: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::UserAccount(this) {
                Ok(ok__) => {
                    pbstruseraccount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstruseruri: core::mem::MaybeUninit<windows_core::BSTR>, bstruseraccount: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCProfile_Impl::SetCredentials(this, core::mem::transmute(&bstruseruri), core::mem::transmute(&bstruseraccount), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn SessionCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsupportedsessions: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::SessionCapabilities(this) {
                Ok(ok__) => {
                    plsupportedsessions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCProfile2_Impl: Sized + IRTCProfile_Impl {
    fn Realm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRealm(&self, bstrrealm: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowedAuth(&self) -> windows_core::Result<i32>;
    fn SetAllowedAuth(&self, lallowedauth: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCProfile2 {}
impl IRTCProfile2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCProfile2_Vtbl
    where
        Identity: IRTCProfile2_Impl,
    {
        unsafe extern "system" fn Realm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrealm: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile2_Impl::Realm(this) {
                Ok(ok__) => {
                    pbstrrealm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrealm: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCProfile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCProfile2_Impl::SetRealm(this, core::mem::transmute(&bstrrealm)).into()
        }
        unsafe extern "system" fn AllowedAuth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plallowedauth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCProfile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfile2_Impl::AllowedAuth(this) {
                Ok(ok__) => {
                    plallowedauth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedAuth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lallowedauth: i32) -> windows_core::HRESULT
        where
            Identity: IRTCProfile2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCProfile2_Impl::SetAllowedAuth(this, core::mem::transmute_copy(&lallowedauth)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCProfileEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCProfileEvent_Vtbl
    where
        Identity: IRTCProfileEvent_Impl,
    {
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCProfileEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfileEvent_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT
        where
            Identity: IRTCProfileEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfileEvent_Impl::Cookie(this) {
                Ok(ok__) => {
                    plcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCProfileEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfileEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCProfileEvent2_Impl: Sized + IRTCProfileEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCProfileEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCProfileEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCProfileEvent2_Vtbl
    where
        Identity: IRTCProfileEvent2_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCProfileEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCProfileEvent2_Impl::EventType(this) {
                Ok(ok__) => {
                    peventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IRTCProfileEvent_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCProfileEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCProfileEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCReInviteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn Accept(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCReInviteEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCReInviteEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCReInviteEvent_Vtbl
    where
        Identity: IRTCReInviteEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCReInviteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCReInviteEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession2.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsessiondescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCReInviteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCReInviteEvent_Impl::Accept(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn Reject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCReInviteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCReInviteEvent_Impl::Reject(this).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCReInviteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCReInviteEvent_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsessiondescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCReInviteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCReInviteEvent_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRegistrationStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn State(&self) -> windows_core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCRegistrationStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCRegistrationStateChangeEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCRegistrationStateChangeEvent_Vtbl
    where
        Identity: IRTCRegistrationStateChangeEvent_Impl,
    {
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCRegistrationStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRegistrationStateChangeEvent_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCRegistrationStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRegistrationStateChangeEvent_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCRegistrationStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRegistrationStateChangeEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCRegistrationStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRegistrationStateChangeEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCRoamingEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCRoamingEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCRoamingEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCRoamingEvent_Vtbl
    where
        Identity: IRTCRoamingEvent_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCRoamingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRoamingEvent_Impl::EventType(this) {
                Ok(ok__) => {
                    peventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCRoamingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRoamingEvent_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCRoamingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRoamingEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCRoamingEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCRoamingEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSession_Impl: Sized {
    fn Client(&self) -> windows_core::Result<IRTCClient>;
    fn State(&self) -> windows_core::Result<RTC_SESSION_STATE>;
    fn Type(&self) -> windows_core::Result<RTC_SESSION_TYPE>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile>;
    fn Participants(&self) -> windows_core::Result<IRTCCollection>;
    fn Answer(&self) -> windows_core::Result<()>;
    fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> windows_core::Result<()>;
    fn Redirect(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &windows_core::BSTR, pprofile: Option<&IRTCProfile>, lflags: i32) -> windows_core::Result<()>;
    fn AddParticipant(&self, bstraddress: &windows_core::BSTR, bstrname: &windows_core::BSTR) -> windows_core::Result<IRTCParticipant>;
    fn RemoveParticipant(&self, pparticipant: Option<&IRTCParticipant>) -> windows_core::Result<()>;
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
impl windows_core::RuntimeName for IRTCSession {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSession_Vtbl
    where
        Identity: IRTCSession_Impl,
    {
        unsafe extern "system" fn Client<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclient: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::Client(this) {
                Ok(ok__) => {
                    ppclient.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::Type(this) {
                Ok(ok__) => {
                    pentype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Participants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::Participants(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::Answer(this).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::Terminate(this, core::mem::transmute_copy(&enreason)).into()
        }
        unsafe extern "system" fn Redirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: core::mem::MaybeUninit<windows_core::BSTR>, pprofile: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::Redirect(this, core::mem::transmute_copy(&entype), core::mem::transmute(&bstrlocalphoneuri), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddParticipant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraddress: core::mem::MaybeUninit<windows_core::BSTR>, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::AddParticipant(this, core::mem::transmute(&bstraddress), core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppparticipant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveParticipant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparticipant: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::RemoveParticipant(this, windows_core::from_raw_borrowed(&pparticipant)).into()
        }
        unsafe extern "system" fn EnumerateParticipants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::EnumerateParticipants(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanAddParticipants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcanadd: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::CanAddParticipants(this) {
                Ok(ok__) => {
                    pfcanadd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruseruri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::RedirectedUserURI(this) {
                Ok(ok__) => {
                    pbstruseruri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectedUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession_Impl::RedirectedUserName(this) {
                Ok(ok__) => {
                    pbstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRedirectedUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::NextRedirectedUser(this).into()
        }
        unsafe extern "system" fn SendMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessageheader: core::mem::MaybeUninit<windows_core::BSTR>, bstrmessage: core::mem::MaybeUninit<windows_core::BSTR>, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::SendMessage(this, core::mem::transmute(&bstrmessageheader), core::mem::transmute(&bstrmessage), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn SendMessageStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::SendMessageStatus(this, core::mem::transmute_copy(&enuserstatus), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn AddStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::AddStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::RemoveStream(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn put_EncryptionKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmediatype: i32, encryptionkey: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession_Impl::put_EncryptionKey(this, core::mem::transmute_copy(&lmediatype), core::mem::transmute(&encryptionkey)).into()
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
pub trait IRTCSession2_Impl: Sized + IRTCSession_Impl {
    fn SendInfo(&self, bstrinfoheader: &windows_core::BSTR, bstrinfo: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()>;
    fn put_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::Result<()>;
    fn get_PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReInviteWithSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, lcookie: isize) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSession2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSession2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSession2_Vtbl
    where
        Identity: IRTCSession2_Impl,
    {
        unsafe extern "system" fn SendInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinfoheader: core::mem::MaybeUninit<windows_core::BSTR>, bstrinfo: core::mem::MaybeUninit<windows_core::BSTR>, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession2_Impl::SendInfo(this, core::mem::transmute(&bstrinfoheader), core::mem::transmute(&bstrinfo), core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession2_Impl::put_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype), core::mem::transmute_copy(&ensecuritylevel)).into()
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession2_Impl::get_PreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                Ok(ok__) => {
                    pensecuritylevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSession2_Impl::IsSecurityEnabled(this, core::mem::transmute_copy(&ensecuritytype)) {
                Ok(ok__) => {
                    pfsecurityenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsessiondescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession2_Impl::AnswerWithSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription)).into()
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsessiondescription: core::mem::MaybeUninit<windows_core::BSTR>, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSession2_Impl::ReInviteWithSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), core::mem::transmute_copy(&lcookie)).into()
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
pub trait IRTCSessionCallControl_Impl: Sized {
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
impl windows_core::RuntimeName for IRTCSessionCallControl {}
impl IRTCSessionCallControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionCallControl_Vtbl
    where
        Identity: IRTCSessionCallControl_Impl,
    {
        unsafe extern "system" fn Hold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::Hold(this, core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn UnHold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::UnHold(this, core::mem::transmute_copy(&lcookie)).into()
        }
        unsafe extern "system" fn Forward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrforwardtouri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::Forward(this, core::mem::transmute(&bstrforwardtouri)).into()
        }
        unsafe extern "system" fn Refer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrefertouri: core::mem::MaybeUninit<windows_core::BSTR>, bstrrefercookie: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::Refer(this, core::mem::transmute(&bstrrefertouri), core::mem::transmute(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn SetReferredByURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrreferredbyuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::SetReferredByURI(this, core::mem::transmute(&bstrreferredbyuri)).into()
        }
        unsafe extern "system" fn ReferredByURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferredbyuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionCallControl_Impl::ReferredByURI(this) {
                Ok(ok__) => {
                    pbstrreferredbyuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrefercookie: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionCallControl_Impl::SetReferCookie(this, core::mem::transmute(&bstrrefercookie)).into()
        }
        unsafe extern "system" fn ReferCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrefercookie: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionCallControl_Impl::ReferCookie(this) {
                Ok(ok__) => {
                    pbstrrefercookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisreferred: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCSessionCallControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionCallControl_Impl::IsReferred(this) {
                Ok(ok__) => {
                    pfisreferred.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCSessionDescriptionManager_Impl: Sized {
    fn EvaluateSessionDescription(&self, bstrcontenttype: &windows_core::BSTR, bstrsessiondescription: &windows_core::BSTR, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCSessionDescriptionManager {}
impl IRTCSessionDescriptionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionDescriptionManager_Vtbl
    where
        Identity: IRTCSessionDescriptionManager_Impl,
    {
        unsafe extern "system" fn EvaluateSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontenttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsessiondescription: core::mem::MaybeUninit<windows_core::BSTR>, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCSessionDescriptionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionDescriptionManager_Impl::EvaluateSessionDescription(this, core::mem::transmute(&bstrcontenttype), core::mem::transmute(&bstrsessiondescription), core::mem::transmute_copy(&pfapplicationsession)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EvaluateSessionDescription: EvaluateSessionDescription::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionDescriptionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionOperationCompleteEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEvent_Vtbl
    where
        Identity: IRTCSessionOperationCompleteEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionOperationCompleteEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionOperationCompleteEvent_Impl::Cookie(this) {
                Ok(ok__) => {
                    plcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionOperationCompleteEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionOperationCompleteEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionOperationCompleteEvent2_Impl: Sized + IRTCSessionOperationCompleteEvent_Impl {
    fn Participant(&self) -> windows_core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionOperationCompleteEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionOperationCompleteEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionOperationCompleteEvent2_Vtbl
    where
        Identity: IRTCSessionOperationCompleteEvent2_Impl,
    {
        unsafe extern "system" fn Participant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparticipant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionOperationCompleteEvent2_Impl::Participant(this) {
                Ok(ok__) => {
                    ppparticipant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsessiondescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionOperationCompleteEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionOperationCompleteEvent2_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
pub trait IRTCSessionPortManagement_Impl: Sized {
    fn SetPortManager(&self, pportmanager: Option<&IRTCPortManager>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCSessionPortManagement {}
impl IRTCSessionPortManagement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionPortManagement_Vtbl
    where
        Identity: IRTCSessionPortManagement_Impl,
    {
        unsafe extern "system" fn SetPortManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionPortManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionPortManagement_Impl::SetPortManager(this, windows_core::from_raw_borrowed(&pportmanager)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetPortManager: SetPortManager::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCSessionPortManagement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferStatusEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ReferStatus(&self) -> windows_core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionReferStatusEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferStatusEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionReferStatusEvent_Vtbl
    where
        Identity: IRTCSessionReferStatusEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferStatusEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferStatusEvent_Impl::ReferStatus(this) {
                Ok(ok__) => {
                    penreferstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferStatusEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferStatusEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferStatusEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionReferredEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession2>;
    fn ReferredByURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReferToURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ReferCookie(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Accept(&self) -> windows_core::Result<()>;
    fn Reject(&self) -> windows_core::Result<()>;
    fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionReferredEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionReferredEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionReferredEvent_Vtbl
    where
        Identity: IRTCSessionReferredEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferredEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferredByURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferredbyuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferredEvent_Impl::ReferredByURI(this) {
                Ok(ok__) => {
                    pbstrreferredbyuri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferToURI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrreferouri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferredEvent_Impl::ReferToURI(this) {
                Ok(ok__) => {
                    pbstrreferouri.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrefercookie: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionReferredEvent_Impl::ReferCookie(this) {
                Ok(ok__) => {
                    pbstrrefercookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionReferredEvent_Impl::Accept(this).into()
        }
        unsafe extern "system" fn Reject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionReferredEvent_Impl::Reject(this).into()
        }
        unsafe extern "system" fn SetReferredSessionState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstate: RTC_SESSION_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCSessionReferredEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionReferredEvent_Impl::SetReferredSessionState(this, core::mem::transmute_copy(&enstate)).into()
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IRTCSession>;
    fn State(&self) -> windows_core::Result<RTC_SESSION_STATE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionStateChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionStateChangeEvent_Vtbl
    where
        Identity: IRTCSessionStateChangeEvent_Impl,
    {
        unsafe extern "system" fn Session<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent_Impl::Session(this) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatustext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent_Impl::StatusText(this) {
                Ok(ok__) => {
                    pbstrstatustext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCSessionStateChangeEvent2_Impl: Sized + IRTCSessionStateChangeEvent_Impl {
    fn MediaTypes(&self) -> windows_core::Result<i32>;
    fn get_RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut windows_core::BSTR, pbstrsessiondescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCSessionStateChangeEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCSessionStateChangeEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCSessionStateChangeEvent2_Vtbl
    where
        Identity: IRTCSessionStateChangeEvent2_Impl,
    {
        unsafe extern "system" fn MediaTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatypes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent2_Impl::MediaTypes(this) {
                Ok(ok__) => {
                    pmediatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent2_Impl::get_RemotePreferredSecurityLevel(this, core::mem::transmute_copy(&ensecuritytype)) {
                Ok(ok__) => {
                    pensecuritylevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisforked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCSessionStateChangeEvent2_Impl::IsForked(this) {
                Ok(ok__) => {
                    pfisforked.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrsessiondescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCSessionStateChangeEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCSessionStateChangeEvent2_Impl::GetRemoteSessionDescription(this, core::mem::transmute_copy(&pbstrcontenttype), core::mem::transmute_copy(&pbstrsessiondescription)).into()
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
pub trait IRTCUserSearch_Impl: Sized {
    fn CreateQuery(&self) -> windows_core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(&self, pquery: Option<&IRTCUserSearchQuery>, pprofile: Option<&IRTCProfile>, lcookie: isize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCUserSearch {}
impl IRTCUserSearch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCUserSearch_Vtbl
    where
        Identity: IRTCUserSearch_Impl,
    {
        unsafe extern "system" fn CreateQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearch_Impl::CreateQuery(this) {
                Ok(ok__) => {
                    ppquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteSearch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void, lcookie: isize) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCUserSearch_Impl::ExecuteSearch(this, windows_core::from_raw_borrowed(&pquery), windows_core::from_raw_borrowed(&pprofile), core::mem::transmute_copy(&lcookie)).into()
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
pub trait IRTCUserSearchQuery_Impl: Sized {
    fn put_SearchTerm(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SearchTerm(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SearchTerms(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::Result<()>;
    fn get_SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> windows_core::Result<i32>;
    fn SetSearchDomain(&self, bstrdomain: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SearchDomain(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IRTCUserSearchQuery {}
impl IRTCUserSearchQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCUserSearchQuery_Vtbl
    where
        Identity: IRTCUserSearchQuery_Impl,
    {
        unsafe extern "system" fn put_SearchTerm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCUserSearchQuery_Impl::put_SearchTerm(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn get_SearchTerm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchQuery_Impl::get_SearchTerm(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchTerms<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnames: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchQuery_Impl::SearchTerms(this) {
                Ok(ok__) => {
                    pbstrnames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_SearchPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCUserSearchQuery_Impl::put_SearchPreference(this, core::mem::transmute_copy(&enpreference), core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn get_SearchPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchQuery_Impl::get_SearchPreference(this, core::mem::transmute_copy(&enpreference)) {
                Ok(ok__) => {
                    plvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomain: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCUserSearchQuery_Impl::SetSearchDomain(this, core::mem::transmute(&bstrdomain)).into()
        }
        unsafe extern "system" fn SearchDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchQuery_Impl::SearchDomain(this) {
                Ok(ok__) => {
                    pbstrdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCUserSearchResult_Impl: Sized {
    fn get_Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IRTCUserSearchResult {}
impl IRTCUserSearchResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCUserSearchResult_Vtbl
    where
        Identity: IRTCUserSearchResult_Impl,
    {
        unsafe extern "system" fn get_Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResult_Impl::get_Value(this, core::mem::transmute_copy(&encolumn)) {
                Ok(ok__) => {
                    pbstrvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_Value: get_Value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCUserSearchResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCUserSearchResultsEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn EnumerateResults(&self) -> windows_core::Result<IRTCEnumUserSearchResults>;
    fn Results(&self) -> windows_core::Result<IRTCCollection>;
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn Query(&self) -> windows_core::Result<IRTCUserSearchQuery>;
    fn Cookie(&self) -> windows_core::Result<isize>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
    fn MoreAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCUserSearchResultsEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCUserSearchResultsEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCUserSearchResultsEvent_Vtbl
    where
        Identity: IRTCUserSearchResultsEvent_Impl,
    {
        unsafe extern "system" fn EnumerateResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::EnumerateResults(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::Results(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::Query(this) {
                Ok(ok__) => {
                    ppquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcookie: *mut isize) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::Cookie(this) {
                Ok(ok__) => {
                    plcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmoreavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IRTCUserSearchResultsEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCUserSearchResultsEvent_Impl::MoreAvailable(this) {
                Ok(ok__) => {
                    pfmoreavailable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IRTCWatcher_Impl: Sized + IRTCPresenceContact_Impl {
    fn State(&self) -> windows_core::Result<RTC_WATCHER_STATE>;
    fn SetState(&self, enstate: RTC_WATCHER_STATE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRTCWatcher {}
impl IRTCWatcher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCWatcher_Vtbl
    where
        Identity: IRTCWatcher_Impl,
    {
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCWatcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcher_Impl::State(this) {
                Ok(ok__) => {
                    penstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> windows_core::HRESULT
        where
            Identity: IRTCWatcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRTCWatcher_Impl::SetState(this, core::mem::transmute_copy(&enstate)).into()
        }
        Self { base__: IRTCPresenceContact_Vtbl::new::<Identity, OFFSET>(), State: State::<Identity, OFFSET>, SetState: SetState::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcher as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID
    }
}
pub trait IRTCWatcher2_Impl: Sized + IRTCWatcher_Impl {
    fn Profile(&self) -> windows_core::Result<IRTCProfile2>;
    fn Scope(&self) -> windows_core::Result<RTC_ACE_SCOPE>;
}
impl windows_core::RuntimeName for IRTCWatcher2 {}
impl IRTCWatcher2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCWatcher2_Vtbl
    where
        Identity: IRTCWatcher2_Impl,
    {
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCWatcher2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcher2_Impl::Profile(this) {
                Ok(ok__) => {
                    ppprofile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> windows_core::HRESULT
        where
            Identity: IRTCWatcher2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcher2_Impl::Scope(this) {
                Ok(ok__) => {
                    penscope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IRTCWatcher_Vtbl::new::<Identity, OFFSET>(), Profile: Profile::<Identity, OFFSET>, Scope: Scope::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcher2 as windows_core::Interface>::IID || iid == &<IRTCPresenceContact as windows_core::Interface>::IID || iid == &<IRTCWatcher as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Watcher(&self) -> windows_core::Result<IRTCWatcher>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCWatcherEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCWatcherEvent_Vtbl
    where
        Identity: IRTCWatcherEvent_Impl,
    {
        unsafe extern "system" fn Watcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRTCWatcherEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcherEvent_Impl::Watcher(this) {
                Ok(ok__) => {
                    ppwatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Watcher: Watcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcherEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCWatcherEvent2_Impl: Sized + IRTCWatcherEvent_Impl {
    fn EventType(&self) -> windows_core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRTCWatcherEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IRTCWatcherEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRTCWatcherEvent2_Vtbl
    where
        Identity: IRTCWatcherEvent2_Impl,
    {
        unsafe extern "system" fn EventType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> windows_core::HRESULT
        where
            Identity: IRTCWatcherEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcherEvent2_Impl::EventType(this) {
                Ok(ok__) => {
                    peventtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatuscode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IRTCWatcherEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRTCWatcherEvent2_Impl::StatusCode(this) {
                Ok(ok__) => {
                    plstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IRTCWatcherEvent_Vtbl::new::<Identity, OFFSET>(), EventType: EventType::<Identity, OFFSET>, StatusCode: StatusCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRTCWatcherEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IRTCWatcherEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternal_Impl: Sized {
    fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()>;
    fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::RuntimeName for ITransportSettingsInternal {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ITransportSettingsInternal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransportSettingsInternal_Vtbl
    where
        Identity: ITransportSettingsInternal_Impl,
    {
        unsafe extern "system" fn ApplySetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> windows_core::HRESULT
        where
            Identity: ITransportSettingsInternal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransportSettingsInternal_Impl::ApplySetting(this, core::mem::transmute_copy(&setting)).into()
        }
        unsafe extern "system" fn QuerySetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> windows_core::HRESULT
        where
            Identity: ITransportSettingsInternal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransportSettingsInternal_Impl::QuerySetting(this, core::mem::transmute_copy(&setting)).into()
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
