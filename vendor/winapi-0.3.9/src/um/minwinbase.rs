// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This module defines the 32-Bit Windows Base APIs
use shared::basetsd::ULONG_PTR;
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, HMODULE, LPVOID, MAX_PATH, UINT, ULONG, WORD};
use shared::ntstatus::{
    STATUS_ACCESS_VIOLATION, STATUS_ARRAY_BOUNDS_EXCEEDED, STATUS_BREAKPOINT,
    STATUS_CONTROL_C_EXIT, STATUS_DATATYPE_MISALIGNMENT, STATUS_FLOAT_DENORMAL_OPERAND,
    STATUS_FLOAT_DIVIDE_BY_ZERO, STATUS_FLOAT_INEXACT_RESULT, STATUS_FLOAT_INVALID_OPERATION,
    STATUS_FLOAT_OVERFLOW, STATUS_FLOAT_STACK_CHECK, STATUS_FLOAT_UNDERFLOW,
    STATUS_GUARD_PAGE_VIOLATION, STATUS_ILLEGAL_INSTRUCTION, STATUS_INTEGER_DIVIDE_BY_ZERO,
    STATUS_INTEGER_OVERFLOW, STATUS_INVALID_DISPOSITION, STATUS_INVALID_HANDLE,
    STATUS_IN_PAGE_ERROR, STATUS_NONCONTINUABLE_EXCEPTION, STATUS_PENDING,
    STATUS_POSSIBLE_DEADLOCK, STATUS_PRIVILEGED_INSTRUCTION, STATUS_SINGLE_STEP,
    STATUS_STACK_OVERFLOW,
};
use um::winnt::{
    CHAR, EXCEPTION_RECORD, HANDLE, LPSTR, LPWSTR, PCONTEXT, PRTL_CRITICAL_SECTION,
    PRTL_CRITICAL_SECTION_DEBUG, PVOID, RTL_CRITICAL_SECTION, RTL_CRITICAL_SECTION_DEBUG, WCHAR,
};
//MoveMemory
//CopyMemory
//FillMemory
//ZeroMemory
STRUCT!{struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}}
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
STRUCT!{struct OVERLAPPED_u_s {
    Offset: DWORD,
    OffsetHigh: DWORD,
}}
UNION!{union OVERLAPPED_u {
    [u32; 2] [u64; 1],
    s s_mut: OVERLAPPED_u_s,
    Pointer Pointer_mut: PVOID,
}}
STRUCT!{struct OVERLAPPED {
    Internal: ULONG_PTR,
    InternalHigh: ULONG_PTR,
    u: OVERLAPPED_u,
    hEvent: HANDLE,
}}
pub type LPOVERLAPPED = *mut OVERLAPPED;
STRUCT!{struct OVERLAPPED_ENTRY {
    lpCompletionKey: ULONG_PTR,
    lpOverlapped: LPOVERLAPPED,
    Internal: ULONG_PTR,
    dwNumberOfBytesTransferred: DWORD,
}}
pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;
STRUCT!{struct SYSTEMTIME {
    wYear: WORD,
    wMonth: WORD,
    wDayOfWeek: WORD,
    wDay: WORD,
    wHour: WORD,
    wMinute: WORD,
    wSecond: WORD,
    wMilliseconds: WORD,
}}
pub type PSYSTEMTIME = *mut SYSTEMTIME;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
STRUCT!{struct WIN32_FIND_DATAA {
    dwFileAttributes: DWORD,
    ftCreationTime: FILETIME,
    ftLastAccessTime: FILETIME,
    ftLastWriteTime: FILETIME,
    nFileSizeHigh: DWORD,
    nFileSizeLow: DWORD,
    dwReserved0: DWORD,
    dwReserved1: DWORD,
    cFileName: [CHAR; MAX_PATH],
    cAlternateFileName: [CHAR; 14],
}}
pub type PWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
pub type LPWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
STRUCT!{struct WIN32_FIND_DATAW {
    dwFileAttributes: DWORD,
    ftCreationTime: FILETIME,
    ftLastAccessTime: FILETIME,
    ftLastWriteTime: FILETIME,
    nFileSizeHigh: DWORD,
    nFileSizeLow: DWORD,
    dwReserved0: DWORD,
    dwReserved1: DWORD,
    cFileName: [WCHAR; MAX_PATH],
    cAlternateFileName: [WCHAR; 14],
}}
pub type PWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
ENUM!{enum FINDEX_INFO_LEVELS {
    FindExInfoStandard,
    FindExInfoBasic,
    FindExInfoMaxInfoLevel,
}}
pub const FIND_FIRST_EX_CASE_SENSITIVE: DWORD = 0x00000001;
pub const FIND_FIRST_EX_LARGE_FETCH: DWORD = 0x00000002;
ENUM!{enum FINDEX_SEARCH_OPS {
    FindExSearchNameMatch,
    FindExSearchLimitToDirectories,
    FindExSearchLimitToDevices,
    FindExSearchMaxSearchOp,
}}
ENUM!{enum GET_FILEEX_INFO_LEVELS {
    GetFileExInfoStandard,
    GetFileExMaxInfoLevel,
}}
ENUM!{enum FILE_INFO_BY_HANDLE_CLASS {
    FileBasicInfo,
    FileStandardInfo,
    FileNameInfo,
    FileRenameInfo,
    FileDispositionInfo,
    FileAllocationInfo,
    FileEndOfFileInfo,
    FileStreamInfo,
    FileCompressionInfo,
    FileAttributeTagInfo,
    FileIdBothDirectoryInfo,
    FileIdBothDirectoryRestartInfo,
    FileIoPriorityHintInfo,
    FileRemoteProtocolInfo,
    FileFullDirectoryInfo,
    FileFullDirectoryRestartInfo,
    FileStorageInfo,
    FileAlignmentInfo,
    FileIdInfo,
    FileIdExtdDirectoryInfo,
    FileIdExtdDirectoryRestartInfo,
    FileDispositionInfoEx,
    FileRenameInfoEx,
    MaximumFileInfoByHandleClass,
}}
pub type PFILE_INFO_BY_HANDLE_CLASS = *mut FILE_INFO_BY_HANDLE_CLASS;
pub type CRITICAL_SECTION = RTL_CRITICAL_SECTION;
pub type PCRITICAL_SECTION = PRTL_CRITICAL_SECTION;
pub type LPCRITICAL_SECTION = PRTL_CRITICAL_SECTION;
pub type CRITICAL_SECTION_DEBUG = RTL_CRITICAL_SECTION_DEBUG;
pub type PCRITICAL_SECTION_DEBUG = PRTL_CRITICAL_SECTION_DEBUG;
pub type LPCRITICAL_SECTION_DEBUG = PRTL_CRITICAL_SECTION_DEBUG;
FN!{stdcall LPOVERLAPPED_COMPLETION_ROUTINE(
    dwErrorCode: DWORD,
    dwNumberOfBytesTransfered: DWORD,
    lpOverlapped: LPOVERLAPPED,
) -> ()}
pub const LOCKFILE_FAIL_IMMEDIATELY: DWORD = 0x00000001;
pub const LOCKFILE_EXCLUSIVE_LOCK: DWORD = 0x00000002;
STRUCT!{struct PROCESS_HEAP_ENTRY_Block {
    hMem: HANDLE,
    dwReserved: [DWORD; 3],
}}
STRUCT!{struct PROCESS_HEAP_ENTRY_Region {
    dwCommittedSize: DWORD,
    dwUnCommittedSize: DWORD,
    lpFirstBlock: LPVOID,
    lpLastBlock: LPVOID,
}}
UNION!{union PROCESS_HEAP_ENTRY_u {
    [u32; 4] [u64; 3],
    Block Block_mut: PROCESS_HEAP_ENTRY_Block,
    Region Region_mut: PROCESS_HEAP_ENTRY_Region,
}}
STRUCT!{struct PROCESS_HEAP_ENTRY {
    lpData: PVOID,
    cbData: DWORD,
    cbOverhead: BYTE,
    iRegionIndex: BYTE,
    wFlags: WORD,
    u: PROCESS_HEAP_ENTRY_u,
}}
pub type LPPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub type PPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub const PROCESS_HEAP_REGION: WORD = 0x0001;
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: WORD = 0x0002;
pub const PROCESS_HEAP_ENTRY_BUSY: WORD = 0x0004;
pub const PROCESS_HEAP_SEG_ALLOC: WORD = 0x0008;
pub const PROCESS_HEAP_ENTRY_MOVEABLE: WORD = 0x0010;
pub const PROCESS_HEAP_ENTRY_DDESHARE: WORD = 0x0020;
STRUCT!{struct REASON_CONTEXT_Detailed {
    LocalizedReasonModule: HMODULE,
    LocalizedReasonId: ULONG,
    ReasonStringCount: ULONG,
    ReasonStrings: *mut LPWSTR,
}}
UNION!{union REASON_CONTEXT_Reason {
    [u32; 4] [u64; 3],
    Detailed Detailed_mut: REASON_CONTEXT_Detailed,
    SimpleReasonString SimpleReasonString_mut: LPWSTR,
}}
STRUCT!{struct REASON_CONTEXT {
    Version: ULONG,
    Flags: DWORD,
    Reason: REASON_CONTEXT_Reason,
}}
pub type PREASON_CONTEXT = *mut REASON_CONTEXT;
pub const EXCEPTION_DEBUG_EVENT: DWORD = 1;
pub const CREATE_THREAD_DEBUG_EVENT: DWORD = 2;
pub const CREATE_PROCESS_DEBUG_EVENT: DWORD = 3;
pub const EXIT_THREAD_DEBUG_EVENT: DWORD = 4;
pub const EXIT_PROCESS_DEBUG_EVENT: DWORD = 5;
pub const LOAD_DLL_DEBUG_EVENT: DWORD = 6;
pub const UNLOAD_DLL_DEBUG_EVENT: DWORD = 7;
pub const OUTPUT_DEBUG_STRING_EVENT: DWORD = 8;
pub const RIP_EVENT: DWORD = 9;
FN!{stdcall PTHREAD_START_ROUTINE(
    lpThreadParameter: LPVOID,
) -> DWORD}
pub type LPTHREAD_START_ROUTINE = PTHREAD_START_ROUTINE;
FN!{stdcall PENCLAVE_ROUTINE(
    lpThreadParameter: LPVOID,
) -> DWORD}
pub type LPENCLAVE_ROUTINE = PENCLAVE_ROUTINE;
STRUCT!{struct EXCEPTION_DEBUG_INFO {
    ExceptionRecord: EXCEPTION_RECORD,
    dwFirstChance: DWORD,
}}
pub type LPEXCEPTION_DEBUG_INFO = *mut EXCEPTION_DEBUG_INFO;
STRUCT!{struct CREATE_THREAD_DEBUG_INFO {
    hThread: HANDLE,
    lpThreadLocalBase: LPVOID,
    lpStartAddress: LPTHREAD_START_ROUTINE,
}}
pub type LPCREATE_THREAD_DEBUG_INFO = *mut CREATE_THREAD_DEBUG_INFO;
STRUCT!{struct CREATE_PROCESS_DEBUG_INFO {
    hFile: HANDLE,
    hProcess: HANDLE,
    hThread: HANDLE,
    lpBaseOfImage: LPVOID,
    dwDebugInfoFileOffset: DWORD,
    nDebugInfoSize: DWORD,
    lpThreadLocalBase: LPVOID,
    lpStartAddress: LPTHREAD_START_ROUTINE,
    lpImageName: LPVOID,
    fUnicode: WORD,
}}
pub type LPCREATE_PROCESS_DEBUG_INFO = *mut CREATE_PROCESS_DEBUG_INFO;
STRUCT!{struct EXIT_THREAD_DEBUG_INFO {
    dwExitCode: DWORD,
}}
pub type LPEXIT_THREAD_DEBUG_INFO = *mut EXIT_THREAD_DEBUG_INFO;
STRUCT!{struct EXIT_PROCESS_DEBUG_INFO {
    dwExitCode: DWORD,
}}
pub type LPEXIT_PROCESS_DEBUG_INFO = *mut EXIT_PROCESS_DEBUG_INFO;
STRUCT!{struct LOAD_DLL_DEBUG_INFO {
    hFile: HANDLE,
    lpBaseOfDll: LPVOID,
    dwDebugInfoFileOffset: DWORD,
    nDebugInfoSize: DWORD,
    lpImageName: LPVOID,
    fUnicode: WORD,
}}
pub type LPLOAD_DLL_DEBUG_INFO = *mut LOAD_DLL_DEBUG_INFO;
STRUCT!{struct UNLOAD_DLL_DEBUG_INFO {
    lpBaseOfDll: LPVOID,
}}
pub type LPUNLOAD_DLL_DEBUG_INFO = *mut UNLOAD_DLL_DEBUG_INFO;
STRUCT!{struct OUTPUT_DEBUG_STRING_INFO {
    lpDebugStringData: LPSTR,
    fUnicode: WORD,
    nDebugStringLength: WORD,
}}
pub type LPOUTPUT_DEBUG_STRING_INFO = *mut OUTPUT_DEBUG_STRING_INFO;
STRUCT!{struct RIP_INFO {
    dwError: DWORD,
    dwType: DWORD,
}}
pub type LPRIP_INFO = *mut RIP_INFO;
UNION!{union DEBUG_EVENT_u {
    [u32; 21] [u64; 20],
    Exception Exception_mut: EXCEPTION_DEBUG_INFO,
    CreateThread CreateThread_mut: CREATE_THREAD_DEBUG_INFO,
    CreateProcessInfo CreateProcessInfo_mut: CREATE_PROCESS_DEBUG_INFO,
    ExitThread ExitThread_mut: EXIT_THREAD_DEBUG_INFO,
    ExitProcess ExitProcess_mut: EXIT_PROCESS_DEBUG_INFO,
    LoadDll LoadDll_mut: LOAD_DLL_DEBUG_INFO,
    UnloadDll UnloadDll_mut: UNLOAD_DLL_DEBUG_INFO,
    DebugString DebugString_mut: OUTPUT_DEBUG_STRING_INFO,
    RipInfo RipInfo_mut: RIP_INFO,
}}
STRUCT!{struct DEBUG_EVENT {
    dwDebugEventCode: DWORD,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
    u: DEBUG_EVENT_u,
}}
pub type LPDEBUG_EVENT = *mut DEBUG_EVENT;
pub type LPCONTEXT = PCONTEXT;
pub const STILL_ACTIVE: DWORD = STATUS_PENDING as u32;
pub const EXCEPTION_ACCESS_VIOLATION: DWORD = STATUS_ACCESS_VIOLATION as u32;
pub const EXCEPTION_DATATYPE_MISALIGNMENT: DWORD = STATUS_DATATYPE_MISALIGNMENT as u32;
pub const EXCEPTION_BREAKPOINT: DWORD = STATUS_BREAKPOINT as u32;
pub const EXCEPTION_SINGLE_STEP: DWORD = STATUS_SINGLE_STEP as u32;
pub const EXCEPTION_ARRAY_BOUNDS_EXCEEDED: DWORD = STATUS_ARRAY_BOUNDS_EXCEEDED as u32;
pub const EXCEPTION_FLT_DENORMAL_OPERAND: DWORD = STATUS_FLOAT_DENORMAL_OPERAND as u32;
pub const EXCEPTION_FLT_DIVIDE_BY_ZERO: DWORD = STATUS_FLOAT_DIVIDE_BY_ZERO as u32;
pub const EXCEPTION_FLT_INEXACT_RESULT: DWORD = STATUS_FLOAT_INEXACT_RESULT as u32;
pub const EXCEPTION_FLT_INVALID_OPERATION: DWORD = STATUS_FLOAT_INVALID_OPERATION as u32;
pub const EXCEPTION_FLT_OVERFLOW: DWORD = STATUS_FLOAT_OVERFLOW as u32;
pub const EXCEPTION_FLT_STACK_CHECK: DWORD = STATUS_FLOAT_STACK_CHECK as u32;
pub const EXCEPTION_FLT_UNDERFLOW: DWORD = STATUS_FLOAT_UNDERFLOW as u32;
pub const EXCEPTION_INT_DIVIDE_BY_ZERO: DWORD = STATUS_INTEGER_DIVIDE_BY_ZERO as u32;
pub const EXCEPTION_INT_OVERFLOW: DWORD = STATUS_INTEGER_OVERFLOW as u32;
pub const EXCEPTION_PRIV_INSTRUCTION: DWORD = STATUS_PRIVILEGED_INSTRUCTION as u32;
pub const EXCEPTION_IN_PAGE_ERROR: DWORD = STATUS_IN_PAGE_ERROR as u32;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: DWORD = STATUS_ILLEGAL_INSTRUCTION as u32;
pub const EXCEPTION_NONCONTINUABLE_EXCEPTION: DWORD = STATUS_NONCONTINUABLE_EXCEPTION as u32;
pub const EXCEPTION_STACK_OVERFLOW: DWORD = STATUS_STACK_OVERFLOW as u32;
pub const EXCEPTION_INVALID_DISPOSITION: DWORD = STATUS_INVALID_DISPOSITION as u32;
pub const EXCEPTION_GUARD_PAGE: DWORD = STATUS_GUARD_PAGE_VIOLATION as u32;
pub const EXCEPTION_INVALID_HANDLE: DWORD = STATUS_INVALID_HANDLE as u32;
pub const EXCEPTION_POSSIBLE_DEADLOCK: DWORD = STATUS_POSSIBLE_DEADLOCK as u32;
pub const CONTROL_C_EXIT: DWORD = STATUS_CONTROL_C_EXIT as u32;
pub const LMEM_FIXED: UINT = 0x0000;
pub const LMEM_MOVEABLE: UINT = 0x0002;
pub const LMEM_NOCOMPACT: UINT = 0x0010;
pub const LMEM_NODISCARD: UINT = 0x0020;
pub const LMEM_ZEROINIT: UINT = 0x0040;
pub const LMEM_MODIFY: UINT = 0x0080;
pub const LMEM_DISCARDABLE: UINT = 0x0F00;
pub const LMEM_VALID_FLAGS: UINT = 0x0F72;
pub const LMEM_INVALID_HANDLE: UINT = 0x8000;
pub const LHND: UINT = LMEM_MOVEABLE | LMEM_ZEROINIT;
pub const LPTR: UINT = LMEM_FIXED | LMEM_ZEROINIT;
pub const NONZEROLHND: UINT = LMEM_MOVEABLE;
pub const NONZEROLPTR: UINT = LMEM_FIXED;
//LocalDiscard
pub const LMEM_DISCARDED: UINT = 0x4000;
pub const LMEM_LOCKCOUNT: UINT = 0x00FF;
pub const NUMA_NO_PREFERRED_NODE: DWORD = -1i32 as u32;
