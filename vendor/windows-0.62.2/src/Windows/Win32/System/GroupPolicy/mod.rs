#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> windows_core::Result<()> {
    windows_core::link!("gpedit.dll" "system" fn BrowseForGPO(lpbrowseinfo : *mut GPOBROWSEINFO) -> windows_core::HRESULT);
    unsafe { BrowseForGPO(lpbrowseinfo as _).ok() }
}
#[inline]
pub unsafe fn CommandLineFromMsiDescriptor<P0>(descriptor: P0, commandline: windows_core::PWSTR, commandlinelength: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CommandLineFromMsiDescriptor(descriptor : windows_core::PCWSTR, commandline : windows_core::PWSTR, commandlinelength : *mut u32) -> u32);
    unsafe { CommandLineFromMsiDescriptor(descriptor.param().abi(), core::mem::transmute(commandline), commandlinelength as _) }
}
#[inline]
pub unsafe fn CreateGPOLink<P0, P1>(lpgpo: P0, lpcontainer: P1, fhighpriority: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn CreateGPOLink(lpgpo : windows_core::PCWSTR, lpcontainer : windows_core::PCWSTR, fhighpriority : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CreateGPOLink(lpgpo.param().abi(), lpcontainer.param().abi(), fhighpriority.into()).ok() }
}
#[inline]
pub unsafe fn DeleteAllGPOLinks<P0>(lpcontainer: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn DeleteAllGPOLinks(lpcontainer : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteAllGPOLinks(lpcontainer.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteGPOLink<P0, P1>(lpgpo: P0, lpcontainer: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn DeleteGPOLink(lpgpo : windows_core::PCWSTR, lpcontainer : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteGPOLink(lpgpo.param().abi(), lpcontainer.param().abi()).ok() }
}
#[inline]
pub unsafe fn EnterCriticalPolicySection(bmachine: bool) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : windows_core::BOOL) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { EnterCriticalPolicySection(bmachine.into()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn ExportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn ExportRSoPData(lpnamespace : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ExportRSoPData(lpnamespace.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : *const GROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { FreeGPOListA(pgpolist).ok() }
}
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : *const GROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { FreeGPOListW(pgpolist).ok() }
}
#[inline]
pub unsafe fn GenerateGPNotification<P1>(bmachine: bool, lpwszmgmtproduct: P1, dwmgmtproductoptions: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : windows_core::BOOL, lpwszmgmtproduct : windows_core::PCWSTR, dwmgmtproductoptions : u32) -> u32);
    unsafe { GenerateGPNotification(bmachine.into(), lpwszmgmtproduct.param().abi(), dwmgmtproductoptions) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetAppliedGPOListA<P1>(dwflags: u32, pmachinename: P1, psiduser: Option<super::super::Security::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : u32, pmachinename : windows_core::PCSTR, psiduser : super::super::Security:: PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut *mut GROUP_POLICY_OBJECTA) -> u32);
    unsafe { GetAppliedGPOListA(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetAppliedGPOListW<P1>(dwflags: u32, pmachinename: P1, psiduser: Option<super::super::Security::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : u32, pmachinename : windows_core::PCWSTR, psiduser : super::super::Security:: PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut *mut GROUP_POLICY_OBJECTW) -> u32);
    unsafe { GetAppliedGPOListW(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[inline]
pub unsafe fn GetGPOListA<P1, P2, P3>(htoken: Option<super::super::Foundation::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::super::Foundation:: HANDLE, lpname : windows_core::PCSTR, lphostname : windows_core::PCSTR, lpcomputername : windows_core::PCSTR, dwflags : u32, pgpolist : *mut *mut GROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { GetGPOListA(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _).ok() }
}
#[inline]
pub unsafe fn GetGPOListW<P1, P2, P3>(htoken: Option<super::super::Foundation::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::super::Foundation:: HANDLE, lpname : windows_core::PCWSTR, lphostname : windows_core::PCWSTR, lpcomputername : windows_core::PCWSTR, dwflags : u32, pgpolist : *mut *mut GROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { GetGPOListW(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _).ok() }
}
#[inline]
pub unsafe fn GetLocalManagedApplicationData<P0>(productcode: P0, displayname: *mut windows_core::PWSTR, supporturl: *mut windows_core::PWSTR)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetLocalManagedApplicationData(productcode : windows_core::PCWSTR, displayname : *mut windows_core::PWSTR, supporturl : *mut windows_core::PWSTR));
    unsafe { GetLocalManagedApplicationData(productcode.param().abi(), displayname as _, supporturl as _) }
}
#[inline]
pub unsafe fn GetLocalManagedApplications(buserapps: bool, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetLocalManagedApplications(buserapps : windows_core::BOOL, pdwapps : *mut u32, prglocalapps : *mut *mut LOCALMANAGEDAPPLICATION) -> u32);
    unsafe { GetLocalManagedApplications(buserapps.into(), pdwapps as _, prglocalapps as _) }
}
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn GetManagedApplicationCategories(dwreserved: Option<u32>, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetManagedApplicationCategories(dwreserved : u32, pappcategory : *mut super::super::UI::Shell:: APPCATEGORYINFOLIST) -> u32);
    unsafe { GetManagedApplicationCategories(dwreserved.unwrap_or(core::mem::zeroed()) as _, pappcategory as _) }
}
#[inline]
pub unsafe fn GetManagedApplications(pcategory: *const windows_core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn GetManagedApplications(pcategory : *const windows_core::GUID, dwqueryflags : u32, dwinfolevel : u32, pdwapps : *mut u32, prgmanagedapps : *mut *mut MANAGEDAPPLICATION) -> u32);
    unsafe { GetManagedApplications(pcategory, dwqueryflags, dwinfolevel, pdwapps as _, prgmanagedapps as _) }
}
#[inline]
pub unsafe fn ImportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn ImportRSoPData(lpnamespace : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ImportRSoPData(lpnamespace.param().abi(), lpfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn InstallApplication(pinstallinfo : *const INSTALLDATA) -> u32);
    unsafe { InstallApplication(pinstallinfo) }
}
#[inline]
pub unsafe fn LeaveCriticalPolicySection(hsection: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { LeaveCriticalPolicySection(hsection).ok() }
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const windows_core::GUID, pasynchandle: usize, dwstatus: u32) -> u32 {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : *const windows_core::GUID, pasynchandle : usize, dwstatus : u32) -> u32);
    unsafe { ProcessGroupPolicyCompleted(extensionid, pasynchandle, dwstatus) }
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const windows_core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: windows_core::HRESULT) -> u32 {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : *const windows_core::GUID, pasynchandle : usize, dwstatus : u32, rsopstatus : windows_core::HRESULT) -> u32);
    unsafe { ProcessGroupPolicyCompletedEx(extensionid, pasynchandle, dwstatus, rsopstatus) }
}
#[inline]
pub unsafe fn RefreshPolicy(bmachine: bool) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RefreshPolicy(bmachine.into()).ok() }
}
#[inline]
pub unsafe fn RefreshPolicyEx(bmachine: bool, dwoptions: u32) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : windows_core::BOOL, dwoptions : u32) -> windows_core::BOOL);
    unsafe { RefreshPolicyEx(bmachine.into(), dwoptions).ok() }
}
#[inline]
pub unsafe fn RegisterGPNotification(hevent: super::super::Foundation::HANDLE, bmachine: bool) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::super::Foundation:: HANDLE, bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterGPNotification(hevent, bmachine.into()).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RsopAccessCheckByType(psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, pprincipalselfsid: Option<super::super::Security::PSID>, prsoptoken: *const core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: Option<&[super::super::Security::OBJECT_TYPE_LIST]>, pgenericmapping: *const super::super::Security::GENERIC_MAPPING, pprivilegeset: Option<*const super::super::Security::PRIVILEGE_SET>, pdwprivilegesetlength: Option<*const u32>, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pprincipalselfsid : super::super::Security:: PSID, prsoptoken : *const core::ffi::c_void, dwdesiredaccessmask : u32, pobjecttypelist : *const super::super::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, pgenericmapping : *const super::super::Security:: GENERIC_MAPPING, pprivilegeset : *const super::super::Security:: PRIVILEGE_SET, pdwprivilegesetlength : *const u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RsopAccessCheckByType(psecuritydescriptor, pprincipalselfsid.unwrap_or(core::mem::zeroed()) as _, prsoptoken, dwdesiredaccessmask, core::mem::transmute(pobjecttypelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pgenericmapping, pprivilegeset.unwrap_or(core::mem::zeroed()) as _, pdwprivilegesetlength.unwrap_or(core::mem::zeroed()) as _, pdwgrantedaccessmask as _, pbaccessstatus as _).ok() }
}
#[inline]
pub unsafe fn RsopFileAccessCheck<P0>(pszfilename: P0, prsoptoken: *const core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut windows_core::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : windows_core::PCWSTR, prsoptoken : *const core::ffi::c_void, dwdesiredaccessmask : u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RsopFileAccessCheck(pszfilename.param().abi(), prsoptoken, dwdesiredaccessmask, pdwgrantedaccessmask as _, pbaccessstatus as _).ok() }
}
#[cfg(feature = "Win32_System_Wmi")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<P1, P2>(dwflags: u32, pservices: P1, psettinginstance: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::Wmi::IWbemServices>,
    P2: windows_core::Param<super::Wmi::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : u32, pservices : * mut core::ffi::c_void, psettinginstance : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RsopResetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Wmi")]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<P1, P2>(dwflags: u32, pservices: P1, psettinginstance: P2, pstatus: &[POLICYSETTINGSTATUSINFO]) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::Wmi::IWbemServices>,
    P2: windows_core::Param<super::Wmi::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : u32, pservices : * mut core::ffi::c_void, psettinginstance : * mut core::ffi::c_void, ninfo : u32, pstatus : *const POLICYSETTINGSTATUSINFO) -> windows_core::HRESULT);
    unsafe { RsopSetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi(), pstatus.len().try_into().unwrap(), core::mem::transmute(pstatus.as_ptr())).ok() }
}
#[inline]
pub unsafe fn UninstallApplication<P0>(productcode: P0, dwstatus: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn UninstallApplication(productcode : windows_core::PCWSTR, dwstatus : u32) -> u32);
    unsafe { UninstallApplication(productcode.param().abi(), dwstatus) }
}
#[inline]
pub unsafe fn UnregisterGPNotification(hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterGPNotification(hevent).ok() }
}
pub const ABSENT: APPSTATE = APPSTATE(0i32);
pub const ADMXCOMMENTS_EXTENSION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6c5a2a86_9eb3_42b9_aa83_a7371ba011b9);
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPSTATE(pub i32);
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
pub const CLSID_GPESnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: windows_core::GUID = windows_core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
pub const FLAG_NO_COMPUTER: u32 = 2u32;
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
pub const FLAG_NO_USER: u32 = 1u32;
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
pub const GPM: windows_core::GUID = windows_core::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: windows_core::GUID = windows_core::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: windows_core::GUID = windows_core::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: windows_core::GUID = windows_core::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: windows_core::GUID = windows_core::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: windows_core::GUID = windows_core::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMBackupType(pub i32);
pub const GPMCSECollection: windows_core::GUID = windows_core::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: windows_core::GUID = windows_core::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: windows_core::GUID = windows_core::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMDestinationOption(pub i32);
pub const GPMDomain: windows_core::GUID = windows_core::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMEntryType(pub i32);
pub const GPMGPO: windows_core::GUID = windows_core::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: windows_core::GUID = windows_core::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: windows_core::GUID = windows_core::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: windows_core::GUID = windows_core::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: windows_core::GUID = windows_core::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: windows_core::GUID = windows_core::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: windows_core::GUID = windows_core::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMPermissionType(pub i32);
pub const GPMRSOP: windows_core::GUID = windows_core::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMRSOPMode(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMReportType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMReportingOptions(pub i32);
pub const GPMResult: windows_core::GUID = windows_core::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: windows_core::GUID = windows_core::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: windows_core::GUID = windows_core::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMSOMType(pub i32);
pub const GPMSearchCriteria: windows_core::GUID = windows_core::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMSearchOperation(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMSearchProperty(pub i32);
pub const GPMSecurityInfo: windows_core::GUID = windows_core::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: windows_core::GUID = windows_core::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: windows_core::GUID = windows_core::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: windows_core::GUID = windows_core::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: windows_core::GUID = windows_core::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPMStarterGPOType(pub i32);
pub const GPMStatusMessage: windows_core::GUID = windows_core::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: windows_core::GUID = windows_core::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: windows_core::GUID = windows_core::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: windows_core::GUID = windows_core::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: windows_core::GUID = windows_core::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: windows_core::GUID = windows_core::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: windows_core::PWSTR,
    pub lpInitialOU: windows_core::PWSTR,
    pub lpDSPath: windows_core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: windows_core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
pub const GPO_FLAG_DISABLE: u32 = 1u32;
pub const GPO_FLAG_FORCE: u32 = 2u32;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPO_LINK(pub i32);
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPO_OPEN_FLAGS(pub u32);
pub const GPO_OPEN_LOAD_REGISTRY: GPO_OPEN_FLAGS = GPO_OPEN_FLAGS(1u32);
pub const GPO_OPEN_READ_ONLY: GPO_OPEN_FLAGS = GPO_OPEN_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPO_OPTIONS(pub u32);
impl GPO_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GPO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GPO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GPO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GPO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GPO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GPO_OPTION_DISABLE_MACHINE: GPO_OPTIONS = GPO_OPTIONS(2u32);
pub const GPO_OPTION_DISABLE_USER: GPO_OPTIONS = GPO_OPTIONS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPO_SECTION(pub u32);
pub const GPO_SECTION_MACHINE: GPO_SECTION = GPO_SECTION(2u32);
pub const GPO_SECTION_ROOT: GPO_SECTION = GPO_SECTION(0u32);
pub const GPO_SECTION_USER: GPO_SECTION = GPO_SECTION(1u32);
pub const GP_DLLNAME: windows_core::PCWSTR = windows_core::w!("DllName");
pub const GP_ENABLEASYNCHRONOUSPROCESSING: windows_core::PCWSTR = windows_core::w!("EnableAsynchronousProcessing");
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: windows_core::PCWSTR = windows_core::w!("MaxNoGPOListChangesInterval");
pub const GP_NOBACKGROUNDPOLICY: windows_core::PCWSTR = windows_core::w!("NoBackgroundPolicy");
pub const GP_NOGPOLISTCHANGES: windows_core::PCWSTR = windows_core::w!("NoGPOListChanges");
pub const GP_NOMACHINEPOLICY: windows_core::PCWSTR = windows_core::w!("NoMachinePolicy");
pub const GP_NOSLOWLINK: windows_core::PCWSTR = windows_core::w!("NoSlowLink");
pub const GP_NOTIFYLINKTRANSITION: windows_core::PCWSTR = windows_core::w!("NotifyLinkTransition");
pub const GP_NOUSERPOLICY: windows_core::PCWSTR = windows_core::w!("NoUserPolicy");
pub const GP_PERUSERLOCALSETTINGS: windows_core::PCWSTR = windows_core::w!("PerUserLocalSettings");
pub const GP_PROCESSGROUPPOLICY: windows_core::PCWSTR = windows_core::w!("ProcessGroupPolicy");
pub const GP_REQUIRESSUCCESSFULREGISTRY: windows_core::PCWSTR = windows_core::w!("RequiresSuccessfulRegistry");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PSTR,
    pub lpFileSysPath: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub szGPOName: [i8; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: windows_core::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: windows_core::PSTR,
}
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PWSTR,
    pub lpFileSysPath: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: windows_core::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: windows_core::PWSTR,
}
impl Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
pub const GROUP_POLICY_TRIGGER_EVENT_PROVIDER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbd2f4252_5e1e_49fc_9a30_f3978ad89ee2);
windows_core::imp::define_interface!(IGPEInformation, IGPEInformation_Vtbl, 0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGPEInformation, windows_core::IUnknown);
impl IGPEInformation {
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegistryKey)(windows_core::Interface::as_raw(self), dwsection, hkey as _).ok() }
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), dwoptions as _).ok() }
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), gpotype as _).ok() }
    }
    pub unsafe fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHint)(windows_core::Interface::as_raw(self), gphint as _).ok() }
    }
    pub unsafe fn PolicyChanged(&self, bmachine: bool, badd: bool, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PolicyChanged)(windows_core::Interface::as_raw(self), bmachine.into(), badd.into(), pguidextension as _, pguidsnapin as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub GetHint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IGPEInformation_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut u32) -> windows_core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()>;
    fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::Result<()>;
    fn PolicyChanged(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: IGPEInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&hkey)).into()
            }
        }
        unsafe extern "system" fn GetDSPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetOptions(this, core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetType(this, core::mem::transmute_copy(&gpotype)).into()
            }
        }
        unsafe extern "system" fn GetHint<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetHint(this, core::mem::transmute_copy(&gphint)).into()
            }
        }
        unsafe extern "system" fn PolicyChanged<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::PolicyChanged(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguidsnapin)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, OFFSET>,
            GetDSPath: GetDSPath::<Identity, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetHint: GetHint::<Identity, OFFSET>,
            PolicyChanged: PolicyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPEInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IGPEInformation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPM, IGPM_Vtbl, 0xf5fae809_3bd6_4da9_a65e_17665b41d763);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPM {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPM, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPM {
    pub unsafe fn GetDomain(&self, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMDomain> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomain)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdomain), core::mem::transmute_copy(bstrdomaincontroller), ldcflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackupDir(&self, bstrbackupdir: &windows_core::BSTR) -> windows_core::Result<IGPMBackupDir> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupDir)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupdir), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSitesContainer(&self, bstrforest: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMSitesContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSitesContainer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrforest), core::mem::transmute_copy(bstrdomain), core::mem::transmute_copy(bstrdomaincontroller), ldcflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRSOP(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: &windows_core::BSTR, lflags: i32) -> windows_core::Result<IGPMRSOP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRSOP)(windows_core::Interface::as_raw(self), gpmrsopmode, core::mem::transmute_copy(bstrnamespace), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePermission(&self, bstrtrustee: &windows_core::BSTR, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IGPMPermission> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePermission)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtrustee), perm, binheritable, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSearchCriteria(&self) -> windows_core::Result<IGPMSearchCriteria> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSearchCriteria)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<IGPMTrustee> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrustee)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtrustee), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetClientSideExtensions(&self) -> windows_core::Result<IGPMCSECollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientSideExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConstants(&self) -> windows_core::Result<IGPMConstants> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConstants)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMigrationTable(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<IGPMMigrationTable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMigrationTable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmigrationtablepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMigrationTable(&self) -> windows_core::Result<IGPMMigrationTable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMigrationTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InitializeReporting(&self, bstradmpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeReporting)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmpath)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSitesContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRSOP: unsafe extern "system" fn(*mut core::ffi::c_void, GPMRSOPMode, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GPMPermissionType, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSearchCriteria: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientSideExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConstants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMigrationTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMigrationTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeReporting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPM_Impl: super::Com::IDispatch_Impl {
    fn GetDomain(&self, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMDomain>;
    fn GetBackupDir(&self, bstrbackupdir: &windows_core::BSTR) -> windows_core::Result<IGPMBackupDir>;
    fn GetSitesContainer(&self, bstrforest: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMSitesContainer>;
    fn GetRSOP(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: &windows_core::BSTR, lflags: i32) -> windows_core::Result<IGPMRSOP>;
    fn CreatePermission(&self, bstrtrustee: &windows_core::BSTR, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IGPMPermission>;
    fn CreateSearchCriteria(&self) -> windows_core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(&self) -> windows_core::Result<IGPMCSECollection>;
    fn GetConstants(&self) -> windows_core::Result<IGPMConstants>;
    fn GetMigrationTable(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(&self) -> windows_core::Result<IGPMMigrationTable>;
    fn InitializeReporting(&self, bstradmpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPM_Vtbl {
    pub const fn new<Identity: IGPM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDomain<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomain: *mut core::ffi::c_void, bstrdomaincontroller: *mut core::ffi::c_void, ldcflags: i32, pigpmdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetDomain(this, core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrdomaincontroller), core::mem::transmute_copy(&ldcflags)) {
                    Ok(ok__) => {
                        pigpmdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupDir<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: *mut core::ffi::c_void, pigpmbackupdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetBackupDir(this, core::mem::transmute(&bstrbackupdir)) {
                    Ok(ok__) => {
                        pigpmbackupdir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSitesContainer<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrforest: *mut core::ffi::c_void, bstrdomain: *mut core::ffi::c_void, bstrdomaincontroller: *mut core::ffi::c_void, ldcflags: i32, ppigpmsitescontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetSitesContainer(this, core::mem::transmute(&bstrforest), core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrdomaincontroller), core::mem::transmute_copy(&ldcflags)) {
                    Ok(ok__) => {
                        ppigpmsitescontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRSOP<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: *mut core::ffi::c_void, lflags: i32, ppigpmrsop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetRSOP(this, core::mem::transmute_copy(&gpmrsopmode), core::mem::transmute(&bstrnamespace), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppigpmrsop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePermission<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: *mut core::ffi::c_void, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL, ppperm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::CreatePermission(this, core::mem::transmute(&bstrtrustee), core::mem::transmute_copy(&perm), core::mem::transmute_copy(&binheritable)) {
                    Ok(ok__) => {
                        ppperm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSearchCriteria<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsearchcriteria: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::CreateSearchCriteria(this) {
                    Ok(ok__) => {
                        ppigpmsearchcriteria.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTrustee<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: *mut core::ffi::c_void, ppigpmtrustee: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::CreateTrustee(this, core::mem::transmute(&bstrtrustee)) {
                    Ok(ok__) => {
                        ppigpmtrustee.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClientSideExtensions<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmcsecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetClientSideExtensions(this) {
                    Ok(ok__) => {
                        ppigpmcsecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConstants<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmconstants: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetConstants(this) {
                    Ok(ok__) => {
                        ppigpmconstants.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMigrationTable<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmigrationtablepath: *mut core::ffi::c_void, ppmigrationtable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::GetMigrationTable(this, core::mem::transmute(&bstrmigrationtablepath)) {
                    Ok(ok__) => {
                        ppmigrationtable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMigrationTable<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmigrationtable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM_Impl::CreateMigrationTable(this) {
                    Ok(ok__) => {
                        ppmigrationtable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeReporting<Identity: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPM_Impl::InitializeReporting(this, core::mem::transmute(&bstradmpath)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDomain: GetDomain::<Identity, OFFSET>,
            GetBackupDir: GetBackupDir::<Identity, OFFSET>,
            GetSitesContainer: GetSitesContainer::<Identity, OFFSET>,
            GetRSOP: GetRSOP::<Identity, OFFSET>,
            CreatePermission: CreatePermission::<Identity, OFFSET>,
            CreateSearchCriteria: CreateSearchCriteria::<Identity, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, OFFSET>,
            GetClientSideExtensions: GetClientSideExtensions::<Identity, OFFSET>,
            GetConstants: GetConstants::<Identity, OFFSET>,
            GetMigrationTable: GetMigrationTable::<Identity, OFFSET>,
            CreateMigrationTable: CreateMigrationTable::<Identity, OFFSET>,
            InitializeReporting: InitializeReporting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPM as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPM {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPM2, IGPM2_Vtbl, 0x00238f8a_3d86_41ac_8f5e_06a6638a634a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPM2 {
    type Target = IGPM;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPM2, windows_core::IUnknown, super::Com::IDispatch, IGPM);
#[cfg(feature = "Win32_System_Com")]
impl IGPM2 {
    pub unsafe fn GetBackupDirEx(&self, bstrbackupdir: &windows_core::BSTR, backupdirtype: GPMBackupType) -> windows_core::Result<IGPMBackupDirEx> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupDirEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupdir), backupdirtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InitializeReportingEx(&self, bstradmpath: &windows_core::BSTR, reportingoptions: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeReportingEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradmpath), reportingoptions).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM2_Vtbl {
    pub base__: IGPM_Vtbl,
    pub GetBackupDirEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GPMBackupType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeReportingEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPM2_Impl: IGPM_Impl {
    fn GetBackupDirEx(&self, bstrbackupdir: &windows_core::BSTR, backupdirtype: GPMBackupType) -> windows_core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(&self, bstradmpath: &windows_core::BSTR, reportingoptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPM2_Vtbl {
    pub const fn new<Identity: IGPM2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBackupDirEx<Identity: IGPM2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: *mut core::ffi::c_void, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPM2_Impl::GetBackupDirEx(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute_copy(&backupdirtype)) {
                    Ok(ok__) => {
                        ppigpmbackupdirex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeReportingEx<Identity: IGPM2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmpath: *mut core::ffi::c_void, reportingoptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPM2_Impl::InitializeReportingEx(this, core::mem::transmute(&bstradmpath), core::mem::transmute_copy(&reportingoptions)).into()
            }
        }
        Self {
            base__: IGPM_Vtbl::new::<Identity, OFFSET>(),
            GetBackupDirEx: GetBackupDirEx::<Identity, OFFSET>,
            InitializeReportingEx: InitializeReportingEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPM2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPM as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPM2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMAsyncCancel, IGPMAsyncCancel_Vtbl, 0xddc67754_be67_4541_8166_f48166868c9c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMAsyncCancel {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMAsyncCancel, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncCancel {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncCancel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMAsyncCancel_Impl: super::Com::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMAsyncCancel_Vtbl {
    pub const fn new<Identity: IGPMAsyncCancel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IGPMAsyncCancel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMAsyncCancel_Impl::Cancel(this).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMAsyncCancel {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMAsyncProgress, IGPMAsyncProgress_Vtbl, 0x6aac29f8_5948_4324_bf70_423818942dbc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMAsyncProgress {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMAsyncProgress, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncProgress {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Status<P4>(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IGPMStatusMsgCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), lprogressnumerator, lprogressdenominator, hrstatus, core::mem::transmute(presult), ppigpmstatusmsgcollection.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::HRESULT, *const super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Status: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMAsyncProgress_Impl: super::Com::IDispatch_Impl {
    fn Status(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: windows_core::Ref<IGPMStatusMsgCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMAsyncProgress_Vtbl {
    pub const fn new<Identity: IGPMAsyncProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IGPMAsyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMAsyncProgress_Impl::Status(this, core::mem::transmute_copy(&lprogressnumerator), core::mem::transmute_copy(&lprogressdenominator), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppigpmstatusmsgcollection)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Status: Status::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMAsyncProgress {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMBackup, IGPMBackup_Vtbl, 0xd8a16a35_3b0d_416b_8d02_4df6f95a7119);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMBackup {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMBackup, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackup {
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GPOID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPODomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GPODisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPODisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Comment(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Comment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupDir)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReportToFile)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute_copy(bstrtargetfilepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GPODisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackupDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub GenerateReportToFile: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackup_Impl: super::Com::IDispatch_Impl {
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
    fn Comment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMBackup_Vtbl {
    pub const fn new<Identity: IGPMBackup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ID<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::ID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GPOID<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::GPOID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GPODomain<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::GPODomain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GPODisplayName<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::GPODisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Comment<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::Comment(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BackupDir<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::BackupDir(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMBackup_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: *mut core::ffi::c_void, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackup_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ID: ID::<Identity, OFFSET>,
            GPOID: GPOID::<Identity, OFFSET>,
            GPODomain: GPODomain::<Identity, OFFSET>,
            GPODisplayName: GPODisplayName::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            Comment: Comment::<Identity, OFFSET>,
            BackupDir: BackupDir::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMBackup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMBackupCollection, IGPMBackupCollection_Vtbl, 0xc786fc0f_26d8_4bab_a745_39ca7e800cac);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMBackupCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMBackupCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMBackupCollection_Vtbl {
    pub const fn new<Identity: IGPMBackupCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmbackup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMBackupCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMBackupDir, IGPMBackupDir_Vtbl, 0xb1568bed_0a93_4acc_810f_afe7081019b9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMBackupDir {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMBackupDir, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDir {
    pub unsafe fn BackupDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<IGPMBackup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchBackups<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMBackupCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchBackups)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDir_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchBackups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupDir_Impl: super::Com::IDispatch_Impl {
    fn BackupDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<IGPMBackup>;
    fn SearchBackups(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMBackupCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMBackupDir_Vtbl {
    pub const fn new<Identity: IGPMBackupDir_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BackupDirectory<Identity: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDir_Impl::BackupDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackup<Identity: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrid: *mut core::ffi::c_void, ppbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDir_Impl::GetBackup(this, core::mem::transmute(&bstrid)) {
                    Ok(ok__) => {
                        ppbackup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmbackupcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDir_Impl::SearchBackups(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmbackupcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BackupDirectory: BackupDirectory::<Identity, OFFSET>,
            GetBackup: GetBackup::<Identity, OFFSET>,
            SearchBackups: SearchBackups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupDir as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMBackupDir {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMBackupDirEx, IGPMBackupDirEx_Vtbl, 0xf8dc55ed_3ba0_4864_aad4_d365189ee1d5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMBackupDirEx {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMBackupDirEx, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDirEx {
    pub unsafe fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupDir)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BackupType(&self) -> windows_core::Result<GPMBackupType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SearchBackups<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchBackups)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDirEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackupType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMBackupType) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetBackup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SearchBackups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SearchBackups: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupDirEx_Impl: super::Com::IDispatch_Impl {
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BackupType(&self) -> windows_core::Result<GPMBackupType>;
    fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn SearchBackups(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMBackupDirEx_Vtbl {
    pub const fn new<Identity: IGPMBackupDirEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BackupDir<Identity: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDirEx_Impl::BackupDir(this) {
                    Ok(ok__) => {
                        pbstrbackupdir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BackupType<Identity: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDirEx_Impl::BackupType(this) {
                    Ok(ok__) => {
                        pgpmbackuptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackup<Identity: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrid: *mut core::ffi::c_void, pvarbackup: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDirEx_Impl::GetBackup(this, core::mem::transmute(&bstrid)) {
                    Ok(ok__) => {
                        pvarbackup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, pvarbackupcollection: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMBackupDirEx_Impl::SearchBackups(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        pvarbackupcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BackupDir: BackupDir::<Identity, OFFSET>,
            BackupType: BackupType::<Identity, OFFSET>,
            GetBackup: GetBackup::<Identity, OFFSET>,
            SearchBackups: SearchBackups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupDirEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMBackupDirEx {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMCSECollection, IGPMCSECollection_Vtbl, 0x2e52a97d_0a4a_4a6f_85db_201622455da0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMCSECollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMCSECollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMCSECollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMCSECollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMCSECollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMCSECollection_Vtbl {
    pub const fn new<Identity: IGPMCSECollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMCSECollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMCSECollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmcses: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMCSECollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmcses.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMCSECollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMCSECollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMClientSideExtension, IGPMClientSideExtension_Vtbl, 0x69da7488_b8db_415e_9266_901be4d49928);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMClientSideExtension {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMClientSideExtension, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMClientSideExtension {
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsComputerEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMClientSideExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMClientSideExtension_Impl: super::Com::IDispatch_Impl {
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMClientSideExtension_Vtbl {
    pub const fn new<Identity: IGPMClientSideExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ID<Identity: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMClientSideExtension_Impl::ID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMClientSideExtension_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUserEnabled<Identity: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMClientSideExtension_Impl::IsUserEnabled(this) {
                    Ok(ok__) => {
                        pvbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMClientSideExtension_Impl::IsComputerEnabled(this) {
                    Ok(ok__) => {
                        pvbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ID: ID::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMClientSideExtension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMClientSideExtension {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMConstants, IGPMConstants_Vtbl, 0x50ef73e6_d35c_4c8d_be63_7ea5d2aac5c4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMConstants {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMConstants, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants {
    pub unsafe fn PermGPOApply(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermGPOApply)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermGPORead(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermGPORead)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermGPOEdit(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermGPOEdit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermGPOEditSecurityAndDelete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermGPOCustom(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermGPOCustom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermWMIFilterEdit(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermWMIFilterEdit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermWMIFilterFullControl(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermWMIFilterFullControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermWMIFilterCustom(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermWMIFilterCustom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMLink(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMLogging(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMLogging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMPlanning(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMPlanning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMGPOCreate(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMGPOCreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMWMICreate(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMWMICreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermSOMWMIFullControl(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermSOMWMIFullControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOPermissions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOEffectivePermissions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPODisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOWMIFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOID(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOComputerExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPOUserExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertySOMLinks(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertySOMLinks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyGPODomain(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyGPODomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyBackupMostRecent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchOpEquals(&self) -> windows_core::Result<GPMSearchOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchOpEquals)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchOpContains(&self) -> windows_core::Result<GPMSearchOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchOpContains)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchOpNotContains(&self) -> windows_core::Result<GPMSearchOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchOpNotContains)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchOpNotEquals(&self) -> windows_core::Result<GPMSearchOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchOpNotEquals)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UsePDC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsePDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UseAnyDC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseAnyDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoNotUseW2KDC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoNotUseW2KDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SOMSite(&self) -> windows_core::Result<GPMSOMType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SOMSite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SOMDomain(&self) -> windows_core::Result<GPMSOMType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SOMDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SOMOU(&self) -> windows_core::Result<GPMSOMType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SOMOU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_SecurityFlags(&self, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecurityFlags)(windows_core::Interface::as_raw(self), vbowner, vbgroup, vbdacl, vbsacl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DoNotValidateDC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoNotValidateDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportHTML(&self) -> windows_core::Result<GPMReportType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportHTML)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportXML(&self) -> windows_core::Result<GPMReportType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportXML)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RSOPModeUnknown(&self) -> windows_core::Result<GPMRSOPMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSOPModeUnknown)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RSOPModePlanning(&self) -> windows_core::Result<GPMRSOPMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSOPModePlanning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RSOPModeLogging(&self) -> windows_core::Result<GPMRSOPMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSOPModeLogging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeUser(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeComputer(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeLocalGroup(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeLocalGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeGlobalGroup(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeGlobalGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeUniversalGroup(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeUniversalGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeUNCPath(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeUNCPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryTypeUnknown(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryTypeUnknown)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestinationOptionSameAsSource(&self) -> windows_core::Result<GPMDestinationOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationOptionSameAsSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestinationOptionNone(&self) -> windows_core::Result<GPMDestinationOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationOptionNone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestinationOptionByRelativeName(&self) -> windows_core::Result<GPMDestinationOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationOptionByRelativeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestinationOptionSet(&self) -> windows_core::Result<GPMDestinationOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationOptionSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MigrationTableOnly(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MigrationTableOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProcessSecurity(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessSecurity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RsopLoggingNoComputer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RsopLoggingNoComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RsopLoggingNoUser(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RsopLoggingNoUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RsopPlanningAssumeSlowLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RsopPlanningLoopbackOption(&self, vbmerge: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RsopPlanningLoopbackOption)(windows_core::Interface::as_raw(self), vbmerge, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RsopPlanningAssumeUserWQLFilterTrue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RsopPlanningAssumeCompWQLFilterTrue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PermGPOApply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermGPORead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermGPOEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermGPOEditSecurityAndDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermGPOCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermWMIFilterEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermWMIFilterFullControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermWMIFilterCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMLogging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMPlanning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMGPOCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMWMICreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermSOMWMIFullControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub SearchPropertyGPOPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPOEffectivePermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPODisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPOWMIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPOComputerExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPOUserExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertySOMLinks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyGPODomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyBackupMostRecent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchOpEquals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchOperation) -> windows_core::HRESULT,
    pub SearchOpContains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchOperation) -> windows_core::HRESULT,
    pub SearchOpNotContains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchOperation) -> windows_core::HRESULT,
    pub SearchOpNotEquals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchOperation) -> windows_core::HRESULT,
    pub UsePDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UseAnyDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DoNotUseW2KDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SOMSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSOMType) -> windows_core::HRESULT,
    pub SOMDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSOMType) -> windows_core::HRESULT,
    pub SOMOU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSOMType) -> windows_core::HRESULT,
    pub get_SecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *mut i32) -> windows_core::HRESULT,
    pub DoNotValidateDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReportHTML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMReportType) -> windows_core::HRESULT,
    pub ReportXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMReportType) -> windows_core::HRESULT,
    pub RSOPModeUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMRSOPMode) -> windows_core::HRESULT,
    pub RSOPModePlanning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMRSOPMode) -> windows_core::HRESULT,
    pub RSOPModeLogging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMRSOPMode) -> windows_core::HRESULT,
    pub EntryTypeUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeLocalGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeGlobalGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeUniversalGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeUNCPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub EntryTypeUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
    pub DestinationOptionSameAsSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMDestinationOption) -> windows_core::HRESULT,
    pub DestinationOptionNone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMDestinationOption) -> windows_core::HRESULT,
    pub DestinationOptionByRelativeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMDestinationOption) -> windows_core::HRESULT,
    pub DestinationOptionSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMDestinationOption) -> windows_core::HRESULT,
    pub MigrationTableOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProcessSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RsopLoggingNoComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RsopLoggingNoUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RsopPlanningAssumeSlowLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RsopPlanningLoopbackOption: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut i32) -> windows_core::HRESULT,
    pub RsopPlanningAssumeUserWQLFilterTrue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RsopPlanningAssumeCompWQLFilterTrue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMConstants_Impl: super::Com::IDispatch_Impl {
    fn PermGPOApply(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPORead(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOEditSecurityAndDelete(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMLink(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMLogging(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMPlanning(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMGPOCreate(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMWMICreate(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMWMIFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn SearchPropertyGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOWMIFilter(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOID(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOComputerExtensions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOUserExtensions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertySOMLinks(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODomain(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyBackupMostRecent(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchOpEquals(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpContains(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotContains(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotEquals(&self) -> windows_core::Result<GPMSearchOperation>;
    fn UsePDC(&self) -> windows_core::Result<i32>;
    fn UseAnyDC(&self) -> windows_core::Result<i32>;
    fn DoNotUseW2KDC(&self) -> windows_core::Result<i32>;
    fn SOMSite(&self) -> windows_core::Result<GPMSOMType>;
    fn SOMDomain(&self) -> windows_core::Result<GPMSOMType>;
    fn SOMOU(&self) -> windows_core::Result<GPMSOMType>;
    fn get_SecurityFlags(&self, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32>;
    fn DoNotValidateDC(&self) -> windows_core::Result<i32>;
    fn ReportHTML(&self) -> windows_core::Result<GPMReportType>;
    fn ReportXML(&self) -> windows_core::Result<GPMReportType>;
    fn RSOPModeUnknown(&self) -> windows_core::Result<GPMRSOPMode>;
    fn RSOPModePlanning(&self) -> windows_core::Result<GPMRSOPMode>;
    fn RSOPModeLogging(&self) -> windows_core::Result<GPMRSOPMode>;
    fn EntryTypeUser(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeComputer(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeLocalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeGlobalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUniversalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUNCPath(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUnknown(&self) -> windows_core::Result<GPMEntryType>;
    fn DestinationOptionSameAsSource(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionNone(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionByRelativeName(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionSet(&self) -> windows_core::Result<GPMDestinationOption>;
    fn MigrationTableOnly(&self) -> windows_core::Result<i32>;
    fn ProcessSecurity(&self) -> windows_core::Result<i32>;
    fn RsopLoggingNoComputer(&self) -> windows_core::Result<i32>;
    fn RsopLoggingNoUser(&self) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeSlowLink(&self) -> windows_core::Result<i32>;
    fn get_RsopPlanningLoopbackOption(&self, vbmerge: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMConstants_Vtbl {
    pub const fn new<Identity: IGPMConstants_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PermGPOApply<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermGPOApply(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermGPORead<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermGPORead(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermGPOEdit<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermGPOEdit(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermGPOEditSecurityAndDelete(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermGPOCustom<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermGPOCustom(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermWMIFilterEdit<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermWMIFilterEdit(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermWMIFilterFullControl(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermWMIFilterCustom<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermWMIFilterCustom(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMLink<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMLink(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMLogging<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMLogging(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMPlanning<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMPlanning(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMGPOCreate<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMGPOCreate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMWMICreate<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMWMICreate(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::PermSOMWMIFullControl(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOPermissions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOEffectivePermissions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPODisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOWMIFilter(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOID<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOComputerExtensions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPOUserExtensions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertySOMLinks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyGPODomain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchPropertyBackupMostRecent(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchOpEquals<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchOpEquals(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchOpContains<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchOpContains(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchOpNotContains<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchOpNotContains(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchOpNotEquals<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SearchOpNotEquals(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsePDC<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::UsePDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseAnyDC<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::UseAnyDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoNotUseW2KDC<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DoNotUseW2KDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SOMSite<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SOMSite(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SOMDomain<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SOMDomain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SOMOU<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::SOMOU(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SecurityFlags<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::get_SecurityFlags(this, core::mem::transmute_copy(&vbowner), core::mem::transmute_copy(&vbgroup), core::mem::transmute_copy(&vbdacl), core::mem::transmute_copy(&vbsacl)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoNotValidateDC<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DoNotValidateDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportHTML<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::ReportHTML(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportXML<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::ReportXML(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RSOPModeUnknown<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RSOPModeUnknown(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RSOPModePlanning<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RSOPModePlanning(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RSOPModeLogging<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RSOPModeLogging(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeUser<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeUser(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeComputer<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeComputer(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeLocalGroup(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeGlobalGroup(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeUniversalGroup(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeUNCPath<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeUNCPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryTypeUnknown<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::EntryTypeUnknown(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DestinationOptionSameAsSource(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationOptionNone<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DestinationOptionNone(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DestinationOptionByRelativeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationOptionSet<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::DestinationOptionSet(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MigrationTableOnly<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::MigrationTableOnly(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessSecurity<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::ProcessSecurity(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RsopLoggingNoComputer(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RsopLoggingNoUser<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RsopLoggingNoUser(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RsopPlanningAssumeSlowLink(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RsopPlanningLoopbackOption<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbmerge: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::get_RsopPlanningLoopbackOption(this, core::mem::transmute_copy(&vbmerge)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RsopPlanningAssumeUserWQLFilterTrue(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Identity: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants_Impl::RsopPlanningAssumeCompWQLFilterTrue(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PermGPOApply: PermGPOApply::<Identity, OFFSET>,
            PermGPORead: PermGPORead::<Identity, OFFSET>,
            PermGPOEdit: PermGPOEdit::<Identity, OFFSET>,
            PermGPOEditSecurityAndDelete: PermGPOEditSecurityAndDelete::<Identity, OFFSET>,
            PermGPOCustom: PermGPOCustom::<Identity, OFFSET>,
            PermWMIFilterEdit: PermWMIFilterEdit::<Identity, OFFSET>,
            PermWMIFilterFullControl: PermWMIFilterFullControl::<Identity, OFFSET>,
            PermWMIFilterCustom: PermWMIFilterCustom::<Identity, OFFSET>,
            PermSOMLink: PermSOMLink::<Identity, OFFSET>,
            PermSOMLogging: PermSOMLogging::<Identity, OFFSET>,
            PermSOMPlanning: PermSOMPlanning::<Identity, OFFSET>,
            PermSOMGPOCreate: PermSOMGPOCreate::<Identity, OFFSET>,
            PermSOMWMICreate: PermSOMWMICreate::<Identity, OFFSET>,
            PermSOMWMIFullControl: PermSOMWMIFullControl::<Identity, OFFSET>,
            SearchPropertyGPOPermissions: SearchPropertyGPOPermissions::<Identity, OFFSET>,
            SearchPropertyGPOEffectivePermissions: SearchPropertyGPOEffectivePermissions::<Identity, OFFSET>,
            SearchPropertyGPODisplayName: SearchPropertyGPODisplayName::<Identity, OFFSET>,
            SearchPropertyGPOWMIFilter: SearchPropertyGPOWMIFilter::<Identity, OFFSET>,
            SearchPropertyGPOID: SearchPropertyGPOID::<Identity, OFFSET>,
            SearchPropertyGPOComputerExtensions: SearchPropertyGPOComputerExtensions::<Identity, OFFSET>,
            SearchPropertyGPOUserExtensions: SearchPropertyGPOUserExtensions::<Identity, OFFSET>,
            SearchPropertySOMLinks: SearchPropertySOMLinks::<Identity, OFFSET>,
            SearchPropertyGPODomain: SearchPropertyGPODomain::<Identity, OFFSET>,
            SearchPropertyBackupMostRecent: SearchPropertyBackupMostRecent::<Identity, OFFSET>,
            SearchOpEquals: SearchOpEquals::<Identity, OFFSET>,
            SearchOpContains: SearchOpContains::<Identity, OFFSET>,
            SearchOpNotContains: SearchOpNotContains::<Identity, OFFSET>,
            SearchOpNotEquals: SearchOpNotEquals::<Identity, OFFSET>,
            UsePDC: UsePDC::<Identity, OFFSET>,
            UseAnyDC: UseAnyDC::<Identity, OFFSET>,
            DoNotUseW2KDC: DoNotUseW2KDC::<Identity, OFFSET>,
            SOMSite: SOMSite::<Identity, OFFSET>,
            SOMDomain: SOMDomain::<Identity, OFFSET>,
            SOMOU: SOMOU::<Identity, OFFSET>,
            get_SecurityFlags: get_SecurityFlags::<Identity, OFFSET>,
            DoNotValidateDC: DoNotValidateDC::<Identity, OFFSET>,
            ReportHTML: ReportHTML::<Identity, OFFSET>,
            ReportXML: ReportXML::<Identity, OFFSET>,
            RSOPModeUnknown: RSOPModeUnknown::<Identity, OFFSET>,
            RSOPModePlanning: RSOPModePlanning::<Identity, OFFSET>,
            RSOPModeLogging: RSOPModeLogging::<Identity, OFFSET>,
            EntryTypeUser: EntryTypeUser::<Identity, OFFSET>,
            EntryTypeComputer: EntryTypeComputer::<Identity, OFFSET>,
            EntryTypeLocalGroup: EntryTypeLocalGroup::<Identity, OFFSET>,
            EntryTypeGlobalGroup: EntryTypeGlobalGroup::<Identity, OFFSET>,
            EntryTypeUniversalGroup: EntryTypeUniversalGroup::<Identity, OFFSET>,
            EntryTypeUNCPath: EntryTypeUNCPath::<Identity, OFFSET>,
            EntryTypeUnknown: EntryTypeUnknown::<Identity, OFFSET>,
            DestinationOptionSameAsSource: DestinationOptionSameAsSource::<Identity, OFFSET>,
            DestinationOptionNone: DestinationOptionNone::<Identity, OFFSET>,
            DestinationOptionByRelativeName: DestinationOptionByRelativeName::<Identity, OFFSET>,
            DestinationOptionSet: DestinationOptionSet::<Identity, OFFSET>,
            MigrationTableOnly: MigrationTableOnly::<Identity, OFFSET>,
            ProcessSecurity: ProcessSecurity::<Identity, OFFSET>,
            RsopLoggingNoComputer: RsopLoggingNoComputer::<Identity, OFFSET>,
            RsopLoggingNoUser: RsopLoggingNoUser::<Identity, OFFSET>,
            RsopPlanningAssumeSlowLink: RsopPlanningAssumeSlowLink::<Identity, OFFSET>,
            get_RsopPlanningLoopbackOption: get_RsopPlanningLoopbackOption::<Identity, OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue: RsopPlanningAssumeUserWQLFilterTrue::<Identity, OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue: RsopPlanningAssumeCompWQLFilterTrue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMConstants as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMConstants {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMConstants2, IGPMConstants2_Vtbl, 0x05ae21b0_ac09_4032_a26f_9e7da786dc19);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMConstants2 {
    type Target = IGPMConstants;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMConstants2, windows_core::IUnknown, super::Com::IDispatch, IGPMConstants);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants2 {
    pub unsafe fn BackupTypeGPO(&self) -> windows_core::Result<GPMBackupType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupTypeGPO)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BackupTypeStarterGPO(&self) -> windows_core::Result<GPMBackupType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupTypeStarterGPO)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StarterGPOTypeSystem(&self) -> windows_core::Result<GPMStarterGPOType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StarterGPOTypeSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StarterGPOTypeCustom(&self) -> windows_core::Result<GPMStarterGPOType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StarterGPOTypeCustom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyStarterGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyStarterGPOPermissions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyStarterGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyStarterGPOEffectivePermissions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyStarterGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyStarterGPODisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyStarterGPOID(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyStarterGPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchPropertyStarterGPODomain(&self) -> windows_core::Result<GPMSearchProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchPropertyStarterGPODomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermStarterGPORead(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermStarterGPORead)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermStarterGPOEdit(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermStarterGPOEdit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermStarterGPOFullControl(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermStarterGPOFullControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PermStarterGPOCustom(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PermStarterGPOCustom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportLegacy(&self) -> windows_core::Result<GPMReportingOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportLegacy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportComments(&self) -> windows_core::Result<GPMReportingOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportComments)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants2_Vtbl {
    pub base__: IGPMConstants_Vtbl,
    pub BackupTypeGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMBackupType) -> windows_core::HRESULT,
    pub BackupTypeStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMBackupType) -> windows_core::HRESULT,
    pub StarterGPOTypeSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMStarterGPOType) -> windows_core::HRESULT,
    pub StarterGPOTypeCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMStarterGPOType) -> windows_core::HRESULT,
    pub SearchPropertyStarterGPOPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyStarterGPOEffectivePermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyStarterGPODisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyStarterGPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub SearchPropertyStarterGPODomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSearchProperty) -> windows_core::HRESULT,
    pub PermStarterGPORead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermStarterGPOEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermStarterGPOFullControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub PermStarterGPOCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub ReportLegacy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMReportingOptions) -> windows_core::HRESULT,
    pub ReportComments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMReportingOptions) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMConstants2_Impl: IGPMConstants_Impl {
    fn BackupTypeGPO(&self) -> windows_core::Result<GPMBackupType>;
    fn BackupTypeStarterGPO(&self) -> windows_core::Result<GPMBackupType>;
    fn StarterGPOTypeSystem(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn StarterGPOTypeCustom(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn SearchPropertyStarterGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOID(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODomain(&self) -> windows_core::Result<GPMSearchProperty>;
    fn PermStarterGPORead(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn ReportLegacy(&self) -> windows_core::Result<GPMReportingOptions>;
    fn ReportComments(&self) -> windows_core::Result<GPMReportingOptions>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMConstants2_Vtbl {
    pub const fn new<Identity: IGPMConstants2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BackupTypeGPO<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMBackupType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::BackupTypeGPO(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMBackupType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::BackupTypeStarterGPO(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::StarterGPOTypeSystem(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::StarterGPOTypeCustom(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::SearchPropertyStarterGPOPermissions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::SearchPropertyStarterGPOEffectivePermissions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::SearchPropertyStarterGPODisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::SearchPropertyStarterGPOID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::SearchPropertyStarterGPODomain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermStarterGPORead<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::PermStarterGPORead(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermStarterGPOEdit<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::PermStarterGPOEdit(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::PermStarterGPOFullControl(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PermStarterGPOCustom<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::PermStarterGPOCustom(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportLegacy<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportingOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::ReportLegacy(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportComments<Identity: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportingOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMConstants2_Impl::ReportComments(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IGPMConstants_Vtbl::new::<Identity, OFFSET>(),
            BackupTypeGPO: BackupTypeGPO::<Identity, OFFSET>,
            BackupTypeStarterGPO: BackupTypeStarterGPO::<Identity, OFFSET>,
            StarterGPOTypeSystem: StarterGPOTypeSystem::<Identity, OFFSET>,
            StarterGPOTypeCustom: StarterGPOTypeCustom::<Identity, OFFSET>,
            SearchPropertyStarterGPOPermissions: SearchPropertyStarterGPOPermissions::<Identity, OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions: SearchPropertyStarterGPOEffectivePermissions::<Identity, OFFSET>,
            SearchPropertyStarterGPODisplayName: SearchPropertyStarterGPODisplayName::<Identity, OFFSET>,
            SearchPropertyStarterGPOID: SearchPropertyStarterGPOID::<Identity, OFFSET>,
            SearchPropertyStarterGPODomain: SearchPropertyStarterGPODomain::<Identity, OFFSET>,
            PermStarterGPORead: PermStarterGPORead::<Identity, OFFSET>,
            PermStarterGPOEdit: PermStarterGPOEdit::<Identity, OFFSET>,
            PermStarterGPOFullControl: PermStarterGPOFullControl::<Identity, OFFSET>,
            PermStarterGPOCustom: PermStarterGPOCustom::<Identity, OFFSET>,
            ReportLegacy: ReportLegacy::<Identity, OFFSET>,
            ReportComments: ReportComments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMConstants2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMConstants as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMConstants2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMDomain, IGPMDomain_Vtbl, 0x6b21cc14_5a00_4f44_a738_feec8a94c7e3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMDomain {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMDomain, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain {
    pub unsafe fn DomainController(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainController)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateGPO(&self) -> windows_core::Result<IGPMGPO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGPO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMGPO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPO)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchGPOs<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMGPOCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchGPOs)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreGPO<P0>(&self, pigpmbackup: P0, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>
    where
        P0: windows_core::Param<IGPMBackup>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RestoreGPO)(windows_core::Interface::as_raw(self), pigpmbackup.param().abi(), ldcflags, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSOM(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMSOM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSOM)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchSOMs<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMSOMCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchSOMs)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWMIFilter(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMWMIFilter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWMIFilter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchWMIFilters<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMWMIFilterCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchWMIFilters)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchGPOs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RestoreGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RestoreGPO: usize,
    pub GetSOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchSOMs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWMIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchWMIFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain_Impl: super::Com::IDispatch_Impl {
    fn DomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateGPO(&self) -> windows_core::Result<IGPMGPO>;
    fn GetGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMGPO>;
    fn SearchGPOs(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMGPOCollection>;
    fn RestoreGPO(&self, pigpmbackup: windows_core::Ref<IGPMBackup>, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GetSOM(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMSOM>;
    fn SearchSOMs(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMWMIFilterCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMDomain_Vtbl {
    pub const fn new<Identity: IGPMDomain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DomainController<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::DomainController(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Domain<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::Domain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGPO<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::CreateGPO(this) {
                    Ok(ok__) => {
                        ppnewgpo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGPO<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, ppgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::GetGPO(this, core::mem::transmute(&bstrguid)) {
                    Ok(ok__) => {
                        ppgpo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchGPOs<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmgpocollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::SearchGPOs(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmgpocollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreGPO<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmbackup: *mut core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::RestoreGPO(this, core::mem::transmute_copy(&pigpmbackup), core::mem::transmute_copy(&ldcflags), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSOM<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void, ppsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::GetSOM(this, core::mem::transmute(&bstrpath)) {
                    Ok(ok__) => {
                        ppsom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchSOMs<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmsomcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::SearchSOMs(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmsomcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void, ppwmifilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::GetWMIFilter(this, core::mem::transmute(&bstrpath)) {
                    Ok(ok__) => {
                        ppwmifilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchWMIFilters<Identity: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain_Impl::SearchWMIFilters(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmwmifiltercollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DomainController: DomainController::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            CreateGPO: CreateGPO::<Identity, OFFSET>,
            GetGPO: GetGPO::<Identity, OFFSET>,
            SearchGPOs: SearchGPOs::<Identity, OFFSET>,
            RestoreGPO: RestoreGPO::<Identity, OFFSET>,
            GetSOM: GetSOM::<Identity, OFFSET>,
            SearchSOMs: SearchSOMs::<Identity, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, OFFSET>,
            SearchWMIFilters: SearchWMIFilters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMDomain {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMDomain2, IGPMDomain2_Vtbl, 0x7ca6bb8b_f1eb_490a_938d_3c4e51c768e6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMDomain2 {
    type Target = IGPMDomain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMDomain2, windows_core::IUnknown, super::Com::IDispatch, IGPMDomain);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain2 {
    pub unsafe fn CreateStarterGPO(&self) -> windows_core::Result<IGPMStarterGPO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStarterGPO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGPOFromStarterGPO<P0>(&self, pgpotemplate: P0) -> windows_core::Result<IGPMGPO>
    where
        P0: windows_core::Param<IGPMStarterGPO>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGPOFromStarterGPO)(windows_core::Interface::as_raw(self), pgpotemplate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStarterGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMStarterGPO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStarterGPO)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchStarterGPOs<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMStarterGPOCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchStarterGPOs)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoadStarterGPO(&self, bstrloadfile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadStarterGPO)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrloadfile), boverwrite, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreStarterGPO<P0>(&self, pigpmtmplbackup: P0, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>
    where
        P0: windows_core::Param<IGPMStarterGPOBackup>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RestoreStarterGPO)(windows_core::Interface::as_raw(self), pigpmtmplbackup.param().abi(), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain2_Vtbl {
    pub base__: IGPMDomain_Vtbl,
    pub CreateStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGPOFromStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchStarterGPOs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoadStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoadStarterGPO: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RestoreStarterGPO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RestoreStarterGPO: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain2_Impl: IGPMDomain_Impl {
    fn CreateStarterGPO(&self) -> windows_core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(&self, pgpotemplate: windows_core::Ref<IGPMStarterGPO>) -> windows_core::Result<IGPMGPO>;
    fn GetStarterGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(&self, bstrloadfile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn RestoreStarterGPO(&self, pigpmtmplbackup: windows_core::Ref<IGPMStarterGPOBackup>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMDomain2_Vtbl {
    pub const fn new<Identity: IGPMDomain2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStarterGPO<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewtemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::CreateStarterGPO(this) {
                    Ok(ok__) => {
                        ppnewtemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpotemplate: *mut core::ffi::c_void, ppnewgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::CreateGPOFromStarterGPO(this, core::mem::transmute_copy(&pgpotemplate)) {
                    Ok(ok__) => {
                        ppnewgpo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStarterGPO<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::GetStarterGPO(this, core::mem::transmute(&bstrguid)) {
                    Ok(ok__) => {
                        pptemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchStarterGPOs<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmtemplatecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::SearchStarterGPOs(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmtemplatecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadStarterGPO<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloadfile: *mut core::ffi::c_void, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::LoadStarterGPO(this, core::mem::transmute(&bstrloadfile), core::mem::transmute_copy(&boverwrite), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreStarterGPO<Identity: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmtmplbackup: *mut core::ffi::c_void, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain2_Impl::RestoreStarterGPO(this, core::mem::transmute_copy(&pigpmtmplbackup), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IGPMDomain_Vtbl::new::<Identity, OFFSET>(),
            CreateStarterGPO: CreateStarterGPO::<Identity, OFFSET>,
            CreateGPOFromStarterGPO: CreateGPOFromStarterGPO::<Identity, OFFSET>,
            GetStarterGPO: GetStarterGPO::<Identity, OFFSET>,
            SearchStarterGPOs: SearchStarterGPOs::<Identity, OFFSET>,
            LoadStarterGPO: LoadStarterGPO::<Identity, OFFSET>,
            RestoreStarterGPO: RestoreStarterGPO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMDomain as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMDomain2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMDomain3, IGPMDomain3_Vtbl, 0x0077fdfe_88c7_4acf_a11d_d10a7c310a03);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMDomain3 {
    type Target = IGPMDomain2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMDomain3, windows_core::IUnknown, super::Com::IDispatch, IGPMDomain, IGPMDomain2);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain3 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InfrastructureDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInfrastructureDC)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInfrastructureFlags)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain3_Vtbl {
    pub base__: IGPMDomain2_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub InfrastructureDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain3_Impl: IGPMDomain2_Impl {
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMDomain3_Vtbl {
    pub const fn new<Identity: IGPMDomain3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GenerateReport<Identity: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain3_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InfrastructureDC<Identity: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMDomain3_Impl::InfrastructureDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMDomain3_Impl::SetInfrastructureDC(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMDomain3_Impl::SetInfrastructureFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: IGPMDomain2_Vtbl::new::<Identity, OFFSET>(),
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            InfrastructureDC: InfrastructureDC::<Identity, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMDomain as windows_core::Interface>::IID || iid == &<IGPMDomain2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMDomain3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPO, IGPMGPO_Vtbl, 0x58cc4352_1ca3_48e5_9864_1da4d6e0d60f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPO {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPO, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ModificationTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserDSVersionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserDSVersionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerDSVersionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserSysvolVersionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerSysvolVersionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWMIFilter(&self) -> windows_core::Result<IGPMWMIFilter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWMIFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetWMIFilter<P0>(&self, pigpmwmifilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMWMIFilter>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWMIFilter)(windows_core::Interface::as_raw(self), pigpmwmifilter.param().abi()).ok() }
    }
    pub unsafe fn SetUserEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserEnabled)(windows_core::Interface::as_raw(self), vbenabled).ok() }
    }
    pub unsafe fn SetComputerEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComputerEnabled)(windows_core::Interface::as_raw(self), vbenabled).ok() }
    }
    pub unsafe fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsComputerEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMSecurityInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityInfo)(windows_core::Interface::as_raw(self), psecurityinfo.param().abi()).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupdir), core::mem::transmute_copy(bstrcomment), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Import<P1>(&self, lflags: i32, pigpmbackup: P1, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>
    where
        P1: windows_core::Param<IGPMBackup>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self), lflags, pigpmbackup.param().abi(), core::mem::transmute(pvarmigrationtable), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReportToFile)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute_copy(bstrtargetfilepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo<P1>(&self, lflags: i32, pigpmdomain: P1, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>
    where
        P1: windows_core::Param<IGPMDomain>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), lflags, pigpmdomain.param().abi(), core::mem::transmute(pvarnewdisplayname), core::mem::transmute(pvarmigrationtable), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityDescriptor<P1>(&self, lflags: i32, psd: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), lflags, psd.param().abi()).ok() }
    }
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsACLConsistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsACLConsistent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MakeACLConsistent(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MakeACLConsistent)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub UserDSVersionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ComputerDSVersionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UserSysvolVersionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ComputerSysvolVersionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetWMIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWMIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetComputerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Import: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub GenerateReportToFile: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyTo: usize,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsACLConsistent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MakeACLConsistent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO_Impl: super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn ModificationTime(&self) -> windows_core::Result<f64>;
    fn UserDSVersionNumber(&self) -> windows_core::Result<i32>;
    fn ComputerDSVersionNumber(&self) -> windows_core::Result<i32>;
    fn UserSysvolVersionNumber(&self) -> windows_core::Result<i32>;
    fn ComputerSysvolVersionNumber(&self) -> windows_core::Result<i32>;
    fn GetWMIFilter(&self) -> windows_core::Result<IGPMWMIFilter>;
    fn SetWMIFilter(&self, pigpmwmifilter: windows_core::Ref<IGPMWMIFilter>) -> windows_core::Result<()>;
    fn SetUserEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetComputerEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: windows_core::Ref<IGPMSecurityInfo>) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn Import(&self, lflags: i32, pigpmbackup: windows_core::Ref<IGPMBackup>, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
    fn CopyTo(&self, lflags: i32, pigpmdomain: windows_core::Ref<IGPMDomain>, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn SetSecurityDescriptor(&self, lflags: i32, psd: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn GetSecurityDescriptor(&self, lflags: i32) -> windows_core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MakeACLConsistent(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPO_Vtbl {
    pub const fn new<Identity: IGPMGPO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetDisplayName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ID<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::ID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainName<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::DomainName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModificationTime<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::ModificationTime(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserDSVersionNumber<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::UserDSVersionNumber(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::ComputerDSVersionNumber(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::UserSysvolVersionNumber(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::ComputerSysvolVersionNumber(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmwmifilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::GetWMIFilter(this) {
                    Ok(ok__) => {
                        ppigpmwmifilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWMIFilter<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmwmifilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetWMIFilter(this, core::mem::transmute_copy(&pigpmwmifilter)).into()
            }
        }
        unsafe extern "system" fn SetUserEnabled<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetUserEnabled(this, core::mem::transmute_copy(&vbenabled)).into()
            }
        }
        unsafe extern "system" fn SetComputerEnabled<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetComputerEnabled(this, core::mem::transmute_copy(&vbenabled)).into()
            }
        }
        unsafe extern "system" fn IsUserEnabled<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::IsUserEnabled(this) {
                    Ok(ok__) => {
                        pvbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::IsComputerEnabled(this) {
                    Ok(ok__) => {
                        pvbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::GetSecurityInfo(this) {
                    Ok(ok__) => {
                        ppsecurityinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetSecurityInfo(this, core::mem::transmute_copy(&psecurityinfo)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Backup<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: *mut core::ffi::c_void, bstrcomment: *mut core::ffi::c_void, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::Backup(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute(&bstrcomment), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Import<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pigpmbackup: *mut core::ffi::c_void, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::Import(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pigpmbackup), core::mem::transmute_copy(&pvarmigrationtable), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: *mut core::ffi::c_void, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pigpmdomain: *mut core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::CopyTo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pigpmdomain), core::mem::transmute_copy(&pvarnewdisplayname), core::mem::transmute_copy(&pvarmigrationtable), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, psd: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::SetSecurityDescriptor(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&psd)).into()
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppsd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppsd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsACLConsistent<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbconsistent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO_Impl::IsACLConsistent(this) {
                    Ok(ok__) => {
                        pvbconsistent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeACLConsistent<Identity: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO_Impl::MakeACLConsistent(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            ID: ID::<Identity, OFFSET>,
            DomainName: DomainName::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            ModificationTime: ModificationTime::<Identity, OFFSET>,
            UserDSVersionNumber: UserDSVersionNumber::<Identity, OFFSET>,
            ComputerDSVersionNumber: ComputerDSVersionNumber::<Identity, OFFSET>,
            UserSysvolVersionNumber: UserSysvolVersionNumber::<Identity, OFFSET>,
            ComputerSysvolVersionNumber: ComputerSysvolVersionNumber::<Identity, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, OFFSET>,
            SetWMIFilter: SetWMIFilter::<Identity, OFFSET>,
            SetUserEnabled: SetUserEnabled::<Identity, OFFSET>,
            SetComputerEnabled: SetComputerEnabled::<Identity, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Backup: Backup::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            IsACLConsistent: IsACLConsistent::<Identity, OFFSET>,
            MakeACLConsistent: MakeACLConsistent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPO {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPO2, IGPMGPO2_Vtbl, 0x8a66a210_b78b_4d99_88e2_c306a817c925);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPO2 {
    type Target = IGPMGPO;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPO2, windows_core::IUnknown, super::Com::IDispatch, IGPMGPO);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO2 {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO2_Vtbl {
    pub base__: IGPMGPO_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO2_Impl: IGPMGPO_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPO2_Vtbl {
    pub const fn new<Identity: IGPMGPO2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO2_Impl::Description(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO2_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
            }
        }
        Self {
            base__: IGPMGPO_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMGPO as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPO2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPO3, IGPMGPO3_Vtbl, 0x7cf123a1_f94a_4112_bfae_6aa1db9cb248);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPO3 {
    type Target = IGPMGPO2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPO3, windows_core::IUnknown, super::Com::IDispatch, IGPMGPO, IGPMGPO2);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO3 {
    pub unsafe fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InfrastructureDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInfrastructureDC)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInfrastructureFlags)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO3_Vtbl {
    pub base__: IGPMGPO2_Vtbl,
    pub InfrastructureDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO3_Impl: IGPMGPO2_Impl {
    fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPO3_Vtbl {
    pub const fn new<Identity: IGPMGPO3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InfrastructureDC<Identity: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPO3_Impl::InfrastructureDC(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO3_Impl::SetInfrastructureDC(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPO3_Impl::SetInfrastructureFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: IGPMGPO2_Vtbl::new::<Identity, OFFSET>(),
            InfrastructureDC: InfrastructureDC::<Identity, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMGPO as windows_core::Interface>::IID || iid == &<IGPMGPO2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPO3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPOCollection, IGPMGPOCollection_Vtbl, 0xf0f0d5cf_70ca_4c39_9e29_b642f8726c01);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPOCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPOCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPOCollection_Vtbl {
    pub const fn new<Identity: IGPMGPOCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmgpos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmgpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPOCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPOLink, IGPMGPOLink_Vtbl, 0x434b99bd_5de7_478a_809c_c251721df70c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPOLink {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPOLink, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLink {
    pub unsafe fn GPOID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPODomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Enforced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enforced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnforced(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnforced)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn SOMLinkOrder(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SOMLinkOrder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SOM(&self) -> windows_core::Result<IGPMSOM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SOM)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Enforced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnforced: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SOMLinkOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOLink_Impl: super::Com::IDispatch_Impl {
    fn GPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enforced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnforced(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SOMLinkOrder(&self) -> windows_core::Result<i32>;
    fn SOM(&self) -> windows_core::Result<IGPMSOM>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPOLink_Vtbl {
    pub const fn new<Identity: IGPMGPOLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GPOID<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::GPOID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GPODomain<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::GPODomain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPOLink_Impl::SetEnabled(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Enforced<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::Enforced(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnforced<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPOLink_Impl::SetEnforced(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn SOMLinkOrder<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::SOMLinkOrder(this) {
                    Ok(ok__) => {
                        lval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SOM<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLink_Impl::SOM(this) {
                    Ok(ok__) => {
                        ppigpmsom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMGPOLink_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GPOID: GPOID::<Identity, OFFSET>,
            GPODomain: GPODomain::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Enforced: Enforced::<Identity, OFFSET>,
            SetEnforced: SetEnforced::<Identity, OFFSET>,
            SOMLinkOrder: SOMLinkOrder::<Identity, OFFSET>,
            SOM: SOM::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOLink as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPOLink {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMGPOLinksCollection, IGPMGPOLinksCollection_Vtbl, 0x189d7b68_16bd_4d0d_a2ec_2e6aa2288c7f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMGPOLinksCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMGPOLinksCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLinksCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLinksCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOLinksCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMGPOLinksCollection_Vtbl {
    pub const fn new<Identity: IGPMGPOLinksCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLinksCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLinksCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmlinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMGPOLinksCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmlinks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMGPOLinksCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMMapEntry, IGPMMapEntry_Vtbl, 0x8e79ad06_2381_4444_be4c_ff693e6e6f2b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMMapEntry {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMMapEntry, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntry {
    pub unsafe fn Source(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Destination(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DestinationOption(&self) -> windows_core::Result<GPMDestinationOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EntryType(&self) -> windows_core::Result<GPMEntryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EntryType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DestinationOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMDestinationOption) -> windows_core::HRESULT,
    pub EntryType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMEntryType) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMapEntry_Impl: super::Com::IDispatch_Impl {
    fn Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Destination(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DestinationOption(&self) -> windows_core::Result<GPMDestinationOption>;
    fn EntryType(&self) -> windows_core::Result<GPMEntryType>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMMapEntry_Vtbl {
    pub const fn new<Identity: IGPMMapEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Source<Identity: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntry_Impl::Source(this) {
                    Ok(ok__) => {
                        pbstrsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Destination<Identity: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntry_Impl::Destination(this) {
                    Ok(ok__) => {
                        pbstrdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationOption<Identity: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntry_Impl::DestinationOption(this) {
                    Ok(ok__) => {
                        pgpmdestoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EntryType<Identity: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntry_Impl::EntryType(this) {
                    Ok(ok__) => {
                        pgpmentrytype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Source: Source::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            DestinationOption: DestinationOption::<Identity, OFFSET>,
            EntryType: EntryType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMapEntry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMMapEntry {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMMapEntryCollection, IGPMMapEntryCollection_Vtbl, 0xbb0bf49b_e53f_443f_b807_8be22bfb6d42);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMMapEntryCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMMapEntryCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntryCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMapEntryCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMMapEntryCollection_Vtbl {
    pub const fn new<Identity: IGPMMapEntryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntryCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMapEntryCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMMapEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMMigrationTable, IGPMMigrationTable_Vtbl, 0x48f823b1_efaf_470b_b6ed_40d14ee1a4ec);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMMigrationTable {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMMigrationTable, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMigrationTable {
    pub unsafe fn Save(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmigrationtablepath)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, lflags: i32, var: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(var)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddEntry(&self, bstrsource: &windows_core::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT) -> windows_core::Result<IGPMMapEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsource), gpmentrytype, core::mem::transmute(pvardestination), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<IGPMMapEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsource), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteEntry)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsource)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UpdateDestination(&self, bstrsource: &windows_core::BSTR, pvardestination: *const super::Variant::VARIANT) -> windows_core::Result<IGPMMapEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateDestination)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsource), core::mem::transmute(pvardestination), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Validate(&self) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEntries(&self) -> windows_core::Result<IGPMMapEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMigrationTable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, GPMEntryType, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddEntry: usize,
    pub GetEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UpdateDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UpdateDestination: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMigrationTable_Impl: super::Com::IDispatch_Impl {
    fn Save(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, lflags: i32, var: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddEntry(&self, bstrsource: &windows_core::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT) -> windows_core::Result<IGPMMapEntry>;
    fn GetEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<IGPMMapEntry>;
    fn DeleteEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateDestination(&self, bstrsource: &windows_core::BSTR, pvardestination: *const super::Variant::VARIANT) -> windows_core::Result<IGPMMapEntry>;
    fn Validate(&self) -> windows_core::Result<IGPMResult>;
    fn GetEntries(&self) -> windows_core::Result<IGPMMapEntryCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMMigrationTable_Vtbl {
    pub const fn new<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Save<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmigrationtablepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMMigrationTable_Impl::Save(this, core::mem::transmute(&bstrmigrationtablepath)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, var: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMMigrationTable_Impl::Add(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn AddEntry<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: *mut core::ffi::c_void, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMigrationTable_Impl::AddEntry(this, core::mem::transmute(&bstrsource), core::mem::transmute_copy(&gpmentrytype), core::mem::transmute_copy(&pvardestination)) {
                    Ok(ok__) => {
                        ppentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntry<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: *mut core::ffi::c_void, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMigrationTable_Impl::GetEntry(this, core::mem::transmute(&bstrsource)) {
                    Ok(ok__) => {
                        ppentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteEntry<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMMigrationTable_Impl::DeleteEntry(this, core::mem::transmute(&bstrsource)).into()
            }
        }
        unsafe extern "system" fn UpdateDestination<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: *mut core::ffi::c_void, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMigrationTable_Impl::UpdateDestination(this, core::mem::transmute(&bstrsource), core::mem::transmute_copy(&pvardestination)) {
                    Ok(ok__) => {
                        ppentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Validate<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMigrationTable_Impl::Validate(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntries<Identity: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppentries: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMMigrationTable_Impl::GetEntries(this) {
                    Ok(ok__) => {
                        ppentries.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddEntry: AddEntry::<Identity, OFFSET>,
            GetEntry: GetEntry::<Identity, OFFSET>,
            DeleteEntry: DeleteEntry::<Identity, OFFSET>,
            UpdateDestination: UpdateDestination::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
            GetEntries: GetEntries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMigrationTable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMMigrationTable {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMPermission, IGPMPermission_Vtbl, 0x35ebca40_e1a1_4a02_8905_d79416fb464a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMPermission {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMPermission, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMPermission {
    pub unsafe fn Inherited(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inherited)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Inheritable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inheritable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Denied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Denied)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Permission(&self) -> windows_core::Result<GPMPermissionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Permission)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Trustee(&self) -> windows_core::Result<IGPMTrustee> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trustee)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMPermission_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Inherited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Inheritable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Denied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Permission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMPermissionType) -> windows_core::HRESULT,
    pub Trustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMPermission_Impl: super::Com::IDispatch_Impl {
    fn Inherited(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Inheritable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Denied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Permission(&self) -> windows_core::Result<GPMPermissionType>;
    fn Trustee(&self) -> windows_core::Result<IGPMTrustee>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMPermission_Vtbl {
    pub const fn new<Identity: IGPMPermission_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Inherited<Identity: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMPermission_Impl::Inherited(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Inheritable<Identity: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMPermission_Impl::Inheritable(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Denied<Identity: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMPermission_Impl::Denied(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Permission<Identity: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMPermission_Impl::Permission(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Trustee<Identity: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtrustee: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMPermission_Impl::Trustee(this) {
                    Ok(ok__) => {
                        ppigpmtrustee.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Inherited: Inherited::<Identity, OFFSET>,
            Inheritable: Inheritable::<Identity, OFFSET>,
            Denied: Denied::<Identity, OFFSET>,
            Permission: Permission::<Identity, OFFSET>,
            Trustee: Trustee::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMPermission as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMPermission {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMRSOP, IGPMRSOP_Vtbl, 0x49ed785a_3237_4ff2_b1f0_fdf5a8d5a1ee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMRSOP {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMRSOP, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMRSOP {
    pub unsafe fn Mode(&self) -> windows_core::Result<GPMRSOPMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Namespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Namespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLoggingComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLoggingComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn LoggingComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggingComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLoggingUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLoggingUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn LoggingUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggingUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLoggingFlags(&self, lval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLoggingFlags)(windows_core::Interface::as_raw(self), lval).ok() }
    }
    pub unsafe fn LoggingFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggingFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlanningFlags(&self, lval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningFlags)(windows_core::Interface::as_raw(self), lval).ok() }
    }
    pub unsafe fn PlanningFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlanningDomainController(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningDomainController)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningDomainController(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningDomainController)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPlanningSiteName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningSiteName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningSiteName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningSiteName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPlanningUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningUser)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPlanningUserSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningUserSOM)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningUserSOM(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningUserSOM)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningUserWMIFilters(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningUserWMIFilters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varval)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningUserWMIFilters(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningUserWMIFilters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningUserSecurityGroups(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningUserSecurityGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varval)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningUserSecurityGroups(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningUserSecurityGroups)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPlanningComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningComputer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPlanningComputerSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningComputerSOM)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrval)).ok() }
    }
    pub unsafe fn PlanningComputerSOM(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningComputerSOM)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningComputerWMIFilters(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningComputerWMIFilters)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varval)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningComputerWMIFilters(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningComputerWMIFilters)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningComputerSecurityGroups(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlanningComputerSecurityGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varval)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningComputerSecurityGroups(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlanningComputerSecurityGroups)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoggingEnumerateUsers(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggingEnumerateUsers)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateQueryResults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateQueryResults)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseQueryResults(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseQueryResults)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReportToFile)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute_copy(bstrtargetfilepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMRSOP_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMRSOPMode) -> windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoggingComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoggingComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoggingUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoggingUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoggingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LoggingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPlanningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PlanningFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPlanningDomainController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningDomainController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlanningSiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningSiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlanningUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlanningUserSOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningUserSOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningUserWMIFilters: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningUserWMIFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningUserSecurityGroups: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningUserSecurityGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningUserSecurityGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningUserSecurityGroups: usize,
    pub SetPlanningComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlanningComputerSOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlanningComputerSOM: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningComputerWMIFilters: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningComputerWMIFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningComputerSecurityGroups: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningComputerSecurityGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoggingEnumerateUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoggingEnumerateUsers: usize,
    pub CreateQueryResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseQueryResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub GenerateReportToFile: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMRSOP_Impl: super::Com::IDispatch_Impl {
    fn Mode(&self) -> windows_core::Result<GPMRSOPMode>;
    fn Namespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoggingComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoggingUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingFlags(&self, lval: i32) -> windows_core::Result<()>;
    fn LoggingFlags(&self) -> windows_core::Result<i32>;
    fn SetPlanningFlags(&self, lval: i32) -> windows_core::Result<()>;
    fn PlanningFlags(&self) -> windows_core::Result<i32>;
    fn SetPlanningDomainController(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningDomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningSiteName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningSiteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUserSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningUserSOM(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUserWMIFilters(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PlanningUserWMIFilters(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningUserSecurityGroups(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PlanningUserSecurityGroups(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningComputerSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningComputerSOM(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningComputerWMIFilters(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PlanningComputerWMIFilters(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningComputerSecurityGroups(&self, varval: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PlanningComputerSecurityGroups(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn LoggingEnumerateUsers(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn CreateQueryResults(&self) -> windows_core::Result<()>;
    fn ReleaseQueryResults(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMRSOP_Vtbl {
    pub const fn new<Identity: IGPMRSOP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Mode<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::Mode(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Namespace<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::Namespace(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoggingComputer<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetLoggingComputer(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn LoggingComputer<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::LoggingComputer(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoggingUser<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetLoggingUser(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn LoggingUser<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::LoggingUser(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLoggingFlags<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetLoggingFlags(this, core::mem::transmute_copy(&lval)).into()
            }
        }
        unsafe extern "system" fn LoggingFlags<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::LoggingFlags(this) {
                    Ok(ok__) => {
                        lval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningFlags<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningFlags(this, core::mem::transmute_copy(&lval)).into()
            }
        }
        unsafe extern "system" fn PlanningFlags<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningFlags(this) {
                    Ok(ok__) => {
                        lval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningDomainController<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningDomainController(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningDomainController<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningDomainController(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningSiteName<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningSiteName(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningSiteName<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningSiteName(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningUser<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningUser(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningUser<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningUser(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningUserSOM<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningUserSOM(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningUserSOM<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningUserSOM(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningUserWMIFilters(this, core::mem::transmute(&varval)).into()
            }
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningUserWMIFilters(this) {
                    Ok(ok__) => {
                        varval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningUserSecurityGroups(this, core::mem::transmute(&varval)).into()
            }
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningUserSecurityGroups(this) {
                    Ok(ok__) => {
                        varval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningComputer<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningComputer(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningComputer<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningComputer(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningComputerSOM(this, core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn PlanningComputerSOM<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningComputerSOM(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningComputerWMIFilters(this, core::mem::transmute(&varval)).into()
            }
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningComputerWMIFilters(this) {
                    Ok(ok__) => {
                        varval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::SetPlanningComputerSecurityGroups(this, core::mem::transmute(&varval)).into()
            }
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::PlanningComputerSecurityGroups(this) {
                    Ok(ok__) => {
                        varval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::LoggingEnumerateUsers(this) {
                    Ok(ok__) => {
                        varval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQueryResults<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::CreateQueryResults(this).into()
            }
        }
        unsafe extern "system" fn ReleaseQueryResults<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMRSOP_Impl::ReleaseQueryResults(this).into()
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: *mut core::ffi::c_void, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMRSOP_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Mode: Mode::<Identity, OFFSET>,
            Namespace: Namespace::<Identity, OFFSET>,
            SetLoggingComputer: SetLoggingComputer::<Identity, OFFSET>,
            LoggingComputer: LoggingComputer::<Identity, OFFSET>,
            SetLoggingUser: SetLoggingUser::<Identity, OFFSET>,
            LoggingUser: LoggingUser::<Identity, OFFSET>,
            SetLoggingFlags: SetLoggingFlags::<Identity, OFFSET>,
            LoggingFlags: LoggingFlags::<Identity, OFFSET>,
            SetPlanningFlags: SetPlanningFlags::<Identity, OFFSET>,
            PlanningFlags: PlanningFlags::<Identity, OFFSET>,
            SetPlanningDomainController: SetPlanningDomainController::<Identity, OFFSET>,
            PlanningDomainController: PlanningDomainController::<Identity, OFFSET>,
            SetPlanningSiteName: SetPlanningSiteName::<Identity, OFFSET>,
            PlanningSiteName: PlanningSiteName::<Identity, OFFSET>,
            SetPlanningUser: SetPlanningUser::<Identity, OFFSET>,
            PlanningUser: PlanningUser::<Identity, OFFSET>,
            SetPlanningUserSOM: SetPlanningUserSOM::<Identity, OFFSET>,
            PlanningUserSOM: PlanningUserSOM::<Identity, OFFSET>,
            SetPlanningUserWMIFilters: SetPlanningUserWMIFilters::<Identity, OFFSET>,
            PlanningUserWMIFilters: PlanningUserWMIFilters::<Identity, OFFSET>,
            SetPlanningUserSecurityGroups: SetPlanningUserSecurityGroups::<Identity, OFFSET>,
            PlanningUserSecurityGroups: PlanningUserSecurityGroups::<Identity, OFFSET>,
            SetPlanningComputer: SetPlanningComputer::<Identity, OFFSET>,
            PlanningComputer: PlanningComputer::<Identity, OFFSET>,
            SetPlanningComputerSOM: SetPlanningComputerSOM::<Identity, OFFSET>,
            PlanningComputerSOM: PlanningComputerSOM::<Identity, OFFSET>,
            SetPlanningComputerWMIFilters: SetPlanningComputerWMIFilters::<Identity, OFFSET>,
            PlanningComputerWMIFilters: PlanningComputerWMIFilters::<Identity, OFFSET>,
            SetPlanningComputerSecurityGroups: SetPlanningComputerSecurityGroups::<Identity, OFFSET>,
            PlanningComputerSecurityGroups: PlanningComputerSecurityGroups::<Identity, OFFSET>,
            LoggingEnumerateUsers: LoggingEnumerateUsers::<Identity, OFFSET>,
            CreateQueryResults: CreateQueryResults::<Identity, OFFSET>,
            ReleaseQueryResults: ReleaseQueryResults::<Identity, OFFSET>,
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMRSOP as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMRSOP {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMResult, IGPMResult_Vtbl, 0x86dff7e9_f76f_42ab_9570_cebc6be8a52d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMResult {
    pub unsafe fn Status(&self) -> windows_core::Result<IGPMStatusMsgCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Result(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OverallStatus(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OverallStatus)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Result: usize,
    pub OverallStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMResult_Impl: super::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<IGPMStatusMsgCollection>;
    fn Result(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn OverallStatus(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMResult_Vtbl {
    pub const fn new<Identity: IGPMResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Status<Identity: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmstatusmsgcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMResult_Impl::Status(this) {
                    Ok(ok__) => {
                        ppigpmstatusmsgcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Result<Identity: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarresult: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMResult_Impl::Result(this) {
                    Ok(ok__) => {
                        pvarresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OverallStatus<Identity: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMResult_Impl::OverallStatus(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            Result: Result::<Identity, OFFSET>,
            OverallStatus: OverallStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMResult {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMSOM, IGPMSOM_Vtbl, 0xc0a7f09e_05a1_4f0c_8158_9e5c33684f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMSOM {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMSOM, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOM {
    pub unsafe fn GPOInheritanceBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GPOInheritanceBlocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGPOInheritanceBlocked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGPOInheritanceBlocked)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateGPOLink<P1>(&self, llinkpos: i32, pgpo: P1) -> windows_core::Result<IGPMGPOLink>
    where
        P1: windows_core::Param<IGPMGPO>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGPOLink)(windows_core::Interface::as_raw(self), llinkpos, pgpo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<GPMSOMType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPOLinks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInheritedGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInheritedGPOLinks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMSecurityInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityInfo)(windows_core::Interface::as_raw(self), psecurityinfo.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GPOInheritanceBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetGPOInheritanceBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGPOLink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMSOMType) -> windows_core::HRESULT,
    pub GetGPOLinks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInheritedGPOLinks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSOM_Impl: super::Com::IDispatch_Impl {
    fn GPOInheritanceBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGPOInheritanceBlocked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateGPOLink(&self, llinkpos: i32, pgpo: windows_core::Ref<IGPMGPO>) -> windows_core::Result<IGPMGPOLink>;
    fn Type(&self) -> windows_core::Result<GPMSOMType>;
    fn GetGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: windows_core::Ref<IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMSOM_Vtbl {
    pub const fn new<Identity: IGPMSOM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GPOInheritanceBlocked<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::GPOInheritanceBlocked(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSOM_Impl::SetGPOInheritanceBlocked(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGPOLink<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llinkpos: i32, pgpo: *mut core::ffi::c_void, ppnewgpolink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::CreateGPOLink(this, core::mem::transmute_copy(&llinkpos), core::mem::transmute_copy(&pgpo)) {
                    Ok(ok__) => {
                        ppnewgpolink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::Type(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGPOLinks<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgpolinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::GetGPOLinks(this) {
                    Ok(ok__) => {
                        ppgpolinks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgpolinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::GetInheritedGPOLinks(this) {
                    Ok(ok__) => {
                        ppgpolinks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOM_Impl::GetSecurityInfo(this) {
                    Ok(ok__) => {
                        ppsecurityinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSOM_Impl::SetSecurityInfo(this, core::mem::transmute_copy(&psecurityinfo)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GPOInheritanceBlocked: GPOInheritanceBlocked::<Identity, OFFSET>,
            SetGPOInheritanceBlocked: SetGPOInheritanceBlocked::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            CreateGPOLink: CreateGPOLink::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            GetGPOLinks: GetGPOLinks::<Identity, OFFSET>,
            GetInheritedGPOLinks: GetInheritedGPOLinks::<Identity, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSOM as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMSOM {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMSOMCollection, IGPMSOMCollection_Vtbl, 0xadc1688e_00e4_4495_abba_bed200df0cab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMSOMCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMSOMCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOMCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOMCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSOMCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMSOMCollection_Vtbl {
    pub const fn new<Identity: IGPMSOMCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOMCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOMCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSOMCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmsom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSOMCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMSOMCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMSearchCriteria, IGPMSearchCriteria_Vtbl, 0xd6f11c42_829b_48d4_83f5_3615b67dfc22);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMSearchCriteria {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMSearchCriteria, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSearchCriteria {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), searchproperty, searchoperation, core::mem::transmute_copy(varvalue)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSearchCriteria_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, GPMSearchProperty, GPMSearchOperation, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSearchCriteria_Impl: super::Com::IDispatch_Impl {
    fn Add(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMSearchCriteria_Vtbl {
    pub const fn new<Identity: IGPMSearchCriteria_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IGPMSearchCriteria_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSearchCriteria_Impl::Add(this, core::mem::transmute_copy(&searchproperty), core::mem::transmute_copy(&searchoperation), core::mem::transmute(&varvalue)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMSearchCriteria {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMSecurityInfo, IGPMSecurityInfo_Vtbl, 0xb6c31ed4_1c93_4d3e_ae84_eb6d61161b60);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMSecurityInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMSecurityInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSecurityInfo {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pperm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMPermission>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pperm.param().abi()).ok() }
    }
    pub unsafe fn Remove<P0>(&self, pperm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMPermission>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), pperm.param().abi()).ok() }
    }
    pub unsafe fn RemoveTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveTrustee)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtrustee)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSecurityInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSecurityInfo_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
    fn Add(&self, pperm: windows_core::Ref<IGPMPermission>) -> windows_core::Result<()>;
    fn Remove(&self, pperm: windows_core::Ref<IGPMPermission>) -> windows_core::Result<()>;
    fn RemoveTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMSecurityInfo_Vtbl {
    pub const fn new<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSecurityInfo_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSecurityInfo_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSecurityInfo_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSecurityInfo_Impl::Add(this, core::mem::transmute_copy(&pperm)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSecurityInfo_Impl::Remove(this, core::mem::transmute_copy(&pperm)).into()
            }
        }
        unsafe extern "system" fn RemoveTrustee<Identity: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMSecurityInfo_Impl::RemoveTrustee(this, core::mem::transmute(&bstrtrustee)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveTrustee: RemoveTrustee::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSecurityInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMSecurityInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMSitesContainer, IGPMSitesContainer_Vtbl, 0x4725a899_2782_4d27_a6bb_d499246ffd72);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMSitesContainer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMSitesContainer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSitesContainer {
    pub unsafe fn DomainController(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainController)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Forest(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Forest)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSite(&self, bstrsitename: &windows_core::BSTR) -> windows_core::Result<IGPMSOM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSite)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsitename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SearchSites<P0>(&self, pigpmsearchcriteria: P0) -> windows_core::Result<IGPMSOMCollection>
    where
        P0: windows_core::Param<IGPMSearchCriteria>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchSites)(windows_core::Interface::as_raw(self), pigpmsearchcriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSitesContainer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Forest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchSites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSitesContainer_Impl: super::Com::IDispatch_Impl {
    fn DomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Forest(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSite(&self, bstrsitename: &windows_core::BSTR) -> windows_core::Result<IGPMSOM>;
    fn SearchSites(&self, pigpmsearchcriteria: windows_core::Ref<IGPMSearchCriteria>) -> windows_core::Result<IGPMSOMCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMSitesContainer_Vtbl {
    pub const fn new<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DomainController<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSitesContainer_Impl::DomainController(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Domain<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSitesContainer_Impl::Domain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Forest<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSitesContainer_Impl::Forest(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSite<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsitename: *mut core::ffi::c_void, ppsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSitesContainer_Impl::GetSite(this, core::mem::transmute(&bstrsitename)) {
                    Ok(ok__) => {
                        ppsom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchSites<Identity: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmsomcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMSitesContainer_Impl::SearchSites(this, core::mem::transmute_copy(&pigpmsearchcriteria)) {
                    Ok(ok__) => {
                        ppigpmsomcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DomainController: DomainController::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            Forest: Forest::<Identity, OFFSET>,
            GetSite: GetSite::<Identity, OFFSET>,
            SearchSites: SearchSites::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSitesContainer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMSitesContainer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStarterGPO, IGPMStarterGPO_Vtbl, 0xdfc3f61b_8880_4490_9337_d29c7ba8c2f0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStarterGPO {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStarterGPO, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPO {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn Author(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Product(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Product)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ModifiedTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<GPMStarterGPOType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputerVersion(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputerVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserVersion(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StarterGPOVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StarterGPOVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save(&self, bstrsavefile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsavefile), boverwrite, bsaveassystem, core::mem::transmute(bstrlanguage), core::mem::transmute(bstrauthor), core::mem::transmute(bstrproduct), core::mem::transmute(bstruniqueid), core::mem::transmute(bstrversion), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Backup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupdir), core::mem::transmute_copy(bstrcomment), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo(&self, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarnewdisplayname), core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReportToFile)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute_copy(bstrtargetfilepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMSecurityInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityInfo)(windows_core::Interface::as_raw(self), psecurityinfo.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Product: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMStarterGPOType) -> windows_core::HRESULT,
    pub ComputerVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UserVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub StarterGPOVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Save: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Backup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyTo: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub GenerateReportToFile: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPO_Impl: super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Product(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModifiedTime(&self) -> windows_core::Result<f64>;
    fn Type(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn ComputerVersion(&self) -> windows_core::Result<u16>;
    fn UserVersion(&self) -> windows_core::Result<u16>;
    fn StarterGPOVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self, bstrsavefile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn CopyTo(&self, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: windows_core::Ref<IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStarterGPO_Vtbl {
    pub const fn new<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStarterGPO_Impl::SetDisplayName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Description(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStarterGPO_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Author<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Author(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Product<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Product(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ID<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::ID(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifiedTime<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::ModifiedTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Type(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputerVersion<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::ComputerVersion(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserVersion<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::UserVersion(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StarterGPOVersion<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::StarterGPOVersion(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStarterGPO_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsavefile: *mut core::ffi::c_void, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Save(this, core::mem::transmute(&bstrsavefile), core::mem::transmute_copy(&boverwrite), core::mem::transmute_copy(&bsaveassystem), core::mem::transmute_copy(&bstrlanguage), core::mem::transmute_copy(&bstrauthor), core::mem::transmute_copy(&bstrproduct), core::mem::transmute_copy(&bstruniqueid), core::mem::transmute_copy(&bstrversion), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Backup<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: *mut core::ffi::c_void, bstrcomment: *mut core::ffi::c_void, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::Backup(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute(&bstrcomment), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::CopyTo(this, core::mem::transmute_copy(&pvarnewdisplayname), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: *mut core::ffi::c_void, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPO_Impl::GetSecurityInfo(this) {
                    Ok(ok__) => {
                        ppsecurityinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStarterGPO_Impl::SetSecurityInfo(this, core::mem::transmute_copy(&psecurityinfo)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            Product: Product::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            ID: ID::<Identity, OFFSET>,
            ModifiedTime: ModifiedTime::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            ComputerVersion: ComputerVersion::<Identity, OFFSET>,
            UserVersion: UserVersion::<Identity, OFFSET>,
            StarterGPOVersion: StarterGPOVersion::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Backup: Backup::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPO as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStarterGPO {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStarterGPOBackup, IGPMStarterGPOBackup_Vtbl, 0x51d98eda_a87e_43dd_b80a_0b66ef1938d6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStarterGPOBackup {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStarterGPOBackup, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackup {
    pub unsafe fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupDir)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Comment(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Comment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn StarterGPOID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StarterGPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<GPMStarterGPOType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReport)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute(pvargpmprogress), core::mem::transmute(pvargpmcancel), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateReportToFile)(windows_core::Interface::as_raw(self), gpmreporttype, core::mem::transmute_copy(bstrtargetfilepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StarterGPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPMStarterGPOType) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *const super::Variant::VARIANT, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub GenerateReportToFile: unsafe extern "system" fn(*mut core::ffi::c_void, GPMReportType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOBackup_Impl: super::Com::IDispatch_Impl {
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Comment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StarterGPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
    fn Type(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStarterGPOBackup_Vtbl {
    pub const fn new<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BackupDir<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::BackupDir(this) {
                    Ok(ok__) => {
                        pbstrbackupdir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Comment<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::Comment(this) {
                    Ok(ok__) => {
                        pbstrcomment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        pbstrdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Domain<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplatedomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::Domain(this) {
                    Ok(ok__) => {
                        pbstrtemplatedomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StarterGPOID<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplateid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::StarterGPOID(this) {
                    Ok(ok__) => {
                        pbstrtemplateid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ID<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::ID(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimestamp: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        ptimestamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::Type(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStarterGPOBackup_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: *mut core::ffi::c_void, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackup_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                    Ok(ok__) => {
                        ppigpmresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BackupDir: BackupDir::<Identity, OFFSET>,
            Comment: Comment::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            StarterGPOID: StarterGPOID::<Identity, OFFSET>,
            ID: ID::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GenerateReport: GenerateReport::<Identity, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStarterGPOBackup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStarterGPOBackupCollection, IGPMStarterGPOBackupCollection_Vtbl, 0xc998031d_add0_4bb5_8dea_298505d8423b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStarterGPOBackupCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStarterGPOBackupCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackupCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOBackupCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStarterGPOBackupCollection_Vtbl {
    pub const fn new<Identity: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackupCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackupCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtmplbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOBackupCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmtmplbackup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStarterGPOBackupCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStarterGPOCollection, IGPMStarterGPOCollection_Vtbl, 0x2e522729_2219_44ad_933a_64dfd650c423);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStarterGPOCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStarterGPOCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStarterGPOCollection_Vtbl {
    pub const fn new<Identity: IGPMStarterGPOCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStarterGPOCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppigpmtemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStarterGPOCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStatusMessage, IGPMStatusMessage_Vtbl, 0x8496c22f_f3de_4a1f_8f58_603caaa93d7b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStatusMessage {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStatusMessage, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMessage {
    pub unsafe fn ObjectPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ErrorCode(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ErrorCode)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExtensionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtensionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettingsName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SettingsName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OperationCode(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OperationCode)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ObjectPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtensionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettingsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OperationCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStatusMessage_Impl: super::Com::IDispatch_Impl {
    fn ObjectPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ErrorCode(&self) -> windows_core::Result<()>;
    fn ExtensionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettingsName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OperationCode(&self) -> windows_core::Result<()>;
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStatusMessage_Vtbl {
    pub const fn new<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ObjectPath<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMessage_Impl::ObjectPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStatusMessage_Impl::ErrorCode(this).into()
            }
        }
        unsafe extern "system" fn ExtensionName<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMessage_Impl::ExtensionName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettingsName<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMessage_Impl::SettingsName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OperationCode<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMStatusMessage_Impl::OperationCode(this).into()
            }
        }
        unsafe extern "system" fn Message<Identity: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMessage_Impl::Message(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ObjectPath: ObjectPath::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            ExtensionName: ExtensionName::<Identity, OFFSET>,
            SettingsName: SettingsName::<Identity, OFFSET>,
            OperationCode: OperationCode::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStatusMessage as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStatusMessage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMStatusMsgCollection, IGPMStatusMsgCollection_Vtbl, 0x9b6e1af0_1a92_40f3_a59d_f36ac1f728b7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMStatusMsgCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMStatusMsgCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMsgCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMsgCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStatusMsgCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMStatusMsgCollection_Vtbl {
    pub const fn new<Identity: IGPMStatusMsgCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMsgCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMsgCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMStatusMsgCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMStatusMsgCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMTrustee, IGPMTrustee_Vtbl, 0x3b466da8_c1a4_4b2a_999a_befcdd56cefb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMTrustee {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMTrustee, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMTrustee {
    pub unsafe fn TrusteeSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrusteeSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TrusteeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrusteeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TrusteeDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrusteeDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TrusteeDSPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrusteeDSPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TrusteeType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrusteeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMTrustee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TrusteeSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrusteeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrusteeDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrusteeDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrusteeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMTrustee_Impl: super::Com::IDispatch_Impl {
    fn TrusteeSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeDSPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeType(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMTrustee_Vtbl {
    pub const fn new<Identity: IGPMTrustee_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TrusteeSid<Identity: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMTrustee_Impl::TrusteeSid(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrusteeName<Identity: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMTrustee_Impl::TrusteeName(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrusteeDomain<Identity: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMTrustee_Impl::TrusteeDomain(this) {
                    Ok(ok__) => {
                        bstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrusteeDSPath<Identity: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMTrustee_Impl::TrusteeDSPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrusteeType<Identity: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMTrustee_Impl::TrusteeType(this) {
                    Ok(ok__) => {
                        lval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TrusteeSid: TrusteeSid::<Identity, OFFSET>,
            TrusteeName: TrusteeName::<Identity, OFFSET>,
            TrusteeDomain: TrusteeDomain::<Identity, OFFSET>,
            TrusteeDSPath: TrusteeDSPath::<Identity, OFFSET>,
            TrusteeType: TrusteeType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMTrustee as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMTrustee {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMWMIFilter, IGPMWMIFilter_Vtbl, 0xef2ff9b4_3c27_459a_b979_038305cec75d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMWMIFilter {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMWMIFilter, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilter {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetQueryList(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQueryList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGPMSecurityInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityInfo)(windows_core::Interface::as_raw(self), psecurityinfo.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetQueryList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetQueryList: usize,
    pub GetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMWMIFilter_Impl: super::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetQueryList(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: windows_core::Ref<IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMWMIFilter_Vtbl {
    pub const fn new<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilter_Impl::Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMWMIFilter_Impl::SetName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilter_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMWMIFilter_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilter_Impl::Description(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQueryList<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqrylist: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilter_Impl::GetQueryList(this) {
                    Ok(ok__) => {
                        pqrylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilter_Impl::GetSecurityInfo(this) {
                    Ok(ok__) => {
                        ppsecurityinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPMWMIFilter_Impl::SetSecurityInfo(this, core::mem::transmute_copy(&psecurityinfo)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            GetQueryList: GetQueryList::<Identity, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMWMIFilter as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMWMIFilter {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGPMWMIFilterCollection, IGPMWMIFilterCollection_Vtbl, 0x5782d582_1a36_4661_8a94_c3c32551945b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGPMWMIFilterCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGPMWMIFilterCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilterCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilterCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMWMIFilterCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGPMWMIFilterCollection_Vtbl {
    pub const fn new<Identity: IGPMWMIFilterCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilterCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilterCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPMWMIFilterCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGPMWMIFilterCollection {}
windows_core::imp::define_interface!(IGroupPolicyObject, IGroupPolicyObject_Vtbl, 0xea502723_a23d_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGroupPolicyObject, windows_core::IUnknown);
impl IGroupPolicyObject {
    pub unsafe fn New<P0, P1>(&self, pszdomainname: P0, pszdisplayname: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).New)(windows_core::Interface::as_raw(self), pszdomainname.param().abi(), pszdisplayname.param().abi(), dwflags).ok() }
    }
    pub unsafe fn OpenDSGPO<P0>(&self, pszpath: P0, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenDSGPO)(windows_core::Interface::as_raw(self), pszpath.param().abi(), dwflags).ok() }
    }
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenLocalMachineGPO)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn OpenRemoteMachineGPO<P0>(&self, pszcomputername: P0, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenRemoteMachineGPO)(windows_core::Interface::as_raw(self), pszcomputername.param().abi(), dwflags).ok() }
    }
    pub unsafe fn Save(&self, bmachine: bool, badd: bool, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), bmachine.into(), badd.into(), pguidextension as _, pguid as _).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), pszname.param().abi()).ok() }
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegistryKey)(windows_core::Interface::as_raw(self), dwsection, hkey as _).ok() }
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut GPO_OPTIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), dwoptions as _).ok() }
    }
    pub unsafe fn SetOptions(&self, dwoptions: GPO_OPTIONS, dwmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), dwoptions, dwmask).ok() }
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), gpotype as _).ok() }
    }
    pub unsafe fn GetMachineName(&self, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMachineName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertySheetPages)(windows_core::Interface::as_raw(self), hpages as _, upagecount as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub New: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, GPO_OPEN_FLAGS) -> windows_core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, GPO_OPEN_FLAGS) -> windows_core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, GPO_OPEN_FLAGS) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, GPO_SECTION, *mut super::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GPO_OPTIONS) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, GPO_OPTIONS, u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPropertySheetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPropertySheetPages: usize,
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObject_Impl: windows_core::IUnknownImpl {
    fn New(&self, pszdomainname: &windows_core::PCWSTR, pszdisplayname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenDSGPO(&self, pszpath: &windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn OpenLocalMachineGPO(&self, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn OpenRemoteMachineGPO(&self, pszcomputername: &windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn Save(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn SetDisplayName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut GPO_OPTIONS) -> windows_core::Result<()>;
    fn SetOptions(&self, dwoptions: GPO_OPTIONS, dwmask: u32) -> windows_core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()>;
    fn GetMachineName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn New<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdomainname: windows_core::PCWSTR, pszdisplayname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::New(this, core::mem::transmute(&pszdomainname), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenDSGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenDSGPO(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenLocalMachineGPO(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcomputername: windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenRemoteMachineGPO(this, core::mem::transmute(&pszcomputername), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::Save(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::SetDisplayName(this, core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPath(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDSPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&hkey)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut GPO_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetOptions(this, core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn SetOptions<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: GPO_OPTIONS, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::SetOptions(this, core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetType(this, core::mem::transmute_copy(&gpotype)).into()
            }
        }
        unsafe extern "system" fn GetMachineName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetMachineName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPropertySheetPages(this, core::mem::transmute_copy(&hpages), core::mem::transmute_copy(&upagecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            New: New::<Identity, OFFSET>,
            OpenDSGPO: OpenDSGPO::<Identity, OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Identity, OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetDSPath: GetDSPath::<Identity, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetMachineName: GetMachineName::<Identity, OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGroupPolicyObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl windows_core::RuntimeName for IGroupPolicyObject {}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: windows_core::PWSTR,
    pub ProgId: windows_core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSTALLSPEC_0 {
    pub Name: windows_core::PWSTR,
    pub GPOId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSTALLSPEC_1 {
    pub Clsid: windows_core::GUID,
    pub ClsCtx: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLSPECTYPE(pub i32);
windows_core::imp::define_interface!(IRSOPInformation, IRSOPInformation_Vtbl, 0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
windows_core::imp::interface_hierarchy!(IRSOPInformation, windows_core::IUnknown);
impl IRSOPInformation {
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNamespace)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pdwflags as _).ok() }
    }
    pub unsafe fn GetEventLogEntryText<P0, P1, P2>(&self, pszeventsource: P0, pszeventlogname: P1, pszeventtime: P2, dweventid: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventLogEntryText)(windows_core::Interface::as_raw(self), pszeventsource.param().abi(), pszeventlogname.param().abi(), pszeventtime.param().abi(), dweventid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IRSOPInformation_Impl: windows_core::IUnknownImpl {
    fn GetNamespace(&self, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn GetEventLogEntryText(&self, pszeventsource: &windows_core::PCWSTR, pszeventlogname: &windows_core::PCWSTR, pszeventtime: &windows_core::PCWSTR, dweventid: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: IRSOPInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamespace<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::GetNamespace(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszeventsource: windows_core::PCWSTR, pszeventlogname: windows_core::PCWSTR, pszeventtime: windows_core::PCWSTR, dweventid: u32, ppsztext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRSOPInformation_Impl::GetEventLogEntryText(this, core::mem::transmute(&pszeventsource), core::mem::transmute(&pszeventlogname), core::mem::transmute(&pszeventtime), core::mem::transmute_copy(&dweventid)) {
                    Ok(ok__) => {
                        ppsztext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespace: GetNamespace::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRSOPInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRSOPInformation {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: windows_core::PWSTR,
    pub pszPolicyName: windows_core::PWSTR,
    pub pszProductId: windows_core::PWSTR,
    pub dwState: u32,
}
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
pub const MACHINE_POLICY_PRESENT_TRIGGER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: windows_core::PWSTR,
    pub pszPublisher: windows_core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: windows_core::GUID,
    pub pszPolicyName: windows_core::PWSTR,
    pub ProductId: windows_core::GUID,
    pub Language: u16,
    pub pszOwner: windows_core::PWSTR,
    pub pszCompany: windows_core::PWSTR,
    pub pszComments: windows_core::PWSTR,
    pub pszContact: windows_core::PWSTR,
    pub pszSupportUrl: windows_core::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: windows_core::BOOL,
}
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: windows_core::GUID = windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: windows_core::GUID = windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut windows_core::BOOL, pwszsite: windows_core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: windows_core::Ref<super::Wmi::IWbemServices>, prsopstatus: *mut windows_core::HRESULT) -> u32>;
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "system" fn(bverbose: windows_core::BOOL, lpmessage: windows_core::PCWSTR) -> u32>;
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_core::PWSTR,
    pub szEventSource: windows_core::PWSTR,
    pub szEventLogName: windows_core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
pub const PT_MANDATORY: u32 = 4u32;
pub const PT_ROAMING: u32 = 2u32;
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
pub const PT_TEMPORARY: u32 = 1u32;
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
pub const REGISTRY_EXTENSION_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x35378eac_683f_11d2_a89a_00c04fbbcfa2);
pub const RP_FORCE: u32 = 1u32;
pub const RP_SYNC: u32 = 2u32;
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
pub const RSOP_NO_USER: u32 = 131072u32;
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
#[derive(Clone, Debug, PartialEq)]
pub struct RSOP_TARGET {
    pub pwszAccountName: windows_core::PWSTR,
    pub pwszNewSOM: windows_core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: core::mem::ManuallyDrop<Option<super::Wmi::IWbemServices>>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETTINGSTATUS(pub i32);
pub const USER_POLICY_PRESENT_TRIGGER_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
pub const repHTML: GPMReportType = GPMReportType(1i32);
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
pub const repXML: GPMReportType = GPMReportType(0i32);
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
pub const somOU: GPMSOMType = GPMSOMType(2i32);
pub const somSite: GPMSOMType = GPMSOMType(0i32);
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
