#[inline]
pub unsafe fn DhcpAddFilterV4<P0, P1>(serveripaddress: P0, addfilterinfo: *const DHCP_FILTER_ADD_INFO, forceflag: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddFilterV4(serveripaddress : windows_core::PCWSTR, addfilterinfo : *const DHCP_FILTER_ADD_INFO, forceflag : super::super::Foundation:: BOOL) -> u32);
    DhcpAddFilterV4(serveripaddress.param().abi(), addfilterinfo, forceflag.param().abi())
}
#[inline]
pub unsafe fn DhcpAddSecurityGroup<P0>(pserver: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddSecurityGroup(pserver : windows_core::PCWSTR) -> u32);
    DhcpAddSecurityGroup(pserver.param().abi())
}
#[inline]
pub unsafe fn DhcpAddServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : *mut DHCPDS_SERVER, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    DhcpAddServer(flags, idinfo, newserver, callbackfn, callbackdata)
}
#[inline]
pub unsafe fn DhcpAddSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA) -> u32);
    DhcpAddSubnetElement(serveripaddress.param().abi(), subnetaddress, addelementinfo)
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4) -> u32);
    DhcpAddSubnetElementV4(serveripaddress.param().abi(), subnetaddress, addelementinfo)
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, addelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, addelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5) -> u32);
    DhcpAddSubnetElementV5(serveripaddress.param().abi(), subnetaddress, addelementinfo)
}
#[inline]
pub unsafe fn DhcpAddSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, addelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAddSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, addelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6) -> u32);
    DhcpAddSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), addelementinfo)
}
#[inline]
pub unsafe fn DhcpAuditLogGetParams<P0>(serveripaddress: P0, flags: u32, auditlogdir: *mut windows_core::PWSTR, diskcheckinterval: *mut u32, maxlogfilessize: *mut u32, minspaceondisk: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAuditLogGetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : *mut windows_core::PWSTR, diskcheckinterval : *mut u32, maxlogfilessize : *mut u32, minspaceondisk : *mut u32) -> u32);
    DhcpAuditLogGetParams(serveripaddress.param().abi(), flags, auditlogdir, diskcheckinterval, maxlogfilessize, minspaceondisk)
}
#[inline]
pub unsafe fn DhcpAuditLogSetParams<P0, P1>(serveripaddress: P0, flags: u32, auditlogdir: P1, diskcheckinterval: u32, maxlogfilessize: u32, minspaceondisk: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpAuditLogSetParams(serveripaddress : windows_core::PCWSTR, flags : u32, auditlogdir : windows_core::PCWSTR, diskcheckinterval : u32, maxlogfilessize : u32, minspaceondisk : u32) -> u32);
    DhcpAuditLogSetParams(serveripaddress.param().abi(), flags, auditlogdir.param().abi(), diskcheckinterval, maxlogfilessize, minspaceondisk)
}
#[inline]
pub unsafe fn DhcpCApiCleanup() {
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpCApiCleanup());
    DhcpCApiCleanup()
}
#[inline]
pub unsafe fn DhcpCApiInitialize(version: *mut u32) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpCApiInitialize(version : *mut u32) -> u32);
    DhcpCApiInitialize(version)
}
#[inline]
pub unsafe fn DhcpCreateClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    DhcpCreateClass(serveripaddress.param().abi(), reservedmustbezero, classinfo)
}
#[inline]
pub unsafe fn DhcpCreateClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    DhcpCreateClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo)
}
#[inline]
pub unsafe fn DhcpCreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    DhcpCreateClientInfo(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpCreateClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    DhcpCreateClientInfoV4(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpCreateClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    DhcpCreateClientInfoVQ(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpCreateOption<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateOption(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *const DHCP_OPTION) -> u32);
    DhcpCreateOption(serveripaddress.param().abi(), optionid, optioninfo)
}
#[inline]
pub unsafe fn DhcpCreateOptionV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    DhcpCreateOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpCreateOptionV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    DhcpCreateOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpCreateSubnet<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    DhcpCreateSubnet(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpCreateSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    DhcpCreateSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo)
}
#[inline]
pub unsafe fn DhcpCreateSubnetVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpCreateSubnetVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    DhcpCreateSubnetVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpDeRegisterParamChange(flags: u32, reserved: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpDeRegisterParamChange(flags : u32, reserved : *mut core::ffi::c_void, event : *mut core::ffi::c_void) -> u32);
    DhcpDeRegisterParamChange(flags, reserved, event)
}
#[inline]
pub unsafe fn DhcpDeleteClass<P0, P1>(serveripaddress: P0, reservedmustbezero: u32, classname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    DhcpDeleteClass(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi())
}
#[inline]
pub unsafe fn DhcpDeleteClassV6<P0, P1>(serveripaddress: P0, reservedmustbezero: u32, classname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classname : windows_core::PCWSTR) -> u32);
    DhcpDeleteClassV6(serveripaddress.param().abi(), reservedmustbezero, classname.param().abi())
}
#[inline]
pub unsafe fn DhcpDeleteClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO) -> u32);
    DhcpDeleteClientInfo(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpDeleteClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_SEARCH_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_SEARCH_INFO_V6) -> u32);
    DhcpDeleteClientInfoV6(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpDeleteFilterV4<P0>(serveripaddress: P0, deletefilterinfo: *const DHCP_ADDR_PATTERN) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteFilterV4(serveripaddress : windows_core::PCWSTR, deletefilterinfo : *const DHCP_ADDR_PATTERN) -> u32);
    DhcpDeleteFilterV4(serveripaddress.param().abi(), deletefilterinfo)
}
#[inline]
pub unsafe fn DhcpDeleteServer(flags: u32, idinfo: *mut core::ffi::c_void, newserver: *mut DHCPDS_SERVER, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteServer(flags : u32, idinfo : *mut core::ffi::c_void, newserver : *mut DHCPDS_SERVER, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    DhcpDeleteServer(flags, idinfo, newserver, callbackfn, callbackdata)
}
#[inline]
pub unsafe fn DhcpDeleteSubnet<P0>(serveripaddress: P0, subnetaddress: u32, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnet(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpDeleteSubnet(serveripaddress.param().abi(), subnetaddress, forceflag)
}
#[inline]
pub unsafe fn DhcpDeleteSubnetV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteSubnetV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpDeleteSubnetV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), forceflag)
}
#[inline]
pub unsafe fn DhcpDeleteSuperScopeV4<P0, P1>(serveripaddress: P0, superscopename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDeleteSuperScopeV4(serveripaddress : windows_core::PCWSTR, superscopename : windows_core::PCWSTR) -> u32);
    DhcpDeleteSuperScopeV4(serveripaddress.param().abi(), superscopename.param().abi())
}
#[inline]
pub unsafe fn DhcpDsCleanup() {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDsCleanup());
    DhcpDsCleanup()
}
#[inline]
pub unsafe fn DhcpDsInit() -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpDsInit() -> u32);
    DhcpDsInit()
}
#[inline]
pub unsafe fn DhcpEnumClasses<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumClasses(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut u32, preferredmaximum : u32, classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY, nread : *mut u32, ntotal : *mut u32) -> u32);
    DhcpEnumClasses(serveripaddress.param().abi(), reservedmustbezero, resumehandle, preferredmaximum, classinfoarray, nread, ntotal)
}
#[inline]
pub unsafe fn DhcpEnumClassesV6<P0>(serveripaddress: P0, reservedmustbezero: u32, resumehandle: *mut u32, preferredmaximum: u32, classinfoarray: *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread: *mut u32, ntotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumClassesV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, resumehandle : *mut u32, preferredmaximum : u32, classinfoarray : *mut *mut DHCP_CLASS_INFO_ARRAY_V6, nread : *mut u32, ntotal : *mut u32) -> u32);
    DhcpEnumClassesV6(serveripaddress.param().abi(), reservedmustbezero, resumehandle, preferredmaximum, classinfoarray, nread, ntotal)
}
#[inline]
pub unsafe fn DhcpEnumFilterV4<P0>(serveripaddress: P0, resumehandle: *mut DHCP_ADDR_PATTERN, preferredmaximum: u32, listtype: DHCP_FILTER_LIST_TYPE, enumfilterinfo: *mut *mut DHCP_FILTER_ENUM_INFO, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumFilterV4(serveripaddress : windows_core::PCWSTR, resumehandle : *mut DHCP_ADDR_PATTERN, preferredmaximum : u32, listtype : DHCP_FILTER_LIST_TYPE, enumfilterinfo : *mut *mut DHCP_FILTER_ENUM_INFO, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumFilterV4(serveripaddress.param().abi(), resumehandle, preferredmaximum, listtype, enumfilterinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptionValues(serveripaddress.param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptionValuesV6<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, resumehandle: *mut u32, preferredmaximum: u32, optionvalues: *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, resumehandle : *mut u32, preferredmaximum : u32, optionvalues : *mut *mut DHCP_OPTION_VALUE_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptionValuesV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo, resumehandle, preferredmaximum, optionvalues, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptions<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptions(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptions(serveripaddress.param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptionsV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptionsV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumOptionsV6<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, resumehandle: *mut u32, preferredmaximum: u32, options: *mut *mut DHCP_OPTION_ARRAY, optionsread: *mut u32, optionstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, options : *mut *mut DHCP_OPTION_ARRAY, optionsread : *mut u32, optionstotal : *mut u32) -> u32);
    DhcpEnumOptionsV6(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), resumehandle, preferredmaximum, options, optionsread, optionstotal)
}
#[inline]
pub unsafe fn DhcpEnumServers(flags: u32, idinfo: *mut core::ffi::c_void, servers: *mut *mut DHCPDS_SERVERS, callbackfn: *mut core::ffi::c_void, callbackdata: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumServers(flags : u32, idinfo : *mut core::ffi::c_void, servers : *mut *mut DHCPDS_SERVERS, callbackfn : *mut core::ffi::c_void, callbackdata : *mut core::ffi::c_void) -> u32);
    DhcpEnumServers(flags, idinfo, servers, callbackfn, callbackdata)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsFilterStatusInfo<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClientsFilterStatusInfo(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV4<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V4, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClientsV4(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV5<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V5, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClientsV5(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, resumehandle: *mut DHCP_IPV6_ADDRESS, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, resumehandle : *mut DHCP_IPV6_ADDRESS, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_V6, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClientsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetClientsVQ<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetClientsVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_ARRAY_VQ, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpEnumSubnetClientsVQ(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetElements<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElements(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnetElements(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV4<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnetElementsV4(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV5<P0>(serveripaddress: P0, subnetaddress: u32, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnetElementsV5(serveripaddress.param().abi(), subnetaddress, enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetElementsV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, enumelementtype: DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetElementsV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, enumelementtype : DHCP_SUBNET_ELEMENT_TYPE_V6, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnetElementsV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), enumelementtype, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnets<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCP_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnets(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, enuminfo : *mut *mut DHCP_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnets(serveripaddress.param().abi(), resumehandle, preferredmaximum, enuminfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpEnumSubnetsV6<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, enuminfo: *mut *mut DHCPV6_IP_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpEnumSubnetsV6(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, enuminfo : *mut *mut DHCPV6_IP_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpEnumSubnetsV6(serveripaddress.param().abi(), resumehandle, preferredmaximum, enuminfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpGetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut *mut DHCP_ALL_OPTION_VALUES) -> u32);
    DhcpGetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo, values)
}
#[inline]
pub unsafe fn DhcpGetAllOptionValuesV6<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, values: *mut *mut DHCP_ALL_OPTION_VALUES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionValuesV6(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, values : *mut *mut DHCP_ALL_OPTION_VALUES) -> u32);
    DhcpGetAllOptionValuesV6(serveripaddress.param().abi(), flags, scopeinfo, values)
}
#[inline]
pub unsafe fn DhcpGetAllOptions<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptions(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut *mut DHCP_ALL_OPTIONS) -> u32);
    DhcpGetAllOptions(serveripaddress.param().abi(), flags, optionstruct)
}
#[inline]
pub unsafe fn DhcpGetAllOptionsV6<P0>(serveripaddress: P0, flags: u32, optionstruct: *mut *mut DHCP_ALL_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetAllOptionsV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionstruct : *mut *mut DHCP_ALL_OPTIONS) -> u32);
    DhcpGetAllOptionsV6(serveripaddress.param().abi(), flags, optionstruct)
}
#[inline]
pub unsafe fn DhcpGetClassInfo<P0>(serveripaddress: P0, reservedmustbezero: u32, partialclassinfo: *mut DHCP_CLASS_INFO, filledclassinfo: *mut *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClassInfo(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, partialclassinfo : *mut DHCP_CLASS_INFO, filledclassinfo : *mut *mut DHCP_CLASS_INFO) -> u32);
    DhcpGetClassInfo(serveripaddress.param().abi(), reservedmustbezero, partialclassinfo, filledclassinfo)
}
#[inline]
pub unsafe fn DhcpGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO) -> u32);
    DhcpGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpGetClientInfoV4<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV4(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_V4) -> u32);
    DhcpGetClientInfoV4(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpGetClientInfoV6<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO_V6, clientinfo: *mut *mut DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoV6(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO_V6, clientinfo : *mut *mut DHCP_CLIENT_INFO_V6) -> u32);
    DhcpGetClientInfoV6(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpGetClientInfoVQ<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClientInfoVQ(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_VQ) -> u32);
    DhcpGetClientInfoVQ(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpGetClientOptions<P0>(serveripaddress: P0, clientipaddress: u32, clientsubnetmask: u32, clientoptions: *mut *mut DHCP_OPTION_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetClientOptions(serveripaddress : windows_core::PCWSTR, clientipaddress : u32, clientsubnetmask : u32, clientoptions : *mut *mut DHCP_OPTION_LIST) -> u32);
    DhcpGetClientOptions(serveripaddress.param().abi(), clientipaddress, clientsubnetmask, clientoptions)
}
#[inline]
pub unsafe fn DhcpGetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *mut DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *mut DHCP_FILTER_GLOBAL_INFO) -> u32);
    DhcpGetFilterV4(serveripaddress.param().abi(), globalfilterinfo)
}
#[inline]
pub unsafe fn DhcpGetMibInfo<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfo(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO) -> u32);
    DhcpGetMibInfo(serveripaddress.param().abi(), mibinfo)
}
#[inline]
pub unsafe fn DhcpGetMibInfoV5<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV5(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO_V5) -> u32);
    DhcpGetMibInfoV5(serveripaddress.param().abi(), mibinfo)
}
#[inline]
pub unsafe fn DhcpGetMibInfoV6<P0>(serveripaddress: P0, mibinfo: *mut *mut DHCP_MIB_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetMibInfoV6(serveripaddress : windows_core::PCWSTR, mibinfo : *mut *mut DHCP_MIB_INFO_V6) -> u32);
    DhcpGetMibInfoV6(serveripaddress.param().abi(), mibinfo)
}
#[inline]
pub unsafe fn DhcpGetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    DhcpGetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo)
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    DhcpGetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpGetOptionInfoV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut *mut DHCP_OPTION) -> u32);
    DhcpGetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpGetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    DhcpGetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpGetOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    DhcpGetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpGetOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    DhcpGetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpGetOriginalSubnetMask<P0>(sadaptername: P0, dwsubnetmask: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpGetOriginalSubnetMask(sadaptername : windows_core::PCWSTR, dwsubnetmask : *mut u32) -> u32);
    DhcpGetOriginalSubnetMask(sadaptername.param().abi(), dwsubnetmask)
}
#[inline]
pub unsafe fn DhcpGetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut *mut DHCP_BIND_ELEMENT_ARRAY) -> u32);
    DhcpGetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementsinfo)
}
#[inline]
pub unsafe fn DhcpGetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementsinfo: *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementsinfo : *mut *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    DhcpGetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementsinfo)
}
#[inline]
pub unsafe fn DhcpGetServerSpecificStrings<P0>(serveripaddress: P0, serverspecificstrings: *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetServerSpecificStrings(serveripaddress : windows_core::PCWSTR, serverspecificstrings : *mut *mut DHCP_SERVER_SPECIFIC_STRINGS) -> u32);
    DhcpGetServerSpecificStrings(serveripaddress.param().abi(), serverspecificstrings)
}
#[inline]
pub unsafe fn DhcpGetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: *mut u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, timedelayinmilliseconds : *mut u16) -> u32);
    DhcpGetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds)
}
#[inline]
pub unsafe fn DhcpGetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *mut *mut DHCP_SUBNET_INFO) -> u32);
    DhcpGetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut *mut DHCP_SUBNET_INFO_V6) -> u32);
    DhcpGetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo)
}
#[inline]
pub unsafe fn DhcpGetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *mut *mut DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *mut *mut DHCP_SUBNET_INFO_VQ) -> u32);
    DhcpGetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpGetSuperScopeInfoV4<P0>(serveripaddress: P0, superscopetable: *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetSuperScopeInfoV4(serveripaddress : windows_core::PCWSTR, superscopetable : *mut *mut DHCP_SUPER_SCOPE_TABLE) -> u32);
    DhcpGetSuperScopeInfoV4(serveripaddress.param().abi(), superscopetable)
}
#[inline]
pub unsafe fn DhcpGetThreadOptions(pflags: *mut u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetThreadOptions(pflags : *mut u32, reserved : *mut core::ffi::c_void) -> u32);
    DhcpGetThreadOptions(pflags, reserved)
}
#[inline]
pub unsafe fn DhcpGetVersion<P0>(serveripaddress: P0, majorversion: *mut u32, minorversion: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpGetVersion(serveripaddress : windows_core::PCWSTR, majorversion : *mut u32, minorversion : *mut u32) -> u32);
    DhcpGetVersion(serveripaddress.param().abi(), majorversion, minorversion)
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyCondition<P0>(policy: *mut DHCP_POLICY, parentexpr: u32, r#type: DHCP_POL_ATTR_TYPE, optionid: u32, suboptionid: u32, vendorname: P0, operator: DHCP_POL_COMPARATOR, value: &[u8], conditionindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyCondition(policy : *mut DHCP_POLICY, parentexpr : u32, r#type : DHCP_POL_ATTR_TYPE, optionid : u32, suboptionid : u32, vendorname : windows_core::PCWSTR, operator : DHCP_POL_COMPARATOR, value : *const u8, valuelength : u32, conditionindex : *mut u32) -> u32);
    DhcpHlprAddV4PolicyCondition(policy, parentexpr, r#type, optionid, suboptionid, vendorname.param().abi(), operator, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap(), conditionindex)
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyExpr(policy: *mut DHCP_POLICY, parentexpr: u32, operator: DHCP_POL_LOGIC_OPER, exprindex: *mut u32) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyExpr(policy : *mut DHCP_POLICY, parentexpr : u32, operator : DHCP_POL_LOGIC_OPER, exprindex : *mut u32) -> u32);
    DhcpHlprAddV4PolicyExpr(policy, parentexpr, operator, exprindex)
}
#[inline]
pub unsafe fn DhcpHlprAddV4PolicyRange(policy: *mut DHCP_POLICY, range: *const DHCP_IP_RANGE) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprAddV4PolicyRange(policy : *mut DHCP_POLICY, range : *const DHCP_IP_RANGE) -> u32);
    DhcpHlprAddV4PolicyRange(policy, range)
}
#[inline]
pub unsafe fn DhcpHlprCreateV4Policy<P0, P1, P2, P3>(policyname: P0, fglobalpolicy: P1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P2, enabled: P3, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4Policy(policyname : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnet : u32, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : super::super::Foundation:: BOOL, policy : *mut *mut DHCP_POLICY) -> u32);
    DhcpHlprCreateV4Policy(policyname.param().abi(), fglobalpolicy.param().abi(), subnet, processingorder, rootoperator, description.param().abi(), enabled.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpHlprCreateV4PolicyEx<P0, P1, P2, P3>(policyname: P0, fglobalpolicy: P1, subnet: u32, processingorder: u32, rootoperator: DHCP_POL_LOGIC_OPER, description: P2, enabled: P3, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprCreateV4PolicyEx(policyname : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnet : u32, processingorder : u32, rootoperator : DHCP_POL_LOGIC_OPER, description : windows_core::PCWSTR, enabled : super::super::Foundation:: BOOL, policy : *mut *mut DHCP_POLICY_EX) -> u32);
    DhcpHlprCreateV4PolicyEx(policyname.param().abi(), fglobalpolicy.param().abi(), subnet, processingorder, rootoperator, description.param().abi(), enabled.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpHlprFindV4DhcpProperty(propertyarray: *const DHCP_PROPERTY_ARRAY, id: DHCP_PROPERTY_ID, r#type: DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFindV4DhcpProperty(propertyarray : *const DHCP_PROPERTY_ARRAY, id : DHCP_PROPERTY_ID, r#type : DHCP_PROPERTY_TYPE) -> *mut DHCP_PROPERTY);
    DhcpHlprFindV4DhcpProperty(propertyarray, id, r#type)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpProperty(property: *mut DHCP_PROPERTY) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpProperty(property : *mut DHCP_PROPERTY));
    DhcpHlprFreeV4DhcpProperty(property)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray: *mut DHCP_PROPERTY_ARRAY) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4DhcpPropertyArray(propertyarray : *mut DHCP_PROPERTY_ARRAY));
    DhcpHlprFreeV4DhcpPropertyArray(propertyarray)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4Policy(policy: *mut DHCP_POLICY) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4Policy(policy : *mut DHCP_POLICY));
    DhcpHlprFreeV4Policy(policy)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyArray(policyarray: *mut DHCP_POLICY_ARRAY) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyArray(policyarray : *mut DHCP_POLICY_ARRAY));
    DhcpHlprFreeV4PolicyArray(policyarray)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyEx(policyex: *mut DHCP_POLICY_EX) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyEx(policyex : *mut DHCP_POLICY_EX));
    DhcpHlprFreeV4PolicyEx(policyex)
}
#[inline]
pub unsafe fn DhcpHlprFreeV4PolicyExArray(policyexarray: *mut DHCP_POLICY_EX_ARRAY) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprFreeV4PolicyExArray(policyexarray : *mut DHCP_POLICY_EX_ARRAY));
    DhcpHlprFreeV4PolicyExArray(policyexarray)
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicySingleUC(policy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicySingleUC(policy : *const DHCP_POLICY) -> super::super::Foundation:: BOOL);
    DhcpHlprIsV4PolicySingleUC(policy)
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyValid(ppolicy: *const DHCP_POLICY) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyValid(ppolicy : *const DHCP_POLICY) -> u32);
    DhcpHlprIsV4PolicyValid(ppolicy)
}
#[inline]
pub unsafe fn DhcpHlprIsV4PolicyWellFormed(ppolicy: *const DHCP_POLICY) -> super::super::Foundation::BOOL {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprIsV4PolicyWellFormed(ppolicy : *const DHCP_POLICY) -> super::super::Foundation:: BOOL);
    DhcpHlprIsV4PolicyWellFormed(ppolicy)
}
#[inline]
pub unsafe fn DhcpHlprModifyV4PolicyExpr(policy: *mut DHCP_POLICY, operator: DHCP_POL_LOGIC_OPER) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprModifyV4PolicyExpr(policy : *mut DHCP_POLICY, operator : DHCP_POL_LOGIC_OPER) -> u32);
    DhcpHlprModifyV4PolicyExpr(policy, operator)
}
#[inline]
pub unsafe fn DhcpHlprResetV4PolicyExpr(policy: *mut DHCP_POLICY) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpHlprResetV4PolicyExpr(policy : *mut DHCP_POLICY) -> u32);
    DhcpHlprResetV4PolicyExpr(policy)
}
#[inline]
pub unsafe fn DhcpModifyClass<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpModifyClass(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO) -> u32);
    DhcpModifyClass(serveripaddress.param().abi(), reservedmustbezero, classinfo)
}
#[inline]
pub unsafe fn DhcpModifyClassV6<P0>(serveripaddress: P0, reservedmustbezero: u32, classinfo: *mut DHCP_CLASS_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpModifyClassV6(serveripaddress : windows_core::PCWSTR, reservedmustbezero : u32, classinfo : *mut DHCP_CLASS_INFO_V6) -> u32);
    DhcpModifyClassV6(serveripaddress.param().abi(), reservedmustbezero, classinfo)
}
#[inline]
pub unsafe fn DhcpRegisterParamChange<P0>(flags: u32, reserved: Option<*const core::ffi::c_void>, adaptername: P0, classid: *mut DHCPCAPI_CLASSID, params: DHCPCAPI_PARAMS_ARRAY, handle: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpRegisterParamChange(flags : u32, reserved : *const core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPCAPI_CLASSID, params : DHCPCAPI_PARAMS_ARRAY, handle : *mut core::ffi::c_void) -> u32);
    DhcpRegisterParamChange(flags, core::mem::transmute(reserved.unwrap_or(std::ptr::null())), adaptername.param().abi(), classid, core::mem::transmute(params), handle)
}
#[inline]
pub unsafe fn DhcpRemoveDNSRegistrations() -> u32 {
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpRemoveDNSRegistrations() -> u32);
    DhcpRemoveDNSRegistrations()
}
#[inline]
pub unsafe fn DhcpRemoveOption<P0>(serveripaddress: P0, optionid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOption(serveripaddress : windows_core::PCWSTR, optionid : u32) -> u32);
    DhcpRemoveOption(serveripaddress.param().abi(), optionid)
}
#[inline]
pub unsafe fn DhcpRemoveOptionV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    DhcpRemoveOptionV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi())
}
#[inline]
pub unsafe fn DhcpRemoveOptionV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR) -> u32);
    DhcpRemoveOptionV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi())
}
#[inline]
pub unsafe fn DhcpRemoveOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO) -> u32);
    DhcpRemoveOptionValue(serveripaddress.param().abi(), optionid, scopeinfo)
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    DhcpRemoveOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo)
}
#[inline]
pub unsafe fn DhcpRemoveOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6) -> u32);
    DhcpRemoveOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo)
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElement<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElement(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpRemoveSubnetElement(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV4<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V4, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpRemoveSubnetElementV4(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV5<P0>(serveripaddress: P0, subnetaddress: u32, removeelementinfo: *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV5(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, removeelementinfo : *const DHCP_SUBNET_ELEMENT_DATA_V5, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpRemoveSubnetElementV5(serveripaddress.param().abi(), subnetaddress, removeelementinfo, forceflag)
}
#[inline]
pub unsafe fn DhcpRemoveSubnetElementV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, removeelementinfo: *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag: DHCP_FORCE_FLAG) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRemoveSubnetElementV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, removeelementinfo : *mut DHCP_SUBNET_ELEMENT_DATA_V6, forceflag : DHCP_FORCE_FLAG) -> u32);
    DhcpRemoveSubnetElementV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), removeelementinfo, forceflag)
}
#[inline]
pub unsafe fn DhcpRequestParams<P0, P1>(flags: u32, reserved: *mut core::ffi::c_void, adaptername: P0, classid: *mut DHCPCAPI_CLASSID, sendparams: DHCPCAPI_PARAMS_ARRAY, recdparams: DHCPCAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32, requestidstr: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpRequestParams(flags : u32, reserved : *mut core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPCAPI_CLASSID, sendparams : DHCPCAPI_PARAMS_ARRAY, recdparams : DHCPCAPI_PARAMS_ARRAY, buffer : *mut u8, psize : *mut u32, requestidstr : windows_core::PCWSTR) -> u32);
    DhcpRequestParams(flags, reserved, adaptername.param().abi(), classid, core::mem::transmute(sendparams), core::mem::transmute(recdparams), buffer, psize, requestidstr.param().abi())
}
#[inline]
pub unsafe fn DhcpRpcFreeMemory(bufferpointer: *mut core::ffi::c_void) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpRpcFreeMemory(bufferpointer : *mut core::ffi::c_void));
    DhcpRpcFreeMemory(bufferpointer)
}
#[inline]
pub unsafe fn DhcpScanDatabase<P0>(serveripaddress: P0, subnetaddress: u32, fixflag: u32, scanlist: *mut *mut DHCP_SCAN_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpScanDatabase(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, fixflag : u32, scanlist : *mut *mut DHCP_SCAN_LIST) -> u32);
    DhcpScanDatabase(serveripaddress.param().abi(), subnetaddress, fixflag, scanlist)
}
#[inline]
pub unsafe fn DhcpServerAuditlogParamsFree(configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerAuditlogParamsFree(configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ));
    DhcpServerAuditlogParamsFree(configinfo)
}
#[inline]
pub unsafe fn DhcpServerBackupDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerBackupDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    DhcpServerBackupDatabase(serveripaddress.param().abi(), path.param().abi())
}
#[inline]
pub unsafe fn DhcpServerGetConfig<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfig(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO) -> u32);
    DhcpServerGetConfig(serveripaddress.param().abi(), configinfo)
}
#[inline]
pub unsafe fn DhcpServerGetConfigV4<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV4(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32);
    DhcpServerGetConfigV4(serveripaddress.param().abi(), configinfo)
}
#[inline]
pub unsafe fn DhcpServerGetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32);
    DhcpServerGetConfigV6(serveripaddress.param().abi(), scopeinfo, configinfo)
}
#[inline]
pub unsafe fn DhcpServerGetConfigVQ<P0>(serveripaddress: P0, configinfo: *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerGetConfigVQ(serveripaddress : windows_core::PCWSTR, configinfo : *mut *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    DhcpServerGetConfigVQ(serveripaddress.param().abi(), configinfo)
}
#[inline]
pub unsafe fn DhcpServerQueryAttribute<P0>(serveripaddr: P0, dwreserved: u32, dhcpattribid: u32, pdhcpattrib: *mut *mut DHCP_ATTRIB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerQueryAttribute(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dhcpattribid : u32, pdhcpattrib : *mut *mut DHCP_ATTRIB) -> u32);
    DhcpServerQueryAttribute(serveripaddr.param().abi(), dwreserved, dhcpattribid, pdhcpattrib)
}
#[inline]
pub unsafe fn DhcpServerQueryAttributes<P0>(serveripaddr: P0, dwreserved: u32, dwattribcount: u32, pdhcpattribs: *mut u32, pdhcpattribarr: *mut *mut DHCP_ATTRIB_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerQueryAttributes(serveripaddr : windows_core::PCWSTR, dwreserved : u32, dwattribcount : u32, pdhcpattribs : *mut u32, pdhcpattribarr : *mut *mut DHCP_ATTRIB_ARRAY) -> u32);
    DhcpServerQueryAttributes(serveripaddr.param().abi(), dwreserved, dwattribcount, pdhcpattribs, pdhcpattribarr)
}
#[inline]
pub unsafe fn DhcpServerQueryDnsRegCredentials<P0>(serveripaddress: P0, uname: &mut [u16], domain: &mut [u16]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerQueryDnsRegCredentials(serveripaddress : windows_core::PCWSTR, unamesize : u32, uname : windows_core::PWSTR, domainsize : u32, domain : windows_core::PWSTR) -> u32);
    DhcpServerQueryDnsRegCredentials(serveripaddress.param().abi(), uname.len().try_into().unwrap(), core::mem::transmute(uname.as_ptr()), domain.len().try_into().unwrap(), core::mem::transmute(domain.as_ptr()))
}
#[inline]
pub unsafe fn DhcpServerRedoAuthorization<P0>(serveripaddr: P0, dwreserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerRedoAuthorization(serveripaddr : windows_core::PCWSTR, dwreserved : u32) -> u32);
    DhcpServerRedoAuthorization(serveripaddr.param().abi(), dwreserved)
}
#[inline]
pub unsafe fn DhcpServerRestoreDatabase<P0, P1>(serveripaddress: P0, path: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerRestoreDatabase(serveripaddress : windows_core::PCWSTR, path : windows_core::PCWSTR) -> u32);
    DhcpServerRestoreDatabase(serveripaddress.param().abi(), path.param().abi())
}
#[inline]
pub unsafe fn DhcpServerSetConfig<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfig(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO) -> u32);
    DhcpServerSetConfig(serveripaddress.param().abi(), fieldstoset, configinfo)
}
#[inline]
pub unsafe fn DhcpServerSetConfigV4<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV4(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V4) -> u32);
    DhcpServerSetConfigV4(serveripaddress.param().abi(), fieldstoset, configinfo)
}
#[inline]
pub unsafe fn DhcpServerSetConfigV6<P0>(serveripaddress: P0, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigV6(serveripaddress : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_V6) -> u32);
    DhcpServerSetConfigV6(serveripaddress.param().abi(), scopeinfo, fieldstoset, configinfo)
}
#[inline]
pub unsafe fn DhcpServerSetConfigVQ<P0>(serveripaddress: P0, fieldstoset: u32, configinfo: *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetConfigVQ(serveripaddress : windows_core::PCWSTR, fieldstoset : u32, configinfo : *mut DHCP_SERVER_CONFIG_INFO_VQ) -> u32);
    DhcpServerSetConfigVQ(serveripaddress.param().abi(), fieldstoset, configinfo)
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentials<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetDnsRegCredentials(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    DhcpServerSetDnsRegCredentials(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi())
}
#[inline]
pub unsafe fn DhcpServerSetDnsRegCredentialsV5<P0, P1, P2, P3>(serveripaddress: P0, uname: P1, domain: P2, passwd: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpServerSetDnsRegCredentialsV5(serveripaddress : windows_core::PCWSTR, uname : windows_core::PCWSTR, domain : windows_core::PCWSTR, passwd : windows_core::PCWSTR) -> u32);
    DhcpServerSetDnsRegCredentialsV5(serveripaddress.param().abi(), uname.param().abi(), domain.param().abi(), passwd.param().abi())
}
#[inline]
pub unsafe fn DhcpSetClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO) -> u32);
    DhcpSetClientInfo(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpSetClientInfoV4<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV4(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V4) -> u32);
    DhcpSetClientInfoV4(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpSetClientInfoV6<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoV6(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    DhcpSetClientInfoV6(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpSetClientInfoVQ<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetClientInfoVQ(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_VQ) -> u32);
    DhcpSetClientInfoVQ(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpSetFilterV4<P0>(serveripaddress: P0, globalfilterinfo: *const DHCP_FILTER_GLOBAL_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetFilterV4(serveripaddress : windows_core::PCWSTR, globalfilterinfo : *const DHCP_FILTER_GLOBAL_INFO) -> u32);
    DhcpSetFilterV4(serveripaddress.param().abi(), globalfilterinfo)
}
#[inline]
pub unsafe fn DhcpSetOptionInfo<P0>(serveripaddress: P0, optionid: u32, optioninfo: *const DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfo(serveripaddress : windows_core::PCWSTR, optionid : u32, optioninfo : *const DHCP_OPTION) -> u32);
    DhcpSetOptionInfo(serveripaddress.param().abi(), optionid, optioninfo)
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    DhcpSetOptionInfoV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpSetOptionInfoV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, optioninfo: *mut DHCP_OPTION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, optioninfo : *mut DHCP_OPTION) -> u32);
    DhcpSetOptionInfoV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), optioninfo)
}
#[inline]
pub unsafe fn DhcpSetOptionValue<P0>(serveripaddress: P0, optionid: u32, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalue: *const DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValue(serveripaddress : windows_core::PCWSTR, optionid : u32, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalue : *const DHCP_OPTION_DATA) -> u32);
    DhcpSetOptionValue(serveripaddress.param().abi(), optionid, scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpSetOptionValueV5<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV5(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    DhcpSetOptionValueV5(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpSetOptionValueV6<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO6, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValueV6(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO6, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    DhcpSetOptionValueV6(serveripaddress.param().abi(), flags, optionid, classname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpSetOptionValues<P0>(serveripaddress: P0, scopeinfo: *const DHCP_OPTION_SCOPE_INFO, optionvalues: *const DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValues(serveripaddress : windows_core::PCWSTR, scopeinfo : *const DHCP_OPTION_SCOPE_INFO, optionvalues : *const DHCP_OPTION_VALUE_ARRAY) -> u32);
    DhcpSetOptionValues(serveripaddress.param().abi(), scopeinfo, optionvalues)
}
#[inline]
pub unsafe fn DhcpSetOptionValuesV5<P0, P1, P2>(serveripaddress: P0, flags: u32, classname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetOptionValuesV5(serveripaddress : windows_core::PCWSTR, flags : u32, classname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    DhcpSetOptionValuesV5(serveripaddress.param().abi(), flags, classname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalues)
}
#[inline]
pub unsafe fn DhcpSetServerBindingInfo<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCP_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfo(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCP_BIND_ELEMENT_ARRAY) -> u32);
    DhcpSetServerBindingInfo(serveripaddress.param().abi(), flags, bindelementinfo)
}
#[inline]
pub unsafe fn DhcpSetServerBindingInfoV6<P0>(serveripaddress: P0, flags: u32, bindelementinfo: *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetServerBindingInfoV6(serveripaddress : windows_core::PCWSTR, flags : u32, bindelementinfo : *mut DHCPV6_BIND_ELEMENT_ARRAY) -> u32);
    DhcpSetServerBindingInfoV6(serveripaddress.param().abi(), flags, bindelementinfo)
}
#[inline]
pub unsafe fn DhcpSetSubnetDelayOffer<P0>(serveripaddress: P0, subnetaddress: u32, timedelayinmilliseconds: u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetDelayOffer(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, timedelayinmilliseconds : u16) -> u32);
    DhcpSetSubnetDelayOffer(serveripaddress.param().abi(), subnetaddress, timedelayinmilliseconds)
}
#[inline]
pub unsafe fn DhcpSetSubnetInfo<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfo(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO) -> u32);
    DhcpSetSubnetInfo(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoV6<P0>(serveripaddress: P0, subnetaddress: DHCP_IPV6_ADDRESS, subnetinfo: *mut DHCP_SUBNET_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoV6(serveripaddress : windows_core::PCWSTR, subnetaddress : DHCP_IPV6_ADDRESS, subnetinfo : *mut DHCP_SUBNET_INFO_V6) -> u32);
    DhcpSetSubnetInfoV6(serveripaddress.param().abi(), core::mem::transmute(subnetaddress), subnetinfo)
}
#[inline]
pub unsafe fn DhcpSetSubnetInfoVQ<P0>(serveripaddress: P0, subnetaddress: u32, subnetinfo: *const DHCP_SUBNET_INFO_VQ) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetSubnetInfoVQ(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, subnetinfo : *const DHCP_SUBNET_INFO_VQ) -> u32);
    DhcpSetSubnetInfoVQ(serveripaddress.param().abi(), subnetaddress, subnetinfo)
}
#[inline]
pub unsafe fn DhcpSetSuperScopeV4<P0, P1, P2>(serveripaddress: P0, subnetaddress: u32, superscopename: P1, changeexisting: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetSuperScopeV4(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, superscopename : windows_core::PCWSTR, changeexisting : super::super::Foundation:: BOOL) -> u32);
    DhcpSetSuperScopeV4(serveripaddress.param().abi(), subnetaddress, superscopename.param().abi(), changeexisting.param().abi())
}
#[inline]
pub unsafe fn DhcpSetThreadOptions(flags: u32, reserved: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpSetThreadOptions(flags : u32, reserved : *mut core::ffi::c_void) -> u32);
    DhcpSetThreadOptions(flags, reserved)
}
#[inline]
pub unsafe fn DhcpUndoRequestParams<P0, P1>(flags: u32, reserved: Option<*const core::ffi::c_void>, adaptername: P0, requestidstr: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc.dll" "system" fn DhcpUndoRequestParams(flags : u32, reserved : *const core::ffi::c_void, adaptername : windows_core::PCWSTR, requestidstr : windows_core::PCWSTR) -> u32);
    DhcpUndoRequestParams(flags, core::mem::transmute(reserved.unwrap_or(std::ptr::null())), adaptername.param().abi(), requestidstr.param().abi())
}
#[inline]
pub unsafe fn DhcpV4AddPolicyRange<P0, P1>(serveripaddress: P0, subnetaddress: u32, policyname: P1, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4AddPolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    DhcpV4AddPolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range)
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_PB) -> u32);
    DhcpV4CreateClientInfo(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpV4CreateClientInfoEx<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4CreateClientInfoEx(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_EX) -> u32);
    DhcpV4CreateClientInfoEx(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpV4CreatePolicy<P0>(serveripaddress: P0, ppolicy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicy(serveripaddress : windows_core::PCWSTR, ppolicy : *const DHCP_POLICY) -> u32);
    DhcpV4CreatePolicy(serveripaddress.param().abi(), ppolicy)
}
#[inline]
pub unsafe fn DhcpV4CreatePolicyEx<P0>(serveripaddress: P0, policyex: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4CreatePolicyEx(serveripaddress : windows_core::PCWSTR, policyex : *const DHCP_POLICY_EX) -> u32);
    DhcpV4CreatePolicyEx(serveripaddress.param().abi(), policyex)
}
#[inline]
pub unsafe fn DhcpV4DeletePolicy<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, policyname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4DeletePolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR) -> u32);
    DhcpV4DeletePolicy(serveripaddress.param().abi(), fglobalpolicy.param().abi(), subnetaddress, policyname.param().abi())
}
#[inline]
pub unsafe fn DhcpV4EnumPolicies<P0, P1>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, fglobalpolicy: P1, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPolicies(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, enuminfo : *mut *mut DHCP_POLICY_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpV4EnumPolicies(serveripaddress.param().abi(), resumehandle, preferredmaximum, fglobalpolicy.param().abi(), subnetaddress, enuminfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpV4EnumPoliciesEx<P0, P1>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, globalpolicy: P1, subnetaddress: u32, enuminfo: *mut *mut DHCP_POLICY_EX_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4EnumPoliciesEx(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, globalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, enuminfo : *mut *mut DHCP_POLICY_EX_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpV4EnumPoliciesEx(serveripaddress.param().abi(), resumehandle, preferredmaximum, globalpolicy.param().abi(), subnetaddress, enuminfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClients<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClients(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_PB_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpV4EnumSubnetClients(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetClientsEx<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread: *mut u32, clientstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetClientsEx(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, clientinfo : *mut *mut DHCP_CLIENT_INFO_EX_ARRAY, clientsread : *mut u32, clientstotal : *mut u32) -> u32);
    DhcpV4EnumSubnetClientsEx(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, clientinfo, clientsread, clientstotal)
}
#[inline]
pub unsafe fn DhcpV4EnumSubnetReservations<P0>(serveripaddress: P0, subnetaddress: u32, resumehandle: *mut u32, preferredmaximum: u32, enumelementinfo: *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread: *mut u32, elementstotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4EnumSubnetReservations(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, resumehandle : *mut u32, preferredmaximum : u32, enumelementinfo : *mut *mut DHCP_RESERVATION_INFO_ARRAY, elementsread : *mut u32, elementstotal : *mut u32) -> u32);
    DhcpV4EnumSubnetReservations(serveripaddress.param().abi(), subnetaddress, resumehandle, preferredmaximum, enumelementinfo, elementsread, elementstotal)
}
#[inline]
pub unsafe fn DhcpV4FailoverAddScopeToRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverAddScopeToRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverAddScopeToRelationship(serveripaddress.param().abi(), prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverCreateRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverCreateRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverCreateRelationship(serveripaddress.param().abi(), prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR) -> u32);
    DhcpV4FailoverDeleteRelationship(serveripaddress.param().abi(), prelationshipname.param().abi())
}
#[inline]
pub unsafe fn DhcpV4FailoverDeleteScopeFromRelationship<P0>(serveripaddress: P0, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress : windows_core::PCWSTR, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverDeleteScopeFromRelationship(serveripaddress.param().abi(), prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverEnumRelationship<P0>(serveripaddress: P0, resumehandle: *mut u32, preferredmaximum: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread: *mut u32, relationshiptotal: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverEnumRelationship(serveripaddress : windows_core::PCWSTR, resumehandle : *mut u32, preferredmaximum : u32, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP_ARRAY, relationshipread : *mut u32, relationshiptotal : *mut u32) -> u32);
    DhcpV4FailoverEnumRelationship(serveripaddress.param().abi(), resumehandle, preferredmaximum, prelationship, relationshipread, relationshiptotal)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetAddressStatus<P0>(serveripaddress: P0, subnetaddress: u32, pstatus: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetAddressStatus(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, pstatus : *mut u32) -> u32);
    DhcpV4FailoverGetAddressStatus(serveripaddress.param().abi(), subnetaddress, pstatus)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCPV4_FAILOVER_CLIENT_INFO) -> u32);
    DhcpV4FailoverGetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetRelationship<P0, P1>(serveripaddress: P0, prelationshipname: P1, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetRelationship(serveripaddress : windows_core::PCWSTR, prelationshipname : windows_core::PCWSTR, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverGetRelationship(serveripaddress.param().abi(), prelationshipname.param().abi(), prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeRelationship<P0>(serveripaddress: P0, scopeid: u32, prelationship: *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeRelationship(serveripaddress : windows_core::PCWSTR, scopeid : u32, prelationship : *mut *mut DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverGetScopeRelationship(serveripaddress.param().abi(), scopeid, prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetScopeStatistics<P0>(serveripaddress: P0, scopeid: u32, pstats: *mut *mut DHCP_FAILOVER_STATISTICS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetScopeStatistics(serveripaddress : windows_core::PCWSTR, scopeid : u32, pstats : *mut *mut DHCP_FAILOVER_STATISTICS) -> u32);
    DhcpV4FailoverGetScopeStatistics(serveripaddress.param().abi(), scopeid, pstats)
}
#[inline]
pub unsafe fn DhcpV4FailoverGetSystemTime<P0>(serveripaddress: P0, ptime: *mut u32, pmaxalloweddeltatime: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverGetSystemTime(serveripaddress : windows_core::PCWSTR, ptime : *mut u32, pmaxalloweddeltatime : *mut u32) -> u32);
    DhcpV4FailoverGetSystemTime(serveripaddress.param().abi(), ptime, pmaxalloweddeltatime)
}
#[inline]
pub unsafe fn DhcpV4FailoverSetRelationship<P0>(serveripaddress: P0, flags: u32, prelationship: *const DHCP_FAILOVER_RELATIONSHIP) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverSetRelationship(serveripaddress : windows_core::PCWSTR, flags : u32, prelationship : *const DHCP_FAILOVER_RELATIONSHIP) -> u32);
    DhcpV4FailoverSetRelationship(serveripaddress.param().abi(), flags, prelationship)
}
#[inline]
pub unsafe fn DhcpV4FailoverTriggerAddrAllocation<P0, P1>(serveripaddress: P0, pfailrelname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4FailoverTriggerAddrAllocation(serveripaddress : windows_core::PCWSTR, pfailrelname : windows_core::PCWSTR) -> u32);
    DhcpV4FailoverTriggerAddrAllocation(serveripaddress.param().abi(), pfailrelname.param().abi())
}
#[inline]
pub unsafe fn DhcpV4GetAllOptionValues<P0>(serveripaddress: P0, flags: u32, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, values: *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetAllOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, values : *mut *mut DHCP_ALL_OPTION_VALUES_PB) -> u32);
    DhcpV4GetAllOptionValues(serveripaddress.param().abi(), flags, scopeinfo, values)
}
#[inline]
pub unsafe fn DhcpV4GetClientInfo<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_PB) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfo(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_PB) -> u32);
    DhcpV4GetClientInfo(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpV4GetClientInfoEx<P0>(serveripaddress: P0, searchinfo: *const DHCP_SEARCH_INFO, clientinfo: *mut *mut DHCP_CLIENT_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetClientInfoEx(serveripaddress : windows_core::PCWSTR, searchinfo : *const DHCP_SEARCH_INFO, clientinfo : *mut *mut DHCP_CLIENT_INFO_EX) -> u32);
    DhcpV4GetClientInfoEx(serveripaddress.param().abi(), searchinfo, clientinfo)
}
#[inline]
pub unsafe fn DhcpV4GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: u32, startip: u32, endip: u32, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCP_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : u32, startip : u32, endip : u32, numfreeaddrreq : u32, ipaddrlist : *mut *mut DHCP_IP_ARRAY) -> u32);
    DhcpV4GetFreeIPAddress(serveripaddress.param().abi(), scopeid, startip, endip, numfreeaddrreq, ipaddrlist)
}
#[inline]
pub unsafe fn DhcpV4GetOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut *mut DHCP_OPTION_VALUE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut *mut DHCP_OPTION_VALUE) -> u32);
    DhcpV4GetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpV4GetPolicy<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *mut *mut DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicy(serveripaddress : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *mut *mut DHCP_POLICY) -> u32);
    DhcpV4GetPolicy(serveripaddress.param().abi(), fglobalpolicy.param().abi(), subnetaddress, policyname.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpV4GetPolicyEx<P0, P1, P2>(serveripaddress: P0, globalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *mut *mut DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4GetPolicyEx(serveripaddress : windows_core::PCWSTR, globalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *mut *mut DHCP_POLICY_EX) -> u32);
    DhcpV4GetPolicyEx(serveripaddress.param().abi(), globalpolicy.param().abi(), subnetaddress, policyname.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpV4QueryPolicyEnforcement<P0, P1>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, enabled: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4QueryPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, enabled : *mut super::super::Foundation:: BOOL) -> u32);
    DhcpV4QueryPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.param().abi(), subnetaddress, enabled)
}
#[inline]
pub unsafe fn DhcpV4RemoveOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4RemoveOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO) -> u32);
    DhcpV4RemoveOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo)
}
#[inline]
pub unsafe fn DhcpV4RemovePolicyRange<P0, P1>(serveripaddress: P0, subnetaddress: u32, policyname: P1, range: *const DHCP_IP_RANGE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4RemovePolicyRange(serveripaddress : windows_core::PCWSTR, subnetaddress : u32, policyname : windows_core::PCWSTR, range : *const DHCP_IP_RANGE) -> u32);
    DhcpV4RemovePolicyRange(serveripaddress.param().abi(), subnetaddress, policyname.param().abi(), range)
}
#[inline]
pub unsafe fn DhcpV4SetOptionValue<P0, P1, P2>(serveripaddress: P0, flags: u32, optionid: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalue: *mut DHCP_OPTION_DATA) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValue(serveripaddress : windows_core::PCWSTR, flags : u32, optionid : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalue : *mut DHCP_OPTION_DATA) -> u32);
    DhcpV4SetOptionValue(serveripaddress.param().abi(), flags, optionid, policyname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalue)
}
#[inline]
pub unsafe fn DhcpV4SetOptionValues<P0, P1, P2>(serveripaddress: P0, flags: u32, policyname: P1, vendorname: P2, scopeinfo: *mut DHCP_OPTION_SCOPE_INFO, optionvalues: *mut DHCP_OPTION_VALUE_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4SetOptionValues(serveripaddress : windows_core::PCWSTR, flags : u32, policyname : windows_core::PCWSTR, vendorname : windows_core::PCWSTR, scopeinfo : *mut DHCP_OPTION_SCOPE_INFO, optionvalues : *mut DHCP_OPTION_VALUE_ARRAY) -> u32);
    DhcpV4SetOptionValues(serveripaddress.param().abi(), flags, policyname.param().abi(), vendorname.param().abi(), scopeinfo, optionvalues)
}
#[inline]
pub unsafe fn DhcpV4SetPolicy<P0, P1, P2>(serveripaddress: P0, fieldsmodified: u32, fglobalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *const DHCP_POLICY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicy(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY) -> u32);
    DhcpV4SetPolicy(serveripaddress.param().abi(), fieldsmodified, fglobalpolicy.param().abi(), subnetaddress, policyname.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpV4SetPolicyEnforcement<P0, P1, P2>(serveripaddress: P0, fglobalpolicy: P1, subnetaddress: u32, enable: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEnforcement(serveripaddress : windows_core::PCWSTR, fglobalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, enable : super::super::Foundation:: BOOL) -> u32);
    DhcpV4SetPolicyEnforcement(serveripaddress.param().abi(), fglobalpolicy.param().abi(), subnetaddress, enable.param().abi())
}
#[inline]
pub unsafe fn DhcpV4SetPolicyEx<P0, P1, P2>(serveripaddress: P0, fieldsmodified: u32, globalpolicy: P1, subnetaddress: u32, policyname: P2, policy: *const DHCP_POLICY_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV4SetPolicyEx(serveripaddress : windows_core::PCWSTR, fieldsmodified : u32, globalpolicy : super::super::Foundation:: BOOL, subnetaddress : u32, policyname : windows_core::PCWSTR, policy : *const DHCP_POLICY_EX) -> u32);
    DhcpV4SetPolicyEx(serveripaddress.param().abi(), fieldsmodified, globalpolicy.param().abi(), subnetaddress, policyname.param().abi(), policy)
}
#[inline]
pub unsafe fn DhcpV6CreateClientInfo<P0>(serveripaddress: P0, clientinfo: *const DHCP_CLIENT_INFO_V6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV6CreateClientInfo(serveripaddress : windows_core::PCWSTR, clientinfo : *const DHCP_CLIENT_INFO_V6) -> u32);
    DhcpV6CreateClientInfo(serveripaddress.param().abi(), clientinfo)
}
#[inline]
pub unsafe fn DhcpV6GetFreeIPAddress<P0>(serveripaddress: P0, scopeid: DHCP_IPV6_ADDRESS, startip: DHCP_IPV6_ADDRESS, endip: DHCP_IPV6_ADDRESS, numfreeaddrreq: u32, ipaddrlist: *mut *mut DHCPV6_IP_ARRAY) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV6GetFreeIPAddress(serveripaddress : windows_core::PCWSTR, scopeid : DHCP_IPV6_ADDRESS, startip : DHCP_IPV6_ADDRESS, endip : DHCP_IPV6_ADDRESS, numfreeaddrreq : u32, ipaddrlist : *mut *mut DHCPV6_IP_ARRAY) -> u32);
    DhcpV6GetFreeIPAddress(serveripaddress.param().abi(), core::mem::transmute(scopeid), core::mem::transmute(startip), core::mem::transmute(endip), numfreeaddrreq, ipaddrlist)
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStatistics<P0>(serveripaddress: P0, statelessstats: *mut *mut DHCPV6_STATELESS_STATS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStatistics(serveripaddress : windows_core::PCWSTR, statelessstats : *mut *mut DHCPV6_STATELESS_STATS) -> u32);
    DhcpV6GetStatelessStatistics(serveripaddress.param().abi(), statelessstats)
}
#[inline]
pub unsafe fn DhcpV6GetStatelessStoreParams<P0, P1>(serveripaddress: P0, fserverlevel: P1, subnetaddress: DHCP_IPV6_ADDRESS, params: *mut *mut DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV6GetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : super::super::Foundation:: BOOL, subnetaddress : DHCP_IPV6_ADDRESS, params : *mut *mut DHCPV6_STATELESS_PARAMS) -> u32);
    DhcpV6GetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.param().abi(), core::mem::transmute(subnetaddress), params)
}
#[inline]
pub unsafe fn DhcpV6SetStatelessStoreParams<P0, P1>(serveripaddress: P0, fserverlevel: P1, subnetaddress: DHCP_IPV6_ADDRESS, fieldmodified: u32, params: *const DHCPV6_STATELESS_PARAMS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dhcpsapi.dll" "system" fn DhcpV6SetStatelessStoreParams(serveripaddress : windows_core::PCWSTR, fserverlevel : super::super::Foundation:: BOOL, subnetaddress : DHCP_IPV6_ADDRESS, fieldmodified : u32, params : *const DHCPV6_STATELESS_PARAMS) -> u32);
    DhcpV6SetStatelessStoreParams(serveripaddress.param().abi(), fserverlevel.param().abi(), core::mem::transmute(subnetaddress), fieldmodified, params)
}
#[inline]
pub unsafe fn Dhcpv6CApiCleanup() {
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6CApiCleanup());
    Dhcpv6CApiCleanup()
}
#[inline]
pub unsafe fn Dhcpv6CApiInitialize(version: *mut u32) {
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6CApiInitialize(version : *mut u32));
    Dhcpv6CApiInitialize(version)
}
#[inline]
pub unsafe fn Dhcpv6ReleasePrefix<P0>(adaptername: P0, classid: *mut DHCPV6CAPI_CLASSID, leaseinfo: *mut DHCPV6PrefixLeaseInformation) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6ReleasePrefix(adaptername : windows_core::PCWSTR, classid : *mut DHCPV6CAPI_CLASSID, leaseinfo : *mut DHCPV6PrefixLeaseInformation) -> u32);
    Dhcpv6ReleasePrefix(adaptername.param().abi(), classid, leaseinfo)
}
#[inline]
pub unsafe fn Dhcpv6RenewPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32, bvalidateprefix: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RenewPrefix(adaptername : windows_core::PCWSTR, pclassid : *mut DHCPV6CAPI_CLASSID, prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation, pdwtimetowait : *mut u32, bvalidateprefix : u32) -> u32);
    Dhcpv6RenewPrefix(adaptername.param().abi(), pclassid, prefixleaseinfo, pdwtimetowait, bvalidateprefix)
}
#[inline]
pub unsafe fn Dhcpv6RequestParams<P0, P1>(forcenewinform: P0, reserved: *mut core::ffi::c_void, adaptername: P1, classid: *mut DHCPV6CAPI_CLASSID, recdparams: DHCPV6CAPI_PARAMS_ARRAY, buffer: *mut u8, psize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RequestParams(forcenewinform : super::super::Foundation:: BOOL, reserved : *mut core::ffi::c_void, adaptername : windows_core::PCWSTR, classid : *mut DHCPV6CAPI_CLASSID, recdparams : DHCPV6CAPI_PARAMS_ARRAY, buffer : *mut u8, psize : *mut u32) -> u32);
    Dhcpv6RequestParams(forcenewinform.param().abi(), reserved, adaptername.param().abi(), classid, core::mem::transmute(recdparams), buffer, psize)
}
#[inline]
pub unsafe fn Dhcpv6RequestPrefix<P0>(adaptername: P0, pclassid: *mut DHCPV6CAPI_CLASSID, prefixleaseinfo: *mut DHCPV6PrefixLeaseInformation, pdwtimetowait: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dhcpcsvc6.dll" "system" fn Dhcpv6RequestPrefix(adaptername : windows_core::PCWSTR, pclassid : *mut DHCPV6CAPI_CLASSID, prefixleaseinfo : *mut DHCPV6PrefixLeaseInformation, pdwtimetowait : *mut u32) -> u32);
    Dhcpv6RequestPrefix(adaptername.param().abi(), pclassid, prefixleaseinfo, pdwtimetowait)
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
pub const DEFAULTQUARSETTING: QuarantineStatus = QuarantineStatus(5i32);
pub const DHCPCAPI_DEREGISTER_HANDLE_EVENT: u32 = 1u32;
pub const DHCPCAPI_REGISTER_HANDLE_EVENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_ASYNCHRONOUS: u32 = 4u32;
pub const DHCPCAPI_REQUEST_CANCEL: u32 = 8u32;
pub const DHCPCAPI_REQUEST_MASK: u32 = 15u32;
pub const DHCPCAPI_REQUEST_PERSISTENT: u32 = 1u32;
pub const DHCPCAPI_REQUEST_SYNCHRONOUS: u32 = 2u32;
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
pub const DHCP_ATTRIB_BOOL_IS_ADMIN: u32 = 5u32;
pub const DHCP_ATTRIB_BOOL_IS_BINDING_AWARE: u32 = 4u32;
pub const DHCP_ATTRIB_BOOL_IS_DYNBOOTP: u32 = 2u32;
pub const DHCP_ATTRIB_BOOL_IS_PART_OF_DSDC: u32 = 3u32;
pub const DHCP_ATTRIB_BOOL_IS_ROGUE: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_BOOL: u32 = 1u32;
pub const DHCP_ATTRIB_TYPE_ULONG: u32 = 2u32;
pub const DHCP_ATTRIB_ULONG_RESTORE_STATUS: u32 = 6u32;
pub const DHCP_CALLOUT_ENTRY_POINT: windows_core::PCSTR = windows_core::s!("DhcpServerCalloutEntry");
pub const DHCP_CALLOUT_LIST_KEY: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\DHCPServer\\Parameters");
pub const DHCP_CALLOUT_LIST_VALUE: windows_core::PCWSTR = windows_core::w!("CalloutDlls");
pub const DHCP_CLIENT_BOOTP: u32 = 805306371u32;
pub const DHCP_CLIENT_DHCP: u32 = 805306372u32;
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
pub const DHCP_FLAGS_DONT_ACCESS_DS: u32 = 1u32;
pub const DHCP_FLAGS_DONT_DO_RPC: u32 = 2u32;
pub const DHCP_FLAGS_OPTION_IS_VENDOR: u32 = 3u32;
pub const DHCP_GIVE_ADDRESS_NEW: u32 = 805306369u32;
pub const DHCP_GIVE_ADDRESS_OLD: u32 = 805306370u32;
pub const DHCP_MAX_DELAY: u32 = 1000u32;
pub const DHCP_MIN_DELAY: u32 = 0u32;
pub const DHCP_OPT_ENUM_IGNORE_VENDOR: u32 = 1u32;
pub const DHCP_OPT_ENUM_USE_CLASSNAME: u32 = 2u32;
pub const DHCP_PROB_CONFLICT: u32 = 536870913u32;
pub const DHCP_PROB_DECLINE: u32 = 536870914u32;
pub const DHCP_PROB_NACKED: u32 = 536870916u32;
pub const DHCP_PROB_RELEASE: u32 = 536870915u32;
pub const DHCP_SEND_PACKET: u32 = 268435456u32;
pub const DHCP_SUBNET_INFO_VQ_FLAG_QUARANTINE: u32 = 1u32;
pub const DNS_FLAG_CLEANUP_EXPIRED: u32 = 4u32;
pub const DNS_FLAG_DISABLE_PTR_UPDATE: u32 = 64u32;
pub const DNS_FLAG_ENABLED: u32 = 1u32;
pub const DNS_FLAG_HAS_DNS_SUFFIX: u32 = 128u32;
pub const DNS_FLAG_UPDATE_BOTH_ALWAYS: u32 = 16u32;
pub const DNS_FLAG_UPDATE_DHCID: u32 = 32u32;
pub const DNS_FLAG_UPDATE_DOWNLEVEL: u32 = 2u32;
pub const DROPPACKET: QuarantineStatus = QuarantineStatus(2i32);
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
pub const HWTYPE_ETHERNET_10MB: u32 = 1u32;
pub const HotStandby: DHCP_FAILOVER_MODE = DHCP_FAILOVER_MODE(1i32);
pub const INIT: FSM_STATE = FSM_STATE(1i32);
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
pub const RECOVER: FSM_STATE = FSM_STATE(9i32);
pub const RECOVER_DONE: FSM_STATE = FSM_STATE(11i32);
pub const RECOVER_WAIT: FSM_STATE = FSM_STATE(10i32);
pub const RESOLUTION_INT: FSM_STATE = FSM_STATE(8i32);
pub const RESTRICTEDACCESS: QuarantineStatus = QuarantineStatus(1i32);
pub const SAFEPERIOD: u32 = 2u32;
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
pub const V5_ADDRESS_BIT_BOTH_REC: u32 = 32u32;
pub const V5_ADDRESS_BIT_DELETED: u32 = 128u32;
pub const V5_ADDRESS_BIT_UNREGISTERED: u32 = 64u32;
pub const V5_ADDRESS_EX_BIT_DISABLE_PTR_RR: u32 = 1u32;
pub const V5_ADDRESS_STATE_ACTIVE: u32 = 1u32;
pub const V5_ADDRESS_STATE_DECLINED: u32 = 2u32;
pub const V5_ADDRESS_STATE_DOOM: u32 = 3u32;
pub const V5_ADDRESS_STATE_OFFERED: u32 = 0u32;
pub const WARNING_EXTENDED_LESS: i32 = 20026i32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCPV6_STATELESS_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for DHCPV6_STATELESS_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCPV6_STATELESS_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCPV6_STATELESS_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_FAILOVER_MODE(pub i32);
impl windows_core::TypeKind for DHCP_FAILOVER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_FAILOVER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_FAILOVER_SERVER(pub i32);
impl windows_core::TypeKind for DHCP_FAILOVER_SERVER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_FAILOVER_SERVER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_FAILOVER_SERVER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_FILTER_LIST_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_FILTER_LIST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_FILTER_LIST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_FILTER_LIST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_FORCE_FLAG(pub i32);
impl windows_core::TypeKind for DHCP_FORCE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_FORCE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_FORCE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_OPTION_DATA_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_OPTION_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_OPTION_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_OPTION_SCOPE_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_OPTION_SCOPE_TYPE6(pub i32);
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_TYPE6 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_OPTION_SCOPE_TYPE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_SCOPE_TYPE6").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_OPTION_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_OPTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_OPTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_OPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_POLICY_FIELDS_TO_UPDATE(pub i32);
impl windows_core::TypeKind for DHCP_POLICY_FIELDS_TO_UPDATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_POLICY_FIELDS_TO_UPDATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_POLICY_FIELDS_TO_UPDATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_POL_ATTR_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_POL_ATTR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_POL_ATTR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_POL_ATTR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_POL_COMPARATOR(pub i32);
impl windows_core::TypeKind for DHCP_POL_COMPARATOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_POL_COMPARATOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_POL_COMPARATOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_POL_LOGIC_OPER(pub i32);
impl windows_core::TypeKind for DHCP_POL_LOGIC_OPER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_POL_LOGIC_OPER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_POL_LOGIC_OPER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for DHCP_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SCAN_FLAG(pub i32);
impl windows_core::TypeKind for DHCP_SCAN_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SCAN_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SCAN_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SEARCH_INFO_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_SEARCH_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SEARCH_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SEARCH_INFO_TYPE_V6(pub i32);
impl windows_core::TypeKind for DHCP_SEARCH_INFO_TYPE_V6 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SEARCH_INFO_TYPE_V6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SEARCH_INFO_TYPE_V6").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SUBNET_ELEMENT_TYPE(pub i32);
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SUBNET_ELEMENT_TYPE_V6(pub i32);
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SUBNET_ELEMENT_TYPE_V6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_ELEMENT_TYPE_V6").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DHCP_SUBNET_STATE(pub i32);
impl windows_core::TypeKind for DHCP_SUBNET_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DHCP_SUBNET_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DHCP_SUBNET_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FSM_STATE(pub i32);
impl windows_core::TypeKind for FSM_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FSM_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FSM_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QuarantineStatus(pub i32);
impl windows_core::TypeKind for QuarantineStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QuarantineStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QuarantineStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StatusCode(pub i32);
impl windows_core::TypeKind for StatusCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StatusCode").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DATE_TIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
impl windows_core::TypeKind for DATE_TIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for DATE_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl windows_core::TypeKind for DHCPAPI_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPAPI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPCAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl windows_core::TypeKind for DHCPCAPI_CLASSID {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPCAPI_CLASSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPCAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPAPI_PARAMS,
}
impl windows_core::TypeKind for DHCPCAPI_PARAMS_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPCAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPDS_SERVER {
    pub Version: u32,
    pub ServerName: windows_core::PWSTR,
    pub ServerAddress: u32,
    pub Flags: u32,
    pub State: u32,
    pub DsLocation: windows_core::PWSTR,
    pub DsLocType: u32,
}
impl windows_core::TypeKind for DHCPDS_SERVER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPDS_SERVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPDS_SERVERS {
    pub Flags: u32,
    pub NumElements: u32,
    pub Servers: *mut DHCPDS_SERVER,
}
impl windows_core::TypeKind for DHCPDS_SERVERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPDS_SERVERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
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
impl windows_core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV4_FAILOVER_CLIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCPV4_FAILOVER_CLIENT_INFO,
}
impl windows_core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV4_FAILOVER_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
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
impl windows_core::TypeKind for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV4_FAILOVER_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6CAPI_CLASSID {
    pub Flags: u32,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl windows_core::TypeKind for DHCPV6CAPI_CLASSID {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6CAPI_CLASSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6CAPI_PARAMS {
    pub Flags: u32,
    pub OptionId: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Data: *mut u8,
    pub nBytesData: u32,
}
impl windows_core::TypeKind for DHCPV6CAPI_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6CAPI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6CAPI_PARAMS_ARRAY {
    pub nParams: u32,
    pub Params: *mut DHCPV6CAPI_PARAMS,
}
impl windows_core::TypeKind for DHCPV6CAPI_PARAMS_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6CAPI_PARAMS_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6Prefix {
    pub prefix: [u8; 16],
    pub prefixLength: u32,
    pub preferredLifeTime: u32,
    pub validLifeTime: u32,
    pub status: StatusCode,
}
impl windows_core::TypeKind for DHCPV6Prefix {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6Prefix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCPV6PrefixLeaseInformation {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6PrefixLeaseInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: DHCP_IPV6_ADDRESS,
    pub AdapterSubnetAddress: DHCP_IPV6_ADDRESS,
    pub IfDescription: windows_core::PWSTR,
    pub IpV6IfIndex: u32,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
impl windows_core::TypeKind for DHCPV6_BIND_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCPV6_BIND_ELEMENT,
}
impl windows_core::TypeKind for DHCPV6_BIND_ELEMENT_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IPV6_ADDRESS,
}
impl windows_core::TypeKind for DHCPV6_IP_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_IP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_STATELESS_PARAMS {
    pub Status: super::super::Foundation::BOOL,
    pub PurgeInterval: u32,
}
impl windows_core::TypeKind for DHCPV6_STATELESS_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_STATELESS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_STATELESS_SCOPE_STATS {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub NumStatelessClientsAdded: u64,
    pub NumStatelessClientsRemoved: u64,
}
impl windows_core::TypeKind for DHCPV6_STATELESS_SCOPE_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_STATELESS_SCOPE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCPV6_STATELESS_STATS {
    pub NumScopes: u32,
    pub ScopeStats: *mut DHCPV6_STATELESS_SCOPE_STATS,
}
impl windows_core::TypeKind for DHCPV6_STATELESS_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCPV6_STATELESS_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ADDR_PATTERN {
    pub MatchHWType: super::super::Foundation::BOOL,
    pub HWType: u8,
    pub IsWildcard: super::super::Foundation::BOOL,
    pub Length: u8,
    pub Pattern: [u8; 255],
}
impl windows_core::TypeKind for DHCP_ADDR_PATTERN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ADDR_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTIONS {
    pub Flags: u32,
    pub NonVendorOptions: *mut DHCP_OPTION_ARRAY,
    pub NumVendorOptions: u32,
    pub VendorOptions: *mut DHCP_ALL_OPTIONS_0,
}
impl windows_core::TypeKind for DHCP_ALL_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ALL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTIONS_0 {
    pub Option: DHCP_OPTION,
    pub VendorName: windows_core::PWSTR,
    pub ClassName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_ALL_OPTIONS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ALL_OPTIONS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_0,
}
impl windows_core::TypeKind for DHCP_ALL_OPTION_VALUES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ALL_OPTION_VALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_0 {
    pub ClassName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
impl windows_core::TypeKind for DHCP_ALL_OPTION_VALUES_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ALL_OPTION_VALUES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB {
    pub Flags: u32,
    pub NumElements: u32,
    pub Options: *mut DHCP_ALL_OPTION_VALUES_PB_0,
}
impl windows_core::TypeKind for DHCP_ALL_OPTION_VALUES_PB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ALL_OPTION_VALUES_PB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ALL_OPTION_VALUES_PB_0 {
    pub PolicyName: windows_core::PWSTR,
    pub VendorName: windows_core::PWSTR,
    pub IsVendor: super::super::Foundation::BOOL,
    pub OptionsArray: *mut DHCP_OPTION_VALUE_ARRAY,
}
impl windows_core::TypeKind for DHCP_ALL_OPTION_VALUES_PB_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_ATTRIB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ATTRIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DHCP_ATTRIB_0 {
    pub DhcpAttribBool: super::super::Foundation::BOOL,
    pub DhcpAttribUlong: u32,
}
impl windows_core::TypeKind for DHCP_ATTRIB_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ATTRIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_ATTRIB_ARRAY {
    pub NumElements: u32,
    pub DhcpAttribs: *mut DHCP_ATTRIB,
}
impl windows_core::TypeKind for DHCP_ATTRIB_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_ATTRIB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_BINARY_DATA {
    pub DataLength: u32,
    pub Data: *mut u8,
}
impl windows_core::TypeKind for DHCP_BINARY_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_BINARY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_BIND_ELEMENT {
    pub Flags: u32,
    pub fBoundToDHCPServer: super::super::Foundation::BOOL,
    pub AdapterPrimaryAddress: u32,
    pub AdapterSubnetAddress: u32,
    pub IfDescription: windows_core::PWSTR,
    pub IfIdSize: u32,
    pub IfId: *mut u8,
}
impl windows_core::TypeKind for DHCP_BIND_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_BIND_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_BIND_ELEMENT_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_BIND_ELEMENT,
}
impl windows_core::TypeKind for DHCP_BIND_ELEMENT_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_BIND_ELEMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_BOOTP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
    pub BootpAllocated: u32,
    pub MaxBootpAllowed: u32,
}
impl windows_core::TypeKind for DHCP_BOOTP_IP_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_BOOTP_IP_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for DHCP_CALLOUT_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CALLOUT_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLASS_INFO {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
impl windows_core::TypeKind for DHCP_CLASS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO,
}
impl windows_core::TypeKind for DHCP_CLASS_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLASS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLASS_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Classes: *mut DHCP_CLASS_INFO_V6,
}
impl windows_core::TypeKind for DHCP_CLASS_INFO_ARRAY_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLASS_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLASS_INFO_V6 {
    pub ClassName: windows_core::PWSTR,
    pub ClassComment: windows_core::PWSTR,
    pub ClassDataLength: u32,
    pub IsVendor: super::super::Foundation::BOOL,
    pub EnterpriseNumber: u32,
    pub Flags: u32,
    pub ClassData: *mut u8,
}
impl windows_core::TypeKind for DHCP_CLASS_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLASS_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
}
impl windows_core::TypeKind for DHCP_CLIENT_FILTER_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_FILTER_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_FILTER_STATUS_INFO,
}
impl windows_core::TypeKind for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_FILTER_STATUS_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO {
    pub ClientIpAddress: u32,
    pub SubnetMask: u32,
    pub ClientHardwareAddress: DHCP_BINARY_DATA,
    pub ClientName: windows_core::PWSTR,
    pub ClientComment: windows_core::PWSTR,
    pub ClientLeaseExpires: DATE_TIME,
    pub OwnerHost: DHCP_HOST_INFO,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V4,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V5,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_V6,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_ARRAY_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_ARRAY_VQ {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_VQ,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_ARRAY_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_ARRAY_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_EX_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_EX,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_EX_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_EX_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
    pub FilterStatus: u32,
    pub PolicyName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_PB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_PB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_CLIENT_INFO_PB_ARRAY {
    pub NumElements: u32,
    pub Clients: *mut *mut DHCP_CLIENT_INFO_PB,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_PB_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_PB_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_CLIENT_INFO_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_CLIENT_INFO_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_CLIENT_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub QuarantineCapable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_CLIENT_INFO_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_CLIENT_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_FAILOVER_RELATIONSHIP {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FAILOVER_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    pub NumElements: u32,
    pub pRelationships: *mut DHCP_FAILOVER_RELATIONSHIP,
}
impl windows_core::TypeKind for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FAILOVER_RELATIONSHIP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FAILOVER_STATISTICS {
    pub NumAddr: u32,
    pub AddrFree: u32,
    pub AddrInUse: u32,
    pub PartnerAddrFree: u32,
    pub ThisAddrFree: u32,
    pub PartnerAddrInUse: u32,
    pub ThisAddrInUse: u32,
}
impl windows_core::TypeKind for DHCP_FAILOVER_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FAILOVER_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FILTER_ADD_INFO {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
    pub ListType: DHCP_FILTER_LIST_TYPE,
}
impl windows_core::TypeKind for DHCP_FILTER_ADD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FILTER_ADD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FILTER_ENUM_INFO {
    pub NumElements: u32,
    pub pEnumRecords: *mut DHCP_FILTER_RECORD,
}
impl windows_core::TypeKind for DHCP_FILTER_ENUM_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FILTER_ENUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FILTER_GLOBAL_INFO {
    pub EnforceAllowList: super::super::Foundation::BOOL,
    pub EnforceDenyList: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_FILTER_GLOBAL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FILTER_GLOBAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_FILTER_RECORD {
    pub AddrPatt: DHCP_ADDR_PATTERN,
    pub Comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_FILTER_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_FILTER_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_HOST_INFO {
    pub IpAddress: u32,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_HOST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_HOST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_HOST_INFO_V6 {
    pub IpAddress: DHCP_IPV6_ADDRESS,
    pub NetBiosName: windows_core::PWSTR,
    pub HostName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_HOST_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_HOST_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IPV6_ADDRESS {
    pub HighOrderBits: u64,
    pub LowOrderBits: u64,
}
impl windows_core::TypeKind for DHCP_IPV6_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IPV6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut u32,
}
impl windows_core::TypeKind for DHCP_IP_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_CLUSTER {
    pub ClusterAddress: u32,
    pub ClusterMask: u32,
}
impl windows_core::TypeKind for DHCP_IP_CLUSTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_CLUSTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RANGE {
    pub StartAddress: u32,
    pub EndAddress: u32,
}
impl windows_core::TypeKind for DHCP_IP_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RANGE_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_IP_RANGE,
}
impl windows_core::TypeKind for DHCP_IP_RANGE_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RANGE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RANGE_V6 {
    pub StartAddress: DHCP_IPV6_ADDRESS,
    pub EndAddress: DHCP_IPV6_ADDRESS,
}
impl windows_core::TypeKind for DHCP_IP_RANGE_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RANGE_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RESERVATION {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
}
impl windows_core::TypeKind for DHCP_IP_RESERVATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RESERVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RESERVATION_INFO {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: DHCP_BINARY_DATA,
    pub ReservedClientName: windows_core::PWSTR,
    pub ReservedClientDesc: windows_core::PWSTR,
    pub bAllowedClientTypes: u8,
    pub fOptionsPresent: u8,
}
impl windows_core::TypeKind for DHCP_IP_RESERVATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RESERVATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RESERVATION_V4 {
    pub ReservedIpAddress: u32,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub bAllowedClientTypes: u8,
}
impl windows_core::TypeKind for DHCP_IP_RESERVATION_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RESERVATION_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_IP_RESERVATION_V6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedForClient: *mut DHCP_BINARY_DATA,
    pub InterfaceId: u32,
}
impl windows_core::TypeKind for DHCP_IP_RESERVATION_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_IP_RESERVATION_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_MIB_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_MIB_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_MIB_INFO_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_MIB_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_MIB_INFO_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION {
    pub OptionID: u32,
    pub OptionName: windows_core::PWSTR,
    pub OptionComment: windows_core::PWSTR,
    pub DefaultValue: DHCP_OPTION_DATA,
    pub OptionType: DHCP_OPTION_TYPE,
}
impl windows_core::TypeKind for DHCP_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION_ARRAY {
    pub NumElements: u32,
    pub Options: *mut DHCP_OPTION,
}
impl windows_core::TypeKind for DHCP_OPTION_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION_DATA {
    pub NumElements: u32,
    pub Elements: *mut DHCP_OPTION_DATA_ELEMENT,
}
impl windows_core::TypeKind for DHCP_OPTION_DATA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_DATA_ELEMENT {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_DATA_ELEMENT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION_DATA_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION_LIST {
    pub NumOptions: u32,
    pub Options: *mut DHCP_OPTION_VALUE,
}
impl windows_core::TypeKind for DHCP_OPTION_LIST {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_INFO {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_INFO_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_INFO6 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_OPTION_SCOPE_INFO6_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION_SCOPE_INFO6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION_VALUE {
    pub OptionID: u32,
    pub Value: DHCP_OPTION_DATA,
}
impl windows_core::TypeKind for DHCP_OPTION_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_OPTION_VALUE_ARRAY {
    pub NumElements: u32,
    pub Values: *mut DHCP_OPTION_VALUE,
}
impl windows_core::TypeKind for DHCP_OPTION_VALUE_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_OPTION_VALUE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_PERF_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_PERF_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POLICY {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POLICY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY,
}
impl windows_core::TypeKind for DHCP_POLICY_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POLICY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POLICY_EX {
    pub PolicyName: windows_core::PWSTR,
    pub IsGlobalPolicy: super::super::Foundation::BOOL,
    pub Subnet: u32,
    pub ProcessingOrder: u32,
    pub Conditions: *mut DHCP_POL_COND_ARRAY,
    pub Expressions: *mut DHCP_POL_EXPR_ARRAY,
    pub Ranges: *mut DHCP_IP_RANGE_ARRAY,
    pub Description: windows_core::PWSTR,
    pub Enabled: super::super::Foundation::BOOL,
    pub Properties: *mut DHCP_PROPERTY_ARRAY,
}
impl windows_core::TypeKind for DHCP_POLICY_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POLICY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POLICY_EX_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POLICY_EX,
}
impl windows_core::TypeKind for DHCP_POLICY_EX_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POLICY_EX_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_POL_COND {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POL_COND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POL_COND_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_COND,
}
impl windows_core::TypeKind for DHCP_POL_COND_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POL_COND_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POL_EXPR {
    pub ParentExpr: u32,
    pub Operator: DHCP_POL_LOGIC_OPER,
}
impl windows_core::TypeKind for DHCP_POL_EXPR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POL_EXPR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_POL_EXPR_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_POL_EXPR,
}
impl windows_core::TypeKind for DHCP_POL_EXPR_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_POL_EXPR_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_PROPERTY {
    pub ID: DHCP_PROPERTY_ID,
    pub Type: DHCP_PROPERTY_TYPE,
    pub Value: DHCP_PROPERTY_0,
}
impl windows_core::TypeKind for DHCP_PROPERTY {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_PROPERTY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_PROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_PROPERTY_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_PROPERTY,
}
impl windows_core::TypeKind for DHCP_PROPERTY_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_RESERVATION_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut *mut DHCP_IP_RESERVATION_INFO,
}
impl windows_core::TypeKind for DHCP_RESERVATION_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_RESERVATION_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_RESERVED_SCOPE {
    pub ReservedIpAddress: u32,
    pub ReservedIpSubnetAddress: u32,
}
impl windows_core::TypeKind for DHCP_RESERVED_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_RESERVED_SCOPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_RESERVED_SCOPE6 {
    pub ReservedIpAddress: DHCP_IPV6_ADDRESS,
    pub ReservedIpSubnetAddress: DHCP_IPV6_ADDRESS,
}
impl windows_core::TypeKind for DHCP_RESERVED_SCOPE6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_RESERVED_SCOPE6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SCAN_ITEM {
    pub IpAddress: u32,
    pub ScanFlag: DHCP_SCAN_FLAG,
}
impl windows_core::TypeKind for DHCP_SCAN_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SCAN_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SCAN_LIST {
    pub NumScanItems: u32,
    pub ScanItems: *mut DHCP_SCAN_ITEM,
}
impl windows_core::TypeKind for DHCP_SCAN_LIST {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SEARCH_INFO {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SEARCH_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SEARCH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SEARCH_INFO_V6 {
    pub SearchType: DHCP_SEARCH_INFO_TYPE_V6,
    pub SearchInfo: DHCP_SEARCH_INFO_V6_0,
}
impl windows_core::TypeKind for DHCP_SEARCH_INFO_V6 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SEARCH_INFO_V6_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SEARCH_INFO_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_SERVER_CONFIG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub fAuditLog: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_SERVER_CONFIG_INFO_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_CONFIG_INFO_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SERVER_CONFIG_INFO_V6 {
    pub UnicastFlag: super::super::Foundation::BOOL,
    pub RapidCommitFlag: super::super::Foundation::BOOL,
    pub PreferredLifetime: u32,
    pub ValidLifetime: u32,
    pub T1: u32,
    pub T2: u32,
    pub PreferredLifetimeIATA: u32,
    pub ValidLifetimeIATA: u32,
    pub fAuditLog: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_SERVER_CONFIG_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_CONFIG_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub fAuditLog: super::super::Foundation::BOOL,
    pub QuarantineOn: super::super::Foundation::BOOL,
    pub QuarDefFail: u32,
    pub QuarRuntimeStatus: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DHCP_SERVER_CONFIG_INFO_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_CONFIG_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub DSDomainNameRequested: super::super::Foundation::BOOLEAN,
    pub DSDomainName: windows_core::PSTR,
    pub DSDomainNameLen: u32,
    pub ScopeId: *mut u32,
}
impl windows_core::TypeKind for DHCP_SERVER_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SERVER_SPECIFIC_STRINGS {
    pub DefaultVendorClassName: windows_core::PWSTR,
    pub DefaultUserClassName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_SERVER_SPECIFIC_STRINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SERVER_SPECIFIC_STRINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DHCP_SUBNET_ELEMENT_DATA {
    pub ElementType: DHCP_SUBNET_ELEMENT_TYPE,
    pub Element: DHCP_SUBNET_ELEMENT_DATA_0,
}
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V4 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V4_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V5 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V5_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V6 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_ELEMENT_DATA_V6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA,
}
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V4,
}
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V5,
}
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    pub NumElements: u32,
    pub Elements: *mut DHCP_SUBNET_ELEMENT_DATA_V6,
}
impl windows_core::TypeKind for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_ELEMENT_INFO_ARRAY_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_INFO {
    pub SubnetAddress: u32,
    pub SubnetMask: u32,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub PrimaryHost: DHCP_HOST_INFO,
    pub SubnetState: DHCP_SUBNET_STATE,
}
impl windows_core::TypeKind for DHCP_SUBNET_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUBNET_INFO_V6 {
    pub SubnetAddress: DHCP_IPV6_ADDRESS,
    pub Prefix: u32,
    pub Preference: u16,
    pub SubnetName: windows_core::PWSTR,
    pub SubnetComment: windows_core::PWSTR,
    pub State: u32,
    pub ScopeId: u32,
}
impl windows_core::TypeKind for DHCP_SUBNET_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DHCP_SUBNET_INFO_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUBNET_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE {
    pub cEntries: u32,
    pub pEntries: *mut DHCP_SUPER_SCOPE_TABLE_ENTRY,
}
impl windows_core::TypeKind for DHCP_SUPER_SCOPE_TABLE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUPER_SCOPE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DHCP_SUPER_SCOPE_TABLE_ENTRY {
    pub SubnetAddress: u32,
    pub SuperScopeNumber: u32,
    pub NextInSuperScope: u32,
    pub SuperScopeName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DHCP_SUPER_SCOPE_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWORD_DWORD {
    pub DWord1: u32,
    pub DWord2: u32,
}
impl windows_core::TypeKind for DWORD_DWORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWORD_DWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCOPE_MIB_INFO {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl windows_core::TypeKind for SCOPE_MIB_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCOPE_MIB_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCOPE_MIB_INFO_V5 {
    pub Subnet: u32,
    pub NumAddressesInuse: u32,
    pub NumAddressesFree: u32,
    pub NumPendingOffers: u32,
}
impl windows_core::TypeKind for SCOPE_MIB_INFO_V5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCOPE_MIB_INFO_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCOPE_MIB_INFO_V6 {
    pub Subnet: DHCP_IPV6_ADDRESS,
    pub NumAddressesInuse: u64,
    pub NumAddressesFree: u64,
    pub NumPendingAdvertises: u64,
}
impl windows_core::TypeKind for SCOPE_MIB_INFO_V6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCOPE_MIB_INFO_V6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for SCOPE_MIB_INFO_VQ {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCOPE_MIB_INFO_VQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPDHCP_CONTROL = Option<unsafe extern "system" fn(dwcontrolcode: u32, lpreserved: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_DELETE_CLIENT = Option<unsafe extern "system" fn(ipaddress: u32, hwaddress: *mut u8, hwaddresslength: u32, reserved: u32, clienttype: u32) -> u32>;
pub type LPDHCP_DROP_SEND = Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, controlcode: u32, ipaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_ENTRY_POINT_FUNC = Option<unsafe extern "system" fn(chaindlls: windows_core::PCWSTR, calloutversion: u32, callouttbl: *mut DHCP_CALLOUT_TABLE) -> u32>;
pub type LPDHCP_GIVE_ADDRESS = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, addrtype: u32, leasetime: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDHCP_HANDLE_OPTIONS = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void, serveroptions: *mut DHCP_SERVER_OPTIONS) -> u32>;
pub type LPDHCP_NEWPKT = Option<unsafe extern "system" fn(packet: *mut *mut u8, packetsize: *mut u32, ipaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut *mut core::ffi::c_void, processit: *mut super::super::Foundation::BOOL) -> u32>;
pub type LPDHCP_PROB = Option<unsafe extern "system" fn(packet: *mut u8, packetsize: u32, controlcode: u32, ipaddress: u32, altaddress: u32, reserved: *mut core::ffi::c_void, pktcontext: *mut core::ffi::c_void) -> u32>;
