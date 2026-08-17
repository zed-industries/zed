// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{SIZE_T, ULONG64};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, LPBYTE, LPDWORD, LPVOID, PDWORD};
use shared::windef::HWND;
use um::wincrypt::ALG_ID;
use um::winnt::{BOOLEAN, HANDLE, LARGE_INTEGER, LPCWSTR, PHANDLE, PVOID, PWCHAR, WCHAR};
DECLARE_HANDLE!{SAFER_LEVEL_HANDLE, __SAFER_LEVEL_HANDLE}
pub const SAFER_SCOPEID_MACHINE: DWORD = 1;
pub const SAFER_SCOPEID_USER: DWORD = 2;
pub const SAFER_LEVELID_DISALLOWED: DWORD = 0x00000;
pub const SAFER_LEVELID_UNTRUSTED: DWORD = 0x01000;
pub const SAFER_LEVELID_CONSTRAINED: DWORD = 0x10000;
pub const SAFER_LEVELID_NORMALUSER: DWORD = 0x20000;
pub const SAFER_LEVELID_FULLYTRUSTED: DWORD = 0x40000;
pub const SAFER_LEVEL_OPEN: DWORD = 1;
pub const SAFER_MAX_FRIENDLYNAME_SIZE: SIZE_T = 256;
pub const SAFER_MAX_DESCRIPTION_SIZE: SIZE_T = 256;
pub const SAFER_MAX_HASH_SIZE: SIZE_T = 64;
pub const SAFER_TOKEN_NULL_IF_EQUAL: DWORD = 0x00000001;
pub const SAFER_TOKEN_COMPARE_ONLY: DWORD = 0x00000002;
pub const SAFER_TOKEN_MAKE_INERT: DWORD = 0x00000004;
pub const SAFER_TOKEN_WANT_FLAGS: DWORD = 0x00000008;
pub const SAFER_CRITERIA_IMAGEPATH: DWORD = 0x00001;
pub const SAFER_CRITERIA_NOSIGNEDHASH: DWORD = 0x00002;
pub const SAFER_CRITERIA_IMAGEHASH: DWORD = 0x00004;
pub const SAFER_CRITERIA_AUTHENTICODE: DWORD = 0x00008;
pub const SAFER_CRITERIA_URLZONE: DWORD = 0x00010;
pub const SAFER_CRITERIA_APPX_PACKAGE: DWORD = 0x00020;
pub const SAFER_CRITERIA_IMAGEPATH_NT: DWORD = 0x01000;
STRUCT!{struct SAFER_CODE_PROPERTIES_V1 {
    cbSize: DWORD,
    dwCheckFlags: DWORD,
    ImagePath: LPCWSTR,
    hImageFileHandle: HANDLE,
    UrlZoneId: DWORD,
    ImageHash: [BYTE; SAFER_MAX_HASH_SIZE],
    dwImageHashSize: DWORD,
    ImageSize: LARGE_INTEGER,
    HashAlgorithm: ALG_ID,
    pByteBlock: LPBYTE,
    hWndParent: HWND,
    dwWVTUIChoice: DWORD,
}}
pub type PSAFER_CODE_PROPERTIES_V1 = *mut SAFER_CODE_PROPERTIES_V1;
STRUCT!{struct SAFER_CODE_PROPERTIES_V2 {
    cbSize: DWORD,
    dwCheckFlags: DWORD,
    ImagePath: LPCWSTR,
    hImageFileHandle: HANDLE,
    UrlZoneId: DWORD,
    ImageHash: [BYTE; SAFER_MAX_HASH_SIZE],
    dwImageHashSize: DWORD,
    ImageSize: LARGE_INTEGER,
    HashAlgorithm: ALG_ID,
    pByteBlock: LPBYTE,
    hWndParent: HWND,
    dwWVTUIChoice: DWORD,
    PackageMoniker: LPCWSTR,
    PackagePublisher: LPCWSTR,
    PackageName: LPCWSTR,
    PackageVersion: ULONG64,
    PackageIsFramework: BOOL,
}}
pub type PSAFER_CODE_PROPERTIES_V2 = *mut SAFER_CODE_PROPERTIES_V2;
pub type SAFER_CODE_PROPERTIES = SAFER_CODE_PROPERTIES_V2;
pub type PSAFER_CODE_PROPERTIES = *mut SAFER_CODE_PROPERTIES;
pub const SAFER_POLICY_JOBID_MASK: DWORD = 0xFF000000;
pub const SAFER_POLICY_JOBID_CONSTRAINED: DWORD = 0x04000000;
pub const SAFER_POLICY_JOBID_UNTRUSTED: DWORD = 0x03000000;
pub const SAFER_POLICY_ONLY_EXES: DWORD = 0x00010000;
pub const SAFER_POLICY_SANDBOX_INERT: DWORD = 0x00020000;
pub const SAFER_POLICY_HASH_DUPLICATE: DWORD = 0x00040000;
pub const SAFER_POLICY_ONLY_AUDIT: DWORD = 0x00001000;
pub const SAFER_POLICY_BLOCK_CLIENT_UI: DWORD = 0x00002000;
pub const SAFER_POLICY_UIFLAGS_MASK: DWORD = 0x000000FF;
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: DWORD = 0x00000001;
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: DWORD = 0x00000002;
pub const SAFER_POLICY_UIFLAGS_HIDDEN: DWORD = 0x00000004;
ENUM!{enum SAFER_POLICY_INFO_CLASS {
    SaferPolicyLevelList = 1,
    SaferPolicyEnableTransparentEnforcement,
    SaferPolicyDefaultLevel,
    SaferPolicyEvaluateUserScope,
    SaferPolicyScopeFlags,
    SaferPolicyDefaultLevelFlags,
    SaferPolicyAuthenticodeEnabled,
}}
ENUM!{enum SAFER_OBJECT_INFO_CLASS {
    SaferObjectLevelId = 1,
    SaferObjectScopeId,
    SaferObjectFriendlyName,
    SaferObjectDescription,
    SaferObjectBuiltin,
    SaferObjectDisallowed,
    SaferObjectDisableMaxPrivilege,
    SaferObjectInvertDeletedPrivileges,
    SaferObjectDeletedPrivileges,
    SaferObjectDefaultOwner,
    SaferObjectSidsToDisable,
    SaferObjectRestrictedSidsInverted,
    SaferObjectRestrictedSidsAdded,
    SaferObjectAllIdentificationGuids,
    SaferObjectSingleIdentification,
    SaferObjectExtendedError,
}}
ENUM!{enum SAFER_IDENTIFICATION_TYPES {
    SaferIdentityDefault,
    SaferIdentityTypeImageName = 1,
    SaferIdentityTypeImageHash,
    SaferIdentityTypeUrlZone,
    SaferIdentityTypeCertificate,
}}
STRUCT!{struct SAFER_IDENTIFICATION_HEADER {
    dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    cbStructSize: DWORD,
    IdentificationGuid: GUID,
    lastModified: FILETIME,
}}
pub type PSAFER_IDENTIFICATION_HEADER = *mut SAFER_IDENTIFICATION_HEADER;
STRUCT!{struct SAFER_PATHNAME_IDENTIFICATION {
    header: SAFER_IDENTIFICATION_HEADER,
    Description: [WCHAR; SAFER_MAX_DESCRIPTION_SIZE],
    ImageName: PWCHAR,
    dwSaferFlags: DWORD,
}}
pub type PSAFER_PATHNAME_IDENTIFICATION = *mut SAFER_PATHNAME_IDENTIFICATION;
STRUCT!{struct SAFER_HASH_IDENTIFICATION {
    header: SAFER_IDENTIFICATION_HEADER,
    Description: [WCHAR; SAFER_MAX_DESCRIPTION_SIZE],
    FriendlyName: [WCHAR; SAFER_MAX_DESCRIPTION_SIZE],
    HashSize: DWORD,
    ImageHash: [BYTE; SAFER_MAX_HASH_SIZE],
    HashAlgorithm: ALG_ID,
    ImageSize: LARGE_INTEGER,
    dwSaferFlags: DWORD,
}}
pub type PSAFER_HASH_IDENTIFICATION = *mut SAFER_HASH_IDENTIFICATION;
STRUCT!{struct SAFER_HASH_IDENTIFICATION2 {
    hashIdentification: SAFER_HASH_IDENTIFICATION,
    HashSize: DWORD,
    ImageHash: [BYTE; SAFER_MAX_HASH_SIZE],
    HashAlgorithm: ALG_ID,
}}
pub type PSAFER_HASH_IDENTIFICATION2 = *mut SAFER_HASH_IDENTIFICATION2;
STRUCT!{struct SAFER_URLZONE_IDENTIFICATION {
    header: SAFER_IDENTIFICATION_HEADER,
    UrlZoneId: DWORD,
    dwSaferFlags: DWORD,
}}
pub type PSAFER_URLZONE_IDENTIFICATION = *mut SAFER_URLZONE_IDENTIFICATION;
extern "system" {
    pub fn SaferGetPolicyInformation(
        dwScopeId: DWORD,
        SaferPolicyInfoClass: SAFER_POLICY_INFO_CLASS,
        InfoBufferSize: DWORD,
        InfoBuffer: PVOID,
        InfoBufferRetSize: PDWORD,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferSetPolicyInformation(
        dwScopeId: DWORD,
        SaferPolicyInfoClass: SAFER_POLICY_INFO_CLASS,
        InfoBufferSize: DWORD,
        InfoBuffer: PVOID,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferCreateLevel(
        dwScopeId: DWORD,
        dwLevelId: DWORD,
        OpenFlags: DWORD,
        pLevelHandle: *mut SAFER_LEVEL_HANDLE,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferCloseLevel(
        hLevelHandle: SAFER_LEVEL_HANDLE,
    ) -> BOOL;
    pub fn SaferIdentifyLevel(
        dwNumProperties: DWORD,
        pCodeProperties: PSAFER_CODE_PROPERTIES,
        pLevelHandle: *mut SAFER_LEVEL_HANDLE,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferComputeTokenFromLevel(
        LevelHandle: SAFER_LEVEL_HANDLE,
        InAccessToken: HANDLE,
        OutAccessToken: PHANDLE,
        dwFlags: DWORD,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferGetLevelInformation(
        LevelHandle: SAFER_LEVEL_HANDLE,
        dwInfoType: SAFER_OBJECT_INFO_CLASS,
        lpQueryBuffer: LPVOID,
        dwInBufferSize: DWORD,
        lpdwOutBufferSize: LPDWORD,
    ) -> BOOL;
    pub fn SaferSetLevelInformation(
        LevelHandle: SAFER_LEVEL_HANDLE,
        dwInfoType: SAFER_OBJECT_INFO_CLASS,
        lpQueryBuffer: LPVOID,
        dwInBufferSize: DWORD,
    ) -> BOOL;
    pub fn SaferRecordEventLogEntry(
        hLevel: SAFER_LEVEL_HANDLE,
        szTargetPath: LPCWSTR,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn SaferiIsExecutableFileType(
        szFullPath: LPCWSTR,
        bFromShellExecute: BOOLEAN,
    ) -> BOOL;
}
pub const SRP_POLICY_EXE: &'static str = "EXE";
pub const SRP_POLICY_DLL: &'static str = "DLL";
pub const SRP_POLICY_MSI: &'static str = "MSI";
pub const SRP_POLICY_SCRIPT: &'static str = "SCRIPT";
pub const SRP_POLICY_SHELL: &'static str = "SHELL";
pub const SRP_POLICY_NOV2: &'static str = "IGNORESRPV2";
pub const SRP_POLICY_APPX: &'static str = "APPX";
pub const SRP_POLICY_WLDPMSI: &'static str = "WLDPMSI";
pub const SRP_POLICY_WLDPSCRIPT: &'static str = "WLDPSCRIPT";
pub const SRP_POLICY_WLDPCONFIGCI: &'static str = "WLDPCONFIGCI";
pub const SRP_POLICY_MANAGEDINSTALLER: &'static str = "MANAGEDINSTALLER";
