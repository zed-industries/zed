// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Microsoft Internet Security Catalog API Prototypes and Definitions
use shared::guiddef::GUID;
use shared::minwindef::DWORD;
use um::mssip::SIP_INDIRECT_DATA;
use um::wincrypt::{CRYPT_ATTR_BLOB, HCRYPTMSG, HCRYPTPROV};
use um::winnt::{HANDLE, LPWSTR};
STRUCT!{struct CRYPTCATSTORE {
    cbStruct: DWORD,
    dwPublicVersion: DWORD,
    pwszP7File: LPWSTR,
    hProv: HCRYPTPROV,
    dwEncodingType: DWORD,
    fdwStoreFlags: DWORD,
    hReserved: HANDLE,
    hAttrs: HANDLE,
    hCryptMsg: HCRYPTMSG,
    hSorted: HANDLE,
}}
STRUCT!{struct CRYPTCATMEMBER {
    cbStruct: DWORD,
    pwszReferenceTag: LPWSTR,
    pwszFileName: LPWSTR,
    gSubjectType: GUID,
    fdwMemberFlags: DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
    dwCertVersion: DWORD,
    dwReserved: DWORD,
    hReserved: HANDLE,
    sEncodedIndirectData: CRYPT_ATTR_BLOB,
    sEncodedMemberInfo: CRYPT_ATTR_BLOB,
}}
