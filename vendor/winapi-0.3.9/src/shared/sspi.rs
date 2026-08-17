// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Security Support Provider Interface Prototypes and structure definitions
use ctypes::{c_char, c_int, c_uchar, c_ulong, c_ushort, c_void};
use shared::basetsd::ULONG_PTR;
use shared::guiddef::GUID;
use shared::minwindef::{PUCHAR, ULONG, USHORT};
use um::subauth::PUNICODE_STRING;
use um::wincred::{PCREDUI_INFOA, PCREDUI_INFOW};
use um::winnt::{
    ANYSIZE_ARRAY, BOOLEAN, CHAR, HANDLE, LARGE_INTEGER, LONG, LPSTR, LPWSTR, LUID, PCSTR, PCWSTR,
    PVOID, WCHAR
};
pub type SEC_WCHAR = WCHAR;
pub type SEC_CHAR = CHAR;
pub type SECURITY_STATUS = LONG;
STRUCT!{struct SecHandle {
    dwLower: ULONG_PTR,
    dwUpper: ULONG_PTR,
}}
pub type PSecHandle = *mut SecHandle;
pub const SEC_DELETED_HANDLE: ULONG_PTR = 2;
pub type CredHandle = SecHandle;
pub type PCredHandle = PSecHandle;
pub type CtxtHandle = SecHandle;
pub type PCtxtHandle = PSecHandle;
pub type SECURITY_INTEGER = LARGE_INTEGER;
pub type PSECURITY_INTEGER = *mut LARGE_INTEGER;
pub type TimeStamp = SECURITY_INTEGER;
pub type PTimeStamp = *mut SECURITY_INTEGER;
STRUCT!{struct SECURITY_STRING {
    Length: c_ushort,
    MaximumLength: c_ushort,
    Buffer: *mut c_ushort,
}}
pub type PSECURITY_STRING = *mut SECURITY_STRING;
STRUCT!{struct SecPkgInfoW {
    fCapabilities: c_ulong,
    wVersion: c_ushort,
    wRPCID: c_ushort,
    cbMaxToken: c_ulong,
    Name: *mut SEC_WCHAR,
    Comment: *mut SEC_WCHAR,
}}
pub type PSecPkgInfoW = *mut SecPkgInfoW;
STRUCT!{struct SecPkgInfoA {
    fCapabilities: c_ulong,
    wVersion: c_ushort,
    wRPCID: c_ushort,
    cbMaxToken: c_ulong,
    Name: *mut SEC_CHAR,
    Comment: *mut SEC_CHAR,
}}
pub type PSecPkgInfoA = *mut SecPkgInfoA;
pub const SECPKG_FLAG_INTEGRITY: c_ulong = 0x00000001;
pub const SECPKG_FLAG_PRIVACY: c_ulong = 0x00000002;
pub const SECPKG_FLAG_TOKEN_ONLY: c_ulong = 0x00000004;
pub const SECPKG_FLAG_DATAGRAM: c_ulong = 0x00000008;
pub const SECPKG_FLAG_CONNECTION: c_ulong = 0x00000010;
pub const SECPKG_FLAG_MULTI_REQUIRED: c_ulong = 0x00000020;
pub const SECPKG_FLAG_CLIENT_ONLY: c_ulong = 0x00000040;
pub const SECPKG_FLAG_EXTENDED_ERROR: c_ulong = 0x00000080;
pub const SECPKG_FLAG_IMPERSONATION: c_ulong = 0x00000100;
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: c_ulong = 0x00000200;
pub const SECPKG_FLAG_STREAM: c_ulong = 0x00000400;
pub const SECPKG_FLAG_NEGOTIABLE: c_ulong = 0x00000800;
pub const SECPKG_FLAG_GSS_COMPATIBLE: c_ulong = 0x00001000;
pub const SECPKG_FLAG_LOGON: c_ulong = 0x00002000;
pub const SECPKG_FLAG_ASCII_BUFFERS: c_ulong = 0x00004000;
pub const SECPKG_FLAG_FRAGMENT: c_ulong = 0x00008000;
pub const SECPKG_FLAG_MUTUAL_AUTH: c_ulong = 0x00010000;
pub const SECPKG_FLAG_DELEGATION: c_ulong = 0x00020000;
pub const SECPKG_FLAG_READONLY_WITH_CHECKSUM: c_ulong = 0x00040000;
pub const SECPKG_FLAG_RESTRICTED_TOKENS: c_ulong = 0x00080000;
pub const SECPKG_FLAG_NEGO_EXTENDER: c_ulong = 0x00100000;
pub const SECPKG_FLAG_NEGOTIABLE2: c_ulong = 0x00200000;
pub const SECPKG_FLAG_APPCONTAINER_PASSTHROUGH: c_ulong = 0x00400000;
pub const SECPKG_FLAG_APPCONTAINER_CHECKS: c_ulong = 0x00800000;
pub const SECPKG_ID_NONE: c_ulong = 0xFFFF;
pub const SECPKG_CALLFLAGS_APPCONTAINER: c_ulong = 0x00000001;
pub const SECPKG_CALLFLAGS_APPCONTAINER_AUTHCAPABLE: c_ulong = 0x00000002;
pub const SECPKG_CALLFLAGS_FORCE_SUPPLIED: c_ulong = 0x00000004;
STRUCT!{struct SecBuffer {
    cbBuffer: c_ulong,
    BufferType: c_ulong,
    pvBuffer: *mut c_void,
}}
pub type PSecBuffer = *mut SecBuffer;
STRUCT!{struct SecBufferDesc {
    ulVersion: c_ulong,
    cBuffers: c_ulong,
    pBuffers: PSecBuffer,
}}
pub type PSecBufferDesc = *mut SecBufferDesc;
pub const SECBUFFER_VERSION: c_ulong = 0;
pub const SECBUFFER_EMPTY: c_ulong = 0;
pub const SECBUFFER_DATA: c_ulong = 1;
pub const SECBUFFER_TOKEN: c_ulong = 2;
pub const SECBUFFER_PKG_PARAMS: c_ulong = 3;
pub const SECBUFFER_MISSING: c_ulong = 4;
pub const SECBUFFER_EXTRA: c_ulong = 5;
pub const SECBUFFER_STREAM_TRAILER: c_ulong = 6;
pub const SECBUFFER_STREAM_HEADER: c_ulong = 7;
pub const SECBUFFER_NEGOTIATION_INFO: c_ulong = 8;
pub const SECBUFFER_PADDING: c_ulong = 9;
pub const SECBUFFER_STREAM: c_ulong = 10;
pub const SECBUFFER_MECHLIST: c_ulong = 11;
pub const SECBUFFER_MECHLIST_SIGNATURE: c_ulong = 12;
pub const SECBUFFER_TARGET: c_ulong = 13;
pub const SECBUFFER_CHANNEL_BINDINGS: c_ulong = 14;
pub const SECBUFFER_CHANGE_PASS_RESPONSE: c_ulong = 15;
pub const SECBUFFER_TARGET_HOST: c_ulong = 16;
pub const SECBUFFER_ALERT: c_ulong = 17;
pub const SECBUFFER_APPLICATION_PROTOCOLS: c_ulong = 18;
pub const SECBUFFER_ATTRMASK: c_ulong = 0xF0000000;
pub const SECBUFFER_READONLY: c_ulong = 0x80000000;
pub const SECBUFFER_READONLY_WITH_CHECKSUM: c_ulong = 0x10000000;
pub const SECBUFFER_RESERVED: c_ulong = 0x60000000;
STRUCT!{struct SEC_NEGOTIATION_INFO {
    Size: c_ulong,
    NameLength: c_ulong,
    Name: *mut SEC_WCHAR,
    Reserved: *mut c_void,
}}
pub type PSEC_NEGOTIATION_INFO = *mut SEC_NEGOTIATION_INFO;
STRUCT!{struct SEC_CHANNEL_BINDINGS {
    dwInitiatorAddrType: c_ulong,
    cbInitiatorLength: c_ulong,
    dwInitiatorOffset: c_ulong,
    dwAcceptorAddrType: c_ulong,
    cbAcceptorLength: c_ulong,
    dwAcceptorOffset: c_ulong,
    cbApplicationDataLength: c_ulong,
    dwApplicationDataOffset: c_ulong,
}}
pub type PSEC_CHANNEL_BINDINGS = *mut SEC_CHANNEL_BINDINGS;
ENUM!{enum SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT {
    SecApplicationProtocolNegotiationExt_None,
    SecApplicationProtocolNegotiationExt_NPN,
    SecApplicationProtocolNegotiationExt_ALPN,
}}
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT = *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT;
STRUCT!{struct SEC_APPLICATION_PROTOCOL_LIST {
    ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    ProtocolListSize: c_ushort,
    ProtocolList: [c_uchar; 0],
}}
pub type PSEC_APPLICATION_PROTOCOL_LIST = *mut SEC_APPLICATION_PROTOCOL_LIST;
STRUCT!{struct SEC_APPLICATION_PROTOCOLS {
    ProtocolListsSize: c_ulong,
    ProtocolLists: [SEC_APPLICATION_PROTOCOL_LIST; ANYSIZE_ARRAY],
}}
pub type PSEC_APPLICATION_PROTOCOLS = *mut SEC_APPLICATION_PROTOCOLS;
pub const SECURITY_NATIVE_DREP: c_ulong = 0x00000010;
pub const SECURITY_NETWORK_DREP: c_ulong = 0x00000000;
pub const SECPKG_CRED_INBOUND: c_ulong = 0x00000001;
pub const SECPKG_CRED_OUTBOUND: c_ulong = 0x00000002;
pub const SECPKG_CRED_BOTH: c_ulong = 0x00000003;
pub const SECPKG_CRED_DEFAULT: c_ulong = 0x00000004;
pub const SECPKG_CRED_RESERVED: c_ulong = 0xF0000000;
pub const SECPKG_CRED_AUTOLOGON_RESTRICTED: c_ulong = 0x00000010;
pub const SECPKG_CRED_PROCESS_POLICY_ONLY: c_ulong = 0x00000020;
pub const ISC_REQ_DELEGATE: c_ulong = 0x00000001;
pub const ISC_REQ_MUTUAL_AUTH: c_ulong = 0x00000002;
pub const ISC_REQ_REPLAY_DETECT: c_ulong = 0x00000004;
pub const ISC_REQ_SEQUENCE_DETECT: c_ulong = 0x00000008;
pub const ISC_REQ_CONFIDENTIALITY: c_ulong = 0x00000010;
pub const ISC_REQ_USE_SESSION_KEY: c_ulong = 0x00000020;
pub const ISC_REQ_PROMPT_FOR_CREDS: c_ulong = 0x00000040;
pub const ISC_REQ_USE_SUPPLIED_CREDS: c_ulong = 0x00000080;
pub const ISC_REQ_ALLOCATE_MEMORY: c_ulong = 0x00000100;
pub const ISC_REQ_USE_DCE_STYLE: c_ulong = 0x00000200;
pub const ISC_REQ_DATAGRAM: c_ulong = 0x00000400;
pub const ISC_REQ_CONNECTION: c_ulong = 0x00000800;
pub const ISC_REQ_CALL_LEVEL: c_ulong = 0x00001000;
pub const ISC_REQ_FRAGMENT_SUPPLIED: c_ulong = 0x00002000;
pub const ISC_REQ_EXTENDED_ERROR: c_ulong = 0x00004000;
pub const ISC_REQ_STREAM: c_ulong = 0x00008000;
pub const ISC_REQ_INTEGRITY: c_ulong = 0x00010000;
pub const ISC_REQ_IDENTIFY: c_ulong = 0x00020000;
pub const ISC_REQ_NULL_SESSION: c_ulong = 0x00040000;
pub const ISC_REQ_MANUAL_CRED_VALIDATION: c_ulong = 0x00080000;
pub const ISC_REQ_RESERVED1: c_ulong = 0x00100000;
pub const ISC_REQ_FRAGMENT_TO_FIT: c_ulong = 0x00200000;
pub const ISC_REQ_FORWARD_CREDENTIALS: c_ulong = 0x00400000;
pub const ISC_REQ_NO_INTEGRITY: c_ulong = 0x00800000;
pub const ISC_REQ_USE_HTTP_STYLE: c_ulong = 0x01000000;
pub const ISC_REQ_UNVERIFIED_TARGET_NAME: c_ulong = 0x20000000;
pub const ISC_REQ_CONFIDENTIALITY_ONLY: c_ulong = 0x40000000;
pub const ISC_RET_DELEGATE: c_ulong = 0x00000001;
pub const ISC_RET_MUTUAL_AUTH: c_ulong = 0x00000002;
pub const ISC_RET_REPLAY_DETECT: c_ulong = 0x00000004;
pub const ISC_RET_SEQUENCE_DETECT: c_ulong = 0x00000008;
pub const ISC_RET_CONFIDENTIALITY: c_ulong = 0x00000010;
pub const ISC_RET_USE_SESSION_KEY: c_ulong = 0x00000020;
pub const ISC_RET_USED_COLLECTED_CREDS: c_ulong = 0x00000040;
pub const ISC_RET_USED_SUPPLIED_CREDS: c_ulong = 0x00000080;
pub const ISC_RET_ALLOCATED_MEMORY: c_ulong = 0x00000100;
pub const ISC_RET_USED_DCE_STYLE: c_ulong = 0x00000200;
pub const ISC_RET_DATAGRAM: c_ulong = 0x00000400;
pub const ISC_RET_CONNECTION: c_ulong = 0x00000800;
pub const ISC_RET_INTERMEDIATE_RETURN: c_ulong = 0x00001000;
pub const ISC_RET_CALL_LEVEL: c_ulong = 0x00002000;
pub const ISC_RET_EXTENDED_ERROR: c_ulong = 0x00004000;
pub const ISC_RET_STREAM: c_ulong = 0x00008000;
pub const ISC_RET_INTEGRITY: c_ulong = 0x00010000;
pub const ISC_RET_IDENTIFY: c_ulong = 0x00020000;
pub const ISC_RET_NULL_SESSION: c_ulong = 0x00040000;
pub const ISC_RET_MANUAL_CRED_VALIDATION: c_ulong = 0x00080000;
pub const ISC_RET_RESERVED1: c_ulong = 0x00100000;
pub const ISC_RET_FRAGMENT_ONLY: c_ulong = 0x00200000;
pub const ISC_RET_FORWARD_CREDENTIALS: c_ulong = 0x00400000;
pub const ISC_RET_USED_HTTP_STYLE: c_ulong = 0x01000000;
pub const ISC_RET_NO_ADDITIONAL_TOKEN: c_ulong = 0x02000000;
pub const ISC_RET_REAUTHENTICATION: c_ulong = 0x08000000;
pub const ISC_RET_CONFIDENTIALITY_ONLY: c_ulong = 0x40000000;
pub const ASC_REQ_DELEGATE: c_ulong = 0x00000001;
pub const ASC_REQ_MUTUAL_AUTH: c_ulong = 0x00000002;
pub const ASC_REQ_REPLAY_DETECT: c_ulong = 0x00000004;
pub const ASC_REQ_SEQUENCE_DETECT: c_ulong = 0x00000008;
pub const ASC_REQ_CONFIDENTIALITY: c_ulong = 0x00000010;
pub const ASC_REQ_USE_SESSION_KEY: c_ulong = 0x00000020;
pub const ASC_REQ_SESSION_TICKET: c_ulong = 0x00000040;
pub const ASC_REQ_ALLOCATE_MEMORY: c_ulong = 0x00000100;
pub const ASC_REQ_USE_DCE_STYLE: c_ulong = 0x00000200;
pub const ASC_REQ_DATAGRAM: c_ulong = 0x00000400;
pub const ASC_REQ_CONNECTION: c_ulong = 0x00000800;
pub const ASC_REQ_CALL_LEVEL: c_ulong = 0x00001000;
pub const ASC_REQ_EXTENDED_ERROR: c_ulong = 0x00008000;
pub const ASC_REQ_STREAM: c_ulong = 0x00010000;
pub const ASC_REQ_INTEGRITY: c_ulong = 0x00020000;
pub const ASC_REQ_LICENSING: c_ulong = 0x00040000;
pub const ASC_REQ_IDENTIFY: c_ulong = 0x00080000;
pub const ASC_REQ_ALLOW_NULL_SESSION: c_ulong = 0x00100000;
pub const ASC_REQ_ALLOW_NON_USER_LOGONS: c_ulong = 0x00200000;
pub const ASC_REQ_ALLOW_CONTEXT_REPLAY: c_ulong = 0x00400000;
pub const ASC_REQ_FRAGMENT_TO_FIT: c_ulong = 0x00800000;
pub const ASC_REQ_FRAGMENT_SUPPLIED: c_ulong = 0x00002000;
pub const ASC_REQ_NO_TOKEN: c_ulong = 0x01000000;
pub const ASC_REQ_PROXY_BINDINGS: c_ulong = 0x04000000;
pub const ASC_REQ_ALLOW_MISSING_BINDINGS: c_ulong = 0x10000000;
pub const ASC_RET_DELEGATE: c_ulong = 0x00000001;
pub const ASC_RET_MUTUAL_AUTH: c_ulong = 0x00000002;
pub const ASC_RET_REPLAY_DETECT: c_ulong = 0x00000004;
pub const ASC_RET_SEQUENCE_DETECT: c_ulong = 0x00000008;
pub const ASC_RET_CONFIDENTIALITY: c_ulong = 0x00000010;
pub const ASC_RET_USE_SESSION_KEY: c_ulong = 0x00000020;
pub const ASC_RET_SESSION_TICKET: c_ulong = 0x00000040;
pub const ASC_RET_ALLOCATED_MEMORY: c_ulong = 0x00000100;
pub const ASC_RET_USED_DCE_STYLE: c_ulong = 0x00000200;
pub const ASC_RET_DATAGRAM: c_ulong = 0x00000400;
pub const ASC_RET_CONNECTION: c_ulong = 0x00000800;
pub const ASC_RET_CALL_LEVEL: c_ulong = 0x00002000;
pub const ASC_RET_THIRD_LEG_FAILED: c_ulong = 0x00004000;
pub const ASC_RET_EXTENDED_ERROR: c_ulong = 0x00008000;
pub const ASC_RET_STREAM: c_ulong = 0x00010000;
pub const ASC_RET_INTEGRITY: c_ulong = 0x00020000;
pub const ASC_RET_LICENSING: c_ulong = 0x00040000;
pub const ASC_RET_IDENTIFY: c_ulong = 0x00080000;
pub const ASC_RET_NULL_SESSION: c_ulong = 0x00100000;
pub const ASC_RET_ALLOW_NON_USER_LOGONS: c_ulong = 0x00200000;
pub const ASC_RET_ALLOW_CONTEXT_REPLAY: c_ulong = 0x00400000;
pub const ASC_RET_FRAGMENT_ONLY: c_ulong = 0x00800000;
pub const ASC_RET_NO_TOKEN: c_ulong = 0x01000000;
pub const ASC_RET_NO_ADDITIONAL_TOKEN: c_ulong = 0x02000000;
pub const SECPKG_CRED_ATTR_NAMES: c_ulong = 1;
pub const SECPKG_CRED_ATTR_SSI_PROVIDER: c_ulong = 2;
pub const SECPKG_CRED_ATTR_KDC_PROXY_SETTINGS: c_ulong = 3;
pub const SECPKG_CRED_ATTR_CERT: c_ulong = 4;
STRUCT!{struct SecPkgCredentials_NamesW {
    sUserName: *mut SEC_WCHAR,
}}
pub type PSecPkgCredentials_NamesW = *mut SecPkgCredentials_NamesW;
STRUCT!{struct SecPkgCredentials_NamesA {
    sUserName: *mut SEC_CHAR,
}}
pub type PSecPkgCredentials_NamesA = *mut SecPkgCredentials_NamesA;
STRUCT!{struct SecPkgCredentials_SSIProviderW {
    sProviderName: *mut SEC_WCHAR,
    ProviderInfoLength: c_ulong,
    ProviderInfo: *mut c_char,
}}
pub type PSecPkgCredentials_SSIProviderW = *mut SecPkgCredentials_SSIProviderW;
STRUCT!{struct SecPkgCredentials_SSIProviderA {
    sProviderName: *mut SEC_CHAR,
    ProviderInfoLength: c_ulong,
    ProviderInfo: *mut c_char,
}}
pub type PSecPkgCredentials_SSIProviderA = *mut SecPkgCredentials_SSIProviderA;
pub const KDC_PROXY_SETTINGS_V1: ULONG = 1;
pub const KDC_PROXY_SETTINGS_FLAGS_FORCEPROXY: ULONG = 0x1;
STRUCT!{struct SecPkgCredentials_KdcProxySettingsW {
    Version: ULONG,
    Flags: ULONG,
    ProxyServerOffset: USHORT,
    ProxyServerLength: USHORT,
    ClientTlsCredOffset: USHORT,
    ClientTlsCredLength: USHORT,
}}
pub type PSecPkgCredentials_KdcProxySettingsW = *mut SecPkgCredentials_KdcProxySettingsW;
STRUCT!{struct SecPkgCredentials_Cert {
    EncodedCertSize: c_ulong,
    EncodedCert: *mut c_uchar,
}}
pub type PSecPkgCredentials_Cert = *mut SecPkgCredentials_Cert;
pub const SECPKG_ATTR_SIZES: c_ulong = 0;
pub const SECPKG_ATTR_NAMES: c_ulong = 1;
pub const SECPKG_ATTR_LIFESPAN: c_ulong = 2;
pub const SECPKG_ATTR_DCE_INFO: c_ulong = 3;
pub const SECPKG_ATTR_STREAM_SIZES: c_ulong = 4;
pub const SECPKG_ATTR_KEY_INFO: c_ulong = 5;
pub const SECPKG_ATTR_AUTHORITY: c_ulong = 6;
pub const SECPKG_ATTR_PROTO_INFO: c_ulong = 7;
pub const SECPKG_ATTR_PASSWORD_EXPIRY: c_ulong = 8;
pub const SECPKG_ATTR_SESSION_KEY: c_ulong = 9;
pub const SECPKG_ATTR_PACKAGE_INFO: c_ulong = 10;
pub const SECPKG_ATTR_USER_FLAGS: c_ulong = 11;
pub const SECPKG_ATTR_NEGOTIATION_INFO: c_ulong = 12;
pub const SECPKG_ATTR_NATIVE_NAMES: c_ulong = 13;
pub const SECPKG_ATTR_FLAGS: c_ulong = 14;
pub const SECPKG_ATTR_USE_VALIDATED: c_ulong = 15;
pub const SECPKG_ATTR_CREDENTIAL_NAME: c_ulong = 16;
pub const SECPKG_ATTR_TARGET_INFORMATION: c_ulong = 17;
pub const SECPKG_ATTR_ACCESS_TOKEN: c_ulong = 18;
pub const SECPKG_ATTR_TARGET: c_ulong = 19;
pub const SECPKG_ATTR_AUTHENTICATION_ID: c_ulong = 20;
pub const SECPKG_ATTR_LOGOFF_TIME: c_ulong = 21;
pub const SECPKG_ATTR_NEGO_KEYS: c_ulong = 22;
pub const SECPKG_ATTR_PROMPTING_NEEDED: c_ulong = 24;
pub const SECPKG_ATTR_UNIQUE_BINDINGS: c_ulong = 25;
pub const SECPKG_ATTR_ENDPOINT_BINDINGS: c_ulong = 26;
pub const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: c_ulong = 27;
pub const SECPKG_ATTR_LAST_CLIENT_TOKEN_STATUS: c_ulong = 30;
pub const SECPKG_ATTR_NEGO_PKG_INFO: c_ulong = 31;
pub const SECPKG_ATTR_NEGO_STATUS: c_ulong = 32;
pub const SECPKG_ATTR_CONTEXT_DELETED: c_ulong = 33;
pub const SECPKG_ATTR_DTLS_MTU: c_ulong = 34;
pub const SECPKG_ATTR_DATAGRAM_SIZES: c_ulong = SECPKG_ATTR_STREAM_SIZES;
pub const SECPKG_ATTR_SUBJECT_SECURITY_ATTRIBUTES: c_ulong = 128;
pub const SECPKG_ATTR_APPLICATION_PROTOCOL: c_ulong = 35;
STRUCT!{struct SecPkgContext_SubjectAttributes {
    AttributeInfo: *mut c_void,
}}
pub type PSecPkgContext_SubjectAttributes = *mut SecPkgContext_SubjectAttributes;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_KERBEROS: c_ulong = 0x1;
pub const SECPKG_ATTR_NEGO_INFO_FLAG_NO_NTLM: c_ulong = 0x2;
ENUM!{enum SECPKG_CRED_CLASS {
    SecPkgCredClass_None = 0,
    SecPkgCredClass_Ephemeral = 10,
    SecPkgCredClass_PersistedGeneric = 20,
    SecPkgCredClass_PersistedSpecific = 30,
    SecPkgCredClass_Explicit = 40,
}}
pub type PSECPKG_CRED_CLASS = *mut SECPKG_CRED_CLASS;
STRUCT!{struct SecPkgContext_CredInfo {
    CredClass: SECPKG_CRED_CLASS,
    IsPromptingNeeded: c_ulong,
}}
pub type PSecPkgContext_CredInfo = *mut SecPkgContext_CredInfo;
STRUCT!{struct SecPkgContext_NegoPackageInfo {
    PackageMask: c_ulong,
}}
pub type PSecPkgContext_NegoPackageInfo = *mut SecPkgContext_NegoPackageInfo;
STRUCT!{struct SecPkgContext_NegoStatus {
    LastStatus: c_ulong,
}}
pub type PSecPkgContext_NegoStatus = *mut SecPkgContext_NegoStatus;
STRUCT!{struct SecPkgContext_Sizes {
    cbMaxToken: c_ulong,
    cbMaxSignature: c_ulong,
    cbBlockSize: c_ulong,
    cbSecurityTrailer: c_ulong,
}}
pub type PSecPkgContext_Sizes = *mut SecPkgContext_Sizes;
STRUCT!{struct SecPkgContext_StreamSizes {
    cbHeader: c_ulong,
    cbTrailer: c_ulong,
    cbMaximumMessage: c_ulong,
    cBuffers: c_ulong,
    cbBlockSize: c_ulong,
}}
pub type PSecPkgContext_StreamSizes = *mut SecPkgContext_StreamSizes;
pub type SecPkgContext_DatagramSizes = SecPkgContext_StreamSizes;
pub type PSecPkgContext_DatagramSizes = PSecPkgContext_StreamSizes;
STRUCT!{struct SecPkgContext_NamesW {
    sUserName: *mut SEC_WCHAR,
}}
pub type PSecPkgContext_NamesW = *mut SecPkgContext_NamesW;
ENUM!{enum SECPKG_ATTR_LCT_STATUS {
    SecPkgAttrLastClientTokenYes,
    SecPkgAttrLastClientTokenNo,
    SecPkgAttrLastClientTokenMaybe,
}}
pub type PSECPKG_ATTR_LCT_STATUS = *mut SECPKG_ATTR_LCT_STATUS;
STRUCT!{struct SecPkgContext_LastClientTokenStatus {
    LastClientTokenStatus: SECPKG_ATTR_LCT_STATUS,
}}
pub type PSecPkgContext_LastClientTokenStatus = *mut SecPkgContext_LastClientTokenStatus;
STRUCT!{struct SecPkgContext_NamesA {
    sUserName: *mut SEC_CHAR,
}}
pub type PSecPkgContext_NamesA = *mut SecPkgContext_NamesA;
STRUCT!{struct SecPkgContext_Lifespan {
    tsStart: TimeStamp,
    tsExpiry: TimeStamp,
}}
pub type PSecPkgContext_Lifespan = *mut SecPkgContext_Lifespan;
STRUCT!{struct SecPkgContext_DceInfo {
    AuthzSvc: c_ulong,
    pPac: *mut c_void,
}}
pub type PSecPkgContext_DceInfo = *mut SecPkgContext_DceInfo;
STRUCT!{struct SecPkgContext_KeyInfoA {
    sSignatureAlgorithmName: *mut SEC_CHAR,
    sEncryptAlgorithmName: *mut SEC_CHAR,
    KeySize: c_ulong,
    SignatureAlgorithm: c_ulong,
    EncryptAlgorithm: c_ulong,
}}
pub type PSecPkgContext_KeyInfoA = *mut SecPkgContext_KeyInfoA;
STRUCT!{struct SecPkgContext_KeyInfoW {
    sSignatureAlgorithmName: *mut SEC_WCHAR,
    sEncryptAlgorithmName: *mut SEC_WCHAR,
    KeySize: c_ulong,
    SignatureAlgorithm: c_ulong,
    EncryptAlgorithm: c_ulong,
}}
pub type PSecPkgContext_KeyInfoW = *mut SecPkgContext_KeyInfoW;
STRUCT!{struct SecPkgContext_AuthorityA {
    sAuthorityName: *mut SEC_CHAR,
}}
pub type PSecPkgContext_AuthorityA = *mut SecPkgContext_AuthorityA;
STRUCT!{struct SecPkgContext_AuthorityW {
    sAuthorityName: *mut SEC_WCHAR,
}}
pub type PSecPkgContext_AuthorityW = *mut SecPkgContext_AuthorityW;
STRUCT!{struct SecPkgContext_ProtoInfoA {
    sProtocolName: *mut SEC_CHAR,
    majorVersion: c_ulong,
    minorVersion: c_ulong,
}}
pub type PSecPkgContext_ProtoInfoA = *mut SecPkgContext_ProtoInfoA;
STRUCT!{struct SecPkgContext_ProtoInfoW {
    sProtocolName: *mut SEC_WCHAR,
    majorVersion: c_ulong,
    minorVersion: c_ulong,
}}
pub type PSecPkgContext_ProtoInfoW = *mut SecPkgContext_ProtoInfoW;
STRUCT!{struct SecPkgContext_PasswordExpiry {
    tsPasswordExpires: TimeStamp,
}}
pub type PSecPkgContext_PasswordExpiry = *mut SecPkgContext_PasswordExpiry;
STRUCT!{struct SecPkgContext_LogoffTime {
    tsLogoffTime: TimeStamp,
}}
pub type PSecPkgContext_LogoffTime = *mut SecPkgContext_LogoffTime;
STRUCT!{struct SecPkgContext_SessionKey {
    SessionKeyLength: c_ulong,
    SessionKey: *mut c_uchar,
}}
pub type PSecPkgContext_SessionKey = *mut SecPkgContext_SessionKey;
STRUCT!{struct SecPkgContext_NegoKeys {
    KeyType: c_ulong,
    KeyLength: c_ushort,
    KeyValue: *mut c_uchar,
    VerifyKeyType: c_ulong,
    VerifyKeyLength: c_ushort,
    VerifyKeyValue: *mut c_uchar,
}}
pub type PSecPkgContext_NegoKeys = *mut SecPkgContext_NegoKeys;
STRUCT!{struct SecPkgContext_PackageInfoW {
    PackageInfo: PSecPkgInfoW,
}}
pub type PSecPkgContext_PackageInfoW = *mut SecPkgContext_PackageInfoW;
STRUCT!{struct SecPkgContext_PackageInfoA {
    PackageInfo: PSecPkgInfoA,
}}
pub type PSecPkgContext_PackageInfoA = *mut SecPkgContext_PackageInfoA;
STRUCT!{struct SecPkgContext_UserFlags {
    UserFlags: c_ulong,
}}
pub type PSecPkgContext_UserFlags = *mut SecPkgContext_UserFlags;
STRUCT!{struct SecPkgContext_Flags {
    Flags: c_ulong,
}}
pub type PSecPkgContext_Flags = *mut SecPkgContext_Flags;
STRUCT!{struct SecPkgContext_NegotiationInfoA {
    PackageInfo: PSecPkgInfoA,
    NegotiationState: c_ulong,
}}
pub type PSecPkgContext_NegotiationInfoA = *mut SecPkgContext_NegotiationInfoA;
STRUCT!{struct SecPkgContext_NegotiationInfoW {
    PackageInfo: PSecPkgInfoW,
    NegotiationState: c_ulong,
}}
pub type PSecPkgContext_NegotiationInfoW = *mut SecPkgContext_NegotiationInfoW;
pub const SECPKG_NEGOTIATION_COMPLETE: c_ulong = 0;
pub const SECPKG_NEGOTIATION_OPTIMISTIC: c_ulong = 1;
pub const SECPKG_NEGOTIATION_IN_PROGRESS: c_ulong = 2;
pub const SECPKG_NEGOTIATION_DIRECT: c_ulong = 3;
pub const SECPKG_NEGOTIATION_TRY_MULTICRED: c_ulong = 4;
STRUCT!{struct SecPkgContext_NativeNamesW {
    sClientName: *mut SEC_WCHAR,
    sServerName: *mut SEC_WCHAR,
}}
pub type PSecPkgContext_NativeNamesW = *mut SecPkgContext_NativeNamesW;
STRUCT!{struct SecPkgContext_NativeNamesA {
    sClientName: *mut SEC_CHAR,
    sServerName: *mut SEC_CHAR,
}}
pub type PSecPkgContext_NativeNamesA = *mut SecPkgContext_NativeNamesA;
STRUCT!{struct SecPkgContext_CredentialNameW {
    CredentialType: c_ulong,
    sCredentialName: *mut SEC_WCHAR,
}}
pub type PSecPkgContext_CredentialNameW = *mut SecPkgContext_CredentialNameW;
STRUCT!{struct SecPkgContext_CredentialNameA {
    CredentialType: c_ulong,
    sCredentialName: *mut SEC_CHAR,
}}
pub type PSecPkgContext_CredentialNameA = *mut SecPkgContext_CredentialNameA;
STRUCT!{struct SecPkgContext_AccessToken {
    AccessToken: *mut c_void,
}}
pub type PSecPkgContext_AccessToken = *mut SecPkgContext_AccessToken;
STRUCT!{struct SecPkgContext_TargetInformation {
    MarshalledTargetInfoLength: c_ulong,
    MarshalledTargetInfo: *mut c_uchar,
}}
pub type PSecPkgContext_TargetInformation = *mut SecPkgContext_TargetInformation;
STRUCT!{struct SecPkgContext_AuthzID {
    AuthzIDLength: c_ulong,
    AuthzID: *mut c_char,
}}
pub type PSecPkgContext_AuthzID = *mut SecPkgContext_AuthzID;
STRUCT!{struct SecPkgContext_Target {
    TargetLength: c_ulong,
    Target: *mut c_char,
}}
pub type PSecPkgContext_Target = *mut SecPkgContext_Target;
STRUCT!{struct SecPkgContext_ClientSpecifiedTarget {
    sTargetName: *mut SEC_WCHAR,
}}
pub type PSecPkgContext_ClientSpecifiedTarget = *mut SecPkgContext_ClientSpecifiedTarget;
STRUCT!{struct SecPkgContext_Bindings {
    BindingsLength: c_ulong,
    Bindings: *mut SEC_CHANNEL_BINDINGS,
}}
pub type PSecPkgContext_Bindings = *mut SecPkgContext_Bindings;
ENUM!{enum SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS {
    SecApplicationProtocolNegotiationStatus_None,
    SecApplicationProtocolNegotiationStatus_Success,
    SecApplicationProtocolNegotiationStatus_SelectedClientOnly,
}}
pub type PSEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS =
    *mut SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS;
pub const MAX_PROTOCOL_ID_SIZE: usize = 0xff;
STRUCT!{struct SecPkgContext_ApplicationProtocol {
    ProtoNegoStatus: SEC_APPLICATION_PROTOCOL_NEGOTIATION_STATUS,
    ProtoNegoExt: SEC_APPLICATION_PROTOCOL_NEGOTIATION_EXT,
    ProtocolIdSize: c_uchar,
    ProtocolId: [c_uchar; MAX_PROTOCOL_ID_SIZE],
}}
pub type PSecPkgContext_ApplicationProtocol = *mut SecPkgContext_ApplicationProtocol;
FN!{stdcall SEC_GET_KEY_FN(
    Arg: *mut c_void,
    Principal: *mut c_void,
    KeyVer: c_ulong,
    Key: *mut *mut c_void,
    Status: *mut SECURITY_STATUS,
) -> ()}
pub const SECPKG_CONTEXT_EXPORT_RESET_NEW: c_ulong = 0x00000001;
pub const SECPKG_CONTEXT_EXPORT_DELETE_OLD: c_ulong = 0x00000002;
pub const SECPKG_CONTEXT_EXPORT_TO_KERNEL: c_ulong = 0x00000004;
extern "system" {
    pub fn AcquireCredentialsHandleW(
        pszPrincipal: LPWSTR,
        pszPackage: LPWSTR,
        fCredentialUse: c_ulong,
        pvLogonId: *mut c_void,
        pAuthData: *mut c_void,
        pGetKeyFn: SEC_GET_KEY_FN,
        pvGetKeyArgument: *mut c_void,
        phCredential: PCredHandle,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
}
FN!{stdcall ACQUIRE_CREDENTIALS_HANDLE_FN_W(
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    c_ulong,
    *mut c_void,
    *mut c_void,
    SEC_GET_KEY_FN,
    *mut c_void,
    PCredHandle,
    PTimeStamp,
) -> SECURITY_STATUS}
extern "system" {
    pub fn AcquireCredentialsHandleA(
        pszPrincipal: LPSTR,
        pszPackage: LPSTR,
        fCredentialUse: c_ulong,
        pvLogonId: *mut c_void,
        pAuthData: *mut c_void,
        pGetKeyFn: SEC_GET_KEY_FN,
        pvGetKeyArgument: *mut c_void,
        phCredential: PCredHandle,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
}
FN!{stdcall ACQUIRE_CREDENTIALS_HANDLE_FN_A(
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    c_ulong,
    *mut c_void,
    *mut c_void,
    SEC_GET_KEY_FN,
    *mut c_void,
    PCredHandle,
    PTimeStamp,
) -> SECURITY_STATUS}
extern "system" {
    pub fn FreeCredentialsHandle(
        phCredential: PCredHandle,
    ) -> SECURITY_STATUS;
}
FN!{stdcall FREE_CREDENTIALS_HANDLE_FN(
    PCredHandle,
) -> SECURITY_STATUS}
extern "system" {
    pub fn AddCredentialsW(
        hCredentials: PCredHandle,
        pszPrincipal: LPWSTR,
        pszPackage: LPWSTR,
        fCredentialUse: c_ulong,
        pAuthData: *mut c_void,
        pGetKeyFn: SEC_GET_KEY_FN,
        pvGetKeyArgument: *mut c_void,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
}
FN!{stdcall ADD_CREDENTIALS_FN_W(
    PCredHandle,
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    c_ulong,
    *mut c_void,
    SEC_GET_KEY_FN,
    *mut c_void,
    PTimeStamp,
) -> SECURITY_STATUS}
extern "system" {
    pub fn AddCredentialsA(
        hCredentials: PCredHandle,
        pszPrincipal: LPSTR,
        pszPackage: LPSTR,
        fCredentialUse: c_ulong,
        pAuthData: *mut c_void,
        pGetKeyFn: SEC_GET_KEY_FN,
        pvGetKeyArgument: *mut c_void,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
}
FN!{stdcall ADD_CREDENTIALS_FN_A(
    PCredHandle,
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    c_ulong,
    *mut c_void,
    SEC_GET_KEY_FN,
    *mut c_void,
    PTimeStamp,
) -> SECURITY_STATUS}
extern "system" {
    // pub fn spiCreateAsyncContext();
    // pub fn SspiFreeAsyncContext();
    // pub fn SspiReinitAsyncContext();
    // pub fn SspiSetAsyncNotifyCallback();
    // pub fn SspiAsyncContextRequiresNotify();
    // pub fn SspiGetAsyncCallStatus();
    // pub fn SspiAcquireCredentialsHandleAsyncW();
    // pub fn SspiAcquireCredentialsHandleAsyncA();
    // pub fn SspiInitializeSecurityContextAsyncW();
    // pub fn SspiInitializeSecurityContextAsyncA();
    // pub fn SspiAcceptSecurityContextAsync();
    // pub fn SspiFreeCredentialsHandleAsync();
    // pub fn SspiDeleteSecurityContextAsync();
    pub fn ChangeAccountPasswordW(
        pszPackageName: *mut SEC_WCHAR,
        pszDomainName: *mut SEC_WCHAR,
        pszAccountName: *mut SEC_WCHAR,
        pszOldPassword: *mut SEC_WCHAR,
        pszNewPassword: *mut SEC_WCHAR,
        bImpersonating: BOOLEAN,
        dwReserved: c_ulong,
        pOutput: PSecBufferDesc,
    ) -> SECURITY_STATUS;
}
FN!{stdcall CHANGE_PASSWORD_FN_W(
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    *mut SEC_WCHAR,
    BOOLEAN,
    c_ulong,
    PSecBufferDesc,
) -> SECURITY_STATUS}
extern "system" {
    pub fn ChangeAccountPasswordA(
        pszPackageName: *mut SEC_CHAR,
        pszDomainName: *mut SEC_CHAR,
        pszAccountName: *mut SEC_CHAR,
        pszOldPassword: *mut SEC_CHAR,
        pszNewPassword: *mut SEC_CHAR,
        bImpersonating: BOOLEAN,
        dwReserved: c_ulong,
        pOutput: PSecBufferDesc,
    ) -> SECURITY_STATUS;
}
FN!{stdcall CHANGE_PASSWORD_FN_A(
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    *mut SEC_CHAR,
    BOOLEAN,
    c_ulong,
    PSecBufferDesc,
) -> SECURITY_STATUS}
extern "system" {
    pub fn InitializeSecurityContextW(
        phCredential: PCredHandle,
        phContext: PCtxtHandle,
        pszTargetName: *mut SEC_WCHAR,
        fContextReq: c_ulong,
        Reserved1: c_ulong,
        TargetDataRep: c_ulong,
        pInput: PSecBufferDesc,
        Reserved2: c_ulong,
        phNewContext: PCtxtHandle,
        pOutput: PSecBufferDesc,
        pfContextAttr: *mut c_ulong,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
}
// INITIALIZE_SECURITY_CONTEXT_FN_W
extern "system" {
    pub fn InitializeSecurityContextA(
        phCredential: PCredHandle,
        phContext: PCtxtHandle,
        pszTargetName: *mut SEC_CHAR,
        fContextReq: c_ulong,
        Reserved1: c_ulong,
        TargetDataRep: c_ulong,
        pInput: PSecBufferDesc,
        Reserved2: c_ulong,
        phNewContext: PCtxtHandle,
        pOutput: PSecBufferDesc,
        pfContextAttr: *mut c_ulong,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn AcceptSecurityContext(
        phCredential: PCredHandle,
        phContext: PCtxtHandle,
        pInput: PSecBufferDesc,
        fContextReq: c_ulong,
        TargetDataRep: c_ulong,
        phNewContext: PCtxtHandle,
        pOutput: PSecBufferDesc,
        pfContextAttr: *mut c_ulong,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn CompleteAuthToken(
        phContext: PCtxtHandle,
        pToken: PSecBufferDesc,
    ) -> SECURITY_STATUS;
    pub fn ImpersonateSecurityContext(
        phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    pub fn RevertSecurityContext(
        phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityContextToken(
        phContext: PCtxtHandle,
        Token: *mut *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn DeleteSecurityContext(
        phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    pub fn ApplyControlToken(
        phContext: PCtxtHandle,
        pInput: PSecBufferDesc,
    ) -> SECURITY_STATUS;
    pub fn QueryContextAttributesW(
        phContext: PCtxtHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    // pub fn QueryContextAttributesExW();
    pub fn QueryContextAttributesA(
        phContext: PCtxtHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    // pub fn QueryContextAttributesExA();
    pub fn SetContextAttributesW(
        phContext: PCtxtHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
        cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn SetContextAttributesA(
        phContext: PCtxtHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
        cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn QueryCredentialsAttributesW(
        phCredential: PCredHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    // pub fn QueryCredentialsAttributesExW();
    pub fn QueryCredentialsAttributesA(
        phCredential: PCredHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    // pub fn QueryCredentialsAttributesExA();
    pub fn SetCredentialsAttributesW(
        phCredential: PCredHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
        cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn SetCredentialsAttributesA(
        phCredential: PCredHandle,
        ulAttribute: c_ulong,
        pBuffer: *mut c_void,
        cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn FreeContextBuffer(
        pvContextBuffer: PVOID,
    ) -> SECURITY_STATUS;
    pub fn MakeSignature(
        phContext: PCtxtHandle,
        fQOP: c_ulong,
        pMessage: PSecBufferDesc,
        MessageSeqNo: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn VerifySignature(
        phContext: PCtxtHandle,
        pMessage: PSecBufferDesc,
        MessageSeqNo: c_ulong,
        pfQOP: *mut c_ulong,
    ) -> SECURITY_STATUS;
    pub fn EncryptMessage(
        phContext: PCtxtHandle,
        fQOP: c_ulong,
        pMessage: PSecBufferDesc,
        MessageSeqNo: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn DecryptMessage(
        phContext: PCtxtHandle,
        pMessage: PSecBufferDesc,
        MessageSeqNo: c_ulong,
        pfQOP: *mut c_ulong,
    ) -> SECURITY_STATUS;
    pub fn EnumerateSecurityPackagesW(
        pcPackages: *mut c_ulong,
        ppPackageInfo: *mut PSecPkgInfoW,
    ) -> SECURITY_STATUS;
    pub fn EnumerateSecurityPackagesA(
        pcPackages: *mut c_ulong,
        ppPackageInfo: *mut PSecPkgInfoA,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityPackageInfoW(
        pszPackageName: LPWSTR,
        ppPackageInfo: *mut PSecPkgInfoW,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityPackageInfoA(
        pszPackageName: LPSTR,
        ppPackageInfo: *mut PSecPkgInfoA,
    ) -> SECURITY_STATUS;
}
ENUM!{enum SecDelegationType {
    SecFull,
    SecService,
    SecTree,
    SecDirectory,
    SecObject,
}}
pub type PSecDelegationType = *mut SecDelegationType;
extern "system" {
    // pub fn DelegateSecurityContext();
    pub fn ExportSecurityContext(
        phContext: PCtxtHandle,
        fFlags: ULONG,
        pPackedContext: PSecBuffer,
        pToken: *mut *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn ImportSecurityContextW(
        pszPackage: LPWSTR,
        pPackedContext: PSecBuffer,
        Token: *mut c_void,
        phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    pub fn ImportSecurityContextA(
        pszPackage: LPSTR,
        pPackedContext: PSecBuffer,
        Token: *mut c_void,
        phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
// pub fn SecMakeSPN();
// pub fn SecMakeSPNEx();
// pub fn SecMakeSPNEx2();
// pub fn SecLookupAccountSid();
// pub fn SecLookupAccountName();
// pub fn SecLookupWellKnownSid();
}
extern "system" {
    // pub fn InitSecurityInterfaceA();
    // pub fn InitSecurityInterfaceW();
    // pub fn SaslEnumerateProfilesA();
    // pub fn SaslEnumerateProfilesW();
    // pub fn SaslGetProfilePackageA();
    // pub fn SaslGetProfilePackageW();
    // pub fn SaslIdentifyPackageA();
    // pub fn SaslIdentifyPackageW();
    // pub fn SaslInitializeSecurityContextW();
    // pub fn SaslInitializeSecurityContextA();
    // pub fn SaslAcceptSecurityContext();
    // pub fn SaslSetContextOption();
    // pub fn SaslGetContextOption();
}
pub type PSEC_WINNT_AUTH_IDENTITY_OPAQUE = PVOID;
extern "system" {
    pub fn SspiPromptForCredentialsW(
        pszTargetName: PCWSTR,
        pUiInfo: PCREDUI_INFOW,
        dwAuthError: c_ulong,
        pszPackage: PCWSTR,
        pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        ppAuthIdentity: *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        pfSave: *mut c_int,
        dwFlags: c_ulong,
    ) -> c_ulong;
    pub fn SspiPromptForCredentialsA(
        pszTargetName: PCSTR,
        pUiInfo: PCREDUI_INFOA,
        dwAuthError: c_ulong,
        pszPackage: PCSTR,
        pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        ppAuthIdentity: *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        pfSave: *mut c_int,
        dwFlags: c_ulong,
    ) -> c_ulong;
}
STRUCT!{struct SEC_WINNT_AUTH_BYTE_VECTOR {
    ByteArrayOffset: c_ulong,
    ByteArrayLength: c_ushort,
}}
pub type PSEC_WINNT_AUTH_BYTE_VECTOR = *mut SEC_WINNT_AUTH_BYTE_VECTOR;
STRUCT!{struct SEC_WINNT_AUTH_DATA {
    CredType: GUID,
    CredData: SEC_WINNT_AUTH_BYTE_VECTOR,
}}
pub type PSEC_WINNT_AUTH_DATA = *mut SEC_WINNT_AUTH_DATA;
STRUCT!{struct SEC_WINNT_AUTH_PACKED_CREDENTIALS {
    cbHeaderLength: c_ushort,
    cbStructureLength: c_ushort,
    AuthData: SEC_WINNT_AUTH_DATA,
}}
pub type PSEC_WINNT_AUTH_PACKED_CREDENTIALS = *mut SEC_WINNT_AUTH_PACKED_CREDENTIALS;
DEFINE_GUID!{SEC_WINNT_AUTH_DATA_TYPE_PASSWORD,
    0x28bfc32f, 0x10f6, 0x4738, 0x98, 0xd1, 0x1a, 0xc0, 0x61, 0xdf, 0x71, 0x6a}
DEFINE_GUID!{SEC_WINNT_AUTH_DATA_TYPE_CERT,
    0x235f69ad, 0x73fb, 0x4dbc, 0x82, 0x3, 0x6, 0x29, 0xe7, 0x39, 0x33, 0x9b}
STRUCT!{struct SEC_WINNT_AUTH_DATA_PASSWORD {
    UnicodePassword: SEC_WINNT_AUTH_BYTE_VECTOR,
}}
pub type PSEC_WINNT_AUTH_DATA_PASSWORD = *mut SEC_WINNT_AUTH_DATA_PASSWORD;
DEFINE_GUID!{SEC_WINNT_AUTH_DATA_TYPE_CSP_DATA,
    0x68fd9879, 0x79c, 0x4dfe, 0x82, 0x81, 0x57, 0x8a, 0xad, 0xc1, 0xc1, 0x0}
// GUID SEC_WINNT_AUTH_DATA_TYPE_SMARTCARD_CONTEXTS
STRUCT!{struct SEC_WINNT_AUTH_CERTIFICATE_DATA {
    cbHeaderLength: c_ushort,
    cbStructureLength: c_ushort,
    Certificate: SEC_WINNT_AUTH_BYTE_VECTOR,
}}
pub type PSEC_WINNT_AUTH_CERTIFICATE_DATA = *mut SEC_WINNT_AUTH_CERTIFICATE_DATA;
STRUCT!{struct SEC_WINNT_CREDUI_CONTEXT_VECTOR {
    CredUIContextArrayOffset: ULONG,
    CredUIContextCount: USHORT,
}}
pub type PSEC_WINNT_CREDUI_CONTEXT_VECTOR = *mut SEC_WINNT_CREDUI_CONTEXT_VECTOR;
STRUCT!{struct SEC_WINNT_AUTH_SHORT_VECTOR {
    ShortArrayOffset: ULONG,
    ShortArrayCount: USHORT,
}}
pub type PSEC_WINNT_AUTH_SHORT_VECTOR = *mut SEC_WINNT_AUTH_SHORT_VECTOR;
extern "system" {
    pub fn SspiGetCredUIContext(
        ContextHandle: HANDLE,
        CredType: *mut GUID,
        LogonId: *mut LUID,
        CredUIContexts: *mut PSEC_WINNT_CREDUI_CONTEXT_VECTOR,
        TokenHandle: *mut HANDLE,
    ) -> SECURITY_STATUS;
    pub fn SspiUpdateCredentials(
        ContextHandle: HANDLE,
        CredType: *mut GUID,
        FlatCredUIContextLength: ULONG,
        FlatCredUIContext: PUCHAR,
    ) -> SECURITY_STATUS;
}
STRUCT!{struct CREDUIWIN_MARSHALED_CONTEXT {
    StructureType: GUID,
    cbHeaderLength: USHORT,
    LogonId: LUID,
    MarshaledDataType: GUID,
    MarshaledDataOffset: ULONG,
    MarshaledDataLength: USHORT,
}}
pub type PCREDUIWIN_MARSHALED_CONTEXT = *mut CREDUIWIN_MARSHALED_CONTEXT;
STRUCT!{struct SEC_WINNT_CREDUI_CONTEXT {
    cbHeaderLength: USHORT,
    CredUIContextHandle: HANDLE,
    UIInfo: PCREDUI_INFOW,
    dwAuthError: ULONG,
    pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
    TargetName: PUNICODE_STRING,
}}
pub type PSEC_WINNT_CREDUI_CONTEXT = *mut SEC_WINNT_CREDUI_CONTEXT;
// GUID CREDUIWIN_STRUCTURE_TYPE_SSPIPFC
// GUID SSPIPFC_STRUCTURE_TYPE_CREDUI_CONTEXT
extern "system" {
    pub fn SspiUnmarshalCredUIContext(
        MarshaledCredUIContext: PUCHAR,
        MarshaledCredUIContextLength: ULONG,
        CredUIContext: *mut PSEC_WINNT_CREDUI_CONTEXT,
    ) -> SECURITY_STATUS;
    // pub fn SspiPrepareForCredRead();
    // pub fn SspiPrepareForCredWrite();
    // pub fn SspiEncryptAuthIdentity();
    // pub fn SspiEncryptAuthIdentityEx();
    // pub fn SspiDecryptAuthIdentity();
    // pub fn SspiDecryptAuthIdentityEx();
    // pub fn SspiIsAuthIdentityEncrypted();
    // pub fn SspiEncodeAuthIdentityAsStrings();
    // pub fn SspiValidateAuthIdentity();
    // pub fn SspiCopyAuthIdentity();
    // pub fn SspiFreeAuthIdentity();
    // pub fn SspiZeroAuthIdentity();
    // pub fn SspiLocalFree();
    // pub fn SspiEncodeStringsAsAuthIdentity();
    // pub fn SspiCompareAuthIdentities();
    // pub fn SspiMarshalAuthIdentity();
    // pub fn SspiUnmarshalAuthIdentity();
    pub fn SspiIsPromptingNeeded(
        ErrorOrNtStatus: c_ulong,
    ) -> BOOLEAN;
    // pub fn SspiGetTargetHostName();
    // pub fn SspiExcludePackage();
    // pub fn AddSecurityPackageA();
    // pub fn AddSecurityPackageW();
    // pub fn DeleteSecurityPackageA();
    // pub fn DeleteSecurityPackageW();
}
