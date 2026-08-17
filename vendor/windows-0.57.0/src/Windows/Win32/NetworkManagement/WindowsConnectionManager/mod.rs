#[inline]
pub unsafe fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE) {
    windows_targets::link!("ondemandconnroutehelper.dll" "system" fn FreeInterfaceContextTable(interfacecontexttable : *const NET_INTERFACE_CONTEXT_TABLE));
    FreeInterfaceContextTable(interfacecontexttable)
}
#[inline]
pub unsafe fn GetInterfaceContextTableForHostName<P0, P1>(hostname: P0, proxyname: P1, flags: u32, connectionprofilefilterrawdata: Option<&[u8]>) -> windows_core::Result<*mut NET_INTERFACE_CONTEXT_TABLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ondemandconnroutehelper.dll" "system" fn GetInterfaceContextTableForHostName(hostname : windows_core::PCWSTR, proxyname : windows_core::PCWSTR, flags : u32, connectionprofilefilterrawdata : *const u8, connectionprofilefilterrawdatasize : u32, interfacecontexttable : *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetInterfaceContextTableForHostName(hostname.param().abi(), proxyname.param().abi(), flags, core::mem::transmute(connectionprofilefilterrawdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), connectionprofilefilterrawdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OnDemandGetRoutingHint<P0>(destinationhostname: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ondemandconnroutehelper.dll" "system" fn OnDemandGetRoutingHint(destinationhostname : windows_core::PCWSTR, interfaceindex : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OnDemandGetRoutingHint(destinationhostname.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("ondemandconnroutehelper.dll" "system" fn OnDemandRegisterNotification(callback : ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext : *const core::ffi::c_void, registrationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OnDemandRegisterNotification(callback, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OnDemandUnRegisterNotification<P0>(registrationhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ondemandconnroutehelper.dll" "system" fn OnDemandUnRegisterNotification(registrationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    OnDemandUnRegisterNotification(registrationhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WcmFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_targets::link!("wcmapi.dll" "system" fn WcmFreeMemory(pmemory : *mut core::ffi::c_void));
    WcmFreeMemory(pmemory)
}
#[inline]
pub unsafe fn WcmGetProfileList(preserved: Option<*const core::ffi::c_void>, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32 {
    windows_targets::link!("wcmapi.dll" "system" fn WcmGetProfileList(preserved : *const core::ffi::c_void, ppprofilelist : *mut *mut WCM_PROFILE_INFO_LIST) -> u32);
    WcmGetProfileList(core::mem::transmute(preserved.unwrap_or(std::ptr::null())), ppprofilelist)
}
#[inline]
pub unsafe fn WcmQueryProperty<P0>(pinterface: Option<*const windows_core::GUID>, strprofilename: P0, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wcmapi.dll" "system" fn WcmQueryProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut *mut u8) -> u32);
    WcmQueryProperty(core::mem::transmute(pinterface.unwrap_or(std::ptr::null())), strprofilename.param().abi(), property, core::mem::transmute(preserved.unwrap_or(std::ptr::null())), pdwdatasize, ppdata)
}
#[inline]
pub unsafe fn WcmSetProfileList<P0>(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: P0, preserved: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wcmapi.dll" "system" fn WcmSetProfileList(pprofilelist : *const WCM_PROFILE_INFO_LIST, dwposition : u32, fignoreunknownprofiles : super::super::Foundation:: BOOL, preserved : *const core::ffi::c_void) -> u32);
    WcmSetProfileList(pprofilelist, dwposition, fignoreunknownprofiles.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn WcmSetProperty<P0>(pinterface: Option<*const windows_core::GUID>, strprofilename: P0, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pbdata: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wcmapi.dll" "system" fn WcmSetProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, dwdatasize : u32, pbdata : *const u8) -> u32);
    WcmSetProperty(core::mem::transmute(pinterface.unwrap_or(std::ptr::null())), strprofilename.param().abi(), property, core::mem::transmute(preserved.unwrap_or(std::ptr::null())), pbdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
pub const WCM_API_VERSION: u32 = 1u32;
pub const WCM_API_VERSION_1_0: u32 = 1u32;
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCM_CONNECTION_COST(pub i32);
impl windows_core::TypeKind for WCM_CONNECTION_COST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCM_CONNECTION_COST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
impl windows_core::TypeKind for WCM_CONNECTION_COST_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCM_CONNECTION_COST_SOURCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST_SOURCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCM_MEDIA_TYPE(pub i32);
impl windows_core::TypeKind for WCM_MEDIA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCM_MEDIA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCM_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCM_PROPERTY(pub i32);
impl windows_core::TypeKind for WCM_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCM_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCM_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_INTERFACE_CONTEXT {
    pub InterfaceIndex: u32,
    pub ConfigurationName: windows_core::PWSTR,
}
impl windows_core::TypeKind for NET_INTERFACE_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_INTERFACE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_INTERFACE_CONTEXT_TABLE {
    pub InterfaceContextHandle: super::super::Foundation::HANDLE,
    pub NumberOfEntries: u32,
    pub InterfaceContextArray: *mut NET_INTERFACE_CONTEXT,
}
impl windows_core::TypeKind for NET_INTERFACE_CONTEXT_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_INTERFACE_CONTEXT_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::super::Foundation::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WCM_BILLING_CYCLE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_BILLING_CYCLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
impl windows_core::TypeKind for WCM_CONNECTION_COST_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_CONNECTION_COST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for WCM_DATAPLAN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_POLICY_VALUE {
    pub fValue: super::super::Foundation::BOOL,
    pub fIsGroupPolicy: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WCM_POLICY_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_POLICY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: windows_core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl windows_core::TypeKind for WCM_PROFILE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl windows_core::TypeKind for WCM_PROFILE_INFO_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl windows_core::TypeKind for WCM_TIME_INTERVAL {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for WCM_USAGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCM_USAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ONDEMAND_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(param0: *const core::ffi::c_void)>;
