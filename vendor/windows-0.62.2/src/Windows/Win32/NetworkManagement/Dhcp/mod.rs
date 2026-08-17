#[inline]
pub unsafe fn DhcpAddFilterV4<P0>(serveripaddress: P0, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddFilterV4(serveripaddress : windows_core::PCWSTR, addfilterinfo : *const DHCP_FILTER_ADD_INFO, forceflag : windows_core::BOOL) -> u32);
    unsafe { DhcpAddFilterV4(serveripaddress.param().abi(), addfilterinfo, forceflag.into()) }
}
#[inline]
pub unsafe fn DhcpAddSecurityGroup<P0>(pserver: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSecurityGroup(pserver : windows_core::PCWSTR) -> u32);
    unsafe { DhcpAddSecurityGroup(pserver.param().abi()) }
}
#[inline]
pub unsafe fn DhcpAddServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : *mut DHCPDS_SERVER, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpAddServer(flags, idinfo as _, newserver as _, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA) -> u32);
    unsafe { DhcpAddSubnetElement(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32);
    unsafe { DhcpAddSubnetElementV4(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32);
    unsafe { DhcpAddSubnetElementV5(serveripaddress.param().abi(), subnetaddress, addelementinfo) }
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, addelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32);
    unsafe { DhcpAddSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), addelementinfo as _) }
}
#[inline]
pub unsafe fn DhcpAuditLogGetParams<P0>(serveripaddress: P0, flags: u32, auditlogdir: *mut windows_core::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAuditLogGetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : *mut windows_core::PWSTR, diskcheckinterval : *mut u32, maxlogfilessize : *mut u32, minspaceondisk : *mut u32) -> u32);
    unsafe { DhcpAuditLogGetParams(serveripaddress.param().abi(), flags, auditlogdir as _, diskcheckinterval as _, maxlogfilessize as _, minspaceondisk as _) }
}
#[inline]
pub unsafe fn DhcpAuditLogSetParams<P0, P2>(serveripaddress: P0, flags: u32, auditlogdir: P2, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpAuditLogSetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : windows_core::PCWSTR, diskcheckinterval : u32, maxlogfilessize : u32, minspaceondisk : u32) -> u32);
    unsafe { DhcpAuditLogSetParams(serveripaddress.param().abi(), flags, auditlogdir.param().abi(), diskcheckinterval, maxlogfilessize, minspaceondisk) }
}
#[inline]
pub unsafe fn DhcpCApiCleanup() {
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpCApiCleanup());
    unsafe { DhcpCApiCleanup() }
}
#[inline]
pub unsafe fn DhcpCApiInitialize(version: *mut u32) -> u32 {
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpCApiInitialize(version : *mut u32) -> u32);
    unsafe { DhcpCApiInitialize(version as _) }
}
#[inline]
pub unsafe fn DhcpCreateClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    unsafe { DhcpCreateClass(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    unsafe { DhcpCreateClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpCreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpCreateClientInfoV4(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpCreateClientInfoVQ(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpCreateOption<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateOption(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *const DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOption(serveripaddress.param().abi(), optionid, optioninfo) }
}
#[inline]
pub unsafe fn DhcpCreateOptionV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateOptionV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpCreateOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateSubnet<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpCreateSubnet(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpCreateSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpCreateSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpCreateSubnetVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpCreateSubnetVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpDeRegisterParamChange(flags : u32, reserved : *mut core::ffi::c_void, event : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpDeRegisterParamChange(flags, reserved as _, event as _) }
}
#[inline]
pub unsafe fn DhcpDeleteClass<P0, P2>(serveripaddress: P0, reservedmustbezero: u32, classname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteClass(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDeleteClassV6<P0, P2>(serveripaddress: P0, reservedmustbezero: u32, classname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteClassV6(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDeleteClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO) -> u32);
    unsafe { DhcpDeleteClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO_V6) -> u32);
    unsafe { DhcpDeleteClientInfoV6(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteFilterV4<P0>(serveripaddress: P0, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteFilterV4(serveripaddress : windows_core::PCWSTR, deletefilterinfo : *const DHCP_ADDR_PATTERN) -> u32);
    unsafe { DhcpDeleteFilterV4(serveripaddress.param().abi(), deletefilterinfo) }
}
#[inline]
pub unsafe fn DhcpDeleteServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : *mut DHCPDS_SERVER, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpDeleteServer(flags, idinfo as _, newserver as _, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpDeleteSubnet<P0>(serveripaddress: P0, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpDeleteSubnet(serveripaddress.param().abi(), subnetaddress, forceflag) }
}
#[inline]
pub unsafe fn DhcpDeleteSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpDeleteSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), forceflag) }
}
#[inline]
pub unsafe fn DhcpDeleteSuperScopeV4<P0, P1>(serveripaddress: P0, superscopename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDeleteSuperScopeV4(serveripaddress : windows_core::PCWSTR, superscopename : windows_core::PCWSTR) -> u32);
    unsafe { DhcpDeleteSuperScopeV4(serveripaddress.param().abi(), superscopename.param().abi()) }
}
#[inline]
pub unsafe fn DhcpDsCleanup() {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDsCleanup());
    unsafe { DhcpDsCleanup() }
}
#[inline]
pub unsafe fn DhcpDsInit() -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpDsInit() -> u32);
    unsafe { DhcpDsInit() }
}
#[inline]
pub unsafe fn DhcpEnumClasses<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumClasses(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut u32, preferredmaximum : u32, classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY, nread : *mut u32, ntotal : *mut u32) -> u32);
    unsafe { DhcpEnumClasses(serveripaddress.param().abi(), reservedmustbezero, resumehandle as _, preferredmaximum, classinfoarray as _, nread as _, ntotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumClassesV6<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumClassesV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut u32, preferredmaximum : u32, classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread : *mut u32, ntotal : *mut u32) -> u32);
    unsafe { DhcpEnumClassesV6(serveripaddress.param().abi(), reservedmustbezero, resumehandle as _, preferredmaximum, classinfoarray as _, nread as _, ntotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumFilterV4<P0>(serveripaddress: P0, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumFilterV4(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_ADDR_PATTERN, preferredmaximum : u32, listtype : DHCP_FILTER_LIST_TYPE, enumfilterinfo : *mut *mut DHCP_FILTER_ENUM_INFO, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumFilterV4(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, listtype, enumfilterinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValues(serveripaddress.param().abi(), scopeinfo, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV6<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionValuesV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, resumehandle as _, preferredmaximum, optionvalues as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptions<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptions(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptions(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionsV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionsV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumOptionsV6<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    unsafe { DhcpEnumOptionsV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle as _, preferredmaximum, options as _, optionsread as _, optionstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumServers(flags: u32, idinfo: *mut core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumServers(flags : u32, idinfo : *mut core::ffi::c_void, servers : *mut *mut DHCPDS_SERVERS, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpEnumServers(flags, idinfo as _, servers as _, callbackfn as _, callbackdata as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsFilterStatusInfo<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV4<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV4(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV5<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV5(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, resumehandle : *mut DHCP_IPV6_ADDRESS, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsVQ<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetClientsVQ(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElements<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElements(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElements(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV4<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV4(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV5<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV5(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetElementsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), enumelementtype, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnets<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnets(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, enuminfo : *mut *mut DHCP_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnets(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpEnumSubnetsV6<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetsV6(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, enuminfo : *mut *mut DHCPV6_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpEnumSubnetsV6(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut *mut DHCP_ALL_OPTION_VALUES) -> u32);
    unsafe { DhcpGetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionValuesV6<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, values : *mut *mut DHCP_ALL_OPTION_VALUES) -> u32);
    unsafe { DhcpGetAllOptionValuesV6(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptions<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptions(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut *mut DHCP_ALL_OPTIONS) -> u32);
    unsafe { DhcpGetAllOptions(serveripaddress.param().abi(), flags, optionstruct as _) }
}
#[inline]
pub unsafe fn DhcpGetAllOptionsV6<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut *mut DHCP_ALL_OPTIONS) -> u32);
    unsafe { DhcpGetAllOptionsV6(serveripaddress.param().abi(), flags, optionstruct as _) }
}
#[inline]
pub unsafe fn DhcpGetClassInfo<P0>(serveripaddress: P0, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClassInfo(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, partialclassinfo : *mut DHCP_CLASS_INFO, filledclassinfo : *mut *mut DHCP_CLASS_INFO) -> u32);
    unsafe { DhcpGetClassInfo(serveripaddress.param().abi(), reservedmustbezero, partialclassinfo as _, filledclassinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoV4<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV4(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpGetClientInfoV4(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoV6<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV6(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO_V6, clientinfo : *mut *mut DHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpGetClientInfoV6(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientInfoVQ<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoVQ(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpGetClientInfoVQ(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetClientOptions<P0>(serveripaddress: P0, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetClientOptions(serveripaddress : windows_core::PCWSTR, clientipaddress : u32, clientsubnetmask : u32, clientoptions : *mut *mut DHCP_OPTION_LIST) -> u32);
    unsafe { DhcpGetClientOptions(serveripaddress.param().abi(), clientipaddress, clientsubnetmask, clientoptions as _) }
}
#[inline]
pub unsafe fn DhcpGetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *mut DHCP_FILTER_GLOBAL_INFO) -> u32);
    unsafe { DhcpGetFilterV4(serveripaddress.param().abi(), globalfilterinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfo<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfo(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO) -> u32);
    unsafe { DhcpGetMibInfo(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfoV5<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV5(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO_V5) -> u32);
    unsafe { DhcpGetMibInfoV5(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetMibInfoV6<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV6(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO_V6) -> u32);
    unsafe { DhcpGetMibInfoV6(serveripaddress.param().abi(), mibinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    unsafe { DhcpGetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpGetOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpGetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpGetOriginalSubnetMask<P0>(sadaptername: P0, dwsubnetmask: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpGetOriginalSubnetMask(sadaptername : windows_core::PCWSTR, dwsubnetmask : *mut u32) -> u32);
    unsafe { DhcpGetOriginalSubnetMask(sadaptername.param().abi(), dwsubnetmask as _) }
}
#[inline]
pub unsafe fn DhcpGetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpGetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementsinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpGetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementsinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetServerSpecificStrings<P0>(serveripaddress: P0, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetServerSpecificStrings(serveripaddress : windows_core::PCWSTR, serverspecificstrings : *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32);
    unsafe { DhcpGetServerSpecificStrings(serveripaddress.param().abi(), serverspecificstrings as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, timedelayinmilliseconds : *mut u16) -> u32);
    unsafe { DhcpGetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *mut *mut DHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpGetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut *mut DHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpGetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *mut *mut DHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpGetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpGetSuperScopeInfoV4<P0>(serveripaddress: P0, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetSuperScopeInfoV4(serveripaddress : windows_core::PCWSTR, superscopetable : *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32);
    unsafe { DhcpGetSuperScopeInfoV4(serveripaddress.param().abi(), superscopetable as _) }
}
#[inline]
pub unsafe fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetThreadOptions(pflags : *mut u32, reserved : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpGetThreadOptions(pflags as _, reserved as _) }
}
#[inline]
pub unsafe fn DhcpGetVersion<P0>(serveripaddress: P0, majorversion: *mut u32, minorversion: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpGetVersion(serveripaddress : windows_core::PCWSTR, majorversion : *mut u32, minorversion : *mut u32) -> u32);
    unsafe { DhcpGetVersion(serveripaddress.param().abi(), majorversion as _, minorversion as _) }
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyCondition<P5>(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: P5, operator: DHCP_POL_COMPARATOR, value: &[u8], conditionindex: *mut u32) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyCondition(policy : *mut DHCP_POLICY, parentexpr : u32, r#type : DHCP_POL_ATTR_TYPE, optionid : u32, suboptionid : u32, vendorname : windows_core::PCWSTR, operator : DHCP_POL_COMPARATOR, value : *const u8, valuelength : u32, conditionindex : *mut u32) -> u32);
    unsafe { DhcpHlprAddV4PolicyCondition(policy as _, parentexpr, r#type, optionid, suboptionid, vendorname.param().abi(), operator, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap(), conditionindex as _) }
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyExpr(policy : *mut DHCP_POLICY, parentexpr : u32, operator : DHCP_POL_LOGIC_OPER, exprindex : *mut u32) -> u32);
    unsafe { DhcpHlprAddV4PolicyExpr(policy as _, parentexpr, operator, exprindex as _) }
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyRange(policy : *mut DHCP_POLICY, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpHlprAddV4PolicyRange(policy as _, range) }
}
#[inline]
pub unsafe fn DhcpHlprCreateV4Policy<P0, P5>(policyname: P0, fglobalpolicy: bool, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P5, enabled: bool, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4Policy(policyname : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnet : u32, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : windows_core::BOOL, policy : *mut *mut DHCP_POLICY) -> u32);
    unsafe { DhcpHlprCreateV4Policy(policyname.param().abi(), fglobalpolicy.into(), subnet, processingorder, rootoperator, description.param().abi(), enabled.into(), policy as _) }
}
#[inline]
pub unsafe fn DhcpHlprCreateV4PolicyEx<P0, P5>(policyname: P0, fglobalpolicy: bool, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P5, enabled: bool, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4PolicyEx(policyname : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnet : u32, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : windows_core::BOOL, policy : *mut *mut DHCP_POLICY_EX) -> u32);
    unsafe { DhcpHlprCreateV4PolicyEx(policyname.param().abi(), fglobalpolicy.into(), subnet, processingorder, rootoperator, description.param().abi(), enabled.into(), policy as _) }
}
#[inline]
pub unsafe fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFindV4DhcpProperty(propertyarray : *const DHCP_PROPERTY_ARRAY, id : DHCP_PROPERTY_ID, r#type : DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY);
    unsafe { DhcpHlprFindV4DhcpProperty(propertyarray, id, r#type) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpProperty(property : *mut DHCP_PROPERTY));
    unsafe { DhcpHlprFreeV4DhcpProperty(property as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray : *mut DHCP_PROPERTY_ARRAY));
    unsafe { DhcpHlprFreeV4DhcpPropertyArray(propertyarray as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4Policy(policy : *mut DHCP_POLICY));
    unsafe { DhcpHlprFreeV4Policy(policy as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyArray(policyarray : *mut DHCP_POLICY_ARRAY));
    unsafe { DhcpHlprFreeV4PolicyArray(policyarray as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyEx(policyex : *mut DHCP_POLICY_EX));
    unsafe { DhcpHlprFreeV4PolicyEx(policyex as _) }
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyExArray(policyexarray : *mut DHCP_POLICY_EX_ARRAY));
    unsafe { DhcpHlprFreeV4PolicyExArray(policyexarray as _) }
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> windows_core::BOOL {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicySingleUC(policy : *const DHCP_POLICY) -> windows_core::BOOL);
    unsafe { DhcpHlprIsV4PolicySingleUC(policy) }
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyValid(ppolicy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpHlprIsV4PolicyValid(ppolicy) }
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> windows_core::BOOL {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyWellFormed(ppolicy : *const DHCP_POLICY) -> windows_core::BOOL);
    unsafe { DhcpHlprIsV4PolicyWellFormed(ppolicy) }
}
#[inline]
pub unsafe fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprModifyV4PolicyExpr(policy : *mut DHCP_POLICY, operator : DHCP_POL_LOGIC_OPER) -> u32);
    unsafe { DhcpHlprModifyV4PolicyExpr(policy as _, operator) }
}
#[inline]
pub unsafe fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpHlprResetV4PolicyExpr(policy : *mut DHCP_POLICY) -> u32);
    unsafe { DhcpHlprResetV4PolicyExpr(policy as _) }
}
#[inline]
pub unsafe fn DhcpModifyClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpModifyClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    unsafe { DhcpModifyClass(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpModifyClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpModifyClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    unsafe { DhcpModifyClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo as _) }
}
#[inline]
pub unsafe fn DhcpRegisterParamChange<P2>(flags: u32, reserved: Option<*const core::ffi::c_void>, adaptername: P2, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpRegisterParamChange(flags : u32, reserved : *const core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPCAPI_CLASSID, params : DHCPCAPI_PARAMS_ARRAY, handle : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpRegisterParamChange(flags, reserved.unwrap_or(core::mem::zeroed()) as _, adaptername.param().abi(), classid as _, core::mem::transmute(params), handle as _) }
}
#[inline]
pub unsafe fn DhcpRemoveDNSRegistrations() -> u32 {
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpRemoveDNSRegistrations() -> u32);
    unsafe { DhcpRemoveDNSRegistrations() }
}
#[inline]
pub unsafe fn DhcpRemoveOption<P0>(serveripaddress: P0, optionid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOption(serveripaddress : windows_core::PCWSTR, optionid : u32) -> u32);
    unsafe { DhcpRemoveOption(serveripaddress.param().abi(), optionid) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpRemoveOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpRemoveOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpRemoveOptionValue(serveripaddress.param().abi(), optionid, scopeinfo) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpRemoveOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6) -> u32);
    unsafe { DhcpRemoveOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElement(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV4(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV5(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag) }
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, removeelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag : DHCP_FORCE_FLAG) -> u32);
    unsafe { DhcpRemoveSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), removeelementinfo as _, forceflag) }
}
#[inline]
pub unsafe fn DhcpRequestParams<P2, P8>(flags: u32, reserved: *mut core::ffi::c_void, adaptername: P2, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: P8) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpRequestParams(flags : u32, reserved : *mut core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPCAPI_CLASSID, sendparams : DHCPCAPI_PARAMS_ARRAY, recdparams : DHCPCAPI_PARAMS_ARRAY, buffer : *mut u8, psize : *mut u32, requestidstr : windows_core::PCWSTR) -> u32);
    unsafe { DhcpRequestParams(flags, reserved as _, adaptername.param().abi(), classid as _, core::mem::transmute(sendparams), core::mem::transmute(recdparams), buffer as _, psize as _, requestidstr.param().abi()) }
}
#[inline]
pub unsafe fn DhcpRpcFreeMemory(bufferpointer: *mut core::ffi::c_void) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpRpcFreeMemory(bufferpointer : *mut core::ffi::c_void));
    unsafe { DhcpRpcFreeMemory(bufferpointer as _) }
}
#[inline]
pub unsafe fn DhcpScanDatabase<P0>(serveripaddress: P0, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpScanDatabase(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, fixflag : u32, scanlist : *mut *mut DHCP_SCAN_LIST) -> u32);
    unsafe { DhcpScanDatabase(serveripaddress.param().abi(), subnetaddress, fixflag, scanlist as _) }
}
#[inline]
pub unsafe fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerAuditlogParamsFree(configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ));
    unsafe { DhcpServerAuditlogParamsFree(configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerBackupDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerBackupDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerBackupDatabase(serveripaddress.param().abi(), path.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerGetConfig<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfig(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32);
    unsafe { DhcpServerGetConfig(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigV4<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV4(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32);
    unsafe { DhcpServerGetConfigV4(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32);
    unsafe { DhcpServerGetConfigV6(serveripaddress.param().abi(), scopeinfo as _, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerGetConfigVQ<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigVQ(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    unsafe { DhcpServerGetConfigVQ(serveripaddress.param().abi(), configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryAttribute<P0>(serveripaddr: P0, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerQueryAttribute(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dhcpattribid : u32, pdhcpattrib : *mut *mut DHCP_ATTRIB) -> u32);
    unsafe { DhcpServerQueryAttribute(serveripaddr.param().abi(), dwreserved, dhcpattribid, pdhcpattrib as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryAttributes<P0>(serveripaddr: P0, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerQueryAttributes(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dwattribcount : u32, pdhcpattribs : *mut u32, pdhcpattribarr : *mut *mut DHCP_ATTRIB_ARRAY) -> u32);
    unsafe { DhcpServerQueryAttributes(serveripaddr.param().abi(), dwreserved, dwattribcount, pdhcpattribs as _, pdhcpattribarr as _) }
}
#[inline]
pub unsafe fn DhcpServerQueryDnsRegCredentials<P0>(serveripaddress: P0, uname: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerQueryDnsRegCredentials(serveripaddress : windows_core::PCWSTR, unamesize : u32, uname : windows_core::PWSTR, domainsize : u32, domain : windows_core::PWSTR) -> u32);
    unsafe { DhcpServerQueryDnsRegCredentials(serveripaddress.param().abi(), uname.len().try_into().unwrap(), core::mem::transmute(uname.as_ptr()), domain.len().try_into().unwrap(), core::mem::transmute(domain.as_ptr())) }
}
#[inline]
pub unsafe fn DhcpServerRedoAuthorization<P0>(serveripaddr: P0, dwreserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerRedoAuthorization(serveripaddr : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { DhcpServerRedoAuthorization(serveripaddr.param().abi(), dwreserved) }
}
#[inline]
pub unsafe fn DhcpServerRestoreDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerRestoreDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerRestoreDatabase(serveripaddress.param().abi(), path.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerSetConfig<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfig(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO) -> u32);
    unsafe { DhcpServerSetConfig(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigV4<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV4(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32);
    unsafe { DhcpServerSetConfigV4(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32);
    unsafe { DhcpServerSetConfigV6(serveripaddress.param().abi(), scopeinfo as _, fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetConfigVQ<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigVQ(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    unsafe { DhcpServerSetConfigVQ(serveripaddress.param().abi(), fieldstoset, configinfo as _) }
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentials<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetDnsRegCredentials(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerSetDnsRegCredentials(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi()) }
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentialsV5<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpServerSetDnsRegCredentialsV5(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    unsafe { DhcpServerSetDnsRegCredentialsV5(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi()) }
}
#[inline]
pub unsafe fn DhcpSetClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    unsafe { DhcpSetClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    unsafe { DhcpSetClientInfoV4(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpSetClientInfoV6(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    unsafe { DhcpSetClientInfoVQ(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpSetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *const DHCP_FILTER_GLOBAL_INFO) -> u32);
    unsafe { DhcpSetFilterV4(serveripaddress.param().abi(), globalfilterinfo) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *const DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    unsafe { DhcpSetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *const DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue) }
}
#[inline]
pub unsafe fn DhcpSetOptionValueV5<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValueV6<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, classname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpSetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpSetOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalues : *const DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpSetOptionValues(serveripaddress.param().abi(), scopeinfo, optionvalues) }
}
#[inline]
pub unsafe fn DhcpSetOptionValuesV5<P0, P2, P3>(serveripaddress: P0, flags: u32, classname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpSetOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalues as _) }
}
#[inline]
pub unsafe fn DhcpSetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCP_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpSetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementinfo as _) }
}
#[inline]
pub unsafe fn DhcpSetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    unsafe { DhcpSetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementinfo as _) }
}
#[inline]
pub unsafe fn DhcpSetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, timedelayinmilliseconds : u16) -> u32);
    unsafe { DhcpSetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    unsafe { DhcpSetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    unsafe { DhcpSetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo as _) }
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    unsafe { DhcpSetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo) }
}
#[inline]
pub unsafe fn DhcpSetSuperScopeV4<P0, P2>(serveripaddress: P0, subnetaddress: u32, superscopename: P2, changeexisting: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetSuperScopeV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, superscopename : windows_core::PCWSTR, changeexisting : windows_core::BOOL) -> u32);
    unsafe { DhcpSetSuperScopeV4(serveripaddress.param().abi(), subnetaddress, superscopename.param().abi(), changeexisting.into()) }
}
#[inline]
pub unsafe fn DhcpSetThreadOptions(flags: u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpSetThreadOptions(flags : u32, reserved : *mut core::ffi::c_void) -> u32);
    unsafe { DhcpSetThreadOptions(flags, reserved as _) }
}
#[inline]
pub unsafe fn DhcpUndoRequestParams<P2, P3>(flags: Option<u32>, reserved: Option<*const core::ffi::c_void>, adaptername: P2, requestidstr: P3) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc.dll" "system" fn DhcpUndoRequestParams(flags : u32, reserved : *const core::ffi::c_void, adaptername : windows_core::PCWSTR, requestidstr : windows_core::PCWSTR) -> u32);
    unsafe { DhcpUndoRequestParams(flags.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _, adaptername.param().abi(), requestidstr.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4AddPolicyRange<P0, P2>(serveripaddress: P0, subnetaddress: u32, policyname: P2, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4AddPolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpV4AddPolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range) }
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_PB) -> u32);
    unsafe { DhcpV4CreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfoEx<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfoEx(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_EX) -> u32);
    unsafe { DhcpV4CreateClientInfoEx(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpV4CreatePolicy<P0>(serveripaddress: P0, ppolicy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicy(serveripaddress : windows_core::PCWSTR, ppolicy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpV4CreatePolicy(serveripaddress.param().abi(), ppolicy) }
}
#[inline]
pub unsafe fn DhcpV4CreatePolicyEx<P0>(serveripaddress: P0, policyex: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicyEx(serveripaddress : windows_core::PCWSTR, policyex : *const DHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4CreatePolicyEx(serveripaddress.param().abi(), policyex) }
}
#[inline]
pub unsafe fn DhcpV4DeletePolicy<P0, P3>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: u32, policyname: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4DeletePolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4DeletePolicy(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, policyname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4EnumPolicies<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: bool, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPolicies(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, enuminfo : *mut *mut DHCP_POLICY_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumPolicies(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, fglobalpolicy.into(), subnetaddress, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumPoliciesEx<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: bool, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPoliciesEx(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, globalpolicy : windows_core::BOOL, subnetaddress : u32, enuminfo : *mut *mut DHCP_POLICY_EX_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumPoliciesEx(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, globalpolicy.into(), subnetaddress, enuminfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClientsEx<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClientsEx(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetClientsEx(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, clientinfo as _, clientsread as _, clientstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetReservations<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetReservations(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    unsafe { DhcpV4EnumSubnetReservations(serveripaddress.param().abi(), subnetaddress, resumehandle as _, preferredmaximum, enumelementinfo as _, elementsread as _, elementstotal as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverAddScopeToRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverAddScopeToRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverAddScopeToRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverCreateRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverCreateRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverCreateRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4FailoverDeleteRelationship(serveripaddress.param().abi(), prelationshipname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteScopeFromRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress.param().abi(), prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverEnumRelationship<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverEnumRelationship(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread : *mut u32, relationshiptotal : *mut u32) -> u32);
    unsafe { DhcpV4FailoverEnumRelationship(serveripaddress.param().abi(), resumehandle as _, preferredmaximum, prelationship as _, relationshipread as _, relationshiptotal as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetAddressStatus<P0>(serveripaddress: P0, subnetaddress: u32, pstatus: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetAddressStatus(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, pstatus : *mut u32) -> u32);
    unsafe { DhcpV4FailoverGetAddressStatus(serveripaddress.param().abi(), subnetaddress, pstatus as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32);
    unsafe { DhcpV4FailoverGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverGetRelationship(serveripaddress.param().abi(), prelationshipname.param().abi(), prelationship as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeRelationship<P0>(serveripaddress: P0, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeRelationship(serveripaddress : windows_core::PCWSTR, scopeid : u32, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverGetScopeRelationship(serveripaddress.param().abi(), scopeid, prelationship as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeStatistics<P0>(serveripaddress: P0, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeStatistics(serveripaddress : windows_core::PCWSTR, scopeid : u32, pstats : *mut *mut DHCP_FAILOVER_STATISTICS) -> u32);
    unsafe { DhcpV4FailoverGetScopeStatistics(serveripaddress.param().abi(), scopeid, pstats as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverGetSystemTime<P0>(serveripaddress: P0, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetSystemTime(serveripaddress : windows_core::PCWSTR, ptime : *mut u32, pmaxalloweddeltatime : *mut u32) -> u32);
    unsafe { DhcpV4FailoverGetSystemTime(serveripaddress.param().abi(), ptime as _, pmaxalloweddeltatime as _) }
}
#[inline]
pub unsafe fn DhcpV4FailoverSetRelationship<P0>(serveripaddress: P0, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverSetRelationship(serveripaddress : windows_core::PCWSTR, flags : u32, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    unsafe { DhcpV4FailoverSetRelationship(serveripaddress.param().abi(), flags, prelationship) }
}
#[inline]
pub unsafe fn DhcpV4FailoverTriggerAddrAllocation<P0, P1>(serveripaddress: P0, pfailrelname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress : windows_core::PCWSTR, pfailrelname : windows_core::PCWSTR) -> u32);
    unsafe { DhcpV4FailoverTriggerAddrAllocation(serveripaddress.param().abi(), pfailrelname.param().abi()) }
}
#[inline]
pub unsafe fn DhcpV4GetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32);
    unsafe { DhcpV4GetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo as _, values as _) }
}
#[inline]
pub unsafe fn DhcpV4GetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_PB) -> u32);
    unsafe { DhcpV4GetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4GetClientInfoEx<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfoEx(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_EX) -> u32);
    unsafe { DhcpV4GetClientInfoEx(serveripaddress.param().abi(), searchinfo, clientinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : u32, startip : u32, endip : u32, numfreeaddrreq : u32, ipaddrlist : *mut *mut DHCP_IP_ARRAY) -> u32);
    unsafe { DhcpV4GetFreeIPAddress(serveripaddress.param().abi(), scopeid, startip, endip, numfreeaddrreq, ipaddrlist as _) }
}
#[inline]
pub unsafe fn DhcpV4GetOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    unsafe { DhcpV4GetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpV4GetPolicy<P0, P3>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: u32, policyname: P3, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *mut *mut DHCP_POLICY) -> u32);
    unsafe { DhcpV4GetPolicy(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, policyname.param().abi(), policy as _) }
}
#[inline]
pub unsafe fn DhcpV4GetPolicyEx<P0, P3>(serveripaddress: P0, globalpolicy: bool, subnetaddress: u32, policyname: P3, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicyEx(serveripaddress : windows_core::PCWSTR, globalpolicy : windows_core::BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *mut *mut DHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4GetPolicyEx(serveripaddress.param().abi(), globalpolicy.into(), subnetaddress, policyname.param().abi(), policy as _) }
}
#[inline]
pub unsafe fn DhcpV4QueryPolicyEnforcement<P0>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: u32, enabled: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4QueryPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, enabled : *mut windows_core::BOOL) -> u32);
    unsafe { DhcpV4QueryPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, enabled as _) }
}
#[inline]
pub unsafe fn DhcpV4RemoveOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4RemoveOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    unsafe { DhcpV4RemoveOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _) }
}
#[inline]
pub unsafe fn DhcpV4RemovePolicyRange<P0, P2>(serveripaddress: P0, subnetaddress: u32, policyname: P2, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4RemovePolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    unsafe { DhcpV4RemovePolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range) }
}
#[inline]
pub unsafe fn DhcpV4SetOptionValue<P0, P3, P4>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P3, vendorname: P4, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    unsafe { DhcpV4SetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalue as _) }
}
#[inline]
pub unsafe fn DhcpV4SetOptionValues<P0, P2, P3>(serveripaddress: P0, flags: u32, policyname: P2, vendorname: P3, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    unsafe { DhcpV4SetOptionValues(serveripaddress.param().abi(), flags, policyname.param().abi(), vendorname.param().abi(), scopeinfo as _, optionvalues as _) }
}
#[inline]
pub unsafe fn DhcpV4SetPolicy<P0, P4>(serveripaddress: P0, fieldsmodified: u32, fglobalpolicy: bool, subnetaddress: u32, policyname: P4, policy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicy(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY) -> u32);
    unsafe { DhcpV4SetPolicy(serveripaddress.param().abi(), fieldsmodified, fglobalpolicy.into(), subnetaddress, policyname.param().abi(), policy) }
}
#[inline]
pub unsafe fn DhcpV4SetPolicyEnforcement<P0>(serveripaddress: P0, fglobalpolicy: bool, subnetaddress: u32, enable: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : windows_core::BOOL, subnetaddress : u32, enable : windows_core::BOOL) -> u32);
    unsafe { DhcpV4SetPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.into(), subnetaddress, enable.into()) }
}
#[inline]
pub unsafe fn DhcpV4SetPolicyEx<P0, P4>(serveripaddress: P0, fieldsmodified: u32, globalpolicy: bool, subnetaddress: u32, policyname: P4, policy: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEx(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, globalpolicy : windows_core::BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY_EX) -> u32);
    unsafe { DhcpV4SetPolicyEx(serveripaddress.param().abi(), fieldsmodified, globalpolicy.into(), subnetaddress, policyname.param().abi(), policy) }
}
#[inline]
pub unsafe fn DhcpV6CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    unsafe { DhcpV6CreateClientInfo(serveripaddress.param().abi(), clientinfo) }
}
#[inline]
pub unsafe fn DhcpV6GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IPV6_ADDRESS, startip : DHCP_IPV6_ADDRESS, endip : DHCP_IPV6_ADDRESS, numfreeaddrreq : u32, ipaddrlist : *mut *mut DHCPV6_IP_ARRAY) -> u32);
    unsafe { DhcpV6GetFreeIPAddress(serveripaddress.param().abi(), core::mem::transmute(scopeid), core::mem::transmute(startip), core::mem::transmute(endip), numfreeaddrreq, ipaddrlist as _) }
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStatistics<P0>(serveripaddress: P0, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStatistics(serveripaddress : windows_core::PCWSTR, statelessstats : *mut *mut DHCPV6_STATELESS_STATS) -> u32);
    unsafe { DhcpV6GetStatelessStatistics(serveripaddress.param().abi(), statelessstats as _) }
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStoreParams<P0>(serveripaddress: P0, fserverlevel: bool, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : windows_core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, params : *mut *mut DHCPV6_STATELESS_PARAMS) -> u32);
    unsafe { DhcpV6GetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.into(), core::mem::transmute(subnetaddress), params as _) }
}
#[inline]
pub unsafe fn DhcpV6SetStatelessStoreParams<P0>(serveripaddress: P0, fserverlevel: bool, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpsapi.dll" "system" fn DhcpV6SetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : windows_core::BOOL, subnetaddress : DHCP_IPV6_ADDRESS, fieldmodified : u32, params : *const DHCPV6_STATELESS_PARAMS) -> u32);
    unsafe { DhcpV6SetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.into(), core::mem::transmute(subnetaddress), fieldmodified, params) }
}
#[inline]
pub unsafe fn Dhcpv6CApiCleanup() {
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6CApiCleanup());
    unsafe { Dhcpv6CApiCleanup() }
}
#[inline]
pub unsafe fn Dhcpv6CApiInitialize(version: *mut u32) {
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6CApiInitialize(version : *mut u32));
    unsafe { Dhcpv6CApiInitialize(version as _) }
}
#[inline]
pub unsafe fn Dhcpv6ReleasePrefix<P0>(adaptername: P0, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6ReleasePrefix(adaptername : windows_core::PCWSTR, classid : *mut DHCPV6CAPI_CLASSID, leaseinfo : *mut DHCPV6PrefixLeaseInformation) -> u32);
    unsafe { Dhcpv6ReleasePrefix(adaptername.param().abi(), classid as _, leaseinfo as _) }
}
#[inline]
pub unsafe fn Dhcpv6RenewPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RenewPrefix(adaptername : windows_core::PCWSTR, pclassid : *mut DHCPV6CAPI_CLASSID, prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation, pdwtimetowait : *mut u32, bvalidateprefix : u32) -> u32);
    unsafe { Dhcpv6RenewPrefix(adaptername.param().abi(), pclassid as _, prefixleaseinfo as _, pdwtimetowait as _, bvalidateprefix) }
}
#[inline]
pub unsafe fn Dhcpv6RequestParams<P2>(forcenewinform: bool, reserved: *mut core::ffi::c_void, adaptername: P2, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RequestParams(forcenewinform : windows_core::BOOL, reserved : *mut core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPV6CAPI_CLASSID, recdparams : DHCPV6CAPI_PARAMS_ARRAY, buffer : *mut u8, psize : *mut u32) -> u32);
    unsafe { Dhcpv6RequestParams(forcenewinform.into(), reserved as _, adaptername.param().abi(), classid as _, core::mem::transmute(recdparams), buffer as _, psize as _) }
}
#[inline]
pub unsafe fn Dhcpv6RequestPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RequestPrefix(adaptername : windows_core::PCWSTR, pclassid : *mut DHCPV6CAPI_CLASSID, prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation, pdwtimetowait : *mut u32) -> u32);
    unsafe { Dhcpv6RequestPrefix(adaptername.param().abi(), pclassid as _, prefixleaseinfo as _, pdwtimetowait as _) }
}
pub const ADDRESS_TYPE_IANA: u32 = 0u32;
pub const ADDRESS_TYPE_IATA: u32 = 1u32;
pub const Allow: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(1i32);
pub const CHANGESTATE: u32 = 4u32;
pub const CLIENT_TYPE_BOOTP: u32 = 2u32;
pub const CLIENT_TYPE_DHCP: u32 = 1u32;
pub const CLIENT_TYPE_NONE: u32 = 100u32;
pub const CLIENT_TYPE_RESERVATION_FLAG: u32 = 4u32;
pub const CLIENT_TYPE_UNSPECIFIED: u32 = 0u32;
pub const COMMUNICATION_INT: FSM_STATE = FSM_STATE(4i32);
pub const CONFLICT_DONE: FSM_STATE = FSM_STATE(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DATE_TIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const DEFAULTQUARSETTING: QuarantineStatus = QuarantineStatus(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: windows_core::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl Default for DHCPAPI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPCAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl Default for DHCPCAPI_CLASSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCPCAPI_DEREGISTER_HANDLE_EVENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPCAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPAPI_PARAMS,
}
impl Default for DHCPCAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCPCAPI_REGISTER_HANDLE_EVENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_ASYNCHRONOUS: u32 = 4u32;
pub const DHCPCAPI_REQUEST_CANCEL: u32 = 8u32;
pub const DHCPCAPI_REQUEST_MASK: u32 = 15u32;
pub const DHCPCAPI_REQUEST_PERSISTENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_SYNCHRONOUS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPDS_SERVER {
    pub Version: u32,
    pub ServerName: windows_core::PWSTR,
    pub ServerAddress: u32,
    pub Flags: u32,
    pub State: u32,
    pub DsLocation: windows_core::PWSTR,
    pub DsLocType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPDS_SERVERS {
    pub Flags: u32,
    pub NumElements: u32,
    pub Servers: *mut DHCPDS_SERVER,
}
impl Default for DHCPDS_SERVERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO,
}
impl Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub SentPotExpTime: u32,
    pub AckPotExpTime: u32,
    pub RecvPotExpTime: u32,
    pub StartTime: u32,
    pub CltLastTransTime: u32,
    pub LastBndUpdTime: u32,
    pub BndMsgStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Flags: u8,
    pub AddressStateEx: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6CAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl Default for DHCPV6CAPI_CLASSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6CAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: windows_core::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl Default for DHCPV6CAPI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6CAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPV6CAPI_PARAMS,
}
impl Default for DHCPV6CAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6Prefix {
    pub prefix: [u8; 16],
    pub prefixLength: u32,
    pub preferredLifeTime: u32,
    pub validLifeTime: u32,
    pub status: StatusCode,
}
impl Default for DHCPV6Prefix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6PrefixLeaseInformation {
    pub nPrefixes: u32,
    pub prefixArray: *mut DHCPV6Prefix,
    pub iaid: u32,
    pub T1: i64,
    pub T2: i64,
    pub MaxLeaseExpirationTime: i64,
    pub LastRenewalTime: i64,
    pub status: StatusCode,
    pub ServerId: *mut u8,
    pub ServerIdLen: u32,
}
impl Default for DHCPV6PrefixLeaseInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: windows_core::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: windows_core::PWSTR,
    pub IpV6IfIndex: u32,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
impl Default for DHCPV6_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCPV6_BIND_ELEMENT,
}
impl Default for DHCPV6_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IPV6_ADDRESS,
}
impl Default for DHCPV6_IP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCPV6_OPTION_CLIENTID: u32 = 1u32;
pub const DHCPV6_OPTION_DNS_SERVERS: u32 = 23u32;
pub const DHCPV6_OPTION_DOMAIN_LIST: u32 = 24u32;
pub const DHCPV6_OPTION_IA_NA: u32 = 3u32;
pub const DHCPV6_OPTION_IA_PD: u32 = 25u32;
pub const DHCPV6_OPTION_IA_TA: u32 = 4u32;
pub const DHCPV6_OPTION_NISP_DOMAIN_NAME: u32 = 30u32;
pub const DHCPV6_OPTION_NISP_SERVERS: u32 = 28u32;
pub const DHCPV6_OPTION_NIS_DOMAIN_NAME: u32 = 29u32;
pub const DHCPV6_OPTION_NIS_SERVERS: u32 = 27u32;
pub const DHCPV6_OPTION_ORO: u32 = 6u32;
pub const DHCPV6_OPTION_PREFERENCE: u32 = 7u32;
pub const DHCPV6_OPTION_RAPID_COMMIT: u32 = 14u32;
pub const DHCPV6_OPTION_RECONF_MSG: u32 = 19u32;
pub const DHCPV6_OPTION_SERVERID: u32 = 2u32;
pub const DHCPV6_OPTION_SIP_SERVERS_ADDRS: u32 = 22u32;
pub const DHCPV6_OPTION_SIP_SERVERS_NAMES: u32 = 21u32;
pub const DHCPV6_OPTION_UNICAST: u32 = 12u32;
pub const DHCPV6_OPTION_USER_CLASS: u32 = 15u32;
pub const DHCPV6_OPTION_VENDOR_CLASS: u32 = 16u32;
pub const DHCPV6_OPTION_VENDOR_OPTS: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: windows_core::BOOL,
    pub PurgeInterval: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCPV6_STATELESS_PARAM_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: u64,
    pub NumStatelessClientsRemoved: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: u32,
    pub ScopeStats: *mut DHCPV6_STATELESS_SCOPE_STATS,
}
impl Default for DHCPV6_STATELESS_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: windows_core::BOOL,
    pub HWType: u8,
    pub IsWildcard: windows_core::BOOL,
    pub Length: u8,
    pub Pattern: [u8; 255],
}
impl Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: u32,
    pub NonVendorOptions: *mut DHCP_OPTION_ARRAY,
    pub NumVendorOptions: u32,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
impl Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: windows_core::PWSTR,
    pub ClassName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
impl Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: windows_core::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
impl Default for DHCP_ALL_OPTION_VALUES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
impl Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: windows_core::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
impl Default for DHCP_ALL_OPTION_VALUES_PB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_ATTRIB {
    pub DhcpAttribId: u32,
    pub DhcpAttribType: u32,
    pub Anonymous: DHCP_ATTRIB_0,
}
impl Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: windows_core::BOOL,
    pub DhcpAttribUlong: u32,
}
impl Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: u32,
    pub DhcpAttribs: *mut DHCP_ATTRIB,
}
impl Default for DHCP_ATTRIB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5u32;
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4u32;
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2u32;
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3u32;
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2u32;
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_BINARY_DATA {
    pub DataLength: u32,
    pub Data: *mut u8,
}
impl Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: windows_core::BOOL,
    pub AdapterPrimaryAddress: u32,
    pub AdapterSubnetAddress: u32,
    pub IfDescription: windows_core::PWSTR,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
impl Default for DHCP_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_BIND_ELEMENT,
}
impl Default for DHCP_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
    pub BootpAllocated: u32,
    pub MaxBootpAllowed: u32,
}
pub const DHCP_CALLOUT_ENTRY_POINT: windows_core::PCSTR = windows_core::s!("DhcpServerCalloutEntry");
pub const DHCP_CALLOUT_LIST_KEY: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\DHCPServer\\Parameters");
pub const DHCP_CALLOUT_LIST_VALUE: windows_core::PCWSTR = windows_core::w!("CalloutDlls");
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DHCP_CALLOUT_TABLE {
    pub DhcpControlHook: LPDHCP_CONTROL,
    pub DhcpNewPktHook: LPDHCP_NEWPKT,
    pub DhcpPktDropHook: LPDHCP_DROP_SEND,
    pub DhcpPktSendHook: LPDHCP_DROP_SEND,
    pub DhcpAddressDelHook: LPDHCP_PROB,
    pub DhcpAddressOfferHook: LPDHCP_GIVE_ADDRESS,
    pub DhcpHandleOptionsHook: LPDHCP_HANDLE_OPTIONS,
    pub DhcpDeleteClientHook: LPDHCP_DELETE_CLIENT,
    pub DhcpExtensionHook: *mut core::ffi::c_void,
    pub DhcpReservedHook: *mut core::ffi::c_void,
}
impl Default for DHCP_CALLOUT_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLASS_INFO {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: windows_core::BOOL,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
impl Default for DHCP_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO,
}
impl Default for DHCP_CLASS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO_V6,
}
impl Default for DHCP_CLASS_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: windows_core::BOOL,
    pub EnterpriseNumber: u32,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
impl Default for DHCP_CLASS_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_CLIENT_BOOTP: u32 = 805306371u32;
pub const DHCP_CLIENT_DHCP: u32 = 805306372u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO,
}
impl Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO,
}
impl Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V4,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V5,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V6,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_VQ,
}
impl Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_EX {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
impl Default for DHCP_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_EX,
}
impl Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_PB {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_PB,
}
impl Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V4 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V5 {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_V6 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub AddressType: u32,
    pub IAID: u32,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientValidLeaseExpires: DATE_TIME,
    pub ClientPrefLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO_V6,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_CLIENT_INFO_VQ {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
    pub bClientType: u8,
    pub AddressState: u8,
    pub Status: QuarantineStatus,
    pub ProbationEnds: DATE_TIME,
    pub QuarantineCapable: windows_core::BOOL,
}
pub const DHCP_CONTROL_CONTINUE: u32 = 4u32;
pub const DHCP_CONTROL_PAUSE: u32 = 3u32;
pub const DHCP_CONTROL_START: u32 = 1u32;
pub const DHCP_CONTROL_STOP: u32 = 2u32;
pub const DHCP_DROP_DUPLICATE: u32 = 1u32;
pub const DHCP_DROP_GEN_FAILURE: u32 = 256u32;
pub const DHCP_DROP_INTERNAL_ERROR: u32 = 3u32;
pub const DHCP_DROP_INVALID: u32 = 8u32;
pub const DHCP_DROP_NOADDRESS: u32 = 10u32;
pub const DHCP_DROP_NOMEM: u32 = 2u32;
pub const DHCP_DROP_NO_SUBNETS: u32 = 7u32;
pub const DHCP_DROP_PAUSED: u32 = 6u32;
pub const DHCP_DROP_PROCESSED: u32 = 11u32;
pub const DHCP_DROP_TIMEOUT: u32 = 4u32;
pub const DHCP_DROP_UNAUTH: u32 = 5u32;
pub const DHCP_DROP_WRONG_SERVER: u32 = 9u32;
pub const DHCP_ENDPOINT_FLAG_CANT_MODIFY: u32 = 1u32;
pub const DHCP_FAILOVER_DELETE_SCOPES: u32 = 1u32;
pub const DHCP_FAILOVER_MAX_NUM_ADD_SCOPES: u32 = 400u32;
pub const DHCP_FAILOVER_MAX_NUM_REL: u32 = 31u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_FAILOVER_MODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_FAILOVER_RELATIONSHIP {
    pub PrimaryServer: u32,
    pub SecondaryServer: u32,
    pub Mode: DHCP_FAILOVER_MODE,
    pub ServerType: DHCP_FAILOVER_SERVER,
    pub State: FSM_STATE,
    pub PrevState: FSM_STATE,
    pub Mclt: u32,
    pub SafePeriod: u32,
    pub RelationshipName: windows_core::PWSTR,
    pub PrimaryServerName: windows_core::PWSTR,
    pub SecondaryServerName: windows_core::PWSTR,
    pub pScopes: *mut DHCP_IP_ARRAY,
    pub Percentage: u8,
    pub SharedSecret: windows_core::PWSTR,
}
impl Default for DHCP_FAILOVER_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: u32,
    pub pRelationships: *mut DHCP_FAILOVER_RELATIONSHIP,
}
impl Default for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_FAILOVER_SERVER(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: u32,
    pub AddrFree: u32,
    pub AddrInUse: u32,
    pub PartnerAddrFree: u32,
    pub ThisAddrFree: u32,
    pub PartnerAddrInUse: u32,
    pub ThisAddrInUse: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: u32,
    pub pEnumRecords: *mut DHCP_FILTER_RECORD,
}
impl Default for DHCP_FILTER_ENUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: windows_core::BOOL,
    pub EnforceDenyList: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_FILTER_LIST_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
}
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1u32;
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2u32;
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_FORCE_FLAG(pub i32);
pub const DHCP_GIVE_ADDRESS_NEW: u32 = 805306369u32;
pub const DHCP_GIVE_ADDRESS_OLD: u32 = 805306370u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_HOST_INFO {
    pub IpAddress: u32,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: u64,
    pub LowOrderBits: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut u32,
}
impl Default for DHCP_IP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: u32,
    pub ClusterMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IP_RANGE,
}
impl Default for DHCP_IP_RANGE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
}
impl Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: DHCP_BINARY_DATA,
    pub ReservedClientName: windows_core::PWSTR,
    pub ReservedClientDesc: windows_core::PWSTR,
    pub bAllowedClientTypes: u8,
    pub fOptionsPresent: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub bAllowedClientTypes: u8,
}
impl Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub InterfaceId: u32,
}
impl Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_MAX_DELAY: u32 = 1000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_MIB_INFO {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO,
}
impl Default for DHCP_MIB_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_MIB_INFO_V5 {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub DelayedOffers: u32,
    pub ScopesWithDelayedOffers: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V5,
}
impl Default for DHCP_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_MIB_INFO_V6 {
    pub Solicits: u32,
    pub Advertises: u32,
    pub Requests: u32,
    pub Renews: u32,
    pub Rebinds: u32,
    pub Replies: u32,
    pub Confirms: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub Informs: u32,
    pub ServerStartTime: DATE_TIME,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_V6,
}
impl Default for DHCP_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_MIB_INFO_VQ {
    pub Discovers: u32,
    pub Offers: u32,
    pub Requests: u32,
    pub Acks: u32,
    pub Naks: u32,
    pub Declines: u32,
    pub Releases: u32,
    pub ServerStartTime: DATE_TIME,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
    pub QtnIASErrors: u32,
    pub Scopes: u32,
    pub ScopeInfo: *mut SCOPE_MIB_INFO_VQ,
}
impl Default for DHCP_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_MIN_DELAY: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION {
    pub OptionID: u32,
    pub OptionName: windows_core::PWSTR,
    pub OptionComment: windows_core::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: u32,
    pub Options: *mut DHCP_OPTION,
}
impl Default for DHCP_OPTION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_OPTION_DATA {
    pub NumElements: u32,
    pub Elements: *mut DHCP_OPTION_DATA_ELEMENT,
}
impl Default for DHCP_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_DATA_ELEMENT {
    pub OptionType: DHCP_OPTION_DATA_TYPE,
    pub Element: DHCP_OPTION_DATA_ELEMENT_0,
}
impl Default for DHCP_OPTION_DATA_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_DATA_ELEMENT_0 {
    pub ByteOption: u8,
    pub WordOption: u16,
    pub DWordOption: u32,
    pub DWordDWordOption: DWORD_DWORD,
    pub IpAddressOption: u32,
    pub StringDataOption: windows_core::PWSTR,
    pub BinaryDataOption: DHCP_BINARY_DATA,
    pub EncapsulatedDataOption: DHCP_BINARY_DATA,
    pub Ipv6AddressDataOption: windows_core::PWSTR,
}
impl Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_OPTION_DATA_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: u32,
    pub Options: *mut DHCP_OPTION_VALUE,
}
impl Default for DHCP_OPTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO_0,
}
impl Default for DHCP_OPTION_SCOPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO_0 {
    pub DefaultScopeInfo: *mut core::ffi::c_void,
    pub GlobalScopeInfo: *mut core::ffi::c_void,
    pub SubnetScopeInfo: u32,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE,
    pub MScopeInfo: windows_core::PWSTR,
}
impl Default for DHCP_OPTION_SCOPE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_OPTION_SCOPE_INFO6 {
    pub ScopeType: DHCP_OPTION_SCOPE_TYPE6,
    pub ScopeInfo: DHCP_OPTION_SCOPE_INFO6_0,
}
impl Default for DHCP_OPTION_SCOPE_INFO6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_OPTION_SCOPE_INFO6_0 {
    pub DefaultScopeInfo: *mut core::ffi::c_void,
    pub SubnetScopeInfo: DHCP_IPV6_ADDRESS,
    pub ReservedScopeInfo: DHCP_RESERVED_SCOPE6,
}
impl Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_OPTION_SCOPE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_OPTION_SCOPE_TYPE6(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_OPTION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: u32,
    pub Value: DHCP_OPTION_DATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: u32,
    pub Values: *mut DHCP_OPTION_VALUE,
}
impl Default for DHCP_OPTION_VALUE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1u32;
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_PERF_STATS {
    pub dwNumPacketsReceived: u32,
    pub dwNumPacketsDuplicate: u32,
    pub dwNumPacketsExpired: u32,
    pub dwNumMilliSecondsProcessed: u32,
    pub dwNumPacketsInActiveQueue: u32,
    pub dwNumPacketsInPingQueue: u32,
    pub dwNumDiscoversReceived: u32,
    pub dwNumOffersSent: u32,
    pub dwNumRequestsReceived: u32,
    pub dwNumInformsReceived: u32,
    pub dwNumAcksSent: u32,
    pub dwNumNacksSent: u32,
    pub dwNumDeclinesReceived: u32,
    pub dwNumReleasesReceived: u32,
    pub dwNumDelayedOfferInQueue: u32,
    pub dwNumPacketsProcessed: u32,
    pub dwNumPacketsInQuarWaitingQueue: u32,
    pub dwNumPacketsInQuarReadyQueue: u32,
    pub dwNumPacketsInQuarDecisionQueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POLICY {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: windows_core::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: windows_core::BOOL,
}
impl Default for DHCP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY,
}
impl Default for DHCP_POLICY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POLICY_EX {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: windows_core::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: windows_core::BOOL,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
impl Default for DHCP_POLICY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY_EX,
}
impl Default for DHCP_POLICY_EX_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_POLICY_FIELDS_TO_UPDATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_POL_ATTR_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_POL_COMPARATOR(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POL_COND {
    pub ParentExpr: u32,
    pub Type: DHCP_POL_ATTR_TYPE,
    pub OptionID: u32,
    pub SubOptionID: u32,
    pub VendorName: windows_core::PWSTR,
    pub Operator: DHCP_POL_COMPARATOR,
    pub Value: *mut u8,
    pub ValueLength: u32,
}
impl Default for DHCP_POL_COND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_COND,
}
impl Default for DHCP_POL_COND_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: u32,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_EXPR,
}
impl Default for DHCP_POL_EXPR_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_POL_LOGIC_OPER(pub i32);
pub const DHCP_PROB_CONFLICT: u32 = 536870913u32;
pub const DHCP_PROB_DECLINE: u32 = 536870914u32;
pub const DHCP_PROB_NACKED: u32 = 536870916u32;
pub const DHCP_PROB_RELEASE: u32 = 536870915u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
impl Default for DHCP_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_PROPERTY_0 {
    pub ByteValue: u8,
    pub WordValue: u16,
    pub DWordValue: u32,
    pub StringValue: windows_core::PWSTR,
    pub BinaryValue: DHCP_BINARY_DATA,
}
impl Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_PROPERTY,
}
impl Default for DHCP_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_PROPERTY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut *mut DHCP_IP_RESERVATION_INFO,
}
impl Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: u32,
    pub ReservedIpSubnetAddress: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SCAN_FLAG(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: u32,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: u32,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
impl Default for DHCP_SCAN_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO {
    pub SearchType: DHCP_SEARCH_INFO_TYPE,
    pub SearchInfo: DHCP_SEARCH_INFO_0,
}
impl Default for DHCP_SEARCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_0 {
    pub ClientIpAddress: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
}
impl Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SEARCH_INFO_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SEARCH_INFO_TYPE_V6(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
impl Default for DHCP_SEARCH_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SEARCH_INFO_V6_0 {
    pub ClientIpAddress: DHCP_IPV6_ADDRESS,
    pub ClientDUID: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
}
impl Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DHCP_SEND_PACKET: u32 = 268435456u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_V4 {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: windows_core::PWSTR,
    pub fAuditLog: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: windows_core::BOOL,
    pub RapidCommitFlag: windows_core::BOOL,
    pub PreferredLifetime: u32,
    pub ValidLifetime: u32,
    pub T1: u32,
    pub T2: u32,
    pub PreferredLifetimeIATA: u32,
    pub ValidLifetimeIATA: u32,
    pub fAuditLog: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_VQ {
    pub APIProtocolSupport: u32,
    pub DatabaseName: windows_core::PWSTR,
    pub DatabasePath: windows_core::PWSTR,
    pub BackupPath: windows_core::PWSTR,
    pub BackupInterval: u32,
    pub DatabaseLoggingFlag: u32,
    pub RestoreFlag: u32,
    pub DatabaseCleanupInterval: u32,
    pub DebugFlag: u32,
    pub dwPingRetries: u32,
    pub cbBootTableString: u32,
    pub wszBootTableString: windows_core::PWSTR,
    pub fAuditLog: windows_core::BOOL,
    pub QuarantineOn: windows_core::BOOL,
    pub QuarDefFail: u32,
    pub QuarRuntimeStatus: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SERVER_OPTIONS {
    pub MessageType: *mut u8,
    pub SubnetMask: *mut u32,
    pub RequestedAddress: *mut u32,
    pub RequestLeaseTime: *mut u32,
    pub OverlayFields: *mut u8,
    pub RouterAddress: *mut u32,
    pub Server: *mut u32,
    pub ParameterRequestList: *mut u8,
    pub ParameterRequestListLength: u32,
    pub MachineName: windows_core::PSTR,
    pub MachineNameLength: u32,
    pub ClientHardwareAddressType: u8,
    pub ClientHardwareAddressLength: u8,
    pub ClientHardwareAddress: *mut u8,
    pub ClassIdentifier: windows_core::PSTR,
    pub ClassIdentifierLength: u32,
    pub VendorClass: *mut u8,
    pub VendorClassLength: u32,
    pub DNSFlags: u32,
    pub DNSNameLength: u32,
    pub DNSName: *mut u8,
    pub DSDomainNameRequested: bool,
    pub DSDomainName: windows_core::PSTR,
    pub DSDomainNameLen: u32,
    pub ScopeId: *mut u32,
}
impl Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: windows_core::PWSTR,
    pub DefaultUserClassName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V4 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V4_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    pub IpRange: *mut DHCP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V5 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V5_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    pub IpRange: *mut DHCP_BOOTP_IP_RANGE,
    pub SecondaryHost: *mut DHCP_HOST_INFO,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V4,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE,
    pub IpUsedCluster: *mut DHCP_IP_CLUSTER,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA_V6 {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE_V6,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_V6_0,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    pub IpRange: *mut DHCP_IP_RANGE_V6,
    pub ReservedIp: *mut DHCP_IP_RESERVATION_V6,
    pub ExcludeIpRange: *mut DHCP_IP_RANGE_V6,
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA,
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V4,
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V5,
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V6,
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_TYPE_V6(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: u32,
    pub Preference: u16,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub State: u32,
    pub ScopeId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUBNET_INFO_VQ {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
    pub QuarantineOn: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: i64,
    pub Reserved4: i64,
}
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DHCP_SUBNET_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: u32,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
impl Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: u32,
    pub SuperScopeNumber: u32,
    pub NextInSuperScope: u32,
    pub SuperScopeName: windows_core::PWSTR,
}
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4u32;
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64u32;
pub const DNS_FLAG_ENABLED: u32 = 1u32;
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128u32;
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16u32;
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32u32;
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2u32;
pub const DROPPACKET: QuarantineStatus = QuarantineStatus(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWORD_DWORD {
    pub DWord1: u32,
    pub DWord2: u32,
}
pub const Deny: DHCP_FILTER_LIST_TYPE = DHCP_FILTER_LIST_TYPE(0i32);
pub const DhcpArrayTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(1i32);
pub const DhcpAttrFqdn: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(3i32);
pub const DhcpAttrFqdnSingleLabel: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(4i32);
pub const DhcpAttrHWAddr: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(0i32);
pub const DhcpAttrOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(1i32);
pub const DhcpAttrSubOption: DHCP_POL_ATTR_TYPE = DHCP_POL_ATTR_TYPE(2i32);
pub const DhcpBinaryDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(6i32);
pub const DhcpByteOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(0i32);
pub const DhcpClientHardwareAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(1i32);
pub const DhcpClientIpAddress: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(0i32);
pub const DhcpClientName: DHCP_SEARCH_INFO_TYPE = DHCP_SEARCH_INFO_TYPE(2i32);
pub const DhcpCompBeginsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(2i32);
pub const DhcpCompEndsWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(4i32);
pub const DhcpCompEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(0i32);
pub const DhcpCompNotBeginWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(3i32);
pub const DhcpCompNotEndWith: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(5i32);
pub const DhcpCompNotEqual: DHCP_POL_COMPARATOR = DHCP_POL_COMPARATOR(1i32);
pub const DhcpDWordDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(3i32);
pub const DhcpDWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(2i32);
pub const DhcpDatabaseFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(1i32);
pub const DhcpDefaultOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(0i32);
pub const DhcpDefaultOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(0i32);
pub const DhcpEncapsulatedDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(7i32);
pub const DhcpExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(3i32);
pub const DhcpFailoverForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(2i32);
pub const DhcpFullForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(0i32);
pub const DhcpGlobalOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(1i32);
pub const DhcpGlobalOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(3i32);
pub const DhcpIpAddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(4i32);
pub const DhcpIpRanges: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(0i32);
pub const DhcpIpRangesBootpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(7i32);
pub const DhcpIpRangesDhcpBootp: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(6i32);
pub const DhcpIpRangesDhcpOnly: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(5i32);
pub const DhcpIpUsedClusters: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(4i32);
pub const DhcpIpv6AddressOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(8i32);
pub const DhcpLogicalAnd: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(1i32);
pub const DhcpLogicalOr: DHCP_POL_LOGIC_OPER = DHCP_POL_LOGIC_OPER(0i32);
pub const DhcpMScopeOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(4i32);
pub const DhcpNoForce: DHCP_FORCE_FLAG = DHCP_FORCE_FLAG(1i32);
pub const DhcpPropIdClientAddressStateEx: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(1i32);
pub const DhcpPropIdPolicyDnsSuffix: DHCP_PROPERTY_ID = DHCP_PROPERTY_ID(0i32);
pub const DhcpPropTypeBinary: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(4i32);
pub const DhcpPropTypeByte: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(0i32);
pub const DhcpPropTypeDword: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(2i32);
pub const DhcpPropTypeString: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(3i32);
pub const DhcpPropTypeWord: DHCP_PROPERTY_TYPE = DHCP_PROPERTY_TYPE(1i32);
pub const DhcpRegistryFix: DHCP_SCAN_FLAG = DHCP_SCAN_FLAG(0i32);
pub const DhcpReservedIps: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(2i32);
pub const DhcpReservedOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(3i32);
pub const DhcpReservedOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(2i32);
pub const DhcpScopeOptions6: DHCP_OPTION_SCOPE_TYPE6 = DHCP_OPTION_SCOPE_TYPE6(1i32);
pub const DhcpSecondaryHosts: DHCP_SUBNET_ELEMENT_TYPE = DHCP_SUBNET_ELEMENT_TYPE(1i32);
pub const DhcpStatelessPurgeInterval: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(1i32);
pub const DhcpStatelessStatus: DHCPV6_STATELESS_PARAM_TYPE = DHCPV6_STATELESS_PARAM_TYPE(2i32);
pub const DhcpStringDataOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(5i32);
pub const DhcpSubnetDisabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(1i32);
pub const DhcpSubnetDisabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(3i32);
pub const DhcpSubnetEnabled: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(0i32);
pub const DhcpSubnetEnabledSwitched: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(2i32);
pub const DhcpSubnetInvalidState: DHCP_SUBNET_STATE = DHCP_SUBNET_STATE(4i32);
pub const DhcpSubnetOptions: DHCP_OPTION_SCOPE_TYPE = DHCP_OPTION_SCOPE_TYPE(2i32);
pub const DhcpUnaryElementTypeOption: DHCP_OPTION_TYPE = DHCP_OPTION_TYPE(0i32);
pub const DhcpUpdatePolicyDescr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(16i32);
pub const DhcpUpdatePolicyDnsSuffix: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(64i32);
pub const DhcpUpdatePolicyExpr: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(4i32);
pub const DhcpUpdatePolicyName: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(1i32);
pub const DhcpUpdatePolicyOrder: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(2i32);
pub const DhcpUpdatePolicyRanges: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(8i32);
pub const DhcpUpdatePolicyStatus: DHCP_POLICY_FIELDS_TO_UPDATE = DHCP_POLICY_FIELDS_TO_UPDATE(32i32);
pub const DhcpWordOption: DHCP_OPTION_DATA_TYPE = DHCP_OPTION_DATA_TYPE(1i32);
pub const Dhcpv6ClientDUID: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(1i32);
pub const Dhcpv6ClientIpAddress: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(0i32);
pub const Dhcpv6ClientName: DHCP_SEARCH_INFO_TYPE_V6 = DHCP_SEARCH_INFO_TYPE_V6(2i32);
pub const Dhcpv6ExcludedIpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(2i32);
pub const Dhcpv6IpRanges: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(0i32);
pub const Dhcpv6ReservedIps: DHCP_SUBNET_ELEMENT_TYPE_V6 = DHCP_SUBNET_ELEMENT_TYPE_V6(1i32);
pub const ERROR_DDS_CLASS_DOES_NOT_EXIST: u32 = 20078u32;
pub const ERROR_DDS_CLASS_EXISTS: u32 = 20077u32;
pub const ERROR_DDS_DHCP_SERVER_NOT_FOUND: u32 = 20074u32;
pub const ERROR_DDS_NO_DHCP_ROOT: u32 = 20071u32;
pub const ERROR_DDS_NO_DS_AVAILABLE: u32 = 20070u32;
pub const ERROR_DDS_OPTION_ALREADY_EXISTS: u32 = 20075u32;
pub const ERROR_DDS_OPTION_DOES_NOT_EXIST: u32 = 20076u32;
pub const ERROR_DDS_POSSIBLE_RANGE_CONFLICT: u32 = 20087u32;
pub const ERROR_DDS_RANGE_DOES_NOT_EXIST: u32 = 20088u32;
pub const ERROR_DDS_RESERVATION_CONFLICT: u32 = 20086u32;
pub const ERROR_DDS_RESERVATION_NOT_PRESENT: u32 = 20085u32;
pub const ERROR_DDS_SERVER_ADDRESS_MISMATCH: u32 = 20081u32;
pub const ERROR_DDS_SERVER_ALREADY_EXISTS: u32 = 20079u32;
pub const ERROR_DDS_SERVER_DOES_NOT_EXIST: u32 = 20080u32;
pub const ERROR_DDS_SUBNET_EXISTS: u32 = 20082u32;
pub const ERROR_DDS_SUBNET_HAS_DIFF_SSCOPE: u32 = 20083u32;
pub const ERROR_DDS_SUBNET_NOT_PRESENT: u32 = 20084u32;
pub const ERROR_DDS_TOO_MANY_ERRORS: u32 = 20073u32;
pub const ERROR_DDS_UNEXPECTED_ERROR: u32 = 20072u32;
pub const ERROR_DHCP_ADDRESS_NOT_AVAILABLE: u32 = 20011u32;
pub const ERROR_DHCP_CANNOT_MODIFY_BINDINGS: u32 = 20051u32;
pub const ERROR_DHCP_CANT_CHANGE_ATTRIBUTE: u32 = 20048u32;
pub const ERROR_DHCP_CLASS_ALREADY_EXISTS: u32 = 20045u32;
pub const ERROR_DHCP_CLASS_NOT_FOUND: u32 = 20044u32;
pub const ERROR_DHCP_CLIENT_EXISTS: u32 = 20014u32;
pub const ERROR_DHCP_DATABASE_INIT_FAILED: u32 = 20001u32;
pub const ERROR_DHCP_DEFAULT_SCOPE_EXITS: u32 = 20047u32;
pub const ERROR_DHCP_DELETE_BUILTIN_CLASS: u32 = 20089u32;
pub const ERROR_DHCP_ELEMENT_CANT_REMOVE: u32 = 20007u32;
pub const ERROR_DHCP_EXEMPTION_EXISTS: u32 = 20055u32;
pub const ERROR_DHCP_EXEMPTION_NOT_PRESENT: u32 = 20056u32;
pub const ERROR_DHCP_FO_ADDSCOPE_LEASES_NOT_SYNCED: u32 = 20127u32;
pub const ERROR_DHCP_FO_BOOT_NOT_SUPPORTED: u32 = 20131u32;
pub const ERROR_DHCP_FO_FEATURE_NOT_SUPPORTED: u32 = 20134u32;
pub const ERROR_DHCP_FO_IPRANGE_TYPE_CONV_ILLEGAL: u32 = 20129u32;
pub const ERROR_DHCP_FO_MAX_ADD_SCOPES: u32 = 20130u32;
pub const ERROR_DHCP_FO_MAX_RELATIONSHIPS: u32 = 20128u32;
pub const ERROR_DHCP_FO_NOT_SUPPORTED: u32 = 20118u32;
pub const ERROR_DHCP_FO_RANGE_PART_OF_REL: u32 = 20132u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_DOES_NOT_EXIST: u32 = 20115u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_EXISTS: u32 = 20114u32;
pub const ERROR_DHCP_FO_RELATIONSHIP_NAME_TOO_LONG: u32 = 20125u32;
pub const ERROR_DHCP_FO_RELATION_IS_SECONDARY: u32 = 20117u32;
pub const ERROR_DHCP_FO_SCOPE_ALREADY_IN_RELATIONSHIP: u32 = 20113u32;
pub const ERROR_DHCP_FO_SCOPE_NOT_IN_RELATIONSHIP: u32 = 20116u32;
pub const ERROR_DHCP_FO_SCOPE_SYNC_IN_PROGRESS: u32 = 20133u32;
pub const ERROR_DHCP_FO_STATE_NOT_NORMAL: u32 = 20120u32;
pub const ERROR_DHCP_FO_TIME_OUT_OF_SYNC: u32 = 20119u32;
pub const ERROR_DHCP_HARDWARE_ADDRESS_TYPE_ALREADY_EXEMPT: u32 = 20101u32;
pub const ERROR_DHCP_INVALID_DELAY: u32 = 20092u32;
pub const ERROR_DHCP_INVALID_DHCP_CLIENT: u32 = 20016u32;
pub const ERROR_DHCP_INVALID_DHCP_MESSAGE: u32 = 20015u32;
pub const ERROR_DHCP_INVALID_PARAMETER_OPTION32: u32 = 20057u32;
pub const ERROR_DHCP_INVALID_POLICY_EXPRESSION: u32 = 20109u32;
pub const ERROR_DHCP_INVALID_PROCESSING_ORDER: u32 = 20110u32;
pub const ERROR_DHCP_INVALID_RANGE: u32 = 20023u32;
pub const ERROR_DHCP_INVALID_SUBNET_PREFIX: u32 = 20091u32;
pub const ERROR_DHCP_IPRANGE_CONV_ILLEGAL: u32 = 20049u32;
pub const ERROR_DHCP_IPRANGE_EXITS: u32 = 20021u32;
pub const ERROR_DHCP_IP_ADDRESS_IN_USE: u32 = 20032u32;
pub const ERROR_DHCP_JET97_CONV_REQUIRED: u32 = 20036u32;
pub const ERROR_DHCP_JET_CONV_REQUIRED: u32 = 20027u32;
pub const ERROR_DHCP_JET_ERROR: u32 = 20013u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_DOES_NOT_EXIST: u32 = 20095u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_EXISTS: u32 = 20093u32;
pub const ERROR_DHCP_LINKLAYER_ADDRESS_RESERVATION_EXISTS: u32 = 20094u32;
pub const ERROR_DHCP_LOG_FILE_PATH_TOO_LONG: u32 = 20033u32;
pub const ERROR_DHCP_MSCOPE_EXISTS: u32 = 20053u32;
pub const ERROR_DHCP_NAP_NOT_SUPPORTED: u32 = 20138u32;
pub const ERROR_DHCP_NETWORK_CHANGED: u32 = 20050u32;
pub const ERROR_DHCP_NETWORK_INIT_FAILED: u32 = 20003u32;
pub const ERROR_DHCP_NOT_RESERVED_CLIENT: u32 = 20018u32;
pub const ERROR_DHCP_NO_ADMIN_PERMISSION: u32 = 20121u32;
pub const ERROR_DHCP_OPTION_EXITS: u32 = 20009u32;
pub const ERROR_DHCP_OPTION_NOT_PRESENT: u32 = 20010u32;
pub const ERROR_DHCP_OPTION_TYPE_MISMATCH: u32 = 20103u32;
pub const ERROR_DHCP_POLICY_BAD_PARENT_EXPR: u32 = 20104u32;
pub const ERROR_DHCP_POLICY_EDIT_FQDN_UNSUPPORTED: u32 = 20137u32;
pub const ERROR_DHCP_POLICY_EXISTS: u32 = 20105u32;
pub const ERROR_DHCP_POLICY_FQDN_OPTION_UNSUPPORTED: u32 = 20136u32;
pub const ERROR_DHCP_POLICY_FQDN_RANGE_UNSUPPORTED: u32 = 20135u32;
pub const ERROR_DHCP_POLICY_NOT_FOUND: u32 = 20111u32;
pub const ERROR_DHCP_POLICY_RANGE_BAD: u32 = 20107u32;
pub const ERROR_DHCP_POLICY_RANGE_EXISTS: u32 = 20106u32;
pub const ERROR_DHCP_PRIMARY_NOT_FOUND: u32 = 20006u32;
pub const ERROR_DHCP_RANGE_EXTENDED: u32 = 20024u32;
pub const ERROR_DHCP_RANGE_FULL: u32 = 20012u32;
pub const ERROR_DHCP_RANGE_INVALID_IN_SERVER_POLICY: u32 = 20108u32;
pub const ERROR_DHCP_RANGE_TOO_SMALL: u32 = 20020u32;
pub const ERROR_DHCP_REACHED_END_OF_SELECTION: u32 = 20126u32;
pub const ERROR_DHCP_REGISTRY_INIT_FAILED: u32 = 20000u32;
pub const ERROR_DHCP_RESERVEDIP_EXITS: u32 = 20022u32;
pub const ERROR_DHCP_RESERVED_CLIENT: u32 = 20019u32;
pub const ERROR_DHCP_ROGUE_DS_CONFLICT: u32 = 20041u32;
pub const ERROR_DHCP_ROGUE_DS_UNREACHABLE: u32 = 20040u32;
pub const ERROR_DHCP_ROGUE_INIT_FAILED: u32 = 20037u32;
pub const ERROR_DHCP_ROGUE_NOT_AUTHORIZED: u32 = 20039u32;
pub const ERROR_DHCP_ROGUE_NOT_OUR_ENTERPRISE: u32 = 20042u32;
pub const ERROR_DHCP_ROGUE_SAMSHUTDOWN: u32 = 20038u32;
pub const ERROR_DHCP_ROGUE_STANDALONE_IN_DS: u32 = 20043u32;
pub const ERROR_DHCP_RPC_INIT_FAILED: u32 = 20002u32;
pub const ERROR_DHCP_SCOPE_NAME_TOO_LONG: u32 = 20046u32;
pub const ERROR_DHCP_SERVER_NAME_NOT_RESOLVED: u32 = 20124u32;
pub const ERROR_DHCP_SERVER_NOT_REACHABLE: u32 = 20122u32;
pub const ERROR_DHCP_SERVER_NOT_RUNNING: u32 = 20123u32;
pub const ERROR_DHCP_SERVICE_PAUSED: u32 = 20017u32;
pub const ERROR_DHCP_SUBNET_EXISTS: u32 = 20052u32;
pub const ERROR_DHCP_SUBNET_EXITS: u32 = 20004u32;
pub const ERROR_DHCP_SUBNET_NOT_PRESENT: u32 = 20005u32;
pub const ERROR_DHCP_SUPER_SCOPE_NAME_TOO_LONG: u32 = 20030u32;
pub const ERROR_DHCP_UNDEFINED_HARDWARE_ADDRESS_TYPE: u32 = 20102u32;
pub const ERROR_DHCP_UNSUPPORTED_CLIENT: u32 = 20034u32;
pub const ERROR_EXTEND_TOO_SMALL: u32 = 20025u32;
pub const ERROR_LAST_DHCP_SERVER_ERROR: u32 = 20139u32;
pub const ERROR_MSCOPE_RANGE_TOO_SMALL: u32 = 20054u32;
pub const ERROR_SCOPE_RANGE_POLICY_RANGE_CONFLICT: u32 = 20112u32;
pub const ERROR_SERVER_INVALID_BOOT_FILE_TABLE: u32 = 20028u32;
pub const ERROR_SERVER_UNKNOWN_BOOT_FILE_NAME: u32 = 20029u32;
pub const EXEMPT: QuarantineStatus = QuarantineStatus(4i32);
pub const FILTER_STATUS_FULL_MATCH_IN_ALLOW_LIST: u32 = 2u32;
pub const FILTER_STATUS_FULL_MATCH_IN_DENY_LIST: u32 = 4u32;
pub const FILTER_STATUS_NONE: u32 = 1u32;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_ALLOW_LIST: u32 = 8u32;
pub const FILTER_STATUS_WILDCARD_MATCH_IN_DENY_LIST: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FSM_STATE(pub i32);
pub const HWTYPE_ETHERNET_10MB: u32 = 1u32;
pub const HotStandby: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(1i32);
pub const INIT: FSM_STATE = FSM_STATE(1i32);
pub type LPDHCP_CONTROL = Option<unsafe extern "system" fn(dwcontrolcode: u32, lpreserved: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_DELETE_CLIENT = Option<unsafe extern "system" fn(ipaddress: u32, hwaddress: *mut u8, hwaddresslength: u32, reserved: u32, clienttype: u32) -> u32>;
pub type LPDHCP_DROP_SEND = Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, controlcode: u32, ipaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_ENTRY_POINT_FUNC = Option<unsafe extern "system" fn(chaindlls: windows_core::PCWSTR, calloutversion: u32, callouttbl: *mut DHCP_CALLOUT_TABLE) -> u32>;
pub type LPDHCP_GIVE_ADDRESS = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, addrtype: u32, leasetime: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_HANDLE_OPTIONS = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void, serveroptions: *mut DHCP_SERVER_OPTIONS) -> u32>;
pub type LPDHCP_NEWPKT = Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, ipaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut *mut core::ffi::c_void, processit: *mut windows_core::BOOL) -> u32>;
pub type LPDHCP_PROB = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
pub const LoadBalance: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(0i32);
pub const MAC_ADDRESS_LENGTH: u32 = 6u32;
pub const MAX_PATTERN_LENGTH: u32 = 255u32;
pub const MCLT: u32 = 1u32;
pub const MODE: u32 = 16u32;
pub const NOQUARANTINE: QuarantineStatus = QuarantineStatus(0i32);
pub const NOQUARINFO: QuarantineStatus = QuarantineStatus(6i32);
pub const NORMAL: FSM_STATE = FSM_STATE(3i32);
pub const NO_STATE: FSM_STATE = FSM_STATE(0i32);
pub const OPTION_ALL_SUBNETS_MTU: u32 = 27u32;
pub const OPTION_ARP_CACHE_TIMEOUT: u32 = 35u32;
pub const OPTION_BE_A_MASK_SUPPLIER: u32 = 30u32;
pub const OPTION_BE_A_ROUTER: u32 = 19u32;
pub const OPTION_BOOTFILE_NAME: u32 = 67u32;
pub const OPTION_BOOT_FILE_SIZE: u32 = 13u32;
pub const OPTION_BROADCAST_ADDRESS: u32 = 28u32;
pub const OPTION_CLIENT_CLASS_INFO: u32 = 60u32;
pub const OPTION_CLIENT_ID: u32 = 61u32;
pub const OPTION_COOKIE_SERVERS: u32 = 8u32;
pub const OPTION_DEFAULT_TTL: u32 = 23u32;
pub const OPTION_DOMAIN_NAME: u32 = 15u32;
pub const OPTION_DOMAIN_NAME_SERVERS: u32 = 6u32;
pub const OPTION_END: u32 = 255u32;
pub const OPTION_ETHERNET_ENCAPSULATION: u32 = 36u32;
pub const OPTION_EXTENSIONS_PATH: u32 = 18u32;
pub const OPTION_HOST_NAME: u32 = 12u32;
pub const OPTION_IEN116_NAME_SERVERS: u32 = 5u32;
pub const OPTION_IMPRESS_SERVERS: u32 = 10u32;
pub const OPTION_KEEP_ALIVE_DATA_SIZE: u32 = 39u32;
pub const OPTION_KEEP_ALIVE_INTERVAL: u32 = 38u32;
pub const OPTION_LEASE_TIME: u32 = 51u32;
pub const OPTION_LOG_SERVERS: u32 = 7u32;
pub const OPTION_LPR_SERVERS: u32 = 9u32;
pub const OPTION_MAX_REASSEMBLY_SIZE: u32 = 22u32;
pub const OPTION_MERIT_DUMP_FILE: u32 = 14u32;
pub const OPTION_MESSAGE: u32 = 56u32;
pub const OPTION_MESSAGE_LENGTH: u32 = 57u32;
pub const OPTION_MESSAGE_TYPE: u32 = 53u32;
pub const OPTION_MSFT_IE_PROXY: u32 = 252u32;
pub const OPTION_MTU: u32 = 26u32;
pub const OPTION_NETBIOS_DATAGRAM_SERVER: u32 = 45u32;
pub const OPTION_NETBIOS_NAME_SERVER: u32 = 44u32;
pub const OPTION_NETBIOS_NODE_TYPE: u32 = 46u32;
pub const OPTION_NETBIOS_SCOPE_OPTION: u32 = 47u32;
pub const OPTION_NETWORK_INFO_SERVERS: u32 = 41u32;
pub const OPTION_NETWORK_INFO_SERVICE_DOM: u32 = 40u32;
pub const OPTION_NETWORK_TIME_SERVERS: u32 = 42u32;
pub const OPTION_NON_LOCAL_SOURCE_ROUTING: u32 = 20u32;
pub const OPTION_OK_TO_OVERLAY: u32 = 52u32;
pub const OPTION_PAD: u32 = 0u32;
pub const OPTION_PARAMETER_REQUEST_LIST: u32 = 55u32;
pub const OPTION_PERFORM_MASK_DISCOVERY: u32 = 29u32;
pub const OPTION_PERFORM_ROUTER_DISCOVERY: u32 = 31u32;
pub const OPTION_PMTU_AGING_TIMEOUT: u32 = 24u32;
pub const OPTION_PMTU_PLATEAU_TABLE: u32 = 25u32;
pub const OPTION_POLICY_FILTER_FOR_NLSR: u32 = 21u32;
pub const OPTION_REBIND_TIME: u32 = 59u32;
pub const OPTION_RENEWAL_TIME: u32 = 58u32;
pub const OPTION_REQUESTED_ADDRESS: u32 = 50u32;
pub const OPTION_RLP_SERVERS: u32 = 11u32;
pub const OPTION_ROOT_DISK: u32 = 17u32;
pub const OPTION_ROUTER_ADDRESS: u32 = 3u32;
pub const OPTION_ROUTER_SOLICITATION_ADDR: u32 = 32u32;
pub const OPTION_SERVER_IDENTIFIER: u32 = 54u32;
pub const OPTION_STATIC_ROUTES: u32 = 33u32;
pub const OPTION_SUBNET_MASK: u32 = 1u32;
pub const OPTION_SWAP_SERVER: u32 = 16u32;
pub const OPTION_TFTP_SERVER_NAME: u32 = 66u32;
pub const OPTION_TIME_OFFSET: u32 = 2u32;
pub const OPTION_TIME_SERVERS: u32 = 4u32;
pub const OPTION_TRAILERS: u32 = 34u32;
pub const OPTION_TTL: u32 = 37u32;
pub const OPTION_VENDOR_SPEC_INFO: u32 = 43u32;
pub const OPTION_XWINDOW_DISPLAY_MANAGER: u32 = 49u32;
pub const OPTION_XWINDOW_FONT_SERVER: u32 = 48u32;
pub const PARTNER_DOWN: FSM_STATE = FSM_STATE(5i32);
pub const PAUSED: FSM_STATE = FSM_STATE(12i32);
pub const PERCENTAGE: u32 = 8u32;
pub const POTENTIAL_CONFLICT: FSM_STATE = FSM_STATE(6i32);
pub const PREVSTATE: u32 = 32u32;
pub const PROBATION: QuarantineStatus = QuarantineStatus(3i32);
pub const PrimaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(0i32);
pub const QUARANTINE_CONFIG_OPTION: u32 = 43222u32;
pub const QUARANTINE_SCOPE_QUARPROFILE_OPTION: u32 = 43221u32;
pub const QUARANTIN_OPTION_BASE: u32 = 43220u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QuarantineStatus(pub i32);
pub const RECOVER: FSM_STATE = FSM_STATE(9i32);
pub const RECOVER_DONE: FSM_STATE = FSM_STATE(11i32);
pub const RECOVER_WAIT: FSM_STATE = FSM_STATE(10i32);
pub const RESOLUTION_INT: FSM_STATE = FSM_STATE(8i32);
pub const RESTRICTEDACCESS: QuarantineStatus = QuarantineStatus(1i32);
pub const SAFEPERIOD: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: u64,
    pub NumAddressesFree: u64,
    pub NumPendingAdvertises: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_MIB_INFO_VQ {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
    pub QtnNumLeases: u32,
    pub QtnPctQtnLeases: u32,
    pub QtnProbationLeases: u32,
    pub QtnNonQtnLeases: u32,
    pub QtnExemptLeases: u32,
    pub QtnCapableClients: u32,
}
pub const SHAREDSECRET: u32 = 64u32;
pub const SHUTDOWN: FSM_STATE = FSM_STATE(13i32);
pub const STARTUP: FSM_STATE = FSM_STATE(2i32);
pub const STATUS_NOPREFIX_AVAIL: StatusCode = StatusCode(6i32);
pub const STATUS_NO_BINDING: StatusCode = StatusCode(3i32);
pub const STATUS_NO_ERROR: StatusCode = StatusCode(0i32);
pub const STATUS_UNSPECIFIED_FAILURE: StatusCode = StatusCode(1i32);
pub const SecondaryServer: DHCP_FAILOVER_SERVER = DHCP_FAILOVER_SERVER(1i32);
pub const Set_APIProtocolSupport: u32 = 1u32;
pub const Set_AuditLogState: u32 = 2048u32;
pub const Set_BackupInterval: u32 = 16u32;
pub const Set_BackupPath: u32 = 8u32;
pub const Set_BootFileTable: u32 = 1024u32;
pub const Set_DatabaseCleanupInterval: u32 = 128u32;
pub const Set_DatabaseLoggingFlag: u32 = 32u32;
pub const Set_DatabaseName: u32 = 2u32;
pub const Set_DatabasePath: u32 = 4u32;
pub const Set_DebugFlag: u32 = 256u32;
pub const Set_PingRetries: u32 = 512u32;
pub const Set_PreferredLifetime: u32 = 4u32;
pub const Set_PreferredLifetimeIATA: u32 = 64u32;
pub const Set_QuarantineDefFail: u32 = 8192u32;
pub const Set_QuarantineON: u32 = 4096u32;
pub const Set_RapidCommitFlag: u32 = 2u32;
pub const Set_RestoreFlag: u32 = 64u32;
pub const Set_T1: u32 = 16u32;
pub const Set_T2: u32 = 32u32;
pub const Set_UnicastFlag: u32 = 1u32;
pub const Set_ValidLifetime: u32 = 8u32;
pub const Set_ValidLifetimeIATA: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StatusCode(pub i32);
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32u32;
pub const V5_ADDRESS_BIT_DELETED: u32 = 128u32;
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64u32;
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1u32;
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1u32;
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2u32;
pub const V5_ADDRESS_STATE_DOOM: u32 = 3u32;
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0u32;
pub const WARNING_EXTENDED_LESS: i32 = 20026i32;
