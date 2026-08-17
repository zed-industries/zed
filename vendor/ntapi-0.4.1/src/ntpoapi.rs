use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LONG, NTSTATUS, PLONG, PUNICODE_STRING, PVOID, UCHAR, ULONG, UNICODE_STRING,
    USHORT,
};
use winapi::um::winnt::{
    DEVICE_POWER_STATE, EXECUTION_STATE, LATENCY_TIME, PDEVICE_POWER_STATE, PEXECUTION_STATE,
    POWER_ACTION, POWER_INFORMATION_LEVEL, SYSTEM_POWER_STATE,
};
UNION!{union POWER_STATE {
    SystemState: SYSTEM_POWER_STATE,
    DeviceState: DEVICE_POWER_STATE,
}}
pub type PPOWER_STATE = *mut POWER_STATE;
ENUM!{enum POWER_STATE_TYPE {
    SystemPowerState = 0,
    DevicePowerState = 1,
}}
pub type PPOWER_STATE_TYPE = *mut POWER_STATE_TYPE;
STRUCT!{struct SYSTEM_POWER_STATE_CONTEXT {
    ContextAsUlong: ULONG,
}}
BITFIELD!{SYSTEM_POWER_STATE_CONTEXT ContextAsUlong: ULONG [
    Reserved1 set_Reserved1[0..8],
    TargetSystemState set_TargetSystemState[8..12],
    EffectiveSystemState set_EffectiveSystemState[12..16],
    CurrentSystemState set_CurrentSystemState[16..20],
    IgnoreHibernationPath set_IgnoreHibernationPath[20..21],
    PseudoTransition set_PseudoTransition[21..22],
    Reserved2 set_Reserved2[22..32],
]}
pub type PSYSTEM_POWER_STATE_CONTEXT = *mut SYSTEM_POWER_STATE_CONTEXT;
STRUCT!{struct COUNTED_REASON_CONTEXT_u_s {
    ResourceFileName: UNICODE_STRING,
    ResourceReasonId: USHORT,
    StringCount: ULONG,
    ReasonStrings: PUNICODE_STRING,
}}
UNION!{union COUNTED_REASON_CONTEXT_u {
    s: COUNTED_REASON_CONTEXT_u_s,
    SimpleString: UNICODE_STRING,
}}
STRUCT!{struct COUNTED_REASON_CONTEXT {
    Version: ULONG,
    Flags: ULONG,
    u: COUNTED_REASON_CONTEXT_u,
}}
pub type PCOUNTED_REASON_CONTEXT = *mut COUNTED_REASON_CONTEXT;
ENUM!{enum POWER_STATE_HANDLER_TYPE {
    PowerStateSleeping1 = 0,
    PowerStateSleeping2 = 1,
    PowerStateSleeping3 = 2,
    PowerStateSleeping4 = 3,
    PowerStateShutdownOff = 4,
    PowerStateShutdownReset = 5,
    PowerStateSleeping4Firmware = 6,
    PowerStateMaximum = 7,
}}
pub type PPOWER_STATE_HANDLER_TYPE = *mut POWER_STATE_HANDLER_TYPE;
FN!{stdcall PENTER_STATE_SYSTEM_HANDLER(
    SystemContext: PVOID,
) -> NTSTATUS}
FN!{stdcall PENTER_STATE_HANDLER(
    Context: PVOID,
    SystemHandler: PENTER_STATE_SYSTEM_HANDLER,
    SystemContext: PVOID,
    NumberProcessors: LONG,
    Number: PLONG,
) -> NTSTATUS}
STRUCT!{struct POWER_STATE_HANDLER {
    Type: POWER_STATE_HANDLER_TYPE,
    RtcWake: BOOLEAN,
    Spare: [UCHAR; 3],
    Handler: PENTER_STATE_HANDLER,
    Context: PVOID,
}}
pub type PPOWER_STATE_HANDLER = *mut POWER_STATE_HANDLER;
FN!{stdcall PENTER_STATE_NOTIFY_HANDLER(
    State: POWER_STATE_HANDLER_TYPE,
    Context: PVOID,
    Entering: BOOLEAN,
) -> NTSTATUS}
STRUCT!{struct POWER_STATE_NOTIFY_HANDLER {
    Handler: PENTER_STATE_NOTIFY_HANDLER,
    Context: PVOID,
}}
pub type PPOWER_STATE_NOTIFY_HANDLER = *mut POWER_STATE_NOTIFY_HANDLER;
STRUCT!{struct PROCESSOR_POWER_INFORMATION {
    Number: ULONG,
    MaxMhz: ULONG,
    CurrentMhz: ULONG,
    MhzLimit: ULONG,
    MaxIdleState: ULONG,
    CurrentIdleState: ULONG,
}}
pub type PPROCESSOR_POWER_INFORMATION = *mut PROCESSOR_POWER_INFORMATION;
STRUCT!{struct SYSTEM_POWER_INFORMATION {
    MaxIdlenessAllowed: ULONG,
    Idleness: ULONG,
    TimeRemaining: ULONG,
    CoolingMode: UCHAR,
}}
pub type PSYSTEM_POWER_INFORMATION = *mut SYSTEM_POWER_INFORMATION;
EXTERN!{extern "system" {
    fn NtPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;
    fn NtSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: PEXECUTION_STATE,
    ) -> NTSTATUS;
    fn NtRequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> NTSTATUS;
    fn NtInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
    fn NtSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtGetDevicePowerState(
        Device: HANDLE,
        State: PDEVICE_POWER_STATE,
    ) -> NTSTATUS;
    fn NtIsSystemResumeAutomatic() -> BOOLEAN;
}}
