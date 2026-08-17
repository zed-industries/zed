// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public Definitions for SCHANNEL Security Provider
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, DWORD, PBYTE, WORD};
use shared::windef::HWND;
use um::wincrypt::{ALG_ID, HCERTSTORE, HCRYPTPROV, PCCERT_CONTEXT, PCERT_NAME_BLOB};
use um::winnt::{HRESULT, LPWSTR, PVOID, WCHAR};
pub const UNISP_NAME: &'static str = "Microsoft Unified Security Protocol Provider";
pub const SSL2SP_NAME: &'static str = "Microsoft SSL 2.0";
pub const SSL3SP_NAME: &'static str = "Microsoft SSL 3.0";
pub const TLS1SP_NAME: &'static str = "Microsoft TLS 1.0";
pub const PCT1SP_NAME: &'static str = "Microsoft PCT 1.0";
pub const SCHANNEL_NAME: &'static str = "Schannel";
ENUM!{enum eTlsSignatureAlgorithm {
    TlsSignatureAlgorithm_Anonymous = 0,
    TlsSignatureAlgorithm_Rsa = 1,
    TlsSignatureAlgorithm_Dsa = 2,
    TlsSignatureAlgorithm_Ecdsa = 3,
}}
ENUM!{enum eTlsHashAlgorithm {
    TlsHashAlgorithm_None = 0,
    TlsHashAlgorithm_Md5 = 1,
    TlsHashAlgorithm_Sha1 = 2,
    TlsHashAlgorithm_Sha224 = 3,
    TlsHashAlgorithm_Sha256 = 4,
    TlsHashAlgorithm_Sha384 = 5,
    TlsHashAlgorithm_Sha512 = 6,
}}
pub const UNISP_RPC_ID: DWORD = 14;
STRUCT!{struct SecPkgContext_RemoteCredentialInfo {
    cbCertificateChain: DWORD,
    pbCertificateChain: PBYTE,
    cCertificates: DWORD,
    fFlags: DWORD,
    dwBits: DWORD,
}}
pub type PSecPkgContext_RemoteCredentialInfo = *mut SecPkgContext_RemoteCredentialInfo;
pub type SecPkgContext_RemoteCredenitalInfo = SecPkgContext_RemoteCredentialInfo;
pub type PSecPkgContext_RemoteCredenitalInfo = *mut SecPkgContext_RemoteCredentialInfo;
pub const RCRED_STATUS_NOCRED: DWORD = 0x00000000;
pub const RCRED_CRED_EXISTS: DWORD = 0x00000001;
pub const RCRED_STATUS_UNKNOWN_ISSUER: DWORD = 0x00000002;
STRUCT!{struct SecPkgContext_LocalCredentialInfo {
    cbCertificateChain: DWORD,
    pbCertificateChain: PBYTE,
    cCertificates: DWORD,
    fFlags: DWORD,
    dwBits: DWORD,
}}
pub type PSecPkgContext_LocalCredentialInfo = *mut SecPkgContext_LocalCredentialInfo;
pub type SecPkgContext_LocalCredenitalInfo = SecPkgContext_LocalCredentialInfo;
pub type PSecPkgContext_LocalCredenitalInfo = *mut SecPkgContext_LocalCredentialInfo;
pub const LCRED_STATUS_NOCRED: DWORD = 0x00000000;
pub const LCRED_CRED_EXISTS: DWORD = 0x00000001;
pub const LCRED_STATUS_UNKNOWN_ISSUER: DWORD = 0x00000002;
STRUCT!{struct SecPkgContext_ClientCertPolicyResult {
    dwPolicyResult: HRESULT,
    guidPolicyId: GUID,
}}
pub type PSecPkgContext_ClientCertPolicyResult = *mut SecPkgContext_ClientCertPolicyResult;
STRUCT!{struct SecPkgContext_IssuerListInfoEx {
    aIssuers: PCERT_NAME_BLOB,
    cIssuers: DWORD,
}}
pub type PSecPkgContext_IssuerListInfoEx = *mut SecPkgContext_IssuerListInfoEx;
STRUCT!{struct SecPkgContext_ConnectionInfo {
    dwProtocol: DWORD,
    aiCipher: ALG_ID,
    dwCipherStrength: DWORD,
    aiHash: ALG_ID,
    dwHashStrength: DWORD,
    aiExch: ALG_ID,
    dwExchStrength: DWORD,
}}
pub type PSecPkgContext_ConnectionInfo = *mut SecPkgContext_ConnectionInfo;
pub const SZ_ALG_MAX_SIZE: usize = 64;
pub const SECPKGCONTEXT_CIPHERINFO_V1: DWORD = 1;
STRUCT!{struct SecPkgContext_CipherInfo {
    dwVersion: DWORD,
    dwProtocol: DWORD,
    dwCipherSuite: DWORD,
    dwBaseCipherSuite: DWORD,
    szCipherSuite: [WCHAR; SZ_ALG_MAX_SIZE],
    szCipher: [WCHAR; SZ_ALG_MAX_SIZE],
    dwCipherLen: DWORD,
    dwCipherBlockLen: DWORD,
    szHash: [WCHAR; SZ_ALG_MAX_SIZE],
    dwHashLen: DWORD,
    szExchange: [WCHAR; SZ_ALG_MAX_SIZE],
    dwMinExchangeLen: DWORD,
    dwMaxExchangeLen: DWORD,
    szCertificate: [WCHAR; SZ_ALG_MAX_SIZE],
    dwKeyType: DWORD,
}}
pub type PSecPkgContext_CipherInfo = *mut SecPkgContext_CipherInfo;
STRUCT!{struct SecPkgContext_EapKeyBlock {
    rgbKeys: [BYTE; 128],
    rgbIVs: [BYTE; 64],
}}
pub type PSecPkgContext_EapKeyBlock = *mut SecPkgContext_EapKeyBlock;
STRUCT!{struct SecPkgContext_MappedCredAttr {
    dwAttribute: DWORD,
    pvBuffer: PVOID,
}}
pub type PSecPkgContext_MappedCredAttr = *mut SecPkgContext_MappedCredAttr;
pub const SSL_SESSION_RECONNECT: DWORD = 1;
STRUCT!{struct SecPkgContext_SessionInfo {
    dwFlags: DWORD,
    cbSessionId: DWORD,
    rgbSessionId: [BYTE; 32],
}}
pub type PSecPkgContext_SessionInfo = *mut SecPkgContext_SessionInfo;
STRUCT!{struct SecPkgContext_SessionAppData {
    dwFlags: DWORD,
    cbAppData: DWORD,
    pbAppData: PBYTE,
}}
pub type PSecPkgContext_SessionAppData = *mut SecPkgContext_SessionAppData;
STRUCT!{struct SecPkgContext_EapPrfInfo {
    dwVersion: DWORD,
    cbPrfData: DWORD,
    pbPrfData: PBYTE,
}}
pub type PSecPkgContext_EapPrfInfo = *mut SecPkgContext_EapPrfInfo;
STRUCT!{struct SecPkgContext_SupportedSignatures {
    cSignatureAndHashAlgorithms: WORD,
    pSignatureAndHashAlgorithms: *mut WORD,
}}
pub type PSecPkgContext_SupportedSignatures = *mut SecPkgContext_SupportedSignatures;
STRUCT!{struct SecPkgContext_Certificates {
    cCertificates: DWORD,
    cbCertificateChain: DWORD,
    pbCertificateChain: PBYTE,
}}
pub type PSecPkgContext_Certificates = *mut SecPkgContext_Certificates;
STRUCT!{struct SecPkgContext_CertInfo {
    dwVersion: DWORD,
    cbSubjectName: DWORD,
    pwszSubjectName: LPWSTR,
    cbIssuerName: DWORD,
    pwszIssuerName: LPWSTR,
    dwKeySize: DWORD,
}}
pub type PSecPkgContext_CertInfo = *mut SecPkgContext_CertInfo;
pub const KERN_CONTEXT_CERT_INFO_V1: DWORD = 0x00000000;
STRUCT!{struct SecPkgContext_UiInfo {
    hParentWindow: HWND,
}}
pub type PSecPkgContext_UiInfo = *mut SecPkgContext_UiInfo;
STRUCT!{struct SecPkgContext_EarlyStart {
    dwEarlyStartFlags: DWORD,
}}
pub type PSecPkgContext_EarlyStart = *mut SecPkgContext_EarlyStart;
pub const ENABLE_TLS_CLIENT_EARLY_START: DWORD = 0x00000001;
pub const SCH_CRED_V1: DWORD = 0x00000001;
pub const SCH_CRED_V2: DWORD = 0x00000002;
pub const SCH_CRED_VERSION: DWORD = 0x00000002;
pub const SCH_CRED_V3: DWORD = 0x00000003;
pub const SCHANNEL_CRED_VERSION: DWORD = 0x00000004;
pub const SCHANNEL_SECRET_TYPE_CAPI: DWORD = 0x00000001;
pub const SCHANNEL_SECRET_PRIVKEY: DWORD = 0x00000002;
pub const SCH_CRED_X509_CERTCHAIN: DWORD = 0x00000001;
pub const SCH_CRED_X509_CAPI: DWORD = 0x00000002;
pub const SCH_CRED_CERT_CONTEXT: DWORD = 0x00000003;
pub enum _HMAPPER {}
STRUCT!{struct SCHANNEL_CRED {
    dwVersion: DWORD,
    cCreds: DWORD,
    paCred: *mut PCCERT_CONTEXT,
    hRootStore: HCERTSTORE,
    cMappers: DWORD,
    aphMappers: *mut *mut _HMAPPER,
    cSupportedAlgs: DWORD,
    palgSupportedAlgs: *mut ALG_ID,
    grbitEnabledProtocols: DWORD,
    dwMinimumCipherStrength: DWORD,
    dwMaximumCipherStrength: DWORD,
    dwSessionLifespan: DWORD,
    dwFlags: DWORD,
    dwCredFormat: DWORD,
}}
pub type PSCHANNEL_CRED = *mut SCHANNEL_CRED;
pub const SCH_CRED_FORMAT_CERT_CONTEXT: DWORD = 0x00000000;
pub const SCH_CRED_FORMAT_CERT_HASH: DWORD = 0x00000001;
pub const SCH_CRED_FORMAT_CERT_HASH_STORE: DWORD = 0x00000002;
pub const SCH_CRED_MAX_STORE_NAME_SIZE: usize = 128;
pub const SCH_CRED_MAX_SUPPORTED_ALGS: DWORD = 256;
pub const SCH_CRED_MAX_SUPPORTED_CERTS: DWORD = 100;
STRUCT!{struct SCHANNEL_CERT_HASH {
    dwLength: DWORD,
    dwFlags: DWORD,
    hProv: HCRYPTPROV,
    ShaHash: [BYTE; 20],
}}
pub type PSCHANNEL_CERT_HASH = *mut SCHANNEL_CERT_HASH;
STRUCT!{struct SCHANNEL_CERT_HASH_STORE {
    dwLength: DWORD,
    dwFlags: DWORD,
    hProv: HCRYPTPROV,
    ShaHash: [BYTE; 20],
    pwszStoreName: [WCHAR; SCH_CRED_MAX_STORE_NAME_SIZE],
}}
pub type PSCHANNEL_CERT_HASH_STORE = *mut SCHANNEL_CERT_HASH_STORE;
pub const SCH_MACHINE_CERT_HASH: DWORD = 0x00000001;
pub const SCH_CRED_NO_SYSTEM_MAPPER: DWORD = 0x00000002;
pub const SCH_CRED_NO_SERVERNAME_CHECK: DWORD = 0x00000004;
pub const SCH_CRED_MANUAL_CRED_VALIDATION: DWORD = 0x00000008;
pub const SCH_CRED_NO_DEFAULT_CREDS: DWORD = 0x00000010;
pub const SCH_CRED_AUTO_CRED_VALIDATION: DWORD = 0x00000020;
pub const SCH_CRED_USE_DEFAULT_CREDS: DWORD = 0x00000040;
pub const SCH_CRED_DISABLE_RECONNECTS: DWORD = 0x00000080;
pub const SCH_CRED_REVOCATION_CHECK_END_CERT: DWORD = 0x00000100;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN: DWORD = 0x00000200;
pub const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: DWORD = 0x00000400;
pub const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: DWORD = 0x00000800;
pub const SCH_CRED_IGNORE_REVOCATION_OFFLINE: DWORD = 0x00001000;
pub const SCH_CRED_RESTRICTED_ROOTS: DWORD = 0x00002000;
pub const SCH_CRED_REVOCATION_CHECK_CACHE_ONLY: DWORD = 0x00004000;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL: DWORD = 0x00008000;
pub const SCH_CRED_MEMORY_STORE_CERT: DWORD = 0x00010000;
pub const SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE: DWORD = 0x00020000;
pub const SCH_SEND_ROOT_CERT: DWORD = 0x00040000;
pub const SCH_CRED_SNI_CREDENTIAL: DWORD = 0x00080000;
pub const SCH_CRED_SNI_ENABLE_OCSP: DWORD = 0x00100000;
pub const SCH_SEND_AUX_RECORD: DWORD = 0x00200000;
pub const SCH_USE_STRONG_CRYPTO: DWORD = 0x00400000;
pub const SCHANNEL_RENEGOTIATE: DWORD = 0;
pub const SCHANNEL_SHUTDOWN: DWORD = 1;
pub const SCHANNEL_ALERT: DWORD = 2;
pub const SCHANNEL_SESSION: DWORD = 3;
STRUCT!{struct SCHANNEL_ALERT_TOKEN {
    dwTokenType: DWORD,
    dwAlertType: DWORD,
    dwAlertNumber: DWORD,
}}
pub const TLS1_ALERT_WARNING: DWORD = 1;
pub const TLS1_ALERT_FATAL: DWORD = 2;
pub const TLS1_ALERT_CLOSE_NOTIFY: DWORD = 0;
pub const TLS1_ALERT_UNEXPECTED_MESSAGE: DWORD = 10;
pub const TLS1_ALERT_BAD_RECORD_MAC: DWORD = 20;
pub const TLS1_ALERT_DECRYPTION_FAILED: DWORD = 21;
pub const TLS1_ALERT_RECORD_OVERFLOW: DWORD = 22;
pub const TLS1_ALERT_DECOMPRESSION_FAIL: DWORD = 30;
pub const TLS1_ALERT_HANDSHAKE_FAILURE: DWORD = 40;
pub const TLS1_ALERT_BAD_CERTIFICATE: DWORD = 42;
pub const TLS1_ALERT_UNSUPPORTED_CERT: DWORD = 43;
pub const TLS1_ALERT_CERTIFICATE_REVOKED: DWORD = 44;
pub const TLS1_ALERT_CERTIFICATE_EXPIRED: DWORD = 45;
pub const TLS1_ALERT_CERTIFICATE_UNKNOWN: DWORD = 46;
pub const TLS1_ALERT_ILLEGAL_PARAMETER: DWORD = 47;
pub const TLS1_ALERT_UNKNOWN_CA: DWORD = 48;
pub const TLS1_ALERT_ACCESS_DENIED: DWORD = 49;
pub const TLS1_ALERT_DECODE_ERROR: DWORD = 50;
pub const TLS1_ALERT_DECRYPT_ERROR: DWORD = 51;
pub const TLS1_ALERT_EXPORT_RESTRICTION: DWORD = 60;
pub const TLS1_ALERT_PROTOCOL_VERSION: DWORD = 70;
pub const TLS1_ALERT_INSUFFIENT_SECURITY: DWORD = 71;
pub const TLS1_ALERT_INTERNAL_ERROR: DWORD = 80;
pub const TLS1_ALERT_USER_CANCELED: DWORD = 90;
pub const TLS1_ALERT_NO_RENEGOTIATION: DWORD = 100;
pub const TLS1_ALERT_UNSUPPORTED_EXT: DWORD = 110;
pub const TLS1_ALERT_NO_APP_PROTOCOL: DWORD = 120;
pub const SSL_SESSION_ENABLE_RECONNECTS: DWORD = 1;
pub const SSL_SESSION_DISABLE_RECONNECTS: DWORD = 2;
STRUCT!{struct SCHANNEL_SESSION_TOKEN {
    dwTokenType: DWORD,
    dwFlags: DWORD,
}}
STRUCT!{struct SCHANNEL_CLIENT_SIGNATURE {
    cbLength: DWORD,
    aiHash: ALG_ID,
    cbHash: DWORD,
    HashValue: [BYTE; 36],
    CertThumbprint: [BYTE; 20],
}}
pub type PSCHANNEL_CLIENT_SIGNATURE = *mut SCHANNEL_CLIENT_SIGNATURE;
pub const SP_PROT_PCT1_SERVER: DWORD = 0x00000001;
pub const SP_PROT_PCT1_CLIENT: DWORD = 0x00000002;
pub const SP_PROT_PCT1: DWORD = SP_PROT_PCT1_SERVER | SP_PROT_PCT1_CLIENT;
pub const SP_PROT_SSL2_SERVER: DWORD = 0x00000004;
pub const SP_PROT_SSL2_CLIENT: DWORD = 0x00000008;
pub const SP_PROT_SSL2: DWORD = SP_PROT_SSL2_SERVER | SP_PROT_SSL2_CLIENT;
pub const SP_PROT_SSL3_SERVER: DWORD = 0x00000010;
pub const SP_PROT_SSL3_CLIENT: DWORD = 0x00000020;
pub const SP_PROT_SSL3: DWORD = SP_PROT_SSL3_SERVER | SP_PROT_SSL3_CLIENT;
pub const SP_PROT_TLS1_SERVER: DWORD = 0x00000040;
pub const SP_PROT_TLS1_CLIENT: DWORD = 0x00000080;
pub const SP_PROT_TLS1: DWORD = SP_PROT_TLS1_SERVER | SP_PROT_TLS1_CLIENT;
pub const SP_PROT_SSL3TLS1_CLIENTS: DWORD = SP_PROT_TLS1_CLIENT | SP_PROT_SSL3_CLIENT;
pub const SP_PROT_SSL3TLS1_SERVERS: DWORD = SP_PROT_TLS1_SERVER | SP_PROT_SSL3_SERVER;
pub const SP_PROT_SSL3TLS1: DWORD = SP_PROT_SSL3 | SP_PROT_TLS1;
pub const SP_PROT_UNI_SERVER: DWORD = 0x40000000;
pub const SP_PROT_UNI_CLIENT: DWORD = 0x80000000;
pub const SP_PROT_UNI: DWORD = SP_PROT_UNI_SERVER | SP_PROT_UNI_CLIENT;
pub const SP_PROT_ALL: DWORD = 0xffffffff;
pub const SP_PROT_NONE: DWORD = 0;
pub const SP_PROT_CLIENTS: DWORD = SP_PROT_PCT1_CLIENT | SP_PROT_SSL2_CLIENT
    | SP_PROT_SSL3_CLIENT | SP_PROT_UNI_CLIENT | SP_PROT_TLS1_CLIENT;
pub const SP_PROT_SERVERS: DWORD = SP_PROT_PCT1_SERVER | SP_PROT_SSL2_SERVER
    | SP_PROT_SSL3_SERVER | SP_PROT_UNI_SERVER | SP_PROT_TLS1_SERVER;
pub const SP_PROT_TLS1_0_SERVER: DWORD = SP_PROT_TLS1_SERVER;
pub const SP_PROT_TLS1_0_CLIENT: DWORD = SP_PROT_TLS1_CLIENT;
pub const SP_PROT_TLS1_0: DWORD = SP_PROT_TLS1_0_SERVER | SP_PROT_TLS1_0_CLIENT;
pub const SP_PROT_TLS1_1_SERVER: DWORD = 0x00000100;
pub const SP_PROT_TLS1_1_CLIENT: DWORD = 0x00000200;
pub const SP_PROT_TLS1_1: DWORD = SP_PROT_TLS1_1_SERVER | SP_PROT_TLS1_1_CLIENT;
pub const SP_PROT_TLS1_2_SERVER: DWORD = 0x00000400;
pub const SP_PROT_TLS1_2_CLIENT: DWORD = 0x00000800;
pub const SP_PROT_TLS1_2: DWORD = SP_PROT_TLS1_2_SERVER | SP_PROT_TLS1_2_CLIENT;
pub const SP_PROT_DTLS_SERVER: DWORD = 0x00010000;
pub const SP_PROT_DTLS_CLIENT: DWORD = 0x00020000;
pub const SP_PROT_DTLS: DWORD = SP_PROT_DTLS_SERVER | SP_PROT_DTLS_CLIENT;
pub const SP_PROT_DTLS1_0_SERVER: DWORD = SP_PROT_DTLS_SERVER;
pub const SP_PROT_DTLS1_0_CLIENT: DWORD = SP_PROT_DTLS_CLIENT;
pub const SP_PROT_DTLS1_0: DWORD = SP_PROT_DTLS1_0_SERVER | SP_PROT_DTLS1_0_CLIENT;
pub const SP_PROT_DTLS1_X_SERVER: DWORD = SP_PROT_DTLS1_0_SERVER;
pub const SP_PROT_DTLS1_X_CLIENT: DWORD = SP_PROT_DTLS1_0_CLIENT;
pub const SP_PROT_DTLS1_X: DWORD = SP_PROT_DTLS1_X_SERVER | SP_PROT_DTLS1_X_CLIENT;
pub const SP_PROT_TLS1_1PLUS_SERVER: DWORD = SP_PROT_TLS1_1_SERVER | SP_PROT_TLS1_2_SERVER;
pub const SP_PROT_TLS1_1PLUS_CLIENT: DWORD = SP_PROT_TLS1_1_CLIENT | SP_PROT_TLS1_2_CLIENT;
pub const SP_PROT_TLS1_1PLUS: DWORD = SP_PROT_TLS1_1PLUS_SERVER | SP_PROT_TLS1_1PLUS_CLIENT;
pub const SP_PROT_TLS1_X_SERVER: DWORD = SP_PROT_TLS1_0_SERVER | SP_PROT_TLS1_1_SERVER
    | SP_PROT_TLS1_2_SERVER;
pub const SP_PROT_TLS1_X_CLIENT: DWORD = SP_PROT_TLS1_0_CLIENT | SP_PROT_TLS1_1_CLIENT
    | SP_PROT_TLS1_2_CLIENT;
pub const SP_PROT_TLS1_X: DWORD = SP_PROT_TLS1_X_SERVER | SP_PROT_TLS1_X_CLIENT;
pub const SP_PROT_SSL3TLS1_X_CLIENTS: DWORD = SP_PROT_TLS1_X_CLIENT | SP_PROT_SSL3_CLIENT;
pub const SP_PROT_SSL3TLS1_X_SERVERS: DWORD = SP_PROT_TLS1_X_SERVER | SP_PROT_SSL3_SERVER;
pub const SP_PROT_SSL3TLS1_X: DWORD = SP_PROT_SSL3 | SP_PROT_TLS1_X;
pub const SP_PROT_X_CLIENTS: DWORD = SP_PROT_CLIENTS | SP_PROT_TLS1_X_CLIENT
    | SP_PROT_DTLS1_X_CLIENT;
pub const SP_PROT_X_SERVERS: DWORD = SP_PROT_SERVERS | SP_PROT_TLS1_X_SERVER
    | SP_PROT_DTLS1_X_SERVER;
pub const SSL_CRACK_CERTIFICATE_NAME: &'static str = "SslCrackCertificate";
pub const SSL_FREE_CERTIFICATE_NAME: &'static str = "SslFreeCertificate";
