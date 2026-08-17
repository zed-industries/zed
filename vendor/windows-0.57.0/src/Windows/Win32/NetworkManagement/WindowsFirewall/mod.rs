#[inline]
pub unsafe fn NcFreeNetconProperties(pprops: *mut NETCON_PROPERTIES) {
    windows_targets::link!("netshell.dll" "system" fn NcFreeNetconProperties(pprops : *mut NETCON_PROPERTIES));
    NcFreeNetconProperties(pprops)
}
#[inline]
pub unsafe fn NcIsValidConnectionName<P0>(pszwname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netshell.dll" "system" fn NcIsValidConnectionName(pszwname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    NcIsValidConnectionName(pszwname.param().abi())
}
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<P0>(wszservername: P0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
    NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.param().abi(), netisoerror)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32);
    NetworkIsolationEnumAppContainers(flags, pdwnumpublicappcs, pppublicappcs)
}
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn NetworkIsolationEnumerateAppContainerRules() -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
    windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationEnumerateAppContainerRules(newenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    NetworkIsolationEnumerateAppContainerRules(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
    NetworkIsolationFreeAppContainers(ppublicappcs)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut *mut super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs, appcontainersids)
}
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdAsync<P0>(wszservername: P0, dwflags: u32, context: Option<*const core::ffi::c_void>, callback: PNETISO_EDP_ID_CALLBACK_FN, hoperation: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdAsync(wszservername : windows_core::PCWSTR, dwflags : u32, context : *const core::ffi::c_void, callback : PNETISO_EDP_ID_CALLBACK_FN, hoperation : *mut super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationGetEnterpriseIdAsync(wszservername.param().abi(), dwflags, core::mem::transmute(context.unwrap_or(std::ptr::null())), callback, hoperation)
}
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdClose<P0, P1>(hoperation: P0, bwaitforoperation: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdClose(hoperation : super::super::Foundation:: HANDLE, bwaitforoperation : super::super::Foundation:: BOOL) -> u32);
    NetworkIsolationGetEnterpriseIdClose(hoperation.param().abi(), bwaitforoperation.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: Option<*const core::ffi::c_void>, registrationobject: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const core::ffi::c_void, registrationobject : *mut super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationRegisterForAppContainerChanges(flags, callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), registrationobject)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[super::super::Security::SID_AND_ATTRIBUTES]) -> u32 {
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    NetworkIsolationSetAppContainerConfig(appcontainersids.len().try_into().unwrap(), core::mem::transmute(appcontainersids.as_ptr()))
}
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<P0, P1, P2, P3, P4>(applicationcontainersid: P0, packagefullname: P1, packagefolder: P2, displayname: P3, bbinariesfullycomputed: P4, binaries: &[windows_core::PCWSTR]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::PSID>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::super::Foundation:: PSID, packagefullname : windows_core::PCWSTR, packagefolder : windows_core::PCWSTR, displayname : windows_core::PCWSTR, bbinariesfullycomputed : super::super::Foundation:: BOOL, binaries : *const windows_core::PCWSTR, binariescount : u32) -> windows_core::HRESULT);
    NetworkIsolationSetupAppContainerBinaries(applicationcontainersid.param().abi(), packagefullname.param().abi(), packagefolder.param().abi(), displayname.param().abi(), bbinariesfullycomputed.param().abi(), core::mem::transmute(binaries.as_ptr()), binaries.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges<P0>(registrationobject: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationUnregisterForAppContainerChanges(registrationobject.param().abi())
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExternalIPAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoteHost(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteHost)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternalClient)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LeaseDuration(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LeaseDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RenewLease)(windows_core::Interface::as_raw(self), lleasedurationdesired, &mut result__).map(|| result__)
    }
    pub unsafe fn EditInternalClient<P0>(&self, bstrinternalclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EditInternalClient)(windows_core::Interface::as_raw(self), bstrinternalclient.param().abi()).ok()
    }
    pub unsafe fn Enable<P0>(&self, vb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), vb.param().abi()).ok()
    }
    pub unsafe fn EditDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EditDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EditInternalPort)(windows_core::Interface::as_raw(self), linternalport).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDynamicPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoteHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LeaseDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RenewLease: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0, P1>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1) -> windows_core::Result<IDynamicPortMapping>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), bstrremotehost.param().abi(), lexternalport, bstrprotocol.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Remove<P0, P1>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), bstrremotehost.param().abi(), lexternalport, bstrprotocol.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2, P3, P4>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1, linternalport: i32, bstrinternalclient: P2, benabled: P3, bstrdescription: P4, lleaseduration: i32) -> windows_core::Result<IDynamicPortMapping>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), bstrremotehost.param().abi(), lexternalport, bstrprotocol.param().abi(), linternalport, bstrinternalclient.param().abi(), benabled.param().abi(), bstrdescription.param().abi(), lleaseduration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDynamicPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
}
windows_core::imp::define_interface!(IEnumNetConnection, IEnumNetConnection_Vtbl, 0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for IEnumNetConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetConnection, windows_core::IUnknown);
impl IEnumNetConnection {
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetConnection>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNetConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetSharingEveryConnection, IEnumNetSharingEveryConnection_Vtbl, 0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for IEnumNetSharingEveryConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetSharingEveryConnection, windows_core::IUnknown);
impl IEnumNetSharingEveryConnection {
    pub unsafe fn Next(&self, rgvar: &mut [windows_core::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingEveryConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNetSharingEveryConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetSharingPortMapping, IEnumNetSharingPortMapping_Vtbl, 0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for IEnumNetSharingPortMapping {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetSharingPortMapping, windows_core::IUnknown);
impl IEnumNetSharingPortMapping {
    pub unsafe fn Next(&self, rgvar: &mut [windows_core::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPortMapping> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNetSharingPortMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetSharingPrivateConnection, IEnumNetSharingPrivateConnection_Vtbl, 0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for IEnumNetSharingPrivateConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetSharingPrivateConnection, windows_core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    pub unsafe fn Next(&self, rgvar: &mut [windows_core::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPrivateConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNetSharingPrivateConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetSharingPublicConnection, IEnumNetSharingPublicConnection_Vtbl, 0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for IEnumNetSharingPublicConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetSharingPublicConnection, windows_core::IUnknown);
impl IEnumNetSharingPublicConnection {
    pub unsafe fn Next(&self, rgvar: &mut [windows_core::VARIANT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetSharingPublicConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumNetSharingPublicConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetExternalIPAddressCallback)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn SetNumberOfEntriesCallback<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetNumberOfEntriesCallback)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INATEventManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetExternalIPAddressCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumberOfEntriesCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INATExternalIPAddressCallback, INATExternalIPAddressCallback_Vtbl, 0x9c416740_a34e_446f_ba06_abd04c3149ae);
impl core::ops::Deref for INATExternalIPAddressCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INATExternalIPAddressCallback, windows_core::IUnknown);
impl INATExternalIPAddressCallback {
    pub unsafe fn NewExternalIPAddress<P0>(&self, bstrnewexternalipaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NewExternalIPAddress)(windows_core::Interface::as_raw(self), bstrnewexternalipaddress.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INATExternalIPAddressCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INATNumberOfEntriesCallback, INATNumberOfEntriesCallback_Vtbl, 0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
impl core::ops::Deref for INATNumberOfEntriesCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INATNumberOfEntriesCallback, windows_core::IUnknown);
impl INATNumberOfEntriesCallback {
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NewNumberOfEntries)(windows_core::Interface::as_raw(self), lnewnumberofentries).ok()
    }
}
#[repr(C)]
pub struct INATNumberOfEntriesCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewNumberOfEntries: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetConnection, INetConnection_Vtbl, 0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for INetConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetConnection, windows_core::IUnknown);
impl INetConnection {
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Duplicate<P0>(&self, pszwduplicatename: P0) -> windows_core::Result<INetConnection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Duplicate)(windows_core::Interface::as_raw(self), pszwduplicatename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<*mut NETCON_PROPERTIES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUiObjectClassId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUiObjectClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Rename<P0>(&self, pszwnewname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), pszwnewname.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(INetConnectionConnectUi, INetConnectionConnectUi_Vtbl, 0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for INetConnectionConnectUi {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetConnectionConnectUi, windows_core::IUnknown);
impl INetConnectionConnectUi {
    pub unsafe fn SetConnection<P0>(&self, pcon: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetConnection>,
    {
        (windows_core::Interface::vtable(self).SetConnection)(windows_core::Interface::as_raw(self), pcon.param().abi()).ok()
    }
    pub unsafe fn Connect<P0>(&self, hwndparent: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), dwflags).ok()
    }
    pub unsafe fn Disconnect<P0>(&self, hwndparent: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct INetConnectionConnectUi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetConnectionManager, INetConnectionManager_Vtbl, 0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for INetConnectionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetConnectionManager, windows_core::IUnknown);
impl INetConnectionManager {
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> windows_core::Result<IEnumNetConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct INetConnectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, NETCONMGR_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Status(&self) -> windows_core::Result<NETCON_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaType(&self) -> windows_core::Result<NETCON_MEDIATYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Characteristics(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Characteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetConnectionProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NETCON_STATUS) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NETCON_MEDIATYPE) -> windows_core::HRESULT,
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessImageFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProcessImageFileName<P0>(&self, imagefilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProcessImageFileName)(windows_core::Interface::as_raw(self), imagefilename.param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), remoteaddrs.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, app: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwAuthorizedApplication>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), app.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, imagefilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), imagefilename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, imagefilename: P0) -> windows_core::Result<INetFwAuthorizedApplication>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), imagefilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowOutboundDestinationUnreachable<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowRedirect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowRedirect<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowRedirect)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInboundEchoRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInboundEchoRequest<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInboundEchoRequest)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowOutboundTimeExceeded<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowOutboundParameterProblem<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowOutboundSourceQuench<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInboundRouterRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInboundRouterRequest<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInboundRouterRequest)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInboundTimestampRequest<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInboundMaskRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInboundMaskRequest<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInboundMaskRequest)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowOutboundPacketTooBig<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentProfileType(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProfileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RestoreDefaults(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreDefaults)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsPortAllowed<P0, P1>(&self, imagefilename: P0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: P1, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut windows_core::VARIANT, restricted: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).IsPortAllowed)(windows_core::Interface::as_raw(self), imagefilename.param().abi(), ipversion, portnumber, localaddress.param().abi(), ipprotocol, core::mem::transmute(allowed), core::mem::transmute(restricted)).ok()
    }
    pub unsafe fn IsIcmpTypeAllowed<P0>(&self, ipversion: NET_FW_IP_VERSION, localaddress: P0, r#type: u8, allowed: *mut windows_core::VARIANT, restricted: *mut windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(windows_core::Interface::as_raw(self), ipversion, localaddress.param().abi(), r#type, core::mem::transmute(allowed), core::mem::transmute(restricted)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwMgr_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LocalPolicy: usize,
    pub CurrentProfileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    pub RestoreDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPortAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, NET_FW_IP_VERSION, i32, core::mem::MaybeUninit<windows_core::BSTR>, NET_FW_IP_PROTOCOL, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION, core::mem::MaybeUninit<windows_core::BSTR>, u8, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<NET_FW_IP_PROTOCOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), ipprotocol).ok()
    }
    pub unsafe fn Port(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), portnumber).ok()
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), remoteaddrs.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn BuiltIn(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BuiltIn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BuiltIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, port: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwOpenPort>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), port.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), portnumber, ipprotocol).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), portnumber, ipprotocol, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwOpenPorts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, NET_FW_IP_PROTOCOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProfile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProfileByType)(windows_core::Interface::as_raw(self), profiletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwPolicy_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProfileByType: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProfileByType: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentProfileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_FirewallEnabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, enabled.param().abi()).ok()
    }
    pub unsafe fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_ExcludedInterfaces<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).put_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, interfaces.param().abi()).ok()
    }
    pub unsafe fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_BlockAllInboundTraffic<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, block: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, block.param().abi()).ok()
    }
    pub unsafe fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_NotificationsDisabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled.param().abi()).ok()
    }
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceRestriction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnableRuleGroup<P0, P1>(&self, profiletypesbitmask: i32, group: P0, enable: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableRuleGroup)(windows_core::Interface::as_raw(self), profiletypesbitmask, group.param().abi(), enable.param().abi()).ok()
    }
    pub unsafe fn IsRuleGroupEnabled<P0>(&self, profiletypesbitmask: i32, group: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRuleGroupEnabled)(windows_core::Interface::as_raw(self), profiletypesbitmask, group.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreLocalFirewallDefaults)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, action).ok()
    }
    pub unsafe fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
    }
    pub unsafe fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, action).ok()
    }
    pub unsafe fn get_IsRuleGroupCurrentlyEnabled<P0>(&self, group: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_IsRuleGroupCurrentlyEnabled)(windows_core::Interface::as_raw(self), group.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalPolicyModifyState(&self) -> windows_core::Result<NET_FW_MODIFY_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalPolicyModifyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentProfileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceRestriction: usize,
    pub EnableRuleGroup: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsRuleGroupEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub put_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, NET_FW_ACTION) -> windows_core::HRESULT,
    pub get_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub put_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_PROFILE_TYPE2, NET_FW_ACTION) -> windows_core::HRESULT,
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LocalPolicyModifyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_MODIFY_STATE) -> windows_core::HRESULT,
}
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
    pub unsafe fn RuleCategories(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RuleCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRuleCategories<P0>(&self, rulecategories: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetRuleCategories)(windows_core::Interface::as_raw(self), rulecategories.param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, displayname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), displayname.param().abi()).ok()
    }
    pub unsafe fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathToSignedProductExe)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwProduct_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetRuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathToSignedProductExe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Register<P0>(&self, product: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<INetFwProduct>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), product.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwProducts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Register: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFirewallEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFirewallEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExceptionsNotAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExceptionsNotAllowed<P0>(&self, notallowed: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetExceptionsNotAllowed)(windows_core::Interface::as_raw(self), notallowed.param().abi()).ok()
    }
    pub unsafe fn NotificationsDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotificationsDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotificationsDisabled<P0>(&self, disabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNotificationsDisabled)(windows_core::Interface::as_raw(self), disabled.param().abi()).ok()
    }
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled<P0>(&self, disabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), disabled.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAdminSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IcmpSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> windows_core::Result<INetFwServices> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthorizedApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub RemoteAdminSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoteAdminSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IcmpSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IcmpSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AuthorizedApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AuthorizedApplications: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), remoteaddrs.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, desc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), desc.param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetApplicationName<P0>(&self, imagefilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetApplicationName)(windows_core::Interface::as_raw(self), imagefilename.param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceName<P0>(&self, servicename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceName)(windows_core::Interface::as_raw(self), servicename.param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalPorts<P0>(&self, portnumbers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalPorts)(windows_core::Interface::as_raw(self), portnumbers.param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemotePorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemotePorts<P0>(&self, portnumbers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemotePorts)(windows_core::Interface::as_raw(self), portnumbers.param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalAddresses<P0>(&self, localaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalAddresses)(windows_core::Interface::as_raw(self), localaddrs.param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), remoteaddrs.param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IcmpTypesAndCodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIcmpTypesAndCodes<P0>(&self, icmptypesandcodes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetIcmpTypesAndCodes)(windows_core::Interface::as_raw(self), icmptypesandcodes.param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDirection)(windows_core::Interface::as_raw(self), dir).ok()
    }
    pub unsafe fn Interfaces(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Interfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInterfaces<P0>(&self, interfaces: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetInterfaces)(windows_core::Interface::as_raw(self), interfaces.param().abi()).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInterfaceTypes<P0>(&self, interfacetypes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterfaceTypes)(windows_core::Interface::as_raw(self), interfacetypes.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn Grouping(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Grouping)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetGrouping<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGrouping)(windows_core::Interface::as_raw(self), context.param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProfiles)(windows_core::Interface::as_raw(self), profiletypesbitmask).ok()
    }
    pub unsafe fn EdgeTraversal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EdgeTraversal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEdgeTraversal<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEdgeTraversal)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn Action(&self) -> windows_core::Result<NET_FW_ACTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAction)(windows_core::Interface::as_raw(self), action).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    pub Interfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub InterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Grouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_ACTION) -> windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_ACTION) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EdgeTraversalOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEdgeTraversalOptions)(windows_core::Interface::as_raw(self), loptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub EdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalAppPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalAppPackageId<P0>(&self, wszpackageid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalAppPackageId)(windows_core::Interface::as_raw(self), wszpackageid.param().abi()).ok()
    }
    pub unsafe fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalUserOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalUserOwner<P0>(&self, wszuserowner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalUserOwner)(windows_core::Interface::as_raw(self), wszuserowner.param().abi()).ok()
    }
    pub unsafe fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalUserAuthorizedList<P0>(&self, wszuserauthlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalUserAuthorizedList)(windows_core::Interface::as_raw(self), wszuserauthlist.param().abi()).ok()
    }
    pub unsafe fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteUserAuthorizedList<P0>(&self, wszuserauthlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), wszuserauthlist.param().abi()).ok()
    }
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteMachineAuthorizedList<P0>(&self, wszuserauthlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), wszuserauthlist.param().abi()).ok()
    }
    pub unsafe fn SecureFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecureFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSecureFlags)(windows_core::Interface::as_raw(self), loptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub LocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, rule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetFwRule>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), rule.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0) -> windows_core::Result<INetFwRule>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<NET_FW_SERVICE_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Customized(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Customized)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), remoteaddrs.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwService_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SERVICE_TYPE) -> windows_core::HRESULT,
    pub Customized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_IP_VERSION) -> windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NET_FW_SCOPE) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SCOPE) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
}
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
    pub unsafe fn RestrictService<P0, P1, P2, P3>(&self, servicename: P0, appname: P1, restrictservice: P2, servicesidrestricted: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).RestrictService)(windows_core::Interface::as_raw(self), servicename.param().abi(), appname.param().abi(), restrictservice.param().abi(), servicesidrestricted.param().abi()).ok()
    }
    pub unsafe fn ServiceRestricted<P0, P1>(&self, servicename: P0, appname: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceRestricted)(windows_core::Interface::as_raw(self), servicename.param().abi(), appname.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwServiceRestriction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RestrictService: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ServiceRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), svctype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetFwServices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, NET_FW_SERVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SharingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SharingConnectionType(&self) -> windows_core::Result<SHARINGCONNECTIONTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SharingConnectionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DisableSharing(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableSharing)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableSharing)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn InternetFirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternetFirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DisableInternetFirewall(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableInternetFirewall)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableInternetFirewall(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableInternetFirewall)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPortMappingCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EnumPortMappings)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPortMapping<P0, P1>(&self, bstrname: P0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: P1, etargettype: ICS_TARGETTYPE) -> windows_core::Result<INetSharingPortMapping>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddPortMapping)(windows_core::Interface::as_raw(self), bstrname.param().abi(), ucipprotocol, usexternalport, usinternalport, dwoptions, bstrtargetnameoripaddress.param().abi(), etargettype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemovePortMapping<P0>(&self, pmapping: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetSharingPortMapping>,
    {
        (windows_core::Interface::vtable(self).RemovePortMapping)(windows_core::Interface::as_raw(self), pmapping.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingConfiguration_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SharingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SharingConnectionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SHARINGCONNECTIONTYPE) -> windows_core::HRESULT,
    pub DisableSharing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableSharing: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTIONTYPE) -> windows_core::HRESULT,
    pub InternetFirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisableInternetFirewall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableInternetFirewall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPortMappings: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPortMappings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPortMapping: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u8, u16, u16, u32, core::mem::MaybeUninit<windows_core::BSTR>, ICS_TARGETTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPortMapping: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemovePortMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemovePortMapping: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingEveryConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SharingInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPublicConnectionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EnumPublicConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPrivateConnectionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EnumPrivateConnections)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_INetSharingConfigurationForINetConnection<P0>(&self, pnetconnection: P0) -> windows_core::Result<INetSharingConfiguration>
    where
        P0: windows_core::Param<INetConnection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_INetSharingConfigurationForINetConnection)(windows_core::Interface::as_raw(self), pnetconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumEveryConnection(&self) -> windows_core::Result<INetSharingEveryConnectionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumEveryConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_NetConnectionProps<P0>(&self, pnetconnection: P0) -> windows_core::Result<INetConnectionProps>
    where
        P0: windows_core::Param<INetConnection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_NetConnectionProps)(windows_core::Interface::as_raw(self), pnetconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SharingInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPublicConnections: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPublicConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPrivateConnections: unsafe extern "system" fn(*mut core::ffi::c_void, SHARINGCONNECTION_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPrivateConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_INetSharingConfigurationForINetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_INetSharingConfigurationForINetConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumEveryConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumEveryConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_NetConnectionProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_NetConnectionProps: usize,
}
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
        (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<INetSharingPortMappingProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IPProtocol(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IPProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Options(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Options)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TargetIPAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetIPAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPortMappingProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IPProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TargetIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPrivateConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetSharingPublicConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExternalIPAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InternalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternalClient)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EditInternalClient<P0>(&self, bstrinternalclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EditInternalClient)(windows_core::Interface::as_raw(self), bstrinternalclient.param().abi()).ok()
    }
    pub unsafe fn Enable<P0>(&self, vb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), vb.param().abi()).ok()
    }
    pub unsafe fn EditDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EditDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EditInternalPort)(windows_core::Interface::as_raw(self), linternalport).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStaticPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EditDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, lexternalport: i32, bstrprotocol: P0) -> windows_core::Result<IStaticPortMapping>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Remove<P0>(&self, lexternalport: i32, bstrprotocol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2, P3>(&self, lexternalport: i32, bstrprotocol: P0, linternalport: i32, bstrinternalclient: P1, benabled: P2, bstrdescription: P3) -> windows_core::Result<IStaticPortMapping>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.param().abi(), linternalport, bstrinternalclient.param().abi(), benabled.param().abi(), bstrdescription.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStaticPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StaticPortMappingCollection(&self) -> windows_core::Result<IStaticPortMappingCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StaticPortMappingCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DynamicPortMappingCollection(&self) -> windows_core::Result<IDynamicPortMappingCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DynamicPortMappingCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NATEventManager(&self) -> windows_core::Result<INATEventManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NATEventManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUPnPNAT_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub StaticPortMappingCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StaticPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DynamicPortMappingCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DynamicPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NATEventManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NATEventManager: usize,
}
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
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
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const S_OBJECT_NO_LONGER_VALID: windows_core::HRESULT = windows_core::HRESULT(0x2_u32 as _);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
impl windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
impl windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ORIGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ICS_TARGETTYPE(pub i32);
impl windows_core::TypeKind for ICS_TARGETTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ICS_TARGETTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ICS_TARGETTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for INET_FIREWALL_AC_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INET_FIREWALL_AC_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
impl windows_core::TypeKind for INET_FIREWALL_AC_CREATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INET_FIREWALL_AC_CREATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CREATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for NETCONMGR_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCONMGR_ENUM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCONMGR_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
impl windows_core::TypeKind for NETCONUI_CONNECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCONUI_CONNECT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCONUI_CONNECT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCON_CHARACTERISTIC_FLAGS(pub i32);
impl windows_core::TypeKind for NETCON_CHARACTERISTIC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCON_CHARACTERISTIC_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCON_CHARACTERISTIC_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCON_MEDIATYPE(pub i32);
impl windows_core::TypeKind for NETCON_MEDIATYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCON_MEDIATYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCON_MEDIATYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCON_STATUS(pub i32);
impl windows_core::TypeKind for NETCON_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCON_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCON_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETCON_TYPE(pub i32);
impl windows_core::TypeKind for NETCON_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETCON_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETCON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETISO_ERROR_TYPE(pub i32);
impl windows_core::TypeKind for NETISO_ERROR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETISO_ERROR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETISO_ERROR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETISO_FLAG(pub i32);
impl windows_core::TypeKind for NETISO_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETISO_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETISO_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_ACTION(pub i32);
impl windows_core::TypeKind for NET_FW_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
impl windows_core::TypeKind for NET_FW_AUTHENTICATE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_AUTHENTICATE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_AUTHENTICATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
impl windows_core::TypeKind for NET_FW_EDGE_TRAVERSAL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_EDGE_TRAVERSAL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
impl windows_core::TypeKind for NET_FW_IP_PROTOCOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_IP_PROTOCOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_IP_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_IP_VERSION(pub i32);
impl windows_core::TypeKind for NET_FW_IP_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_IP_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_IP_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_MODIFY_STATE(pub i32);
impl windows_core::TypeKind for NET_FW_MODIFY_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_MODIFY_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_MODIFY_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_POLICY_TYPE(pub i32);
impl windows_core::TypeKind for NET_FW_POLICY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_POLICY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
impl windows_core::TypeKind for NET_FW_PROFILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_PROFILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
impl windows_core::TypeKind for NET_FW_PROFILE_TYPE2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_PROFILE_TYPE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
impl windows_core::TypeKind for NET_FW_RULE_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_RULE_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
impl windows_core::TypeKind for NET_FW_RULE_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_RULE_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_SCOPE(pub i32);
impl windows_core::TypeKind for NET_FW_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
impl windows_core::TypeKind for NET_FW_SERVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_FW_SERVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_FW_SERVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
impl windows_core::TypeKind for SHARINGCONNECTIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHARINGCONNECTIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for SHARINGCONNECTION_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SHARINGCONNECTION_ENUM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTION_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_core::GUID,
    pub keyword: windows_core::PCWSTR,
    pub flags: u32,
    pub addresses: windows_core::PCWSTR,
}
impl windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut windows_core::PWSTR,
}
impl windows_core::TypeKind for INET_FIREWALL_AC_BINARIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for INET_FIREWALL_AC_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for INET_FIREWALL_AC_CHANGE {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for INET_FIREWALL_AC_CHANGE_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for INET_FIREWALL_APP_CONTAINER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for NETCON_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NetFwAuthorizedApplication: windows_core::GUID = windows_core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: windows_core::GUID = windows_core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: windows_core::GUID = windows_core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: windows_core::GUID = windows_core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: windows_core::GUID = windows_core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: windows_core::GUID = windows_core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: windows_core::GUID = windows_core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: windows_core::GUID = windows_core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
pub const UPnPNAT: windows_core::GUID = windows_core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
#[cfg(feature = "Win32_Security")]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, updatedaddresses: windows_core::PCWSTR, append: super::super::Foundation::BOOL) -> u32>;
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_core::PCWSTR, dwerr: u32)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
