// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Data Protection API Prototypes and Definitions
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID};
use shared::windef::HWND;
use um::wincrypt::DATA_BLOB;
use um::winnt::{LPCWSTR, LPWSTR, PSID, PVOID};
pub const szFORCE_KEY_PROTECTION: &'static str = "ForceKeyProtection";
pub const dwFORCE_KEY_PROTECTION_DISABLED: DWORD = 0x0;
pub const dwFORCE_KEY_PROTECTION_USER_SELECT: DWORD = 0x1;
pub const dwFORCE_KEY_PROTECTION_HIGH: DWORD = 0x2;
STRUCT!{struct CRYPTPROTECT_PROMPTSTRUCT {
    cbSize: DWORD,
    dwPromptFlags: DWORD,
    hwndApp: HWND,
    szPrompt: LPCWSTR,
}}
pub type PCRYPTPROTECT_PROMPTSTRUCT = *mut CRYPTPROTECT_PROMPTSTRUCT;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: DWORD = 0x1;
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: DWORD = 0x2;
pub const CRYPTPROTECT_PROMPT_RESERVED: DWORD = 0x04;
pub const CRYPTPROTECT_PROMPT_STRONG: DWORD = 0x08;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: DWORD = 0x10;
pub const CRYPTPROTECT_UI_FORBIDDEN: DWORD = 0x1;
pub const CRYPTPROTECT_LOCAL_MACHINE: DWORD = 0x4;
pub const CRYPTPROTECT_CRED_SYNC: DWORD = 0x8;
pub const CRYPTPROTECT_AUDIT: DWORD = 0x10;
pub const CRYPTPROTECT_NO_RECOVERY: DWORD = 0x20;
pub const CRYPTPROTECT_VERIFY_PROTECTION: DWORD = 0x40;
pub const CRYPTPROTECT_CRED_REGENERATE: DWORD = 0x80;
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: DWORD = 0x0FFFFFFF;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: DWORD = 0xFFFFFFFF;
extern "system" {
    pub fn CryptProtectData(
        pDataIn: *mut DATA_BLOB,
        szDataDescr: LPCWSTR,
        pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID,
        pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT,
        dwFlags: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptUnprotectData(
        pDataIn: *mut DATA_BLOB,
        ppszDataDescr: *mut LPWSTR,
        pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID,
        pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT,
        dwFlags: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptProtectDataNoUI(
        pDataIn: *mut DATA_BLOB,
        szDataDescr: LPCWSTR,
        pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID,
        pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT,
        dwFlags: DWORD,
        pbOptionalPassword: *const BYTE,
        cbOptionalPassword: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptUnprotectDataNoUI(
        pDataIn: *mut DATA_BLOB,
        ppszDataDescr: *mut LPWSTR,
        pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID,
        pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT,
        dwFlags: DWORD,
        pbOptionalPassword: *const BYTE,
        cbOptionalPassword: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptUpdateProtectedState(
        pOldSid: PSID,
        pwszOldPassword: LPCWSTR,
        dwFlags: DWORD,
        pdwSuccessCount: *mut DWORD,
        pdwFailureCount: *mut DWORD,
    ) -> BOOL;
}
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: DWORD = 16;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: DWORD = 0x00;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: DWORD = 0x01;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: DWORD = 0x02;
extern "system" {
    pub fn CryptProtectMemory(
        pDataIn: LPVOID,
        cbDataIn: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptUnprotectMemory(
        pDataIn: LPVOID,
        cbDataIn: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
}
