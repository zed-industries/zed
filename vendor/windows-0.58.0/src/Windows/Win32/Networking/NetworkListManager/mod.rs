#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumNetworkConnections, IEnumNetworkConnections_Vtbl, 0xdcb00006_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumNetworkConnections {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumNetworkConnections, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworkConnections {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetworkConnection>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetworkConnections> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumNetworkConnections_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEnumNetworks, IEnumNetworks_Vtbl, 0xdcb00003_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEnumNetworks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEnumNetworks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworks {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetwork>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNetworks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetwork, INetwork_Vtbl, 0xdcb00002_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetwork {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetwork, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetwork {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, sznetworknewname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), sznetworknewname.param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), szdescription.param().abi()).ok()
    }
    pub unsafe fn GetNetworkId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDomainType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTimeCreatedAndConnected)(windows_core::Interface::as_raw(self), pdwlowdatetimecreated, pdwhighdatetimecreated, pdwlowdatetimeconnected, pdwhighdatetimeconnected).ok()
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<NLM_NETWORK_CATEGORY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCategory)(windows_core::Interface::as_raw(self), newcategory).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNetworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    pub GetTimeCreatedAndConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_NETWORK_CATEGORY) -> windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_NETWORK_CATEGORY) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetwork2, INetwork2_Vtbl, 0xb5550abb_3391_4310_804f_25dcc325ed81);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetwork2 {
    type Target = INetwork;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetwork2, windows_core::IUnknown, super::super::System::Com::IDispatch, INetwork);
#[cfg(feature = "Win32_System_Com")]
impl INetwork2 {
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDomainAuthenticatedBy)(windows_core::Interface::as_raw(self), domainauthenticationkind, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetwork2_Vtbl {
    pub base__: INetwork_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_DOMAIN_AUTHENTICATION_KIND, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkConnection, INetworkConnection_Vtbl, 0xdcb00005_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkConnection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkConnection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self) -> windows_core::Result<INetwork> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetwork)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectionId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAdapterId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAdapterId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDomainType(&self) -> windows_core::Result<NLM_DOMAIN_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDomainType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkConnection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DOMAIN_TYPE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkConnection2, INetworkConnection2_Vtbl, 0x00e676ed_5a35_4738_92eb_8581738d0f0a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkConnection2 {
    type Target = INetworkConnection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkConnection2, windows_core::IUnknown, super::super::System::Com::IDispatch, INetworkConnection);
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnection2 {
    pub unsafe fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDomainAuthenticatedBy)(windows_core::Interface::as_raw(self), domainauthenticationkind, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkConnection2_Vtbl {
    pub base__: INetworkConnection_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_DOMAIN_AUTHENTICATION_KIND, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkConnectionCost, INetworkConnectionCost_Vtbl, 0xdcb0000a_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkConnectionCost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkConnectionCost, windows_core::IUnknown);
impl INetworkConnectionCost {
    pub unsafe fn GetCost(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataPlanStatus)(windows_core::Interface::as_raw(self), pdataplanstatus).ok()
    }
}
#[repr(C)]
pub struct INetworkConnectionCost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DATAPLAN_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkConnectionCostEvents, INetworkConnectionCostEvents_Vtbl, 0xdcb0000b_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkConnectionCostEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkConnectionCostEvents, windows_core::IUnknown);
impl INetworkConnectionCostEvents {
    pub unsafe fn ConnectionCostChanged(&self, connectionid: windows_core::GUID, newcost: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectionCostChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), newcost).ok()
    }
    pub unsafe fn ConnectionDataPlanStatusChanged(&self, connectionid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectionDataPlanStatusChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid)).ok()
    }
}
#[repr(C)]
pub struct INetworkConnectionCostEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionCostChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub ConnectionDataPlanStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkConnectionEvents, INetworkConnectionEvents_Vtbl, 0xdcb00007_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkConnectionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkConnectionEvents, windows_core::IUnknown);
impl INetworkConnectionEvents {
    pub unsafe fn NetworkConnectionConnectivityChanged(&self, connectionid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkConnectionConnectivityChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), newconnectivity).ok()
    }
    pub unsafe fn NetworkConnectionPropertyChanged(&self, connectionid: windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkConnectionPropertyChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(connectionid), flags).ok()
    }
}
#[repr(C)]
pub struct INetworkConnectionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NetworkConnectionConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub NetworkConnectionPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTION_PROPERTY_CHANGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkCostManager, INetworkCostManager_Vtbl, 0xdcb00008_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkCostManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkCostManager, windows_core::IUnknown);
impl INetworkCostManager {
    pub unsafe fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCost)(windows_core::Interface::as_raw(self), pcost, pdestipaddr).ok()
    }
    pub unsafe fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataPlanStatus)(windows_core::Interface::as_raw(self), pdataplanstatus, pdestipaddr).ok()
    }
    pub unsafe fn SetDestinationAddresses<P0>(&self, pdestipaddrlist: &[NLM_SOCKADDR], bappend: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDestinationAddresses)(windows_core::Interface::as_raw(self), pdestipaddrlist.len().try_into().unwrap(), core::mem::transmute(pdestipaddrlist.as_ptr()), bappend.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetworkCostManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_DATAPLAN_STATUS, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub SetDestinationAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const NLM_SOCKADDR, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkCostManagerEvents, INetworkCostManagerEvents_Vtbl, 0xdcb00009_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkCostManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkCostManagerEvents, windows_core::IUnknown);
impl INetworkCostManagerEvents {
    pub unsafe fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CostChanged)(windows_core::Interface::as_raw(self), newcost, pdestaddr).ok()
    }
    pub unsafe fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DataPlanStatusChanged)(windows_core::Interface::as_raw(self), pdestaddr).ok()
    }
}
#[repr(C)]
pub struct INetworkCostManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CostChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const NLM_SOCKADDR) -> windows_core::HRESULT,
    pub DataPlanStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const NLM_SOCKADDR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkEvents, INetworkEvents_Vtbl, 0xdcb00004_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkEvents, windows_core::IUnknown);
impl INetworkEvents {
    pub unsafe fn NetworkAdded(&self, networkid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkAdded)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid)).ok()
    }
    pub unsafe fn NetworkDeleted(&self, networkid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkDeleted)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid)).ok()
    }
    pub unsafe fn NetworkConnectivityChanged(&self, networkid: windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkConnectivityChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid), newconnectivity).ok()
    }
    pub unsafe fn NetworkPropertyChanged(&self, networkid: windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NetworkPropertyChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(networkid), flags).ok()
    }
}
#[repr(C)]
pub struct INetworkEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NetworkAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub NetworkDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub NetworkConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub NetworkPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, NLM_NETWORK_PROPERTY_CHANGE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkListManager, INetworkListManager_Vtbl, 0xdcb00000_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkListManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkListManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetworkListManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> windows_core::Result<IEnumNetworks> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetwork(&self, gdnetworkid: windows_core::GUID) -> windows_core::Result<INetwork> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetwork)(windows_core::Interface::as_raw(self), core::mem::transmute(gdnetworkid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnections(&self) -> windows_core::Result<IEnumNetworkConnections> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNetworkConnection(&self, gdnetworkconnectionid: windows_core::GUID) -> windows_core::Result<INetworkConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNetworkConnection)(windows_core::Interface::as_raw(self), core::mem::transmute(gdnetworkconnectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsConnectedToInternet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnectedToInternet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectivity(&self) -> windows_core::Result<NLM_CONNECTIVITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSimulatedProfileInfo)(windows_core::Interface::as_raw(self), psimulatedinfo).ok()
    }
    pub unsafe fn ClearSimulatedProfileInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearSimulatedProfileInfo)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkListManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworks: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_ENUM_NETWORK, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetwork: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNetworkConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNetworkConnection: usize,
    pub IsConnectedToInternet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NLM_CONNECTIVITY) -> windows_core::HRESULT,
    pub SetSimulatedProfileInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const NLM_SIMULATED_PROFILE_INFO) -> windows_core::HRESULT,
    pub ClearSimulatedProfileInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkListManagerEvents, INetworkListManagerEvents_Vtbl, 0xdcb00001_570f_4a9b_8d69_199fdba5723b);
impl core::ops::Deref for INetworkListManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetworkListManagerEvents, windows_core::IUnknown);
impl INetworkListManagerEvents {
    pub unsafe fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectivityChanged)(windows_core::Interface::as_raw(self), newconnectivity).ok()
    }
}
#[repr(C)]
pub struct INetworkListManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectivityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, NLM_CONNECTIVITY) -> windows_core::HRESULT,
}
pub const NA_AllowMerge: windows_core::PCWSTR = windows_core::w!("NA_AllowMerge");
pub const NA_CategoryReadOnly: windows_core::PCWSTR = windows_core::w!("NA_CategoryReadOnly");
pub const NA_CategorySetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_CategorySetByPolicy");
pub const NA_DescriptionReadOnly: windows_core::PCWSTR = windows_core::w!("NA_DescriptionReadOnly");
pub const NA_DescriptionSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_DescriptionSetByPolicy");
pub const NA_DomainAuthenticationFailed: windows_core::PCWSTR = windows_core::w!("NA_DomainAuthenticationFailed");
pub const NA_IconReadOnly: windows_core::PCWSTR = windows_core::w!("NA_IconReadOnly");
pub const NA_IconSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_IconSetByPolicy");
pub const NA_InternetConnectivityV4: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV4");
pub const NA_InternetConnectivityV6: windows_core::PCWSTR = windows_core::w!("NA_InternetConnectivityV6");
pub const NA_NameReadOnly: windows_core::PCWSTR = windows_core::w!("NA_NameReadOnly");
pub const NA_NameSetByPolicy: windows_core::PCWSTR = windows_core::w!("NA_NameSetByPolicy");
pub const NA_NetworkClass: windows_core::PCWSTR = windows_core::w!("NA_NetworkClass");
pub const NLM_CONNECTION_COST_APPROACHINGDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(524288i32);
pub const NLM_CONNECTION_COST_CONGESTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(131072i32);
pub const NLM_CONNECTION_COST_FIXED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(2i32);
pub const NLM_CONNECTION_COST_OVERDATALIMIT: NLM_CONNECTION_COST = NLM_CONNECTION_COST(65536i32);
pub const NLM_CONNECTION_COST_ROAMING: NLM_CONNECTION_COST = NLM_CONNECTION_COST(262144i32);
pub const NLM_CONNECTION_COST_UNKNOWN: NLM_CONNECTION_COST = NLM_CONNECTION_COST(0i32);
pub const NLM_CONNECTION_COST_UNRESTRICTED: NLM_CONNECTION_COST = NLM_CONNECTION_COST(1i32);
pub const NLM_CONNECTION_COST_VARIABLE: NLM_CONNECTION_COST = NLM_CONNECTION_COST(4i32);
pub const NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION: NLM_CONNECTION_PROPERTY_CHANGE = NLM_CONNECTION_PROPERTY_CHANGE(1i32);
pub const NLM_CONNECTIVITY_DISCONNECTED: NLM_CONNECTIVITY = NLM_CONNECTIVITY(0i32);
pub const NLM_CONNECTIVITY_IPV4_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(64i32);
pub const NLM_CONNECTIVITY_IPV4_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(32i32);
pub const NLM_CONNECTIVITY_IPV4_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1i32);
pub const NLM_CONNECTIVITY_IPV4_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(16i32);
pub const NLM_CONNECTIVITY_IPV6_INTERNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(1024i32);
pub const NLM_CONNECTIVITY_IPV6_LOCALNETWORK: NLM_CONNECTIVITY = NLM_CONNECTIVITY(512i32);
pub const NLM_CONNECTIVITY_IPV6_NOTRAFFIC: NLM_CONNECTIVITY = NLM_CONNECTIVITY(2i32);
pub const NLM_CONNECTIVITY_IPV6_SUBNET: NLM_CONNECTIVITY = NLM_CONNECTIVITY(256i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_LDAP: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(1i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_NONE: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(0i32);
pub const NLM_DOMAIN_AUTHENTICATION_KIND_TLS: NLM_DOMAIN_AUTHENTICATION_KIND = NLM_DOMAIN_AUTHENTICATION_KIND(2i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(2i32);
pub const NLM_DOMAIN_TYPE_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(1i32);
pub const NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK: NLM_DOMAIN_TYPE = NLM_DOMAIN_TYPE(0i32);
pub const NLM_ENUM_NETWORK_ALL: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(3i32);
pub const NLM_ENUM_NETWORK_CONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(1i32);
pub const NLM_ENUM_NETWORK_DISCONNECTED: NLM_ENUM_NETWORK = NLM_ENUM_NETWORK(2i32);
pub const NLM_INTERNET_CONNECTIVITY_CORPORATE: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(4i32);
pub const NLM_INTERNET_CONNECTIVITY_PROXIED: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(2i32);
pub const NLM_INTERNET_CONNECTIVITY_WEBHIJACK: NLM_INTERNET_CONNECTIVITY = NLM_INTERNET_CONNECTIVITY(1i32);
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
pub const NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(2i32);
pub const NLM_NETWORK_CATEGORY_PRIVATE: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(1i32);
pub const NLM_NETWORK_CATEGORY_PUBLIC: NLM_NETWORK_CATEGORY = NLM_NETWORK_CATEGORY(0i32);
pub const NLM_NETWORK_IDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(2i32);
pub const NLM_NETWORK_IDENTIFYING: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(1i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(16i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_CONNECTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(1i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(2i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_ICON: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(8i32);
pub const NLM_NETWORK_PROPERTY_CHANGE_NAME: NLM_NETWORK_PROPERTY_CHANGE = NLM_NETWORK_PROPERTY_CHANGE(4i32);
pub const NLM_NETWORK_UNIDENTIFIED: NLM_NETWORK_CLASS = NLM_NETWORK_CLASS(3i32);
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_CONNECTION_COST(pub i32);
impl windows_core::TypeKind for NLM_CONNECTION_COST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_CONNECTION_COST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_COST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_CONNECTION_PROPERTY_CHANGE(pub i32);
impl windows_core::TypeKind for NLM_CONNECTION_PROPERTY_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_CONNECTION_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_CONNECTION_PROPERTY_CHANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_CONNECTIVITY(pub i32);
impl windows_core::TypeKind for NLM_CONNECTIVITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_CONNECTIVITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_CONNECTIVITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_DOMAIN_AUTHENTICATION_KIND(pub i32);
impl windows_core::TypeKind for NLM_DOMAIN_AUTHENTICATION_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_DOMAIN_AUTHENTICATION_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_DOMAIN_AUTHENTICATION_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_DOMAIN_TYPE(pub i32);
impl windows_core::TypeKind for NLM_DOMAIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_DOMAIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_DOMAIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_ENUM_NETWORK(pub i32);
impl windows_core::TypeKind for NLM_ENUM_NETWORK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_ENUM_NETWORK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_ENUM_NETWORK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_INTERNET_CONNECTIVITY(pub i32);
impl windows_core::TypeKind for NLM_INTERNET_CONNECTIVITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_INTERNET_CONNECTIVITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_INTERNET_CONNECTIVITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_NETWORK_CATEGORY(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_NETWORK_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_NETWORK_CLASS(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_NETWORK_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NLM_NETWORK_PROPERTY_CHANGE(pub i32);
impl windows_core::TypeKind for NLM_NETWORK_PROPERTY_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NLM_NETWORK_PROPERTY_CHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NLM_NETWORK_PROPERTY_CHANGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLM_DATAPLAN_STATUS {
    pub InterfaceGuid: windows_core::GUID,
    pub UsageData: NLM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub NextBillingCycle: super::super::Foundation::FILETIME,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for NLM_DATAPLAN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLM_SIMULATED_PROFILE_INFO {
    pub ProfileName: [u16; 256],
    pub cost: NLM_CONNECTION_COST,
    pub UsageInMegabytes: u32,
    pub DataLimitInMegabytes: u32,
}
impl windows_core::TypeKind for NLM_SIMULATED_PROFILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLM_SIMULATED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLM_SOCKADDR {
    pub data: [u8; 128],
}
impl windows_core::TypeKind for NLM_SOCKADDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLM_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for NLM_USAGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NLM_USAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NetworkListManager: windows_core::GUID = windows_core::GUID::from_u128(0xdcb00c01_570f_4a9b_8d69_199fdba5723b);
#[cfg(feature = "implement")]
core::include!("impl.rs");
