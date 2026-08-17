// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public Definitions for MIN SCHANNEL Security Provider
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD};
use um::wincrypt::ALG_ID;
use um::winnt::LPWSTR;
pub const SECPKG_ATTR_ISSUER_LIST: DWORD = 0x50;
pub const SECPKG_ATTR_REMOTE_CRED: DWORD = 0x51;
pub const SECPKG_ATTR_LOCAL_CRED: DWORD = 0x52;
pub const SECPKG_ATTR_REMOTE_CERT_CONTEXT: DWORD = 0x53;
pub const SECPKG_ATTR_LOCAL_CERT_CONTEXT: DWORD = 0x54;
pub const SECPKG_ATTR_ROOT_STORE: DWORD = 0x55;
pub const SECPKG_ATTR_SUPPORTED_ALGS: DWORD = 0x56;
pub const SECPKG_ATTR_CIPHER_STRENGTHS: DWORD = 0x57;
pub const SECPKG_ATTR_SUPPORTED_PROTOCOLS: DWORD = 0x58;
pub const SECPKG_ATTR_ISSUER_LIST_EX: DWORD = 0x59;
pub const SECPKG_ATTR_CONNECTION_INFO: DWORD = 0x5a;
pub const SECPKG_ATTR_EAP_KEY_BLOCK: DWORD = 0x5b;
pub const SECPKG_ATTR_MAPPED_CRED_ATTR: DWORD = 0x5c;
pub const SECPKG_ATTR_SESSION_INFO: DWORD = 0x5d;
pub const SECPKG_ATTR_APP_DATA: DWORD = 0x5e;
pub const SECPKG_ATTR_REMOTE_CERTIFICATES: DWORD = 0x5F;
pub const SECPKG_ATTR_CLIENT_CERT_POLICY: DWORD = 0x60;
pub const SECPKG_ATTR_CC_POLICY_RESULT: DWORD = 0x61;
pub const SECPKG_ATTR_USE_NCRYPT: DWORD = 0x62;
pub const SECPKG_ATTR_LOCAL_CERT_INFO: DWORD = 0x63;
pub const SECPKG_ATTR_CIPHER_INFO: DWORD = 0x64;
pub const SECPKG_ATTR_EAP_PRF_INFO: DWORD = 0x65;
pub const SECPKG_ATTR_SUPPORTED_SIGNATURES: DWORD = 0x66;
pub const SECPKG_ATTR_REMOTE_CERT_CHAIN: DWORD = 0x67;
pub const SECPKG_ATTR_UI_INFO: DWORD = 0x68;
pub const SECPKG_ATTR_EARLY_START: DWORD = 0x69;
STRUCT!{struct SecPkgCred_SupportedAlgs {
    cSupportedAlgs: DWORD,
    palgSupportedAlgs: *mut ALG_ID,
}}
STRUCT!{struct SecPkgCred_CipherStrengths {
    dwMinimumCipherStrength: DWORD,
    dwMaximumCipherStrength: DWORD,
}}
STRUCT!{struct SecPkgCred_SupportedProtocols {
    grbitProtocol: DWORD,
}}
STRUCT!{struct SecPkgCred_ClientCertPolicy {
    dwFlags: DWORD,
    guidPolicyId: GUID,
    dwCertFlags: DWORD,
    dwUrlRetrievalTimeout: DWORD,
    fCheckRevocationFreshnessTime: BOOL,
    dwRevocationFreshnessTime: DWORD,
    fOmitUsageCheck: BOOL,
    pwszSslCtlStoreName: LPWSTR,
    pwszSslCtlIdentifier: LPWSTR,
}}
