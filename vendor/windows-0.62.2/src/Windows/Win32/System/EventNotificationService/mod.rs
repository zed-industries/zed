#[inline]
pub unsafe fn IsDestinationReachableA<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_core::PCSTR, lpqocinfo : *mut QOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableA(lpszdestination.param().abi(), lpqocinfo as _).ok() }
}
#[inline]
pub unsafe fn IsDestinationReachableW<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_core::PCWSTR, lpqocinfo : *mut QOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableW(lpszdestination.param().abi(), lpqocinfo as _).ok() }
}
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { IsNetworkAlive(lpdwflags as _).ok() }
}
pub const CONNECTION_AOL: u32 = 4u32;
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(0u32);
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(1u32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISensLogon, ISensLogon_Vtbl, 0xd597bab3_5b9f_11d1_8dd2_00aa004abd5e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISensLogon {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISensLogon, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon {
    pub unsafe fn Logon(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Logon)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn Logoff(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Logoff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn StartShell(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartShell)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn DisplayLock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayLock)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn DisplayUnlock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayUnlock)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn StartScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartScreenSaver)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn StopScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopScreenSaver)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Logoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartShell: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartScreenSaver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopScreenSaver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensLogon_Impl: super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Logoff(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartShell(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayLock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayUnlock(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopScreenSaver(&self, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISensLogon_Vtbl {
    pub const fn new<Identity: ISensLogon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Logon<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::Logon(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn Logoff<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::Logoff(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn StartShell<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::StartShell(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn DisplayLock<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::DisplayLock(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn DisplayUnlock<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::DisplayUnlock(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn StartScreenSaver<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::StartScreenSaver(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn StopScreenSaver<Identity: ISensLogon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon_Impl::StopScreenSaver(this, core::mem::transmute(&bstrusername)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Logon: Logon::<Identity, OFFSET>,
            Logoff: Logoff::<Identity, OFFSET>,
            StartShell: StartShell::<Identity, OFFSET>,
            DisplayLock: DisplayLock::<Identity, OFFSET>,
            DisplayUnlock: DisplayUnlock::<Identity, OFFSET>,
            StartScreenSaver: StartScreenSaver::<Identity, OFFSET>,
            StopScreenSaver: StopScreenSaver::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensLogon as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensLogon {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISensLogon2, ISensLogon2_Vtbl, 0xd597bab4_5b9f_11d1_8dd2_00aa004abd5e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISensLogon2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISensLogon2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISensLogon2 {
    pub unsafe fn Logon(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Logon)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), dwsessionid).ok() }
    }
    pub unsafe fn Logoff(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Logoff)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), dwsessionid).ok() }
    }
    pub unsafe fn SessionDisconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionDisconnect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), dwsessionid).ok() }
    }
    pub unsafe fn SessionReconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionReconnect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), dwsessionid).ok() }
    }
    pub unsafe fn PostShell(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostShell)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), dwsessionid).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensLogon2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Logoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SessionDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SessionReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PostShell: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensLogon2_Impl: super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn Logoff(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn SessionDisconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn SessionReconnect(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
    fn PostShell(&self, bstrusername: &windows_core::BSTR, dwsessionid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISensLogon2_Vtbl {
    pub const fn new<Identity: ISensLogon2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Logon<Identity: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, dwsessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon2_Impl::Logon(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
            }
        }
        unsafe extern "system" fn Logoff<Identity: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, dwsessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon2_Impl::Logoff(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
            }
        }
        unsafe extern "system" fn SessionDisconnect<Identity: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, dwsessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon2_Impl::SessionDisconnect(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
            }
        }
        unsafe extern "system" fn SessionReconnect<Identity: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, dwsessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon2_Impl::SessionReconnect(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
            }
        }
        unsafe extern "system" fn PostShell<Identity: ISensLogon2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, dwsessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensLogon2_Impl::PostShell(this, core::mem::transmute(&bstrusername), core::mem::transmute_copy(&dwsessionid)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Logon: Logon::<Identity, OFFSET>,
            Logoff: Logoff::<Identity, OFFSET>,
            SessionDisconnect: SessionDisconnect::<Identity, OFFSET>,
            SessionReconnect: SessionReconnect::<Identity, OFFSET>,
            PostShell: PostShell::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensLogon2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensLogon2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISensNetwork, ISensNetwork_Vtbl, 0xd597bab1_5b9f_11d1_8dd2_00aa004abd5e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISensNetwork {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISensNetwork, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISensNetwork {
    pub unsafe fn ConnectionMade(&self, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectionMade)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnection), ultype, lpqocinfo).ok() }
    }
    pub unsafe fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectionMadeNoQOCInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnection), ultype).ok() }
    }
    pub unsafe fn ConnectionLost(&self, bstrconnection: &windows_core::BSTR, ultype: SENS_CONNECTION_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectionLost)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnection), ultype).ok() }
    }
    pub unsafe fn DestinationReachable(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestinationReachable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdestination), core::mem::transmute_copy(bstrconnection), ultype, lpqocinfo).ok() }
    }
    pub unsafe fn DestinationReachableNoQOCInfo(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestinationReachableNoQOCInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdestination), core::mem::transmute_copy(bstrconnection), ultype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensNetwork_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectionMade: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const SENS_QOCINFO) -> windows_core::HRESULT,
    pub ConnectionMadeNoQOCInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConnectionLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SENS_CONNECTION_TYPE) -> windows_core::HRESULT,
    pub DestinationReachable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const SENS_QOCINFO) -> windows_core::HRESULT,
    pub DestinationReachableNoQOCInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensNetwork_Impl: super::Com::IDispatch_Impl {
    fn ConnectionMade(&self, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>;
    fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()>;
    fn ConnectionLost(&self, bstrconnection: &windows_core::BSTR, ultype: SENS_CONNECTION_TYPE) -> windows_core::Result<()>;
    fn DestinationReachable(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>;
    fn DestinationReachableNoQOCInfo(&self, bstrdestination: &windows_core::BSTR, bstrconnection: &windows_core::BSTR, ultype: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISensNetwork_Vtbl {
    pub const fn new<Identity: ISensNetwork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionMade<Identity: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: *mut core::ffi::c_void, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensNetwork_Impl::ConnectionMade(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&lpqocinfo)).into()
            }
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Identity: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: *mut core::ffi::c_void, ultype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensNetwork_Impl::ConnectionMadeNoQOCInfo(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
            }
        }
        unsafe extern "system" fn ConnectionLost<Identity: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnection: *mut core::ffi::c_void, ultype: SENS_CONNECTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensNetwork_Impl::ConnectionLost(this, core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
            }
        }
        unsafe extern "system" fn DestinationReachable<Identity: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: *mut core::ffi::c_void, bstrconnection: *mut core::ffi::c_void, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensNetwork_Impl::DestinationReachable(this, core::mem::transmute(&bstrdestination), core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype), core::mem::transmute_copy(&lpqocinfo)).into()
            }
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Identity: ISensNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: *mut core::ffi::c_void, bstrconnection: *mut core::ffi::c_void, ultype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensNetwork_Impl::DestinationReachableNoQOCInfo(this, core::mem::transmute(&bstrdestination), core::mem::transmute(&bstrconnection), core::mem::transmute_copy(&ultype)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectionMade: ConnectionMade::<Identity, OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Identity, OFFSET>,
            ConnectionLost: ConnectionLost::<Identity, OFFSET>,
            DestinationReachable: DestinationReachable::<Identity, OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensNetwork as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensNetwork {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISensOnNow, ISensOnNow_Vtbl, 0xd597bab2_5b9f_11d1_8dd2_00aa004abd5e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISensOnNow {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISensOnNow, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISensOnNow {
    pub unsafe fn OnACPower(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnACPower)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBatteryPower)(windows_core::Interface::as_raw(self), dwbatterylifepercent).ok() }
    }
    pub unsafe fn BatteryLow(&self, dwbatterylifepercent: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BatteryLow)(windows_core::Interface::as_raw(self), dwbatterylifepercent).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISensOnNow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OnACPower: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBatteryPower: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BatteryLow: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensOnNow_Impl: super::Com::IDispatch_Impl {
    fn OnACPower(&self) -> windows_core::Result<()>;
    fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> windows_core::Result<()>;
    fn BatteryLow(&self, dwbatterylifepercent: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISensOnNow_Vtbl {
    pub const fn new<Identity: ISensOnNow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnACPower<Identity: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensOnNow_Impl::OnACPower(this).into()
            }
        }
        unsafe extern "system" fn OnBatteryPower<Identity: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatterylifepercent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensOnNow_Impl::OnBatteryPower(this, core::mem::transmute_copy(&dwbatterylifepercent)).into()
            }
        }
        unsafe extern "system" fn BatteryLow<Identity: ISensOnNow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatterylifepercent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensOnNow_Impl::BatteryLow(this, core::mem::transmute_copy(&dwbatterylifepercent)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnACPower: OnACPower::<Identity, OFFSET>,
            OnBatteryPower: OnBatteryPower::<Identity, OFFSET>,
            BatteryLow: BatteryLow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensOnNow as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISensOnNow {}
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
pub const SENS: windows_core::GUID = windows_core::GUID::from_u128(0xd597cafe_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON: windows_core::GUID = windows_core::GUID::from_u128(0xd5978630_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON2: windows_core::GUID = windows_core::GUID::from_u128(0xd5978650_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_NETWORK: windows_core::GUID = windows_core::GUID::from_u128(0xd5978620_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_ONNOW: windows_core::GUID = windows_core::GUID::from_u128(0xd5978640_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_PUBLISHER: windows_core::GUID = windows_core::GUID::from_u128(0x5fee1bd6_5b9b_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_LCE: windows_core::GUID = windows_core::GUID::from_u128(0xd3938ab0_5b9d_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_WININET: windows_core::GUID = windows_core::GUID::from_u128(0xd3938ab5_5b9d_11d1_8dd2_00aa004abd5e);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SENS_CONNECTION_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
