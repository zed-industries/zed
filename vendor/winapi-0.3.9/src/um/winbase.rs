// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the 32-Bit Windows Base APIs
use ctypes::{c_char, c_int, c_long, c_void};
use shared::basetsd::{
    DWORD64, DWORD_PTR, LONG_PTR, PDWORD64, PDWORD_PTR, PSIZE_T, PULONG_PTR, SIZE_T, UINT_PTR,
    ULONG_PTR,
};
use shared::guiddef::GUID;
use shared::minwindef::{
    ATOM, BOOL, BYTE, DWORD, FARPROC, FILETIME, HFILE, HGLOBAL, HLOCAL, HMODULE, HRSRC, LPBOOL,
    LPBYTE, LPCVOID, LPDWORD, LPFILETIME, LPVOID, LPWORD, PBOOL, PDWORD, PUCHAR, PULONG, PUSHORT,
    UCHAR, UINT, ULONG, USHORT, WORD,
};
use shared::windef::HWND;
use um::cfgmgr32::MAX_PROFILE_LEN;
use um::fileapi::STREAM_INFO_LEVELS;
use um::libloaderapi::{
    ENUMRESLANGPROCA, ENUMRESLANGPROCW, ENUMRESNAMEPROCA, ENUMRESTYPEPROCA, ENUMRESTYPEPROCW,
};
use um::minwinbase::{
    FILE_INFO_BY_HANDLE_CLASS, FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS, GET_FILEEX_INFO_LEVELS,
    LPOVERLAPPED, LPOVERLAPPED_COMPLETION_ROUTINE, LPSECURITY_ATTRIBUTES, PREASON_CONTEXT,
};
use um::processthreadsapi::{
    LPPROCESS_INFORMATION, LPPROC_THREAD_ATTRIBUTE_LIST, LPSTARTUPINFOA, LPSTARTUPINFOW,
    STARTUPINFOA, STARTUPINFOW,
};
use um::winnt::{
    BOOLEAN, CHAR, DWORDLONG, EXECUTION_STATE, FILE_ID_128, HANDLE, HRESULT, INT, LANGID,
    LARGE_INTEGER, LATENCY_TIME, LONG, LPCCH, LPCH, LPCSTR, LPCWSTR, LPOSVERSIONINFOEXA,
    LPOSVERSIONINFOEXW, LPSTR, LPWSTR, MAXLONG, PBOOLEAN, PCONTEXT, PCWSTR, PFIRMWARE_TYPE,
    PHANDLE, PIO_COUNTERS, PJOB_SET_ARRAY, PLUID, POWER_REQUEST_TYPE, PPERFORMANCE_DATA,
    PPROCESSOR_NUMBER, PQUOTA_LIMITS, PRTL_UMS_SCHEDULER_ENTRY_POINT,
    PSECURE_MEMORY_CACHE_CALLBACK, PSID, PSID_NAME_USE, PULONGLONG, PVOID, PWOW64_CONTEXT,
    PWOW64_LDT_ENTRY, PWSTR, RTL_UMS_THREAD_INFO_CLASS, STATUS_ABANDONED_WAIT_0, STATUS_USER_APC,
    STATUS_WAIT_0, SecurityAnonymous, SecurityDelegation, SecurityIdentification,
    SecurityImpersonation, THREAD_BASE_PRIORITY_IDLE, THREAD_BASE_PRIORITY_LOWRT,
    THREAD_BASE_PRIORITY_MAX, THREAD_BASE_PRIORITY_MIN, ULARGE_INTEGER, VOID, WAITORTIMERCALLBACK,
    WCHAR, WOW64_CONTEXT,
};
#[cfg(target_arch = "x86")]
use um::winnt::PLDT_ENTRY;
use vc::vadefs::va_list;
pub const FILE_BEGIN: DWORD = 0;
pub const FILE_CURRENT: DWORD = 1;
pub const FILE_END: DWORD = 2;
pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;
pub const WAIT_OBJECT_0: DWORD = STATUS_WAIT_0 as u32;
pub const WAIT_ABANDONED: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
pub const WAIT_ABANDONED_0: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
pub const WAIT_IO_COMPLETION: DWORD = STATUS_USER_APC as u32;
pub const FILE_FLAG_WRITE_THROUGH: DWORD = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: DWORD = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: DWORD = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: DWORD = 0x08000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: DWORD = 0x04000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: DWORD = 0x02000000;
pub const FILE_FLAG_POSIX_SEMANTICS: DWORD = 0x01000000;
pub const FILE_FLAG_SESSION_AWARE: DWORD = 0x00800000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: DWORD = 0x00200000;
pub const FILE_FLAG_OPEN_NO_RECALL: DWORD = 0x00100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: DWORD = 0x00080000;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: DWORD = 0x00040000;
pub const PROGRESS_CONTINUE: DWORD = 0;
pub const PROGRESS_CANCEL: DWORD = 1;
pub const PROGRESS_STOP: DWORD = 2;
pub const PROGRESS_QUIET: DWORD = 3;
pub const CALLBACK_CHUNK_FINISHED: DWORD = 0x00000000;
pub const CALLBACK_STREAM_SWITCH: DWORD = 0x00000001;
pub const COPY_FILE_FAIL_IF_EXISTS: DWORD = 0x00000001;
pub const COPY_FILE_RESTARTABLE: DWORD = 0x00000002;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: DWORD = 0x00000004;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: DWORD = 0x00000008;
pub const COPY_FILE_COPY_SYMLINK: DWORD = 0x00000800;
pub const COPY_FILE_NO_BUFFERING: DWORD = 0x00001000;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: DWORD = 0x00002000;
pub const COPY_FILE_RESUME_FROM_PAUSE: DWORD = 0x00004000;
pub const COPY_FILE_NO_OFFLOAD: DWORD = 0x00040000;
pub const COPY_FILE_IGNORE_EDP_BLOCK: DWORD = 0x00400000;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: DWORD = 0x00800000;
pub const REPLACEFILE_WRITE_THROUGH: DWORD = 0x00000001;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: DWORD = 0x00000002;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: DWORD = 0x00000004;
pub const PIPE_ACCESS_INBOUND: DWORD = 0x00000001;
pub const PIPE_ACCESS_OUTBOUND: DWORD = 0x00000002;
pub const PIPE_ACCESS_DUPLEX: DWORD = 0x00000003;
pub const PIPE_CLIENT_END: DWORD = 0x00000000;
pub const PIPE_SERVER_END: DWORD = 0x00000001;
pub const PIPE_WAIT: DWORD = 0x00000000;
pub const PIPE_NOWAIT: DWORD = 0x00000001;
pub const PIPE_READMODE_BYTE: DWORD = 0x00000000;
pub const PIPE_READMODE_MESSAGE: DWORD = 0x00000002;
pub const PIPE_TYPE_BYTE: DWORD = 0x00000000;
pub const PIPE_TYPE_MESSAGE: DWORD = 0x00000004;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: DWORD = 0x00000000;
pub const PIPE_REJECT_REMOTE_CLIENTS: DWORD = 0x00000008;
pub const PIPE_UNLIMITED_INSTANCES: DWORD = 255;
pub const SECURITY_ANONYMOUS: DWORD = SecurityAnonymous << 16;
pub const SECURITY_IDENTIFICATION: DWORD = SecurityIdentification << 16;
pub const SECURITY_IMPERSONATION: DWORD = SecurityImpersonation << 16;
pub const SECURITY_DELEGATION: DWORD = SecurityDelegation << 16;
pub const SECURITY_CONTEXT_TRACKING: DWORD = 0x00040000;
pub const SECURITY_EFFECTIVE_ONLY: DWORD = 0x00080000;
pub const SECURITY_SQOS_PRESENT: DWORD = 0x00100000;
pub const SECURITY_VALID_SQOS_FLAGS: DWORD = 0x001F0000;
FN!{stdcall PFIBER_START_ROUTINE(
    lpFiberParameter: LPVOID,
) -> ()}
pub type LPFIBER_START_ROUTINE = PFIBER_START_ROUTINE;
FN!{stdcall PFIBER_CALLOUT_ROUTINE(
    lpParameter: LPVOID,
) -> LPVOID}
// FAIL_FAST_*
#[cfg(target_arch = "x86")]
pub type LPLDT_ENTRY = PLDT_ENTRY;
#[cfg(not(target_arch = "x86"))]
pub type LPLDT_ENTRY = LPVOID; // TODO - fix this for 32-bit
//SP_SERIALCOMM
//PST_*
// PCF_*
// SP_*
// BAUD_*
// DATABITS_*
// STOPBITS_*
// PARITY_*
STRUCT!{struct COMMPROP {
    wPacketLength: WORD,
    wPacketVersion: WORD,
    dwServiceMask: DWORD,
    dwReserved1: DWORD,
    dwMaxTxQueue: DWORD,
    dwMaxRxQueue: DWORD,
    dwMaxBaud: DWORD,
    dwProvSubType: DWORD,
    dwProvCapabilities: DWORD,
    dwSettableParams: DWORD,
    dwSettableBaud: DWORD,
    wSettableData: WORD,
    wSettableStopParity: WORD,
    dwCurrentTxQueue: DWORD,
    dwCurrentRxQueue: DWORD,
    dwProvSpec1: DWORD,
    dwProvSpec2: DWORD,
    wcProvChar: [WCHAR; 1],
}}
pub type LPCOMMPROP = *mut COMMPROP;
STRUCT!{struct COMSTAT {
    BitFields: DWORD,
    cbInQue: DWORD,
    cbOutQue: DWORD,
}}
BITFIELD!{COMSTAT BitFields: DWORD [
    fCtsHold set_fCtsHold[0..1],
    fDsrHold set_fDsrHold[1..2],
    fRlsdHold set_fRlsdHold[2..3],
    fXoffHold set_fXoffHold[3..4],
    fXoffSent set_fXoffSent[4..5],
    fEof set_fEof[5..6],
    fTxim set_fTxim[6..7],
    fReserved set_fReserved[7..32],
]}
pub type LPCOMSTAT = *mut COMSTAT;
pub const DTR_CONTROL_DISABLE: DWORD = 0x00;
pub const DTR_CONTROL_ENABLE: DWORD = 0x01;
pub const DTR_CONTROL_HANDSHAKE: DWORD = 0x02;
pub const RTS_CONTROL_DISABLE: DWORD = 0x00;
pub const RTS_CONTROL_ENABLE: DWORD = 0x01;
pub const RTS_CONTROL_HANDSHAKE: DWORD = 0x02;
pub const RTS_CONTROL_TOGGLE: DWORD = 0x03;
STRUCT!{struct DCB {
    DCBlength: DWORD,
    BaudRate: DWORD,
    BitFields: DWORD,
    wReserved: WORD,
    XonLim: WORD,
    XoffLim: WORD,
    ByteSize: BYTE,
    Parity: BYTE,
    StopBits: BYTE,
    XonChar: c_char,
    XoffChar: c_char,
    ErrorChar: c_char,
    EofChar: c_char,
    EvtChar: c_char,
    wReserved1: WORD,
}}
BITFIELD!{DCB BitFields: DWORD [
    fBinary set_fBinary[0..1],
    fParity set_fParity[1..2],
    fOutxCtsFlow set_fOutxCtsFlow[2..3],
    fOutxDsrFlow set_fOutxDsrFlow[3..4],
    fDtrControl set_fDtrControl[4..6],
    fDsrSensitivity set_fDsrSensitivity[6..7],
    fTXContinueOnXoff set_fTXContinueOnXoff[7..8],
    fOutX set_fOutX[8..9],
    fInX set_fInX[9..10],
    fErrorChar set_fErrorChar[10..11],
    fNull set_fNull[11..12],
    fRtsControl set_fRtsControl[12..14],
    fAbortOnError set_fAbortOnError[14..15],
    fDummy2 set_fDummy2[15..32],
]}
pub type LPDCB = *mut DCB;
STRUCT!{struct COMMTIMEOUTS {
    ReadIntervalTimeout: DWORD,
    ReadTotalTimeoutMultiplier: DWORD,
    ReadTotalTimeoutConstant: DWORD,
    WriteTotalTimeoutMultiplier: DWORD,
    WriteTotalTimeoutConstant: DWORD,
}}
pub type LPCOMMTIMEOUTS = *mut COMMTIMEOUTS;
STRUCT!{struct COMMCONFIG {
    dwSize: DWORD,
    wVersion: WORD,
    wReserved: WORD,
    dcb: DCB,
    dwProviderSubType: DWORD,
    dwProviderOffset: DWORD,
    dwProviderSize: DWORD,
    wcProviderData: [WCHAR; 1],
}}
pub type LPCOMMCONFIG = *mut COMMCONFIG;
pub const GMEM_FIXED: UINT = 0x0000;
pub const GMEM_MOVEABLE: UINT = 0x0002;
pub const GMEM_NOCOMPACT: UINT = 0x0010;
pub const GMEM_NODISCARD: UINT = 0x0020;
pub const GMEM_ZEROINIT: UINT = 0x0040;
pub const GMEM_MODIFY: UINT = 0x0080;
pub const GMEM_DISCARDABLE: UINT = 0x0100;
pub const GMEM_NOT_BANKED: UINT = 0x1000;
pub const GMEM_SHARE: UINT = 0x2000;
pub const GMEM_DDESHARE: UINT = 0x2000;
pub const GMEM_NOTIFY: UINT = 0x4000;
pub const GMEM_LOWER: UINT = GMEM_NOT_BANKED;
pub const GMEM_VALID_FLAGS: UINT = 0x7F72;
pub const GMEM_INVALID_HANDLE: UINT = 0x8000;
pub const GHND: UINT = GMEM_MOVEABLE | GMEM_ZEROINIT;
pub const GPTR: UINT = GMEM_FIXED | GMEM_ZEROINIT;
pub const GMEM_DISCARDED: UINT = 0x4000;
pub const GMEM_LOCKCOUNT: UINT = 0x00FF;
STRUCT!{struct MEMORYSTATUS {
    dwLength: DWORD,
    dwMemoryLoad: DWORD,
    dwTotalPhys: SIZE_T,
    dwAvailPhys: SIZE_T,
    dwTotalPageFile: SIZE_T,
    dwAvailPageFile: SIZE_T,
    dwTotalVirtual: SIZE_T,
    dwAvailVirtual: SIZE_T,
}}
pub type LPMEMORYSTATUS = *mut MEMORYSTATUS;
// NUMA_NO_PREFERRED_NODE
pub const DEBUG_PROCESS: DWORD = 0x00000001;
pub const DEBUG_ONLY_THIS_PROCESS: DWORD = 0x00000002;
pub const CREATE_SUSPENDED: DWORD = 0x00000004;
pub const DETACHED_PROCESS: DWORD = 0x00000008;
pub const CREATE_NEW_CONSOLE: DWORD = 0x00000010;
pub const NORMAL_PRIORITY_CLASS: DWORD = 0x00000020;
pub const IDLE_PRIORITY_CLASS: DWORD = 0x00000040;
pub const HIGH_PRIORITY_CLASS: DWORD = 0x00000080;
pub const REALTIME_PRIORITY_CLASS: DWORD = 0x00000100;
pub const CREATE_NEW_PROCESS_GROUP: DWORD = 0x00000200;
pub const CREATE_UNICODE_ENVIRONMENT: DWORD = 0x00000400;
pub const CREATE_SEPARATE_WOW_VDM: DWORD = 0x00000800;
pub const CREATE_SHARED_WOW_VDM: DWORD = 0x00001000;
pub const CREATE_FORCEDOS: DWORD = 0x00002000;
pub const BELOW_NORMAL_PRIORITY_CLASS: DWORD = 0x00004000;
pub const ABOVE_NORMAL_PRIORITY_CLASS: DWORD = 0x00008000;
pub const INHERIT_PARENT_AFFINITY: DWORD = 0x00010000;
pub const INHERIT_CALLER_PRIORITY: DWORD = 0x00020000;
pub const CREATE_PROTECTED_PROCESS: DWORD = 0x00040000;
pub const EXTENDED_STARTUPINFO_PRESENT: DWORD = 0x00080000;
pub const PROCESS_MODE_BACKGROUND_BEGIN: DWORD = 0x00100000;
pub const PROCESS_MODE_BACKGROUND_END: DWORD = 0x00200000;
pub const CREATE_BREAKAWAY_FROM_JOB: DWORD = 0x01000000;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: DWORD = 0x02000000;
pub const CREATE_DEFAULT_ERROR_MODE: DWORD = 0x04000000;
pub const CREATE_NO_WINDOW: DWORD = 0x08000000;
pub const PROFILE_USER: DWORD = 0x10000000;
pub const PROFILE_KERNEL: DWORD = 0x20000000;
pub const PROFILE_SERVER: DWORD = 0x40000000;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: DWORD = 0x80000000;
// STACK_SIZE_PARAM_IS_A_RESERVATION
pub const THREAD_PRIORITY_LOWEST: DWORD = THREAD_BASE_PRIORITY_MIN;
pub const THREAD_PRIORITY_BELOW_NORMAL: DWORD = THREAD_PRIORITY_LOWEST + 1;
pub const THREAD_PRIORITY_NORMAL: DWORD = 0;
pub const THREAD_PRIORITY_HIGHEST: DWORD = THREAD_BASE_PRIORITY_MAX;
pub const THREAD_PRIORITY_ABOVE_NORMAL: DWORD = THREAD_PRIORITY_HIGHEST - 1;
pub const THREAD_PRIORITY_ERROR_RETURN: DWORD = MAXLONG as u32;
pub const THREAD_PRIORITY_TIME_CRITICAL: DWORD = THREAD_BASE_PRIORITY_LOWRT;
pub const THREAD_PRIORITY_IDLE: DWORD = THREAD_BASE_PRIORITY_IDLE;
pub const THREAD_MODE_BACKGROUND_BEGIN: DWORD = 0x00010000;
pub const THREAD_MODE_BACKGROUND_END: DWORD = 0x00020000;
pub const VOLUME_NAME_DOS: DWORD = 0x0;
// VOLUME_NAME_*
// FILE_NAME_*
// JIT_DEBUG_*
pub const DRIVE_UNKNOWN: DWORD = 0;
pub const DRIVE_NO_ROOT_DIR: DWORD = 1;
pub const DRIVE_REMOVABLE: DWORD = 2;
pub const DRIVE_FIXED: DWORD = 3;
pub const DRIVE_REMOTE: DWORD = 4;
pub const DRIVE_CDROM: DWORD = 5;
pub const DRIVE_RAMDISK: DWORD = 6;
// pub fn GetFreeSpace();
pub const FILE_TYPE_UNKNOWN: DWORD = 0x0000;
pub const FILE_TYPE_DISK: DWORD = 0x0001;
pub const FILE_TYPE_CHAR: DWORD = 0x0002;
pub const FILE_TYPE_PIPE: DWORD = 0x0003;
pub const FILE_TYPE_REMOTE: DWORD = 0x8000;
pub const STD_INPUT_HANDLE: DWORD = -10i32 as u32;
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as u32;
pub const STD_ERROR_HANDLE: DWORD = -12i32 as u32;
pub const NOPARITY: BYTE = 0;
pub const ODDPARITY: BYTE = 1;
pub const EVENPARITY: BYTE = 2;
pub const MARKPARITY: BYTE = 3;
pub const SPACEPARITY: BYTE = 4;
pub const ONESTOPBIT: BYTE = 0;
pub const ONE5STOPBITS: BYTE = 1;
pub const TWOSTOPBITS: BYTE = 2;
pub const IGNORE: DWORD = 0;
pub const INFINITE: DWORD = 0xFFFFFFFF;
pub const CBR_110: DWORD = 110;
pub const CBR_300: DWORD = 300;
pub const CBR_600: DWORD = 600;
pub const CBR_1200: DWORD = 1200;
pub const CBR_2400: DWORD = 2400;
pub const CBR_4800: DWORD = 4800;
pub const CBR_9600: DWORD = 9600;
pub const CBR_14400: DWORD = 14400;
pub const CBR_19200: DWORD = 19200;
pub const CBR_38400: DWORD = 38400;
pub const CBR_56000: DWORD = 56000;
pub const CBR_57600: DWORD = 57600;
pub const CBR_115200: DWORD = 115200;
pub const CBR_128000: DWORD = 128000;
pub const CBR_256000: DWORD = 256000;
// CE_*
// IE_*
// EV_*
pub const SETXOFF: DWORD = 1;
pub const SETXON: DWORD = 2;
pub const SETRTS: DWORD = 3;
pub const CLRRTS: DWORD = 4;
pub const SETDTR: DWORD = 5;
pub const CLRDTR: DWORD = 6;
pub const RESETDEV: DWORD = 7;
pub const SETBREAK: DWORD = 8;
pub const CLRBREAK: DWORD = 9;
pub const PURGE_TXABORT: DWORD = 0x0001;
pub const PURGE_RXABORT: DWORD = 0x0002;
pub const PURGE_TXCLEAR: DWORD = 0x0004;
pub const PURGE_RXCLEAR: DWORD = 0x0008;
pub const MS_CTS_ON: DWORD = 0x0010;
pub const MS_DSR_ON: DWORD = 0x0020;
pub const MS_RING_ON: DWORD = 0x0040;
pub const MS_RLSD_ON: DWORD = 0x0080;
// S_*
// NMPWAIT_*
// FS_*
// OF_*
pub const OFS_MAXPATHNAME: usize = 128;
STRUCT!{struct OFSTRUCT {
    cBytes: BYTE,
    fFixedDisk: BYTE,
    nErrCode: WORD,
    Reserved1: WORD,
    Reserved2: WORD,
    szPathName: [CHAR; OFS_MAXPATHNAME],
}}
pub type POFSTRUCT = *mut OFSTRUCT;
pub type LPOFSTRUCT = *mut OFSTRUCT;
extern "system" {
    pub fn GlobalAlloc(
        uFlags: UINT,
        dwBytes: SIZE_T,
    ) -> HGLOBAL;
    pub fn GlobalReAlloc(
        hMem: HGLOBAL,
        dwBytes: SIZE_T,
        uFlags: UINT,
    ) -> HGLOBAL;
    pub fn GlobalSize(
        hMem: HGLOBAL,
    ) -> SIZE_T;
    pub fn GlobalFlags(
        hMem: HGLOBAL,
    ) -> UINT;
    pub fn GlobalLock(
        hMem: HGLOBAL,
    ) -> LPVOID;
    pub fn GlobalHandle(
        pMem: LPCVOID,
    ) -> HGLOBAL;
    pub fn GlobalUnlock(
        hMem: HGLOBAL,
    ) -> BOOL;
    pub fn GlobalFree(
        hMem: HGLOBAL,
    ) -> HGLOBAL;
    pub fn GlobalCompact(
        dwMinFree: DWORD,
    ) -> SIZE_T;
    pub fn GlobalFix(
        hMem: HGLOBAL,
    );
    pub fn GlobalUnfix(
        hMem: HGLOBAL,
    );
    pub fn GlobalWire(
        hMem: HGLOBAL,
    ) -> LPVOID;
    pub fn GlobalUnWire(
        hMem: HGLOBAL,
    ) -> BOOL;
    pub fn GlobalMemoryStatus(
        lpBuffer: LPMEMORYSTATUS,
    );
    pub fn LocalAlloc(
        uFlags: UINT,
        uBytes: SIZE_T,
    ) -> HLOCAL;
    pub fn LocalReAlloc(
        hMem: HLOCAL,
        uBytes: SIZE_T,
        uFlags: UINT,
    ) -> HLOCAL;
    pub fn LocalLock(
        hMem: HLOCAL,
    ) -> LPVOID;
    pub fn LocalHandle(
        pMem: LPCVOID,
    ) -> HLOCAL;
    pub fn LocalUnlock(
        hMem: HLOCAL,
    ) -> BOOL;
    pub fn LocalSize(
        hMem: HLOCAL,
    ) -> SIZE_T;
    pub fn LocalFlags(
        hMem: HLOCAL,
    ) -> UINT;
    pub fn LocalFree(
        hMem: HLOCAL,
    ) -> HLOCAL;
    pub fn LocalShrink(
        hMem: HLOCAL,
        cbNewSize: UINT,
    ) -> SIZE_T;
    pub fn LocalCompact(
        uMinFree: UINT,
    ) -> SIZE_T;
}
// SCS_*
extern "system" {
    pub fn GetBinaryTypeA(
        lpApplicationName: LPCSTR,
        lpBinaryType: LPDWORD,
    ) -> BOOL;
    pub fn GetBinaryTypeW(
        lpApplicationName: LPCWSTR,
        lpBinaryType: LPDWORD,
    ) -> BOOL;
    pub fn GetShortPathNameA(
        lpszLongPath: LPCSTR,
        lpszShortPath: LPSTR,
        cchBuffer: DWORD,
    ) -> DWORD;
    pub fn GetLongPathNameTransactedA(
        lpszShortPath: LPCSTR,
        lpszLongPath: LPSTR,
        cchBuffer: DWORD,
        hTransaction: HANDLE,
    ) -> DWORD;
    pub fn GetLongPathNameTransactedW(
        lpszShortPath: LPCWSTR,
        lpszLongPath: LPWSTR,
        cchBuffer: DWORD,
        hTransaction: HANDLE,
    ) -> DWORD;
    pub fn GetProcessAffinityMask(
        hProcess: HANDLE,
        lpProcessAffinityMask: PDWORD_PTR,
        lpSystemAffinityMask: PDWORD_PTR,
    ) -> BOOL;
    pub fn SetProcessAffinityMask(
        hProcess: HANDLE,
        dwProcessAffinityMask: DWORD,
    ) -> BOOL;
    pub fn GetProcessIoCounters(
        hProcess: HANDLE,
        lpIoCounters: PIO_COUNTERS,
    ) -> BOOL;
    pub fn GetProcessWorkingSetSize(
        hProcess: HANDLE,
        lpMinimumWorkingSetSize: PSIZE_T,
        lpMaximumWorkingSetSize: PSIZE_T,
    ) -> BOOL;
    pub fn SetProcessWorkingSetSize(
        hProcess: HANDLE,
        dwMinimumWorkingSetSize: SIZE_T,
        dwMaximumWorkingSetSize: SIZE_T,
    ) -> BOOL;
    pub fn FatalExit(
        ExitCode: c_int,
    );
    pub fn SetEnvironmentStringsA(
        NewEnvironment: LPCH,
    ) -> BOOL;
    pub fn SwitchToFiber(
        lpFiber: LPVOID,
    );
    pub fn DeleteFiber(
        lpFiber: LPVOID,
    );
    pub fn ConvertFiberToThread() -> BOOL;
    pub fn CreateFiberEx(
        dwStackCommitSize: SIZE_T,
        dwStackReserveSize: SIZE_T,
        dwFlags: DWORD,
        lpStartAddress: LPFIBER_START_ROUTINE,
        lpParameter: LPVOID,
    ) -> LPVOID;
    pub fn ConvertThreadToFiberEx(
        lpParameter: LPVOID,
        dwFlags: DWORD,
    ) -> LPVOID;
    pub fn CreateFiber(
        dwStackSize: SIZE_T,
        lpStartAddress: LPFIBER_START_ROUTINE,
        lpParameter: LPVOID,
    ) -> LPVOID;
    pub fn ConvertThreadToFiber(
        lpParameter: LPVOID,
    ) -> LPVOID;
}
pub type PUMS_CONTEXT = *mut c_void;
pub type PUMS_COMPLETION_LIST = *mut c_void;
pub type UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS;
pub type PUMS_THREAD_INFO_CLASS = *mut UMS_THREAD_INFO_CLASS;
pub type PUMS_SCHEDULER_ENTRY_POINT = PRTL_UMS_SCHEDULER_ENTRY_POINT;
STRUCT!{struct UMS_SCHEDULER_STARTUP_INFO {
    UmsVersion: ULONG,
    CompletionList: PUMS_COMPLETION_LIST,
    SchedulerProc: PUMS_SCHEDULER_ENTRY_POINT,
    SchedulerParam: PVOID,
}}
pub type PUMS_SCHEDULER_STARTUP_INFO = *mut UMS_SCHEDULER_STARTUP_INFO;
STRUCT!{struct UMS_SYSTEM_THREAD_INFORMATION {
    UmsVersion: ULONG,
    ThreadUmsFlags: ULONG,
}}
BITFIELD!{UMS_SYSTEM_THREAD_INFORMATION ThreadUmsFlags: ULONG [
    IsUmsSchedulerThread set_IsUmsSchedulerThread[0..1],
    IsUmsWorkerThread set_IsUmsWorkerThread[1..2],
]}
pub type PUMS_SYSTEM_THREAD_INFORMATION = *mut UMS_SYSTEM_THREAD_INFORMATION;
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn CreateUmsCompletionList(
        UmsCompletionList: *mut PUMS_COMPLETION_LIST,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn DequeueUmsCompletionListItems(
        UmsCompletionList: PUMS_COMPLETION_LIST,
        WaitTimeOut: DWORD,
        UmsThreadList: *mut PUMS_CONTEXT,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn GetUmsCompletionListEvent(
        UmsCompletionList: PUMS_COMPLETION_LIST,
        UmsCompletionEvent: PHANDLE,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn ExecuteUmsThread(
        UmsThread: PUMS_CONTEXT,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn UmsThreadYield(
        SchedulerParam: PVOID,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn DeleteUmsCompletionList(
        UmsCompletionList: PUMS_COMPLETION_LIST,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn GetCurrentUmsThread() -> PUMS_CONTEXT;
    #[cfg(target_pointer_width = "64")]
    pub fn GetNextUmsListItem(
        UmsContext: PUMS_CONTEXT,
    ) -> PUMS_CONTEXT;
    #[cfg(target_pointer_width = "64")]
    pub fn QueryUmsThreadInformation(
        UmsThread: PUMS_CONTEXT,
        UmsThreadInfoClass: UMS_THREAD_INFO_CLASS,
        UmsThreadInformation: PVOID,
        UmsThreadInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn SetUmsThreadInformation(
        UmsThread: PUMS_CONTEXT,
        UmsThreadInfoClass: UMS_THREAD_INFO_CLASS,
        UmsThreadInformation: PVOID,
        UmsThreadInformationLength: ULONG,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn DeleteUmsThreadContext(
        UmsThread: PUMS_CONTEXT,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn CreateUmsThreadContext(
        lpUmsThread: *mut PUMS_CONTEXT,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn EnterUmsSchedulingMode(
        SchedulerStartupInfo: PUMS_SCHEDULER_STARTUP_INFO,
    ) -> BOOL;
    #[cfg(target_pointer_width = "64")]
    pub fn GetUmsSystemThreadInformation(
        ThreadHandle: HANDLE,
        SystemThreadInfo: PUMS_SYSTEM_THREAD_INFORMATION,
    ) -> BOOL;
    pub fn SetThreadAffinityMask(
        hThread: HANDLE,
        dwThreadAffinityMask: DWORD_PTR,
    ) -> DWORD_PTR;
    pub fn SetProcessDEPPolicy(
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn GetProcessDEPPolicy(
        hProcess: HANDLE,
        lpFlags: LPDWORD,
        lpPermanent: PBOOL,
    ) -> BOOL;
    pub fn RequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> BOOL;
    pub fn IsSystemResumeAutomatic() -> BOOL;
    pub fn GetThreadSelectorEntry(
        hThread: HANDLE,
        dwSelector: DWORD,
        lpSelectorEntry: LPLDT_ENTRY,
    ) -> BOOL;
    pub fn SetThreadExecutionState(
        esFlags: EXECUTION_STATE,
    ) -> EXECUTION_STATE;
    pub fn PowerCreateRequest(
        Context: PREASON_CONTEXT,
    ) -> HANDLE;
    pub fn PowerSetRequest(
        PowerRequest: HANDLE,
        RequestType: POWER_REQUEST_TYPE,
    ) -> BOOL;
    pub fn PowerClearRequest(
        PowerRequest: HANDLE,
        RequestType: POWER_REQUEST_TYPE,
    ) -> BOOL;
    pub fn RestoreLastError(
        dwErrCode: DWORD,
    );
}
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: UCHAR = 0x1;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: UCHAR = 0x2;
extern "system" {
    pub fn SetFileCompletionNotificationModes(
        FileHandle: HANDLE,
        Flags: UCHAR,
    ) -> BOOL;
}
pub const SEM_FAILCRITICALERRORS: UINT = 0x0001;
pub const SEM_NOGPFAULTERRORBOX: UINT = 0x0002;
pub const SEM_NOALIGNMENTFAULTEXCEPT: UINT = 0x0004;
pub const SEM_NOOPENFILEERRORBOX: UINT = 0x8000;
extern "system" {
    pub fn Wow64GetThreadContext(
        hThread: HANDLE,
        lpContext: PWOW64_CONTEXT,
    ) -> BOOL;
    pub fn Wow64SetThreadContext(
        hThread: HANDLE,
        lpContext: *const WOW64_CONTEXT,
    ) -> BOOL;
    pub fn Wow64GetThreadSelectorEntry(
        hThread: HANDLE,
        dwSelector: DWORD,
        lpSelectorEntry: PWOW64_LDT_ENTRY,
    ) -> BOOL;
    pub fn Wow64SuspendThread(
        hThread: HANDLE,
    ) -> DWORD;
    pub fn DebugSetProcessKillOnExit(
        KillOnExit: BOOL,
    ) -> BOOL;
    pub fn DebugBreakProcess(
        Process: HANDLE,
    ) -> BOOL;
    pub fn PulseEvent(
        hEvent: HANDLE,
    ) -> BOOL;
    pub fn GlobalDeleteAtom(
        nAtom: ATOM,
    ) -> ATOM;
    pub fn InitAtomTable(
        nSize: DWORD,
    ) -> BOOL;
    pub fn DeleteAtom(
        nAtom: ATOM,
    ) -> ATOM;
    pub fn SetHandleCount(
        uNumber: UINT,
    ) -> UINT;
    pub fn RequestDeviceWakeup(
        hDevice: HANDLE,
    ) -> BOOL;
    pub fn CancelDeviceWakeupRequest(
        hDevice: HANDLE,
    ) -> BOOL;
    pub fn GetDevicePowerState(
        hDevice: HANDLE,
        pfOn: *mut BOOL,
    ) -> BOOL;
    pub fn SetMessageWaitingIndicator(
        hMsgIndicator: HANDLE,
        ulMsgCount: ULONG,
    ) -> BOOL;
    pub fn SetFileShortNameA(
        hFile: HANDLE,
        lpShortName: LPCSTR,
    ) -> BOOL;
    pub fn SetFileShortNameW(
        hFile: HANDLE,
        lpShortName: LPCWSTR,
    ) -> BOOL;
}
pub const HANDLE_FLAG_INHERIT: DWORD = 0x00000001;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: DWORD = 0x00000002;
extern "system" {
    pub fn LoadModule(
        lpModuleName: LPCSTR,
        lpParameterBlock: LPVOID,
    ) -> DWORD;
    pub fn WinExec(
        lpCmdLine: LPCSTR,
        uCmdShow: UINT,
    ) -> UINT;
    // ClearCommBreak
    // ClearCommError
    // SetupComm
    // EscapeCommFunction
    // GetCommConfig
    // GetCommMask
    // GetCommProperties
    // GetCommModemStatus
    // GetCommState
    // GetCommTimeouts
    // PurgeComm
    // SetCommBreak
    // SetCommConfig
    // SetCommMask
    // SetCommState
    // SetCommTimeouts
    // TransmitCommChar
    // WaitCommEvent
    pub fn SetTapePosition(
        hDevice: HANDLE,
        dwPositionMethod: DWORD,
        dwPartition: DWORD,
        dwOffsetLow: DWORD,
        dwOffsetHigh: DWORD,
        bImmediate: BOOL,
    ) -> DWORD;
    pub fn GetTapePosition(
        hDevice: HANDLE,
        dwPositionType: DWORD,
        lpdwPartition: LPDWORD,
        lpdwOffsetLow: LPDWORD,
        lpdwOffsetHigh: LPDWORD,
    ) -> DWORD;
    pub fn PrepareTape(
        hDevice: HANDLE,
        dwOperation: DWORD,
        bImmediate: BOOL,
    ) -> DWORD;
    pub fn EraseTape(
        hDevice: HANDLE,
        dwEraseType: DWORD,
        bImmediate: BOOL,
    ) -> DWORD;
    pub fn CreateTapePartition(
        hDevice: HANDLE,
        dwPartitionMethod: DWORD,
        dwCount: DWORD,
        dwSize: DWORD,
    ) -> DWORD;
    pub fn WriteTapemark(
        hDevice: HANDLE,
        dwTapemarkType: DWORD,
        dwTapemarkCount: DWORD,
        bImmediate: BOOL,
    ) -> DWORD;
    pub fn GetTapeStatus(
        hDevice: HANDLE,
    ) -> DWORD;
    pub fn GetTapeParameters(
        hDevice: HANDLE,
        dwOperation: DWORD,
        lpdwSize: LPDWORD,
        lpTapeInformation: LPVOID,
    ) -> DWORD;
    pub fn SetTapeParameters(
        hDevice: HANDLE,
        dwOperation: DWORD,
        lpTapeInformation: LPVOID,
    ) -> DWORD;
    pub fn MulDiv(
        nNumber: c_int,
        nNumerator: c_int,
        nDenominator: c_int,
    ) -> c_int;
}
ENUM!{enum DEP_SYSTEM_POLICY_TYPE {
    DEPPolicyAlwaysOff = 0,
    DEPPolicyAlwaysOn,
    DEPPolicyOptIn,
    DEPPolicyOptOut,
    DEPTotalPolicyCount,
}}
extern "system" {
    pub fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
    pub fn GetSystemRegistryQuota(
        pdwQuotaAllowed: PDWORD,
        pdwQuotaUsed: PDWORD,
    ) -> BOOL;
    pub fn FileTimeToDosDateTime(
        lpFileTime: *const FILETIME,
        lpFatDate: LPWORD,
        lpFatTime: LPWORD,
    ) -> BOOL;
    pub fn DosDateTimeToFileTime(
        wFatDate: WORD,
        wFatTime: WORD,
        lpFileTime: LPFILETIME,
    ) -> BOOL;
    pub fn FormatMessageA(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPSTR,
        nSize: DWORD,
        Arguments: *mut va_list,
    ) -> DWORD;
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: *mut va_list,
    ) -> DWORD;
}
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: DWORD = 0x000000FF;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
extern "system" {
    pub fn CreateMailslotA(
        lpName: LPCSTR,
        nMaxMessageSize: DWORD,
        lReadTimeout: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> HANDLE;
    pub fn CreateMailslotW(
        lpName: LPCWSTR,
        nMaxMessageSize: DWORD,
        lReadTimeout: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> HANDLE;
    pub fn GetMailslotInfo(
        hMailslot: HANDLE,
        lpMaxMessageSize: LPDWORD,
        lpNextSize: LPDWORD,
        lpMessageCount: LPDWORD,
        lpReadTimeout: LPDWORD,
    ) -> BOOL;
    pub fn SetMailslotInfo(
        hMailslot: HANDLE,
        lReadTimeout: DWORD,
    ) -> BOOL;
    // pub fn EncryptFileA();
    // pub fn EncryptFileW();
    // pub fn DecryptFileA();
    // pub fn DecryptFileW();
    // pub fn FileEncryptionStatusA();
    // pub fn FileEncryptionStatusW();
    // pub fn OpenEncryptedFileRawA();
    // pub fn OpenEncryptedFileRawW();
    // pub fn ReadEncryptedFileRaw();
    // pub fn WriteEncryptedFileRaw();
    // pub fn CloseEncryptedFileRaw();
    pub fn lstrcmpA(
        lpString1: LPCSTR,
        lpString2: LPCSTR,
    ) -> c_int;
    pub fn lstrcmpW(
        lpString1: LPCWSTR,
        lpString2: LPCWSTR,
    ) -> c_int;
    pub fn lstrcmpiA(
        lpString1: LPCSTR,
        lpString2: LPCSTR,
    ) -> c_int;
    pub fn lstrcmpiW(
        lpString1: LPCWSTR,
        lpString2: LPCWSTR,
    ) -> c_int;
    pub fn lstrcpynA(
        lpString1: LPSTR,
        lpString2: LPCSTR,
        iMaxLength: c_int,
    ) -> LPSTR;
    pub fn lstrcpynW(
        lpString1: LPWSTR,
        lpString2: LPCWSTR,
        iMaxLength: c_int,
    ) -> LPWSTR;
    pub fn lstrcpyA(
        lpString1: LPSTR,
        lpString2: LPCSTR,
    ) -> LPSTR;
    pub fn lstrcpyW(
        lpString1: LPWSTR,
        lpString2: LPCWSTR,
    ) -> LPWSTR;
    pub fn lstrcatA(
        lpString1: LPSTR,
        lpString2: LPCSTR,
    ) -> LPSTR;
    pub fn lstrcatW(
        lpString1: LPWSTR,
        lpString2: LPCWSTR,
    ) -> LPWSTR;
    pub fn lstrlenA(
        lpString: LPCSTR,
    ) -> c_int;
    pub fn lstrlenW(
        lpString: LPCWSTR,
    ) -> c_int;
    pub fn OpenFile(
        lpFileName: LPCSTR,
        lpReOpenBuff: LPOFSTRUCT,
        uStyle: UINT,
    ) -> HFILE;
    pub fn _lopen(
        lpPathName: LPCSTR,
        iReadWrite: c_int,
    ) -> HFILE;
    pub fn _lcreat(
        lpPathName: LPCSTR,
        iAttrubute: c_int,
    ) -> HFILE;
    pub fn _lread(
        hFile: HFILE,
        lpBuffer: LPVOID,
        uBytes: UINT,
    ) -> UINT;
    pub fn _lwrite(
        hFile: HFILE,
        lpBuffer: LPCCH,
        uBytes: UINT,
    ) -> UINT;
    pub fn _hread(
        hFile: HFILE,
        lpBuffer: LPVOID,
        lBytes: c_long,
    ) -> c_long;
    pub fn _hwrite(
        hFile: HFILE,
        lpBuffer: LPCCH,
        lBytes: c_long,
    ) -> c_long;
    pub fn _lclose(
        hFile: HFILE,
    ) -> HFILE;
    pub fn _llseek(
        hFile: HFILE,
        lOffset: LONG,
        iOrigin: c_int,
    ) -> LONG;
    // pub fn IsTextUnicode();
    // pub fn SignalObjectAndWait();
    pub fn BackupRead(
        hFile: HANDLE,
        lpBuffer: LPBYTE,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: LPDWORD,
        bAbort: BOOL,
        bProcessSecurity: BOOL,
        lpContext: *mut LPVOID,
    ) -> BOOL;
    pub fn BackupSeek(
        hFile: HANDLE,
        dwLowBytesToSeek: DWORD,
        dwHighBytesToSeek: DWORD,
        lpdwLowByteSeeked: LPDWORD,
        lpdwHighByteSeeked: LPDWORD,
        lpContext: *mut LPVOID,
    ) -> BOOL;
    pub fn BackupWrite(
        hFile: HANDLE,
        lpBuffer: LPBYTE,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD,
        bAbort: BOOL,
        bProcessSecurity: BOOL,
        lpContext: *mut LPVOID,
    ) -> BOOL;
}
//2886
pub const STARTF_USESHOWWINDOW: DWORD = 0x00000001;
pub const STARTF_USESIZE: DWORD = 0x00000002;
pub const STARTF_USEPOSITION: DWORD = 0x00000004;
pub const STARTF_USECOUNTCHARS: DWORD = 0x00000008;
pub const STARTF_USEFILLATTRIBUTE: DWORD = 0x00000010;
pub const STARTF_RUNFULLSCREEN: DWORD = 0x00000020;
pub const STARTF_FORCEONFEEDBACK: DWORD = 0x00000040;
pub const STARTF_FORCEOFFFEEDBACK: DWORD = 0x00000080;
pub const STARTF_USESTDHANDLES: DWORD = 0x00000100;
pub const STARTF_USEHOTKEY: DWORD = 0x00000200;
pub const STARTF_TITLEISLINKNAME: DWORD = 0x00000800;
pub const STARTF_TITLEISAPPID: DWORD = 0x00001000;
pub const STARTF_PREVENTPINNING: DWORD = 0x00002000;
pub const STARTF_UNTRUSTEDSOURCE: DWORD = 0x00008000;
STRUCT!{struct STARTUPINFOEXA {
    StartupInfo: STARTUPINFOA,
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}}
pub type LPSTARTUPINFOEXA = *mut STARTUPINFOEXA;
STRUCT!{struct STARTUPINFOEXW {
    StartupInfo: STARTUPINFOW,
    lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}}
pub type LPSTARTUPINFOEXW = *mut STARTUPINFOEXW;
extern "system" {
    pub fn OpenMutexA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateSemaphoreA(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
        lInitialCount: LONG,
        lMaximumCount: LONG,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn OpenSemaphoreA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateWaitableTimerA(
        lpTimerAttributes: LPSECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        lpTimerName: LPCSTR,
    ) -> HANDLE;
    pub fn OpenWaitableTimerA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpTimerName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateSemaphoreExA(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
        lInitialCount: LONG,
        lMaximumCount: LONG,
        lpName: LPCSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateWaitableTimerExA(
        lpTimerAttributes: LPSECURITY_ATTRIBUTES,
        lpTimerName: LPCSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateFileMappingA(
        hFile: HANDLE,
        lpAttributes: LPSECURITY_ATTRIBUTES,
        flProtect: DWORD,
        dwMaximumSizeHigh: DWORD,
        dwMaximumSizeLow: DWORD,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateFileMappingNumaA(
        hFile: HANDLE,
        lpFileMappingAttributes: LPSECURITY_ATTRIBUTES,
        flProtect: DWORD,
        dwMaximumSizeHigh: DWORD,
        dwMaximumSizeLow: DWORD,
        lpName: LPCSTR,
        nndPreferred: DWORD,
    ) -> HANDLE;
    pub fn OpenFileMappingA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn GetLogicalDriveStringsA(
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
    ) -> DWORD;
    pub fn LoadPackagedLibrary(
        lpwLibFileName: LPCWSTR,
        Reserved: DWORD,
    ) -> HMODULE;
    pub fn QueryFullProcessImageNameA(
        hProcess: HANDLE,
        dwFlags: DWORD,
        lpExeName: LPSTR,
        lpdwSize: PDWORD,
    ) -> BOOL;
    pub fn QueryFullProcessImageNameW(
        hProcess: HANDLE,
        dwFlags: DWORD,
        lpExeName: LPWSTR,
        lpdwSize: PDWORD,
    ) -> BOOL;
}
//3233
extern "system" {
    pub fn GetStartupInfoA(
        lpStartupInfo: LPSTARTUPINFOA,
    );
    pub fn GetFirmwareEnvironmentVariableA(
        lpName: LPCSTR,
        lpGuid: LPCSTR,
        pBuffer: PVOID,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetFirmwareEnvironmentVariableW(
        lpName: LPCWSTR,
        lpGuid: LPCWSTR,
        pBuffer: PVOID,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetFirmwareEnvironmentVariableExA(
        lpName: LPCSTR,
        lpGuid: LPCSTR,
        pBuffer: PVOID,
        nSize: DWORD,
        pdwAttribubutes: PDWORD,
    ) -> DWORD;
    pub fn GetFirmwareEnvironmentVariableExW(
        lpName: LPCWSTR,
        lpGuid: LPCWSTR,
        pBuffer: PVOID,
        nSize: DWORD,
        pdwAttribubutes: PDWORD,
    ) -> DWORD;
    pub fn SetFirmwareEnvironmentVariableA(
        lpName: LPCSTR,
        lpGuid: LPCSTR,
        pValue: PVOID,
        nSize: DWORD,
    ) -> BOOL;
    pub fn SetFirmwareEnvironmentVariableW(
        lpName: LPCWSTR,
        lpGuid: LPCWSTR,
        pValue: PVOID,
        nSize: DWORD,
    ) -> BOOL;
    pub fn SetFirmwareEnvironmentVariableExA(
        lpName: LPCSTR,
        lpGuid: LPCSTR,
        pValue: PVOID,
        nSize: DWORD,
        dwAttributes: DWORD,
    ) -> BOOL;
    pub fn SetFirmwareEnvironmentVariableExW(
        lpName: LPCWSTR,
        lpGuid: LPCWSTR,
        pValue: PVOID,
        nSize: DWORD,
        dwAttributes: DWORD,
    ) -> BOOL;
    pub fn GetFirmwareType(
        FirmwareType: PFIRMWARE_TYPE,
    ) -> BOOL;
    pub fn IsNativeVhdBoot(
        NativeVhdBoot: PBOOL,
    ) -> BOOL;
    pub fn FindResourceA(
        hModule: HMODULE,
        lpName: LPCSTR,
        lpType: LPCSTR,
    ) -> HRSRC;
    pub fn FindResourceExA(
        hModule: HMODULE,
        lpName: LPCSTR,
        lpType: LPCSTR,
        wLanguage: WORD,
    ) -> HRSRC;
    pub fn EnumResourceTypesA(
        hModule: HMODULE,
        lpEnumFunc: ENUMRESTYPEPROCA,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumResourceTypesW(
        hModule: HMODULE,
        lpEnumFunc: ENUMRESTYPEPROCW,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumResourceNamesA(
        hModule: HMODULE,
        lpType: LPCSTR,
        lpEnumFunc: ENUMRESNAMEPROCA,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumResourceLanguagesA(
        hModule: HMODULE,
        lpType: LPCSTR,
        lpName: LPCSTR,
        lpEnumFunc: ENUMRESLANGPROCA,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn EnumResourceLanguagesW(
        hModule: HMODULE,
        lpType: LPCWSTR,
        lpName: LPCWSTR,
        lpEnumFunc: ENUMRESLANGPROCW,
        lParam: LONG_PTR,
    ) -> BOOL;
    pub fn BeginUpdateResourceA(
        pFileName: LPCSTR,
        bDeleteExistingResources: BOOL,
    ) -> HANDLE;
    pub fn BeginUpdateResourceW(
        pFileName: LPCWSTR,
        bDeleteExistingResources: BOOL,
    ) -> HANDLE;
    pub fn UpdateResourceA(
        hUpdate: HANDLE,
        lpType: LPCSTR,
        lpName: LPCSTR,
        wLanguage: WORD,
        lpData: LPVOID,
        cb: DWORD,
    ) -> BOOL;
    pub fn UpdateResourceW(
        hUpdate: HANDLE,
        lpType: LPCWSTR,
        lpName: LPCWSTR,
        wLanguage: WORD,
        lpData: LPVOID,
        cb: DWORD,
    ) -> BOOL;
    pub fn EndUpdateResourceA(
        hUpdate: HANDLE,
        fDiscard: BOOL,
    ) -> BOOL;
    pub fn EndUpdateResourceW(
        hUpdate: HANDLE,
        fDiscard: BOOL,
    ) -> BOOL;
    pub fn GlobalAddAtomA(
        lpString: LPCSTR,
    ) -> ATOM;
    pub fn GlobalAddAtomW(
        lpString: LPCWSTR,
    ) -> ATOM;
    pub fn GlobalAddAtomExA(
        lpString: LPCSTR,
        Flags: DWORD,
    ) -> ATOM;
    pub fn GlobalAddAtomExW(
        lpString: LPCWSTR,
        Flags: DWORD,
    ) -> ATOM;
    pub fn GlobalFindAtomA(
        lpString: LPCSTR,
    ) -> ATOM;
    pub fn GlobalFindAtomW(
        lpString: LPCWSTR,
    ) -> ATOM;
    pub fn GlobalGetAtomNameA(
        nAtom: ATOM,
        lpBuffer: LPSTR,
        nSize: c_int,
    ) -> UINT;
    pub fn GlobalGetAtomNameW(
        nAtom: ATOM,
        lpBuffer: LPWSTR,
        nSize: c_int,
    ) -> UINT;
    pub fn AddAtomA(
        lpString: LPCSTR,
    ) -> ATOM;
    pub fn AddAtomW(
        lpString: LPCWSTR,
    ) -> ATOM;
    pub fn FindAtomA(
        lpString: LPCSTR,
    ) -> ATOM;
    pub fn FindAtomW(
        lpString: LPCWSTR,
    ) -> ATOM;
    pub fn GetAtomNameA(
        nAtom: ATOM,
        lpBuffer: LPSTR,
        nSize: c_int,
    ) -> UINT;
    pub fn GetAtomNameW(
        nAtom: ATOM,
        lpBuffer: LPWSTR,
        nSize: c_int,
    ) -> UINT;
    pub fn GetProfileIntA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        nDefault: INT,
    ) -> UINT;
    pub fn GetProfileIntW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        nDefault: INT,
    ) -> UINT;
    pub fn GetProfileStringA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        lpDefault: LPCSTR,
        lpReturnedString: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetProfileStringW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        lpDefault: LPCWSTR,
        lpReturnedString: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn WriteProfileStringA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        lpString: LPCSTR,
    ) -> BOOL;
    pub fn WriteProfileStringW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        lpString: LPCWSTR,
    ) -> BOOL;
    pub fn GetProfileSectionA(
        lpAppName: LPCSTR,
        lpReturnedString: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetProfileSectionW(
        lpAppName: LPCWSTR,
        lpReturnedString: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn WriteProfileSectionA(
        lpAppName: LPCSTR,
        lpString: LPCSTR,
    ) -> BOOL;
    pub fn WriteProfileSectionW(
        lpAppName: LPCWSTR,
        lpString: LPCWSTR,
    ) -> BOOL;
    pub fn GetPrivateProfileIntA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        nDefault: INT,
        lpFileName: LPCSTR,
    ) -> UINT;
    pub fn GetPrivateProfileIntW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        nDefault: INT,
        lpFileName: LPCWSTR,
    ) -> UINT;
    pub fn GetPrivateProfileStringA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        lpDefault: LPCSTR,
        lpReturnedString: LPSTR,
        nSize: DWORD,
        lpFileName: LPCSTR,
    ) -> DWORD;
    pub fn GetPrivateProfileStringW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        lpDefault: LPCWSTR,
        lpReturnedString: LPWSTR,
        nSize: DWORD,
        lpFileName: LPCWSTR,
    ) -> DWORD;
    pub fn WritePrivateProfileStringA(
        lpAppName: LPCSTR,
        lpKeyName: LPCSTR,
        lpString: LPCSTR,
        lpFileName: LPCSTR,
    ) -> BOOL;
    pub fn WritePrivateProfileStringW(
        lpAppName: LPCWSTR,
        lpKeyName: LPCWSTR,
        lpString: LPCWSTR,
        lpFileName: LPCWSTR,
    ) -> BOOL;
    pub fn GetPrivateProfileSectionA(
        lpAppName: LPCSTR,
        lpReturnedString: LPSTR,
        nSize: DWORD,
        lpFileName: LPCSTR,
    ) -> DWORD;
    pub fn GetPrivateProfileSectionW(
        lpAppName: LPCWSTR,
        lpReturnedString: LPWSTR,
        nSize: DWORD,
        lpFileName: LPCWSTR,
    ) -> DWORD;
    pub fn WritePrivateProfileSectionA(
        lpAppName: LPCSTR,
        lpString: LPCSTR,
        lpFileName: LPCSTR,
    ) -> BOOL;
    pub fn WritePrivateProfileSectionW(
        lpAppName: LPCWSTR,
        lpString: LPCWSTR,
        lpFileName: LPCWSTR,
    ) -> BOOL;
    pub fn GetPrivateProfileSectionNamesA(
        lpszReturnBuffer: LPSTR,
        nSize: DWORD,
        lpFileName: LPCSTR,
    ) -> DWORD;
    pub fn GetPrivateProfileSectionNamesW(
        lpszReturnBuffer: LPWSTR,
        nSize: DWORD,
        lpFileName: LPCWSTR,
    ) -> DWORD;
    pub fn GetPrivateProfileStructA(
        lpszSection: LPCSTR,
        lpszKey: LPCSTR,
        lpStruct: LPVOID,
        uSizeStruct: UINT,
        szFile: LPCSTR,
    ) -> BOOL;
    pub fn GetPrivateProfileStructW(
        lpszSection: LPCWSTR,
        lpszKey: LPCWSTR,
        lpStruct: LPVOID,
        uSizeStruct: UINT,
        szFile: LPCWSTR,
    ) -> BOOL;
    pub fn WritePrivateProfileStructA(
        lpszSection: LPCSTR,
        lpszKey: LPCSTR,
        lpStruct: LPVOID,
        uSizeStruct: UINT,
        szFile: LPCSTR,
    ) -> BOOL;
    pub fn WritePrivateProfileStructW(
        lpszSection: LPCWSTR,
        lpszKey: LPCWSTR,
        lpStruct: LPVOID,
        uSizeStruct: UINT,
        szFile: LPCWSTR,
    ) -> BOOL;
    pub fn Wow64EnableWow64FsRedirection(
        Wow64FsEnableRedirection: BOOLEAN,
    ) -> BOOLEAN;
    pub fn SetDllDirectoryA(
        lpPathName: LPCSTR,
    ) -> BOOL;
    pub fn SetDllDirectoryW(
        lpPathName: LPCWSTR,
    ) -> BOOL;
    pub fn GetDllDirectoryA(
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
    ) -> DWORD;
    pub fn GetDllDirectoryW(
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
    ) -> DWORD;
    pub fn SetSearchPathMode(
        Flags: DWORD,
    ) -> BOOL;
    pub fn CreateDirectoryExA(
        lpTemplateDirectory: LPCSTR,
        lpNewDirectory: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateDirectoryExW(
        lpTemplateDirectory: LPCWSTR,
        lpNewDirectory: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateDirectoryTransactedA(
        lpTemplateDirectory: LPCSTR,
        lpNewDirectory: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn CreateDirectoryTransactedW(
        lpTemplateDirectory: LPCWSTR,
        lpNewDirectory: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn RemoveDirectoryTransactedA(
        lpPathName: LPCSTR,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn RemoveDirectoryTransactedW(
        lpPathName: LPCWSTR,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn GetFullPathNameTransactedA(
        lpFileName: LPCSTR,
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
        lpFilePart: *mut LPSTR,
        hTransaction: HANDLE,
    ) -> DWORD;
    pub fn GetFullPathNameTransactedW(
        lpFileName: LPCWSTR,
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
        lpFilePart: *mut LPWSTR,
        hTransaction: HANDLE,
    );
    pub fn DefineDosDeviceA(
        dwFlags: DWORD,
        lpDeviceName: LPCSTR,
        lpTargetPath: LPCSTR,
    ) -> BOOL;
    pub fn QueryDosDeviceA(
        lpDeviceName: LPCSTR,
        lpTargetPath: LPSTR,
        ucchMax: DWORD,
    ) -> DWORD;
    pub fn CreateFileTransactedA(
        lpFileName: LPCSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
        hTransaction: HANDLE,
        pusMiniVersion: PUSHORT,
        lpExtendedParameter: PVOID,
    ) -> HANDLE;
    pub fn CreateFileTransactedW(
        lpFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
        hTransaction: HANDLE,
        pusMiniVersion: PUSHORT,
        lpExtendedParameter: PVOID,
    ) -> HANDLE;
    pub fn ReOpenFile(
        hOriginalFile: HANDLE,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        dwFlags: DWORD,
    ) -> HANDLE;
    pub fn SetFileAttributesTransactedA(
        lpFileName: LPCSTR,
        dwFileAttributes: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn SetFileAttributesTransactedW(
        lpFileName: LPCWSTR,
        dwFileAttributes: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn GetFileAttributesTransactedA(
        lpFileName: LPCSTR,
        fInfoLevelId: GET_FILEEX_INFO_LEVELS,
        lpFileInformation: LPVOID,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn GetFileAttributesTransactedW(
        lpFileName: LPCWSTR,
        fInfoLevelId: GET_FILEEX_INFO_LEVELS,
        lpFileInformation: LPVOID,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn GetCompressedFileSizeTransactedA(
        lpFileName: LPCSTR,
        lpFileSizeHigh: LPDWORD,
        hTransaction: HANDLE,
    ) -> DWORD;
    pub fn GetCompressedFileSizeTransactedW(
        lpFileName: LPCWSTR,
        lpFileSizeHigh: LPDWORD,
        hTransaction: HANDLE,
    );
    pub fn DeleteFileTransactedA(
        lpFileName: LPCSTR,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn DeleteFileTransactedW(
        lpFileName: LPCWSTR,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn CheckNameLegalDOS8Dot3A(
        lpName: LPCSTR,
        lpOemName: LPSTR,
        OemNameSize: DWORD,
        pbNameContainsSpaces: PBOOL,
        pbNameLegal: PBOOL,
    ) -> BOOL;
    pub fn CheckNameLegalDOS8Dot3W(
        lpName: LPCWSTR,
        lpOemName: LPSTR,
        OemNameSize: DWORD,
        pbNameContainsSpaces: PBOOL,
        pbNameLegal: PBOOL,
    ) -> BOOL;
    pub fn FindFirstFileTransactedA(
        lpFileName: LPCSTR,
        fInfoLevelId: FINDEX_INFO_LEVELS,
        lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS,
        lpSearchFilter: LPVOID,
        dwAdditionalFlags: DWORD,
        hTransaction: HANDLE,
    ) -> HANDLE;
    pub fn FindFirstFileTransactedW(
        lpFileName: LPCWSTR,
        fInfoLevelId: FINDEX_INFO_LEVELS,
        lpFindFileData: LPVOID,
        fSearchOp: FINDEX_SEARCH_OPS,
        lpSearchFilter: LPVOID,
        dwAdditionalFlags: DWORD,
        hTransaction: HANDLE,
    ) -> HANDLE;
    pub fn CopyFileA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
        bFailIfExists: BOOL,
    ) -> BOOL;
    pub fn CopyFileW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        bFailIfExists: BOOL,
    ) -> BOOL;
}
FN!{stdcall LPPROGRESS_ROUTINE(
    TotalFileSize: LARGE_INTEGER,
    TotalBytesTransferred: LARGE_INTEGER,
    StreamSize: LARGE_INTEGER,
    StreamBytesTransferred: LARGE_INTEGER,
    dwStreamNumber: DWORD,
    dwCallbackReason: DWORD,
    hSourceFile: HANDLE,
    hDestinationFile: HANDLE,
    lpData: LPVOID,
) -> DWORD}
extern "system" {
    pub fn CopyFileExA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        pbCancel: LPBOOL,
        dwCopyFlags: DWORD,
    ) -> BOOL;
    pub fn CopyFileExW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        pbCancel: LPBOOL,
        dwCopyFlags: DWORD,
    ) -> BOOL;
    pub fn CopyFileTransactedA(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        pbCancel: LPBOOL,
        dwCopyFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn CopyFileTransactedW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        pbCancel: LPBOOL,
        dwCopyFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
}
ENUM!{enum COPYFILE2_MESSAGE_TYPE {
    COPYFILE2_CALLBACK_NONE = 0,
    COPYFILE2_CALLBACK_CHUNK_STARTED,
    COPYFILE2_CALLBACK_CHUNK_FINISHED,
    COPYFILE2_CALLBACK_STREAM_STARTED,
    COPYFILE2_CALLBACK_STREAM_FINISHED,
    COPYFILE2_CALLBACK_POLL_CONTINUE,
    COPYFILE2_CALLBACK_ERROR,
    COPYFILE2_CALLBACK_MAX,
}}
ENUM!{enum COPYFILE2_MESSAGE_ACTION {
    COPYFILE2_PROGRESS_CONTINUE = 0,
    COPYFILE2_PROGRESS_CANCEL,
    COPYFILE2_PROGRESS_STOP,
    COPYFILE2_PROGRESS_QUIET,
    COPYFILE2_PROGRESS_PAUSE,
}}
ENUM!{enum COPYFILE2_COPY_PHASE {
    COPYFILE2_PHASE_NONE = 0,
    COPYFILE2_PHASE_PREPARE_SOURCE,
    COPYFILE2_PHASE_PREPARE_DEST,
    COPYFILE2_PHASE_READ_SOURCE,
    COPYFILE2_PHASE_WRITE_DESTINATION,
    COPYFILE2_PHASE_SERVER_COPY,
    COPYFILE2_PHASE_NAMEGRAFT_COPY,
    COPYFILE2_PHASE_MAX,
}}
STRUCT!{struct COPYFILE2_MESSAGE_ChunkStarted {
    dwStreamNumber: DWORD,
    dwReserved: DWORD,
    hSourceFile: HANDLE,
    hDestinationFile: HANDLE,
    uliChunkNumber: ULARGE_INTEGER,
    uliChunkSize: ULARGE_INTEGER,
    uliStreamSize: ULARGE_INTEGER,
    uliTotalFileSize: ULARGE_INTEGER,
}}
STRUCT!{struct COPYFILE2_MESSAGE_ChunkFinished {
    dwStreamNumber: DWORD,
    dwFlags: DWORD,
    hSourceFile: HANDLE,
    hDestinationFile: HANDLE,
    uliChunkNumber: ULARGE_INTEGER,
    uliChunkSize: ULARGE_INTEGER,
    uliStreamSize: ULARGE_INTEGER,
    uliStreamBytesTransferred: ULARGE_INTEGER,
    uliTotalFileSize: ULARGE_INTEGER,
    uliTotalBytesTransferred: ULARGE_INTEGER,
}}
STRUCT!{struct COPYFILE2_MESSAGE_StreamStarted {
    dwStreamNumber: DWORD,
    dwReserved: DWORD,
    hSourceFile: HANDLE,
    hDestinationFile: HANDLE,
    uliStreamSize: ULARGE_INTEGER,
    uliTotalFileSize: ULARGE_INTEGER,
}}
STRUCT!{struct COPYFILE2_MESSAGE_StreamFinished {
    dwStreamNumber: DWORD,
    dwReserved: DWORD,
    hSourceFile: HANDLE,
    hDestinationFile: HANDLE,
    uliStreamSize: ULARGE_INTEGER,
    uliStreamBytesTransferred: ULARGE_INTEGER,
    uliTotalFileSize: ULARGE_INTEGER,
    uliTotalBytesTransferred: ULARGE_INTEGER,
}}
STRUCT!{struct COPYFILE2_MESSAGE_PollContinue {
    dwReserved: DWORD,
}}
STRUCT!{struct COPYFILE2_MESSAGE_Error {
    CopyPhase: COPYFILE2_COPY_PHASE,
    dwStreamNumber: DWORD,
    hrFailure: HRESULT,
    dwReserved: DWORD,
    uliChunkNumber: ULARGE_INTEGER,
    uliStreamSize: ULARGE_INTEGER,
    uliStreamBytesTransferred: ULARGE_INTEGER,
    uliTotalFileSize: ULARGE_INTEGER,
    uliTotalBytesTransferred: ULARGE_INTEGER,
}}
UNION!{union COPYFILE2_MESSAGE_Info {
    [u64; 8] [u64; 9],
    ChunkStarted ChunkStarted_mut: COPYFILE2_MESSAGE_ChunkStarted,
    ChunkFinished ChunkFinished_mut: COPYFILE2_MESSAGE_ChunkFinished,
    StreamStarted StreamStarted_mut: COPYFILE2_MESSAGE_StreamStarted,
    StreamFinished StreamFinished_mut: COPYFILE2_MESSAGE_StreamFinished,
    PollContinue PollContinue_mut: COPYFILE2_MESSAGE_PollContinue,
    Error Error_mut: COPYFILE2_MESSAGE_Error,
}}
STRUCT!{struct COPYFILE2_MESSAGE {
    Type: COPYFILE2_MESSAGE_TYPE,
    dwPadding: DWORD,
    Info: COPYFILE2_MESSAGE_Info,
}}
FN!{stdcall PCOPYFILE2_PROGRESS_ROUTINE(
    pMessage: *const COPYFILE2_MESSAGE,
    pvCallbackContext: PVOID,
) -> COPYFILE2_MESSAGE_ACTION}
STRUCT!{struct COPYFILE2_EXTENDED_PARAMETERS {
    dwSize: DWORD,
    dwCopyFlags: DWORD,
    pfCancel: *mut BOOL,
    pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pvCallbackContext: PVOID,
}}
extern "system" {
    pub fn CopyFile2(
        pwszExistingFileName: PCWSTR,
        pwszNewFileName: PCWSTR,
        pExtendedParameters: *mut COPYFILE2_EXTENDED_PARAMETERS,
    ) -> HRESULT;
    pub fn MoveFileA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
    ) -> BOOL;
    pub fn MoveFileW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
    ) -> BOOL;
    pub fn MoveFileExA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn MoveFileExW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn MoveFileWithProgressA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn MoveFileWithProgressW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn MoveFileTransactedA(
        lpExistingFileName: LPCSTR,
        lpNewFileName: LPCSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        dwFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn MoveFileTransactedW(
        lpExistingFileName: LPCWSTR,
        lpNewFileName: LPCWSTR,
        lpProgressRoutine: LPPROGRESS_ROUTINE,
        lpData: LPVOID,
        dwFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOL;
}
pub const MOVEFILE_REPLACE_EXISTING: DWORD = 0x00000001;
pub const MOVEFILE_COPY_ALLOWED: DWORD = 0x00000002;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: DWORD = 0x00000004;
pub const MOVEFILE_WRITE_THROUGH: DWORD = 0x00000008;
pub const MOVEFILE_CREATE_HARDLINK: DWORD = 0x00000010;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: DWORD = 0x00000020;
extern "system" {
    pub fn ReplaceFileA(
        lpReplacedFileName: LPCSTR,
        lpReplacementFileName: LPCSTR,
        lpBackupFileName: LPCSTR,
        dwReplaceFlags: DWORD,
        lpExclude: LPVOID,
        lpReserved: LPVOID,
    );
    pub fn ReplaceFileW(
        lpReplacedFileName: LPCWSTR,
        lpReplacementFileName: LPCWSTR,
        lpBackupFileName: LPCWSTR,
        dwReplaceFlags: DWORD,
        lpExclude: LPVOID,
        lpReserved: LPVOID,
    );
    pub fn CreateHardLinkA(
        lpFileName: LPCSTR,
        lpExistingFileName: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateHardLinkW(
        lpFileName: LPCWSTR,
        lpExistingFileName: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> BOOL;
    pub fn CreateHardLinkTransactedA(
        lpFileName: LPCSTR,
        lpExistingFileName: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hTransaction: HANDLE,
    ) -> BOOL;
    pub fn CreateHardLinkTransactedW(
        lpFileName: LPCWSTR,
        lpExistingFileName: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        hTransaction: HANDLE,
    );
    pub fn FindFirstStreamTransactedW(
        lpFileName: LPCWSTR,
        InfoLevel: STREAM_INFO_LEVELS,
        lpFindStreamData: LPVOID,
        dwFlags: DWORD,
        hTransaction: HANDLE,
    ) -> HANDLE;
    pub fn FindFirstFileNameTransactedW(
        lpFileName: LPCWSTR,
        dwFlags: DWORD,
        StringLength: LPDWORD,
        LinkName: PWSTR,
        hTransaction: HANDLE,
    ) -> HANDLE;
    pub fn CreateNamedPipeA(
        lpName: LPCSTR,
        dwOpenMode: DWORD,
        dwPipeMode: DWORD,
        nMaxInstances: DWORD,
        nOutBufferSize: DWORD,
        nInBufferSize: DWORD,
        nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> HANDLE;
    pub fn GetNamedPipeHandleStateA(
        hNamedPipe: HANDLE,
        lpState: LPDWORD,
        lpCurInstances: LPDWORD,
        lpMaxCollectionCount: LPDWORD,
        lpCollectDataTimeout: LPDWORD,
        lpUserName: LPSTR,
        nMaxUserNameSize: DWORD,
    ) -> BOOL;
    pub fn CallNamedPipeA(
        lpNamedPipeName: LPCSTR,
        lpInBuffer: LPVOID,
        nInBufferSize: DWORD,
        lpOutBuffer: LPVOID,
        nOutBufferSize: DWORD,
        lpBytesRead: LPDWORD,
        nTimeOut: DWORD,
    ) -> BOOL;
    pub fn WaitNamedPipeA(
        lpNamedPipeName: LPCSTR,
        nTimeOut: DWORD,
    ) -> BOOL;
    pub fn GetNamedPipeClientComputerNameA(
        Pipe: HANDLE,
        ClientComputerName: LPSTR,
        ClientComputerNameLength: ULONG,
    ) -> BOOL;
    pub fn GetNamedPipeClientProcessId(
        Pipe: HANDLE,
        ClientProcessId: PULONG,
    ) -> BOOL;
    pub fn GetNamedPipeClientSessionId(
        Pipe: HANDLE,
        ClientSessionId: PULONG,
    ) -> BOOL;
    pub fn GetNamedPipeServerProcessId(
        Pipe: HANDLE,
        ServerProcessId: PULONG,
    ) -> BOOL;
    pub fn GetNamedPipeServerSessionId(
        Pipe: HANDLE,
        ServerSessionId: PULONG,
    ) -> BOOL;
    pub fn SetVolumeLabelA(
        lpRootPathName: LPCSTR,
        lpVolumeName: LPCSTR,
    ) -> BOOL;
    pub fn SetVolumeLabelW(
        lpRootPathName: LPCWSTR,
        lpVolumeName: LPCWSTR,
    ) -> BOOL;
    pub fn SetFileBandwidthReservation(
        hFile: HANDLE,
        nPeriodMilliseconds: DWORD,
        nBytesPerPeriod: DWORD,
        bDiscardable: BOOL,
        lpTransferSize: LPDWORD,
        lpNumOutstandingRequests: LPDWORD,
    ) -> BOOL;
    pub fn GetFileBandwidthReservation(
        hFile: HANDLE,
        lpPeriodMilliseconds: LPDWORD,
        lpBytesPerPeriod: LPDWORD,
        pDiscardable: LPBOOL,
        lpTransferSize: LPDWORD,
        lpNumOutstandingRequests: LPDWORD,
    ) -> BOOL;
    // pub fn ClearEventLogA();
    // pub fn ClearEventLogW();
    // pub fn BackupEventLogA();
    // pub fn BackupEventLogW();
    // pub fn CloseEventLog();
    pub fn DeregisterEventSource(
        hEventLog: HANDLE,
    ) -> BOOL;
    // pub fn NotifyChangeEventLog();
    // pub fn GetNumberOfEventLogRecords();
    // pub fn GetOldestEventLogRecord();
    // pub fn OpenEventLogA();
    // pub fn OpenEventLogW();
    pub fn RegisterEventSourceA(
        lpUNCServerName: LPCSTR,
        lpSourceName: LPCSTR,
    ) -> HANDLE;
    pub fn RegisterEventSourceW(
        lpUNCServerName: LPCWSTR,
        lpSourceName: LPCWSTR,
    ) -> HANDLE;
    // pub fn OpenBackupEventLogA();
    // pub fn OpenBackupEventLogW();
    // pub fn ReadEventLogA();
    // pub fn ReadEventLogW();
    pub fn ReportEventA(
        hEventLog: HANDLE,
        wType: WORD,
        wCategory: WORD,
        dwEventID: DWORD,
        lpUserSid: PSID,
        wNumStrings: WORD,
        dwDataSize: DWORD,
        lpStrings: *mut LPCSTR,
        lpRawData: LPVOID,
    ) -> BOOL;
    pub fn ReportEventW(
        hEventLog: HANDLE,
        wType: WORD,
        wCategory: WORD,
        dwEventID: DWORD,
        lpUserSid: PSID,
        wNumStrings: WORD,
        dwDataSize: DWORD,
        lpStrings: *mut LPCWSTR,
        lpRawData: LPVOID,
    ) -> BOOL;
    // pub fn GetEventLogInformation();
    // pub fn OperationStart();
    // pub fn OperationEnd();
    // pub fn AccessCheckAndAuditAlarmA();
    // pub fn AccessCheckByTypeAndAuditAlarmA();
    // pub fn AccessCheckByTypeResultListAndAuditAlarmA();
    // pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA();
    // pub fn ObjectOpenAuditAlarmA();
    // pub fn ObjectPrivilegeAuditAlarmA();
    // pub fn ObjectCloseAuditAlarmA();
    // pub fn ObjectDeleteAuditAlarmA();
    // pub fn PrivilegedServiceAuditAlarmA();
    // pub fn AddConditionalAce();
    // pub fn SetFileSecurityA();
    // pub fn GetFileSecurityA();
    pub fn ReadDirectoryChangesW(
        hDirectory: HANDLE,
        lpBuffer: LPVOID,
        nBufferLength: DWORD,
        bWatchSubtree: BOOL,
        dwNotifyFilter: DWORD,
        lpBytesReturned: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
    ) -> BOOL;
    pub fn MapViewOfFileExNuma(
        hFileMappingObject: HANDLE,
        dwDesiredAccess: DWORD,
        dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD,
        dwNumberOfBytesToMap: SIZE_T,
        lpBaseAddress: LPVOID,
        nndPreferred: DWORD,
    ) -> LPVOID;
    pub fn IsBadReadPtr(
        lp: *const VOID,
        ucb: UINT_PTR,
    ) -> BOOL;
    pub fn IsBadWritePtr(
        lp: LPVOID,
        ucb: UINT_PTR,
    ) -> BOOL;
    pub fn IsBadHugeReadPtr(
        lp: *const VOID,
        ucb: UINT_PTR,
    ) -> BOOL;
    pub fn IsBadHugeWritePtr(
        lp: LPVOID,
        ucb: UINT_PTR,
    ) -> BOOL;
    pub fn IsBadCodePtr(
        lpfn: FARPROC,
    ) -> BOOL;
    pub fn IsBadStringPtrA(
        lpsz: LPCSTR,
        ucchMax: UINT_PTR,
    ) -> BOOL;
    pub fn IsBadStringPtrW(
        lpsz: LPCWSTR,
        ucchMax: UINT_PTR,
    ) -> BOOL;
    pub fn LookupAccountSidA(
        lpSystemName: LPCSTR,
        Sid: PSID,
        Name: LPSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;
    pub fn LookupAccountSidW(
        lpSystemName: LPCWSTR,
        Sid: PSID,
        Name: LPWSTR,
        cchName: LPDWORD,
        ReferencedDomainName: LPWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;
    pub fn LookupAccountNameA(
        lpSystemName: LPCSTR,
        lpAccountName: LPCSTR,
        Sid: PSID,
        cbSid: LPDWORD,
        ReferencedDomainName: LPCSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;
    pub fn LookupAccountNameW(
        lpSystemName: LPCWSTR,
        lpAccountName: LPCWSTR,
        Sid: PSID,
        cbSid: LPDWORD,
        ReferencedDomainName: LPCWSTR,
        cchReferencedDomainName: LPDWORD,
        peUse: PSID_NAME_USE,
    ) -> BOOL;
    // pub fn LookupAccountNameLocalA();
    // pub fn LookupAccountNameLocalW();
    // pub fn LookupAccountSidLocalA();
    // pub fn LookupAccountSidLocalW();
    pub fn LookupPrivilegeValueA(
        lpSystemName: LPCSTR,
        lpName: LPCSTR,
        lpLuid: PLUID,
    ) -> BOOL;
    pub fn LookupPrivilegeValueW(
        lpSystemName: LPCWSTR,
        lpName: LPCWSTR,
        lpLuid: PLUID,
    ) -> BOOL;
    pub fn LookupPrivilegeNameA(
        lpSystemName: LPCSTR,
        lpLuid: PLUID,
        lpName: LPSTR,
        cchName: LPDWORD,
    ) -> BOOL;
    pub fn LookupPrivilegeNameW(
        lpSystemName: LPCWSTR,
        lpLuid: PLUID,
        lpName: LPWSTR,
        cchName: LPDWORD,
    ) -> BOOL;
    // pub fn LookupPrivilegeDisplayNameA();
    // pub fn LookupPrivilegeDisplayNameW();
    pub fn BuildCommDCBA(
        lpDef: LPCSTR,
        lpDCB: LPDCB,
    ) -> BOOL;
    pub fn BuildCommDCBW(
        lpDef: LPCWSTR,
        lpDCB: LPDCB,
    ) -> BOOL;
    pub fn BuildCommDCBAndTimeoutsA(
        lpDef: LPCSTR,
        lpDCB: LPDCB,
        lpCommTimeouts: LPCOMMTIMEOUTS,
    ) -> BOOL;
    pub fn BuildCommDCBAndTimeoutsW(
        lpDef: LPCWSTR,
        lpDCB: LPDCB,
        lpCommTimeouts: LPCOMMTIMEOUTS,
    ) -> BOOL;
    pub fn CommConfigDialogA(
        lpszName: LPCSTR,
        hWnd: HWND,
        lpCC: LPCOMMCONFIG,
    ) -> BOOL;
    pub fn CommConfigDialogW(
        lpszName: LPCWSTR,
        hWnd: HWND,
        lpCC: LPCOMMCONFIG,
    ) -> BOOL;
    pub fn GetDefaultCommConfigA(
        lpszName: LPCSTR,
        lpCC: LPCOMMCONFIG,
        lpdwSize: LPDWORD,
    ) -> BOOL;
    pub fn GetDefaultCommConfigW(
        lpszName: LPCWSTR,
        lpCC: LPCOMMCONFIG,
        lpdwSize: LPDWORD,
    ) -> BOOL;
    pub fn SetDefaultCommConfigA(
        lpszName: LPCSTR,
        lpCC: LPCOMMCONFIG,
        dwSize: DWORD,
    ) -> BOOL;
    pub fn SetDefaultCommConfigW(
        lpszName: LPCWSTR,
        lpCC: LPCOMMCONFIG,
        dwSize: DWORD,
    ) -> BOOL;
    pub fn GetComputerNameA(
        lpBuffer: LPSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetComputerNameW(
        lpBuffer: LPWSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn DnsHostnameToComputerNameA(
        Hostname: LPCSTR,
        ComputerName: LPCSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn DnsHostnameToComputerNameW(
        Hostname: LPCWSTR,
        ComputerName: LPWSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetUserNameA(
        lpBuffer: LPSTR,
        pcbBuffer: LPDWORD,
    ) -> BOOL;
    pub fn GetUserNameW(
        lpBuffer: LPWSTR,
        pcbBuffer: LPDWORD,
    ) -> BOOL;
}
pub const LOGON32_LOGON_INTERACTIVE: DWORD = 2;
pub const LOGON32_LOGON_NETWORK: DWORD = 3;
pub const LOGON32_LOGON_BATCH: DWORD = 4;
pub const LOGON32_LOGON_SERVICE: DWORD = 5;
pub const LOGON32_LOGON_UNLOCK: DWORD = 7;
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: DWORD = 8;
pub const LOGON32_LOGON_NEW_CREDENTIALS: DWORD = 9;
pub const LOGON32_PROVIDER_DEFAULT: DWORD = 0;
pub const LOGON32_PROVIDER_WINNT35: DWORD = 1;
pub const LOGON32_PROVIDER_WINNT40: DWORD = 2;
pub const LOGON32_PROVIDER_WINNT50: DWORD = 3;
pub const LOGON32_PROVIDER_VIRTUAL: DWORD = 4;
extern "system" {
    pub fn LogonUserA(
        lpUsername: LPCSTR,
        lpDomain: LPCSTR,
        lpPassword: LPCSTR,
        dwLogonType: DWORD,
        dwLogonProvider: DWORD,
        phToken: PHANDLE,
    ) -> BOOL;
    pub fn LogonUserW(
        lpUsername: LPCWSTR,
        lpDomain: LPCWSTR,
        lpPassword: LPCWSTR,
        dwLogonType: DWORD,
        dwLogonProvider: DWORD,
        phToken: PHANDLE,
    ) -> BOOL;
    pub fn LogonUserExA(
        lpUsername: LPCSTR,
        lpDomain: LPCSTR,
        lpPassword: LPCSTR,
        dwLogonType: DWORD,
        dwLogonProvider: DWORD,
        phToken: PHANDLE,
        ppLogonSid: *mut PSID,
        ppProfileBuffer: *mut PVOID,
        pdwProfileLength: LPDWORD,
        pQuotaLimits: PQUOTA_LIMITS,
    ) -> BOOL;
    pub fn LogonUserExW(
        lpUsername: LPCWSTR,
        lpDomain: LPCWSTR,
        lpPassword: LPCWSTR,
        dwLogonType: DWORD,
        dwLogonProvider: DWORD,
        phToken: PHANDLE,
        ppLogonSid: *mut PSID,
        ppProfileBuffer: *mut PVOID,
        pdwProfileLength: LPDWORD,
        pQuotaLimits: PQUOTA_LIMITS,
    ) -> BOOL;
}
pub const LOGON_WITH_PROFILE: DWORD = 0x00000001;
pub const LOGON_NETCREDENTIALS_ONLY: DWORD = 0x00000002;
extern "system" {
    pub fn CreateProcessWithLogonW(
        lpUsername: LPCWSTR,
        lpDomain: LPCWSTR,
        lpPassword: LPCWSTR,
        dwLogonFlags: DWORD,
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;
    pub fn CreateProcessWithTokenW(
        hToken: HANDLE,
        dwLogonFlags: DWORD,
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;
    pub fn IsTokenUntrusted(
        TokenHandle: HANDLE,
    ) -> BOOL;
    pub fn RegisterWaitForSingleObject(
        phNewWaitObject: PHANDLE,
        hObject: HANDLE,
        Callback: WAITORTIMERCALLBACK,
        Context: PVOID,
        dwMilliseconds: ULONG,
        dwFlags: ULONG,
    ) -> BOOL;
    pub fn UnregisterWait(
        WaitHandle: HANDLE,
    ) -> BOOL;
    pub fn BindIoCompletionCallback(
        FileHandle: HANDLE,
        Function: LPOVERLAPPED_COMPLETION_ROUTINE,
        Flags: ULONG,
    ) -> BOOL;
    pub fn SetTimerQueueTimer(
        TimerQueue: HANDLE,
        Callback: WAITORTIMERCALLBACK,
        Parameter: PVOID,
        DueTime: DWORD,
        Period: DWORD,
        PreferIo: BOOL,
    ) -> HANDLE;
    pub fn CancelTimerQueueTimer(
        TimerQueue: HANDLE,
        Timer: HANDLE,
    ) -> BOOL;
    pub fn DeleteTimerQueue(
        TimerQueue: HANDLE,
    ) -> BOOL;
    // pub fn InitializeThreadpoolEnvironment();
    // pub fn SetThreadpoolCallbackPool();
    // pub fn SetThreadpoolCallbackCleanupGroup();
    // pub fn SetThreadpoolCallbackRunsLong();
    // pub fn SetThreadpoolCallbackLibrary();
    // pub fn SetThreadpoolCallbackPriority();
    // pub fn DestroyThreadpoolEnvironment();
    // pub fn SetThreadpoolCallbackPersistent();
    pub fn CreatePrivateNamespaceA(
        lpPrivateNamespaceAttributes: LPSECURITY_ATTRIBUTES,
        lpBoundaryDescriptor: LPVOID,
        lpAliasPrefix: LPCSTR,
    ) -> HANDLE;
    pub fn OpenPrivateNamespaceA(
        lpBoundaryDescriptor: LPVOID,
        lpAliasPrefix: LPCSTR,
    ) -> HANDLE;
    pub fn CreateBoundaryDescriptorA(
        Name: LPCSTR,
        Flags: ULONG,
    ) -> HANDLE;
    pub fn AddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut HANDLE,
        IntegrityLabel: PSID,
    ) -> BOOL;
}
pub const HW_PROFILE_GUIDLEN: usize = 39;
// MAX_PROFILE_LEN
pub const DOCKINFO_UNDOCKED: DWORD = 0x1;
pub const DOCKINFO_DOCKED: DWORD = 0x2;
pub const DOCKINFO_USER_SUPPLIED: DWORD = 0x4;
pub const DOCKINFO_USER_UNDOCKED: DWORD = DOCKINFO_USER_SUPPLIED | DOCKINFO_UNDOCKED;
pub const DOCKINFO_USER_DOCKED: DWORD = DOCKINFO_USER_SUPPLIED | DOCKINFO_DOCKED;
STRUCT!{struct HW_PROFILE_INFOA {
    dwDockInfo: DWORD,
    szHwProfileGuid: [CHAR; HW_PROFILE_GUIDLEN],
    szHwProfileName: [CHAR; MAX_PROFILE_LEN],
}}
pub type LPHW_PROFILE_INFOA = *mut HW_PROFILE_INFOA;
STRUCT!{struct HW_PROFILE_INFOW {
    dwDockInfo: DWORD,
    szHwProfileGuid: [WCHAR; HW_PROFILE_GUIDLEN],
    szHwProfileName: [WCHAR; MAX_PROFILE_LEN],
}}
pub type LPHW_PROFILE_INFOW = *mut HW_PROFILE_INFOW;
extern "system" {
    pub fn GetCurrentHwProfileA(
        lpHwProfileInfo: LPHW_PROFILE_INFOA,
    ) -> BOOL;
    pub fn GetCurrentHwProfileW(
        lpHwProfileInfo: LPHW_PROFILE_INFOW,
    ) -> BOOL;
    pub fn VerifyVersionInfoA(
        lpVersionInformation: LPOSVERSIONINFOEXA,
        dwTypeMask: DWORD,
        dwlConditionMask: DWORDLONG,
    ) -> BOOL;
    pub fn VerifyVersionInfoW(
        lpVersionInformation: LPOSVERSIONINFOEXW,
        dwTypeMask: DWORD,
        dwlConditionMask: DWORDLONG,
    ) -> BOOL;
}
STRUCT!{struct SYSTEM_POWER_STATUS {
    ACLineStatus: BYTE,
    BatteryFlag: BYTE,
    BatteryLifePercent: BYTE,
    Reserved1: BYTE,
    BatteryLifeTime: DWORD,
    BatteryFullLifeTime: DWORD,
}}
pub type LPSYSTEM_POWER_STATUS = *mut SYSTEM_POWER_STATUS;
extern "system" {
    pub fn GetSystemPowerStatus(
        lpSystemPowerStatus: LPSYSTEM_POWER_STATUS,
    ) -> BOOL;
    pub fn SetSystemPowerState(
        fSuspend: BOOL,
        fForce: BOOL,
    ) -> BOOL;
    pub fn MapUserPhysicalPagesScatter(
        VirtualAddresses: *mut PVOID,
        NumberOfPages: ULONG_PTR,
        PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn CreateJobObjectA(
        lpJobAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn OpenJobObjectA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateJobSet(
        NumJob: ULONG,
        UserJobSet: PJOB_SET_ARRAY,
        Flags: ULONG,
    ) -> BOOL;
    pub fn FindFirstVolumeA(
        lpszVolumeName: LPSTR,
        cchBufferLength: DWORD,
    ) -> HANDLE;
    pub fn FindNextVolumeA(
        hFindVolume: HANDLE,
        lpszVolumeName: LPSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn FindFirstVolumeMountPointA(
        lpszRootPathName: LPCSTR,
        lpszVolumeMountPoint: LPSTR,
        cchBufferLength: DWORD,
    ) -> HANDLE;
    pub fn FindFirstVolumeMountPointW(
        lpszRootPathName: LPCWSTR,
        lpszVolumeMountPoint: LPWSTR,
        cchBufferLength: DWORD,
    ) -> HANDLE;
    pub fn FindNextVolumeMountPointA(
        hFindVolumeMountPoint: HANDLE,
        lpszVolumeMountPoint: LPSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn FindNextVolumeMountPointW(
        hFindVolumeMountPoint: HANDLE,
        lpszVolumeMountPoint: LPWSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn FindVolumeMountPointClose(
        hFindVolumeMountPoint: HANDLE,
    ) -> BOOL;
    pub fn SetVolumeMountPointA(
        lpszVolumeMountPoint: LPCSTR,
        lpszVolumeName: LPCSTR,
    ) -> BOOL;
    pub fn SetVolumeMountPointW(
        lpszVolumeMountPoint: LPCWSTR,
        lpszVolumeName: LPCWSTR,
    ) -> BOOL;
    pub fn DeleteVolumeMountPointA(
        lpszVolumeMountPoint: LPCSTR,
    ) -> BOOL;
    pub fn GetVolumeNameForVolumeMountPointA(
        lpszVolumeMountPoint: LPCSTR,
        lpszVolumeName: LPSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn GetVolumePathNameA(
        lpszFileName: LPCSTR,
        lpszVolumePathName: LPSTR,
        cchBufferLength: DWORD,
    ) -> BOOL;
    pub fn GetVolumePathNamesForVolumeNameA(
        lpszVolumeName: LPCSTR,
        lpszVolumePathNames: LPCH,
        cchBufferLength: DWORD,
        lpcchReturnLength: PDWORD,
    ) -> BOOL;
}
// ACTCTX_FLAG_*
STRUCT!{struct ACTCTXA {
    cbSize: ULONG,
    dwFlags: DWORD,
    lpSource: LPCSTR,
    wProcessorArchitecture: USHORT,
    wLangId: LANGID,
    lpAssemblyDirectory: LPCSTR,
    lpResourceName: LPCSTR,
    lpApplicationName: LPCSTR,
    hModule: HMODULE,
}}
pub type PACTCTXA = *mut ACTCTXA;
STRUCT!{struct ACTCTXW {
    cbSize: ULONG,
    dwFlags: DWORD,
    lpSource: LPCWSTR,
    wProcessorArchitecture: USHORT,
    wLangId: LANGID,
    lpAssemblyDirectory: LPCWSTR,
    lpResourceName: LPCWSTR,
    lpApplicationName: LPCWSTR,
    hModule: HMODULE,
}}
pub type PACTCTXW = *mut ACTCTXW;
pub type PCACTCTXA = *const ACTCTXA;
pub type PCACTCTXW = *const ACTCTXW;
extern "system" {
    pub fn CreateActCtxA(
        pActCtx: PCACTCTXA,
    ) -> HANDLE;
    pub fn CreateActCtxW(
        pActCtx: PCACTCTXW,
    ) -> HANDLE;
    pub fn AddRefActCtx(
        hActCtx: HANDLE,
    );
    pub fn ReleaseActCtx(
        hActCtx: HANDLE,
    );
    pub fn ZombifyActCtx(
        hActCtx: HANDLE,
    ) -> BOOL;
    pub fn ActivateActCtx(
        hActCtx: HANDLE,
        lpCookie: *mut ULONG_PTR,
    ) -> BOOL;
    pub fn DeactivateActCtx(
        dwFlags: DWORD,
        ulCookie: ULONG_PTR,
    ) -> BOOL;
    pub fn GetCurrentActCtx(
        lphActCtx: *mut HANDLE,
    ) -> BOOL;
}
STRUCT!{struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    lpInformation: PVOID,
    lpSectionBase: PVOID,
    ulSectionLength: ULONG,
    lpSectionGlobalDataBase: PVOID,
    ulSectionGlobalDataLength: ULONG,
}}
pub type PACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA =
    *mut ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
pub type PCACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA =
    *const ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
STRUCT!{struct ACTCTX_SECTION_KEYED_DATA {
    cbSize: ULONG,
    ulDataFormatVersion: ULONG,
    lpData: PVOID,
    ulLength: ULONG,
    lpSectionGlobalData: PVOID,
    ulSectionGlobalDataLength: ULONG,
    lpSectionBase: PVOID,
    ulSectionTotalLength: ULONG,
    hActCtx: HANDLE,
    ulAssemblyRosterIndex: ULONG,
    ulFlags: ULONG,
    AssemblyMetadata: ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}}
pub type PACTCTX_SECTION_KEYED_DATA = *mut ACTCTX_SECTION_KEYED_DATA;
pub type PCACTCTX_SECTION_KEYED_DATA = *const ACTCTX_SECTION_KEYED_DATA;
extern "system" {
    pub fn FindActCtxSectionStringA(
        dwFlags: DWORD,
        lpExtensionGuid: *const GUID,
        ulSectionId: ULONG,
        lpStringToFind: LPCSTR,
        ReturnedData: PACTCTX_SECTION_KEYED_DATA,
    ) -> BOOL;
    pub fn FindActCtxSectionStringW(
        dwFlags: DWORD,
        lpExtensionGuid: *const GUID,
        ulSectionId: ULONG,
        lpStringToFind: LPCWSTR,
        ReturnedData: PACTCTX_SECTION_KEYED_DATA,
    ) -> BOOL;
    pub fn FindActCtxSectionGuid(
        dwFlags: DWORD,
        lpExtensionGuid: *const GUID,
        ulSectionId: ULONG,
        lpGuidToFind: *const GUID,
        ReturnedData: PACTCTX_SECTION_KEYED_DATA,
    ) -> BOOL;
    pub fn QueryActCtxW(
        dwFlags: DWORD,
        hActCtx: HANDLE,
        pvSubInstance: PVOID,
        ulInfoClass: ULONG,
        pvBuffer: PVOID,
        cbBuffer: SIZE_T,
        pcbWrittenOrRequired: *mut SIZE_T,
    ) -> BOOL;
    pub fn WTSGetActiveConsoleSessionId() -> DWORD;
    // pub fn WTSGetServiceSessionId();
    // pub fn WTSIsServerContainer();
    pub fn GetActiveProcessorGroupCount() -> WORD;
    pub fn GetMaximumProcessorGroupCount() -> WORD;
    pub fn GetActiveProcessorCount(
        GroupNumber: WORD,
    ) -> DWORD;
    pub fn GetMaximumProcessorCount(
        GroupNumber: WORD,
    ) -> DWORD;
    pub fn GetNumaProcessorNode(
        Processor: UCHAR,
        NodeNumber: PUCHAR,
    ) -> BOOL;
    pub fn GetNumaNodeNumberFromHandle(
        hFile: HANDLE,
        NodeNumber: PUSHORT,
    ) -> BOOL;
    pub fn GetNumaProcessorNodeEx(
        Processor: PPROCESSOR_NUMBER,
        NodeNumber: PUSHORT,
    ) -> BOOL;
    pub fn GetNumaNodeProcessorMask(
        Node: UCHAR,
        ProcessorMask: PULONGLONG,
    ) -> BOOL;
    pub fn GetNumaAvailableMemoryNode(
        Node: UCHAR,
        AvailableBytes: PULONGLONG,
    ) -> BOOL;
    pub fn GetNumaAvailableMemoryNodeEx(
        Node: USHORT,
        AvailableBytes: PULONGLONG,
    ) -> BOOL;
    pub fn GetNumaProximityNode(
        ProximityId: ULONG,
        NodeNumber: PUCHAR,
    ) -> BOOL;
}
FN!{stdcall APPLICATION_RECOVERY_CALLBACK(
    pvParameter: PVOID,
) -> DWORD}
// RESTART_*
// RECOVERY_*
extern "system" {
    pub fn RegisterApplicationRecoveryCallback(
        pRecoveyCallback: APPLICATION_RECOVERY_CALLBACK,
        pvParameter: PVOID,
        dwPingInterval: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn UnregisterApplicationRecoveryCallback() -> HRESULT;
    pub fn RegisterApplicationRestart(
        pwzCommandline: PCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn UnregisterApplicationRestart() -> HRESULT;
    pub fn GetApplicationRecoveryCallback(
        hProcess: HANDLE,
        pRecoveryCallback: *mut APPLICATION_RECOVERY_CALLBACK,
        ppvParameter: *mut PVOID,
        pdwPingInterval: PDWORD,
        pdwFlags: PDWORD,
    ) -> HRESULT;
    pub fn GetApplicationRestartSettings(
        hProcess: HANDLE,
        pwzCommandline: PWSTR,
        pcchSize: PDWORD,
        pdwFlags: PDWORD,
    ) -> HRESULT;
    pub fn ApplicationRecoveryInProgress(
        pbCancelled: PBOOL,
    ) -> HRESULT;
    pub fn ApplicationRecoveryFinished(
        bSuccess: BOOL,
    );
}
// FILE_BASIC_INFO, etc.
extern "system" {
    pub fn GetFileInformationByHandleEx(
        hFile: HANDLE,
        FileInformationClass: FILE_INFO_BY_HANDLE_CLASS,
        lpFileInformation: LPVOID,
        dwBufferSize: DWORD,
    ) -> BOOL;
}
ENUM!{enum FILE_ID_TYPE {
    FileIdType,
    ObjectIdType,
    ExtendedFileIdType,
    MaximumFileIdType,
}}
UNION!{union FILE_ID_DESCRIPTOR_u {
    [u64; 2],
    FileId FileId_mut: LARGE_INTEGER,
    ObjectId ObjectId_mut: GUID,
    ExtendedFileId ExtendedFileId_mut: FILE_ID_128,
}}
STRUCT!{struct FILE_ID_DESCRIPTOR {
    dwSize: DWORD,
    Type: FILE_ID_TYPE,
    u: FILE_ID_DESCRIPTOR_u,
}}
pub type LPFILE_ID_DESCRIPTOR = *mut FILE_ID_DESCRIPTOR;
extern "system" {
    pub fn OpenFileById(
        hVolumeHint: HANDLE,
        lpFileId: LPFILE_ID_DESCRIPTOR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwFlagsAndAttributes: DWORD,
    ) -> HANDLE;
    pub fn CreateSymbolicLinkA(
        lpSymlinkFileName: LPCSTR,
        lpTargetFileName: LPCSTR,
        dwFlags: DWORD,
    ) -> BOOLEAN;
    pub fn CreateSymbolicLinkW(
        lpSymlinkFileName: LPCWSTR,
        lpTargetFileName: LPCWSTR,
        dwFlags: DWORD,
    ) -> BOOLEAN;
    pub fn QueryActCtxSettingsW(
        dwFlags: DWORD,
        hActCtx: HANDLE,
        settingsNameSpace: PCWSTR,
        settingName: PCWSTR,
        pvBuffer: PWSTR,
        dwBuffer: SIZE_T,
        pdwWrittenOrRequired: *mut SIZE_T,
    ) -> BOOL;
    pub fn CreateSymbolicLinkTransactedA(
        lpSymlinkFileName: LPCSTR,
        lpTargetFileName: LPCSTR,
        dwFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOLEAN;
    pub fn CreateSymbolicLinkTransactedW(
        lpSymlinkFileName: LPCWSTR,
        lpTargetFileName: LPCWSTR,
        dwFlags: DWORD,
        hTransaction: HANDLE,
    ) -> BOOLEAN;
    pub fn ReplacePartitionUnit(
        TargetPartition: PWSTR,
        SparePartition: PWSTR,
        Flags: ULONG,
    ) -> BOOL;
    pub fn AddSecureMemoryCacheCallback(
        pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK,
    ) -> BOOL;
    pub fn RemoveSecureMemoryCacheCallback(
        pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK,
    ) -> BOOL;
    pub fn CopyContext(
        Destination: PCONTEXT,
        ContextFlags: DWORD,
        Source: PCONTEXT,
    ) -> BOOL;
    pub fn InitializeContext(
        Buffer: PVOID,
        ContextFlags: DWORD,
        Context: *mut PCONTEXT,
        ContextLength: PDWORD,
    ) -> BOOL;
    pub fn GetEnabledXStateFeatures() -> DWORD64;
    pub fn GetXStateFeaturesMask(
        Context: PCONTEXT,
        FeatureMask: PDWORD64,
    ) -> BOOL;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn LocateXStateFeature(
        Context: PCONTEXT,
        FeatureId: DWORD,
        Length: PDWORD,
    ) -> PVOID;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn SetXStateFeaturesMask(
        Context: PCONTEXT,
        FeatureMask: DWORD64,
    ) -> BOOL;
    pub fn EnableThreadProfiling(
        ThreadHandle: HANDLE,
        Flags: DWORD,
        HardwareCounters: DWORD64,
        PerformanceDataHandle: *mut HANDLE,
    ) -> BOOL;
    pub fn DisableThreadProfiling(
        PerformanceDataHandle: HANDLE,
    ) -> DWORD;
    pub fn QueryThreadProfiling(
        ThreadHandle: HANDLE,
        Enabled: PBOOLEAN,
    ) -> DWORD;
    pub fn ReadThreadProfilingData(
        PerformanceDataHandle: HANDLE,
        Flags: DWORD,
        PerformanceData: PPERFORMANCE_DATA,
    ) -> DWORD;
    // intrinsic InterlockedIncrement
    // intrinsic InterlockedDecrement
    // intrinsic InterlockedExchange
    // intrinsic InterlockedExchangeAdd
    // intrinsic InterlockedExchangeSubtract
    // intrinsic InterlockedCompareExchange
    // intrinsic InterlockedAnd
    // intrinsic InterlockedOr
    // intrinsic InterlockedXor
}
