// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::basetsd::{DWORD_PTR, SIZE_T};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, LPBOOL, LPDWORD, PULONG};
use um::winnt::{HRESULT, LARGE_INTEGER, WCHAR};
ENUM!{enum WCT_OBJECT_TYPE {
    WctCriticalSectionType = 1,
    WctSendMessageType,
    WctMutexType,
    WctAlpcType,
    WctComType,
    WctThreadWaitType,
    WctProcessWaitType,
    WctThreadType,
    WctComActivationType,
    WctUnknownType,
    WctSocketIoType,
    WctSmbIoType,
    WctMaxType,
}}
ENUM!{enum WCT_OBJECT_STATUS {
    WctStatusNoAccess = 1,
    WctStatusRunning,
    WctStatusBlocked,
    WctStatusPidOnly,
    WctStatusPidOnlyRpcss,
    WctStatusOwned,
    WctStatusNotOwned,
    WctStatusAbandoned,
    WctStatusUnknown,
    WctStatusError,
    WctStatusMax,
}}
pub const WCT_MAX_NODE_COUNT: SIZE_T = 16;
pub const WCT_OBJNAME_LENGTH: SIZE_T = 128;
STRUCT!{struct WAITCHAIN_NODE_INFO_LOCK_OBJECT {
    ObjectName: [WCHAR; WCT_OBJNAME_LENGTH],
    Timeout: LARGE_INTEGER,
    Alertable: BOOL,
}}
STRUCT!{struct WAITCHAIN_NODE_INFO_THREAD_OBJECT {
    ProcessId: DWORD,
    ThreadId: DWORD,
    WaitTime: DWORD,
    ContextSwitches: DWORD,
}}
UNION!{union WAITCHAIN_NODE_INFO_u {
    [u64; 34],
    LockObject LockObject_mut: WAITCHAIN_NODE_INFO_LOCK_OBJECT,
    ThreadObject ThreadObject_mut: WAITCHAIN_NODE_INFO_THREAD_OBJECT,
}}
STRUCT!{struct WAITCHAIN_NODE_INFO {
    ObjectType: WCT_OBJECT_TYPE,
    ObjectStatus: WCT_OBJECT_STATUS,
    u: WAITCHAIN_NODE_INFO_u,
}}
pub type PWAITCHAIN_NODE_INFO = *mut WAITCHAIN_NODE_INFO;
DECLARE_HANDLE!{HWCT, HWCT__}
FN!{cdecl PWAITCHAINCALLBACK(
    WctHandle: HWCT,
    Context: DWORD_PTR,
    CallbackStatus: DWORD,
    NodeCount: LPDWORD,
    NodeInfoArray: PWAITCHAIN_NODE_INFO,
    IsCycle: LPBOOL,
) -> ()}
pub const WCT_ASYNC_OPEN_FLAG: DWORD = 1;
pub const WCTP_OPEN_ALL_FLAGS: DWORD = WCT_ASYNC_OPEN_FLAG;
extern "system" {
    pub fn OpenThreadWaitChainSession(
        Flags: DWORD,
        callback: PWAITCHAINCALLBACK,
    ) -> HWCT;
    pub fn CloseThreadWaitChainSession(
        WctHandle: HWCT,
    );
}
pub const WCT_OUT_OF_PROC_FLAG: DWORD = 0x1;
pub const WCT_OUT_OF_PROC_COM_FLAG: DWORD = 0x2;
pub const WCT_OUT_OF_PROC_CS_FLAG: DWORD = 0x4;
pub const WCT_NETWORK_IO_FLAG: DWORD = 0x8;
pub const WCTP_GETINFO_ALL_FLAGS: DWORD = WCT_OUT_OF_PROC_FLAG | WCT_OUT_OF_PROC_COM_FLAG
    | WCT_OUT_OF_PROC_CS_FLAG;
extern "system" {
    pub fn GetThreadWaitChain(
        WctHandle: HWCT,
        Context: DWORD_PTR,
        Flags: DWORD,
        ThreadId: DWORD,
        NodeCount: LPDWORD,
        NodeInfoArray: PWAITCHAIN_NODE_INFO,
        IsCycle: LPBOOL,
    ) -> BOOL;
}
FN!{cdecl PCOGETCALLSTATE(
    c_int,
    PULONG,
) -> HRESULT}
FN!{cdecl PCOGETACTIVATIONSTATE(
    GUID,
    DWORD,
    *mut DWORD,
) -> HRESULT}
extern "system" {
    pub fn RegisterWaitChainCOMCallback(
        CallStateCallback: PCOGETCALLSTATE,
        ActivationStateCallback: PCOGETACTIVATIONSTATE,
    );
}
