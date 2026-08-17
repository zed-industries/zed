#[inline]
pub unsafe fn SaferCloseLevel<P0>(hlevelhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::SAFER_LEVEL_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferCloseLevel(hlevelhandle : super:: SAFER_LEVEL_HANDLE) -> super::super::Foundation:: BOOL);
    SaferCloseLevel(hlevelhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn SaferComputeTokenFromLevel<P0, P1>(levelhandle: P0, inaccesstoken: P1, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::SAFER_LEVEL_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferComputeTokenFromLevel(levelhandle : super:: SAFER_LEVEL_HANDLE, inaccesstoken : super::super::Foundation:: HANDLE, outaccesstoken : *mut super::super::Foundation:: HANDLE, dwflags : SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferComputeTokenFromLevel(levelhandle.param().abi(), inaccesstoken.param().abi(), outaccesstoken, dwflags, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn SaferCreateLevel(dwscopeid : u32, dwlevelid : u32, openflags : u32, plevelhandle : *mut super:: SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferCreateLevel(dwscopeid, dwlevelid, openflags, plevelhandle, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn SaferGetLevelInformation<P0>(levelhandle: P0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: Option<*mut core::ffi::c_void>, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::SAFER_LEVEL_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferGetLevelInformation(levelhandle : super:: SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *mut core::ffi::c_void, dwinbuffersize : u32, lpdwoutbuffersize : *mut u32) -> super::super::Foundation:: BOOL);
    SaferGetLevelInformation(levelhandle.param().abi(), dwinfotype, core::mem::transmute(lpquerybuffer.unwrap_or(std::ptr::null_mut())), dwinbuffersize, lpdwoutbuffersize).ok()
}
#[inline]
pub unsafe fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn SaferGetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *mut core::ffi::c_void, infobufferretsize : *mut u32, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferGetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, infobuffer, infobufferretsize, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn SaferIdentifyLevel(pcodeproperties: Option<&[SAFER_CODE_PROPERTIES_V2]>, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn SaferIdentifyLevel(dwnumproperties : u32, pcodeproperties : *const SAFER_CODE_PROPERTIES_V2, plevelhandle : *mut super:: SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferIdentifyLevel(pcodeproperties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcodeproperties.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), plevelhandle, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn SaferRecordEventLogEntry<P0, P1>(hlevel: P0, sztargetpath: P1, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::SAFER_LEVEL_HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferRecordEventLogEntry(hlevel : super:: SAFER_LEVEL_HANDLE, sztargetpath : windows_core::PCWSTR, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferRecordEventLogEntry(hlevel.param().abi(), sztargetpath.param().abi(), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn SaferSetLevelInformation<P0>(levelhandle: P0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const core::ffi::c_void, dwinbuffersize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::SAFER_LEVEL_HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferSetLevelInformation(levelhandle : super:: SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *const core::ffi::c_void, dwinbuffersize : u32) -> super::super::Foundation:: BOOL);
    SaferSetLevelInformation(levelhandle.param().abi(), dwinfotype, lpquerybuffer, dwinbuffersize).ok()
}
#[inline]
pub unsafe fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const core::ffi::c_void, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn SaferSetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SaferSetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, infobuffer, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn SaferiIsExecutableFileType<P0, P1>(szfullpathname: P0, bfromshellexecute: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn SaferiIsExecutableFileType(szfullpathname : windows_core::PCWSTR, bfromshellexecute : super::super::Foundation:: BOOLEAN) -> super::super::Foundation:: BOOL);
    SaferiIsExecutableFileType(szfullpathname.param().abi(), bfromshellexecute.param().abi())
}
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864u32;
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080u32;
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648u32;
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096u32;
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536u32;
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072u32;
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4u32;
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1u32;
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255u32;
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2u32;
pub const SAFER_SCOPEID_MACHINE: u32 = 1u32;
pub const SAFER_SCOPEID_USER: u32 = 2u32;
pub const SAFER_TOKEN_COMPARE_ONLY: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(2u32);
pub const SAFER_TOKEN_MAKE_INERT: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(4u32);
pub const SAFER_TOKEN_NULL_IF_EQUAL: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(1u32);
pub const SAFER_TOKEN_WANT_FLAGS: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(8u32);
pub const SRP_POLICY_APPX: windows_core::PCWSTR = windows_core::w!("APPX");
pub const SRP_POLICY_DLL: windows_core::PCWSTR = windows_core::w!("DLL");
pub const SRP_POLICY_EXE: windows_core::PCWSTR = windows_core::w!("EXE");
pub const SRP_POLICY_MANAGEDINSTALLER: windows_core::PCWSTR = windows_core::w!("MANAGEDINSTALLER");
pub const SRP_POLICY_MSI: windows_core::PCWSTR = windows_core::w!("MSI");
pub const SRP_POLICY_NOV2: windows_core::PCWSTR = windows_core::w!("IGNORESRPV2");
pub const SRP_POLICY_SCRIPT: windows_core::PCWSTR = windows_core::w!("SCRIPT");
pub const SRP_POLICY_SHELL: windows_core::PCWSTR = windows_core::w!("SHELL");
pub const SRP_POLICY_WLDPCONFIGCI: windows_core::PCWSTR = windows_core::w!("WLDPCONFIGCI");
pub const SRP_POLICY_WLDPMSI: windows_core::PCWSTR = windows_core::w!("WLDPMSI");
pub const SRP_POLICY_WLDPSCRIPT: windows_core::PCWSTR = windows_core::w!("WLDPSCRIPT");
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(0i32);
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(4i32);
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(2i32);
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(1i32);
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(3i32);
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(14i32);
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(5i32);
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(10i32);
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(9i32);
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(4i32);
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(7i32);
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(6i32);
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(16i32);
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(3i32);
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(8i32);
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(1i32);
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(13i32);
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(12i32);
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(2i32);
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(11i32);
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(15i32);
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(7i32);
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(3i32);
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(6i32);
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(2i32);
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(4i32);
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(1i32);
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(5i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(pub u32);
impl windows_core::TypeKind for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS").field(&self.0).finish()
    }
}
impl SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SAFER_IDENTIFICATION_TYPES(pub i32);
impl windows_core::TypeKind for SAFER_IDENTIFICATION_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SAFER_IDENTIFICATION_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SAFER_IDENTIFICATION_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SAFER_OBJECT_INFO_CLASS(pub i32);
impl windows_core::TypeKind for SAFER_OBJECT_INFO_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SAFER_OBJECT_INFO_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SAFER_OBJECT_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SAFER_POLICY_INFO_CLASS(pub i32);
impl windows_core::TypeKind for SAFER_POLICY_INFO_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SAFER_POLICY_INFO_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SAFER_POLICY_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: super::Cryptography::ALG_ID,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SAFER_CODE_PROPERTIES_V1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: super::Cryptography::ALG_ID,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: windows_core::PCWSTR,
    pub PackagePublisher: windows_core::PCWSTR,
    pub PackageName: windows_core::PCWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SAFER_CODE_PROPERTIES_V2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_HASH_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub FriendlyName: [u16; 256],
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: super::Cryptography::ALG_ID,
    pub ImageSize: i64,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SAFER_HASH_IDENTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: super::Cryptography::ALG_ID,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SAFER_HASH_IDENTIFICATION2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: windows_core::GUID,
    pub lastModified: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for SAFER_IDENTIFICATION_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFER_IDENTIFICATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: windows_core::PWSTR,
    pub dwSaferFlags: u32,
}
impl windows_core::TypeKind for SAFER_PATHNAME_IDENTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
impl windows_core::TypeKind for SAFER_URLZONE_IDENTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFER_URLZONE_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
