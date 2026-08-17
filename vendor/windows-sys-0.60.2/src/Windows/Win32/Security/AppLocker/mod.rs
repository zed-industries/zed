windows_targets::link!("advapi32.dll" "system" fn SaferCloseLevel(hlevelhandle : super:: SAFER_LEVEL_HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferComputeTokenFromLevel(levelhandle : super:: SAFER_LEVEL_HANDLE, inaccesstoken : super::super::Foundation:: HANDLE, outaccesstoken : *mut super::super::Foundation:: HANDLE, dwflags : SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferCreateLevel(dwscopeid : u32, dwlevelid : u32, openflags : u32, plevelhandle : *mut super:: SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferGetLevelInformation(levelhandle : super:: SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *mut core::ffi::c_void, dwinbuffersize : u32, lpdwoutbuffersize : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferGetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *mut core::ffi::c_void, infobufferretsize : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography")]
windows_targets::link!("advapi32.dll" "system" fn SaferIdentifyLevel(dwnumproperties : u32, pcodeproperties : *const SAFER_CODE_PROPERTIES_V2, plevelhandle : *mut super:: SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferRecordEventLogEntry(hlevel : super:: SAFER_LEVEL_HANDLE, sztargetpath : windows_sys::core::PCWSTR, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferSetLevelInformation(levelhandle : super:: SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *const core::ffi::c_void, dwinbuffersize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferSetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn SaferiIsExecutableFileType(szfullpathname : windows_sys::core::PCWSTR, bfromshellexecute : bool) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_sys::core::PCWSTR,
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
impl Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_sys::core::PCWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: super::Cryptography::ALG_ID,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: windows_sys::core::PCWSTR,
    pub PackagePublisher: windows_sys::core::PCWSTR,
    pub PackageName: windows_sys::core::PCWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = u32;
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
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
impl Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy)]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: super::Cryptography::ALG_ID,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: windows_sys::core::GUID,
    pub lastModified: super::super::Foundation::FILETIME,
}
pub type SAFER_IDENTIFICATION_TYPES = i32;
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
pub type SAFER_OBJECT_INFO_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: windows_sys::core::PWSTR,
    pub dwSaferFlags: u32,
}
impl Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
pub type SAFER_POLICY_INFO_CLASS = i32;
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
pub const SAFER_TOKEN_COMPARE_ONLY: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = 2u32;
pub const SAFER_TOKEN_MAKE_INERT: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = 4u32;
pub const SAFER_TOKEN_NULL_IF_EQUAL: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = 1u32;
pub const SAFER_TOKEN_WANT_FLAGS: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
pub const SRP_POLICY_APPX: windows_sys::core::PCWSTR = windows_sys::core::w!("APPX");
pub const SRP_POLICY_DLL: windows_sys::core::PCWSTR = windows_sys::core::w!("DLL");
pub const SRP_POLICY_EXE: windows_sys::core::PCWSTR = windows_sys::core::w!("EXE");
pub const SRP_POLICY_MANAGEDINSTALLER: windows_sys::core::PCWSTR = windows_sys::core::w!("MANAGEDINSTALLER");
pub const SRP_POLICY_MSI: windows_sys::core::PCWSTR = windows_sys::core::w!("MSI");
pub const SRP_POLICY_NOV2: windows_sys::core::PCWSTR = windows_sys::core::w!("IGNORESRPV2");
pub const SRP_POLICY_SCRIPT: windows_sys::core::PCWSTR = windows_sys::core::w!("SCRIPT");
pub const SRP_POLICY_SHELL: windows_sys::core::PCWSTR = windows_sys::core::w!("SHELL");
pub const SRP_POLICY_WLDPCONFIGCI: windows_sys::core::PCWSTR = windows_sys::core::w!("WLDPCONFIGCI");
pub const SRP_POLICY_WLDPMSI: windows_sys::core::PCWSTR = windows_sys::core::w!("WLDPMSI");
pub const SRP_POLICY_WLDPSCRIPT: windows_sys::core::PCWSTR = windows_sys::core::w!("WLDPSCRIPT");
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = 0i32;
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES = 4i32;
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = 2i32;
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = 1i32;
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = 3i32;
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS = 14i32;
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = 5i32;
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = 10i32;
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = 9i32;
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = 4i32;
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = 7i32;
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = 6i32;
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = 16i32;
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = 3i32;
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = 8i32;
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = 1i32;
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = 13i32;
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS = 12i32;
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = 2i32;
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = 11i32;
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = 15i32;
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = 7i32;
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = 3i32;
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = 6i32;
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS = 2i32;
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = 4i32;
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = 1i32;
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = 5i32;
