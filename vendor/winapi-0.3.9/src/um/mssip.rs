// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Microsoft SIP Provider Prototypes and Definitions
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID};
use um::mscat::{CRYPTCATMEMBER, CRYPTCATSTORE};
use um::wincrypt::{
    CRYPT_ALGORITHM_IDENTIFIER, CRYPT_ATTRIBUTE_TYPE_VALUE, CRYPT_HASH_BLOB, HCRYPTPROV,
};
use um::winnt::{HANDLE, LPCWSTR, PWSTR, WCHAR};
pub type CRYPT_DIGEST_DATA = CRYPT_HASH_BLOB;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: DWORD = 0x00010000;
pub const MSSIP_FLAGS_USE_CATALOG: DWORD = 0x00020000;
pub const MSSIP_FLAGS_MULTI_HASH: DWORD = 0x00040000;
pub const SPC_INC_PE_RESOURCES_FLAG: DWORD = 0x80;
pub const SPC_INC_PE_DEBUG_INFO_FLAG: DWORD = 0x40;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: DWORD = 0x20;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: DWORD = 0x10;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: DWORD = 0x100;
pub const SPC_DIGEST_GENERATE_FLAG: DWORD = 0x200;
pub const SPC_DIGEST_SIGN_FLAG: DWORD = 0x400;
pub const SPC_RELAXED_PE_MARKER_CHECK: DWORD = 0x800;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: DWORD = 0x00000001;
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: DWORD
    = SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG;
pub const MSSIP_ADDINFO_NONE: DWORD = 0;
pub const MSSIP_ADDINFO_FLAT: DWORD = 1;
pub const MSSIP_ADDINFO_CATMEMBER: DWORD = 2;
pub const MSSIP_ADDINFO_BLOB: DWORD = 3;
pub const MSSIP_ADDINFO_NONMSSIP: DWORD = 500;
UNION!{union SIP_SUBJECTINFO_u {
    [usize; 1],
    psFlat psFlat_mut: *mut MS_ADDINFO_FLAT,
    psCatMember psCatMember_mut: *mut MS_ADDINFO_CATALOGMEMBER,
    psBlob psBlob_mut: *mut MS_ADDINFO_BLOB,
}}
STRUCT!{struct SIP_SUBJECTINFO {
    cbSize: DWORD,
    pgSubjectType: *mut GUID,
    hFile: HANDLE,
    pwsFileName: LPCWSTR,
    pwsDisplayName: LPCWSTR,
    dwReserved1: DWORD,
    dwIntVersion: DWORD,
    hProv: HCRYPTPROV,
    DigestAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    dwFlags: DWORD,
    dwEncodingType: DWORD,
    dwReserved2: DWORD,
    fdwCAPISettings: DWORD,
    fdwSecuritySettings: DWORD,
    dwIndex: DWORD,
    dwUnionChoice: DWORD,
    u: SIP_SUBJECTINFO_u,
    pClientData: LPVOID,
}}
pub type LPSIP_SUBJECTINFO = *mut SIP_SUBJECTINFO;
STRUCT!{struct MS_ADDINFO_FLAT {
    cbStruct: DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
}}
pub type PMS_ADDINFO_FLAT = *mut MS_ADDINFO_FLAT;
STRUCT!{struct MS_ADDINFO_CATALOGMEMBER {
    cbStruct: DWORD,
    pStore: *mut CRYPTCATSTORE,
    pMember: *mut CRYPTCATMEMBER,
}}
pub type PMS_ADDINFO_CATALOGMEMBER = *mut MS_ADDINFO_CATALOGMEMBER;
STRUCT!{struct MS_ADDINFO_BLOB {
    cbStruct: DWORD,
    cbMemObject: DWORD,
    pbMemObject: *mut BYTE,
    cbMemSignedMsg: DWORD,
    pbMemSignedMsg: *mut BYTE,
}}
pub type PMS_ADDINFO_BLOB = *mut MS_ADDINFO_BLOB;
STRUCT!{struct SIP_CAP_SET_V2 {
    cbSize: DWORD,
    dwVersion: DWORD,
    isMultiSign: BOOL,
    dwReserved: DWORD,
}}
pub type PSIP_CAP_SET_V2 = *mut SIP_CAP_SET_V2;
UNION!{union SIP_CAP_SET_V3_u {
    [u32; 1],
    dwFlags dwFlags_mut: DWORD,
    dwReserved dwReserved_mut: DWORD,
}}
STRUCT!{struct SIP_CAP_SET_V3 {
    cbSize: DWORD,
    dwVersion: DWORD,
    isMultiSign: BOOL,
    u: SIP_CAP_SET_V3_u,
}}
pub type PSIP_CAP_SET_V3 = *mut SIP_CAP_SET_V3;
pub type SIP_CAP_SET = SIP_CAP_SET_V3;
pub type PSIP_CAP_SET = PSIP_CAP_SET_V3;
pub const SIP_CAP_SET_VERSION_2: DWORD = 2;
pub const SIP_CAP_SET_VERSION_3: DWORD = 3;
pub const SIP_CAP_SET_CUR_VER: DWORD = 3;
pub const SIP_CAP_FLAG_SEALING: DWORD = 0x00000001;
STRUCT!{struct SIP_INDIRECT_DATA {
    Data: CRYPT_ATTRIBUTE_TYPE_VALUE,
    DigestAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    Digest: CRYPT_HASH_BLOB,
}}
pub type PSIP_INDIRECT_DATA = *mut SIP_INDIRECT_DATA;
extern "system" {
    pub fn CryptSIPGetSignedDataMsg(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        pdwEncodingType: *mut DWORD,
        dwIndex: DWORD,
        pcbSignedDataMsg: *mut DWORD,
        pbSignedDataMsg: *mut BYTE,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPGetSignedDataMsg(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    pdwEncodingType: *mut DWORD,
    dwIndex: DWORD,
    pcbSignedDataMsg: *mut DWORD,
    pbSignedDataMsg: *mut BYTE,
) -> BOOL}
extern "system" {
    pub fn CryptSIPPutSignedDataMsg(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        dwEncodingType: DWORD,
        pdwIndex: *mut DWORD,
        cbSignedDataMsg: DWORD,
        pbSignedDataMsg: *mut BYTE,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPPutSignedDataMsg(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    dwEncodingType: DWORD,
    pdwIndex: *mut DWORD,
    cbSignedDataMsg: DWORD,
    pbSignedDataMsg: *mut BYTE,
) -> BOOL}
extern "system" {
    pub fn CryptSIPCreateIndirectData(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        pcbIndirectData: *mut DWORD,
        pIndirectData: *mut SIP_INDIRECT_DATA,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPCreateIndirectData(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    pcbIndirectData: *mut DWORD,
    pIndirectData: *mut SIP_INDIRECT_DATA,
) -> BOOL}
extern "system" {
    pub fn CryptSIPVerifyIndirectData(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        pIndirectData: *mut SIP_INDIRECT_DATA,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPVerifyIndirectData(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    pIndirectData: *mut SIP_INDIRECT_DATA,
) -> BOOL}
extern "system" {
    pub fn CryptSIPRemoveSignedDataMsg(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        dwIndex: DWORD,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPRemoveSignedDataMsg(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    dwIndex: DWORD,
) -> BOOL}
STRUCT!{struct SIP_DISPATCH_INFO {
    cbSize: DWORD,
    hSIP: HANDLE,
    pfGet: pCryptSIPGetSignedDataMsg,
    pfPut: pCryptSIPPutSignedDataMsg,
    pfCreate: pCryptSIPCreateIndirectData,
    pfVerify: pCryptSIPVerifyIndirectData,
    pfRemove: pCryptSIPRemoveSignedDataMsg,
}}
pub type LPSIP_DISPATCH_INFO = *mut SIP_DISPATCH_INFO;
FN!{stdcall pfnIsFileSupported(
    hFile: HANDLE,
    pgSubject: *mut GUID,
) -> BOOL}
FN!{stdcall pfnIsFileSupportedName(
    pwszFileName: *mut WCHAR,
    pgSubject: *mut GUID,
) -> BOOL}
STRUCT!{struct SIP_ADD_NEWPROVIDER {
    cbStruct: DWORD,
    pgSubject: *mut GUID,
    pwszDLLFileName: *mut WCHAR,
    pwszMagicNumber: *mut WCHAR,
    pwszIsFunctionName: *mut WCHAR,
    pwszGetFuncName: *mut WCHAR,
    pwszPutFuncName: *mut WCHAR,
    pwszCreateFuncName: *mut WCHAR,
    pwszVerifyFuncName: *mut WCHAR,
    pwszRemoveFuncName: *mut WCHAR,
    pwszIsFunctionNameFmt2: *mut WCHAR,
    pwszGetCapFuncName: PWSTR,
}}
pub type PSIP_ADD_NEWPROVIDER = *mut SIP_ADD_NEWPROVIDER;
pub const SIP_MAX_MAGIC_NUMBER: DWORD = 4;
extern "system" {
    pub fn CryptSIPLoad(
        pgSubject: *const GUID,
        dwFlags: DWORD,
        pSipDispatch: *mut SIP_DISPATCH_INFO,
    ) -> BOOL;
    pub fn CryptSIPRetrieveSubjectGuid(
        FileName: LPCWSTR,
        hFileIn: HANDLE,
        pgSubject: *mut GUID,
    ) -> BOOL;
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile(
        FileName: LPCWSTR,
        hFileIn: HANDLE,
        pgSubject: *mut GUID,
    ) -> BOOL;
    pub fn CryptSIPAddProvider(
        psNewProv: *mut SIP_ADD_NEWPROVIDER,
    ) -> BOOL;
    pub fn CryptSIPRemoveProvider(
        pgProv: *mut GUID,
    ) -> BOOL;
    pub fn CryptSIPGetCaps(
        pSubjInfo: *mut SIP_SUBJECTINFO,
        pCaps: *mut SIP_CAP_SET,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPGetCaps(
    pSubjInfo: *mut SIP_SUBJECTINFO,
    pCaps: *mut SIP_CAP_SET,
) -> BOOL}
extern "system" {
    pub fn CryptSIPGetSealedDigest(
        pSubjectInfo: *mut SIP_SUBJECTINFO,
        pSig: *const BYTE,
        dwSig: DWORD,
        pbDigest: *mut BYTE,
        pcbDigest: *mut DWORD,
    ) -> BOOL;
}
FN!{stdcall pCryptSIPGetSealedDigest(
    pSubjectInfo: *mut SIP_SUBJECTINFO,
    pSig: *const BYTE,
    dwSig: DWORD,
    pbDigest: *mut BYTE,
    pcbDigest: *mut DWORD,
) -> BOOL}
