// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{PSIZE_T, SIZE_T};
use shared::minwindef::{BOOL, DWORD, LPCVOID, LPDWORD, LPVOID};
use shared::ntdef::{HANDLE};
use um::minwinbase::LPENCLAVE_ROUTINE;
use um::winnt::{LPCSTR, LPCWSTR};
extern "system" {
    pub fn IsEnclaveTypeSupported(
        flEnclaveType: DWORD,
    ) -> BOOL;
    pub fn CreateEnclave(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        dwInitialCommitment: SIZE_T,
        flEnclaveType: DWORD,
        lpEnclaveInformation: LPCVOID,
        dwInfoLength: DWORD,
        lpEnclaveError: LPDWORD,
    ) -> LPVOID;
    pub fn LoadEnclaveData(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        lpBuffer: LPCVOID,
        nSize: SIZE_T,
        flProtect: DWORD,
        lpPageInformation: LPCVOID,
        dwInfoLength: DWORD,
        lpNumberOfBytesWritten: PSIZE_T,
        lpEnclaveError: LPDWORD,
    ) -> BOOL;
    pub fn InitializeEnclave(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        lpEnclaveInformation: LPCVOID,
        dwInfoLength: DWORD,
        lpEnclaveError: LPDWORD,
    ) -> BOOL;
    pub fn LoadEnclaveImageA(
        lpEnclaveAddress: LPVOID,
        lpImageName: LPCSTR,
    ) -> BOOL;
    pub fn LoadEnclaveImageW(
        lpEnclaveAddress: LPVOID,
        lpImageName: LPCWSTR,
    ) -> BOOL;
    pub fn CallEnclave(
        lpRoutine: LPENCLAVE_ROUTINE,
        lpParameter: LPVOID,
        fWaitForThread: BOOL,
        lpReturnValue: *mut LPVOID,
    ) -> BOOL;
    pub fn TerminateEnclave(
        lpAddress: LPVOID,
        fWait: BOOL,
    ) -> BOOL;
    pub fn DeleteEnclave(
        lpAddress: LPVOID,
    ) -> BOOL;
}
