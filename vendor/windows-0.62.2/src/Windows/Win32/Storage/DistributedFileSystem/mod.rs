#[inline]
pub unsafe fn NetDfsAdd<P0, P1, P2, P3>(dfsentrypath: P0, servername: P1, sharename: P2, comment: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsAdd(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsAdd(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), comment.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsAddFtRoot<P0, P1, P2, P3>(servername: P0, rootshare: P1, ftdfsname: P2, comment: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsAddFtRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsAddFtRoot(servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), comment.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsAddRootTarget<P0, P1, P3>(pdfspath: P0, ptargetpath: P1, majorversion: u32, pcomment: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsAddRootTarget(pdfspath : windows_core::PCWSTR, ptargetpath : windows_core::PCWSTR, majorversion : u32, pcomment : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsAddRootTarget(pdfspath.param().abi(), ptargetpath.param().abi(), majorversion, pcomment.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsAddStdRoot<P0, P1, P2>(servername: P0, rootshare: P1, comment: P2, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsAddStdRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsAddStdRoot(servername.param().abi(), rootshare.param().abi(), comment.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsEnum<P0>(dfsname: P0, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsEnum(dfsname : windows_core::PCWSTR, level : u32, prefmaxlen : u32, buffer : *mut *mut u8, entriesread : *mut u32, resumehandle : *mut u32) -> u32);
    unsafe { NetDfsEnum(dfsname.param().abi(), level, prefmaxlen, buffer as _, entriesread as _, resumehandle as _) }
}
#[inline]
pub unsafe fn NetDfsGetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetClientInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
    unsafe { NetDfsGetClientInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetFtContainerSecurity<P0>(domainname: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetFtContainerSecurity(domainname : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    unsafe { NetDfsGetFtContainerSecurity(domainname.param().abi(), securityinformation, ppsecuritydescriptor as _, lpcbsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn NetDfsGetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
    unsafe { NetDfsGetInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetSecurity<P0>(dfsentrypath: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetSecurity(dfsentrypath : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    unsafe { NetDfsGetSecurity(dfsentrypath.param().abi(), securityinformation, ppsecuritydescriptor as _, lpcbsecuritydescriptor as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetStdContainerSecurity<P0>(machinename: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetStdContainerSecurity(machinename : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    unsafe { NetDfsGetStdContainerSecurity(machinename.param().abi(), securityinformation, ppsecuritydescriptor as _, lpcbsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn NetDfsGetSupportedNamespaceVersion<P1>(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: P1, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsGetSupportedNamespaceVersion(origin : DFS_NAMESPACE_VERSION_ORIGIN, pname : windows_core::PCWSTR, ppversioninfo : *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32);
    unsafe { NetDfsGetSupportedNamespaceVersion(origin, pname.param().abi(), ppversioninfo as _) }
}
#[inline]
pub unsafe fn NetDfsMove<P0, P1>(olddfsentrypath: P0, newdfsentrypath: P1, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsMove(olddfsentrypath : windows_core::PCWSTR, newdfsentrypath : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsMove(olddfsentrypath.param().abi(), newdfsentrypath.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsRemove<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsRemove(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR) -> u32);
    unsafe { NetDfsRemove(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi()) }
}
#[inline]
pub unsafe fn NetDfsRemoveFtRoot<P0, P1, P2>(servername: P0, rootshare: P1, ftdfsname: P2, flags: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsRemoveFtRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsRemoveFtRoot(servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetDfsRemoveFtRootForced<P0, P1, P2, P3>(domainname: P0, servername: P1, rootshare: P2, ftdfsname: P3, flags: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsRemoveFtRootForced(domainname : windows_core::PCWSTR, servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsRemoveFtRootForced(domainname.param().abi(), servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetDfsRemoveRootTarget<P0, P1>(pdfspath: P0, ptargetpath: P1, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsRemoveRootTarget(pdfspath : windows_core::PCWSTR, ptargetpath : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsRemoveRootTarget(pdfspath.param().abi(), ptargetpath.param().abi(), flags) }
}
#[inline]
pub unsafe fn NetDfsRemoveStdRoot<P0, P1>(servername: P0, rootshare: P1, flags: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsRemoveStdRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { NetDfsRemoveStdRoot(servername.param().abi(), rootshare.param().abi(), flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetDfsSetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsSetClientInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *const u8) -> u32);
    unsafe { NetDfsSetClientInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetFtContainerSecurity<P0>(domainname: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsSetFtContainerSecurity(domainname : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    unsafe { NetDfsSetFtContainerSecurity(domainname.param().abi(), securityinformation, psecuritydescriptor) }
}
#[inline]
pub unsafe fn NetDfsSetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsSetInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *const u8) -> u32);
    unsafe { NetDfsSetInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetSecurity<P0>(dfsentrypath: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsSetSecurity(dfsentrypath : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    unsafe { NetDfsSetSecurity(dfsentrypath.param().abi(), securityinformation, psecuritydescriptor) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetStdContainerSecurity<P0>(machinename: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetDfsSetStdContainerSecurity(machinename : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    unsafe { NetDfsSetStdContainerSecurity(machinename.param().abi(), securityinformation, psecuritydescriptor) }
}
pub const DFS_ADD_VOLUME: u32 = 1u32;
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_GET_PKT_ENTRY_STATE_ARG {
    pub DfsEntryPathLen: u16,
    pub ServerNameLen: u16,
    pub ShareNameLen: u16,
    pub Level: u32,
    pub Buffer: [u16; 1],
}
impl Default for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_1 {
    pub EntryPath: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_100 {
    pub Comment: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_101 {
    pub State: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_105 {
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_107 {
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_2 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_200 {
    pub FtDfsName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_INFO_3 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl Default for DFS_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_INFO_4 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl Default for DFS_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_5 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_INFO_6 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
impl Default for DFS_INFO_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_7 {
    pub GenerationGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_INFO_8 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_INFO_9 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DFS_NAMESPACE_VERSION_ORIGIN(pub i32);
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(0i32);
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(2i32);
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(1i32);
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
impl Default for DFS_SITELIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: windows_core::PWSTR,
}
pub const DFS_SITE_PRIMARY: u32 = 1u32;
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: windows_core::PWSTR,
    pub ShareName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: windows_core::PWSTR,
    pub ShareName: windows_core::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
pub const DFS_STORAGE_STATES: u32 = 15u32;
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DFS_TARGET_PRIORITY_CLASS(pub i32);
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
pub const DFS_VOLUME_STATES: u32 = 15u32;
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(1i32);
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(4i32);
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(-1i32);
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(2i32);
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(3i32);
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(0i32);
pub const FSCTL_DFS_BASE: u32 = 6u32;
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
