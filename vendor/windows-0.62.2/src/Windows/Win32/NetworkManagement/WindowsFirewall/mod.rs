#[inline]
pub unsafe fn NcFreeNetconProperties(pprops: *mut NETCON_PROPERTIES) {
    windows_core::link!("netshell.dll" "system" fn NcFreeNetconProperties(pprops : *mut NETCON_PROPERTIES));
    unsafe { NcFreeNetconProperties(pprops as _) }
}
#[inline]
pub unsafe fn NcIsValidConnectionName<P0>(pszwname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netshell.dll" "system" fn NcIsValidConnectionName(pszwname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { NcIsValidConnectionName(pszwname.param().abi()) }
}
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<P0>(wszservername: P0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
    unsafe { NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.param().abi(), netisoerror as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32);
    unsafe { NetworkIsolationEnumAppContainers(flags, pdwnumpublicappcs as _, pppublicappcs as _) }
}
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn NetworkIsolationEnumerateAppContainerRules() -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
    windows_core::link!("firewallapi.dll" "system" fn NetworkIsolationEnumerateAppContainerRules(newenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NetworkIsolationEnumerateAppContainerRules(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
    unsafe { NetworkIsolationFreeAppContainers(ppublicappcs) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut *mut super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    unsafe { NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs as _, appcontainersids as _) }
}
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdAsync<P0>(wszservername: P0, dwflags: u32, context: Option<*const core::ffi::c_void>, callback: PNETISO_EDP_ID_CALLBACK_FN, hoperation: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdAsync(wszservername : windows_core::PCWSTR, dwflags : u32, context : *const core::ffi::c_void, callback : PNETISO_EDP_ID_CALLBACK_FN, hoperation : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { NetworkIsolationGetEnterpriseIdAsync(wszservername.param().abi(), dwflags, context.unwrap_or(core::mem::zeroed()) as _, callback, hoperation as _) }
}
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdClose(hoperation: super::super::Foundation::HANDLE, bwaitforoperation: bool) -> u32 {
    windows_core::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdClose(hoperation : super::super::Foundation:: HANDLE, bwaitforoperation : windows_core::BOOL) -> u32);
    unsafe { NetworkIsolationGetEnterpriseIdClose(hoperation, bwaitforoperation.into()) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: Option<*const core::ffi::c_void>, registrationobject: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const core::ffi::c_void, registrationobject : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { NetworkIsolationRegisterForAppContainerChanges(flags, callback, context.unwrap_or(core::mem::zeroed()) as _, registrationobject as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[super::super::Security::SID_AND_ATTRIBUTES]) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    unsafe { NetworkIsolationSetAppContainerConfig(appcontainersids.len().try_into().unwrap(), core::mem::transmute(appcontainersids.as_ptr())) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<P1, P2, P3>(applicationcontainersid: super::super::Security::PSID, packagefullname: P1, packagefolder: P2, displayname: P3, bbinariesfullycomputed: bool, binaries: &[windows_core::PCWSTR]) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::super::Security:: PSID, packagefullname : windows_core::PCWSTR, packagefolder : windows_core::PCWSTR, displayname : windows_core::PCWSTR, bbinariesfullycomputed : windows_core::BOOL, binaries : *const windows_core::PCWSTR, binariescount : u32) -> windows_core::HRESULT);
    unsafe { NetworkIsolationSetupAppContainerBinaries(applicationcontainersid, packagefullname.param().abi(), packagefolder.param().abi(), displayname.param().abi(), bbinariesfullycomputed.into(), core::mem::transmute(binaries.as_ptr()), binaries.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::super::Foundation:: HANDLE) -> u32);
    unsafe { NetworkIsolationUnregisterForAppContainerChanges(registrationobject) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_core::GUID,
    pub keyword: windows_core::PCWSTR,
    pub flags: u32,
    pub addresses: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl Default for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
impl FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
impl FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICS_TARGETTYPE(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDynamicPortMapping, IDynamicPortMapping_Vtbl, 0x4fc80282_23b6_4378_9a27_cd8f17c9400c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDynamicPortMapping {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDynamicPortMapping, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalIPAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RemoteHost(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteHost)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternalClient)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LeaseDuration(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LeaseDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RenewLease)(windows_core::Interface::as_raw(self), lleasedurationdesired, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditInternalClient)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinternalclient)).ok() }
    }
    pub unsafe fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), vb).ok() }
    }
    pub unsafe fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)).ok() }
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditInternalPort)(windows_core::Interface::as_raw(self), linternalport).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LeaseDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RenewLease: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDynamicPortMapping_Impl: super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RemoteHost(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn Protocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LeaseDuration(&self) -> windows_core::Result<i32>;
    fn RenewLease(&self, lleasedurationdesired: i32) -> windows_core::Result<i32>;
    fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDynamicPortMapping_Vtbl {
    pub const fn new<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExternalIPAddress<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::ExternalIPAddress(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoteHost<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::RemoteHost(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::ExternalPort(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Protocol<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::Protocol(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InternalPort<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::InternalPort(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InternalClient<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::InternalClient(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::Description(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LeaseDuration<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::LeaseDuration(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RenewLease<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMapping_Impl::RenewLease(this, core::mem::transmute_copy(&lleasedurationdesired)) {
                    Ok(ok__) => {
                        pleasedurationreturned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
            }
        }
        unsafe extern "system" fn Enable<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
            }
        }
        unsafe extern "system" fn EditDescription<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn EditInternalPort<Identity: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, OFFSET>,
            RemoteHost: RemoteHost::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            InternalClient: InternalClient::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            LeaseDuration: LeaseDuration::<Identity, OFFSET>,
            RenewLease: RenewLease::<Identity, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            EditDescription: EditDescription::<Identity, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDynamicPortMapping {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDynamicPortMappingCollection, IDynamicPortMappingCollection_Vtbl, 0xb60de00f_156e_4e8d_9ec1_3a2342c10899);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDynamicPortMappingCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDynamicPortMappingCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IDynamicPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrremotehost), lexternalport, core::mem::transmute_copy(bstrprotocol), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrremotehost), lexternalport, core::mem::transmute_copy(bstrprotocol)).ok() }
    }
    pub unsafe fn Add(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR, lleaseduration: i32) -> windows_core::Result<IDynamicPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrremotehost), lexternalport, core::mem::transmute_copy(bstrprotocol), linternalport, core::mem::transmute_copy(bstrinternalclient), benabled, core::mem::transmute_copy(bstrdescription), lleaseduration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDynamicPortMappingCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IDynamicPortMapping>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR, lleaseduration: i32) -> windows_core::Result<IDynamicPortMapping>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDynamicPortMappingCollection_Vtbl {
    pub const fn new<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMappingCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMappingCollection_Impl::get_Item(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        ppdpm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMappingCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicPortMappingCollection_Impl::Remove(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void, linternalport: i32, bstrinternalclient: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: *mut core::ffi::c_void, lleaseduration: i32, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDynamicPortMappingCollection_Impl::Add(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription), core::mem::transmute_copy(&lleaseduration)) {
                    Ok(ok__) => {
                        ppdpm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDynamicPortMappingCollection {}
windows_core::imp::define_interface!(IEnumNetConnection, IEnumNetConnection_Vtbl, 0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(IEnumNetConnection, windows_core::IUnknown);
impl IEnumNetConnection {
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetConnection>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumNetConnection_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<INetConnection>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetConnection>;
}
impl IEnumNetConnection_Vtbl {
    pub const fn new<Identity: IEnumNetConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetConnection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetConnection_Impl::Clone(this) {
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
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumNetConnection {}
windows_core::imp::define_interface!(IEnumNetSharingEveryConnection, IEnumNetSharingEveryConnection_Vtbl, 0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(IEnumNetSharingEveryConnection, windows_core::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingEveryConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::System::Variant::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingEveryConnection_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingEveryConnection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetSharingEveryConnection_Vtbl {
    pub const fn new<Identity: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingEveryConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingEveryConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingEveryConnection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetSharingEveryConnection_Impl::Clone(this) {
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
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingEveryConnection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetSharingEveryConnection {}
windows_core::imp::define_interface!(IEnumNetSharingPortMapping, IEnumNetSharingPortMapping_Vtbl, 0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(IEnumNetSharingPortMapping, windows_core::IUnknown);
impl IEnumNetSharingPortMapping {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::System::Variant::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPortMapping_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPortMapping>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetSharingPortMapping_Vtbl {
    pub const fn new<Identity: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPortMapping_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPortMapping_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPortMapping_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetSharingPortMapping_Impl::Clone(this) {
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
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPortMapping as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetSharingPortMapping {}
windows_core::imp::define_interface!(IEnumNetSharingPrivateConnection, IEnumNetSharingPrivateConnection_Vtbl, 0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(IEnumNetSharingPrivateConnection, windows_core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPrivateConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::System::Variant::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPrivateConnection_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPrivateConnection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetSharingPrivateConnection_Vtbl {
    pub const fn new<Identity: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPrivateConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPrivateConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPrivateConnection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetSharingPrivateConnection_Impl::Clone(this) {
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
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPrivateConnection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetSharingPrivateConnection {}
windows_core::imp::define_interface!(IEnumNetSharingPublicConnection, IEnumNetSharingPublicConnection_Vtbl, 0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(IEnumNetSharingPublicConnection, windows_core::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPublicConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::System::Variant::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPublicConnection_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPublicConnection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetSharingPublicConnection_Vtbl {
    pub const fn new<Identity: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPublicConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPublicConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNetSharingPublicConnection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNetSharingPublicConnection_Impl::Clone(this) {
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
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPublicConnection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumNetSharingPublicConnection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INATEventManager, INATEventManager_Vtbl, 0x624bd588_9060_4109_b0b0_1adbbcac32df);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INATEventManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INATEventManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INATEventManager {
    pub unsafe fn SetExternalIPAddressCallback<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetExternalIPAddressCallback)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn SetNumberOfEntriesCallback<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNumberOfEntriesCallback)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetExternalIPAddressCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumberOfEntriesCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INATEventManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn SetExternalIPAddressCallback(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetNumberOfEntriesCallback(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INATEventManager_Vtbl {
    pub const fn new<Identity: INATEventManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetExternalIPAddressCallback<Identity: INATEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INATEventManager_Impl::SetExternalIPAddressCallback(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Identity: INATEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INATEventManager_Impl::SetNumberOfEntriesCallback(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Identity, OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATEventManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INATEventManager {}
windows_core::imp::define_interface!(INATExternalIPAddressCallback, INATExternalIPAddressCallback_Vtbl, 0x9c416740_a34e_446f_ba06_abd04c3149ae);
windows_core::imp::interface_hierarchy!(INATExternalIPAddressCallback, windows_core::IUnknown);
impl INATExternalIPAddressCallback {
    pub unsafe fn NewExternalIPAddress(&self, bstrnewexternalipaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewExternalIPAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnewexternalipaddress)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait INATExternalIPAddressCallback_Impl: windows_core::IUnknownImpl {
    fn NewExternalIPAddress(&self, bstrnewexternalipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl INATExternalIPAddressCallback_Vtbl {
    pub const fn new<Identity: INATExternalIPAddressCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewExternalIPAddress<Identity: INATExternalIPAddressCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewexternalipaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INATExternalIPAddressCallback_Impl::NewExternalIPAddress(this, core::mem::transmute(&bstrnewexternalipaddress)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewExternalIPAddress: NewExternalIPAddress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATExternalIPAddressCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INATExternalIPAddressCallback {}
windows_core::imp::define_interface!(INATNumberOfEntriesCallback, INATNumberOfEntriesCallback_Vtbl, 0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
windows_core::imp::interface_hierarchy!(INATNumberOfEntriesCallback, windows_core::IUnknown);
impl INATNumberOfEntriesCallback {
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewNumberOfEntries)(windows_core::Interface::as_raw(self), lnewnumberofentries).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewNumberOfEntries: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait INATNumberOfEntriesCallback_Impl: windows_core::IUnknownImpl {
    fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> windows_core::Result<()>;
}
impl INATNumberOfEntriesCallback_Vtbl {
    pub const fn new<Identity: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewNumberOfEntries<Identity: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewnumberofentries: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INATNumberOfEntriesCallback_Impl::NewNumberOfEntries(this, core::mem::transmute_copy(&lnewnumberofentries)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewNumberOfEntries: NewNumberOfEntries::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATNumberOfEntriesCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INATNumberOfEntriesCallback {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut windows_core::PWSTR,
}
impl Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: windows_core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: windows_core::PWSTR,
    pub displayName: windows_core::PWSTR,
    pub description: windows_core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(INetConnection, INetConnection_Vtbl, 0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(INetConnection, windows_core::IUnknown);
impl INetConnection {
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Duplicate<P0>(&self, pszwduplicatename: P0) -> windows_core::Result<INetConnection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duplicate)(windows_core::Interface::as_raw(self), pszwduplicatename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<*mut NETCON_PROPERTIES> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUiObjectClassId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUiObjectClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Rename<P0>(&self, pszwnewname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), pszwnewname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duplicate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut NETCON_PROPERTIES) -> windows_core::HRESULT,
    pub GetUiObjectClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait INetConnection_Impl: windows_core::IUnknownImpl {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Duplicate(&self, pszwduplicatename: &windows_core::PCWSTR) -> windows_core::Result<INetConnection>;
    fn GetProperties(&self) -> windows_core::Result<*mut NETCON_PROPERTIES>;
    fn GetUiObjectClassId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Rename(&self, pszwnewname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl INetConnection_Vtbl {
    pub const fn new<Identity: INetConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnection_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnection_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnection_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Duplicate<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwduplicatename: windows_core::PCWSTR, ppcon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnection_Impl::Duplicate(this, core::mem::transmute(&pszwduplicatename)) {
                    Ok(ok__) => {
                        ppcon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnection_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        ppprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnection_Impl::GetUiObjectClassId(this) {
                    Ok(ok__) => {
                        pclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Rename<Identity: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwnewname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnection_Impl::Rename(this, core::mem::transmute(&pszwnewname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Duplicate: Duplicate::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetConnection {}
windows_core::imp::define_interface!(INetConnectionConnectUi, INetConnectionConnectUi_Vtbl, 0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(INetConnectionConnectUi, windows_core::IUnknown);
impl INetConnectionConnectUi {
    pub unsafe fn SetConnection<P0>(&self, pcon: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetConnection)(windows_core::Interface::as_raw(self), pcon.param().abi()).ok() }
    }
    pub unsafe fn Connect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), hwndparent, dwflags).ok() }
    }
    pub unsafe fn Disconnect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), hwndparent, dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
}
pub trait INetConnectionConnectUi_Impl: windows_core::IUnknownImpl {
    fn SetConnection(&self, pcon: windows_core::Ref<INetConnection>) -> windows_core::Result<()>;
    fn Connect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
    fn Disconnect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
}
impl INetConnectionConnectUi_Vtbl {
    pub const fn new<Identity: INetConnectionConnectUi_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConnection<Identity: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnectionConnectUi_Impl::SetConnection(this, core::mem::transmute_copy(&pcon)).into()
            }
        }
        unsafe extern "system" fn Connect<Identity: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnectionConnectUi_Impl::Connect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetConnectionConnectUi_Impl::Disconnect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConnection: SetConnection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionConnectUi as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetConnectionConnectUi {}
windows_core::imp::define_interface!(INetConnectionManager, INetConnectionManager_Vtbl, 0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
windows_core::imp::interface_hierarchy!(INetConnectionManager, windows_core::IUnknown);
impl INetConnectionManager {
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> windows_core::Result<IEnumNetConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, NETCONMGR_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait INetConnectionManager_Impl: windows_core::IUnknownImpl {
    fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> windows_core::Result<IEnumNetConnection>;
}
impl INetConnectionManager_Vtbl {
    pub const fn new<Identity: INetConnectionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumConnections<Identity: INetConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionManager_Impl::EnumConnections(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumConnections: EnumConnections::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INetConnectionManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetConnectionProps, INetConnectionProps_Vtbl, 0xf4277c95_ce5b_463d_8167_5662d9bcaa72);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetConnectionProps {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetConnectionProps, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetConnectionProps {
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<NETCON_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MediaType(&self) -> windows_core::Result<NETCON_MEDIATYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Characteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Characteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NETCON_STATUS) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NETCON_MEDIATYPE) -> windows_core::HRESULT,
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetConnectionProps_Impl: super::super::System::Com::IDispatch_Impl {
    fn Guid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<NETCON_STATUS>;
    fn MediaType(&self) -> windows_core::Result<NETCON_MEDIATYPE>;
    fn Characteristics(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetConnectionProps_Vtbl {
    pub const fn new<Identity: INetConnectionProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Guid<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::Guid(this) {
                    Ok(ok__) => {
                        pbstrguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceName<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::DeviceName(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaType<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::MediaType(this) {
                    Ok(ok__) => {
                        pmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Characteristics<Identity: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetConnectionProps_Impl::Characteristics(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Guid: Guid::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            MediaType: MediaType::<Identity, OFFSET>,
            Characteristics: Characteristics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionProps as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetConnectionProps {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwAuthorizedApplication, INetFwAuthorizedApplication_Vtbl, 0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwAuthorizedApplication {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplication, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplication {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessImageFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProcessImageFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)).ok() }
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok() }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok() }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwAuthorizedApplication_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::ProcessImageFileName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetProcessImageFileName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwAuthorizedApplication {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwAuthorizedApplications, INetFwAuthorizedApplications_Vtbl, 0x644efd52_ccf9_486c_97a2_39f352570b30);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwAuthorizedApplications {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplications, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplications {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, app: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwAuthorizedApplication>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), app.param().abi()).ok() }
    }
    pub unsafe fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)).ok() }
    }
    pub unsafe fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwAuthorizedApplications_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, app: windows_core::Ref<INetFwAuthorizedApplication>) -> windows_core::Result<()>;
    fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplications_Impl::Add(this, core::mem::transmute_copy(&app)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplications_Impl::Remove(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void, app: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::Item(this, core::mem::transmute(&imagefilename)) {
                    Ok(ok__) => {
                        app.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwAuthorizedApplications {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwIcmpSettings, INetFwIcmpSettings_Vtbl, 0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwIcmpSettings {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwIcmpSettings, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwIcmpSettings {
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowRedirect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowRedirect(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowRedirect)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundEchoRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundEchoRequest)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundRouterRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundRouterRequest)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundMaskRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundMaskRequest)(windows_core::Interface::as_raw(self), allow).ok() }
    }
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), allow).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwIcmpSettings_Impl: super::super::System::Com::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundDestinationUnreachable(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowRedirect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowRedirect(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundEchoRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundTimeExceeded(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundParameterProblem(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundSourceQuench(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundRouterRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundTimestampRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundMaskRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundPacketTooBig(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundDestinationUnreachable(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundDestinationUnreachable(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowRedirect(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowRedirect(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundEchoRequest(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundEchoRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundTimeExceeded(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundTimeExceeded(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundParameterProblem(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundParameterProblem(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundSourceQuench(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundSourceQuench(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundRouterRequest(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundRouterRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundTimestampRequest(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundTimestampRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundMaskRequest(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundMaskRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundPacketTooBig(this) {
                    Ok(ok__) => {
                        allow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundPacketTooBig(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwIcmpSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwMgr, INetFwMgr_Vtbl, 0xf7898af5_cac4_4632_a2ec_da06e5111af2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwMgr {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwMgr, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwMgr {
    pub unsafe fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentProfileType(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDefaults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestoreDefaults)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &windows_core::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPortAllowed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename), ipversion, portnumber, core::mem::transmute_copy(localaddress), ipprotocol, core::mem::transmute(allowed), core::mem::transmute(restricted)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn IsIcmpTypeAllowed(&self, ipversion: NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(windows_core::Interface::as_raw(self), ipversion, core::mem::transmute_copy(localaddress), r#type, core::mem::transmute(allowed), core::mem::transmute(restricted)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProfileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    pub RestoreDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub IsPortAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, NET_FW_IP_VERSION, i32, *mut core::ffi::c_void, NET_FW_IP_PROTOCOL, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION, *mut core::ffi::c_void, u8, *mut super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    IsIcmpTypeAllowed: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwMgr_Impl: super::super::System::Com::IDispatch_Impl {
    fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy>;
    fn CurrentProfileType(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&self) -> windows_core::Result<()>;
    fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &windows_core::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn IsIcmpTypeAllowed(&self, ipversion: NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: INetFwMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalPolicy<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::LocalPolicy(this) {
                    Ok(ok__) => {
                        localpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProfileType<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::CurrentProfileType(this) {
                    Ok(ok__) => {
                        profiletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDefaults<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::RestoreDefaults(this).into()
            }
        }
        unsafe extern "system" fn IsPortAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: *mut core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsPortAllowed(this, core::mem::transmute(&imagefilename), core::mem::transmute_copy(&ipversion), core::mem::transmute_copy(&portnumber), core::mem::transmute(&localaddress), core::mem::transmute_copy(&ipprotocol), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: *mut core::ffi::c_void, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsIcmpTypeAllowed(this, core::mem::transmute_copy(&ipversion), core::mem::transmute(&localaddress), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LocalPolicy: LocalPolicy::<Identity, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwMgr as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwMgr {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwOpenPort, INetFwOpenPort_Vtbl, 0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwOpenPort {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwOpenPort, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPort {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok() }
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<NET_FW_IP_PROTOCOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), ipprotocol).ok() }
    }
    pub unsafe fn Port(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), portnumber).ok() }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok() }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn BuiltIn(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuiltIn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BuiltIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwOpenPort_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<NET_FW_IP_PROTOCOL>;
    fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Port(&self) -> windows_core::Result<i32>;
    fn SetPort(&self, portnumber: i32) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BuiltIn(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: INetFwOpenPort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Protocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Protocol(this) {
                    Ok(ok__) => {
                        ipprotocol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetProtocol(this, core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn Port<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Port(this) {
                    Ok(ok__) => {
                        portnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPort<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetPort(this, core::mem::transmute_copy(&portnumber)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn BuiltIn<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, builtin: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::BuiltIn(this) {
                    Ok(ok__) => {
                        builtin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            Port: Port::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            BuiltIn: BuiltIn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPort as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwOpenPort {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwOpenPorts, INetFwOpenPorts_Vtbl, 0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwOpenPorts {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwOpenPorts, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPorts {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, port: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwOpenPort>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), port.param().abi()).ok() }
    }
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), portnumber, ipprotocol).ok() }
    }
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), portnumber, ipprotocol, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct INetFwOpenPorts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, NET_FW_IP_PROTOCOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwOpenPorts_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, port: windows_core::Ref<INetFwOpenPort>) -> windows_core::Result<()>;
    fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPorts_Impl::Add(this, core::mem::transmute_copy(&port)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPorts_Impl::Remove(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::Item(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)) {
                    Ok(ok__) => {
                        openport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPorts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwOpenPorts {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwPolicy, INetFwPolicy_Vtbl, 0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwPolicy {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwPolicy, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy {
    pub unsafe fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProfileByType)(windows_core::Interface::as_raw(self), profiletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProfileByType: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwPolicy_Impl: super::super::System::Com::IDispatch_Impl {
    fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile>;
    fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: INetFwPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentProfile<Identity: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy_Impl::CurrentProfile(this) {
                    Ok(ok__) => {
                        profile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProfileByType<Identity: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy_Impl::GetProfileByType(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        profile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfile: CurrentProfile::<Identity, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwPolicy {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwPolicy2, INetFwPolicy2_Vtbl, 0x98325047_c671_4174_8d81_defcd3f03186);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwPolicy2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwPolicy2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy2 {
    pub unsafe fn CurrentProfileTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, enabled).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, core::mem::transmute_copy(interfaces)).ok() }
    }
    pub unsafe fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, block).ok() }
    }
    pub unsafe fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled).ok() }
    }
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled).ok() }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceRestriction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableRuleGroup)(windows_core::Interface::as_raw(self), profiletypesbitmask, core::mem::transmute_copy(group), enable).ok() }
    }
    pub unsafe fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRuleGroupEnabled)(windows_core::Interface::as_raw(self), profiletypesbitmask, core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestoreLocalFirewallDefaults)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, action).ok() }
    }
    pub unsafe fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, action).ok() }
    }
    pub unsafe fn get_IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsRuleGroupCurrentlyEnabled)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalPolicyModifyState(&self) -> windows_core::Result<NET_FW_MODIFY_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPolicyModifyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentProfileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_ExcludedInterfaces: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_ExcludedInterfaces: usize,
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableRuleGroup: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsRuleGroupEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub put_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, NET_FW_ACTION) -> windows_core::HRESULT,
    pub get_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub put_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, NET_FW_ACTION) -> windows_core::HRESULT,
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LocalPolicyModifyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_MODIFY_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwPolicy2_Impl: super::super::System::Com::IDispatch_Impl {
    fn CurrentProfileTypes(&self) -> windows_core::Result<i32>;
    fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
    fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()>;
    fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalPolicyModifyState(&self) -> windows_core::Result<NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: INetFwPolicy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentProfileTypes<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::CurrentProfileTypes(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_FirewallEnabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_FirewallEnabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_ExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype), core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn get_BlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        block.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&block)).into()
            }
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn Rules<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::Rules(this) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceRestriction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicerestriction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::ServiceRestriction(this) {
                    Ok(ok__) => {
                        servicerestriction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: *mut core::ffi::c_void, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::EnableRuleGroup(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::IsRuleGroupEnabled(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group)) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::RestoreLocalFirewallDefaults(this).into()
            }
        }
        unsafe extern "system" fn get_DefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn get_DefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn get_IsRuleGroupCurrentlyEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_IsRuleGroupCurrentlyEnabled(this, core::mem::transmute(&group)) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::LocalPolicyModifyState(this) {
                    Ok(ok__) => {
                        modifystate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Identity, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, OFFSET>,
            get_ExcludedInterfaces: get_ExcludedInterfaces::<Identity, OFFSET>,
            put_ExcludedInterfaces: put_ExcludedInterfaces::<Identity, OFFSET>,
            get_BlockAllInboundTraffic: get_BlockAllInboundTraffic::<Identity, OFFSET>,
            put_BlockAllInboundTraffic: put_BlockAllInboundTraffic::<Identity, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, OFFSET>,
            get_DefaultInboundAction: get_DefaultInboundAction::<Identity, OFFSET>,
            put_DefaultInboundAction: put_DefaultInboundAction::<Identity, OFFSET>,
            get_DefaultOutboundAction: get_DefaultOutboundAction::<Identity, OFFSET>,
            put_DefaultOutboundAction: put_DefaultOutboundAction::<Identity, OFFSET>,
            get_IsRuleGroupCurrentlyEnabled: get_IsRuleGroupCurrentlyEnabled::<Identity, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwPolicy2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwProduct, INetFwProduct_Vtbl, 0x71881699_18f4_458b_b892_3ffce5e07f75);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwProduct {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwProduct, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProduct {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RuleCategories(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetRuleCategories(&self, rulecategories: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRuleCategories)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulecategories)).ok() }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)).ok() }
    }
    pub unsafe fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathToSignedProductExe)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RuleCategories: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetRuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetRuleCategories: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathToSignedProductExe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProduct_Impl: super::super::System::Com::IDispatch_Impl {
    fn RuleCategories(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetRuleCategories(&self, rulecategories: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: INetFwProduct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::RuleCategories(this) {
                    Ok(ok__) => {
                        rulecategories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::SetRuleCategories(this, core::mem::transmute(&rulecategories)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::PathToSignedProductExe(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RuleCategories: RuleCategories::<Identity, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProduct as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwProduct {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwProducts, INetFwProducts_Vtbl, 0x39eb36e0_2097_40bd_8af2_63a13b525362);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwProducts {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwProducts, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProducts {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Register<P0>(&self, product: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<INetFwProduct>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), product.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct INetFwProducts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProducts_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Register(&self, product: windows_core::Ref<INetFwProduct>) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: INetFwProducts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Register<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, product: *mut core::ffi::c_void, registration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Register(this, core::mem::transmute_copy(&product)) {
                    Ok(ok__) => {
                        registration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, product: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        product.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProducts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwProducts {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwProfile, INetFwProfile_Vtbl, 0x174a0dda_e9f9_449d_993b_21ab667ca456);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwProfile {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwProfile, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProfile {
    pub unsafe fn Type(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFirewallEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFirewallEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExceptionsNotAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionsNotAllowed)(windows_core::Interface::as_raw(self), notallowed).ok() }
    }
    pub unsafe fn NotificationsDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationsDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationsDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationsDisabled)(windows_core::Interface::as_raw(self), disabled).ok() }
    }
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), disabled).ok() }
    }
    pub unsafe fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAdminSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IcmpSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Services(&self) -> windows_core::Result<INetFwServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthorizedApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    pub FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetFirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RemoteAdminSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IcmpSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthorizedApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProfile_Impl: super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFirewallEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExceptionsNotAllowed(&self, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NotificationsDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNotificationsDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
    fn Services(&self) -> windows_core::Result<INetFwServices>;
    fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: INetFwProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::FirewallEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetFirewallEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::ExceptionsNotAllowed(this) {
                    Ok(ok__) => {
                        notallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetExceptionsNotAllowed(this, core::mem::transmute_copy(&notallowed)).into()
            }
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::NotificationsDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetNotificationsDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::UnicastResponsesToMulticastBroadcastDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteadminsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::RemoteAdminSettings(this) {
                    Ok(ok__) => {
                        remoteadminsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IcmpSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmpsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::IcmpSettings(this) {
                    Ok(ok__) => {
                        icmpsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::GloballyOpenPorts(this) {
                    Ok(ok__) => {
                        openports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Services<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, services: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::Services(this) {
                    Ok(ok__) => {
                        services.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::AuthorizedApplications(this) {
                    Ok(ok__) => {
                        apps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
            Services: Services::<Identity, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProfile as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwProfile {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwRemoteAdminSettings, INetFwRemoteAdminSettings_Vtbl, 0xd4becddf_6f73_4a83_b832_9c66874cd20e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwRemoteAdminSettings {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwRemoteAdminSettings, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRemoteAdminSettings {
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok() }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok() }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRemoteAdminSettings_Impl: super::super::System::Com::IDispatch_Impl {
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwRemoteAdminSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwRule, INetFwRule_Vtbl, 0xaf230d27_baba_4e42_aced_f524f22cfce2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwRule {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwRule, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, desc: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desc)).ok() }
    }
    pub unsafe fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)).ok() }
    }
    pub unsafe fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename)).ok() }
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), protocol).ok() }
    }
    pub unsafe fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalPorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)).ok() }
    }
    pub unsafe fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemotePorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemotePorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)).ok() }
    }
    pub unsafe fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localaddrs)).ok() }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)).ok() }
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IcmpTypesAndCodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIcmpTypesAndCodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(icmptypesandcodes)).ok() }
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<NET_FW_RULE_DIRECTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDirection)(windows_core::Interface::as_raw(self), dir).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Interfaces(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Interfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetInterfaces(&self, interfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfaces)).ok() }
    }
    pub unsafe fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterfaceTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceTypes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfacetypes)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn Grouping(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Grouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetGrouping(&self, context: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGrouping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(context)).ok() }
    }
    pub unsafe fn Profiles(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProfiles)(windows_core::Interface::as_raw(self), profiletypesbitmask).ok() }
    }
    pub unsafe fn EdgeTraversal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EdgeTraversal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEdgeTraversal(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEdgeTraversal)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn Action(&self) -> windows_core::Result<NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAction)(windows_core::Interface::as_raw(self), action).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Interfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Interfaces: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetInterfaces: usize,
    pub InterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Grouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_ACTION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, desc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<i32>;
    fn SetProtocol(&self, protocol: i32) -> windows_core::Result<()>;
    fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Direction(&self) -> windows_core::Result<NET_FW_RULE_DIRECTION>;
    fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> windows_core::Result<()>;
    fn Interfaces(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetInterfaces(&self, interfaces: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Grouping(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGrouping(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Profiles(&self) -> windows_core::Result<i32>;
    fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()>;
    fn EdgeTraversal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEdgeTraversal(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Action(&self) -> windows_core::Result<NET_FW_ACTION>;
    fn SetAction(&self, action: NET_FW_ACTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: INetFwRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetDescription(this, core::mem::transmute(&desc)).into()
            }
        }
        unsafe extern "system" fn ApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::ApplicationName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetApplicationName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn ServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::ServiceName(this) {
                    Ok(ok__) => {
                        servicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetServiceName(this, core::mem::transmute(&servicename)).into()
            }
        }
        unsafe extern "system" fn Protocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Protocol(this) {
                    Ok(ok__) => {
                        protocol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetProtocol(this, core::mem::transmute_copy(&protocol)).into()
            }
        }
        unsafe extern "system" fn LocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::LocalPorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetLocalPorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn RemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::RemotePorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetRemotePorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn LocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::LocalAddresses(this) {
                    Ok(ok__) => {
                        localaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetLocalAddresses(this, core::mem::transmute(&localaddrs)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::IcmpTypesAndCodes(this) {
                    Ok(ok__) => {
                        icmptypesandcodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetIcmpTypesAndCodes(this, core::mem::transmute(&icmptypesandcodes)).into()
            }
        }
        unsafe extern "system" fn Direction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Direction(this) {
                    Ok(ok__) => {
                        dir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDirection<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetDirection(this, core::mem::transmute_copy(&dir)).into()
            }
        }
        unsafe extern "system" fn Interfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Interfaces(this) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInterfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetInterfaces(this, core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn InterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::InterfaceTypes(this) {
                    Ok(ok__) => {
                        interfacetypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetInterfaceTypes(this, core::mem::transmute(&interfacetypes)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Grouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Grouping(this) {
                    Ok(ok__) => {
                        context.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGrouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetGrouping(this, core::mem::transmute(&context)).into()
            }
        }
        unsafe extern "system" fn Profiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Profiles(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProfiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetProfiles(this, core::mem::transmute_copy(&profiletypesbitmask)).into()
            }
        }
        unsafe extern "system" fn EdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::EdgeTraversal(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetEdgeTraversal(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Action<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Action(this) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetAction(this, core::mem::transmute_copy(&action)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationName: ApplicationName::<Identity, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, OFFSET>,
            ServiceName: ServiceName::<Identity, OFFSET>,
            SetServiceName: SetServiceName::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            LocalPorts: LocalPorts::<Identity, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, OFFSET>,
            RemotePorts: RemotePorts::<Identity, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            SetDirection: SetDirection::<Identity, OFFSET>,
            Interfaces: Interfaces::<Identity, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Grouping: Grouping::<Identity, OFFSET>,
            SetGrouping: SetGrouping::<Identity, OFFSET>,
            Profiles: Profiles::<Identity, OFFSET>,
            SetProfiles: SetProfiles::<Identity, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, OFFSET>,
            Action: Action::<Identity, OFFSET>,
            SetAction: SetAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwRule {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwRule2, INetFwRule2_Vtbl, 0x9c27c8da_189b_4dde_89f7_8b39a316782c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwRule2 {
    type Target = INetFwRule;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwRule2, windows_core::IUnknown, super::super::System::Com::IDispatch, INetFwRule);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule2 {
    pub unsafe fn EdgeTraversalOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EdgeTraversalOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEdgeTraversalOptions)(windows_core::Interface::as_raw(self), loptions).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub EdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule2_Impl: INetFwRule_Impl {
    fn EdgeTraversalOptions(&self) -> windows_core::Result<i32>;
    fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: INetFwRule2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule2_Impl::EdgeTraversalOptions(this) {
                    Ok(ok__) => {
                        loptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule2_Impl::SetEdgeTraversalOptions(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule_Vtbl::new::<Identity, OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwRule2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwRule3, INetFwRule3_Vtbl, 0xb21563ff_d696_4222_ab46_4e89b73ab34a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwRule3 {
    type Target = INetFwRule2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwRule3, windows_core::IUnknown, super::super::System::Com::IDispatch, INetFwRule, INetFwRule2);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule3 {
    pub unsafe fn LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalAppPackageId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAppPackageId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszpackageid)).ok() }
    }
    pub unsafe fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserOwner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserowner)).ok() }
    }
    pub unsafe fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)).ok() }
    }
    pub unsafe fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)).ok() }
    }
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)).ok() }
    }
    pub unsafe fn SecureFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecureFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecureFlags)(windows_core::Interface::as_raw(self), loptions).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub LocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule3_Impl: INetFwRule2_Impl {
    fn LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SecureFlags(&self) -> windows_core::Result<i32>;
    fn SetSecureFlags(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: INetFwRule3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalAppPackageId(this) {
                    Ok(ok__) => {
                        wszpackageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalAppPackageId(this, core::mem::transmute(&wszpackageid)).into()
            }
        }
        unsafe extern "system" fn LocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalUserOwner(this) {
                    Ok(ok__) => {
                        wszuserowner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalUserOwner(this, core::mem::transmute(&wszuserowner)).into()
            }
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::RemoteUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetRemoteUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::RemoteMachineAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetRemoteMachineAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn SecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::SecureFlags(this) {
                    Ok(ok__) => {
                        loptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetSecureFlags(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule2_Vtbl::new::<Identity, OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Identity, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, OFFSET>,
            SecureFlags: SecureFlags::<Identity, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<INetFwRule2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwRule3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwRules, INetFwRules_Vtbl, 0x9c4c6277_5027_441e_afae_ca1f542da009);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwRules {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwRules, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRules {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, rule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), rule.param().abi()).ok() }
    }
    pub unsafe fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct INetFwRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRules_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, rule: windows_core::Ref<INetFwRule>) -> windows_core::Result<()>;
    fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: INetFwRules_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRules_Impl::Add(this, core::mem::transmute_copy(&rule)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRules_Impl::Remove(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwRules {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwService, INetFwService_Vtbl, 0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwService {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwService, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwService {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<NET_FW_SERVICE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Customized(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Customized)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok() }
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok() }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SERVICE_TYPE) -> windows_core::HRESULT,
    pub Customized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwService_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<NET_FW_SERVICE_TYPE>;
    fn Customized(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwService_Vtbl {
    pub const fn new<Identity: INetFwService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Customized<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customized: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Customized(this) {
                    Ok(ok__) => {
                        customized.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::GloballyOpenPorts(this) {
                    Ok(ok__) => {
                        openports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Customized: Customized::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwService as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwService {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwServiceRestriction, INetFwServiceRestriction_Vtbl, 0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwServiceRestriction {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwServiceRestriction, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwServiceRestriction {
    pub unsafe fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestrictService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(appname), restrictservice, servicesidrestricted).ok() }
    }
    pub unsafe fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceRestricted)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(appname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RestrictService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ServiceRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwServiceRestriction_Impl: super::super::System::Com::IDispatch_Impl {
    fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwServiceRestriction_Vtbl {
    pub const fn new<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RestrictService<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, appname: *mut core::ffi::c_void, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwServiceRestriction_Impl::RestrictService(this, core::mem::transmute(&servicename), core::mem::transmute(&appname), core::mem::transmute_copy(&restrictservice), core::mem::transmute_copy(&servicesidrestricted)).into()
            }
        }
        unsafe extern "system" fn ServiceRestricted<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, appname: *mut core::ffi::c_void, servicerestricted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServiceRestriction_Impl::ServiceRestricted(this, core::mem::transmute(&servicename), core::mem::transmute(&appname)) {
                    Ok(ok__) => {
                        servicerestricted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Rules<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServiceRestriction_Impl::Rules(this) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RestrictService: RestrictService::<Identity, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwServiceRestriction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetFwServices, INetFwServices_Vtbl, 0x79649bb4_903e_421b_94c9_79848e79f6ee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetFwServices {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetFwServices, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetFwServices {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), svctype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct INetFwServices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SERVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwServices_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: INetFwServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::Item(this, core::mem::transmute_copy(&svctype)) {
                    Ok(ok__) => {
                        service.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetFwServices {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingConfiguration, INetSharingConfiguration_Vtbl, 0xc08956b6_1cd3_11d1_b1c5_00805fc1270e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingConfiguration {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingConfiguration, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingConfiguration {
    pub unsafe fn SharingEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SharingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SharingConnectionType(&self) -> windows_core::Result<SHARINGCONNECTIONTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SharingConnectionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DisableSharing(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableSharing)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableSharing)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn InternetFirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternetFirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DisableInternetFirewall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableInternetFirewall)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableInternetFirewall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableInternetFirewall)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPortMappingCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumPortMappings)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddPortMapping(&self, bstrname: &windows_core::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &windows_core::BSTR, etargettype: ICS_TARGETTYPE) -> windows_core::Result<INetSharingPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddPortMapping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), ucipprotocol, usexternalport, usinternalport, dwoptions, core::mem::transmute_copy(bstrtargetnameoripaddress), etargettype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemovePortMapping<P0>(&self, pmapping: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetSharingPortMapping>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemovePortMapping)(windows_core::Interface::as_raw(self), pmapping.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfiguration_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SharingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SharingConnectionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SHARINGCONNECTIONTYPE) -> windows_core::HRESULT,
    pub DisableSharing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableSharing: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTIONTYPE) -> windows_core::HRESULT,
    pub InternetFirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableInternetFirewall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableInternetFirewall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EnumPortMappings: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPortMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u8, u16, u16, u32, *mut core::ffi::c_void, ICS_TARGETTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePortMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingConfiguration_Impl: super::super::System::Com::IDispatch_Impl {
    fn SharingEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SharingConnectionType(&self) -> windows_core::Result<SHARINGCONNECTIONTYPE>;
    fn DisableSharing(&self) -> windows_core::Result<()>;
    fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> windows_core::Result<()>;
    fn InternetFirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisableInternetFirewall(&self) -> windows_core::Result<()>;
    fn EnableInternetFirewall(&self) -> windows_core::Result<()>;
    fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPortMappingCollection>;
    fn AddPortMapping(&self, bstrname: &windows_core::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &windows_core::BSTR, etargettype: ICS_TARGETTYPE) -> windows_core::Result<INetSharingPortMapping>;
    fn RemovePortMapping(&self, pmapping: windows_core::Ref<INetSharingPortMapping>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingConfiguration_Vtbl {
    pub const fn new<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SharingEnabled<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingConfiguration_Impl::SharingEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SharingConnectionType<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingConfiguration_Impl::SharingConnectionType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisableSharing<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingConfiguration_Impl::DisableSharing(this).into()
            }
        }
        unsafe extern "system" fn EnableSharing<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingConfiguration_Impl::EnableSharing(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn InternetFirewallEnabled<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingConfiguration_Impl::InternetFirewallEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingConfiguration_Impl::DisableInternetFirewall(this).into()
            }
        }
        unsafe extern "system" fn EnableInternetFirewall<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingConfiguration_Impl::EnableInternetFirewall(this).into()
            }
        }
        unsafe extern "system" fn get_EnumPortMappings<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingConfiguration_Impl::get_EnumPortMappings(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppcoll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPortMapping<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: *mut core::ffi::c_void, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingConfiguration_Impl::AddPortMapping(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&ucipprotocol), core::mem::transmute_copy(&usexternalport), core::mem::transmute_copy(&usinternalport), core::mem::transmute_copy(&dwoptions), core::mem::transmute(&bstrtargetnameoripaddress), core::mem::transmute_copy(&etargettype)) {
                    Ok(ok__) => {
                        ppmapping.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePortMapping<Identity: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmapping: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingConfiguration_Impl::RemovePortMapping(this, core::mem::transmute_copy(&pmapping)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SharingEnabled: SharingEnabled::<Identity, OFFSET>,
            SharingConnectionType: SharingConnectionType::<Identity, OFFSET>,
            DisableSharing: DisableSharing::<Identity, OFFSET>,
            EnableSharing: EnableSharing::<Identity, OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Identity, OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Identity, OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Identity, OFFSET>,
            get_EnumPortMappings: get_EnumPortMappings::<Identity, OFFSET>,
            AddPortMapping: AddPortMapping::<Identity, OFFSET>,
            RemovePortMapping: RemovePortMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingConfiguration as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingConfiguration {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingEveryConnectionCollection, INetSharingEveryConnectionCollection_Vtbl, 0x33c4643c_7811_46fa_a89a_768597bd7223);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingEveryConnectionCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingEveryConnectionCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingEveryConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingEveryConnectionCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingEveryConnectionCollection_Vtbl {
    pub const fn new<Identity: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingEveryConnectionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingEveryConnectionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingEveryConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingEveryConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingManager, INetSharingManager_Vtbl, 0xc08956b7_1cd3_11d1_b1c5_00805fc1270e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingManager {
    pub unsafe fn SharingInstalled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SharingInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPublicConnectionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumPublicConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPrivateConnectionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumPrivateConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_INetSharingConfigurationForINetConnection<P0>(&self, pnetconnection: P0) -> windows_core::Result<INetSharingConfiguration>
    where
        P0: windows_core::Param<INetConnection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_INetSharingConfigurationForINetConnection)(windows_core::Interface::as_raw(self), pnetconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumEveryConnection(&self) -> windows_core::Result<INetSharingEveryConnectionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumEveryConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_NetConnectionProps<P0>(&self, pnetconnection: P0) -> windows_core::Result<INetConnectionProps>
    where
        P0: windows_core::Param<INetConnection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NetConnectionProps)(windows_core::Interface::as_raw(self), pnetconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SharingInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_EnumPublicConnections: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EnumPrivateConnections: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_INetSharingConfigurationForINetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEveryConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_NetConnectionProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn SharingInstalled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPublicConnectionCollection>;
    fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPrivateConnectionCollection>;
    fn get_INetSharingConfigurationForINetConnection(&self, pnetconnection: windows_core::Ref<INetConnection>) -> windows_core::Result<INetSharingConfiguration>;
    fn EnumEveryConnection(&self) -> windows_core::Result<INetSharingEveryConnectionCollection>;
    fn get_NetConnectionProps(&self, pnetconnection: windows_core::Ref<INetConnection>) -> windows_core::Result<INetConnectionProps>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingManager_Vtbl {
    pub const fn new<Identity: INetSharingManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SharingInstalled<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinstalled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::SharingInstalled(this) {
                    Ok(ok__) => {
                        pbinstalled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EnumPublicConnections<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::get_EnumPublicConnections(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppcoll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EnumPrivateConnections<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::get_EnumPrivateConnections(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppcoll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_INetSharingConfigurationForINetConnection<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppnetsharingconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::get_INetSharingConfigurationForINetConnection(this, core::mem::transmute_copy(&pnetconnection)) {
                    Ok(ok__) => {
                        ppnetsharingconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::EnumEveryConnection(this) {
                    Ok(ok__) => {
                        ppcoll.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NetConnectionProps<Identity: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingManager_Impl::get_NetConnectionProps(this, core::mem::transmute_copy(&pnetconnection)) {
                    Ok(ok__) => {
                        ppprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SharingInstalled: SharingInstalled::<Identity, OFFSET>,
            get_EnumPublicConnections: get_EnumPublicConnections::<Identity, OFFSET>,
            get_EnumPrivateConnections: get_EnumPrivateConnections::<Identity, OFFSET>,
            get_INetSharingConfigurationForINetConnection: get_INetSharingConfigurationForINetConnection::<Identity, OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Identity, OFFSET>,
            get_NetConnectionProps: get_NetConnectionProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingPortMapping, INetSharingPortMapping_Vtbl, 0xc08956b1_1cd3_11d1_b1c5_00805fc1270e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingPortMapping {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingPortMapping, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMapping {
    pub unsafe fn Disable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<INetSharingPortMappingProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMapping_Impl: super::super::System::Com::IDispatch_Impl {
    fn Disable(&self) -> windows_core::Result<()>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<INetSharingPortMappingProps>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingPortMapping_Vtbl {
    pub const fn new<Identity: INetSharingPortMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Disable<Identity: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingPortMapping_Impl::Disable(this).into()
            }
        }
        unsafe extern "system" fn Enable<Identity: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingPortMapping_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnspmp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMapping_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppnspmp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetSharingPortMapping_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Disable: Disable::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingPortMapping {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingPortMappingCollection, INetSharingPortMappingCollection_Vtbl, 0x02e4a2de_da20_4e34_89c8_ac22275a010b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingPortMappingCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingPortMappingCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMappingCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingPortMappingCollection_Vtbl {
    pub const fn new<Identity: INetSharingPortMappingCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingPortMappingCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingPortMappingProps, INetSharingPortMappingProps_Vtbl, 0x24b7e9b5_e38f_4685_851b_00892cf5f940);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingPortMappingProps {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingPortMappingProps, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingProps {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IPProtocol(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IPProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Options(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Options)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TargetIPAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetIPAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IPProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMappingProps_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IPProtocol(&self) -> windows_core::Result<u8>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn Options(&self) -> windows_core::Result<i32>;
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TargetIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingPortMappingProps_Vtbl {
    pub const fn new<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IPProtocol<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucipprot: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::IPProtocol(this) {
                    Ok(ok__) => {
                        pucipprot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::ExternalPort(this) {
                    Ok(ok__) => {
                        pusport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InternalPort<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::InternalPort(this) {
                    Ok(ok__) => {
                        pusport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Options<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::Options(this) {
                    Ok(ok__) => {
                        pdwoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetName<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::TargetName(this) {
                    Ok(ok__) => {
                        pbstrtargetname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetIPAddress<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetipaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::TargetIPAddress(this) {
                    Ok(ok__) => {
                        pbstrtargetipaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPortMappingProps_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pbool.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            IPProtocol: IPProtocol::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            Options: Options::<Identity, OFFSET>,
            TargetName: TargetName::<Identity, OFFSET>,
            TargetIPAddress: TargetIPAddress::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMappingProps as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingPortMappingProps {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingPrivateConnectionCollection, INetSharingPrivateConnectionCollection_Vtbl, 0x38ae69e0_4409_402a_a2cb_e965c727f840);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingPrivateConnectionCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingPrivateConnectionCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPrivateConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPrivateConnectionCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingPrivateConnectionCollection_Vtbl {
    pub const fn new<Identity: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPrivateConnectionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPrivateConnectionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPrivateConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingPrivateConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetSharingPublicConnectionCollection, INetSharingPublicConnectionCollection_Vtbl, 0x7d7a6355_f372_4971_a149_bfc927be762a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetSharingPublicConnectionCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetSharingPublicConnectionCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPublicConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPublicConnectionCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetSharingPublicConnectionCollection_Vtbl {
    pub const fn new<Identity: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPublicConnectionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetSharingPublicConnectionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPublicConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetSharingPublicConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStaticPortMapping, IStaticPortMapping_Vtbl, 0x6f10711f_729b_41e5_93b8_f21d0f818df1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStaticPortMapping {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStaticPortMapping, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalIPAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InternalClient)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditInternalClient)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinternalclient)).ok() }
    }
    pub unsafe fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), vb).ok() }
    }
    pub unsafe fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdescription)).ok() }
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditInternalPort)(windows_core::Interface::as_raw(self), linternalport).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStaticPortMapping_Impl: super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn Protocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IStaticPortMapping_Vtbl {
    pub const fn new<Identity: IStaticPortMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExternalIPAddress<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::ExternalIPAddress(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::ExternalPort(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InternalPort<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::InternalPort(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Protocol<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::Protocol(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InternalClient<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::InternalClient(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMapping_Impl::Description(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStaticPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
            }
        }
        unsafe extern "system" fn Enable<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStaticPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
            }
        }
        unsafe extern "system" fn EditDescription<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStaticPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
            }
        }
        unsafe extern "system" fn EditInternalPort<Identity: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStaticPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            InternalClient: InternalClient::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            EditDescription: EditDescription::<Identity, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStaticPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IStaticPortMapping {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStaticPortMappingCollection, IStaticPortMappingCollection_Vtbl, 0xcd1f3e77_66d6_4664_82c7_36dbb641d0f1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStaticPortMappingCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStaticPortMappingCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lexternalport, core::mem::transmute_copy(bstrprotocol), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lexternalport, core::mem::transmute_copy(bstrprotocol)).ok() }
    }
    pub unsafe fn Add(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), lexternalport, core::mem::transmute_copy(bstrprotocol), linternalport, core::mem::transmute_copy(bstrinternalclient), benabled, core::mem::transmute_copy(bstrdescription), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStaticPortMappingCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IStaticPortMappingCollection_Vtbl {
    pub const fn new<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMappingCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMappingCollection_Impl::get_Item(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        ppspm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMappingCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStaticPortMappingCollection_Impl::Remove(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: *mut core::ffi::c_void, linternalport: i32, bstrinternalclient: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: *mut core::ffi::c_void, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticPortMappingCollection_Impl::Add(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription)) {
                    Ok(ok__) => {
                        ppspm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStaticPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IStaticPortMappingCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUPnPNAT, IUPnPNAT_Vtbl, 0xb171c812_cc76_485a_94d8_b6b3a2794e99);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUPnPNAT {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUPnPNAT, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPNAT {
    pub unsafe fn StaticPortMappingCollection(&self) -> windows_core::Result<IStaticPortMappingCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StaticPortMappingCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DynamicPortMappingCollection(&self) -> windows_core::Result<IDynamicPortMappingCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DynamicPortMappingCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NATEventManager(&self) -> windows_core::Result<INATEventManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NATEventManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNAT_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StaticPortMappingCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DynamicPortMappingCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NATEventManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPNAT_Impl: super::super::System::Com::IDispatch_Impl {
    fn StaticPortMappingCollection(&self) -> windows_core::Result<IStaticPortMappingCollection>;
    fn DynamicPortMappingCollection(&self) -> windows_core::Result<IDynamicPortMappingCollection>;
    fn NATEventManager(&self) -> windows_core::Result<INATEventManager>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUPnPNAT_Vtbl {
    pub const fn new<Identity: IUPnPNAT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StaticPortMappingCollection<Identity: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPNAT_Impl::StaticPortMappingCollection(this) {
                    Ok(ok__) => {
                        ppspms.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Identity: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdpms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPNAT_Impl::DynamicPortMappingCollection(this) {
                    Ok(ok__) => {
                        ppdpms.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NATEventManager<Identity: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUPnPNAT_Impl::NATEventManager(this) {
                    Ok(ok__) => {
                        ppnem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StaticPortMappingCollection: StaticPortMappingCollection::<Identity, OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Identity, OFFSET>,
            NATEventManager: NATEventManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPNAT as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUPnPNAT {}
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2i32);
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4i32);
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8i32);
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1i32);
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(983040i32);
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(128i32);
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(512i32);
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2048i32);
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1024i32);
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4096i32);
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(65536i32);
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32i32);
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(15728640i32);
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(0i32);
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(64i32);
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(16384i32);
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32768i32);
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(256i32);
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8192i32);
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(131072i32);
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(262144i32);
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(0i32);
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(1i32);
pub const NCM_BRIDGE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(7i32);
pub const NCM_DIRECT: NETCON_MEDIATYPE = NETCON_MEDIATYPE(1i32);
pub const NCM_ISDN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(2i32);
pub const NCM_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(3i32);
pub const NCM_NONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(0i32);
pub const NCM_PHONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(4i32);
pub const NCM_PPPOE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(6i32);
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(8i32);
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = NETCON_MEDIATYPE(9i32);
pub const NCM_TUNNEL: NETCON_MEDIATYPE = NETCON_MEDIATYPE(5i32);
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = NETCON_STATUS(13i32);
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = NETCON_STATUS(14i32);
pub const NCS_AUTHENTICATING: NETCON_STATUS = NETCON_STATUS(8i32);
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = NETCON_STATUS(10i32);
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = NETCON_STATUS(9i32);
pub const NCS_CONNECTED: NETCON_STATUS = NETCON_STATUS(2i32);
pub const NCS_CONNECTING: NETCON_STATUS = NETCON_STATUS(1i32);
pub const NCS_CONNECT_FAILED: NETCON_STATUS = NETCON_STATUS(15i32);
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = NETCON_STATUS(12i32);
pub const NCS_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(0i32);
pub const NCS_DISCONNECTING: NETCON_STATUS = NETCON_STATUS(3i32);
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = NETCON_STATUS(5i32);
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = NETCON_STATUS(6i32);
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = NETCON_STATUS(4i32);
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = NETCON_STATUS(11i32);
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(7i32);
pub const NCT_BRIDGE: NETCON_TYPE = NETCON_TYPE(6i32);
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = NETCON_TYPE(0i32);
pub const NCT_INBOUND: NETCON_TYPE = NETCON_TYPE(1i32);
pub const NCT_INTERNET: NETCON_TYPE = NETCON_TYPE(2i32);
pub const NCT_LAN: NETCON_TYPE = NETCON_TYPE(3i32);
pub const NCT_PHONE: NETCON_TYPE = NETCON_TYPE(4i32);
pub const NCT_TUNNEL: NETCON_TYPE = NETCON_TYPE(5i32);
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(0i32);
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(2i32);
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCON_CHARACTERISTIC_FLAGS(pub i32);
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCON_MEDIATYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETCON_PROPERTIES {
    pub guidId: windows_core::GUID,
    pub pszwName: windows_core::PWSTR,
    pub pszwDeviceName: windows_core::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: windows_core::GUID,
    pub clsidUiObject: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCON_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETCON_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETISO_ERROR_TYPE(pub i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETISO_FLAG(pub i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_ACTION(pub i32);
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_IP_VERSION(pub i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_MODIFY_STATE(pub i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_POLICY_TYPE(pub i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_SCOPE(pub i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const NetFwAuthorizedApplication: windows_core::GUID = windows_core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: windows_core::GUID = windows_core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: windows_core::GUID = windows_core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: windows_core::GUID = windows_core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: windows_core::GUID = windows_core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: windows_core::GUID = windows_core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: windows_core::GUID = windows_core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: windows_core::GUID = windows_core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
#[cfg(feature = "Win32_Security")]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, updatedaddresses: windows_core::PCWSTR, append: windows_core::BOOL) -> u32>;
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_core::PCWSTR, dwerr: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
pub const S_OBJECT_NO_LONGER_VALID: windows_core::HRESULT = windows_core::HRESULT(0x2_u32 as _);
pub const UPnPNAT: windows_core::GUID = windows_core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
