// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This module defines the DAV specific functions that are exposed to the user
use shared::minwindef::{BOOL, DWORD, LPDWORD, PBYTE, PULONG, ULONG};
use um::winnt::{HANDLE, LPCWSTR, LPWSTR, PVOID, PWSTR};
pub type OPAQUE_HANDLE = DWORD;
STRUCT!{struct DAV_CALLBACK_AUTH_BLOB {
    pBuffer: PVOID,
    ulSize: ULONG,
    ulType: ULONG,
}}
pub type PDAV_CALLBACK_AUTH_BLOB = *mut DAV_CALLBACK_AUTH_BLOB;
STRUCT!{struct DAV_CALLBACK_AUTH_UNP {
    pszUserName: LPWSTR,
    ulUserNameLength: ULONG,
    pszPassword: LPWSTR,
    ulPasswordLength: ULONG,
}}
pub type PDAV_CALLBACK_AUTH_UNP = *mut DAV_CALLBACK_AUTH_UNP;
STRUCT!{struct DAV_CALLBACK_CRED {
    AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    UNPBlob: DAV_CALLBACK_AUTH_UNP,
    bAuthBlobValid: BOOL,
    bSave: BOOL,
}}
pub type PDAV_CALLBACK_CRED = *mut DAV_CALLBACK_CRED;
pub const DAV_AUTHN_SCHEME_BASIC: DWORD = 0x00000001;
pub const DAV_AUTHN_SCHEME_NTLM: DWORD = 0x00000002;
pub const DAV_AUTHN_SCHEME_PASSPORT: DWORD = 0x00000004;
pub const DAV_AUTHN_SCHEME_DIGEST: DWORD = 0x00000008;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: DWORD = 0x00000010;
pub const DAV_AUTHN_SCHEME_CERT: DWORD = 0x00010000;
pub const DAV_AUTHN_SCHEME_FBA: DWORD = 0x00100000;
ENUM!{enum AUTHNEXTSTEP {
    DefaultBehavior,
    RetryRequest,
    CancelRequest,
}}
FN!{stdcall PFNDAVAUTHCALLBACK_FREECRED(
    pbuffer: PVOID,
) -> DWORD}
FN!{stdcall PFNDAVAUTHCALLBACK(
    lpwzServerName: LPWSTR,
    lpwzRemoteName: LPWSTR,
    dwAuthScheme: DWORD,
    dwFlags: DWORD,
    pCallbackCred: PDAV_CALLBACK_CRED,
    NextStep: *mut AUTHNEXTSTEP,
    pFreeCred: *mut PFNDAVAUTHCALLBACK_FREECRED,
) -> DWORD}
extern "system" {
    pub fn DavAddConnection(
        ConnectionHandle: *mut HANDLE,
        RemoteName: LPCWSTR,
        UserName: LPCWSTR,
        Password: LPCWSTR,
        ClientCert: PBYTE,
        CertSize: DWORD,
    ) -> DWORD;
    pub fn DavDeleteConnection(
        ConnectionHandle: HANDLE,
    ) -> DWORD;
    pub fn DavGetUNCFromHTTPPath(
        HttpPath: LPCWSTR,
        UncPath: LPWSTR,
        lpSize: LPDWORD,
    ) -> DWORD;
    pub fn DavGetHTTPFromUNCPath(
        UncPath: LPCWSTR,
        HttpPath: LPWSTR,
        lpSize: LPDWORD,
    ) -> DWORD;
    pub fn DavGetTheLockOwnerOfTheFile(
        FileName: LPCWSTR,
        LockOwnerName: PWSTR,
        LockOwnerNameLengthInBytes: PULONG,
    ) -> DWORD;
    pub fn DavGetExtendedError(
        hFile: HANDLE,
        ExtError: *mut DWORD,
        ExtErrorString: LPWSTR,
        cChSize: *mut DWORD,
    ) -> DWORD;
    pub fn DavFlushFile(
        hFile: HANDLE,
    ) -> DWORD;
    pub fn DavInvalidateCache(
        URLName: LPWSTR,
    ) -> DWORD;
    pub fn DavCancelConnectionsToServer(
        URLName: LPWSTR,
        fForce: BOOL,
    ) -> DWORD;
    pub fn DavRegisterAuthCallback(
        CallBack: PFNDAVAUTHCALLBACK,
        Version: ULONG,
    ) -> OPAQUE_HANDLE;
    pub fn DavUnregisterAuthCallback(
        hCallback: OPAQUE_HANDLE,
    );
}
