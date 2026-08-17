// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! HTTP API specification
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, PUCHAR, PULONG, UCHAR, ULONG, USHORT};
use shared::sspi::SECURITY_STATUS;
use shared::ws2def::{PSOCKADDR, SOCKADDR_STORAGE};
use um::minwinbase::{LPOVERLAPPED, PSECURITY_ATTRIBUTES};
use um::winnt::{
    ANYSIZE_ARRAY, BOOLEAN, HANDLE, PCHAR, PCSTR, PCWSTR, PHANDLE, PSECURITY_DESCRIPTOR, PVOID,
    PWCHAR, PWSTR, ULARGE_INTEGER, ULONGLONG,
};
pub const HTTP_INITIALIZE_SERVER: ULONG = 0x00000001;
pub const HTTP_INITIALIZE_CONFIG: ULONG = 0x00000002;
pub const HTTP_DEMAND_CBT: ULONG = 0x00000004;
ENUM!{enum HTTP_SERVER_PROPERTY {
    HttpServerAuthenticationProperty,
    HttpServerLoggingProperty,
    HttpServerQosProperty,
    HttpServerTimeoutsProperty,
    HttpServerQueueLengthProperty,
    HttpServerStateProperty,
    HttpServer503VerbosityProperty,
    HttpServerBindingProperty,
    HttpServerExtendedAuthenticationProperty,
    HttpServerListenEndpointProperty,
    HttpServerChannelBindProperty,
    HttpServerProtectionLevelProperty,
}}
pub type PHTTP_SERVER_PROPERTY = *mut HTTP_SERVER_PROPERTY;
STRUCT!{struct HTTP_PROPERTY_FLAGS {
    BitFields: ULONG,
}}
BITFIELD!{HTTP_PROPERTY_FLAGS BitFields: ULONG [
    Present set_Present[0..1],
]}
pub type PHTTP_PROPERTY_FLAGS = *mut HTTP_PROPERTY_FLAGS;
ENUM!{enum HTTP_ENABLED_STATE {
    HttpEnabledStateActive,
    HttpEnabledStateInactive,
}}
pub type PHTTP_ENABLED_STATE = *mut HTTP_ENABLED_STATE;
STRUCT!{struct HTTP_STATE_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    State: HTTP_ENABLED_STATE,
}}
pub type PHTTP_STATE_INFO = *mut HTTP_STATE_INFO;
ENUM!{enum HTTP_503_RESPONSE_VERBOSITY {
    Http503ResponseVerbosityBasic,
    Http503ResponseVerbosityLimited,
    Http503ResponseVerbosityFull,
}}
pub type PHTTP_503_RESPONSE_VERBOSITY = *mut HTTP_503_RESPONSE_VERBOSITY;
ENUM!{enum HTTP_QOS_SETTING_TYPE {
    HttpQosSettingTypeBandwidth,
    HttpQosSettingTypeConnectionLimit,
    HttpQosSettingTypeFlowRate,
}}
pub type PHTTP_QOS_SETTING_TYPE = *mut HTTP_QOS_SETTING_TYPE;
STRUCT!{struct HTTP_QOS_SETTING_INFO {
    QosType: HTTP_QOS_SETTING_TYPE,
    QosSetting: PVOID,
}}
pub type PHTTP_QOS_SETTING_INFO = *mut HTTP_QOS_SETTING_INFO;
STRUCT!{struct HTTP_CONNECTION_LIMIT_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    MaxConnections: ULONG,
}}
pub type PHTTP_CONNECTION_LIMIT_INFO = *mut HTTP_CONNECTION_LIMIT_INFO;
STRUCT!{struct HTTP_BANDWIDTH_LIMIT_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    MaxBandwidth: ULONG,
}}
pub type PHTTP_BANDWIDTH_LIMIT_INFO = *mut HTTP_BANDWIDTH_LIMIT_INFO;
STRUCT!{struct HTTP_FLOWRATE_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    MaxBandwidth: ULONG,
    MaxPeakBandwidth: ULONG,
    BurstSize: ULONG,
}}
pub type PHTTP_FLOWRATE_INFO = *mut HTTP_FLOWRATE_INFO;
pub const HTTP_MIN_ALLOWED_BANDWIDTH_THROTTLING_RATE: ULONG = 1024;
pub const HTTP_LIMIT_INFINITE: ULONG = !0;
ENUM!{enum HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    IdleConnectionTimeout = 0,
    HeaderWaitTimeout,
}}
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_KEY = *mut HTTP_SERVICE_CONFIG_TIMEOUT_KEY;
pub type HTTP_SERVICE_CONFIG_TIMEOUT_PARAM = USHORT;
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_PARAM = *mut USHORT;
STRUCT!{struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_TIMEOUT_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_TIMEOUT_SET = *mut HTTP_SERVICE_CONFIG_TIMEOUT_SET;
STRUCT!{struct HTTP_TIMEOUT_LIMIT_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    EntityBody: USHORT,
    DrainEntityBody: USHORT,
    RequestQueue: USHORT,
    IdleConnection: USHORT,
    HeaderWait: USHORT,
    MinSendRate: ULONG,
}}
pub type PHTTP_TIMEOUT_LIMIT_INFO = *mut HTTP_TIMEOUT_LIMIT_INFO;
STRUCT!{struct HTTP_LISTEN_ENDPOINT_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    EnableSharing: BOOLEAN,
}}
pub type PHTTP_LISTEN_ENDPOINT_INFO = *mut HTTP_LISTEN_ENDPOINT_INFO;
STRUCT!{struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    DomainNameLength: USHORT,
    DomainName: PWSTR,
    RealmLength: USHORT,
    Realm: PWSTR,
}}
pub type PHTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS;
STRUCT!{struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    RealmLength: USHORT,
    Realm: PWSTR,
}}
pub type PHTTP_SERVER_AUTHENTICATION_BASIC_PARAMS = *mut HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS;
pub const HTTP_AUTH_ENABLE_BASIC: ULONG = 0x00000001;
pub const HTTP_AUTH_ENABLE_DIGEST: ULONG = 0x00000002;
pub const HTTP_AUTH_ENABLE_NTLM: ULONG = 0x00000004;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: ULONG = 0x00000008;
pub const HTTP_AUTH_ENABLE_KERBEROS: ULONG = 0x00000010;
pub const HTTP_AUTH_ENABLE_ALL: ULONG = HTTP_AUTH_ENABLE_BASIC | HTTP_AUTH_ENABLE_DIGEST |
    HTTP_AUTH_ENABLE_NTLM | HTTP_AUTH_ENABLE_NEGOTIATE | HTTP_AUTH_ENABLE_KERBEROS;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: UCHAR = 0x01;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: UCHAR = 0x02;
STRUCT!{struct HTTP_SERVER_AUTHENTICATION_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    AuthSchemes: ULONG,
    ReceiveMutualAuth: BOOLEAN,
    ReceiveContextHandle: BOOLEAN,
    DisableNTLMCredentialCaching: BOOLEAN,
    ExFlags: UCHAR,
    DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}}
pub type PHTTP_SERVER_AUTHENTICATION_INFO = *mut HTTP_SERVER_AUTHENTICATION_INFO;
ENUM!{enum HTTP_SERVICE_BINDING_TYPE {
    HttpServiceBindingTypeNone = 0,
    HttpServiceBindingTypeW,
    HttpServiceBindingTypeA,
}}
STRUCT!{struct HTTP_SERVICE_BINDING_BASE {
    Type: HTTP_SERVICE_BINDING_TYPE,
}}
pub type PHTTP_SERVICE_BINDING_BASE = *mut HTTP_SERVICE_BINDING_BASE;
STRUCT!{struct HTTP_SERVICE_BINDING_A {
    Base: HTTP_SERVICE_BINDING_BASE,
    Buffer: PCHAR,
    BufferSize: ULONG,
}}
pub type PHTTP_SERVICE_BINDING_A = *mut HTTP_SERVICE_BINDING_A;
STRUCT!{struct HTTP_SERVICE_BINDING_W {
    Base: HTTP_SERVICE_BINDING_BASE,
    Buffer: PWCHAR,
    BufferSize: ULONG,
}}
pub type PHTTP_SERVICE_BINDING_W = *mut HTTP_SERVICE_BINDING_W;
ENUM!{enum HTTP_AUTHENTICATION_HARDENING_LEVELS {
    HttpAuthenticationHardeningLegacy = 0,
    HttpAuthenticationHardeningMedium,
    HttpAuthenticationHardeningStrict,
}}
pub const HTTP_CHANNEL_BIND_PROXY: ULONG = 0x1;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: ULONG = 0x20;
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: ULONG = 0x2;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: ULONG = 0x4;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: ULONG = 0x8;
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: ULONG = 0x10;
STRUCT!{struct HTTP_CHANNEL_BIND_INFO {
    Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    Flags: ULONG,
    ServiceNames: *mut PHTTP_SERVICE_BINDING_BASE,
    NumberOfServiceNames: ULONG,
}}
pub type PHTTP_CHANNEL_BIND_INFO = *mut HTTP_CHANNEL_BIND_INFO;
STRUCT!{struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    ServiceName: PHTTP_SERVICE_BINDING_BASE,
    ChannelToken: PUCHAR,
    ChannelTokenSize: ULONG,
    Flags: ULONG,
}}
pub type PHTTP_REQUEST_CHANNEL_BIND_STATUS = *mut HTTP_REQUEST_CHANNEL_BIND_STATUS;
pub const HTTP_LOG_FIELD_DATE: ULONG = 0x00000001;
pub const HTTP_LOG_FIELD_TIME: ULONG = 0x00000002;
pub const HTTP_LOG_FIELD_CLIENT_IP: ULONG = 0x00000004;
pub const HTTP_LOG_FIELD_USER_NAME: ULONG = 0x00000008;
pub const HTTP_LOG_FIELD_SITE_NAME: ULONG = 0x00000010;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: ULONG = 0x00000020;
pub const HTTP_LOG_FIELD_SERVER_IP: ULONG = 0x00000040;
pub const HTTP_LOG_FIELD_METHOD: ULONG = 0x00000080;
pub const HTTP_LOG_FIELD_URI_STEM: ULONG = 0x00000100;
pub const HTTP_LOG_FIELD_URI_QUERY: ULONG = 0x00000200;
pub const HTTP_LOG_FIELD_STATUS: ULONG = 0x00000400;
pub const HTTP_LOG_FIELD_WIN32_STATUS: ULONG = 0x00000800;
pub const HTTP_LOG_FIELD_BYTES_SENT: ULONG = 0x00001000;
pub const HTTP_LOG_FIELD_BYTES_RECV: ULONG = 0x00002000;
pub const HTTP_LOG_FIELD_TIME_TAKEN: ULONG = 0x00004000;
pub const HTTP_LOG_FIELD_SERVER_PORT: ULONG = 0x00008000;
pub const HTTP_LOG_FIELD_USER_AGENT: ULONG = 0x00010000;
pub const HTTP_LOG_FIELD_COOKIE: ULONG = 0x00020000;
pub const HTTP_LOG_FIELD_REFERER: ULONG = 0x00040000;
pub const HTTP_LOG_FIELD_VERSION: ULONG = 0x00080000;
pub const HTTP_LOG_FIELD_HOST: ULONG = 0x00100000;
pub const HTTP_LOG_FIELD_SUB_STATUS: ULONG = 0x00200000;
pub const HTTP_LOG_FIELD_CLIENT_PORT: ULONG = 0x00400000;
pub const HTTP_LOG_FIELD_URI: ULONG = 0x00800000;
pub const HTTP_LOG_FIELD_SITE_ID: ULONG = 0x01000000;
pub const HTTP_LOG_FIELD_REASON: ULONG = 0x02000000;
pub const HTTP_LOG_FIELD_QUEUE_NAME: ULONG = 0x04000000;
ENUM!{enum HTTP_LOGGING_TYPE {
    HttpLoggingTypeW3C,
    HttpLoggingTypeIIS,
    HttpLoggingTypeNCSA,
    HttpLoggingTypeRaw,
}}
ENUM!{enum HTTP_LOGGING_ROLLOVER_TYPE {
    HttpLoggingRolloverSize,
    HttpLoggingRolloverDaily,
    HttpLoggingRolloverWeekly,
    HttpLoggingRolloverMonthly,
    HttpLoggingRolloverHourly,
}}
pub const HTTP_MIN_ALLOWED_LOG_FILE_ROLLOVER_SIZE: ULONG = 1 * 1024 * 1024;
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: ULONG = 0x00000001;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: ULONG = 0x00000002;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: ULONG = 0x00000004;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: ULONG = 0x00000008;
STRUCT!{struct HTTP_LOGGING_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    LoggingFlags: ULONG,
    SoftwareName: PCWSTR,
    SoftwareNameLength: USHORT,
    DirectoryNameLength: USHORT,
    DirectoryName: PCWSTR,
    Format: HTTP_LOGGING_TYPE,
    Fields: ULONG,
    pExtFields: PVOID,
    NumOfExtFields: USHORT,
    MaxRecordSize: USHORT,
    RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    RolloverSize: ULONG,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
}}
pub type PHTTP_LOGGING_INFO = *mut HTTP_LOGGING_INFO;
STRUCT!{struct HTTP_BINDING_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    RequestQueueHandle: HANDLE,
}}
pub type PHTTP_BINDING_INFO = *mut HTTP_BINDING_INFO;
ENUM!{enum HTTP_PROTECTION_LEVEL_TYPE {
    HttpProtectionLevelUnrestricted,
    HttpProtectionLevelEdgeRestricted,
    HttpProtectionLevelRestricted,
}}
pub type PHTTP_PROTECTION_LEVEL_TYPE = *mut HTTP_PROTECTION_LEVEL_TYPE;
STRUCT!{struct HTTP_PROTECTION_LEVEL_INFO {
    Flags: HTTP_PROPERTY_FLAGS,
    Level: HTTP_PROTECTION_LEVEL_TYPE,
}}
pub type PHTTP_PROTECTION_LEVEL_INFO = *mut HTTP_PROTECTION_LEVEL_INFO;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: ULONG = 0x00000001;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: ULONG = 0x00000002;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: ULONG = 0x00000001;
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: ULONG = 0x00000002;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: ULONG = 0x00000001;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: ULONG = 0x00000001;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: ULONG = 0x00000002;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: ULONG = 0x00000004;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: ULONG = 0x00000008;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: ULONG = 0x00000020;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: ULONG = 0x00000040;
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: ULONG = 0x00000001;
pub type HTTP_OPAQUE_ID = ULONGLONG;
pub type PHTTP_OPAQUE_ID = *mut ULONGLONG;
pub type HTTP_REQUEST_ID = HTTP_OPAQUE_ID;
pub type PHTTP_REQUEST_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_CONNECTION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_RAW_CONNECTION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_RAW_CONNECTION_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_URL_GROUP_ID = HTTP_OPAQUE_ID;
pub type PHTTP_URL_GROUP_ID = *mut HTTP_OPAQUE_ID;
pub type HTTP_SERVER_SESSION_ID = HTTP_OPAQUE_ID;
pub type PHTTP_SERVER_SESSION_ID = *mut HTTP_OPAQUE_ID;
pub const HTTP_BYTE_RANGE_TO_EOF: ULONGLONG = !0;
STRUCT!{struct HTTP_BYTE_RANGE {
    StartingOffset: ULARGE_INTEGER,
    Length: ULARGE_INTEGER,
}}
pub type PHTTP_BYTE_RANGE = *mut HTTP_BYTE_RANGE;
STRUCT!{struct HTTP_VERSION {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
}}
pub type PHTTP_VERSION = *mut HTTP_VERSION;
pub const HTTP_VERSION_UNKNOWN: HTTP_VERSION = HTTP_VERSION { MajorVersion: 0, MinorVersion: 0 };
pub const HTTP_VERSION_0_9: HTTP_VERSION = HTTP_VERSION { MajorVersion: 0, MinorVersion: 9 };
pub const HTTP_VERSION_1_0: HTTP_VERSION = HTTP_VERSION { MajorVersion: 1, MinorVersion: 0 };
pub const HTTP_VERSION_1_1: HTTP_VERSION = HTTP_VERSION { MajorVersion: 1, MinorVersion: 1 };
#[inline]
pub fn HTTP_SET_VERSION(mut version: HTTP_VERSION, major: USHORT, minor: USHORT) {
    version.MajorVersion = major;
    version.MinorVersion = minor;
}
#[inline]
pub fn HTTP_EQUAL_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.MajorVersion == major && version.MinorVersion == minor
}
#[inline]
pub fn HTTP_GREATER_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.MajorVersion > major || (version.MajorVersion == major && version.MinorVersion > minor)
}
#[inline]
pub fn HTTP_LESS_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.MajorVersion < major || (version.MajorVersion == major && version.MinorVersion < minor)
}
#[inline]
pub fn HTTP_NOT_EQUAL_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    !HTTP_EQUAL_VERSION(version, major, minor)
}
#[inline]
pub fn HTTP_GREATER_EQUAL_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    !HTTP_LESS_VERSION(version, major, minor)
}
#[inline]
pub fn HTTP_LESS_EQUAL_VERSION(version: HTTP_VERSION, major: USHORT, minor: USHORT) -> bool {
    !HTTP_GREATER_VERSION(version, major, minor)
}
ENUM!{enum HTTP_VERB {
    HttpVerbUnparsed,
    HttpVerbUnknown,
    HttpVerbInvalid,
    HttpVerbOPTIONS,
    HttpVerbGET,
    HttpVerbHEAD,
    HttpVerbPOST,
    HttpVerbPUT,
    HttpVerbDELETE,
    HttpVerbTRACE,
    HttpVerbCONNECT,
    HttpVerbTRACK,
    HttpVerbMOVE,
    HttpVerbCOPY,
    HttpVerbPROPFIND,
    HttpVerbPROPPATCH,
    HttpVerbMKCOL,
    HttpVerbLOCK,
    HttpVerbUNLOCK,
    HttpVerbSEARCH,
    HttpVerbMaximum,
}}
pub type PHTTP_VERB = *mut HTTP_VERB;
ENUM!{enum HTTP_HEADER_ID {
    HttpHeaderCacheControl = 0,
    HttpHeaderConnection = 1,
    HttpHeaderDate = 2,
    HttpHeaderKeepAlive = 3,
    HttpHeaderPragma = 4,
    HttpHeaderTrailer = 5,
    HttpHeaderTransferEncoding = 6,
    HttpHeaderUpgrade = 7,
    HttpHeaderVia = 8,
    HttpHeaderWarning = 9,
    HttpHeaderAllow = 10,
    HttpHeaderContentLength = 11,
    HttpHeaderContentType = 12,
    HttpHeaderContentEncoding = 13,
    HttpHeaderContentLanguage = 14,
    HttpHeaderContentLocation = 15,
    HttpHeaderContentMd5 = 16,
    HttpHeaderContentRange = 17,
    HttpHeaderExpires = 18,
    HttpHeaderLastModified = 19,
    HttpHeaderAccept = 20,
    HttpHeaderAcceptCharset = 21,
    HttpHeaderAcceptEncoding = 22,
    HttpHeaderAcceptLanguage = 23,
    HttpHeaderAuthorization = 24,
    HttpHeaderCookie = 25,
    HttpHeaderExpect = 26,
    HttpHeaderFrom = 27,
    HttpHeaderHost = 28,
    HttpHeaderIfMatch = 29,
    HttpHeaderIfModifiedSince = 30,
    HttpHeaderIfNoneMatch = 31,
    HttpHeaderIfRange = 32,
    HttpHeaderIfUnmodifiedSince = 33,
    HttpHeaderMaxForwards = 34,
    HttpHeaderProxyAuthorization = 35,
    HttpHeaderReferer = 36,
    HttpHeaderRange = 37,
    HttpHeaderTe = 38,
    HttpHeaderTranslate = 39,
    HttpHeaderUserAgent = 40,
    HttpHeaderRequestMaximum = 41,
    HttpHeaderAcceptRanges = 20,
    HttpHeaderAge = 21,
    HttpHeaderEtag = 22,
    HttpHeaderLocation = 23,
    HttpHeaderProxyAuthenticate = 24,
    HttpHeaderRetryAfter = 25,
    HttpHeaderServer = 26,
    HttpHeaderSetCookie = 27,
    HttpHeaderVary = 28,
    HttpHeaderWwwAuthenticate = 29,
    HttpHeaderResponseMaximum = 30,
    HttpHeaderMaximum = 41,
}}
pub type PHTTP_HEADER_ID = *mut HTTP_HEADER_ID;
STRUCT!{struct HTTP_KNOWN_HEADER {
    RawValueLength: USHORT,
    pRawValue: PCSTR,
}}
pub type PHTTP_KNOWN_HEADER = *mut HTTP_KNOWN_HEADER;
STRUCT!{struct HTTP_UNKNOWN_HEADER {
    NameLength: USHORT,
    RawValueLength: USHORT,
    pName: PCSTR,
    pRawValue: PCSTR,
}}
pub type PHTTP_UNKNOWN_HEADER = *mut HTTP_UNKNOWN_HEADER;
ENUM!{enum HTTP_LOG_DATA_TYPE {
    HttpLogDataTypeFields = 0,
}}
pub type PHTTP_LOG_DATA_TYPE = *mut HTTP_LOG_DATA_TYPE;
STRUCT!{struct HTTP_LOG_DATA {
    Type: HTTP_LOG_DATA_TYPE,
}}
pub type PHTTP_LOG_DATA = *mut HTTP_LOG_DATA;
STRUCT!{struct HTTP_LOG_FIELDS_DATA {
    Base: HTTP_LOG_DATA,
    UserNameLength: USHORT,
    UriStemLength: USHORT,
    ClientIpLength: USHORT,
    ServerNameLength: USHORT,
    ServiceNameLength: USHORT,
    ServerIpLength: USHORT,
    MethodLength: USHORT,
    UriQueryLength: USHORT,
    HostLength: USHORT,
    UserAgentLength: USHORT,
    CookieLength: USHORT,
    ReferrerLength: USHORT,
    UserName: PWCHAR,
    UriStem: PWCHAR,
    ClientIp: PCHAR,
    ServerName: PCHAR,
    ServiceName: PCHAR,
    ServerIp: PCHAR,
    Method: PCHAR,
    UriQuery: PCHAR,
    Host: PCHAR,
    UserAgent: PCHAR,
    Cookie: PCHAR,
    Referrer: PCHAR,
    ServerPort: USHORT,
    ProtocolStatus: USHORT,
    Win32Status: ULONG,
    MethodNum: HTTP_VERB,
    SubStatus: USHORT,
}}
pub type PHTTP_LOG_FIELDS_DATA = *mut HTTP_LOG_FIELDS_DATA;
ENUM!{enum HTTP_DATA_CHUNK_TYPE {
    HttpDataChunkFromMemory,
    HttpDataChunkFromFileHandle,
    HttpDataChunkFromFragmentCache,
    HttpDataChunkFromFragmentCacheEx,
    HttpDataChunkMaximum,
}}
pub type PHTTP_DATA_CHUNK_TYPE = *mut HTTP_DATA_CHUNK_TYPE;
STRUCT!{struct HTTP_DATA_CHUNK_FromMemory {
    pBuffer: PVOID,
    BufferLength: ULONG,
}}
STRUCT!{struct HTTP_DATA_CHUNK_FromFileHandle {
    ByteRange: HTTP_BYTE_RANGE,
    FileHandle: HANDLE,
}}
STRUCT!{struct HTTP_DATA_CHUNK_FromFragmentCache {
    FragmentNameLength: USHORT,
    pFragmentName: PCWSTR,
}}
STRUCT!{struct HTTP_DATA_CHUNK_FromFragmentCacheEx {
    ByteRange: HTTP_BYTE_RANGE,
    pFragmentName: PCWSTR,
}}
UNION!{union HTTP_DATA_CHUNK_u {
    [u64; 3],
    FromMemory FromMemory_mut: HTTP_DATA_CHUNK_FromMemory,
    FromFileHandle FromFileHandle_mut: HTTP_DATA_CHUNK_FromFileHandle,
    FromFragmentCache FromFragmentCache_mut: HTTP_DATA_CHUNK_FromFragmentCache,
    FromFragmentCacheEx FromFragmentCacheEx_mut: HTTP_DATA_CHUNK_FromFragmentCacheEx,
}}
STRUCT!{struct HTTP_DATA_CHUNK {
    DataChunkType: HTTP_DATA_CHUNK_TYPE,
    u: HTTP_DATA_CHUNK_u,
}}
pub type PHTTP_DATA_CHUNK = *mut HTTP_DATA_CHUNK;
STRUCT!{struct HTTP_REQUEST_HEADERS {
    UnknownHeaderCount: USHORT,
    pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    TrailerCount: USHORT,
    pTrailers: PHTTP_UNKNOWN_HEADER,
    KnownHeaders: [HTTP_KNOWN_HEADER; 41], // FIXME HttpHeaderRequestMaximum
}}
pub type PHTTP_REQUEST_HEADERS = *mut HTTP_REQUEST_HEADERS;
STRUCT!{struct HTTP_RESPONSE_HEADERS {
    UnknownHeaderCount: USHORT,
    pUnknownHeaders: PHTTP_UNKNOWN_HEADER,
    TrailerCount: USHORT,
    pTrailers: PHTTP_UNKNOWN_HEADER,
    KnownHeaders: [HTTP_KNOWN_HEADER; 30], // FIXME HttpHeaderResponseMaximum
}}
pub type PHTTP_RESPONSE_HEADERS = *mut HTTP_RESPONSE_HEADERS;
STRUCT!{struct HTTP_TRANSPORT_ADDRESS {
    pRemoteAddress: PSOCKADDR,
    pLocalAddress: PSOCKADDR,
}}
pub type PHTTP_TRANSPORT_ADDRESS = *mut HTTP_TRANSPORT_ADDRESS;
STRUCT!{struct HTTP_COOKED_URL {
    FullUrlLength: USHORT,
    HostLength: USHORT,
    AbsPathLength: USHORT,
    QueryStringLength: USHORT,
    pFullUrl: PCWSTR,
    pHost: PCWSTR,
    pAbsPath: PCWSTR,
    pQueryString: PCWSTR,
}}
pub type PHTTP_COOKED_URL = *mut HTTP_COOKED_URL;
pub type HTTP_URL_CONTEXT = ULONGLONG;
pub const HTTP_URL_FLAG_REMOVE_ALL: ULONG = 0x00000001;
ENUM!{enum HTTP_AUTH_STATUS {
    HttpAuthStatusSuccess,
    HttpAuthStatusNotAuthenticated,
    HttpAuthStatusFailure,
}}
pub type PHTTP_AUTH_STATUS = *mut HTTP_AUTH_STATUS;
ENUM!{enum HTTP_REQUEST_AUTH_TYPE {
    HttpRequestAuthTypeNone = 0,
    HttpRequestAuthTypeBasic,
    HttpRequestAuthTypeDigest,
    HttpRequestAuthTypeNTLM,
    HttpRequestAuthTypeNegotiate,
    HttpRequestAuthTypeKerberos,
}}
pub type PHTTP_REQUEST_AUTH_TYPE = *mut HTTP_REQUEST_AUTH_TYPE;
STRUCT!{struct HTTP_SSL_CLIENT_CERT_INFO {
    CertFlags: ULONG,
    CertEncodedSize: ULONG,
    pCertEncoded: PUCHAR,
    Token: HANDLE,
    CertDeniedByMapper: BOOLEAN,
}}
pub type PHTTP_SSL_CLIENT_CERT_INFO = *mut HTTP_SSL_CLIENT_CERT_INFO;
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: ULONG = 0x1;
STRUCT!{struct HTTP_SSL_INFO {
    ServerCertKeySize: USHORT,
    ConnectionKeySize: USHORT,
    ServerCertIssuerSize: ULONG,
    ServerCertSubjectSize: ULONG,
    pServerCertIssuer: PCSTR,
    pServerCertSubject: PCSTR,
    pClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO,
    SslClientCertNegotiated: ULONG,
}}
pub type PHTTP_SSL_INFO = *mut HTTP_SSL_INFO;
ENUM!{enum HTTP_REQUEST_INFO_TYPE {
    HttpRequestInfoTypeAuth,
    HttpRequestInfoTypeChannelBind,
}}
STRUCT!{struct HTTP_REQUEST_INFO {
    InfoType: HTTP_REQUEST_INFO_TYPE,
    InfoLength: ULONG,
    pInfo: PVOID,
}}
pub type PHTTP_REQUEST_INFO = *mut HTTP_REQUEST_INFO;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: ULONG = 0x00000001;
STRUCT!{struct HTTP_REQUEST_AUTH_INFO {
    AuthStatus: HTTP_AUTH_STATUS,
    SecStatus: SECURITY_STATUS,
    Flags: ULONG,
    AuthType: HTTP_REQUEST_AUTH_TYPE,
    AccessToken: HANDLE,
    ContextAttributes: ULONG,
    PackedContextLength: ULONG,
    PackedContextType: ULONG,
    PackedContext: PVOID,
    MutualAuthDataLength: ULONG,
    pMutualAuthData: PCHAR,
    PackageNameLength: USHORT,
    pPackageName: PWSTR,
}}
pub type PHTTP_REQUEST_AUTH_INFO = *mut HTTP_REQUEST_AUTH_INFO;
STRUCT!{struct HTTP_REQUEST_V1 {
    Flags: ULONG,
    ConnectionId: HTTP_CONNECTION_ID,
    RequestId: HTTP_REQUEST_ID,
    UrlContext: HTTP_URL_CONTEXT,
    Version: HTTP_VERSION,
    Verb: HTTP_VERB,
    UnknownVerbLength: USHORT,
    RawUrlLength: USHORT,
    pUnknownVerb: PCSTR,
    pRawUrl: PCSTR,
    CookedUrl: HTTP_COOKED_URL,
    Address: HTTP_TRANSPORT_ADDRESS,
    Headers: HTTP_REQUEST_HEADERS,
    BytesReceived: ULONGLONG,
    EntityChunkCount: USHORT,
    pEntityChunks: PHTTP_DATA_CHUNK,
    RawConnectionId: HTTP_RAW_CONNECTION_ID,
    pSslInfo: PHTTP_SSL_INFO,
}}
pub type PHTTP_REQUEST_V1 = *mut HTTP_REQUEST_V1;
STRUCT!{struct HTTP_REQUEST_V2 {
    Base: HTTP_REQUEST_V1,
    RequestInfoCount: USHORT,
    pRequestInfo: PHTTP_REQUEST_INFO,
}}
pub type PHTTP_REQUEST_V2 = *mut HTTP_REQUEST_V2;
pub type HTTP_REQUEST = HTTP_REQUEST_V2;
pub type PHTTP_REQUEST = *mut HTTP_REQUEST;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: ULONG = 0x00000001;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: ULONG = 0x00000002;
STRUCT!{struct HTTP_RESPONSE_V1 {
    Flags: ULONG,
    Version: HTTP_VERSION,
    StatusCode: USHORT,
    ReasonLength: USHORT,
    pReason: PCSTR,
    Headers: HTTP_RESPONSE_HEADERS,
    EntityChunkCount: USHORT,
    pEntityChunks: PHTTP_DATA_CHUNK,
}}
pub type PHTTP_RESPONSE_V1 = *mut HTTP_RESPONSE_V1;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: ULONG = 0x00000001;
ENUM!{enum HTTP_RESPONSE_INFO_TYPE {
    HttpResponseInfoTypeMultipleKnownHeaders,
    HttpResponseInfoTypeAuthenticationProperty,
    HttpResponseInfoTypeQoSProperty,
    HttpResponseInfoTypeChannelBind,
}}
pub type PHTTP_RESPONSE_INFO_TYPE = *mut HTTP_RESPONSE_INFO_TYPE;
STRUCT!{struct HTTP_RESPONSE_INFO {
    Type: HTTP_RESPONSE_INFO_TYPE,
    Length: ULONG,
    pInfo: PVOID,
}}
pub type PHTTP_RESPONSE_INFO = *mut HTTP_RESPONSE_INFO;
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: ULONG = 0x00000001;
STRUCT!{struct HTTP_MULTIPLE_KNOWN_HEADERS {
    HeaderId: HTTP_HEADER_ID,
    Flags: ULONG,
    KnownHeaderCount: USHORT,
    KnownHeaders: PHTTP_KNOWN_HEADER,
}}
pub type PHTTP_MULTIPLE_KNOWN_HEADERS = *mut HTTP_MULTIPLE_KNOWN_HEADERS;
STRUCT!{struct HTTP_RESPONSE_V2 {
    Base: HTTP_RESPONSE_V1,
    ResponseInfoCount: USHORT,
    pResponseInfo: PHTTP_RESPONSE_INFO,
}}
pub type PHTTP_RESPONSE_V2 = *mut HTTP_RESPONSE_V2;
pub type HTTP_RESPONSE = HTTP_RESPONSE_V2;
pub type PHTTP_RESPONSE = *mut HTTP_RESPONSE;
STRUCT!{struct HTTPAPI_VERSION {
    HttpApiMajorVersion: USHORT,
    HttpApiMinorVersion: USHORT,
}}
pub type PHTTPAPI_VERSION = *mut HTTPAPI_VERSION;
pub const HTTPAPI_VERSION_2: HTTPAPI_VERSION = HTTPAPI_VERSION {
    HttpApiMajorVersion: 2,
    HttpApiMinorVersion: 0,
};
pub const HTTPAPI_VERSION_1: HTTPAPI_VERSION = HTTPAPI_VERSION {
    HttpApiMajorVersion: 1,
    HttpApiMinorVersion: 0,
};
#[inline]
pub fn HTTPAPI_EQUAL_VERSION(version: HTTPAPI_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.HttpApiMajorVersion == major && version.HttpApiMinorVersion == minor
}
#[inline]
pub fn HTTPAPI_GREATER_VERSION(version: HTTPAPI_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.HttpApiMajorVersion > major ||
    (version.HttpApiMajorVersion == major && version.HttpApiMinorVersion > minor)
}
#[inline]
pub fn HTTPAPI_LESS_VERSION(version: HTTPAPI_VERSION, major: USHORT, minor: USHORT) -> bool {
    version.HttpApiMajorVersion < major ||
    (version.HttpApiMajorVersion == major && version.HttpApiMinorVersion < minor)
}
#[inline]
pub fn HTTPAPI_VERSION_GREATER_OR_EQUAL(
    version: HTTPAPI_VERSION,
    major: USHORT,
    minor: USHORT,
) -> bool {
    !HTTPAPI_LESS_VERSION(version, major, minor)
}
ENUM!{enum HTTP_CACHE_POLICY_TYPE {
    HttpCachePolicyNocache,
    HttpCachePolicyUserInvalidates,
    HttpCachePolicyTimeToLive,
    HttpCachePolicyMaximum,
}}
pub type PHTTP_CACHE_POLICY_TYPE = *mut HTTP_CACHE_POLICY_TYPE;
STRUCT!{struct HTTP_CACHE_POLICY {
    Policy: HTTP_CACHE_POLICY_TYPE,
    SecondsToLive: ULONG,
}}
pub type PHTTP_CACHE_POLICY = *mut HTTP_CACHE_POLICY;
ENUM!{enum HTTP_SERVICE_CONFIG_ID {
    HttpServiceConfigIPListenList,
    HttpServiceConfigSSLCertInfo,
    HttpServiceConfigUrlAclInfo,
    HttpServiceConfigTimeout,
    HttpServiceConfigCache,
    HttpServiceConfigSslSniCertInfo,
    HttpServiceConfigSslCcsCertInfo,
    HttpServiceConfigMax,
}}
pub type PHTTP_SERVICE_CONFIG_ID = *mut HTTP_SERVICE_CONFIG_ID;
ENUM!{enum HTTP_SERVICE_CONFIG_QUERY_TYPE {
    HttpServiceConfigQueryExact,
    HttpServiceConfigQueryNext,
    HttpServiceConfigQueryMax,
}}
pub type PHTTP_SERVICE_CONFIG_QUERY_TYPE = *mut HTTP_SERVICE_CONFIG_QUERY_TYPE;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pIpPort: PSOCKADDR,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_KEY = *mut HTTP_SERVICE_CONFIG_SSL_KEY;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    IpPort: SOCKADDR_STORAGE,
    Host: PWSTR,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_KEY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_KEY;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    LocalAddress: SOCKADDR_STORAGE,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_KEY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_KEY;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    SslHashLength: ULONG,
    pSslHash: PVOID,
    AppId: GUID,
    pSslCertStoreName: PWSTR,
    DefaultCertCheckMode: DWORD,
    DefaultRevocationFreshnessTime: DWORD,
    DefaultRevocationUrlRetrievalTimeout: DWORD,
    pDefaultSslCtlIdentifier: PWSTR,
    pDefaultSslCtlStoreName: PWSTR,
    DefaultFlags: DWORD,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_PARAM = *mut HTTP_SERVICE_CONFIG_SSL_PARAM;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: DWORD = 0x00000001;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: DWORD = 0x00000002;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: DWORD = 0x00000004;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_SET = *mut HTTP_SERVICE_CONFIG_SSL_SET;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_SET = *mut HTTP_SERVICE_CONFIG_SSL_SNI_SET;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_SET = *mut HTTP_SERVICE_CONFIG_SSL_CCS_SET;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    dwToken: DWORD,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_QUERY;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    dwToken: DWORD,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_SNI_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_SNI_QUERY;
STRUCT!{struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    dwToken: DWORD,
}}
pub type PHTTP_SERVICE_CONFIG_SSL_CCS_QUERY = *mut HTTP_SERVICE_CONFIG_SSL_CCS_QUERY;
STRUCT!{struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    AddrLength: USHORT,
    pAddress: PSOCKADDR,
}}
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_PARAM = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM;
STRUCT!{struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    AddrCount: ULONG,
    AddrList: [SOCKADDR_STORAGE; ANYSIZE_ARRAY],
}}
pub type PHTTP_SERVICE_CONFIG_IP_LISTEN_QUERY = *mut HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY;
STRUCT!{struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pUrlPrefix: PWSTR,
}}
pub type PHTTP_SERVICE_CONFIG_URLACL_KEY = *mut HTTP_SERVICE_CONFIG_URLACL_KEY;
STRUCT!{struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pStringSecurityDescriptor: PWSTR,
}}
pub type PHTTP_SERVICE_CONFIG_URLACL_PARAM = *mut HTTP_SERVICE_CONFIG_URLACL_PARAM;
STRUCT!{struct HTTP_SERVICE_CONFIG_URLACL_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_URLACL_SET = *mut HTTP_SERVICE_CONFIG_URLACL_SET;
STRUCT!{struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    dwToken: DWORD,
}}
pub type PHTTP_SERVICE_CONFIG_URLACL_QUERY = *mut HTTP_SERVICE_CONFIG_URLACL_QUERY;
ENUM!{enum HTTP_SERVICE_CONFIG_CACHE_KEY {
    MaxCacheResponseSize = 0,
    CacheRangeChunkSize,
}}
pub type PHTTP_SERVICE_CONFIG_CACHE_KEY = *mut HTTP_SERVICE_CONFIG_CACHE_KEY;
pub type HTTP_SERVICE_CONFIG_CACHE_PARAM = ULONG;
pub type PHTTP_SERVICE_CONFIG_CACHE_PARAM = *mut ULONG;
STRUCT!{struct HTTP_SERVICE_CONFIG_CACHE_SET {
    KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    ParamDesc: HTTP_SERVICE_CONFIG_CACHE_PARAM,
}}
pub type PHTTP_SERVICE_CONFIG_CACHE_SET = *mut HTTP_SERVICE_CONFIG_CACHE_SET;
pub const HTTP_NULL_ID: ULONGLONG = 0;
#[inline]
pub unsafe fn HTTP_IS_NULL_ID(pid: PHTTP_OPAQUE_ID) -> bool {
    HTTP_NULL_ID == *pid
}
#[inline]
pub unsafe fn HTTP_SET_NULL_ID(pid: PHTTP_OPAQUE_ID) {
    *pid = HTTP_NULL_ID
}
extern "system" {
    pub fn HttpInitialize(
        Version: HTTPAPI_VERSION,
        Flags: ULONG,
        pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpTerminate(
        Flags: ULONG,
        pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpCreateHttpHandle(
        pReqQueueHandle: PHANDLE,
        Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpCreateRequestQueue(
        Version: HTTPAPI_VERSION,
        pName: PCWSTR,
        pSecurityAttributes: PSECURITY_ATTRIBUTES,
        Flags: ULONG,
        pReqQueueHandle: PHANDLE,
    ) -> ULONG;
    pub fn HttpCloseRequestQueue(
        ReqQueueHandle: HANDLE,
    ) -> ULONG;
    pub fn HttpSetRequestQueueProperty(
        Handle: HANDLE,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
        Reserved: ULONG,
        pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpQueryRequestQueueProperty(
        Handle: HANDLE,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
        Reserved: ULONG,
        pReturnLength: PULONG,
        pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpShutdownRequestQueue(
        ReqQueueHandle: HANDLE,
    ) -> ULONG;
    pub fn HttpReceiveClientCertificate(
        ReqQueueHandle: HANDLE,
        ConnectionId: HTTP_CONNECTION_ID,
        Flags: ULONG,
        pSslClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO,
        SslClientCertInfoSize: ULONG,
        pBytesReceived: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpCreateServerSession(
        Version: HTTPAPI_VERSION,
        pServerSessionId: PHTTP_SERVER_SESSION_ID,
        Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpCloseServerSession(
        ServerSessionId: HTTP_SERVER_SESSION_ID,
    ) -> ULONG;
    pub fn HttpQueryServerSessionProperty(
        ServerSessionId: HTTP_SERVER_SESSION_ID,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
        pReturnLength: PULONG,
    ) -> ULONG;
    pub fn HttpSetServerSessionProperty(
        ServerSessionId: HTTP_SERVER_SESSION_ID,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
    ) -> ULONG;
    pub fn HttpAddUrl(
        ReqQueueHandle: HANDLE,
        pFullyQualifiedUrl: PCWSTR,
        pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpRemoveUrl(
        ReqQueueHandle: HANDLE,
        pFullyQualifiedUrl: PCWSTR,
    ) -> ULONG;
    pub fn HttpCreateUrlGroup(
        ServerSessionId: HTTP_SERVER_SESSION_ID,
        pUrlGroupId: PHTTP_URL_GROUP_ID,
        Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpCloseUrlGroup(
        UrlGroupId: HTTP_URL_GROUP_ID,
    ) -> ULONG;
    pub fn HttpAddUrlToUrlGroup(
        UrlGroupId: HTTP_URL_GROUP_ID,
        pFullyQualifiedUrl: PCWSTR,
        UrlContext: HTTP_URL_CONTEXT,
        Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpRemoveUrlFromUrlGroup(
        UrlGroupId: HTTP_URL_GROUP_ID,
        pFullyQualifiedUrl: PCWSTR,
        Flags: ULONG,
    ) -> ULONG;
    pub fn HttpSetUrlGroupProperty(
        UrlGroupId: HTTP_URL_GROUP_ID,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
    ) -> ULONG;
    pub fn HttpQueryUrlGroupProperty(
        UrlGroupId: HTTP_URL_GROUP_ID,
        Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
        pReturnLength: PULONG,
    ) -> ULONG;
    pub fn HttpPrepareUrl(
        Reserved: PVOID,
        Flags: ULONG,
        Url: PCWSTR,
        PreparedUrl: *mut PWSTR,
    ) -> ULONG;
    pub fn HttpReceiveHttpRequest(
        ReqQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        Flags: ULONG,
        pRequestBuffer: PHTTP_REQUEST,
        RequestBufferLength: ULONG,
        pBytesReturned: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpReceiveRequestEntityBody(
        ReqQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        Flags: ULONG,
        pBuffer: PVOID,
        EntityBufferLength: ULONG,
        pBytesReturned: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpSendHttpResponse(
        ReqQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        Flags: ULONG,
        pHttpResponse: PHTTP_RESPONSE,
        pCachePolicy: PHTTP_CACHE_POLICY,
        pBytesSent: PULONG,
        pReserved1: PVOID,
        Reserved2: ULONG,
        pOverlapped: LPOVERLAPPED,
        pLogData: PHTTP_LOG_DATA,
    ) -> ULONG;
    pub fn HttpSendResponseEntityBody(
        ReqQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        Flags: ULONG,
        EntityChunkCount: USHORT,
        pEntityChunks: PHTTP_DATA_CHUNK,
        pBytesSent: PULONG,
        pReserved1: PVOID,
        Reserved2: ULONG,
        pOverlapped: LPOVERLAPPED,
        pLogData: PHTTP_LOG_DATA,
    ) -> ULONG;
    pub fn HttpWaitForDisconnect(
        ReqQueueHandle: HANDLE,
        ConnectionId: HTTP_CONNECTION_ID,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpWaitForDisconnectEx(
        ReqQueueHandle: HANDLE,
        ConnectionId: HTTP_CONNECTION_ID,
        Reserved: ULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpCancelHttpRequest(
        ReqQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpWaitForDemandStart(
        ReqQueueHandle: HANDLE,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpFlushResponseCache(
        ReqQueueHandle: HANDLE,
        pUrlPrefix: PCWSTR,
        Flags: ULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpAddFragmentToCache(
        ReqQueueHandle: HANDLE,
        pUrlPrefix: PCWSTR,
        pDataChunk: PHTTP_DATA_CHUNK,
        pCachePolicy: PHTTP_CACHE_POLICY,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpReadFragmentFromCache(
        ReqQueueHandle: HANDLE,
        pUrlPrefix: PCWSTR,
        pByteRange: PHTTP_BYTE_RANGE,
        pBuffer: PVOID,
        BufferLength: ULONG,
        pBytesRead: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpSetServiceConfiguration(
        ServiceHandle: HANDLE,
        ConfigId: HTTP_SERVICE_CONFIG_ID,
        pConfigInformation: PVOID,
        ConfigInformationLength: ULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpDeleteServiceConfiguration(
        ServiceHandle: HANDLE,
        ConfigId: HTTP_SERVICE_CONFIG_ID,
        pConfigInformation: PVOID,
        ConfigInformationLength: ULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpQueryServiceConfiguration(
        ServiceHandle: HANDLE,
        ConfigId: HTTP_SERVICE_CONFIG_ID,
        pInput: PVOID,
        InputLength: ULONG,
        pOutput: PVOID,
        OutputLength: ULONG,
        pReturnLength: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpDeclarePush(
        RequestQueueHandle: HANDLE,
        RequestId: HTTP_REQUEST_ID,
        Verb: HTTP_VERB,
        Path: PCWSTR,
        Query: PCSTR,
        Headers: PHTTP_REQUEST_HEADERS,
    ) -> ULONG;
    pub fn HttpUpdateServiceConfiguration(
        Handle: HANDLE,
        ConfigId: HTTP_SERVICE_CONFIG_ID,
        ConfigInfo: PVOID,
        ConfigInfoLength: ULONG,
        Overlapped: LPOVERLAPPED,
    ) -> ULONG;
}
