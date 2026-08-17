// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, LPVOID, PBYTE, PDWORD, ULONG};
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::wincrypt::ALG_ID;
use um::winnt::{LPCWSTR, LPWSTR, SID};
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: DWORD = 0x00000001;
STRUCT!{struct EFS_CERTIFICATE_BLOB {
    dwCertEncodingType: DWORD,
    cbData: DWORD,
    pbData: PBYTE,
}}
pub type PEFS_CERTIFICATE_BLOB = *mut EFS_CERTIFICATE_BLOB;
STRUCT!{struct EFS_HASH_BLOB {
    cbData: DWORD,
    pbData: PBYTE,
}}
pub type PEFS_HASH_BLOB = *mut EFS_HASH_BLOB;
STRUCT!{struct EFS_RPC_BLOB {
    cbData: DWORD,
    pbData: PBYTE,
}}
pub type PEFS_RPC_BLOB = *mut EFS_RPC_BLOB;
STRUCT!{struct EFS_PIN_BLOB {
    cbPadding: DWORD,
    cbData: DWORD,
    pbData: PBYTE,
}}
pub type PEFS_PIN_BLOB = *mut EFS_PIN_BLOB;
STRUCT!{struct EFS_KEY_INFO {
    dwVersion: DWORD,
    Entropy: ULONG,
    Algorithm: ALG_ID,
    KeyLength: ULONG,
}}
pub type PEFS_KEY_INFO = *mut EFS_KEY_INFO;
STRUCT!{struct EFS_COMPATIBILITY_INFO {
    EfsVersion: DWORD,
}}
pub type PEFS_COMPATIBILITY_INFO = *mut EFS_COMPATIBILITY_INFO;
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: DWORD = 5;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: DWORD = 6;
#[inline]
pub fn EFS_IS_DESCRIPTOR_VERSION(v: DWORD) -> bool {
    v == EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR
        || v == EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR
}
pub const EFS_SUBVER_UNKNOWN: DWORD = 0;
pub const EFS_EFS_SUBVER_EFS_CERT: DWORD = 1;
pub const EFS_PFILE_SUBVER_RMS: DWORD = 2;
pub const EFS_PFILE_SUBVER_APPX: DWORD = 3;
STRUCT!{struct EFS_VERSION_INFO {
    EfsVersion: DWORD,
    SubVersion: DWORD,
}}
pub type PEFS_VERSION_INFO = *mut EFS_VERSION_INFO;
#[inline]
pub fn EFS_IS_APPX_VERSION(v: DWORD, subV: DWORD) -> bool {
    v == EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR && subV == EFS_PFILE_SUBVER_APPX
}
STRUCT!{struct EFS_DECRYPTION_STATUS_INFO {
    dwDecryptionError: DWORD,
    dwHashOffset: DWORD,
    cbHash: DWORD,
}}
pub type PEFS_DECRYPTION_STATUS_INFO = *mut EFS_DECRYPTION_STATUS_INFO;
STRUCT!{struct EFS_ENCRYPTION_STATUS_INFO {
    bHasCurrentKey: BOOL,
    dwEncryptionError: DWORD,
}}
pub type PEFS_ENCRYPTION_STATUS_INFO = *mut EFS_ENCRYPTION_STATUS_INFO;
STRUCT!{struct ENCRYPTION_CERTIFICATE {
    cbTotalLength: DWORD,
    pUserSid: *mut SID,
    pCertBlob: PEFS_CERTIFICATE_BLOB,
}}
pub type PENCRYPTION_CERTIFICATE = *mut ENCRYPTION_CERTIFICATE;
pub const MAX_SID_SIZE: SIZE_T = 256;
STRUCT!{struct ENCRYPTION_CERTIFICATE_HASH {
    cbTotalLength: DWORD,
    pUserSid: *mut SID,
    pHash: PEFS_HASH_BLOB,
    lpDisplayInformation: LPWSTR,
}}
pub type PENCRYPTION_CERTIFICATE_HASH = *mut ENCRYPTION_CERTIFICATE_HASH;
STRUCT!{struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    nCert_Hash: DWORD,
    pUsers: *mut PENCRYPTION_CERTIFICATE_HASH,
}}
pub type PENCRYPTION_CERTIFICATE_HASH_LIST = *mut ENCRYPTION_CERTIFICATE_HASH_LIST;
STRUCT!{struct ENCRYPTION_CERTIFICATE_LIST {
    nUsers: DWORD,
    pUsers: *mut PENCRYPTION_CERTIFICATE,
}}
pub type PENCRYPTION_CERTIFICATE_LIST = *mut ENCRYPTION_CERTIFICATE_LIST;
pub const EFS_METADATA_ADD_USER: DWORD = 0x00000001;
pub const EFS_METADATA_REMOVE_USER: DWORD = 0x00000002;
pub const EFS_METADATA_REPLACE_USER: DWORD = 0x00000004;
pub const EFS_METADATA_GENERAL_OP: DWORD = 0x00000008;
STRUCT!{struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    dwEfsAccessType: DWORD,
    pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    pEfsStreamSignature: PEFS_RPC_BLOB,
}}
pub type PENCRYPTED_FILE_METADATA_SIGNATURE = *mut ENCRYPTED_FILE_METADATA_SIGNATURE;
STRUCT!{struct ENCRYPTION_PROTECTOR {
    cbTotalLength: DWORD,
    pUserSid: *mut SID,
    lpProtectorDescriptor: LPWSTR,
}}
pub type PENCRYPTION_PROTECTOR = *mut ENCRYPTION_PROTECTOR;
STRUCT!{struct ENCRYPTION_PROTECTOR_LIST {
    nProtectors: DWORD,
    pProtectors: *mut PENCRYPTION_PROTECTOR,
}}
pub type PENCRYPTION_PROTECTOR_LIST = *mut ENCRYPTION_PROTECTOR_LIST;
extern "system" {
    pub fn QueryUsersOnEncryptedFile(
        lpFileName: LPCWSTR,
        pUsers: *mut PENCRYPTION_CERTIFICATE_HASH_LIST,
    ) -> DWORD;
    pub fn QueryRecoveryAgentsOnEncryptedFile(
        lpFileName: LPCWSTR,
        pRecoveryAgents: *mut PENCRYPTION_CERTIFICATE_HASH_LIST,
    ) -> DWORD;
    pub fn RemoveUsersFromEncryptedFile(
        lpFileName: LPCWSTR,
        pHashes: PENCRYPTION_CERTIFICATE_HASH_LIST,
    ) -> DWORD;
    pub fn AddUsersToEncryptedFile(
        lpFileName: LPCWSTR,
        pEncryptionCertificate: PENCRYPTION_CERTIFICATE_LIST,
    ) -> DWORD;
    pub fn SetUserFileEncryptionKey(
        pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    ) -> DWORD;
    pub fn SetUserFileEncryptionKeyEx(
        pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
        dwCapabilities: DWORD,
        dwFlags: DWORD,
        pvReserved: LPVOID,
    ) -> DWORD;
    pub fn FreeEncryptionCertificateHashList(
        pUsers: PENCRYPTION_CERTIFICATE_HASH_LIST,
    );
    pub fn EncryptionDisable(
        DirPath: LPCWSTR,
        Disable: BOOL,
    ) -> BOOL;
    pub fn DuplicateEncryptionInfoFile(
        SrcFileName: LPCWSTR,
        DstFileName: LPCWSTR,
        dwCreationDistribution: DWORD,
        dwAttributes: DWORD,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
    ) -> DWORD;
    pub fn GetEncryptedFileMetadata(
        lpFileName: LPCWSTR,
        pcbMetadata: PDWORD,
        ppbMetadata: *mut PBYTE,
    ) -> DWORD;
    pub fn SetEncryptedFileMetadata(
        lpFileName: LPCWSTR,
        pbOldMetadata: PBYTE,
        pbNewMetadata: PBYTE,
        pOwnerHash: PENCRYPTION_CERTIFICATE_HASH,
        dwOperation: DWORD,
        pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    ) -> DWORD;
    pub fn FreeEncryptedFileMetadata(
        pbMetadata: PBYTE,
    );
}
