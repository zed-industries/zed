#[inline]
pub unsafe fn NetDfsAdd<P0, P1, P2, P3>(dfsentrypath: P0, servername: P1, sharename: P2, comment: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsAdd(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsAdd(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), comment.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsAddFtRoot<P0, P1, P2, P3>(servername: P0, rootshare: P1, ftdfsname: P2, comment: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsAddFtRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsAddFtRoot(servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), comment.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsAddRootTarget<P0, P1, P2>(pdfspath: P0, ptargetpath: P1, majorversion: u32, pcomment: P2, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsAddRootTarget(pdfspath : windows_core::PCWSTR, ptargetpath : windows_core::PCWSTR, majorversion : u32, pcomment : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsAddRootTarget(pdfspath.param().abi(), ptargetpath.param().abi(), majorversion, pcomment.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsAddStdRoot<P0, P1, P2>(servername: P0, rootshare: P1, comment: P2, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsAddStdRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, comment : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsAddStdRoot(servername.param().abi(), rootshare.param().abi(), comment.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsEnum<P0>(dfsname: P0, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsEnum(dfsname : windows_core::PCWSTR, level : u32, prefmaxlen : u32, buffer : *mut *mut u8, entriesread : *mut u32, resumehandle : *mut u32) -> u32);
    NetDfsEnum(dfsname.param().abi(), level, prefmaxlen, buffer, entriesread, resumehandle)
}
#[inline]
pub unsafe fn NetDfsGetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetClientInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
    NetDfsGetClientInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetFtContainerSecurity<P0>(domainname: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetFtContainerSecurity(domainname : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    NetDfsGetFtContainerSecurity(domainname.param().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[inline]
pub unsafe fn NetDfsGetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
    NetDfsGetInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetSecurity<P0>(dfsentrypath: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetSecurity(dfsentrypath : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    NetDfsGetSecurity(dfsentrypath.param().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetStdContainerSecurity<P0>(machinename: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetStdContainerSecurity(machinename : windows_core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
    NetDfsGetStdContainerSecurity(machinename.param().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[inline]
pub unsafe fn NetDfsGetSupportedNamespaceVersion<P0>(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: P0, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsGetSupportedNamespaceVersion(origin : DFS_NAMESPACE_VERSION_ORIGIN, pname : windows_core::PCWSTR, ppversioninfo : *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32);
    NetDfsGetSupportedNamespaceVersion(origin, pname.param().abi(), ppversioninfo)
}
#[inline]
pub unsafe fn NetDfsMove<P0, P1>(olddfsentrypath: P0, newdfsentrypath: P1, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsMove(olddfsentrypath : windows_core::PCWSTR, newdfsentrypath : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsMove(olddfsentrypath.param().abi(), newdfsentrypath.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsRemove<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsRemove(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR) -> u32);
    NetDfsRemove(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi())
}
#[inline]
pub unsafe fn NetDfsRemoveFtRoot<P0, P1, P2>(servername: P0, rootshare: P1, ftdfsname: P2, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsRemoveFtRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsRemoveFtRoot(servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsRemoveFtRootForced<P0, P1, P2, P3>(domainname: P0, servername: P1, rootshare: P2, ftdfsname: P3, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsRemoveFtRootForced(domainname : windows_core::PCWSTR, servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, ftdfsname : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsRemoveFtRootForced(domainname.param().abi(), servername.param().abi(), rootshare.param().abi(), ftdfsname.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsRemoveRootTarget<P0, P1>(pdfspath: P0, ptargetpath: P1, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsRemoveRootTarget(pdfspath : windows_core::PCWSTR, ptargetpath : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsRemoveRootTarget(pdfspath.param().abi(), ptargetpath.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsRemoveStdRoot<P0, P1>(servername: P0, rootshare: P1, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsRemoveStdRoot(servername : windows_core::PCWSTR, rootshare : windows_core::PCWSTR, flags : u32) -> u32);
    NetDfsRemoveStdRoot(servername.param().abi(), rootshare.param().abi(), flags)
}
#[inline]
pub unsafe fn NetDfsSetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsSetClientInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *const u8) -> u32);
    NetDfsSetClientInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetFtContainerSecurity<P0, P1>(domainname: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsSetFtContainerSecurity(domainname : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    NetDfsSetFtContainerSecurity(domainname.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn NetDfsSetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsSetInfo(dfsentrypath : windows_core::PCWSTR, servername : windows_core::PCWSTR, sharename : windows_core::PCWSTR, level : u32, buffer : *const u8) -> u32);
    NetDfsSetInfo(dfsentrypath.param().abi(), servername.param().abi(), sharename.param().abi(), level, buffer)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetSecurity<P0, P1>(dfsentrypath: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsSetSecurity(dfsentrypath : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    NetDfsSetSecurity(dfsentrypath.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetStdContainerSecurity<P0, P1>(machinename: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetDfsSetStdContainerSecurity(machinename : windows_core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
    NetDfsSetStdContainerSecurity(machinename.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
pub const DFS_ADD_VOLUME: u32 = 1u32;
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
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
pub const DFS_SITE_PRIMARY: u32 = 1u32;
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
pub const DFS_STORAGE_STATES: u32 = 15u32;
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DFS_NAMESPACE_VERSION_ORIGIN(pub i32);
impl windows_core::TypeKind for DFS_NAMESPACE_VERSION_ORIGIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DFS_NAMESPACE_VERSION_ORIGIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DFS_NAMESPACE_VERSION_ORIGIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DFS_TARGET_PRIORITY_CLASS(pub i32);
impl windows_core::TypeKind for DFS_TARGET_PRIORITY_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DFS_TARGET_PRIORITY_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DFS_TARGET_PRIORITY_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_GET_PKT_ENTRY_STATE_ARG {
    pub DfsEntryPathLen: u16,
    pub ServerNameLen: u16,
    pub ShareNameLen: u16,
    pub Level: u32,
    pub Buffer: [u16; 1],
}
impl windows_core::TypeKind for DFS_GET_PKT_ENTRY_STATE_ARG {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_1 {
    pub EntryPath: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_100 {
    pub Comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_INFO_100 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_101 {
    pub State: u32,
}
impl windows_core::TypeKind for DFS_INFO_101 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
impl windows_core::TypeKind for DFS_INFO_102 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl windows_core::TypeKind for DFS_INFO_103 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_103 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl windows_core::TypeKind for DFS_INFO_104 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_104 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_105 {
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl windows_core::TypeKind for DFS_INFO_105 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_105 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl windows_core::TypeKind for DFS_INFO_106 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_106 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_107 {
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for DFS_INFO_107 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_107 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for DFS_INFO_150 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_150 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DFS_INFO_1_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DFS_INFO_1_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_2 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
impl windows_core::TypeKind for DFS_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_200 {
    pub FtDfsName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_INFO_200 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_200 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DFS_INFO_2_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DFS_INFO_2_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_3 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl windows_core::TypeKind for DFS_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_INFO_300 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_300 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DFS_INFO_3_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DFS_INFO_3_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_4 {
    pub EntryPath: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl windows_core::TypeKind for DFS_INFO_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DFS_INFO_4_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DFS_INFO_4_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DFS_INFO_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
impl windows_core::TypeKind for DFS_INFO_50 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_50 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DFS_INFO_6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_INFO_7 {
    pub GenerationGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DFS_INFO_7 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_INFO_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for DFS_INFO_8 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for DFS_INFO_9 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
impl windows_core::TypeKind for DFS_SITELIST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_SITELIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_SITENAME_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_SITENAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: windows_core::PWSTR,
    pub ShareName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DFS_STORAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_STORAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for DFS_STORAGE_INFO_0_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for DFS_STORAGE_INFO_0_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: windows_core::PWSTR,
    pub ShareName: windows_core::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl windows_core::TypeKind for DFS_STORAGE_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_STORAGE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
impl windows_core::TypeKind for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
impl windows_core::TypeKind for DFS_TARGET_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DFS_TARGET_PRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
