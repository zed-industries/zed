// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module contains the DCE RPC runtime APIs.
use ctypes::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void, wchar_t};
use shared::guiddef::GUID;
use shared::minwindef::DWORD;
use shared::rpc::{I_RPC_HANDLE, RPC_STATUS};
pub type RPC_CSTR = *mut c_uchar;
pub type RPC_WSTR = *mut wchar_t;
pub type RPC_CWSTR = *const wchar_t;
pub type RPC_BINDING_HANDLE = I_RPC_HANDLE;
pub type handle_t = RPC_BINDING_HANDLE;
pub type rpc_binding_handle_t = RPC_BINDING_HANDLE;
pub type UUID = GUID;
pub type uuid_t = UUID;
STRUCT!{struct RPC_BINDING_VECTOR {
    Count: c_ulong,
    BindingH: [RPC_BINDING_HANDLE; 1],
}}
pub type rpc_binding_vector_t = RPC_BINDING_VECTOR;
STRUCT!{struct UUID_VECTOR {
    Count: c_ulong,
    Uuid: [*mut UUID; 1],
}}
pub type uuid_vector_t = UUID_VECTOR;
pub type RPC_IF_HANDLE = *mut c_void;
STRUCT!{struct RPC_IF_ID {
    Uuid: UUID,
    VersMajor: c_ushort,
    VersMinor: c_ushort,
}}
pub const RPC_C_BINDING_INFINITE_TIMEOUT: DWORD = 10;
pub const RPC_C_BINDING_MIN_TIMEOUT: DWORD = 0;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: DWORD = 5;
pub const RPC_C_BINDING_MAX_TIMEOUT: DWORD = 9;
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: c_int = -1;
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: DWORD = 1234;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: DWORD = 10;
pub const RPC_C_BIND_TO_ALL_NICS: DWORD = 1;
pub const RPC_C_USE_INTERNET_PORT: DWORD = 0x1;
pub const RPC_C_USE_INTRANET_PORT: DWORD = 0x2;
pub const RPC_C_DONT_FAIL: DWORD = 0x4;
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: DWORD = 0x8;
pub const RPC_C_MQ_TEMPORARY: DWORD = 0x0000;
pub const RPC_C_MQ_PERMANENT: DWORD = 0x0001;
pub const RPC_C_MQ_CLEAR_ON_OPEN: DWORD = 0x0002;
pub const RPC_C_MQ_USE_EXISTING_SECURITY: DWORD = 0x0004;
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: DWORD = 0x0000;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: DWORD = 0x0008;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: DWORD = 0x0010;
pub const RPC_C_OPT_MQ_DELIVERY: DWORD = 1;
pub const RPC_C_OPT_MQ_PRIORITY: DWORD = 2;
pub const RPC_C_OPT_MQ_JOURNAL: DWORD = 3;
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: DWORD = 4;
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: DWORD = 5;
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: DWORD = 6;
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: DWORD = 7;
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: DWORD = 8;
pub const RPC_C_OPT_BINDING_NONCAUSAL: DWORD = 9;
pub const RPC_C_OPT_SECURITY_CALLBACK: DWORD = 10;
pub const RPC_C_OPT_UNIQUE_BINDING: DWORD = 11;
pub const RPC_C_OPT_CALL_TIMEOUT: DWORD = 12;
pub const RPC_C_OPT_DONT_LINGER: DWORD = 13;
pub const RPC_C_OPT_TRUST_PEER: DWORD = 14;
pub const RPC_C_OPT_ASYNC_BLOCK: DWORD = 15;
pub const RPC_C_OPT_OPTIMIZE_TIME: DWORD = 16;
pub const RPC_C_OPT_MAX_OPTIONS: DWORD = 17;
pub const RPC_C_MQ_EXPRESS: DWORD = 0;
pub const RPC_C_MQ_RECOVERABLE: DWORD = 1;
pub const RPC_C_MQ_JOURNAL_NONE: DWORD = 0;
pub const RPC_C_MQ_JOURNAL_DEADLETTER: DWORD = 1;
pub const RPC_C_MQ_JOURNAL_ALWAYS: DWORD = 2;
pub const RPC_C_FULL_CERT_CHAIN: DWORD = 0x0001;
STRUCT!{struct RPC_PROTSEQ_VECTORA {
    Count: c_uint,
    Protseq: [*mut c_uchar; 1],
}}
STRUCT!{struct RPC_PROTSEQ_VECTORW {
    Count: c_uint,
    Protseq: [*mut c_ushort; 1],
}}
STRUCT!{struct RPC_POLICY {
    Length: c_uint,
    EndpointFlags: c_ulong,
    NICFlags: c_ulong,
}}
pub type PRPC_POLICY = *mut RPC_POLICY;
FN!{stdcall RPC_OBJECT_INQ_FN(
    ObjectUuid: *mut UUID,
    TypeUuid: *mut UUID,
    Status: *mut RPC_STATUS,
) -> ()}
FN!{stdcall RPC_IF_CALLBACK_FN(
    InterfaceUuid: RPC_IF_HANDLE,
    Context: *mut c_void,
) -> RPC_STATUS}
FN!{stdcall RPC_SECURITY_CALLBACK_FN(
    Context: *mut c_void,
) -> ()}
pub type RPC_MGR_EPV = c_void;
STRUCT!{struct RPC_STATS_VECTOR {
    Count: c_uint,
    Stats: [c_ulong; 1],
}}
pub const RPC_C_STATS_CALLS_IN: c_ulong = 0;
pub const RPC_C_STATS_CALLS_OUT: c_ulong = 1;
pub const RPC_C_STATS_PKTS_IN: c_ulong = 2;
pub const RPC_C_STATS_PKTS_OUT: c_ulong = 3;
STRUCT!{struct RPC_IF_ID_VECTOR {
    Count: c_ulong,
    IfId: [*mut RPC_IF_ID; 1],
}}
pub type RPC_AUTH_IDENTITY_HANDLE = *mut c_void;
pub type RPC_AUTHZ_HANDLE = *mut c_void;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: DWORD = 0;
pub const RPC_C_AUTHN_LEVEL_NONE: DWORD = 1;
pub const RPC_C_AUTHN_LEVEL_CONNECT: DWORD = 2;
pub const RPC_C_AUTHN_LEVEL_CALL: DWORD = 3;
pub const RPC_C_AUTHN_LEVEL_PKT: DWORD = 4;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: DWORD = 5;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: DWORD = 6;
pub const RPC_C_IMP_LEVEL_DEFAULT: DWORD = 0;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: DWORD = 1;
pub const RPC_C_IMP_LEVEL_IDENTIFY: DWORD = 2;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: DWORD = 3;
pub const RPC_C_IMP_LEVEL_DELEGATE: DWORD = 4;
pub const RPC_C_QOS_IDENTITY_STATIC: DWORD = 0;
pub const RPC_C_QOS_IDENTITY_DYNAMIC: DWORD = 1;
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: DWORD = 0x0;
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: DWORD = 0x1;
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: DWORD = 0x2;
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: DWORD = 0x4;
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: DWORD = 0x8;
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: DWORD = 0x10;
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: DWORD = 0x20;
pub const RPC_C_PROTECT_LEVEL_DEFAULT: DWORD = RPC_C_AUTHN_LEVEL_DEFAULT;
pub const RPC_C_PROTECT_LEVEL_NONE: DWORD = RPC_C_AUTHN_LEVEL_NONE;
pub const RPC_C_PROTECT_LEVEL_CONNECT: DWORD = RPC_C_AUTHN_LEVEL_CONNECT;
pub const RPC_C_PROTECT_LEVEL_CALL: DWORD = RPC_C_AUTHN_LEVEL_CALL;
pub const RPC_C_PROTECT_LEVEL_PKT: DWORD = RPC_C_AUTHN_LEVEL_PKT;
pub const RPC_C_PROTECT_LEVEL_PKT_INTEGRITY: DWORD = RPC_C_AUTHN_LEVEL_PKT_INTEGRITY;
pub const RPC_C_PROTECT_LEVEL_PKT_PRIVACY: DWORD = RPC_C_AUTHN_LEVEL_PKT_PRIVACY;
pub const RPC_C_AUTHN_NONE: DWORD = 0;
pub const RPC_C_AUTHN_DCE_PRIVATE: DWORD = 1;
pub const RPC_C_AUTHN_DCE_PUBLIC: DWORD = 2;
pub const RPC_C_AUTHN_DEC_PUBLIC: DWORD = 4;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: DWORD = 9;
pub const RPC_C_AUTHN_WINNT: DWORD = 10;
pub const RPC_C_AUTHN_GSS_SCHANNEL: DWORD = 14;
pub const RPC_C_AUTHN_GSS_KERBEROS: DWORD = 16;
pub const RPC_C_AUTHN_DPA: DWORD = 17;
pub const RPC_C_AUTHN_MSN: DWORD = 18;
pub const RPC_C_AUTHN_DIGEST: DWORD = 21;
pub const RPC_C_AUTHN_KERNEL: DWORD = 20;
pub const RPC_C_AUTHN_NEGO_EXTENDER: DWORD = 30;
pub const RPC_C_AUTHN_PKU2U: DWORD = 31;
pub const RPC_C_AUTHN_LIVE_SSP: DWORD = 32;
pub const RPC_C_AUTHN_LIVEXP_SSP: DWORD = 35;
pub const RPC_C_AUTHN_MSONLINE: DWORD = 82;
pub const RPC_C_AUTHN_MQ: DWORD = 100;
pub const RPC_C_AUTHN_DEFAULT: DWORD = 0xFFFFFFFF;
pub const RPC_C_NO_CREDENTIALS: DWORD = 0xFFFFFFFF;
pub const RPC_C_SECURITY_QOS_VERSION: DWORD = 1;
pub const RPC_C_SECURITY_QOS_VERSION_1: DWORD = 1;
STRUCT!{struct RPC_SECURITY_QOS {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
}}
pub type PRPC_SECURITY_QOS = *mut RPC_SECURITY_QOS;
STRUCT!{struct SEC_WINNT_AUTH_IDENTITY_W {
    User: *mut c_ushort,
    UserLength: c_ulong,
    Domain: *mut c_ushort,
    DomainLength: c_ulong,
    Password: *mut c_ushort,
    PasswordLength: c_ulong,
    Flags: c_ulong,
}}
pub type PSEC_WINNT_AUTH_IDENTITY_W = *mut SEC_WINNT_AUTH_IDENTITY_W;
STRUCT!{struct SEC_WINNT_AUTH_IDENTITY_A {
    User: *mut c_uchar,
    UserLength: c_ulong,
    Domain: *mut c_uchar,
    DomainLength: c_ulong,
    Password: *mut c_uchar,
    PasswordLength: c_ulong,
    Flags: c_ulong,
}}
pub type PSEC_WINNT_AUTH_IDENTITY_A = *mut SEC_WINNT_AUTH_IDENTITY_A;
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: c_ulong = 1;
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: c_ulong = 1;
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: c_ulong = 2;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: c_ulong = 0x00000001;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: c_ulong = 0x00000002;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: c_ulong = 0x00000004;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: c_ulong = 0x00000008;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: c_ulong = 0x00000010;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: c_ulong = 0x00010000;
pub const RPC_C_HTTP_FLAG_USE_SSL: c_ulong = 1;
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: c_ulong = 2;
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: c_ulong = 8;
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: c_ulong = 16;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_ushort,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_uchar,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_ushort,
    ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    NumberOfProxyAuthnSchemes: c_ulong,
    ProxyAuthnSchemes: *mut c_ulong,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_uchar,
    ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    NumberOfProxyAuthnSchemes: c_ulong,
    ProxyAuthnSchemes: *mut c_ulong,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_ushort,
    ProxyCredentials: *mut RPC_AUTH_IDENTITY_HANDLE,
    NumberOfProxyAuthnSchemes: c_ulong,
    ProxyAuthnSchemes: *mut c_ulong,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W;
STRUCT!{struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    Flags: c_ulong,
    AuthenticationTarget: c_ulong,
    NumberOfAuthnSchemes: c_ulong,
    AuthnSchemes: *mut c_ulong,
    ServerCertificateSubject: *mut c_uchar,
    ProxyCredentials: *mut RPC_AUTH_IDENTITY_HANDLE,
    NumberOfProxyAuthnSchemes: c_ulong,
    ProxyAuthnSchemes: *mut c_ulong,
}}
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A;
STRUCT!{struct RPC_SECURITY_QOS_V2_W_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}}
STRUCT!{struct RPC_SECURITY_QOS_V2_W {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V2_W_union,
}}
pub type PRPC_SECURITY_QOS_V2_W = *mut RPC_SECURITY_QOS_V2_W;
STRUCT!{struct RPC_SECURITY_QOS_V2_A_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}}
STRUCT!{struct RPC_SECURITY_QOS_V2_A {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V2_A_union,
}}
pub type PRPC_SECURITY_QOS_V2_A = *mut RPC_SECURITY_QOS_V2_A;
STRUCT!{struct RPC_SECURITY_QOS_V3_W_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}}
STRUCT!{struct RPC_SECURITY_QOS_V3_W {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V3_W_union,
    Sid: *mut c_void,
}}
pub type PRPC_SECURITY_QOS_V3_W = *mut RPC_SECURITY_QOS_V3_W;
STRUCT!{struct RPC_SECURITY_QOS_V3_A_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}}
STRUCT!{struct RPC_SECURITY_QOS_V3_A {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V3_A_union,
    Sid: *mut c_void,
}}
pub type PRPC_SECURITY_QOS_V3_A = *mut RPC_SECURITY_QOS_V3_A;
STRUCT!{struct RPC_SECURITY_QOS_V4_W_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}}
STRUCT!{struct RPC_SECURITY_QOS_V4_W {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V4_W_union,
    Sid: *mut c_void,
    EffectiveOnly: c_uint,
}}
pub type PRPC_SECURITY_QOS_V4_W = *mut RPC_SECURITY_QOS_V4_W;
STRUCT!{struct RPC_SECURITY_QOS_V4_A_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}}
STRUCT!{struct RPC_SECURITY_QOS_V4_A {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V4_A_union,
    Sid: *mut c_void,
    EffectiveOnly: c_uint,
}}
pub type PRPC_SECURITY_QOS_V4_A = *mut RPC_SECURITY_QOS_V4_A;
STRUCT!{struct RPC_SECURITY_QOS_V5_W_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}}
STRUCT!{struct RPC_SECURITY_QOS_V5_W {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V5_W_union,
    Sid: *mut c_void,
    EffectiveOnly: c_uint,
    ServerSecurityDescriptor: *mut c_void,
}}
pub type PRPC_SECURITY_QOS_V5_W = *mut RPC_SECURITY_QOS_V5_W;
STRUCT!{struct RPC_SECURITY_QOS_V5_A_union {
    HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}}
STRUCT!{struct RPC_SECURITY_QOS_V5_A {
    Version: c_ulong,
    Capabilities: c_ulong,
    IdentityTracking: c_ulong,
    ImpersonationType: c_ulong,
    AdditionalSecurityInfoType: c_ulong,
    u: RPC_SECURITY_QOS_V5_A_union,
    Sid: *mut c_void,
    EffectiveOnly: c_uint,
    ServerSecurityDescriptor: *mut c_void,
}}
pub type PRPC_SECURITY_QOS_V5_A = *mut RPC_SECURITY_QOS_V5_A;
pub const RPC_PROTSEQ_TCP: c_ulong = 0x1;
pub const RPC_PROTSEQ_NMP: c_ulong = 0x2;
pub const RPC_PROTSEQ_LRPC: c_ulong = 0x3;
pub const RPC_PROTSEQ_HTTP: c_ulong = 0x4;
pub const RPC_BHT_OBJECT_UUID_VALID: c_ulong = 0x1;
pub const RPC_BHO_NONCAUSAL: c_ulong = 0x1;
pub const RPC_BHO_DONTLINGER: c_ulong = 0x2;
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: c_ulong = 0x4;
STRUCT!{struct RPC_BINDING_HANDLE_TEMPLATE_V1_W_union {
    Reserved: *mut c_ushort,
}}
STRUCT!{struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    Version: c_ulong,
    Flags: c_ulong,
    ProtocolSequence: c_ulong,
    NetworkAddress: *mut c_ushort,
    StringEndpoint: *mut c_ushort,
    u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_union,
    ObjectUuid: UUID,
}}
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_W = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_W;
STRUCT!{struct RPC_BINDING_HANDLE_TEMPLATE_V1_A_union {
    Reserved: *mut c_uchar,
}}
STRUCT!{struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    Version: c_ulong,
    Flags: c_ulong,
    ProtocolSequence: c_ulong,
    NetworkAddress: *mut c_uchar,
    StringEndpoint: *mut c_uchar,
    u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_union,
    ObjectUuid: UUID,
}}
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_A = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_A;
STRUCT!{struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    Version: c_ulong,
    ServerPrincName: *mut c_ushort,
    AuthnLevel: c_ulong,
    AuthnSvc: c_ulong,
    AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    SecurityQos: *mut RPC_SECURITY_QOS,
}}
pub type PRPC_BINDING_HANDLE_SECURITY_V1_W = *mut RPC_BINDING_HANDLE_SECURITY_V1_W;
STRUCT!{struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    Version: c_ulong,
    ServerPrincName: *mut c_uchar,
    AuthnLevel: c_ulong,
    AuthnSvc: c_ulong,
    AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    SecurityQos: *mut RPC_SECURITY_QOS,
}}
pub type PRPC_BINDING_HANDLE_SECURITY_V1_A = *mut RPC_BINDING_HANDLE_SECURITY_V1_A;
STRUCT!{struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    Version: c_ulong,
    Flags: c_ulong,
    ComTimeout: c_ulong,
    CallTimeout: c_ulong,
}}
pub type PRPC_BINDING_HANDLE_OPTIONS_V1 = *mut RPC_BINDING_HANDLE_OPTIONS_V1;
ENUM!{enum RPC_HTTP_REDIRECTOR_STAGE {
    RPCHTTP_RS_REDIRECT = 1,
    RPCHTTP_RS_ACCESS_1,
    RPCHTTP_RS_SESSION,
    RPCHTTP_RS_ACCESS_2,
    RPCHTTP_RS_INTERFACE,
}}
FN!{stdcall RPC_NEW_HTTP_PROXY_CHANNEL(
    RedirectorStage: RPC_HTTP_REDIRECTOR_STAGE,
    ServerName: RPC_WSTR,
    ServerPort: RPC_WSTR,
    RemoteUser: RPC_WSTR,
    AuthType: RPC_WSTR,
    ResourceUuid: *mut c_void,
    SessionId: *mut c_void,
    Interface: *mut c_void,
    Reserved: *mut c_void,
    Flags: c_ulong,
    NewServerName: *mut RPC_WSTR,
    NewServerPort: *mut RPC_WSTR,
) -> RPC_STATUS}
FN!{stdcall RPC_HTTP_PROXY_FREE_STRING(
    String: RPC_WSTR,
) -> ()}
pub const RPC_C_AUTHZ_NONE: DWORD = 0;
pub const RPC_C_AUTHZ_NAME: DWORD = 1;
pub const RPC_C_AUTHZ_DCE: DWORD = 2;
pub const RPC_C_AUTHZ_DEFAULT: DWORD = 0xffffffff;
FN!{stdcall RPC_AUTH_KEY_RETRIEVAL_FN(
    Arg: *mut c_void,
    ServerPrincName: RPC_WSTR,
    KeyVer: c_ulong,
    Key: *mut *mut c_void,
    Status: *mut RPC_STATUS,
) -> ()}
STRUCT!{struct RPC_CLIENT_INFORMATION1 {
    UserName: *mut c_uchar,
    ComputerName: *mut c_uchar,
    Privilege: c_ushort,
    AuthFlags: c_ulong,
}}
pub type PRPC_CLIENT_INFORMATION1 = *mut RPC_CLIENT_INFORMATION1;
pub type RPC_EP_INQ_HANDLE = *mut I_RPC_HANDLE;
pub const RPC_C_EP_ALL_ELTS: c_ulong = 0;
pub const RPC_C_EP_MATCH_BY_IF: c_ulong = 1;
pub const RPC_C_EP_MATCH_BY_OBJ: c_ulong = 2;
pub const RPC_C_EP_MATCH_BY_BOTH: c_ulong = 3;
pub const RPC_C_VERS_ALL: c_ulong = 1;
pub const RPC_C_VERS_COMPATIBLE: c_ulong = 2;
pub const RPC_C_VERS_EXACT: c_ulong = 3;
pub const RPC_C_VERS_MAJOR_ONLY: c_ulong = 4;
pub const RPC_C_VERS_UPTO: c_ulong = 5;
FN!{stdcall RPC_MGMT_AUTHORIZATION_FN(
    ClientBinding: RPC_BINDING_HANDLE,
    RequestedMgmtOperation: c_ulong,
    Status: *mut RPC_STATUS,
) -> c_int}
pub const RPC_C_MGMT_INQ_IF_IDS: c_ulong = 0;
pub const RPC_C_MGMT_INQ_PRINC_NAME: c_ulong = 1;
pub const RPC_C_MGMT_INQ_STATS: c_ulong = 2;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: c_ulong = 3;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: c_ulong = 4;
pub const RPC_IF_AUTOLISTEN: c_uint = 0x0001;
pub const RPC_IF_OLE: c_uint = 0x0002;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: c_uint = 0x0004;
pub const RPC_IF_ALLOW_SECURE_ONLY: c_uint = 0x0008;
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: c_uint = 0x0010;
pub const RPC_IF_ALLOW_LOCAL_ONLY: c_uint = 0x0020;
pub const RPC_IF_SEC_NO_CACHE: c_uint = 0x0040;
pub const RPC_IF_SEC_CACHE_PER_PROC: c_uint = 0x0080;
pub const RPC_IF_ASYNC_CALLBACK: c_uint = 0x0100;
pub const RPC_FW_IF_FLAG_DCOM: c_uint = 0x0001;
pub type RPC_INTERFACE_GROUP = *mut c_void;
pub type PRPC_INTERFACE_GROUP = *mut *mut c_void;
STRUCT!{struct RPC_ENDPOINT_TEMPLATEW {
    Version: c_ulong,
    ProtSeq: RPC_WSTR,
    Endpoint: RPC_WSTR,
    SecurityDescriptor: *mut c_void,
    Backlog: c_ulong,
}}
pub type PRPC_ENDPOINT_TEMPLATEW = *mut RPC_ENDPOINT_TEMPLATEW;
STRUCT!{struct RPC_ENDPOINT_TEMPLATEA {
    Version: c_ulong,
    ProtSeq: RPC_CSTR,
    Endpoint: RPC_CSTR,
    SecurityDescriptor: *mut c_void,
    Backlog: c_ulong,
}}
pub type PRPC_ENDPOINT_TEMPLATEA = *mut RPC_ENDPOINT_TEMPLATEA;
STRUCT!{struct RPC_INTERFACE_TEMPLATEA {
    Version: c_ulong,
    IfSpec: RPC_IF_HANDLE,
    MgrTypeUuid: *mut UUID,
    MgrEpv: *mut RPC_MGR_EPV,
    Flags: c_uint,
    MaxCalls: c_uint,
    MaxRpcSize: c_uint,
    IfCallback: *mut RPC_IF_CALLBACK_FN,
    UuidVector: *mut UUID_VECTOR,
    Annotation: RPC_CSTR,
    SecurityDescriptor: *mut c_void,
}}
pub type PRPC_INTERFACE_TEMPLATEA = *mut RPC_INTERFACE_TEMPLATEA;
STRUCT!{struct RPC_INTERFACE_TEMPLATEW {
    Version: c_ulong,
    IfSpec: RPC_IF_HANDLE,
    MgrTypeUuid: *mut UUID,
    MgrEpv: *mut RPC_MGR_EPV,
    Flags: c_uint,
    MaxCalls: c_uint,
    MaxRpcSize: c_uint,
    IfCallback: *mut RPC_IF_CALLBACK_FN,
    UuidVector: *mut UUID_VECTOR,
    Annotation: RPC_WSTR,
    SecurityDescriptor: *mut c_void,
}}
pub type PRPC_INTERFACE_TEMPLATEW = *mut RPC_INTERFACE_TEMPLATEW;
FN!{stdcall RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN(
    IfGroup: RPC_INTERFACE_GROUP,
    IdleCallbackContext: *mut c_void,
    IsGroupIdle: c_ulong,
) -> ()}
