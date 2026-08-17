// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Function prototypes for Windows Error Reporting (WER)
use shared::minwindef::{BOOL, DWORD, PDWORD};
use um::winnt::{HANDLE, HRESULT, PCWSTR, PVOID};
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: DWORD = 1;
pub const WER_FAULT_REPORTING_FLAG_QUEUE: DWORD = 2;
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: DWORD = 4;
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: DWORD = 8;
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: DWORD = 16;
pub const WER_FAULT_REPORTING_NO_UI: DWORD = 32;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: DWORD = 64;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: DWORD = 128;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: DWORD = 256;
pub const WER_FAULT_REPORTING_CRITICAL: DWORD = 512;
pub const WER_FAULT_REPORTING_DURABLE: DWORD = 1024;
ENUM!{enum WER_REGISTER_FILE_TYPE {
    WerRegFileTypeUserDocument = 1,
    WerRegFileTypeOther = 2,
    WerRegFileTypeMax,
}}
extern "system" {
    pub fn WerRegisterFile(
        pwzFile: PCWSTR,
        regFileType: WER_REGISTER_FILE_TYPE,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn WerUnregisterFile(
        pwzFilePath: PCWSTR,
    ) -> HRESULT;
    pub fn WerRegisterMemoryBlock(
        pvAddress: PVOID,
        dwSize: DWORD,
    ) -> HRESULT;
    pub fn WerUnregisterMemoryBlock(
        pvAddress: PVOID,
    ) -> HRESULT;
    pub fn WerSetFlags(
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn WerGetFlags(
        hProcess: HANDLE,
        pdwFlags: PDWORD,
    ) -> HRESULT;
    pub fn WerAddExcludedApplication(
        pwzExeName: PCWSTR,
        bAllUsers: BOOL,
    ) -> HRESULT;
    pub fn WerRemoveExcludedApplication(
        pwzExeName: PCWSTR,
        bAllUsers: BOOL,
    ) -> HRESULT;
    pub fn WerRegisterRuntimeExceptionModule(
        pwszOutOfProcessCallbackDll: PCWSTR,
        pContext: PVOID,
    ) -> HRESULT;
    pub fn WerUnregisterRuntimeExceptionModule(
        pwszOutOfProcessCallbackDll: PCWSTR,
        pContext: PVOID,
    ) -> HRESULT;
}
