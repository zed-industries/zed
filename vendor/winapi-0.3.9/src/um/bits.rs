// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::guiddef::{GUID, REFGUID};
use shared::minwindef::{BOOL, DWORD, FILETIME, ULONG};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, WCHAR};
RIDL!{#[uuid(0x4991d34b, 0x80a1, 0x4291, 0x83, 0xb6, 0x33, 0x28, 0x36, 0x6b, 0x90, 0x97)]
class BackgroundCopyManager;}
pub const BG_SIZE_UNKNOWN: UINT64 = -1i64 as u64;
STRUCT!{struct BG_FILE_PROGRESS {
    BytesTotal: UINT64,
    BytesTransferred: UINT64,
    Completed: BOOL,
}}
RIDL!{#[uuid(0x01b7bd23, 0xfb88, 0x4a77, 0x84, 0x90, 0x58, 0x91, 0xd3, 0xe4, 0x65, 0x3a)]
interface IBackgroundCopyFile(IBackgroundCopyFileVtbl): IUnknown(IUnknownVtbl) {
    fn GetRemoteName(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn GetLocalName(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn GetProgress(
        pVal: *mut BG_FILE_PROGRESS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xca51e165, 0xc365, 0x424c, 0x8d, 0x41, 0x24, 0xaa, 0xa4, 0xff, 0x3c, 0x40)]
interface IEnumBackgroundCopyFiles(IEnumBackgroundCopyFilesVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IBackgroundCopyFile,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumBackgroundCopyFiles,
    ) -> HRESULT,
    fn GetCount(
        puCount: *mut ULONG,
    ) -> HRESULT,
}}
ENUM!{enum BG_ERROR_CONTEXT {
    BG_ERROR_CONTEXT_NONE = 0,
    BG_ERROR_CONTEXT_UNKNOWN = 1,
    BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER = 2,
    BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION = 3,
    BG_ERROR_CONTEXT_LOCAL_FILE = 4,
    BG_ERROR_CONTEXT_REMOTE_FILE = 5,
    BG_ERROR_CONTEXT_GENERAL_TRANSPORT = 6,
    BG_ERROR_CONTEXT_REMOTE_APPLICATION = 7,
}}
RIDL!{#[uuid(0x19c613a0, 0xfcb8, 0x4f28, 0x81, 0xae, 0x89, 0x7c, 0x3d, 0x07, 0x8f, 0x81)]
interface IBackgroundCopyError(IBackgroundCopyErrorVtbl): IUnknown(IUnknownVtbl) {
    fn GetError(
        pContext: *mut BG_ERROR_CONTEXT,
        pCode: *mut HRESULT,
    ) -> HRESULT,
    fn GetFile(
        pVal: *mut *mut IBackgroundCopyFile,
    ) -> HRESULT,
    fn GetErrorDescription(
        LanguageId: DWORD,
        pErrorDescription: *mut LPWSTR,
    ) -> HRESULT,
    fn GetErrorContextDescription(
        LanguageId: DWORD,
        pContextDescription: *mut LPWSTR,
    ) -> HRESULT,
    fn GetProtocol(
        pProtocol: *mut LPWSTR,
    ) -> HRESULT,
}}
STRUCT!{struct BG_FILE_INFO {
    RemoteName: LPWSTR,
    LocalName: LPWSTR,
}}
STRUCT!{struct BG_JOB_PROGRESS {
    BytesTotal: UINT64,
    BytesTransferred: UINT64,
    FilesTotal: ULONG,
    FilesTransferred: ULONG,
}}
STRUCT!{struct BG_JOB_TIMES {
    CreationTime: FILETIME,
    ModificationTime: FILETIME,
    TransferCompletionTime: FILETIME,
}}
ENUM!{enum BG_JOB_PRIORITY {
    BG_JOB_PRIORITY_FOREGROUND = 0,
    BG_JOB_PRIORITY_HIGH = BG_JOB_PRIORITY_FOREGROUND + 1,
    BG_JOB_PRIORITY_NORMAL = BG_JOB_PRIORITY_HIGH + 1,
    BG_JOB_PRIORITY_LOW = BG_JOB_PRIORITY_NORMAL + 1,
}}
ENUM!{enum BG_JOB_STATE {
    BG_JOB_STATE_QUEUED = 0,
    BG_JOB_STATE_CONNECTING = BG_JOB_STATE_QUEUED + 1,
    BG_JOB_STATE_TRANSFERRING = BG_JOB_STATE_CONNECTING + 1,
    BG_JOB_STATE_SUSPENDED = BG_JOB_STATE_TRANSFERRING + 1,
    BG_JOB_STATE_ERROR = BG_JOB_STATE_SUSPENDED + 1,
    BG_JOB_STATE_TRANSIENT_ERROR = BG_JOB_STATE_ERROR + 1,
    BG_JOB_STATE_TRANSFERRED = BG_JOB_STATE_TRANSIENT_ERROR + 1,
    BG_JOB_STATE_ACKNOWLEDGED = BG_JOB_STATE_TRANSFERRED + 1,
    BG_JOB_STATE_CANCELLED = BG_JOB_STATE_ACKNOWLEDGED + 1,
}}
ENUM!{enum BG_JOB_TYPE {
    BG_JOB_TYPE_DOWNLOAD = 0,
    BG_JOB_TYPE_UPLOAD = BG_JOB_TYPE_DOWNLOAD + 1,
    BG_JOB_TYPE_UPLOAD_REPLY = BG_JOB_TYPE_UPLOAD + 1,
}}
ENUM!{enum BG_JOB_PROXY_USAGE {
    BG_JOB_PROXY_USAGE_PRECONFIG = 0,
    BG_JOB_PROXY_USAGE_NO_PROXY = BG_JOB_PROXY_USAGE_PRECONFIG + 1,
    BG_JOB_PROXY_USAGE_OVERRIDE = BG_JOB_PROXY_USAGE_NO_PROXY + 1,
    BG_JOB_PROXY_USAGE_AUTODETECT = BG_JOB_PROXY_USAGE_OVERRIDE + 1,
}}
RIDL!{#[uuid(0x37668d37, 0x507e, 0x4160, 0x93, 0x16, 0x26, 0x30, 0x6d, 0x15, 0x0b, 0x12)]
interface IBackgroundCopyJob(IBackgroundCopyJobVtbl): IUnknown(IUnknownVtbl) {
    fn AddFileSet(
        cFileCount: ULONG,
        pFileSet: *mut BG_FILE_INFO,
    ) -> HRESULT,
    fn AddFile(
        RemoteUrl: LPCWSTR,
        LocalName: LPCWSTR,
    ) -> HRESULT,
    fn EnumFiles(
        pErrorDescription: *mut *mut IEnumBackgroundCopyFiles,
    ) -> HRESULT,
    fn Suspend() -> HRESULT,
    fn Resume() -> HRESULT,
    fn Cancel() -> HRESULT,
    fn Complete() -> HRESULT,
    fn GetId(
        pVal: *mut GUID,
    ) -> HRESULT,
    fn GetType(
        pVal: *mut BG_JOB_TYPE,
    ) -> HRESULT,
    fn GetProgress(
        pVal: *mut BG_JOB_PROGRESS,
    ) -> HRESULT,
    fn GetTimes(
        pVal: *mut BG_JOB_TIMES,
    ) -> HRESULT,
    fn GetState(
        pVal: *mut BG_JOB_STATE,
    ) -> HRESULT,
    fn GetError(
        ppError: *mut *mut IBackgroundCopyError,
    ) -> HRESULT,
    fn GetOwner(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn SetDisplayName(
        Val: LPCWSTR,
    ) -> HRESULT,
    fn GetDisplayName(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn SetDescription(
        Val: LPCWSTR,
    ) -> HRESULT,
    fn GetDescription(
        pVal: *mut LPWSTR,
    ) -> HRESULT,
    fn SetPriority(
        Val: BG_JOB_PRIORITY,
    ) -> HRESULT,
    fn GetPriority(
        pVal: *mut BG_JOB_PRIORITY,
    ) -> HRESULT,
    fn SetNotifyFlags(
        Val: ULONG,
    ) -> HRESULT,
    fn GetNotifyFlags(
        pVal: *mut ULONG,
    ) -> HRESULT,
    fn SetNotifyInterface(
        Val: *mut IUnknown,
    ) -> HRESULT,
    fn GetNotifyInterface(
        pVal: *mut *mut IUnknown,
    ) -> HRESULT,
    fn SetMinimumRetryDelay(
        Seconds: ULONG,
    ) -> HRESULT,
    fn GetMinimumRetryDelay(
        Seconds: *mut ULONG,
    ) -> HRESULT,
    fn SetNoProgressTimeout(
        Seconds: ULONG,
    ) -> HRESULT,
    fn GetNoProgressTimeout(
        Seconds: *mut ULONG,
    ) -> HRESULT,
    fn GetErrorCount(
        Errors: *mut ULONG,
    ) -> HRESULT,
    fn SetProxySettings(
        ProxyUsage: BG_JOB_PROXY_USAGE,
        ProxyList: *const WCHAR,
        ProxyBypassList: *const WCHAR,
    ) -> HRESULT,
    fn GetProxySettings(
        pProxyUsage: *mut BG_JOB_PROXY_USAGE,
        pProxyList: *mut LPWSTR,
        pProxyBypassListpProxyList: *mut LPWSTR,
    ) -> HRESULT,
    fn TakeOwnership() -> HRESULT,
}}
RIDL!{#[uuid(0x1af4f612, 0x3b71, 0x466f, 0x8f, 0x58, 0x7b, 0x6f, 0x73, 0xac, 0x57, 0xad)]
interface IEnumBackgroundCopyJobs(IEnumBackgroundCopyJobsVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut IBackgroundCopyJob,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumBackgroundCopyJobs,
    ) -> HRESULT,
    fn GetCount(
        puCount: *mut ULONG,
    ) -> HRESULT,
}}
pub const BG_NOTIFY_JOB_TRANSFERRED: DWORD = 0x0001;
pub const BG_NOTIFY_JOB_ERROR: DWORD = 0x0002;
pub const BG_NOTIFY_DISABLE: DWORD = 0x0004;
pub const BG_NOTIFY_JOB_MODIFICATION: DWORD = 0x0008;
pub const BG_NOTIFY_FILE_TRANSFERRED: DWORD = 0x0010;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: DWORD = 0x0020;
RIDL!{#[uuid(0x97ea99c7, 0x0186, 0x4ad4, 0x8d, 0xf9, 0xc5, 0xb4, 0xe0, 0xed, 0x6b, 0x22)]
interface IBackgroundCopyCallback(IBackgroundCopyCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn JobTransferred(
        pJob: *mut IBackgroundCopyJob,
    ) -> HRESULT,
    fn JobError(
        pJob: *mut IBackgroundCopyJob,
        pError: *mut IBackgroundCopyError,
    ) -> HRESULT,
    fn JobModification(
        pJob: *mut IBackgroundCopyJob,
        dwReserved: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xca29d251, 0xb4bb, 0x4679, 0xa3, 0xd9, 0xae, 0x80, 0x06, 0x11, 0x9d, 0x54)]
interface AsyncIBackgroundCopyCallback(AsyncIBackgroundCopyCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_JobTransferred(
        pJob: *mut IBackgroundCopyJob,
    ) -> HRESULT,
    fn Finish_JobTransferred() -> HRESULT,
    fn Begin_JobError(
        pJob: *mut IBackgroundCopyJob,
        pError: *mut IBackgroundCopyError,
    ) -> HRESULT,
    fn Finish_JobError() -> HRESULT,
    fn Begin_JobModification(
        pJob: *mut IBackgroundCopyJob,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Finish_JobModification() -> HRESULT,
}}
pub const BG_JOB_ENUM_ALL_USERS: DWORD = 0x0001;
RIDL!{#[uuid(0x5ce34c0d, 0x0dc9, 0x4c1f, 0x89, 0x7c, 0xda, 0xa1, 0xb7, 0x8c, 0xee, 0x7c)]
interface IBackgroundCopyManager(IBackgroundCopyManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CreateJob(
        DisplayName: LPCWSTR,
        Type: BG_JOB_TYPE,
        pJobId: *mut GUID,
        ppJob: *mut *mut IBackgroundCopyJob,
    ) -> HRESULT,
    fn GetJob(
        jobID: REFGUID,
        ppJob: *mut *mut IBackgroundCopyJob,
    ) -> HRESULT,
    fn EnumJobs(
        dwFlags: DWORD,
        ppEnum: *mut *mut IEnumBackgroundCopyJobs,
    ) -> HRESULT,
    fn GetErrorDescription(
        hResult: HRESULT,
        LanguageId: DWORD,
        pErrorDescription: *mut LPWSTR,
    ) -> HRESULT,
}}
