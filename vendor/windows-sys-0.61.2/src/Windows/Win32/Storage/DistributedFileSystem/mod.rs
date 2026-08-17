windows_link::link!("netapi32.dll" "system" fn NetDfsAdd(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR, comment : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsAddFtRoot(servername : windows_sys::core::PCWSTR, rootshare : windows_sys::core::PCWSTR, ftdfsname : windows_sys::core::PCWSTR, comment : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsAddRootTarget(pdfspath : windows_sys::core::PCWSTR, ptargetpath : windows_sys::core::PCWSTR, majorversion : u32, pcomment : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsAddStdRoot(servername : windows_sys::core::PCWSTR, rootshare : windows_sys::core::PCWSTR, comment : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsEnum(dfsname : windows_sys::core::PCWSTR, level : u32, prefmaxlen : u32, buffer : *mut *mut u8, entriesread : *mut u32, resumehandle : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsGetClientInfo(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsGetFtContainerSecurity(domainname : windows_sys::core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsGetInfo(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR, level : u32, buffer : *mut *mut u8) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsGetSecurity(dfsentrypath : windows_sys::core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsGetStdContainerSecurity(machinename : windows_sys::core::PCWSTR, securityinformation : u32, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsGetSupportedNamespaceVersion(origin : DFS_NAMESPACE_VERSION_ORIGIN, pname : windows_sys::core::PCWSTR, ppversioninfo : *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsMove(olddfsentrypath : windows_sys::core::PCWSTR, newdfsentrypath : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsRemove(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsRemoveFtRoot(servername : windows_sys::core::PCWSTR, rootshare : windows_sys::core::PCWSTR, ftdfsname : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsRemoveFtRootForced(domainname : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, rootshare : windows_sys::core::PCWSTR, ftdfsname : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsRemoveRootTarget(pdfspath : windows_sys::core::PCWSTR, ptargetpath : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsRemoveStdRoot(servername : windows_sys::core::PCWSTR, rootshare : windows_sys::core::PCWSTR, flags : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsSetClientInfo(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR, level : u32, buffer : *const u8) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsSetFtContainerSecurity(domainname : windows_sys::core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetDfsSetInfo(dfsentrypath : windows_sys::core::PCWSTR, servername : windows_sys::core::PCWSTR, sharename : windows_sys::core::PCWSTR, level : u32, buffer : *const u8) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsSetSecurity(dfsentrypath : windows_sys::core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("netapi32.dll" "system" fn NetDfsSetStdContainerSecurity(machinename : windows_sys::core::PCWSTR, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> u32);
pub const DFS_ADD_VOLUME: u32 = 1u32;
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct DFS_INFO_1 {
    pub EntryPath: windows_sys::core::PWSTR,
}
impl Default for DFS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_100 {
    pub Comment: windows_sys::core::PWSTR,
}
impl Default for DFS_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_101 {
    pub State: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_105 {
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl Default for DFS_INFO_105 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct DFS_INFO_107 {
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_107 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_150 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_2 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
impl Default for DFS_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_200 {
    pub FtDfsName: windows_sys::core::PWSTR,
}
impl Default for DFS_INFO_200 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_3 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: windows_sys::core::PWSTR,
}
impl Default for DFS_INFO_300 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_4 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
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
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_5 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
impl Default for DFS_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_INFO_6 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
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
#[derive(Clone, Copy, Default)]
pub struct DFS_INFO_7 {
    pub GenerationGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct DFS_INFO_8 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Security")]
impl Default for DFS_INFO_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct DFS_INFO_9 {
    pub EntryPath: windows_sys::core::PWSTR,
    pub Comment: windows_sys::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: windows_sys::core::GUID,
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
pub type DFS_NAMESPACE_VERSION_ORIGIN = i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = 0i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = 2i32;
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = 1i32;
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: windows_sys::core::PWSTR,
}
impl Default for DFS_SITENAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DFS_SITE_PRIMARY: u32 = 1u32;
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: windows_sys::core::PWSTR,
    pub ShareName: windows_sys::core::PWSTR,
}
impl Default for DFS_STORAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: windows_sys::core::PWSTR,
    pub ShareName: windows_sys::core::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl Default for DFS_STORAGE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DFS_STORAGE_STATES: u32 = 15u32;
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
pub type DFS_TARGET_PRIORITY_CLASS = i32;
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
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 1i32;
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 4i32;
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = -1i32;
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 2i32;
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 3i32;
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = 0i32;
pub const FSCTL_DFS_BASE: u32 = 6u32;
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
