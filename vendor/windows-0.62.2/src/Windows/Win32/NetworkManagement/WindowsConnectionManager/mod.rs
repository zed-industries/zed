#[inline]
pub unsafe fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE) {
    windows_core::link!("ondemandconnroutehelper.dll" "system" fn FreeInterfaceContextTable(interfacecontexttable : *const NET_INTERFACE_CONTEXT_TABLE));
    unsafe { FreeInterfaceContextTable(interfacecontexttable) }
}
#[inline]
pub unsafe fn GetInterfaceContextTableForHostName<P0, P1>(hostname: P0, proxyname: P1, flags: u32, connectionprofilefilterrawdata: Option<&[u8]>) -> windows_core::Result<*mut NET_INTERFACE_CONTEXT_TABLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ondemandconnroutehelper.dll" "system" fn GetInterfaceContextTableForHostName(hostname : windows_core::PCWSTR, proxyname : windows_core::PCWSTR, flags : u32, connectionprofilefilterrawdata : *const u8, connectionprofilefilterrawdatasize : u32, interfacecontexttable : *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetInterfaceContextTableForHostName(hostname.param().abi(), proxyname.param().abi(), flags, core::mem::transmute(connectionprofilefilterrawdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), connectionprofilefilterrawdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn OnDemandGetRoutingHint<P0>(destinationhostname: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ondemandconnroutehelper.dll" "system" fn OnDemandGetRoutingHint(destinationhostname : windows_core::PCWSTR, interfaceindex : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OnDemandGetRoutingHint(destinationhostname.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("ondemandconnroutehelper.dll" "system" fn OnDemandRegisterNotification(callback : ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext : *const core::ffi::c_void, registrationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OnDemandRegisterNotification(callback, callbackcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ondemandconnroutehelper.dll" "system" fn OnDemandUnRegisterNotification(registrationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { OnDemandUnRegisterNotification(registrationhandle).ok() }
}
#[inline]
pub unsafe fn WcmFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_core::link!("wcmapi.dll" "system" fn WcmFreeMemory(pmemory : *mut core::ffi::c_void));
    unsafe { WcmFreeMemory(pmemory as _) }
}
#[inline]
pub unsafe fn WcmGetProfileList(preserved: Option<*const core::ffi::c_void>, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32 {
    windows_core::link!("wcmapi.dll" "system" fn WcmGetProfileList(preserved : *const core::ffi::c_void, ppprofilelist : *mut *mut WCM_PROFILE_INFO_LIST) -> u32);
    unsafe { WcmGetProfileList(preserved.unwrap_or(core::mem::zeroed()) as _, ppprofilelist as _) }
}
#[inline]
pub unsafe fn WcmQueryProperty<P1>(pinterface: Option<*const windows_core::GUID>, strprofilename: P1, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wcmapi.dll" "system" fn WcmQueryProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut *mut u8) -> u32);
    unsafe { WcmQueryProperty(pinterface.unwrap_or(core::mem::zeroed()) as _, strprofilename.param().abi(), property, preserved.unwrap_or(core::mem::zeroed()) as _, pdwdatasize as _, ppdata as _) }
}
#[inline]
pub unsafe fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: bool, preserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("wcmapi.dll" "system" fn WcmSetProfileList(pprofilelist : *const WCM_PROFILE_INFO_LIST, dwposition : u32, fignoreunknownprofiles : windows_core::BOOL, preserved : *const core::ffi::c_void) -> u32);
    unsafe { WcmSetProfileList(pprofilelist, dwposition, fignoreunknownprofiles.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WcmSetProperty<P1>(pinterface: Option<*const windows_core::GUID>, strprofilename: P1, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pbdata: Option<&[u8]>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wcmapi.dll" "system" fn WcmSetProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, dwdatasize : u32, pbdata : *const u8) -> u32);
    unsafe { WcmSetProperty(pinterface.unwrap_or(core::mem::zeroed()) as _, strprofilename.param().abi(), property, preserved.unwrap_or(core::mem::zeroed()) as _, pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NET_INTERFACE_CONTEXT {
    pub InterfaceIndex: u32,
    pub ConfigurationName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NET_INTERFACE_CONTEXT_TABLE {
    pub InterfaceContextHandle: super::super::Foundation::HANDLE,
    pub NumberOfEntries: u32,
    pub InterfaceContextArray: *mut NET_INTERFACE_CONTEXT,
}
impl Default for NET_INTERFACE_CONTEXT_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
pub type ONDEMAND_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(param0: *const core::ffi::c_void)>;
pub const WCM_API_VERSION: u32 = 1u32;
pub const WCM_API_VERSION_1_0: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::super::Foundation::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCM_CONNECTION_COST(pub i32);
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCM_MEDIA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_POLICY_VALUE {
    pub fValue: windows_core::BOOL,
    pub fIsGroupPolicy: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: windows_core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCM_PROPERTY(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
pub const wcm_global_property_domain_policy: WCM_PROPERTY = WCM_PROPERTY(0i32);
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = WCM_PROPERTY(1i32);
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = WCM_PROPERTY(3i32);
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = WCM_PROPERTY(2i32);
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = WCM_PROPERTY(4i32);
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = WCM_PROPERTY(5i32);
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = WCM_PROPERTY(6i32);
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(1i32);
pub const wcm_media_invalid: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(4i32);
pub const wcm_media_max: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(5i32);
pub const wcm_media_mbn: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(3i32);
pub const wcm_media_unknown: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(0i32);
pub const wcm_media_wlan: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(2i32);
