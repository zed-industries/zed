// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_long, c_short};
use shared::minwindef::{DWORD, INT};
use shared::wtypes::{BSTR, DATE, VARIANT_BOOL};
use um::minwinbase::SYSTEMTIME;
use um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{HRESULT, LONG};
RIDL!{#[uuid(0x0f87369f, 0xa4e5, 0x4cfc, 0xbd, 0x3e, 0x73, 0xe6, 0x15, 0x45, 0x72, 0xdd)]
class TaskScheduler;}
RIDL!{#[uuid(0xf2a69db7, 0xda2c, 0x4352, 0x90, 0x66, 0x86, 0xfe, 0xe6, 0xda, 0xca, 0xc9)]
class TaskHandlerPS;}
RIDL!{#[uuid(0x9f15266d, 0xd7ba, 0x48f0, 0x93, 0xc1, 0xe6, 0x89, 0x5f, 0x6f, 0xe5, 0xac)]
class TaskHandlerStatusPS;}
RIDL!{#[uuid(0x79184a66, 0x8664, 0x423f, 0x97, 0xf1, 0x63, 0x73, 0x56, 0xa5, 0xd8, 0x12)]
interface ITaskFolderCollection(ITaskFolderCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut LONG,
    ) -> HRESULT,
    fn get_Item(
        index: VARIANT,
        ppFolder: *mut *mut ITaskFolder,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8cfac062, 0xa080, 0x4c15, 0x9a, 0x88, 0xaa, 0x7c, 0x2a, 0xf8, 0x0d, 0xfc)]
interface ITaskFolder(ITaskFolderVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn get_Path(
        pPath: *mut BSTR,
    ) -> HRESULT,
    fn GetFolder(
        Path: BSTR,
        ppFolder: *mut *mut ITaskFolder,
    ) -> HRESULT,
    fn GetFolders(
        flags: LONG,
        ppFolders: *mut *mut ITaskFolderCollection,
    ) -> HRESULT,
    fn CreateFolder(
        subFolderName: BSTR,
        sddl: VARIANT,
        ppFolder: *mut *mut ITaskFolder,
    ) -> HRESULT,
    fn DeleteFolder(
        subFolderName: BSTR,
        flags: LONG,
    ) -> HRESULT,
    fn GetTask(
        Path: BSTR,
        ppTask: *mut *mut IRegisteredTask,
    ) -> HRESULT,
    fn GetTasks(
        flags: LONG,
        ppTasks: *mut *mut IRegisteredTaskCollection,
    ) -> HRESULT,
    fn DeleteTask(
        Name: BSTR,
        flags: LONG,
    ) -> HRESULT,
    fn RegisterTask(
        Path: BSTR,
        XmlText: BSTR,
        flags: LONG,
        UserId: VARIANT,
        password: VARIANT,
        LogonType: TASK_LOGON_TYPE,
        sddl: VARIANT,
        ppTask: *mut *mut IRegisteredTask,
    ) -> HRESULT,
    fn RegisterTaskDefinition(
        Path: BSTR,
        pDefinition: *const ITaskDefinition,
        flags: LONG,
        UserId: VARIANT,
        password: VARIANT,
        LogonType: TASK_LOGON_TYPE,
        sddl: VARIANT,
        ppTask: *mut *mut IRegisteredTask,
    ) -> HRESULT,
    fn GetSecurityDescriptor(
        securityInformation: LONG,
        pSddl: *mut BSTR,
    ) -> HRESULT,
    fn SetSecurityDescriptor(
        sddl: BSTR,
        flags: LONG,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9c86f320, 0xdee3, 0x4dd1, 0xb9, 0x72, 0xa3, 0x03, 0xf2, 0x6b, 0x06, 0x1e)]
interface IRegisteredTask(IRegisteredTaskVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn get_Path(
        pPath: *mut BSTR,
    ) -> HRESULT,
    fn get_State(
        pState: *mut TASK_STATE,
    ) -> HRESULT,
    fn get_Enabled(
        pEnabled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pEnabled: VARIANT_BOOL,
    ) -> HRESULT,
    fn Run(
        params: VARIANT,
        ppRunningTask: *mut *mut IRunningTask,
    ) -> HRESULT,
    fn RunEx(
        params: VARIANT,
        flags: LONG,
        sessionID: LONG,
        user: BSTR,
        ppRunningTask: *mut *mut IRunningTask,
    ) -> HRESULT,
    fn GetInstances(
        flags: LONG,
        ppRunningTasks: *mut *mut IRunningTaskCollection,
    ) -> HRESULT,
    fn get_LastRunTime(
        pLastRunTime: *mut DATE,
    ) -> HRESULT,
    fn get_LastTaskResult(
        pLastTaskResult: *mut LONG,
    ) -> HRESULT,
    fn get_NumberOfMissedRuns(
        pNumberOfMissedRuns: *mut LONG,
    ) -> HRESULT,
    fn get_NextRunTime(
        pNextRunTime: *mut DATE,
    ) -> HRESULT,
    fn get_Definition(
        ppDefinition: *mut *mut ITaskDefinition,
    ) -> HRESULT,
    fn get_Xml(
        pXml: *mut BSTR,
    ) -> HRESULT,
    fn GetSecurityDescriptor(
        securityInformation: LONG,
        pSddl: *mut BSTR,
    ) -> HRESULT,
    fn SetSecurityDescriptor(
        sddl: BSTR,
        flags: LONG,
    ) -> HRESULT,
    fn Stop(
        flags: LONG,
    ) -> HRESULT,
    fn GetRunTimes(
        pstStart: *const SYSTEMTIME,
        pstEnd: *const SYSTEMTIME,
        pCount: *mut DWORD,
        pRunTimes: *mut *mut SYSTEMTIME,
    ) -> HRESULT,
}}
ENUM!{enum TASK_STATE {
    TASK_STATE_UNKNOWN = 0,
    TASK_STATE_DISABLED = 1,
    TASK_STATE_QUEUED = 2,
    TASK_STATE_READY = 3,
    TASK_STATE_RUNNING = 4,
}}
RIDL!{#[uuid(0x653758fb, 0x7b9a, 0x4f1e, 0xa4, 0x71, 0xbe, 0xeb, 0x8e, 0x9b, 0x83, 0x4e)]
interface IRunningTask(IRunningTaskVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn get_InstanceGuid(
        pGuid: *mut BSTR,
    ) -> HRESULT,
    fn get_Path(
        pPath: *mut BSTR,
    ) -> HRESULT,
    fn get_State(
        pState: *mut TASK_STATE,
    ) -> HRESULT,
    fn get_CurrentAction(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Refresh() -> HRESULT,
    fn get_EnginePID(
        pPID: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6a67614b, 0x6828, 0x4fec, 0xaa, 0x54, 0x6d, 0x52, 0xe8, 0xf1, 0xf2, 0xdb)]
interface IRunningTaskCollection(IRunningTaskCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut LONG,
    ) -> HRESULT,
    fn get_Item(
        index: VARIANT,
        ppRunningTask: *mut *mut IRunningTask,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf5bc8fc5, 0x536d, 0x4f77, 0xb8, 0x52, 0xfb, 0xc1, 0x35, 0x6f, 0xde, 0xb6)]
interface ITaskDefinition(ITaskDefinitionVtbl): IDispatch(IDispatchVtbl) {
    fn get_RegistrationInfo(
        ppRegistrationInfo: *mut *mut IRegistrationInfo,
    ) -> HRESULT,
    fn put_RegistrationInfo(
        ppRegistrationInfo: *const IRegistrationInfo,
    ) -> HRESULT,
    fn get_Triggers(
        ppTriggers: *mut *mut ITriggerCollection,
    ) -> HRESULT,
    fn put_Triggers(
        ppTriggers: *const ITriggerCollection,
    ) -> HRESULT,
    fn get_Settings(
        ppSettings: *mut *mut ITaskSettings,
    ) -> HRESULT,
    fn put_Settings(
        ppSettings: *const ITaskSettings,
    ) -> HRESULT,
    fn get_Data(
        pData: *mut BSTR,
    ) -> HRESULT,
    fn put_Data(
        pData: BSTR,
    ) -> HRESULT,
    fn get_Principal(
        ppPrincipal: *mut *mut IPrincipal,
    ) -> HRESULT,
    fn put_Principal(
        ppPrincipal: *const IPrincipal,
    ) -> HRESULT,
    fn get_Actions(
        ppActions: *mut *mut IActionCollection,
    ) -> HRESULT,
    fn put_Actions(
        ppActions: *const IActionCollection,
    ) -> HRESULT,
    fn get_XmlText(
        pXml: *mut BSTR,
    ) -> HRESULT,
    fn put_XmlText(
        pXml: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x416d8b73, 0xcb41, 0x4ea1, 0x80, 0x5c, 0x9b, 0xe9, 0xa5, 0xac, 0x4a, 0x74)]
interface IRegistrationInfo(IRegistrationInfoVtbl): IDispatch(IDispatchVtbl) {
    fn get_Description(
        pDescription: *mut BSTR,
    ) -> HRESULT,
    fn put_Description(
        pDescription: BSTR,
    ) -> HRESULT,
    fn get_Author(
        pAuthor: *mut BSTR,
    ) -> HRESULT,
    fn put_Author(
        pAuthor: BSTR,
    ) -> HRESULT,
    fn get_Version(
        pVersion: *mut BSTR,
    ) -> HRESULT,
    fn put_Version(
        pVersion: BSTR,
    ) -> HRESULT,
    fn get_Date(
        pDate: *mut BSTR,
    ) -> HRESULT,
    fn put_Date(
        pDate: BSTR,
    ) -> HRESULT,
    fn get_Documentation(
        pDocumentation: *mut BSTR,
    ) -> HRESULT,
    fn put_Documentation(
        pDocumentation: BSTR,
    ) -> HRESULT,
    fn get_XmlText(
        pText: *mut BSTR,
    ) -> HRESULT,
    fn put_XmlText(
        pText: BSTR,
    ) -> HRESULT,
    fn get_URI(
        pUri: *mut BSTR,
    ) -> HRESULT,
    fn put_URI(
        pUri: BSTR,
    ) -> HRESULT,
    fn get_SecurityDescriptor(
        pSddl: *mut VARIANT,
    ) -> HRESULT,
    fn put_SecurityDescriptor(
        pSddl: VARIANT,
    ) -> HRESULT,
    fn get_Source(
        pSource: *mut BSTR,
    ) -> HRESULT,
    fn put_Source(
        pSource: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x85df5081, 0x1b24, 0x4f32, 0x87, 0x8a, 0xd9, 0xd1, 0x4d, 0xf4, 0xcb, 0x77)]
interface ITriggerCollection(ITriggerCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut c_long,
    ) -> HRESULT,
    fn get_Item(
        index: c_long,
        ppTrigger: *mut *mut ITrigger,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Create(
        Type: TASK_TRIGGER_TYPE2,
        ppTrigger: *mut *mut ITrigger,
    ) -> HRESULT,
    fn Remove(
        index: VARIANT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
RIDL!{#[uuid(0x09941815, 0xea89, 0x4b5b, 0x89, 0xe0, 0x2a, 0x77, 0x38, 0x01, 0xfa, 0xc3)]
interface ITrigger(ITriggerVtbl): IDispatch(IDispatchVtbl) {
    fn get_Type(
        pType: *mut TASK_TRIGGER_TYPE2,
    ) -> HRESULT,
    fn get_Id(
        pId: *mut BSTR,
    ) -> HRESULT,
    fn put_Id(
        pId: BSTR,
    ) -> HRESULT,
    fn get_Repetition(
        ppRepeat: *mut *mut IRepetitionPattern,
    ) -> HRESULT,
    fn put_Repetition(
        ppRepeat: *const IRepetitionPattern,
    ) -> HRESULT,
    fn get_ExecutionTimeLimit(
        pTimeLimit: *mut BSTR,
    ) -> HRESULT,
    fn put_ExecutionTimeLimit(
        pTimeLimit: BSTR,
    ) -> HRESULT,
    fn get_StartBoundary(
        pStart: *mut BSTR,
    ) -> HRESULT,
    fn put_StartBoundary(
        pStart: BSTR,
    ) -> HRESULT,
    fn get_EndBoundary(
        pEnd: *mut BSTR,
    ) -> HRESULT,
    fn put_EndBoundary(
        pEnd: BSTR,
    ) -> HRESULT,
    fn get_Enabled(
        pEnabled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pEnabled: VARIANT_BOOL,
    ) -> HRESULT,
}}
ENUM!{enum TASK_TRIGGER_TYPE2 {
    TASK_TRIGGER_EVENT = 0,
    TASK_TRIGGER_TIME = 1,
    TASK_TRIGGER_DAILY = 2,
    TASK_TRIGGER_WEEKLY = 3,
    TASK_TRIGGER_MONTHLY = 4,
    TASK_TRIGGER_MONTHLYDOW = 5,
    TASK_TRIGGER_IDLE = 6,
    TASK_TRIGGER_REGISTRATION = 7,
    TASK_TRIGGER_BOOT = 8,
    TASK_TRIGGER_LOGON = 9,
    TASK_TRIGGER_SESSION_STATE_CHANGE = 11,
    TASK_TRIGGER_CUSTOM_TRIGGER_01 = 12,
}}
RIDL!{#[uuid(0x7fb9acf1, 0x26be, 0x400e, 0x85, 0xb5, 0x29, 0x4b, 0x9c, 0x75, 0xdf, 0xd6)]
interface IRepetitionPattern(IRepetitionPatternVtbl): IDispatch(IDispatchVtbl) {
    fn get_Interval(
        pInterval: *mut BSTR,
    ) -> HRESULT,
    fn put_Interval(
        pInterval: BSTR,
    ) -> HRESULT,
    fn get_Duration(
        pDuration: *mut BSTR,
    ) -> HRESULT,
    fn put_Duration(
        pDuration: BSTR,
    ) -> HRESULT,
    fn get_StopAtDurationEnd(
        pStop: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StopAtDurationEnd(
        pStop: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8fd4711d, 0x2d02, 0x4c8c, 0x87, 0xe3, 0xef, 0xf6, 0x99, 0xde, 0x12, 0x7e)]
interface ITaskSettings(ITaskSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_AllowDemandStart(
        pAllowDemandStart: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AllowDemandStart(
        pAllowDemandStart: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RestartInterval(
        pRestartInterval: *mut BSTR,
    ) -> HRESULT,
    fn put_RestartInterval(
        pRestartInterval: BSTR,
    ) -> HRESULT,
    fn get_RestartCount(
        pRestartCount: *mut INT,
    ) -> HRESULT,
    fn put_RestartCount(
        pRestartCount: INT,
    ) -> HRESULT,
    fn get_MultipleInstances(
        pPolicy: *mut TASK_INSTANCES_POLICY,
    ) -> HRESULT,
    fn put_MultipleInstances(
        pPolicy: TASK_INSTANCES_POLICY,
    ) -> HRESULT,
    fn get_StopIfGoingOnBatteries(
        pStopIfOnBatteries: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StopIfGoingOnBatteries(
        pStopIfOnBatteries: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DisallowStartIfOnBatteries(
        pDisallowStart: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DisallowStartIfOnBatteries(
        pDisallowStart: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AllowHardTerminate(
        pAllowHardTerminate: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AllowHardTerminate(
        pAllowHardTerminate: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_StartWhenAvailable(
        pStartWhenAvailable: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StartWhenAvailable(
        pStartWhenAvailable: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_XmlText(
        pText: *mut BSTR,
    ) -> HRESULT,
    fn put_XmlText(
        pText: BSTR,
    ) -> HRESULT,
    fn get_RunOnlyIfNetworkAvailable(
        pRunOnlyIfNetworkAvailable: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RunOnlyIfNetworkAvailable(
        pRunOnlyIfNetworkAvailable: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_ExecutionTimeLimit(
        pExecutionTimeLimit: *mut BSTR,
    ) -> HRESULT,
    fn put_ExecutionTimeLimit(
        pExecutionTimeLimit: BSTR,
    ) -> HRESULT,
    fn get_Enabled(
        pEnabled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Enabled(
        pEnabled: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_DeleteExpiredTaskAfter(
        pExpirationDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_DeleteExpiredTaskAfter(
        pExpirationDelay: BSTR,
    ) -> HRESULT,
    fn get_Priority(
        pPriority: *mut INT,
    ) -> HRESULT,
    fn put_Priority(
        pPriority: INT,
    ) -> HRESULT,
    fn get_Compatibility(
        pCompatLevel: *mut TASK_COMPATIBILITY,
    ) -> HRESULT,
    fn put_Compatibility(
        pCompatLevel: TASK_COMPATIBILITY,
    ) -> HRESULT,
    fn get_Hidden(
        pHidden: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Hidden(
        pHidden: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IdleSettings(
        ppIdleSettings: *mut *mut IIdleSettings,
    ) -> HRESULT,
    fn put_IdleSettings(
        ppIdleSettings: *const IIdleSettings,
    ) -> HRESULT,
    fn get_RunOnlyIfIdle(
        pRunOnlyIfIdle: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RunOnlyIfIdle(
        pRunOnlyIfIdle: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_WakeToRun(
        pWake: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_WakeToRun(
        pWake: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_NetworkSettings(
        ppNetworkSettings: *mut *mut INetworkSettings,
    ) -> HRESULT,
    fn put_NetworkSettings(
        ppNetworkSettings: *const INetworkSettings,
    ) -> HRESULT,
}}
ENUM!{enum TASK_INSTANCES_POLICY {
    TASK_INSTANCES_PARALLEL = 0,
    TASK_INSTANCES_QUEUE = 1,
    TASK_INSTANCES_IGNORE_NEW = 2,
    TASK_INSTANCES_STOP_EXISTING = 3,
}}
ENUM!{enum TASK_COMPATIBILITY {
    TASK_COMPATIBILITY_AT = 0,
    TASK_COMPATIBILITY_V1 = 1,
    TASK_COMPATIBILITY_V2 = 2,
    TASK_COMPATIBILITY_V2_1 = 3,
    TASK_COMPATIBILITY_V2_2 = 4,
    TASK_COMPATIBILITY_V2_3 = 5,
    TASK_COMPATIBILITY_V2_4 = 6,
}}
RIDL!{#[uuid(0x84594461, 0x0053, 0x4342, 0xa8, 0xfd, 0x08, 0x8f, 0xab, 0xf1, 0x1f, 0x32)]
interface IIdleSettings(IIdleSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_IdleDuration(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_IdleDuration(
        pDelay: BSTR,
    ) -> HRESULT,
    fn get_WaitTimeout(
        pTimeout: *mut BSTR,
    ) -> HRESULT,
    fn put_WaitTimeout(
        pTimeout: BSTR,
    ) -> HRESULT,
    fn get_StopOnIdleEnd(
        pStop: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StopOnIdleEnd(
        pStop: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RestartOnIdle(
        pRestart: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RestartOnIdle(
        pRestart: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9f7dea84, 0xc30b, 0x4245, 0x80, 0xb6, 0x00, 0xe9, 0xf6, 0x46, 0xf1, 0xb4)]
interface INetworkSettings(INetworkSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pName: BSTR,
    ) -> HRESULT,
    fn get_Id(
        pId: *mut BSTR,
    ) -> HRESULT,
    fn put_Id(
        pId: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd98d51e5, 0xc9b4, 0x496a, 0xa9, 0xc1, 0x18, 0x98, 0x02, 0x61, 0xcf, 0x0f)]
interface IPrincipal(IPrincipalVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(
        pId: *mut BSTR,
    ) -> HRESULT,
    fn put_Id(
        pId: BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn put_DisplayName(
        pName: BSTR,
    ) -> HRESULT,
    fn get_UserId(
        pUser: *mut BSTR,
    ) -> HRESULT,
    fn put_UserId(
        pUser: BSTR,
    ) -> HRESULT,
    fn get_LogonType(
        pLogon: *mut TASK_LOGON_TYPE,
    ) -> HRESULT,
    fn put_LogonType(
        pLogon: TASK_LOGON_TYPE,
    ) -> HRESULT,
    fn get_GroupId(
        pGroup: *mut BSTR,
    ) -> HRESULT,
    fn put_GroupId(
        pGroup: BSTR,
    ) -> HRESULT,
    fn get_RunLevel(
        pRunLevel: *mut TASK_RUNLEVEL,
    ) -> HRESULT,
    fn put_RunLevel(
        pRunLevel: TASK_RUNLEVEL,
    ) -> HRESULT,
}}
ENUM!{enum TASK_LOGON_TYPE {
    TASK_LOGON_NONE = 0,
    TASK_LOGON_PASSWORD = 1,
    TASK_LOGON_S4U = 2,
    TASK_LOGON_INTERACTIVE_TOKEN = 3,
    TASK_LOGON_GROUP = 4,
    TASK_LOGON_SERVICE_ACCOUNT = 5,
    TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD = 6,
}}
ENUM!{enum TASK_RUNLEVEL {
    TASK_RUNLEVEL_LUA = 0,
    TASK_RUNLEVEL_HIGHEST = 1,
}}
RIDL!{#[uuid(0x02820e19, 0x7b98, 0x4ed2, 0xb2, 0xe8, 0xfd, 0xcc, 0xce, 0xff, 0x61, 0x9b)]
interface IActionCollection(IActionCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut c_long,
    ) -> HRESULT,
    fn get_Item(
        index: c_long,
        ppAction: *mut *mut IAction,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn get_XmlText(
        pText: *mut BSTR,
    ) -> HRESULT,
    fn put_XmlText(
        pText: BSTR,
    ) -> HRESULT,
    fn Create(
        Type: TASK_ACTION_TYPE,
        ppAction: *mut *mut IAction,
    ) -> HRESULT,
    fn Remove(
        index: VARIANT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_Context(
        pContext: *mut BSTR,
    ) -> HRESULT,
    fn put_Context(
        pContext: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbae54997, 0x48b1, 0x4cbe, 0x99, 0x65, 0xd6, 0xbe, 0x26, 0x3e, 0xbe, 0xa4)]
interface IAction(IActionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Id(
        pId: *mut BSTR,
    ) -> HRESULT,
    fn put_Id(
        pId: BSTR,
    ) -> HRESULT,
    fn get_Type(
        pType: *mut TASK_ACTION_TYPE,
    ) -> HRESULT,
}}
ENUM!{enum TASK_ACTION_TYPE {
    TASK_ACTION_EXEC = 0,
    TASK_ACTION_COM_HANDLER = 5,
    TASK_ACTION_SEND_EMAIL = 6,
    TASK_ACTION_SHOW_MESSAGE = 7,
}}
RIDL!{#[uuid(0x86627eb4, 0x42a7, 0x41e4, 0xa4, 0xd9, 0xac, 0x33, 0xa7, 0x2f, 0x2d, 0x52)]
interface IRegisteredTaskCollection(IRegisteredTaskCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut LONG,
    ) -> HRESULT,
    fn get_Item(
        index: VARIANT,
        ppRegisteredTask: *mut *mut IRegisteredTask,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2faba4c7, 0x4da9, 0x4013, 0x96, 0x97, 0x20, 0xcc, 0x3f, 0xd4, 0x0f, 0x85)]
interface ITaskService(ITaskServiceVtbl): IDispatch(IDispatchVtbl) {
    fn GetFolder(
        Path: BSTR,
        ppFolder: *mut *mut ITaskFolder,
    ) -> HRESULT,
    fn GetRunningTasks(
        flags: LONG,
        ppRunningTasks: *mut *mut IRunningTaskCollection,
    ) -> HRESULT,
    fn NewTask(
        flags: DWORD,
        ppDefinition: *mut *mut ITaskDefinition,
    ) -> HRESULT,
    fn Connect(
        serverName: VARIANT,
        user: VARIANT,
        domain: VARIANT,
        password: VARIANT,
    ) -> HRESULT,
    fn get_Connected(
        pConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TargetServer(
        pServer: *mut BSTR,
    ) -> HRESULT,
    fn get_ConnectedUser(
        pUser: *mut BSTR,
    ) -> HRESULT,
    fn get_ConnectedDomain(
        pDomain: *mut BSTR,
    ) -> HRESULT,
    fn get_HighestVersion(
        pVersion: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x839d7762, 0x5121, 0x4009, 0x92, 0x34, 0x4f, 0x0d, 0x19, 0x39, 0x4f, 0x04)]
interface ITaskHandler(ITaskHandlerVtbl): IUnknown(IUnknownVtbl) {
    fn Start(
        pHandlerServices: LPUNKNOWN,
        Data: BSTR,
    ) -> HRESULT,
    fn Stop(
        pRetCode: *mut HRESULT,
    ) -> HRESULT,
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
}}
RIDL!{#[uuid(0xeaec7a8f, 0x27a0, 0x4ddc, 0x86, 0x75, 0x14, 0x72, 0x6a, 0x01, 0xa3, 0x8a)]
interface ITaskHandlerStatus(ITaskHandlerStatusVtbl): IUnknown(IUnknownVtbl) {
    fn UpdateStatus(
        percentComplete: c_short,
        statusMessage: BSTR,
    ) -> HRESULT,
    fn TaskCompleted(
        taskErrCode: HRESULT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3e4c9351, 0xd966, 0x4b8b, 0xbb, 0x87, 0xce, 0xba, 0x68, 0xbb, 0x01, 0x07)]
interface ITaskVariables(ITaskVariablesVtbl): IUnknown(IUnknownVtbl) {
    fn GetInput(
        pInput: *mut BSTR,
    ) -> HRESULT,
    fn SetOutput(
        input: BSTR,
    ) -> HRESULT,
    fn GetContext(
        pContext: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x39038068, 0x2b46, 0x4afd, 0x86, 0x62, 0x7b, 0xb6, 0xf8, 0x68, 0xd2, 0x21)]
interface ITaskNamedValuePair(ITaskNamedValuePairVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        pName: *mut BSTR,
    ) -> HRESULT,
    fn put_Name(
        pName: BSTR,
    ) -> HRESULT,
    fn get_Value(
        pValue: *mut BSTR,
    ) -> HRESULT,
    fn put_Value(
        pValue: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb4ef826b, 0x63c3, 0x46e4, 0xa5, 0x04, 0xef, 0x69, 0xe4, 0xf7, 0xea, 0x4d)]
interface ITaskNamedValueCollection(ITaskNamedValueCollectionVtbl): IDispatch(IDispatchVtbl) {
    fn get_Count(
        pCount: *mut c_long,
    ) -> HRESULT,
    fn get_Item(
        index: LONG,
        ppPair: *mut *mut ITaskNamedValuePair,
    ) -> HRESULT,
    fn get__NewEnum(
        ppEnum: *mut LPUNKNOWN,
    ) -> HRESULT,
    fn Create(
        Name: BSTR,
        Value: BSTR,
        ppPair: *mut *mut ITaskNamedValuePair,
    ) -> HRESULT,
    fn Remove(
        index: LONG,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
}}
RIDL!{#[uuid(0xd537d2b0, 0x9fb3, 0x4d34, 0x97, 0x39, 0x1f, 0xf5, 0xce, 0x7b, 0x1e, 0xf3)]
interface IIdleTrigger(IIdleTriggerVtbl): ITrigger(ITriggerVtbl) {}}
RIDL!{#[uuid(0x72dade38, 0xfae4, 0x4b3e, 0xba, 0xf4, 0x5d, 0x00, 0x9a, 0xf0, 0x2b, 0x1c)]
interface ILogonTrigger(ILogonTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_Delay(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_Delay(
        pDelay: BSTR,
    ) -> HRESULT,
    fn get_UserId(
        pUser: *mut BSTR,
    ) -> HRESULT,
    fn put_UserId(
        pUser: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x754da71b, 0x4385, 0x4475, 0x9d, 0xd9, 0x59, 0x82, 0x94, 0xfa, 0x36, 0x41)]
interface ISessionStateChangeTrigger(ISessionStateChangeTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_Delay(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_Delay(
        pDelay: BSTR,
    ) -> HRESULT,
    fn get_UserId(
        pUser: *mut BSTR,
    ) -> HRESULT,
    fn put_UserId(
        pUser: BSTR,
    ) -> HRESULT,
    fn get_StateChange(
        pType: *mut TASK_SESSION_STATE_CHANGE_TYPE,
    ) -> HRESULT,
    fn put_StateChange(
        pType: TASK_SESSION_STATE_CHANGE_TYPE,
    ) -> HRESULT,
}}
ENUM!{enum TASK_SESSION_STATE_CHANGE_TYPE {
    TASK_CONSOLE_CONNECT = 1,
    TASK_CONSOLE_DISCONNECT = 2,
    TASK_REMOTE_CONNECT = 3,
    TASK_REMOTE_DISCONNECT = 4,
    TASK_SESSION_LOCK = 7,
    TASK_SESSION_UNLOCK = 8,
}}
RIDL!{#[uuid(0xd45b0167, 0x9653, 0x4eef, 0xb9, 0x4f, 0x07, 0x32, 0xca, 0x7a, 0xf2, 0x51)]
interface IEventTrigger(IEventTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_Subscription(
        pQuery: *mut BSTR,
    ) -> HRESULT,
    fn put_Subscription(
        pQuery: BSTR,
    ) -> HRESULT,
    fn get_Delay(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_Delay(
        pDelay: BSTR,
    ) -> HRESULT,
    fn get_ValueQueries(
        ppNamedXPaths: *mut *mut ITaskNamedValueCollection,
    ) -> HRESULT,
    fn put_ValueQueries(
        ppNamedXPaths: *const ITaskNamedValueCollection,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb45747e0, 0xeba7, 0x4276, 0x9f, 0x29, 0x85, 0xc5, 0xbb, 0x30, 0x00, 0x06)]
interface ITimeTrigger(ITimeTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_RandomDelay(
        pRandomDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_RandomDelay(
        pRandomDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x126c5cd8, 0xb288, 0x41d5, 0x8d, 0xbf, 0xe4, 0x91, 0x44, 0x6a, 0xdc, 0x5c)]
interface IDailyTrigger(IDailyTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_DaysInterval(
        pDays: *mut c_short,
    ) -> HRESULT,
    fn put_DaysInterval(
        pDays: c_short,
    ) -> HRESULT,
    fn get_RandomDelay(
        pRandomDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_RandomDelay(
        pRandomDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5038fc98, 0x82ff, 0x436d, 0x87, 0x28, 0xa5, 0x12, 0xa5, 0x7c, 0x9d, 0xc1)]
interface IWeeklyTrigger(IWeeklyTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_DaysOfWeek(
        pDays: *mut c_short,
    ) -> HRESULT,
    fn put_DaysOfWeek(
        pDays: c_short,
    ) -> HRESULT,
    fn get_WeeksInterval(
        pWeeks: *mut c_short,
    ) -> HRESULT,
    fn put_WeeksInterval(
        pWeeks: c_short,
    ) -> HRESULT,
    fn get_RandomDelay(
        pRandomDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_RandomDelay(
        pRandomDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x97c45ef1, 0x6b02, 0x4a1a, 0x9c, 0x0e, 0x1e, 0xbf, 0xba, 0x15, 0x00, 0xac)]
interface IMonthlyTrigger(IMonthlyTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_DaysOfMonth(
        pDays: *mut c_long,
    ) -> HRESULT,
    fn put_DaysOfMonth(
        pDays: c_long,
    ) -> HRESULT,
    fn get_MonthsOfYear(
        pMonths: *mut c_short,
    ) -> HRESULT,
    fn put_MonthsOfYear(
        pMonths: c_short,
    ) -> HRESULT,
    fn get_RunOnLastDayOfMonth(
        pLastDay: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RunOnLastDayOfMonth(
        pLastDay: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RandomDelay(
        pRandomDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_RandomDelay(
        pRandomDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x77d025a3, 0x90fa, 0x43aa, 0xb5, 0x2e, 0xcd, 0xa5, 0x49, 0x9b, 0x94, 0x6a)]
interface IMonthlyDOWTrigger(IMonthlyDOWTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_DaysOfWeek(
        pDays: *mut c_short,
    ) -> HRESULT,
    fn put_DaysOfWeek(
        pDays: c_short,
    ) -> HRESULT,
    fn get_WeeksOfMonth(
        pWeeks: *mut c_short,
    ) -> HRESULT,
    fn put_WeeksOfMonth(
        pWeeks: c_short,
    ) -> HRESULT,
    fn get_MonthsOfYear(
        pMonths: *mut c_short,
    ) -> HRESULT,
    fn put_MonthsOfYear(
        pMonths: c_short,
    ) -> HRESULT,
    fn get_RunOnLastWeekOfMonth(
        pLastWeek: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RunOnLastWeekOfMonth(
        pLastWeek: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RandomDelay(
        pRandomDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_RandomDelay(
        pRandomDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2a9c35da, 0xd357, 0x41f4, 0xbb, 0xc1, 0x20, 0x7a, 0xc1, 0xb1, 0xf3, 0xcb)]
interface IBootTrigger(IBootTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_Delay(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_Delay(
        pDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4c8fec3a, 0xc218, 0x4e0c, 0xb2, 0x3d, 0x62, 0x90, 0x24, 0xdb, 0x91, 0xa2)]
interface IRegistrationTrigger(IRegistrationTriggerVtbl): ITrigger(ITriggerVtbl) {
    fn get_Delay(
        pDelay: *mut BSTR,
    ) -> HRESULT,
    fn put_Delay(
        pDelay: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4c3d624d, 0xfd6b, 0x49a3, 0xb9, 0xb7, 0x09, 0xcb, 0x3c, 0xd3, 0xf0, 0x47)]
interface IExecAction(IExecActionVtbl): IAction(IActionVtbl) {
    fn get_Path(
        pPath: *mut BSTR,
    ) -> HRESULT,
    fn put_Path(
        pPath: BSTR,
    ) -> HRESULT,
    fn get_Arguments(
        pArgument: *mut BSTR,
    ) -> HRESULT,
    fn put_Arguments(
        pArgument: BSTR,
    ) -> HRESULT,
    fn get_WorkingDirectory(
        pWorkingDirectory: *mut BSTR,
    ) -> HRESULT,
    fn put_WorkingDirectory(
        pWorkingDirectory: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf2a82542, 0xbda5, 0x4e6b, 0x91, 0x43, 0xe2, 0xbf, 0x4f, 0x89, 0x87, 0xb6)]
interface IExecAction2(IExecAction2Vtbl): IExecAction(IExecActionVtbl) {
    fn get_HideAppWindow(
        pHideAppWindow: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HideAppWindow(
        pHideAppWindow: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x505e9e68, 0xaf89, 0x46b8, 0xa3, 0x0f, 0x56, 0x16, 0x2a, 0x83, 0xd5, 0x37)]
interface IShowMessageAction(IShowMessageActionVtbl): IAction(IActionVtbl) {
    fn get_Title(
        pTitle: *mut BSTR,
    ) -> HRESULT,
    fn put_Title(
        pTitle: BSTR,
    ) -> HRESULT,
    fn get_MessageBody(
        pMessageBody: *mut BSTR,
    ) -> HRESULT,
    fn put_MessageBody(
        pMessageBody: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6d2fd252, 0x75c5, 0x4f66, 0x90, 0xba, 0x2a, 0x7d, 0x8c, 0xc3, 0x03, 0x9f)]
interface IComHandlerAction(IComHandlerActionVtbl): IAction(IActionVtbl) {
    fn get_ClassId(
        pClsid: *mut BSTR,
    ) -> HRESULT,
    fn put_ClassId(
        pClsid: BSTR,
    ) -> HRESULT,
    fn get_Data(
        pData: *mut BSTR,
    ) -> HRESULT,
    fn put_Data(
        pData: BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x10f62c64, 0x7e16, 0x4314, 0xa0, 0xc2, 0x0c, 0x36, 0x83, 0xf9, 0x9d, 0x40)]
interface IEmailAction(IEmailActionVtbl): IAction(IActionVtbl) {
    fn get_Server(
        pServer: *mut BSTR,
    ) -> HRESULT,
    fn put_Server(
        pServer: BSTR,
    ) -> HRESULT,
    fn get_Subject(
        pSubject: *mut BSTR,
    ) -> HRESULT,
    fn put_Subject(
        pSubject: BSTR,
    ) -> HRESULT,
    fn get_To(
        pTo: *mut BSTR,
    ) -> HRESULT,
    fn put_To(
        pTo: BSTR,
    ) -> HRESULT,
    fn get_Cc(
        pCc: *mut BSTR,
    ) -> HRESULT,
    fn put_Cc(
        pCc: BSTR,
    ) -> HRESULT,
    fn get_Bcc(
        pBcc: *mut BSTR,
    ) -> HRESULT,
    fn put_Bcc(
        pBcc: BSTR,
    ) -> HRESULT,
    fn get_ReplyTo(
        pReplyTo: *mut BSTR,
    ) -> HRESULT,
    fn put_ReplyTo(
        pReplyTo: BSTR,
    ) -> HRESULT,
    fn get_From(
        pFrom: *mut BSTR,
    ) -> HRESULT,
    fn put_From(
        pFrom: BSTR,
    ) -> HRESULT,
    fn get_HeaderFields(
        ppHeaderFields: *mut *mut ITaskNamedValueCollection,
    ) -> HRESULT,
    fn put_HeaderFields(
        ppHeaderFields: *const ITaskNamedValueCollection,
    ) -> HRESULT,
    fn get_Body(
        pBody: *mut BSTR,
    ) -> HRESULT,
    fn put_Body(
        pBody: BSTR,
    ) -> HRESULT,
    fn get_Attachments(
        pAttachements: *mut SAFEARRAY,
    ) -> HRESULT,
    fn put_Attachments(
        pAttachements: SAFEARRAY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x248919ae, 0xe345, 0x4a6d, 0x8a, 0xeb, 0xe0, 0xd3, 0x16, 0x5c, 0x90, 0x4e)]
interface IPrincipal2(IPrincipal2Vtbl): IDispatch(IDispatchVtbl) {
    fn get_ProcessTokenSidType(
        pProcessTokenSidType: *mut TASK_PROCESSTOKENSID,
    ) -> HRESULT,
    fn put_ProcessTokenSidType(
        pProcessTokenSidType: TASK_PROCESSTOKENSID,
    ) -> HRESULT,
    fn get_RequiredPrivilegeCount(
        pCount: *mut c_long,
    ) -> HRESULT,
    fn get_RequiredPrivilege(
        index: c_long,
        pPrivilege: *mut BSTR,
    ) -> HRESULT,
    fn AddRequiredPrivilege(
        privilege: BSTR,
    ) -> HRESULT,
}}
ENUM!{enum TASK_PROCESSTOKENSID {
    TASK_PROCESSTOKENSID_NONE = 0,
    TASK_PROCESSTOKENSID_UNRESTRICTED = 1,
    TASK_PROCESSTOKENSID_DEFAULT = 2,
}}
RIDL!{#[uuid(0x2c05c3f0, 0x6eed, 0x4c05, 0xa1, 0x5f, 0xed, 0x7d, 0x7a, 0x98, 0xa3, 0x69)]
interface ITaskSettings2(ITaskSettings2Vtbl): IDispatch(IDispatchVtbl) {
    fn get_DisallowStartOnRemoteAppSession(
        pDisallowStart: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DisallowStartOnRemoteAppSession(
        pDisallowStart: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseUnifiedSchedulingEngine(
        pUseUnifiedEngine: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseUnifiedSchedulingEngine(
        pUseUnifiedEngine: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0ad9d0d7, 0x0c7f, 0x4ebb, 0x9a, 0x5f, 0xd1, 0xc6, 0x48, 0xdc, 0xa5, 0x28)]
interface ITaskSettings3(ITaskSettings3Vtbl): ITaskSettings(ITaskSettingsVtbl) {
    fn get_DisallowStartOnRemoteAppSession(
        pDisallowStart: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DisallowStartOnRemoteAppSession(
        pDisallowStart: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UseUnifiedSchedulingEngine(
        pUseUnifiedEngine: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UseUnifiedSchedulingEngine(
        pUseUnifiedEngine: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_MaintenanceSettings(
        ppMaintenanceSettings: *mut *mut IMaintenanceSettings,
    ) -> HRESULT,
    fn put_MaintenanceSettings(
        ppMaintenanceSettings: *const IMaintenanceSettings,
    ) -> HRESULT,
    fn CreateMaintenanceSettings(
        ppMaintenanceSettings: *mut *mut IMaintenanceSettings,
    ) -> HRESULT,
    fn get_Volatile(
        pVolatile: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Volatile(
        pVolatile: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa6024fa8, 0x9652, 0x4adb, 0xa6, 0xbf, 0x5c, 0xfc, 0xd8, 0x77, 0xa7, 0xba)]
interface IMaintenanceSettings(IMaintenanceSettingsVtbl): IDispatch(IDispatchVtbl) {
    fn put_Period(
        target: BSTR,
    ) -> HRESULT,
    fn get_Period(
        target: *mut BSTR,
    ) -> HRESULT,
    fn put_Deadline(
        target: BSTR,
    ) -> HRESULT,
    fn get_Deadline(
        target: *mut BSTR,
    ) -> HRESULT,
    fn put_Exclusive(
        target: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Exclusive(
        target: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
ENUM!{enum TASK_RUN_FLAGS {
    TASK_RUN_NO_FLAGS = 0,
    TASK_RUN_AS_SELF = 1,
    TASK_RUN_IGNORE_CONSTRAINTS = 2,
    TASK_RUN_USE_SESSION_ID = 4,
    TASK_RUN_USER_SID = 8,
}}
ENUM!{enum TASK_ENUM_FLAGS {
    TASK_ENUM_HIDDEN = 1,
}}
ENUM!{enum TASK_CREATION {
    TASK_VALIDATE_ONLY = 1,
    TASK_CREATE = 2,
    TASK_UPDATE = 4,
    TASK_CREATE_OR_UPDATE = 6,
    TASK_DISABLE = 8,
    TASK_DONT_ADD_PRINCIPAL_ACE = 16,
    TASK_IGNORE_REGISTRATION_TRIGGERS = 32,
}}
