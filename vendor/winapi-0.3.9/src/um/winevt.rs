// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Windows Events API
use ctypes::{c_double, c_float};
use shared::basetsd::{INT16, INT32, INT64, INT8, UINT16, UINT32, UINT64, UINT8};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, FILETIME, PBYTE, PDWORD};
use um::minwinbase::SYSTEMTIME;
use um::winnt::{HANDLE, LCID, LONGLONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PSID, PVOID, ULONGLONG};
use vc::vcruntime::size_t;
pub type EVT_HANDLE = HANDLE;
pub type PEVT_HANDLE = *mut HANDLE;
ENUM!{enum EVT_VARIANT_TYPE {
    EvtVarTypeNull = 0,
    EvtVarTypeString = 1,
    EvtVarTypeAnsiString = 2,
    EvtVarTypeSByte = 3,
    EvtVarTypeByte = 4,
    EvtVarTypeInt16 = 5,
    EvtVarTypeUInt16 = 6,
    EvtVarTypeInt32 = 7,
    EvtVarTypeUInt32 = 8,
    EvtVarTypeInt64 = 9,
    EvtVarTypeUInt64 = 10,
    EvtVarTypeSingle = 11,
    EvtVarTypeDouble = 12,
    EvtVarTypeBoolean = 13,
    EvtVarTypeBinary = 14,
    EvtVarTypeGuid = 15,
    EvtVarTypeSizeT = 16,
    EvtVarTypeFileTime = 17,
    EvtVarTypeSysTime = 18,
    EvtVarTypeSid = 19,
    EvtVarTypeHexInt32 = 20,
    EvtVarTypeHexInt64 = 21,
    EvtVarTypeEvtHandle = 32,
    EvtVarTypeEvtXml = 35,
}}
pub const EVT_VARIANT_TYPE_MASK: DWORD = 0x7f;
pub const EVT_VARIANT_TYPE_ARRAY: DWORD = 128;
UNION!{union EVT_VARIANT_u {
    [u64; 1],
    BooleanVal BooleanVal_mut: BOOL,
    SByteVal SByteVal_mut: INT8,
    Int16Val Int16Val_mut: INT16,
    Int32Val Int32Val_mut: INT32,
    Int64Val Int64Val_mut: INT64,
    ByteVal ByteVal_mut: UINT8,
    UInt16Val UInt16Val_mut: UINT16,
    UInt32Val UInt32Val_mut: UINT32,
    UInt64Val UInt64Val_mut: UINT64,
    SingleVal SingleVal_mut: c_float,
    DoubleVal DoubleVal_mut: c_double,
    FileTimeVal FileTimeVal_mut: ULONGLONG,
    SysTimeVal SysTimeVal_mut: *mut SYSTEMTIME,
    GuidVal GuidVal_mut: *mut GUID,
    StringVal StringVal_mut: LPCWSTR,
    AnsiStringVal AnsiStringVal_mut: LPCSTR,
    BinaryVal BinaryVal_mut: PBYTE,
    SidVal SidVal_mut: PSID,
    SizeTVal SizeTVal_mut: size_t,
    BooleanArr BooleanArr_mut: *mut BOOL,
    SByteArr SByteArr_mut: *mut INT8,
    Int16Arr Int16Arr_mut: *mut INT16,
    Int32Arr Int32Arr_mut: *mut INT32,
    Int64Arr Int64Arr_mut: *mut INT64,
    ByteArr ByteArr_mut: *mut UINT8,
    UInt16Arr UInt16Arr_mut: *mut UINT16,
    UInt32Arr UInt32Arr_mut: *mut UINT32,
    UInt64Arr UInt64Arr_mut: *mut UINT64,
    SingleArr SingleArr_mut: *mut c_float,
    DoubleArr DoubleArr_mut: *mut c_double,
    FileTimeArr FileTimeArr_mut: *mut FILETIME,
    SysTimeArr SysTimeArr_mut: *mut SYSTEMTIME,
    GuidArr GuidArr_mut: *mut GUID,
    StringArr StringArr_mut: *mut LPWSTR,
    AnsiStringArr AnsiStringArr_mut: *mut LPSTR,
    SidArr SidArr_mut: *mut PSID,
    SizeTArr SizeTArr_mut: *mut size_t,
    EvtHandleVal EvtHandleVal_mut: EVT_HANDLE,
    XmlVal XmlVal_mut: LPCWSTR,
    XmlValArr XmlValArr_mut: *mut LPCWSTR,
}}
STRUCT!{struct EVT_VARIANT {
    u: EVT_VARIANT_u,
    Count: DWORD,
    Type: DWORD,
}}
pub type PEVT_VARIANT = *mut EVT_VARIANT;
ENUM!{enum EVT_LOGIN_CLASS {
    EvtRpcLogin = 1,
}}
ENUM!{enum EVT_RPC_LOGIN_FLAGS {
    EvtRpcLoginAuthDefault = 0,
    EvtRpcLoginAuthNegotiate,
    EvtRpcLoginAuthKerberos,
    EvtRpcLoginAuthNTLM,
}}
STRUCT!{struct EVT_RPC_LOGIN {
    Server: LPWSTR,
    User: LPWSTR,
    Domain: LPWSTR,
    Password: LPWSTR,
    Flags: DWORD,
}}
extern "system" {
    pub fn EvtOpenSession(
        LoginClass: EVT_LOGIN_CLASS,
        Login: PVOID,
        Timeout: DWORD,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtClose(
        Object: EVT_HANDLE,
    ) -> BOOL;
    pub fn EvtCancel(
        Object: EVT_HANDLE,
    ) -> BOOL;
    pub fn EvtGetExtendedStatus(
        BufferSize: DWORD,
        Buffer: LPWSTR,
        BufferUsed: PDWORD,
    ) -> DWORD;
}
ENUM!{enum EVT_QUERY_FLAGS {
    EvtQueryChannelPath = 0x1,
    EvtQueryFilePath = 0x2,
    EvtQueryForwardDirection = 0x100,
    EvtQueryReverseDirection = 0x200,
    EvtQueryTolerateQueryErrors = 0x1000,
}}
ENUM!{enum EVT_SEEK_FLAGS {
    EvtSeekRelativeToFirst = 1,
    EvtSeekRelativeToLast = 2,
    EvtSeekRelativeToCurrent = 3,
    EvtSeekRelativeToBookmark = 4,
    EvtSeekOriginMask = 7,
    EvtSeekStrict = 0x10000,
}}
extern "system" {
    pub fn EvtQuery(
        Session: EVT_HANDLE,
        Path: LPCWSTR,
        Query: LPCWSTR,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtNext(
        ResultSet: EVT_HANDLE,
        EventsSize: DWORD,
        Events: PEVT_HANDLE,
        Timeout: DWORD,
        Flags: DWORD,
        Returned: PDWORD,
    ) -> BOOL;
    pub fn EvtSeek(
        ResultSet: EVT_HANDLE,
        Position: LONGLONG,
        Bookmark: EVT_HANDLE,
        Timeout: DWORD,
        Flags: DWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_SUBSCRIBE_FLAGS {
    EvtSubscribeToFutureEvents = 1,
    EvtSubscribeStartAtOldestRecord = 2,
    EvtSubscribeStartAfterBookmark = 3,
    EvtSubscribeOriginMask = 3,
    EvtSubscribeTolerateQueryErrors = 0x1000,
    EvtSubscribeStrict = 0x10000,
}}
ENUM!{enum EVT_SUBSCRIBE_NOTIFY_ACTION {
    EvtSubscribeActionError = 0,
    EvtSubscribeActionDeliver,
}}
FN!{stdcall EVT_SUBSCRIBE_CALLBACK(
    Action: EVT_SUBSCRIBE_NOTIFY_ACTION,
    UserContext: PVOID,
    Event: EVT_HANDLE,
) -> DWORD}
extern "system" {
    pub fn EvtSubscribe(
        Session: EVT_HANDLE,
        SignalEvent: HANDLE,
        ChannelPath: LPCWSTR,
        Query: LPCWSTR,
        Bookmark: EVT_HANDLE,
        Context: PVOID,
        Callback: EVT_SUBSCRIBE_CALLBACK,
        Flags: DWORD,
    ) -> EVT_HANDLE;
}
ENUM!{enum EVT_SYSTEM_PROPERTY_ID {
    EvtSystemProviderName = 0,
    EvtSystemProviderGuid,
    EvtSystemEventID,
    EvtSystemQualifiers,
    EvtSystemLevel,
    EvtSystemTask,
    EvtSystemOpcode,
    EvtSystemKeywords,
    EvtSystemTimeCreated,
    EvtSystemEventRecordId,
    EvtSystemActivityID,
    EvtSystemRelatedActivityID,
    EvtSystemProcessID,
    EvtSystemThreadID,
    EvtSystemChannel,
    EvtSystemComputer,
    EvtSystemUserID,
    EvtSystemVersion,
    EvtSystemPropertyIdEND,
}}
ENUM!{enum EVT_RENDER_CONTEXT_FLAGS {
    EvtRenderContextValues = 0,
    EvtRenderContextSystem,
    EvtRenderContextUser,
}}
ENUM!{enum EVT_RENDER_FLAGS {
    EvtRenderEventValues = 0,
    EvtRenderEventXml,
    EvtRenderBookmark,
}}
extern "system" {
    pub fn EvtCreateRenderContext(
        ValuePathsCount: DWORD,
        ValuePaths: *mut LPCWSTR,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtRender(
        Context: EVT_HANDLE,
        Fragment: EVT_HANDLE,
        Flags: DWORD,
        BufferSize: DWORD,
        Buffer: PVOID,
        BufferUsed: PDWORD,
        PropertyCount: PDWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_FORMAT_MESSAGE_FLAGS {
    EvtFormatMessageEvent = 1,
    EvtFormatMessageLevel,
    EvtFormatMessageTask,
    EvtFormatMessageOpcode,
    EvtFormatMessageKeyword,
    EvtFormatMessageChannel,
    EvtFormatMessageProvider,
    EvtFormatMessageId,
    EvtFormatMessageXml,
}}
extern "system" {
    pub fn EvtFormatMessage(
        PublisherMetadata: EVT_HANDLE,
        Event: EVT_HANDLE,
        MessageId: DWORD,
        ValueCount: DWORD,
        Values: PEVT_VARIANT,
        Flags: DWORD,
        BufferSize: DWORD,
        Buffer: LPWSTR,
        BufferUsed: PDWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_OPEN_LOG_FLAGS {
    EvtOpenChannelPath = 0x1,
    EvtOpenFilePath = 0x2,
}}
ENUM!{enum EVT_LOG_PROPERTY_ID {
    EvtLogCreationTime = 0,
    EvtLogLastAccessTime,
    EvtLogLastWriteTime,
    EvtLogFileSize,
    EvtLogAttributes,
    EvtLogNumberOfLogRecords,
    EvtLogOldestRecordNumber,
    EvtLogFull,
}}
extern "system" {
    pub fn EvtOpenLog(
        Session: EVT_HANDLE,
        Path: LPCWSTR,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtGetLogInfo(
        Log: EVT_HANDLE,
        PropertyId: EVT_LOG_PROPERTY_ID,
        PropertyValueBufferSize: DWORD,
        PropertyValueBuffer: PEVT_VARIANT,
        PropertyValueBufferUsed: PDWORD,
    ) -> BOOL;
    pub fn EvtClearLog(
        Session: EVT_HANDLE,
        ChannelPath: LPCWSTR,
        TargetFilePath: LPCWSTR,
        Flags: DWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_EXPORTLOG_FLAGS {
    EvtExportLogChannelPath = 0x1,
    EvtExportLogFilePath = 0x2,
    EvtExportLogTolerateQueryErrors = 0x1000,
    EvtExportLogOverwrite = 0x2000,
}}
extern "system" {
    pub fn EvtExportLog(
        Session: EVT_HANDLE,
        Path: LPCWSTR,
        Query: LPCWSTR,
        TargetFilePath: LPCWSTR,
        Flags: DWORD,
    ) -> BOOL;
    pub fn EvtArchiveExportedLog(
        Session: EVT_HANDLE,
        LogFilePath: LPCWSTR,
        Locale: LCID,
        Flags: DWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_CHANNEL_CONFIG_PROPERTY_ID {
    EvtChannelConfigEnabled = 0,
    EvtChannelConfigIsolation,
    EvtChannelConfigType,
    EvtChannelConfigOwningPublisher,
    EvtChannelConfigClassicEventlog,
    EvtChannelConfigAccess,
    EvtChannelLoggingConfigRetention,
    EvtChannelLoggingConfigAutoBackup,
    EvtChannelLoggingConfigMaxSize,
    EvtChannelLoggingConfigLogFilePath,
    EvtChannelPublishingConfigLevel,
    EvtChannelPublishingConfigKeywords,
    EvtChannelPublishingConfigControlGuid,
    EvtChannelPublishingConfigBufferSize,
    EvtChannelPublishingConfigMinBuffers,
    EvtChannelPublishingConfigMaxBuffers,
    EvtChannelPublishingConfigLatency,
    EvtChannelPublishingConfigClockType,
    EvtChannelPublishingConfigSidType,
    EvtChannelPublisherList,
    EvtChannelPublishingConfigFileMax,
    EvtChannelConfigPropertyIdEND,
}}
ENUM!{enum EVT_CHANNEL_TYPE {
    EvtChannelTypeAdmin = 0,
    EvtChannelTypeOperational,
    EvtChannelTypeAnalytic,
    EvtChannelTypeDebug,
}}
ENUM!{enum EVT_CHANNEL_ISOLATION_TYPE {
    EvtChannelIsolationTypeApplication = 0,
    EvtChannelIsolationTypeSystem,
    EvtChannelIsolationTypeCustom,
}}
ENUM!{enum EVT_CHANNEL_CLOCK_TYPE {
    EvtChannelClockTypeSystemTime = 0,
    EvtChannelClockTypeQPC,
}}
ENUM!{enum EVT_CHANNEL_SID_TYPE {
    EvtChannelSidTypeNone = 0,
    EvtChannelSidTypePublishing,
}}
extern "system" {
    pub fn EvtOpenChannelEnum(
        Session: EVT_HANDLE,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtNextChannelPath(
        ChannelEnum: EVT_HANDLE,
        ChannelPathBufferSize: DWORD,
        ChannelPathBuffer: LPWSTR,
        ChannelPathBufferUsed: PDWORD,
    ) -> BOOL;
    pub fn EvtOpenChannelConfig(
        Session: EVT_HANDLE,
        ChannelPath: LPCWSTR,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtSaveChannelConfig(
        ChannelConfig: EVT_HANDLE,
        Flags: DWORD,
    ) -> BOOL;
    pub fn EvtSetChannelConfigProperty(
        ChannelConfig: EVT_HANDLE,
        PropertyId: EVT_CHANNEL_CONFIG_PROPERTY_ID,
        Flags: DWORD,
        PropertyValue: PEVT_VARIANT,
    ) -> BOOL;
    pub fn EvtGetChannelConfigProperty(
        ChannelConfig: EVT_HANDLE,
        PropertyId: EVT_CHANNEL_CONFIG_PROPERTY_ID,
        Flags: DWORD,
        PropertyValueBufferSize: DWORD,
        PropertyValueBuffer: PEVT_VARIANT,
        PropertyValueBufferUsed: PDWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_CHANNEL_REFERENCE_FLAGS {
    EvtChannelReferenceImported = 0x1,
}}
ENUM!{enum EVT_PUBLISHER_METADATA_PROPERTY_ID {
    EvtPublisherMetadataPublisherGuid = 0,
    EvtPublisherMetadataResourceFilePath,
    EvtPublisherMetadataParameterFilePath,
    EvtPublisherMetadataMessageFilePath,
    EvtPublisherMetadataHelpLink,
    EvtPublisherMetadataPublisherMessageID,
    EvtPublisherMetadataChannelReferences,
    EvtPublisherMetadataChannelReferencePath,
    EvtPublisherMetadataChannelReferenceIndex,
    EvtPublisherMetadataChannelReferenceID,
    EvtPublisherMetadataChannelReferenceFlags,
    EvtPublisherMetadataChannelReferenceMessageID,
    EvtPublisherMetadataLevels,
    EvtPublisherMetadataLevelName,
    EvtPublisherMetadataLevelValue,
    EvtPublisherMetadataLevelMessageID,
    EvtPublisherMetadataTasks,
    EvtPublisherMetadataTaskName,
    EvtPublisherMetadataTaskEventGuid,
    EvtPublisherMetadataTaskValue,
    EvtPublisherMetadataTaskMessageID,
    EvtPublisherMetadataOpcodes,
    EvtPublisherMetadataOpcodeName,
    EvtPublisherMetadataOpcodeValue,
    EvtPublisherMetadataOpcodeMessageID,
    EvtPublisherMetadataKeywords,
    EvtPublisherMetadataKeywordName,
    EvtPublisherMetadataKeywordValue,
    EvtPublisherMetadataKeywordMessageID,
    EvtPublisherMetadataPropertyIdEND,
}}
extern "system" {
    pub fn EvtOpenPublisherEnum(
        Session: EVT_HANDLE,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtNextPublisherId(
        PublisherEnum: EVT_HANDLE,
        PublisherIdBufferSize: DWORD,
        PublisherIdBuffer: LPWSTR,
        PublisherIdBufferUsed: PDWORD,
    ) -> BOOL;
    pub fn EvtOpenPublisherMetadata(
        Session: EVT_HANDLE,
        PublisherId: LPCWSTR,
        LogFilePath: LPCWSTR,
        Locale: LCID,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtGetPublisherMetadataProperty(
        PublisherMetadata: EVT_HANDLE,
        PropertyId: EVT_PUBLISHER_METADATA_PROPERTY_ID,
        Flags: DWORD,
        PublisherMetadataPropertyBufferSize: DWORD,
        PublisherMetadataPropertyBuffer: PEVT_VARIANT,
        PublisherMetadataPropertyBufferUsed: PDWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_EVENT_METADATA_PROPERTY_ID {
    EventMetadataEventID,
    EventMetadataEventVersion,
    EventMetadataEventChannel,
    EventMetadataEventLevel,
    EventMetadataEventOpcode,
    EventMetadataEventTask,
    EventMetadataEventKeyword,
    EventMetadataEventMessageID,
    EventMetadataEventTemplate,
    EvtEventMetadataPropertyIdEND,
}}
extern "system" {
    pub fn EvtOpenEventMetadataEnum(
        PublisherMetadata: EVT_HANDLE,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtNextEventMetadata(
        EventMetadataEnum: EVT_HANDLE,
        Flags: DWORD,
    ) -> EVT_HANDLE;
    pub fn EvtGetEventMetadataProperty(
        EventMetadata: EVT_HANDLE,
        PropertyId: EVT_EVENT_METADATA_PROPERTY_ID,
        Flags: DWORD,
        EventMetadataPropertyBufferSize: DWORD,
        EventMetadataPropertyBuffer: PEVT_VARIANT,
        EventMetadataPropertyBufferUsed: PDWORD,
    ) -> BOOL;
}
pub type EVT_OBJECT_ARRAY_PROPERTY_HANDLE = HANDLE;
extern "system" {
    pub fn EvtGetObjectArraySize(
        ObjectArray: EVT_OBJECT_ARRAY_PROPERTY_HANDLE,
        ObjectArraySize: PDWORD,
    ) -> BOOL;
    pub fn EvtGetObjectArrayProperty(
        ObjectArray: EVT_OBJECT_ARRAY_PROPERTY_HANDLE,
        PropertyId: DWORD,
        ArrayIndex: DWORD,
        Flags: DWORD,
        PropertyValueBufferSize: DWORD,
        PropertyValueBuffer: PEVT_VARIANT,
        PropertyValueBufferUsed: PDWORD,
    ) -> BOOL;
}
ENUM!{enum EVT_QUERY_PROPERTY_ID {
    EvtQueryNames,
    EvtQueryStatuses,
    EvtQueryPropertyIdEND,
}}
ENUM!{enum EVT_EVENT_PROPERTY_ID {
    EvtEventQueryIDs = 0,
    EvtEventPath,
    EvtEventPropertyIdEND,
}}
extern "system" {
    pub fn EvtGetQueryInfo(
        QueryOrSubscription: EVT_HANDLE,
        PropertyId: EVT_QUERY_PROPERTY_ID,
        PropertyValueBufferSize: DWORD,
        PropertyValueBuffer: PEVT_VARIANT,
        PropertyValueBufferUsed: PDWORD,
    ) -> BOOL;
    pub fn EvtCreateBookmark(
        BookmarkXml: LPCWSTR,
    ) -> EVT_HANDLE;
    pub fn EvtUpdateBookmark(
        Bookmark: EVT_HANDLE,
        Event: EVT_HANDLE,
    ) -> BOOL;
    pub fn EvtGetEventInfo(
        Event: EVT_HANDLE,
        PropertyId: EVT_EVENT_PROPERTY_ID,
        PropertyValueBufferSize: DWORD,
        PropertyValueBuffer: PEVT_VARIANT,
        PropertyValueBufferUsed: PDWORD,
    ) -> BOOL;
}
pub const EVT_READ_ACCESS: DWORD = 0x1;
pub const EVT_WRITE_ACCESS: DWORD = 0x2;
pub const EVT_CLEAR_ACCESS: DWORD = 0x4;
pub const EVT_ALL_ACCESS: DWORD = 0x7;
