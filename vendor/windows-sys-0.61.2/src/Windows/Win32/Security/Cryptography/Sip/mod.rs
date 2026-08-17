windows_link::link!("crypt32.dll" "system" fn CryptSIPAddProvider(psnewprov : *mut SIP_ADD_NEWPROVIDER) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPCreateIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pcbindirectdata : *mut u32, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPGetCaps(psubjinfo : *const SIP_SUBJECTINFO, pcaps : *mut SIP_CAP_SET_V3) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPGetSealedDigest(psubjectinfo : *const SIP_SUBJECTINFO, psig : *const u8, dwsig : u32, pbdigest : *mut u8, pcbdigest : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPGetSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, pdwencodingtype : *mut super:: CERT_QUERY_ENCODING_TYPE, dwindex : u32, pcbsigneddatamsg : *mut u32, pbsigneddatamsg : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("crypt32.dll" "system" fn CryptSIPLoad(pgsubject : *const windows_sys::core::GUID, dwflags : u32, psipdispatch : *mut SIP_DISPATCH_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPPutSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwencodingtype : super:: CERT_QUERY_ENCODING_TYPE, pdwindex : *mut u32, cbsigneddatamsg : u32, pbsigneddatamsg : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSIPRemoveProvider(pgprov : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPRemoveSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwindex : u32) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuid(filename : windows_sys::core::PCWSTR, hfilein : super::super::super::Foundation:: HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename : windows_sys::core::PCWSTR, hfilein : super::super::super::Foundation:: HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
windows_link::link!("wintrust.dll" "system" fn CryptSIPVerifyIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
pub const MSSIP_ADDINFO_BLOB: u32 = 3u32;
pub const MSSIP_ADDINFO_CATMEMBER: u32 = 2u32;
pub const MSSIP_ADDINFO_FLAT: u32 = 1u32;
pub const MSSIP_ADDINFO_NONE: u32 = 0u32;
pub const MSSIP_ADDINFO_NONMSSIP: u32 = 500u32;
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl Default for MS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
impl Default for MS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut windows_sys::core::GUID,
    pub pwszDLLFileName: windows_sys::core::PWSTR,
    pub pwszMagicNumber: windows_sys::core::PWSTR,
    pub pwszIsFunctionName: windows_sys::core::PWSTR,
    pub pwszGetFuncName: windows_sys::core::PWSTR,
    pub pwszPutFuncName: windows_sys::core::PWSTR,
    pub pwszCreateFuncName: windows_sys::core::PWSTR,
    pub pwszVerifyFuncName: windows_sys::core::PWSTR,
    pub pwszRemoveFuncName: windows_sys::core::PWSTR,
    pub pwszIsFunctionNameFmt2: windows_sys::core::PWSTR,
    pub pwszGetCapFuncName: windows_sys::core::PWSTR,
}
impl Default for SIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_sys::core::BOOL,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_sys::core::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
impl Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
#[derive(Clone, Copy)]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::super::super::Foundation::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
impl Default for SIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPT_INTEGER_BLOB,
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
#[derive(Clone, Copy)]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut windows_sys::core::GUID,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub pwsFileName: windows_sys::core::PCWSTR,
    pub pwsDisplayName: windows_sys::core::PCWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
impl Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
#[derive(Clone, Copy)]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut super::Catalog::MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
impl Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPGetCaps = Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPGetSealedDigest = Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
pub type pfnIsFileSupported = Option<unsafe extern "system" fn(hfile: super::super::super::Foundation::HANDLE, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
pub type pfnIsFileSupportedName = Option<unsafe extern "system" fn(pwszfilename: windows_sys::core::PCWSTR, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
