use crate::ntapi_base::{CLIENT_ID, PCLIENT_ID};
use winapi::shared::evntprov::EVENT_FILTER_DESCRIPTOR;
use winapi::shared::guiddef::LPCGUID;
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, NTSTATUS, PCCH, PCH, PCSTR, PHANDLE, PLARGE_INTEGER, POBJECT_ATTRIBUTES,
    PULONG, PVOID, UCHAR, ULONG, ULONGLONG,
};
use winapi::um::minwinbase::LPDEBUG_EVENT;
use winapi::um::winnt::{ACCESS_MASK, EXCEPTION_RECORD, STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE};
use winapi::vc::vadefs::va_list;
EXTERN!{extern "system" {
    fn DbgUserBreakPoint();
    fn DbgBreakPoint();
    fn DbgBreakPointWithStatus(
        Status: ULONG,
    );
}}
pub const DBG_STATUS_CONTROL_C: u32 = 1;
pub const DBG_STATUS_SYSRQ: u32 = 2;
pub const DBG_STATUS_BUGCHECK_FIRST: u32 = 3;
pub const DBG_STATUS_BUGCHECK_SECOND: u32 = 4;
pub const DBG_STATUS_FATAL: u32 = 5;
pub const DBG_STATUS_DEBUG_CONTROL: u32 = 6;
pub const DBG_STATUS_WORKER: u32 = 7;
EXTERN!{extern "C" {
    fn DbgPrint(
        Format: PCSTR,
        ...
    ) -> ULONG;
    fn DbgPrintEx(
        ComponentId: ULONG,
        Level: ULONG,
        Format: PCSTR,
        ...
    ) -> ULONG;
}}
EXTERN!{extern "system" {
    fn vDbgPrintEx(
        ComponentId: ULONG,
        Level: ULONG,
        Format: PCCH,
        arglist: va_list,
    ) -> ULONG;
    fn vDbgPrintExWithPrefix(
        Prefix: PCH,
        ComponentId: ULONG,
        Level: ULONG,
        Format: PCCH,
        arglist: va_list,
    ) -> ULONG;
    fn DbgQueryDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
    ) -> NTSTATUS;
    fn DbgSetDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
        State: BOOLEAN,
    ) -> NTSTATUS;
    fn DbgPrompt(
        Prompt: PCCH,
        Response: PCH,
        Length: ULONG,
    ) -> ULONG;
}}
STRUCT!{struct DBGKM_EXCEPTION {
    ExceptionRecord: EXCEPTION_RECORD,
    FirstChance: ULONG,
}}
pub type PDBGKM_EXCEPTION = *mut DBGKM_EXCEPTION;
STRUCT!{struct DBGKM_CREATE_THREAD {
    SubSystemKey: ULONG,
    StartAddress: PVOID,
}}
pub type PDBGKM_CREATE_THREAD = *mut DBGKM_CREATE_THREAD;
STRUCT!{struct DBGKM_CREATE_PROCESS {
    SubSystemKey: ULONG,
    FileHandle: HANDLE,
    BaseOfImage: PVOID,
    DebugInfoFileOffset: ULONG,
    DebugInfoSize: ULONG,
    InitialThread: DBGKM_CREATE_THREAD,
}}
pub type PDBGKM_CREATE_PROCESS = *mut DBGKM_CREATE_PROCESS;
STRUCT!{struct DBGKM_EXIT_THREAD {
    ExitStatus: NTSTATUS,
}}
pub type PDBGKM_EXIT_THREAD = *mut DBGKM_EXIT_THREAD;
STRUCT!{struct DBGKM_EXIT_PROCESS {
    ExitStatus: NTSTATUS,
}}
pub type PDBGKM_EXIT_PROCESS = *mut DBGKM_EXIT_PROCESS;
STRUCT!{struct DBGKM_LOAD_DLL {
    FileHandle: HANDLE,
    BaseOfDll: PVOID,
    DebugInfoFileOffset: ULONG,
    DebugInfoSize: ULONG,
    NamePointer: PVOID,
}}
pub type PDBGKM_LOAD_DLL = *mut DBGKM_LOAD_DLL;
STRUCT!{struct DBGKM_UNLOAD_DLL {
    BaseAddress: PVOID,
}}
pub type PDBGKM_UNLOAD_DLL = *mut DBGKM_UNLOAD_DLL;
ENUM!{enum DBG_STATE {
    DbgIdle = 0,
    DbgReplyPending = 1,
    DbgCreateThreadStateChange = 2,
    DbgCreateProcessStateChange = 3,
    DbgExitThreadStateChange = 4,
    DbgExitProcessStateChange = 5,
    DbgExceptionStateChange = 6,
    DbgBreakpointStateChange = 7,
    DbgSingleStepStateChange = 8,
    DbgLoadDllStateChange = 9,
    DbgUnloadDllStateChange = 10,
}}
pub type PDBG_STATE = *mut DBG_STATE;
STRUCT!{struct DBGUI_CREATE_THREAD {
    HandleToThread: HANDLE,
    NewThread: DBGKM_CREATE_THREAD,
}}
pub type PDBGUI_CREATE_THREAD = *mut DBGUI_CREATE_THREAD;
STRUCT!{struct DBGUI_CREATE_PROCESS {
    HandleToProcess: HANDLE,
    HandleToThread: HANDLE,
    NewProcess: DBGKM_CREATE_PROCESS,
}}
UNION!{union DBGUI_WAIT_STATE_CHANGE_StateInfo {
    Exception: DBGKM_EXCEPTION,
    CreateThread: DBGUI_CREATE_THREAD,
    CreateProcessInfo: DBGUI_CREATE_PROCESS,
    ExitThread: DBGKM_EXIT_THREAD,
    ExitProcess: DBGKM_EXIT_PROCESS,
    LoadDll: DBGKM_LOAD_DLL,
    UnloadDll: DBGKM_UNLOAD_DLL,
}}
pub type PDBGUI_CREATE_PROCESS = *mut DBGUI_CREATE_PROCESS;
STRUCT!{struct DBGUI_WAIT_STATE_CHANGE {
    NewState: DBG_STATE,
    AppClientId: CLIENT_ID,
    StateInfo: DBGUI_WAIT_STATE_CHANGE_StateInfo,
}}
pub type PDBGUI_WAIT_STATE_CHANGE = *mut DBGUI_WAIT_STATE_CHANGE;
pub const DEBUG_READ_EVENT: ULONG = 0x0001;
pub const DEBUG_PROCESS_ASSIGN: ULONG = 0x0002;
pub const DEBUG_SET_INFORMATION: ULONG = 0x0004;
pub const DEBUG_QUERY_INFORMATION: ULONG = 0x0008;
pub const DEBUG_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | DEBUG_READ_EVENT
    | DEBUG_PROCESS_ASSIGN | DEBUG_SET_INFORMATION | DEBUG_QUERY_INFORMATION;
pub const DEBUG_KILL_ON_CLOSE: u32 = 0x1;
ENUM!{enum DEBUGOBJECTINFOCLASS {
    DebugObjectUnusedInformation = 0,
    DebugObjectKillProcessOnExitInformation = 1,
    MaxDebugObjectInfoClass = 2,
}}
pub type PDEBUGOBJECTINFOCLASS = *mut DEBUGOBJECTINFOCLASS;
EXTERN!{extern "system" {
    fn NtCreateDebugObject(
        DebugObjectHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: PVOID,
        DebugInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
        WaitStateChange: PVOID,
    ) -> NTSTATUS;
    fn DbgUiConnectToDbg() -> NTSTATUS;
    fn DbgUiGetThreadDebugObject() -> HANDLE;
    fn DbgUiSetThreadDebugObject(
        DebugObject: HANDLE,
    );
    fn DbgUiWaitStateChange(
        StateChange: PDBGUI_WAIT_STATE_CHANGE,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn DbgUiContinue(
        AppClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn DbgUiStopDebugging(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiDebugActiveProcess(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiRemoteBreakin(
        Context: PVOID,
    );
    fn DbgUiIssueRemoteBreakin(
        Process: HANDLE,
    ) -> NTSTATUS;
    fn DbgUiConvertStateChangeStructure(
        StateChange: PDBGUI_WAIT_STATE_CHANGE,
        DebugEvent: LPDEBUG_EVENT,
    ) -> NTSTATUS;
}}
FN!{stdcall PENABLECALLBACK(
    SourceId: LPCGUID,
    IsEnabled: ULONG,
    Level: UCHAR,
    MatchAnyKeyword: ULONGLONG,
    MatchAllKeyword: ULONGLONG,
    FilterData: *mut EVENT_FILTER_DESCRIPTOR,
    CallbackContext: PVOID,
) -> ()}
pub type REGHANDLE = ULONGLONG;
pub type PREGHANDLE = *mut ULONGLONG;
EXTERN!{extern "system" {
    fn EtwEventRegister(
        ProviderId: LPCGUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: PVOID,
        RegHandle: PREGHANDLE,
    ) -> NTSTATUS;
}}
