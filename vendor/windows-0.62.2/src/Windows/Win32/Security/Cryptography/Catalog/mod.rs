#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: Option<*const windows_core::GUID>, dwflags: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut isize, pgsubsystem : *const windows_core::GUID, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P2>(phcatadmin: *mut isize, pgsubsystem: Option<*const windows_core::GUID>, pwszhashalgorithm: P2, pstronghashpolicy: Option<*const super::CERT_STRONG_SIGN_PARA>, dwflags: Option<u32>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut isize, pgsubsystem : *const windows_core::GUID, pwszhashalgorithm : windows_core::PCWSTR, pstronghashpolicy : *const super:: CERT_STRONG_SIGN_PARA, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminAcquireContext2(phcatadmin as _, pgsubsystem.unwrap_or(core::mem::zeroed()) as _, pwszhashalgorithm.param().abi(), pstronghashpolicy.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P1, P2>(hcatadmin: isize, pwszcatalogfile: P1, pwszselectbasename: P2, dwflags: u32) -> isize
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, pwszselectbasename : windows_core::PCWSTR, dwflags : u32) -> isize);
    unsafe { CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.param().abi(), pwszselectbasename.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle(hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::super::super::Foundation:: HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle(hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin: isize, hfile: super::super::super::Foundation::HANDLE, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : isize, hfile : super::super::super::Foundation:: HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile, pcbhash as _, pbhash.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: &[u8], dwflags: Option<u32>, phprevcatinfo: Option<*mut isize>) -> isize {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : isize, pbhash : *const u8, cbhash : u32, dwflags : u32, phprevcatinfo : *mut isize) -> isize);
    unsafe { CryptCATAdminEnumCatalogFromHash(hcatadmin, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), dwflags.unwrap_or(core::mem::zeroed()) as _, phprevcatinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup(dwflags: u32, fresume: bool) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : u32, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CryptCATAdminPauseServiceForBackup(dwflags, fresume.into()) }
}
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : isize, hcatinfo : isize, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseCatalogContext(hcatadmin, hcatinfo, dwflags) }
}
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : isize, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminReleaseContext(hcatadmin, dwflags) }
}
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<P1>(hcatadmin: isize, pwszcatalogfile: P1, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminRemoveCatalog(hcatadmin, pwszcatalogfile.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<P1>(hcatadmin: isize, pwszcatalogfile: P1, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile.param().abi(), pscatinfo as _, dwflags).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P1>(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATAllocSortedMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> windows_core::BOOL);
    unsafe { CryptCATCDFClose(pcdf as _) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumAttributes(pcdf as _, pmember as _, pprevattr as _, pfnparseerror) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributesWithCDFTag<P1>(pcdf: *const CRYPTCATCDF, pwszmembertag: P1, pmember: *const CRYPTCATMEMBER, pprevattr: *const CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributesWithCDFTag(pcdf : *const CRYPTCATCDF, pwszmembertag : windows_core::PCWSTR, pmember : *const CRYPTCATMEMBER, pprevattr : *const CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumAttributesWithCDFTag(pcdf, pwszmembertag.param().abi(), pmember, pprevattr, pfnparseerror) }
}
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATCDFEnumCatAttributes(pcdf as _, pprevattr as _, pfnparseerror) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATCDFEnumMembers(pcdf as _, pprevmember as _, pfnparseerror) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumMembersByCDFTagEx(pcdf: *const CRYPTCATCDF, pwszprevcdftag: windows_core::PWSTR, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK, ppmember: *const *const CRYPTCATMEMBER, fcontinueonerror: bool, pvreserved: *const core::ffi::c_void) -> windows_core::PWSTR {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembersByCDFTagEx(pcdf : *const CRYPTCATCDF, pwszprevcdftag : windows_core::PWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK, ppmember : *const *const CRYPTCATMEMBER, fcontinueonerror : windows_core::BOOL, pvreserved : *const core::ffi::c_void) -> windows_core::PWSTR);
    unsafe { CryptCATCDFEnumMembersByCDFTagEx(pcdf, core::mem::transmute(pwszprevcdftag), pfnparseerror, ppmember, fcontinueonerror.into(), pvreserved) }
}
#[inline]
pub unsafe fn CryptCATCDFOpen<P0>(pwszfilepath: P0, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
    unsafe { CryptCATCDFOpen(pwszfilepath.param().abi(), pfnparseerror) }
}
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : isize, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> windows_core::BOOL);
    unsafe { CryptCATCatalogInfoFromContext(hcatinfo, pscatinfo as _, dwflags).ok() }
}
#[inline]
pub unsafe fn CryptCATClose(hcatalog: super::super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATClose(hcatalog) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATEnumerateAttr(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateAttr(hcatalog, pcatmember as _, pprevattr as _) }
}
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr(hcatalog: super::super::super::Foundation::HANDLE, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::super::super::Foundation:: HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATEnumerateCatAttr(hcatalog, pprevattr as _) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATEnumerateMember(hcatalog: super::super::super::Foundation::HANDLE, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER {
    windows_core::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::super::super::Foundation:: HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATEnumerateMember(hcatalog, pprevmember as _) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER) {
    windows_core::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER));
    unsafe { CryptCATFreeSortedMemberInfo(hcatalog, pcatmember as _) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P2>(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetAttrInfo(hcatalog, pcatmember as _, pwszreferencetag.param().abi()) }
}
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P1>(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATGetCatAttrInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P1>(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATGetMemberInfo(hcatalog, pwszreferencetag.param().abi()) }
}
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::super::super::Foundation:: HANDLE);
    unsafe { CryptCATHandleFromStore(pcatstore as _) }
}
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_core::PCWSTR, fdwopenflags : CRYPTCAT_OPEN_FLAGS, hprov : usize, dwpublicversion : CRYPTCAT_VERSION, dwencodingtype : u32) -> super::super::super::Foundation:: HANDLE);
    unsafe { CryptCATOpen(pwszfilename.param().abi(), fdwopenflags, hprov, dwpublicversion, dwencodingtype) }
}
#[inline]
pub unsafe fn CryptCATPersistStore(hcatalog: super::super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CryptCATPersistStore(hcatalog).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P2>(hcatalog: super::super::super::Foundation::HANDLE, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P2, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutAttrInfo(hcatalog, pcatmember as _, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P1>(hcatalog: super::super::super::Foundation::HANDLE, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    unsafe { CryptCATPutCatAttrInfo(hcatalog, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata as _) }
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P1, P2>(hcatalog: super::super::super::Foundation::HANDLE, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *mut windows_core::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszfilename : windows_core::PCWSTR, pwszreferencetag : windows_core::PCWSTR, pgsubjecttype : *mut windows_core::GUID, dwcertversion : u32, cbsipindirectdata : u32, pbsipindirectdata : *mut u8) -> *mut CRYPTCATMEMBER);
    unsafe { CryptCATPutMemberInfo(hcatalog, pwszfilename.param().abi(), pwszreferencetag.param().abi(), pgsubjecttype as _, dwcertversion, cbsipindirectdata, pbsipindirectdata as _) }
}
#[inline]
pub unsafe fn CryptCATStoreFromHandle(hcatalog: super::super::super::Foundation::HANDLE) -> *mut CRYPTCATSTORE {
    windows_core::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::super::super::Foundation:: HANDLE) -> *mut CRYPTCATSTORE);
    unsafe { CryptCATStoreFromHandle(hcatalog) }
}
#[inline]
pub unsafe fn IsCatalogFile<P1>(hfile: super::super::super::Foundation::HANDLE, pwszfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::super::super::Foundation:: HANDLE, pwszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsCatalogFile(hfile, pwszfilename.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub dwAttrTypeAndAction: CRYPTCATATTRIBUTE_FLAGS,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
impl Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCATATTRIBUTE_FLAGS(pub u32);
impl CRYPTCATATTRIBUTE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPTCATATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPTCATATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPTCATATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPTCATATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPTCATATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: windows_core::BOOL,
    pub pwszResultDir: windows_core::PWSTR,
    pub hCATStore: super::super::super::Foundation::HANDLE,
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub pwszFileName: windows_core::PWSTR,
    pub gSubjectType: windows_core::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut super::Sip::SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub sEncodedIndirectData: super::CRYPT_INTEGER_BLOB,
    pub sEncodedMemberInfo: super::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
impl Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: windows_core::PWSTR,
    pub hProv: usize,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: CRYPTCAT_OPEN_FLAGS,
    pub hReserved: super::super::super::Foundation::HANDLE,
    pub hAttrs: super::super::super::Foundation::HANDLE,
    pub hCryptMsg: *mut core::ffi::c_void,
    pub hSorted: super::super::super::Foundation::HANDLE,
}
impl Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1u32;
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0u32;
pub const CRYPTCAT_ATTR_AUTHENTICATED: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(268435456u32);
pub const CRYPTCAT_ATTR_DATAASCII: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(65536u32);
pub const CRYPTCAT_ATTR_DATABASE64: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(131072u32);
pub const CRYPTCAT_ATTR_DATAREPLACE: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(262144u32);
pub const CRYPTCAT_ATTR_NAMEASCII: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(1u32);
pub const CRYPTCAT_ATTR_NAMEOBJID: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(2u32);
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(16777216u32);
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: CRYPTCATATTRIBUTE_FLAGS = CRYPTCATATTRIBUTE_FLAGS(536870912u32);
pub const CRYPTCAT_E_AREA_ATTRIBUTE: u32 = 131072u32;
pub const CRYPTCAT_E_AREA_HEADER: u32 = 0u32;
pub const CRYPTCAT_E_AREA_MEMBER: u32 = 65536u32;
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: u32 = 131074u32;
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: u32 = 131076u32;
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: u32 = 131073u32;
pub const CRYPTCAT_E_CDF_DUPLICATE: u32 = 2u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: u32 = 65540u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: u32 = 65537u32;
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: u32 = 65538u32;
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: u32 = 4u32;
pub const CRYPTCAT_E_CDF_UNSUPPORTED: u32 = 1u32;
pub const CRYPTCAT_FILEEXT: windows_core::PCWSTR = windows_core::w!("CAT");
pub const CRYPTCAT_MAX_MEMBERTAG: u32 = 64u32;
pub const CRYPTCAT_MEMBER_SORTED: u32 = 1073741824u32;
pub const CRYPTCAT_OPEN_ALWAYS: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(2u32);
pub const CRYPTCAT_OPEN_CREATENEW: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1u32);
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(65536u32);
pub const CRYPTCAT_OPEN_EXISTING: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCAT_OPEN_FLAGS(pub u32);
impl CRYPTCAT_OPEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPTCAT_OPEN_FLAGS_MASK: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4294901760u32);
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(131072u32);
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(536870912u32);
pub const CRYPTCAT_OPEN_SORTED: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1073741824u32);
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(268435456u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPTCAT_VERSION(pub u32);
pub const CRYPTCAT_VERSION_1: CRYPTCAT_VERSION = CRYPTCAT_VERSION(256u32);
pub const CRYPTCAT_VERSION_2: CRYPTCAT_VERSION = CRYPTCAT_VERSION(512u32);
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut CRYPTCATSTORE,
    pub pMember: *mut CRYPTCATMEMBER,
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
impl Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: windows_core::PCWSTR)>;
pub const szOID_CATALOG_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.3");
