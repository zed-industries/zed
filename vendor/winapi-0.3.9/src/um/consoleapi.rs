// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-console-l1
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, UINT};
use um::wincon::{PCONSOLE_READCONSOLE_CONTROL, PHANDLER_ROUTINE};
use um::wincontypes::{COORD, HPCON, PINPUT_RECORD};
use um::winnt::{HANDLE, HRESULT, VOID};
extern "system" {
    pub fn AllocConsole() -> BOOL;
    pub fn GetConsoleCP() -> UINT;
    pub fn GetConsoleMode(
        hConsoleHandle: HANDLE,
        lpMode: LPDWORD,
    ) -> BOOL;
    pub fn GetConsoleOutputCP() -> UINT;
    pub fn GetNumberOfConsoleInputEvents(
        hConsoleInput: HANDLE,
        lpNumberOfEvents: LPDWORD,
    ) -> BOOL;
    pub fn PeekConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: PINPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleA(
        hConsoleInput: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD,
        pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadConsoleW(
        hConsoleInput: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD,
        pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: PINPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn ReadConsoleInputW(
        hConsoleInput: HANDLE,
        lpBuffer: PINPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;
    pub fn SetConsoleCtrlHandler(
        HandlerRoutine: PHANDLER_ROUTINE,
        Add: BOOL,
    ) -> BOOL;
    pub fn SetConsoleMode(
        hConsoleHandle: HANDLE,
        dwMode: DWORD,
    ) -> BOOL;
    pub fn WriteConsoleA(
        hConsoleOutput: HANDLE,
        lpBuffer: *const VOID,
        nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn WriteConsoleW(
        hConsoleOutput: HANDLE,
        lpBuffer: *const VOID,
        nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD,
        lpReserved: LPVOID,
    ) -> BOOL;
    pub fn CreatePseudoConsole(
        size: COORD,
        hInput: HANDLE,
        hOutput: HANDLE,
        dwFlags: DWORD,
        phPC: *mut HPCON,
    ) -> HRESULT;
    pub fn ResizePseudoConsole(
        hPC: HPCON,
        size: COORD,
    ) -> HRESULT;
    pub fn ClosePseudoConsole(
        hPC: HPCON,
    );
}
