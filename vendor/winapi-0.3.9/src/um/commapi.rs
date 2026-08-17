// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_char;
use shared::minwindef::{BOOL, DWORD, LPDWORD};
use um::minwinbase::LPOVERLAPPED;
use um::winbase::{LPCOMMCONFIG, LPCOMMPROP, LPCOMMTIMEOUTS, LPCOMSTAT, LPDCB};
use um::winnt::HANDLE;
extern "system" {
    pub fn ClearCommBreak(
        hFile: HANDLE,
    ) -> BOOL;
    pub fn ClearCommError(
        hFile: HANDLE,
        lpErrors: LPDWORD,
        lpStat: LPCOMSTAT,
    ) -> BOOL;
    pub fn SetupComm(
        hFile: HANDLE,
        dwInQueue: DWORD,
        dwOutQueue: DWORD,
    ) -> BOOL;
    pub fn EscapeCommFunction(
        hFile: HANDLE,
        dwFunc: DWORD,
    ) -> BOOL;
    pub fn GetCommConfig(
        hCommDev: HANDLE,
        lpCC: LPCOMMCONFIG,
        lpdwSize: LPDWORD,
    ) -> BOOL;
    pub fn GetCommMask(
        hFile: HANDLE,
        lpEvtMask: LPDWORD,
    ) -> BOOL;
    pub fn GetCommModemStatus(
        hFile: HANDLE,
        lpModemStat: LPDWORD,
    ) -> BOOL;
    pub fn GetCommProperties(
        hFile: HANDLE,
        lpCommProp: LPCOMMPROP,
    ) -> BOOL;
    pub fn GetCommState(
        hFile: HANDLE,
        lpDCB: LPDCB,
    ) -> BOOL;
    pub fn GetCommTimeouts(
        hFile: HANDLE,
        lpCommTimeouts: LPCOMMTIMEOUTS,
    ) -> BOOL;
    pub fn PurgeComm(
        hFile: HANDLE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn SetCommBreak(
        hFile: HANDLE,
    ) -> BOOL;
    pub fn SetCommConfig(
        hCommDev: HANDLE,
        lpCC: LPCOMMCONFIG,
        dwSize: DWORD,
    ) -> BOOL;
    pub fn SetCommMask(
        hFile: HANDLE,
        dwEvtMask: DWORD,
    ) -> BOOL;
    pub fn SetCommState(
        hFile: HANDLE,
        lpDCB: LPDCB,
    ) -> BOOL;
    pub fn SetCommTimeouts(
        hFile: HANDLE,
        lpCommTimeouts: LPCOMMTIMEOUTS,
    ) -> BOOL;
    pub fn TransmitCommChar(
        hFile: HANDLE,
        cChar: c_char,
    ) -> BOOL;
    pub fn WaitCommEvent(
        hFile: HANDLE,
        lpEvtMask: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
}
