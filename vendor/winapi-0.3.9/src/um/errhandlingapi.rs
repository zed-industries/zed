// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-errorhandling-l1
use shared::basetsd::ULONG_PTR;
use shared::minwindef::{BOOL, DWORD, LPDWORD, UINT, ULONG};
use um::winnt::{
    EXCEPTION_POINTERS, LONG, LPCSTR, LPCWSTR, PCONTEXT, PEXCEPTION_RECORD,
    PVECTORED_EXCEPTION_HANDLER, PVOID,
};
FN!{stdcall PTOP_LEVEL_EXCEPTION_FILTER(
    ExceptionInfo: *mut EXCEPTION_POINTERS,
) -> LONG}
pub type LPTOP_LEVEL_EXCEPTION_FILTER = PTOP_LEVEL_EXCEPTION_FILTER;
extern "system" {
    pub fn RaiseException(
        dwExceptionCode: DWORD,
        dwExceptionFlags: DWORD,
        nNumberOfArguments: DWORD,
        lpArguments: *const ULONG_PTR,
    );
    pub fn UnhandledExceptionFilter(
        ExceptionInfo: *mut EXCEPTION_POINTERS,
    ) -> LONG;
    pub fn SetUnhandledExceptionFilter(
        lpTopLevelExceptionFilter: LPTOP_LEVEL_EXCEPTION_FILTER,
    ) -> LPTOP_LEVEL_EXCEPTION_FILTER;
    pub fn GetLastError() -> DWORD;
    pub fn SetLastError(
        dwErrCode: DWORD,
    );
    pub fn GetErrorMode() -> UINT;
    pub fn SetErrorMode(
        uMode: UINT,
    ) -> UINT;
    pub fn AddVectoredExceptionHandler(
        First: ULONG,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> PVOID;
    pub fn RemoveVectoredExceptionHandler(
        Handle: PVOID,
    ) -> ULONG;
    pub fn AddVectoredContinueHandler(
        First: ULONG,
        Handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> PVOID;
    pub fn RemoveVectoredContinueHandler(
        Handle: PVOID,
    ) -> ULONG;
}
// RestoreLastError
extern "system" {
    pub fn RaiseFailFastException(
        pExceptionRecord: PEXCEPTION_RECORD,
        pContextRecord: PCONTEXT,
        dwFlags: DWORD,
    );
    pub fn FatalAppExitA(
        uAction: UINT,
        lpMessageText: LPCSTR,
    );
    pub fn FatalAppExitW(
        uAction: UINT,
        lpMessageText: LPCWSTR,
    );
    pub fn GetThreadErrorMode() -> DWORD;
    pub fn SetThreadErrorMode(
        dwNewMode: DWORD,
        lpOldMode: LPDWORD,
    ) -> BOOL;
}
// What library provides this function?
// TerminateProcessOnMemoryExhaustion
