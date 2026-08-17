#[cfg(feature = "Win32_System_Com")]
pub trait ISensLogon_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Logoff(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartShell(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayLock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayUnlock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISensLogon {}
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>() -> ISensLogon_Vtbl {
        unsafe extern "system" fn Logon<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::Logon(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn Logoff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::Logoff(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartShell<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::StartShell(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayLock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::DisplayLock(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayUnlock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::DisplayUnlock(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartScreenSaver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::StartScreenSaver(this, core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StopScreenSaver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon_Impl::StopScreenSaver(this, core::mem::transmute(&bstrusername)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            StartShell: StartShell::<Identity, Impl, OFFSET>,
            DisplayLock: DisplayLock::<Identity, Impl, OFFSET>,
            DisplayUnlock: DisplayUnlock::<Identity, Impl, OFFSET>,
            StartScreenSaver: StartScreenSaver::<Identity, Impl, OFFSET>,
            StopScreenSaver: StopScreenSaver::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensLogon as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensLogon2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn Logoff(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn SessionDisconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn SessionReconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn PostShell(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISensLogon2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>() -> ISensLogon2_Vtbl {
        unsafe extern "system" fn Logon<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon2_Impl::Logon(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn Logoff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon2_Impl::Logoff(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionDisconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon2_Impl::SessionDisconnect(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionReconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon2_Impl::SessionReconnect(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn PostShell<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, dwsessionid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensLogon2_Impl::PostShell(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            SessionDisconnect: SessionDisconnect::<Identity, Impl, OFFSET>,
            SessionReconnect: SessionReconnect::<Identity, Impl, OFFSET>,
            PostShell: PostShell::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensLogon2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensNetwork_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionMade(&self, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>;
    fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()>;
    fn ConnectionLost(&self, bstrconnection: &windows_core::BSTR, ultype: SENS_CONNECTION_TYPE) -> windows_core::Result<()>;
    fn DestinationReachable(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>;
    fn DestinationReachableNoQOCInfo(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISensNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ISensNetwork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>() -> ISensNetwork_Vtbl {
        unsafe extern "system" fn ConnectionMade<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: core::mem::MaybeUninit<windows_core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensNetwork_Impl::ConnectionMade(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: core::mem::MaybeUninit<windows_core::BSTR>, ultype: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensNetwork_Impl::ConnectionMadeNoQOCInfo(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn ConnectionLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: core::mem::MaybeUninit<windows_core::BSTR>, ultype: SENS_CONNECTION_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensNetwork_Impl::ConnectionLost(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn DestinationReachable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: core::mem::MaybeUninit<windows_core::BSTR>, bstrconnection: core::mem::MaybeUninit<windows_core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensNetwork_Impl::DestinationReachable(this, core::mem::transmute(&bstrdestination), core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: core::mem::MaybeUninit<windows_core::BSTR>, bstrconnection: core::mem::MaybeUninit<windows_core::BSTR>, ultype: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensNetwork_Impl::DestinationReachableNoQOCInfo(this, core::mem::transmute(&bstrdestination), core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectionMade: ConnectionMade::<Identity, Impl, OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Identity, Impl, OFFSET>,
            ConnectionLost: ConnectionLost::<Identity, Impl, OFFSET>,
            DestinationReachable: DestinationReachable::<Identity, Impl, OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensNetwork as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensOnNow_Impl: Sized + super::Com::IDispatch_Impl {
    fn OnACPower(&self) -> windows_core::Result<()>;
    fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> windows_core::Result<()>;
    fn BatteryLow(&self, dwbatterylifepercent: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISensOnNow {}
#[cfg(feature = "Win32_System_Com")]
impl ISensOnNow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>() -> ISensOnNow_Vtbl {
        unsafe extern "system" fn OnACPower<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensOnNow_Impl::OnACPower(this).into()
        }
        unsafe extern "system" fn OnBatteryPower<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatterylifepercent: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensOnNow_Impl::OnBatteryPower(this, core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        unsafe extern "system" fn BatteryLow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatterylifepercent: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISensOnNow_Impl::BatteryLow(this, core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnACPower: OnACPower::<Identity, Impl, OFFSET>,
            OnBatteryPower: OnBatteryPower::<Identity, Impl, OFFSET>,
            BatteryLow: BatteryLow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensOnNow as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
