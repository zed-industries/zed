// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Authentication API Prototypes and Definitions
use shared::minwindef::{
    BOOL, DWORD, FILETIME, LPBYTE, LPCVOID, LPDWORD, LPVOID, PBOOL, PBYTE, UCHAR, ULONG
};
use shared::windef::{HBITMAP, HWND};
use um::sspi::PCtxtHandle;
use um::winnt::{CHAR, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCSTR, PCWSTR, PSTR, PVOID, PWSTR, WCHAR};
// STATUS_*
pub const NERR_BASE: DWORD = 2100;
pub const NERR_PasswordExpired: DWORD = NERR_BASE + 142;
pub const CRED_MAX_STRING_LENGTH: DWORD = 256;
pub const CRED_MAX_USERNAME_LENGTH: DWORD = 256 + 1 + 256;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: DWORD = 32767;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: DWORD = 256 + 1 + 80;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: DWORD = 256;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: DWORD = 256;
pub const CRED_MAX_VALUE_SIZE: DWORD = 256;
pub const CRED_MAX_ATTRIBUTES: DWORD = 64;
STRUCT!{struct CREDENTIAL_ATTRIBUTEA {
    Keyword: LPSTR,
    Flags: DWORD,
    ValueSize: DWORD,
    Value: LPBYTE,
}}
pub type PCREDENTIAL_ATTRIBUTEA = *mut CREDENTIAL_ATTRIBUTEA;
STRUCT!{struct CREDENTIAL_ATTRIBUTEW {
    Keyword: LPWSTR,
    Flags: DWORD,
    ValueSize: DWORD,
    Value: LPBYTE,
}}
pub type PCREDENTIAL_ATTRIBUTEW = *mut CREDENTIAL_ATTRIBUTEW;
pub const CRED_LOGON_TYPES_MASK: DWORD = 0xF000;
pub const CRED_FLAGS_PASSWORD_FOR_CERT: DWORD = 0x0001;
pub const CRED_FLAGS_PROMPT_NOW: DWORD = 0x0002;
pub const CRED_FLAGS_USERNAME_TARGET: DWORD = 0x0004;
pub const CRED_FLAGS_OWF_CRED_BLOB: DWORD = 0x0008;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: DWORD = 0x0010;
pub const CRED_FLAGS_WILDCARD_MATCH: DWORD = 0x0020;
pub const CRED_FLAGS_VALID_FLAGS: DWORD = 0xF03F;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: DWORD = 0xF01F;
pub const CRED_TYPE_GENERIC: DWORD = 1;
pub const CRED_TYPE_DOMAIN_PASSWORD: DWORD = 2;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: DWORD = 3;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: DWORD = 4;
pub const CRED_TYPE_GENERIC_CERTIFICATE: DWORD = 5;
pub const CRED_TYPE_DOMAIN_EXTENDED: DWORD = 6;
pub const CRED_TYPE_MAXIMUM: DWORD = 7;
pub const CRED_TYPE_MAXIMUM_EX: DWORD = CRED_TYPE_MAXIMUM + 1000;
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: DWORD = 5 * 512;
pub const CRED_PERSIST_NONE: DWORD = 0;
pub const CRED_PERSIST_SESSION: DWORD = 1;
pub const CRED_PERSIST_LOCAL_MACHINE: DWORD = 2;
pub const CRED_PERSIST_ENTERPRISE: DWORD = 3;
STRUCT!{struct CREDENTIALA {
    Flags: DWORD,
    Type: DWORD,
    TargetName: LPSTR,
    Comment: LPSTR,
    LastWritten: FILETIME,
    CredentialBlobSize: DWORD,
    CredentialBlob: LPBYTE,
    Persist: DWORD,
    AttributeCount: DWORD,
    Attributes: PCREDENTIAL_ATTRIBUTEA,
    TargetAlias: LPSTR,
    UserName: LPSTR,
}}
pub type PCREDENTIALA = *mut CREDENTIALA;
STRUCT!{struct CREDENTIALW {
    Flags: DWORD,
    Type: DWORD,
    TargetName: LPWSTR,
    Comment: LPWSTR,
    LastWritten: FILETIME,
    CredentialBlobSize: DWORD,
    CredentialBlob: LPBYTE,
    Persist: DWORD,
    AttributeCount: DWORD,
    Attributes: PCREDENTIAL_ATTRIBUTEW,
    TargetAlias: LPWSTR,
    UserName: LPWSTR,
}}
pub type PCREDENTIALW = *mut CREDENTIALW;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: ULONG = 0x0001;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: ULONG = 0x0002;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: ULONG = 0x0004;
pub const CRED_TI_USERNAME_TARGET: ULONG = 0x0008;
pub const CRED_TI_CREATE_EXPLICIT_CRED: ULONG = 0x0010;
pub const CRED_TI_WORKGROUP_MEMBER: ULONG = 0x0020;
pub const CRED_TI_VALID_FLAGS: ULONG = 0xF07F;
STRUCT!{struct CREDENTIAL_TARGET_INFORMATIONA {
    TargetName: LPSTR,
    NetbiosServerName: LPSTR,
    DnsServerName: LPSTR,
    NetbiosDomainName: LPSTR,
    DnsDomainName: LPSTR,
    DnsTreeName: LPSTR,
    PackageName: LPSTR,
    Flags: ULONG,
    CredTypeCount: DWORD,
    CredTypes: LPDWORD,
}}
pub type PCREDENTIAL_TARGET_INFORMATIONA = *mut CREDENTIAL_TARGET_INFORMATIONA;
STRUCT!{struct CREDENTIAL_TARGET_INFORMATIONW {
    TargetName: LPWSTR,
    NetbiosServerName: LPWSTR,
    DnsServerName: LPWSTR,
    NetbiosDomainName: LPWSTR,
    DnsDomainName: LPWSTR,
    DnsTreeName: LPWSTR,
    PackageName: LPWSTR,
    Flags: ULONG,
    CredTypeCount: DWORD,
    CredTypes: LPDWORD,
}}
pub type PCREDENTIAL_TARGET_INFORMATIONW = *mut CREDENTIAL_TARGET_INFORMATIONW;
pub const CERT_HASH_LENGTH: usize = 20;
STRUCT!{struct CERT_CREDENTIAL_INFO {
    cbSize: ULONG,
    rgbHashOfCert: [UCHAR; CERT_HASH_LENGTH],
}}
pub type PCERT_CREDENTIAL_INFO = *mut CERT_CREDENTIAL_INFO;
STRUCT!{struct USERNAME_TARGET_CREDENTIAL_INFO {
    UserName: LPWSTR,
}}
pub type PUSERNAME_TARGET_CREDENTIAL_INFO = *mut USERNAME_TARGET_CREDENTIAL_INFO;
STRUCT!{struct BINARY_BLOB_CREDENTIAL_INFO {
    cbBlob: ULONG,
    pbBlob: LPBYTE,
}}
pub type PBINARY_BLOB_CREDENTIAL_INFO = *mut BINARY_BLOB_CREDENTIAL_INFO;
ENUM!{enum CRED_MARSHAL_TYPE {
    CertCredential = 1,
    UsernameTargetCredential,
    BinaryBlobCredential,
    UsernameForPackedCredentials,
}}
pub type PCRED_MARSHAL_TYPE = *mut CRED_MARSHAL_TYPE;
ENUM!{enum CRED_PROTECTION_TYPE {
    CredUnprotected,
    CredUserProtection,
    CredTrustedProtection,
}}
pub type PCRED_PROTECTION_TYPE = *mut CRED_PROTECTION_TYPE;
pub const CRED_PACK_PROTECTED_CREDENTIALS: DWORD = 0x1;
pub const CRED_PACK_WOW_BUFFER: DWORD = 0x2;
pub const CRED_PACK_GENERIC_CREDENTIALS: DWORD = 0x4;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: DWORD = 0x8;
STRUCT!{struct CREDUI_INFOA {
    cbSize: DWORD,
    hwndParent: HWND,
    pszMessageText: PCSTR,
    pszCaptionText: PCSTR,
    hbmBanner: HBITMAP,
}}
pub type PCREDUI_INFOA = *mut CREDUI_INFOA;
STRUCT!{struct CREDUI_INFOW {
    cbSize: DWORD,
    hwndParent: HWND,
    pszMessageText: PCWSTR,
    pszCaptionText: PCWSTR,
    hbmBanner: HBITMAP,
}}
pub type PCREDUI_INFOW = *mut CREDUI_INFOW;
pub const CREDUI_MAX_MESSAGE_LENGTH: DWORD = 1024;
pub const CREDUI_MAX_CAPTION_LENGTH: DWORD = 128;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: DWORD = CRED_MAX_GENERIC_TARGET_NAME_LENGTH;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: DWORD = CRED_MAX_DOMAIN_TARGET_NAME_LENGTH;
pub const CREDUI_MAX_USERNAME_LENGTH: DWORD = CRED_MAX_USERNAME_LENGTH;
pub const CREDUI_MAX_PASSWORD_LENGTH: DWORD = 512 / 2;
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: DWORD = 0x00001;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: DWORD = 0x00002;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: DWORD = 0x00004;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: DWORD = 0x00008;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: DWORD = 0x00010;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: DWORD = 0x00040;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: DWORD = 0x00080;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: DWORD = 0x00100;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: DWORD = 0x00200;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: DWORD = 0x00400;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: DWORD = 0x00800;
pub const CREDUI_FLAGS_PERSIST: DWORD = 0x01000;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: DWORD = 0x04000;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: DWORD = 0x20000;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: DWORD = 0x40000;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: DWORD = 0x80000;
pub const CREDUI_FLAGS_KEEP_USERNAME: DWORD = 0x100000;
pub const CREDUI_FLAGS_PROMPT_VALID: DWORD = CREDUI_FLAGS_INCORRECT_PASSWORD
    | CREDUI_FLAGS_DO_NOT_PERSIST | CREDUI_FLAGS_REQUEST_ADMINISTRATOR
    | CREDUI_FLAGS_EXCLUDE_CERTIFICATES | CREDUI_FLAGS_REQUIRE_CERTIFICATE
    | CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX | CREDUI_FLAGS_ALWAYS_SHOW_UI
    | CREDUI_FLAGS_REQUIRE_SMARTCARD | CREDUI_FLAGS_PASSWORD_ONLY_OK
    | CREDUI_FLAGS_VALIDATE_USERNAME | CREDUI_FLAGS_COMPLETE_USERNAME | CREDUI_FLAGS_PERSIST
    | CREDUI_FLAGS_SERVER_CREDENTIAL | CREDUI_FLAGS_EXPECT_CONFIRMATION
    | CREDUI_FLAGS_GENERIC_CREDENTIALS | CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS
    | CREDUI_FLAGS_KEEP_USERNAME;
pub const CREDUIWIN_GENERIC: DWORD = 0x00000001;
pub const CREDUIWIN_CHECKBOX: DWORD = 0x00000002;
pub const CREDUIWIN_AUTHPACKAGE_ONLY: DWORD = 0x00000010;
pub const CREDUIWIN_IN_CRED_ONLY: DWORD = 0x00000020;
pub const CREDUIWIN_ENUMERATE_ADMINS: DWORD = 0x00000100;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: DWORD = 0x00000200;
pub const CREDUIWIN_SECURE_PROMPT: DWORD = 0x00001000;
pub const CREDUIWIN_PREPROMPTING: DWORD = 0x00002000;
pub const CREDUIWIN_PACK_32_WOW: DWORD = 0x10000000;
pub const CREDUIWIN_VALID_FLAGS: DWORD = CREDUIWIN_GENERIC | CREDUIWIN_CHECKBOX
    | CREDUIWIN_AUTHPACKAGE_ONLY | CREDUIWIN_IN_CRED_ONLY | CREDUIWIN_ENUMERATE_ADMINS
    | CREDUIWIN_ENUMERATE_CURRENT_USER | CREDUIWIN_SECURE_PROMPT | CREDUIWIN_PREPROMPTING
    | CREDUIWIN_PACK_32_WOW;
pub const CRED_PRESERVE_CREDENTIAL_BLOB: DWORD = 0x1;
extern "system" {
    pub fn CredWriteW(
        Credential: PCREDENTIALW,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredWriteA(
        Credential: PCREDENTIALA,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredReadW(
        TargetName: LPCWSTR,
        Type: DWORD,
        Flags: DWORD,
        Credential: *mut PCREDENTIALW,
    ) -> BOOL;
    pub fn CredReadA(
        TargetName: LPCSTR,
        Type: DWORD,
        Flags: DWORD,
        Credential: *mut PCREDENTIALA,
    ) -> BOOL;
}
pub const CRED_ENUMERATE_ALL_CREDENTIALS: DWORD = 0x1;
extern "system" {
    pub fn CredEnumerateW(
        Filter: LPCWSTR,
        Flags: DWORD,
        Count: *mut DWORD,
        Credential: *mut *mut PCREDENTIALW,
    ) -> BOOL;
    pub fn CredEnumerateA(
        Filter: LPCSTR,
        Flags: DWORD,
        Count: *mut DWORD,
        Credential: *mut *mut PCREDENTIALA,
    ) -> BOOL;
    pub fn CredWriteDomainCredentialsW(
        TargetInfo: PCREDENTIAL_TARGET_INFORMATIONW,
        Credential: PCREDENTIALW,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredWriteDomainCredentialsA(
        TargetInfo: PCREDENTIAL_TARGET_INFORMATIONA,
        Credential: PCREDENTIALA,
        Flags: DWORD,
    ) -> BOOL;
}
pub const CRED_CACHE_TARGET_INFORMATION: DWORD = 0x1;
extern "system" {
    pub fn CredReadDomainCredentialsW(
        TargetInfo: PCREDENTIAL_TARGET_INFORMATIONW,
        Flags: DWORD,
        Count: *mut DWORD,
        Credential: *mut *mut PCREDENTIALW,
    ) -> BOOL;
    pub fn CredReadDomainCredentialsA(
        TargetInfo: PCREDENTIAL_TARGET_INFORMATIONA,
        Flags: DWORD,
        Count: *mut DWORD,
        Credential: *mut *mut PCREDENTIALA,
    ) -> BOOL;
    pub fn CredDeleteW(
        TargetName: LPCWSTR,
        Type: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredDeleteA(
        TargetName: LPCSTR,
        Type: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredRenameW(
        OldTargetName: LPCWSTR,
        NewTargetName: LPCWSTR,
        Type: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CredRenameA(
        OldTargetName: LPCSTR,
        NewTargetName: LPCSTR,
        Type: DWORD,
        Flags: DWORD,
    ) -> BOOL;
}
pub const CRED_ALLOW_NAME_RESOLUTION: DWORD = 0x1;
extern "system" {
    pub fn CredGetTargetInfoW(
        TargetName: LPCWSTR,
        Flags: DWORD,
        TargetInfo: *mut PCREDENTIAL_TARGET_INFORMATIONW,
    ) -> BOOL;
    pub fn CredGetTargetInfoA(
        TargetName: LPCSTR,
        Flags: DWORD,
        TargetInfo: *mut PCREDENTIAL_TARGET_INFORMATIONA,
    ) -> BOOL;
    pub fn CredMarshalCredentialW(
        CredType: CRED_MARSHAL_TYPE,
        Credential: PVOID,
        MarhaledCredential: *mut LPWSTR,
    ) -> BOOL;
    pub fn CredMarshalCredentialA(
        CredType: CRED_MARSHAL_TYPE,
        Credential: PVOID,
        MarhaledCredential: *mut LPSTR,
    ) -> BOOL;
    pub fn CredUnmarshalCredentialW(
        MarshaledCredential: LPCWSTR,
        CredType: PCRED_MARSHAL_TYPE,
        Credential: *mut PVOID,
    ) -> BOOL;
    pub fn CredUnmarshalCredentialA(
        MarshaledCredential: LPCSTR,
        CredType: PCRED_MARSHAL_TYPE,
        Credential: *mut PVOID,
    ) -> BOOL;
    pub fn CredIsMarshaledCredentialW(
        MarshaledCredential: LPCWSTR,
    ) -> BOOL;
    pub fn CredIsMarshaledCredentialA(
        MarshaledCredential: LPCSTR,
    ) -> BOOL;
    pub fn CredUnPackAuthenticationBufferW(
        dwFlags: DWORD,
        pAuthBuffer: PVOID,
        cbAuthBuffer: DWORD,
        pszUserName: LPWSTR,
        pcchlMaxUserName: *mut DWORD,
        pszDomainName: LPWSTR,
        pcchMaxDomainName: *mut DWORD,
        pszPassword: LPWSTR,
        pcchMaxPassword: *mut DWORD,
    ) -> BOOL;
    pub fn CredUnPackAuthenticationBufferA(
        dwFlags: DWORD,
        pAuthBuffer: PVOID,
        cbAuthBuffer: DWORD,
        pszUserName: LPSTR,
        pcchlMaxUserName: *mut DWORD,
        pszDomainName: LPSTR,
        pcchMaxDomainName: *mut DWORD,
        pszPassword: LPSTR,
        pcchMaxPassword: *mut DWORD,
    ) -> BOOL;
    pub fn CredPackAuthenticationBufferW(
        dwFlags: DWORD,
        pszUserName: LPWSTR,
        pszPassword: LPWSTR,
        pPackedCredentials: PBYTE,
        pcbPackedCredentials: *mut DWORD,
    ) -> BOOL;
    pub fn CredPackAuthenticationBufferA(
        dwFlags: DWORD,
        pszUserName: LPSTR,
        pszPassword: LPSTR,
        pPackedCredentials: PBYTE,
        pcbPackedCredentials: *mut DWORD,
    ) -> BOOL;
    pub fn CredProtectW(
        fAsSelf: BOOL,
        pszCredentials: LPWSTR,
        cchCredentials: DWORD,
        pszProtectedCredentials: LPWSTR,
        pcchMaxChars: *mut DWORD,
        ProtectionType: *mut CRED_PROTECTION_TYPE,
    ) -> BOOL;
    pub fn CredProtectA(
        fAsSelf: BOOL,
        pszCredentials: LPSTR,
        cchCredentials: DWORD,
        pszProtectedCredentials: LPSTR,
        pcchMaxChars: *mut DWORD,
        ProtectionType: *mut CRED_PROTECTION_TYPE,
    ) -> BOOL;
    pub fn CredUnprotectW(
        fAsSelf: BOOL,
        pszProtectedCredentials: LPWSTR,
        cchCredentials: DWORD,
        pszCredentials: LPWSTR,
        pcchMaxChars: *mut DWORD,
    ) -> BOOL;
    pub fn CredUnprotectA(
        fAsSelf: BOOL,
        pszProtectedCredentials: LPSTR,
        cchCredentials: DWORD,
        pszCredentials: LPSTR,
        pcchMaxChars: *mut DWORD,
    ) -> BOOL;
    pub fn CredIsProtectedW(
        pszProtectedCredentials: LPWSTR,
        pProtectionType: *mut CRED_PROTECTION_TYPE,
    ) -> BOOL;
    pub fn CredIsProtectedA(
        pszProtectedCredentials: LPSTR,
        pProtectionType: *mut CRED_PROTECTION_TYPE,
    ) -> BOOL;
    pub fn CredFindBestCredentialW(
        TargetName: LPCWSTR,
        Type: DWORD,
        Flags: DWORD,
        Credential: *mut PCREDENTIALW,
    ) -> BOOL;
    pub fn CredFindBestCredentialA(
        TargetName: LPCSTR,
        Type: DWORD,
        Flags: DWORD,
        Credential: *mut PCREDENTIALA,
    ) -> BOOL;
    pub fn CredGetSessionTypes(
        MaximumPersistCount: DWORD,
        MaximumPersist: LPDWORD,
    ) -> BOOL;
    pub fn CredFree(
        Buffer: PVOID,
    );
    pub fn CredUIPromptForCredentialsW(
        pUiInfo: PCREDUI_INFOW,
        pszTargetName: PCWSTR,
        pContext: PCtxtHandle,
        dwAuthError: DWORD,
        pszUserName: PWSTR,
        ulUserNameBufferSize: ULONG,
        pszPassword: PWSTR,
        ulPasswordBufferSize: ULONG,
        save: *mut BOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForCredentialsA(
        pUiInfo: PCREDUI_INFOA,
        pszTargetName: PCSTR,
        pContext: PCtxtHandle,
        dwAuthError: DWORD,
        pszUserName: PSTR,
        ulUserNameBufferSize: ULONG,
        pszPassword: PSTR,
        ulPasswordBufferSize: ULONG,
        save: *mut BOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForWindowsCredentialsW(
        pUiInfo: PCREDUI_INFOW,
        dwAuthError: DWORD,
        pulAuthPackage: *mut ULONG,
        pvInAuthBuffer: LPCVOID,
        ulInAuthBufferSize: ULONG,
        ppvOutAuthBuffer: *mut LPVOID,
        pulOutAuthBufferSize: *mut ULONG,
        pfSave: *mut BOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForWindowsCredentialsA(
        pUiInfo: PCREDUI_INFOA,
        dwAuthError: DWORD,
        pulAuthPackage: *mut ULONG,
        pvInAuthBuffer: LPCVOID,
        ulInAuthBufferSize: ULONG,
        ppvOutAuthBuffer: *mut LPVOID,
        pulOutAuthBufferSize: *mut ULONG,
        pfSave: *mut BOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIParseUserNameW(
        userName: PCWSTR,
        user: *mut WCHAR,
        userBufferSize: ULONG,
        domain: *mut WCHAR,
        domainBufferSize: ULONG,
    ) -> DWORD;
    pub fn CredUIParseUserNameA(
        userName: PCSTR,
        user: *mut CHAR,
        userBufferSize: ULONG,
        domain: *mut CHAR,
        domainBufferSize: ULONG,
    ) -> DWORD;
    pub fn CredUICmdLinePromptForCredentialsW(
        pszTargetName: PCWSTR,
        pContext: PCtxtHandle,
        dwAuthError: DWORD,
        UserName: PWSTR,
        ulUserBufferSize: ULONG,
        pszPassword: PWSTR,
        ulPasswordBufferSize: ULONG,
        pfSave: PBOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUICmdLinePromptForCredentialsA(
        pszTargetName: PCSTR,
        pContext: PCtxtHandle,
        dwAuthError: DWORD,
        UserName: PSTR,
        ulUserBufferSize: ULONG,
        pszPassword: PSTR,
        ulPasswordBufferSize: ULONG,
        pfSave: PBOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIConfirmCredentialsW(
        pszTargetName: PCWSTR,
        bConfirm: BOOL,
    ) -> DWORD;
    pub fn CredUIConfirmCredentialsA(
        pszTargetName: PCSTR,
        bConfirm: BOOL,
    ) -> DWORD;
    pub fn CredUIStoreSSOCredW(
        pszRealm: PCWSTR,
        pszUsername: PCWSTR,
        pszPassword: PCWSTR,
        bPersist: BOOL,
    ) -> DWORD;
    pub fn CredUIReadSSOCredW(
        pszRealm: PCWSTR,
        ppszUsername: *mut PWSTR,
    ) -> DWORD;
}
