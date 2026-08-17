// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::bcrypt::{BCRYPT_NO_KEY_VALIDATION, BCryptBufferDesc};
use shared::minwindef::{DWORD, LPVOID, PBYTE};
use um::winnt::{LONG, LPCWSTR, VOID};
pub type SECURITY_STATUS = LONG;
pub type HCRYPTPROV = ULONG_PTR;
pub type HCRYPTKEY = ULONG_PTR;
pub type HCRYPTHASH = ULONG_PTR;
FN!{stdcall PFN_NCRYPT_ALLOC(
    cbSize: SIZE_T,
) -> LPVOID}
FN!{stdcall PFN_NCRYPT_FREE(
    pv: LPVOID,
) -> VOID}
STRUCT!{struct NCRYPT_ALLOC_PARA {
    cbSize: DWORD,
    pfnAlloc: PFN_NCRYPT_ALLOC,
    pfnFree: PFN_NCRYPT_FREE,
}}
pub type NCryptBufferDesc = BCryptBufferDesc;
pub type NCRYPT_HANDLE = ULONG_PTR;
pub type NCRYPT_PROV_HANDLE = ULONG_PTR;
pub type NCRYPT_KEY_HANDLE = ULONG_PTR;
pub type NCRYPT_HASH_HANDLE = ULONG_PTR;
pub type NCRYPT_SECRET_HANDLE = ULONG_PTR;
pub const NCRYPT_NO_PADDING_FLAG: DWORD = 0x00000001;
pub const NCRYPT_PAD_PKCS1_FLAG: DWORD = 0x00000002;
pub const NCRYPT_PAD_OAEP_FLAG: DWORD = 0x00000004;
pub const NCRYPT_PAD_PSS_FLAG: DWORD = 0x00000008;
pub const NCRYPT_PAD_CIPHER_FLAG: DWORD = 0x00000010;
pub const NCRYPT_ATTESTATION_FLAG: DWORD = 0x00000020;
pub const NCRYPT_SEALING_FLAG: DWORD = 0x00000100;
pub const NCRYPT_REGISTER_NOTIFY_FLAG: DWORD = 0x00000001;
pub const NCRYPT_UNREGISTER_NOTIFY_FLAG: DWORD = 0x00000002;
pub const NCRYPT_NO_KEY_VALIDATION: DWORD = BCRYPT_NO_KEY_VALIDATION;
pub const NCRYPT_MACHINE_KEY_FLAG: DWORD = 0x00000020;
pub const NCRYPT_SILENT_FLAG: DWORD = 0x00000040;
pub const NCRYPT_OVERWRITE_KEY_FLAG: DWORD = 0x00000080;
pub const NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG: DWORD = 0x00000200;
pub const NCRYPT_DO_NOT_FINALIZE_FLAG: DWORD = 0x00000400;
pub const NCRYPT_EXPORT_LEGACY_FLAG: DWORD = 0x00000800;
pub const NCRYPT_IGNORE_DEVICE_STATE_FLAG: DWORD = 0x00001000;
pub const NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG: DWORD = 0x00002000;
pub const NCRYPT_NO_CACHED_PASSWORD: DWORD = 0x00004000;
pub const NCRYPT_PROTECT_TO_LOCAL_SYSTEM: DWORD = 0x00008000;
pub const NCRYPT_PERSIST_ONLY_FLAG: DWORD = 0x40000000;
pub const NCRYPT_PERSIST_FLAG: DWORD = 0x80000000;
pub const NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG: DWORD = 0x00010000;
pub const NCRYPT_USE_VIRTUAL_ISOLATION_FLAG: DWORD = 0x00020000;
pub const NCRYPT_USE_PER_BOOT_KEY_FLAG: DWORD = 0x00040000;
extern "system" {
    pub fn NCryptOpenStorageProvider(
        phProvider: *mut NCRYPT_PROV_HANDLE,
        pszProviderName: LPCWSTR,
        dwFlags: DWORD,
    ) -> SECURITY_STATUS;
}
pub const NCRYPT_ALLOW_EXPORT_FLAG: DWORD = 0x00000001;
pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: DWORD = 0x00000002;
pub const NCRYPT_ALLOW_ARCHIVING_FLAG: DWORD = 0x00000004;
pub const NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: DWORD = 0x00000008;
extern "system" {
    pub fn NCryptSetProperty(
        hObject: NCRYPT_HANDLE,
        pszProperty: LPCWSTR,
        pbInput: PBYTE,
        cbInput: DWORD,
        dwFlags: DWORD,
    ) -> SECURITY_STATUS;
    pub fn NCryptImportKey(
        hProvider: NCRYPT_PROV_HANDLE,
        hImportKey: NCRYPT_KEY_HANDLE,
        pszBlobType: LPCWSTR,
        pParameterList: *const NCryptBufferDesc,
        phKey: *mut NCRYPT_KEY_HANDLE,
        pbData: PBYTE,
        cbData: DWORD,
        dwFlags: DWORD,
    ) -> SECURITY_STATUS;
    pub fn NCryptFreeObject(
        hObject: NCRYPT_HANDLE,
    ) -> SECURITY_STATUS;
}
