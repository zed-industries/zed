#[inline]
pub unsafe fn IsDestinationReachableA<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_core::PCSTR, lpqocinfo : *mut QOCINFO) -> super::super::Foundation:: BOOL);
    IsDestinationReachableA(lpszdestination.param().abi(), lpqocinfo).ok()
}
#[inline]
pub unsafe fn IsDestinationReachableW<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_core::PCWSTR, lpqocinfo : *mut QOCINFO) -> super::super::Foundation:: BOOL);
    IsDestinationReachableW(lpszdestination.param().abi(), lpqocinfo).ok()
}
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : *mut u32) -> super::super::Foundation:: BOOL);
    IsNetworkAlive(lpdwflags).ok()
}
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
    pub unsafe fn Logon<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Logon)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn Logoff<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Logoff)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn StartShell<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartShell)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn DisplayLock<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayLock)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn DisplayUnlock<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayUnlock)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn StartScreenSaver<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartScreenSaver)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
    pub unsafe fn StopScreenSaver<P0>(&self, bstrusername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StopScreenSaver)(windows_core::Interface::as_raw(self), bstrusername.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensLogon_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Logoff: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartShell: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayLock: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartScreenSaver: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StopScreenSaver: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn Logon<P0>(&self, bstrusername: P0, dwsessionid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Logon)(windows_core::Interface::as_raw(self), bstrusername.param().abi(), dwsessionid).ok()
    }
    pub unsafe fn Logoff<P0>(&self, bstrusername: P0, dwsessionid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Logoff)(windows_core::Interface::as_raw(self), bstrusername.param().abi(), dwsessionid).ok()
    }
    pub unsafe fn SessionDisconnect<P0>(&self, bstrusername: P0, dwsessionid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SessionDisconnect)(windows_core::Interface::as_raw(self), bstrusername.param().abi(), dwsessionid).ok()
    }
    pub unsafe fn SessionReconnect<P0>(&self, bstrusername: P0, dwsessionid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SessionReconnect)(windows_core::Interface::as_raw(self), bstrusername.param().abi(), dwsessionid).ok()
    }
    pub unsafe fn PostShell<P0>(&self, bstrusername: P0, dwsessionid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PostShell)(windows_core::Interface::as_raw(self), bstrusername.param().abi(), dwsessionid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensLogon2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Logon: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub Logoff: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub SessionDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub SessionReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub PostShell: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
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
    pub unsafe fn ConnectionMade<P0>(&self, bstrconnection: P0, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConnectionMade)(windows_core::Interface::as_raw(self), bstrconnection.param().abi(), ultype, lpqocinfo).ok()
    }
    pub unsafe fn ConnectionMadeNoQOCInfo<P0>(&self, bstrconnection: P0, ultype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConnectionMadeNoQOCInfo)(windows_core::Interface::as_raw(self), bstrconnection.param().abi(), ultype).ok()
    }
    pub unsafe fn ConnectionLost<P0>(&self, bstrconnection: P0, ultype: SENS_CONNECTION_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConnectionLost)(windows_core::Interface::as_raw(self), bstrconnection.param().abi(), ultype).ok()
    }
    pub unsafe fn DestinationReachable<P0, P1>(&self, bstrdestination: P0, bstrconnection: P1, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DestinationReachable)(windows_core::Interface::as_raw(self), bstrdestination.param().abi(), bstrconnection.param().abi(), ultype, lpqocinfo).ok()
    }
    pub unsafe fn DestinationReachableNoQOCInfo<P0, P1>(&self, bstrdestination: P0, bstrconnection: P1, ultype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DestinationReachableNoQOCInfo)(windows_core::Interface::as_raw(self), bstrdestination.param().abi(), bstrconnection.param().abi(), ultype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensNetwork_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectionMade: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const SENS_QOCINFO) -> windows_core::HRESULT,
    pub ConnectionMadeNoQOCInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub ConnectionLost: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SENS_CONNECTION_TYPE) -> windows_core::HRESULT,
    pub DestinationReachable: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const SENS_QOCINFO) -> windows_core::HRESULT,
    pub DestinationReachableNoQOCInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).OnACPower)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBatteryPower)(windows_core::Interface::as_raw(self), dwbatterylifepercent).ok()
    }
    pub unsafe fn BatteryLow(&self, dwbatterylifepercent: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BatteryLow)(windows_core::Interface::as_raw(self), dwbatterylifepercent).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISensOnNow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OnACPower: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBatteryPower: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BatteryLow: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub const CONNECTION_AOL: u32 = 4u32;
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(0u32);
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(1u32);
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
pub const SENSGUID_EVENTCLASS_LOGON: windows_core::GUID = windows_core::GUID::from_u128(0xd5978630_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON2: windows_core::GUID = windows_core::GUID::from_u128(0xd5978650_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_NETWORK: windows_core::GUID = windows_core::GUID::from_u128(0xd5978620_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_ONNOW: windows_core::GUID = windows_core::GUID::from_u128(0xd5978640_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_PUBLISHER: windows_core::GUID = windows_core::GUID::from_u128(0x5fee1bd6_5b9b_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_LCE: windows_core::GUID = windows_core::GUID::from_u128(0xd3938ab0_5b9d_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_WININET: windows_core::GUID = windows_core::GUID::from_u128(0xd3938ab5_5b9d_11d1_8dd2_00aa004abd5e);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SENS_CONNECTION_TYPE(pub u32);
impl windows_core::TypeKind for SENS_CONNECTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SENS_CONNECTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SENS_CONNECTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
impl windows_core::TypeKind for QOCINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for QOCINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SENS: windows_core::GUID = windows_core::GUID::from_u128(0xd597cafe_5b9f_11d1_8dd2_00aa004abd5e);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
impl windows_core::TypeKind for SENS_QOCINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SENS_QOCINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
