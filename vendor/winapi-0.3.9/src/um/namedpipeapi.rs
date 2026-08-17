// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, ULONG};
use um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES};
use um::winnt::{HANDLE, LPCWSTR, LPWSTR, PHANDLE};
extern "system" {
    pub fn CreatePipe(
        hReadPipe: PHANDLE,
        hWritePipe: PHANDLE,
        lpPipeAttributes: LPSECURITY_ATTRIBUTES,
        nSize: DWORD,
    ) -> BOOL;
    pub fn ConnectNamedPipe(
        hNamedPipe: HANDLE,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn DisconnectNamedPipe(
        hNamedPipe: HANDLE,
    ) -> BOOL;
    pub fn SetNamedPipeHandleState(
        hNamedPipe: HANDLE,
        lpMode: LPDWORD,
        lpMaxCollectionCount: LPDWORD,
        lpCollectDataTimeout: LPDWORD,
    ) -> BOOL;
    pub fn PeekNamedPipe(
        hNamedPipe: HANDLE,
        lpBuffer: LPVOID,
        nBufferSize: DWORD,
        lpBytesRead: LPDWORD,
        lpTotalBytesAvail: LPDWORD,
        lpBytesLeftThisMessage: LPDWORD,
    ) -> BOOL;
    pub fn TransactNamedPipe(
        hNamedPipe: HANDLE,
        lpInBuffer: LPVOID,
        nInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        nOutBufferSize: DWORD,
        lpBytesRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn CreateNamedPipeW(
        lpName: LPCWSTR,
        dwOpenMode: DWORD,
        dwPipeMode: DWORD,
        nMaxInstances: DWORD,
        nOutBufferSize: DWORD,
        nInBufferSize: DWORD,
        nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> HANDLE;
    pub fn WaitNamedPipeW(
        lpNamedPipeName: LPCWSTR,
        nTimeOut: DWORD,
    ) -> BOOL;
    pub fn GetNamedPipeClientComputerNameW(
        Pipe: HANDLE,
        ClientComputerName: LPWSTR,
        ClientComputerNameLength: ULONG,
    ) -> BOOL;
    pub fn ImpersonateNamedPipeClient(
        hNamedPipe: HANDLE,
    ) -> BOOL;
    pub fn GetNamedPipeInfo(
        hNamedPipe: HANDLE,
        lpFlags: LPDWORD,
        lpOutBufferSize: LPDWORD,
        lpInBufferSize: LPDWORD,
        lpMaxInstances: LPDWORD,
    ) -> BOOL;
    pub fn GetNamedPipeHandleStateW(
        hNamedPipe: HANDLE,
        lpState: LPDWORD,
        lpCurInstances: LPDWORD,
        lpMaxCollectionCount: LPDWORD,
        lpCollectDataTimeout: LPDWORD,
        lpUserName: LPWSTR,
        nMaxUserNameSize: DWORD,
    ) -> BOOL;
    pub fn CallNamedPipeW(
        lpNamedPipeName: LPCWSTR,
        lpInBuffer: LPVOID,
        nInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        nOutBufferSize: DWORD,
        lpBytesRead: LPDWORD,
        nTimeOut: DWORD,
    ) -> BOOL;
}
