#[inline]
pub unsafe fn CryptCATAdminAcquireContext(phcatadmin: *mut isize, pgsubsystem: Option<*const windows_core::GUID>, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext(phcatadmin : *mut isize, pgsubsystem : *const windows_core::GUID, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminAcquireContext(phcatadmin, core::mem::transmute(pgsubsystem.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[inline]
pub unsafe fn CryptCATAdminAcquireContext2<P0>(phcatadmin: *mut isize, pgsubsystem: Option<*const windows_core::GUID>, pwszhashalgorithm: P0, pstronghashpolicy: Option<*const super::CERT_STRONG_SIGN_PARA>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminAcquireContext2(phcatadmin : *mut isize, pgsubsystem : *const windows_core::GUID, pwszhashalgorithm : windows_core::PCWSTR, pstronghashpolicy : *const super:: CERT_STRONG_SIGN_PARA, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminAcquireContext2(phcatadmin, core::mem::transmute(pgsubsystem.unwrap_or(std::ptr::null())), pwszhashalgorithm.param().abi(), core::mem::transmute(pstronghashpolicy.unwrap_or(std::ptr::null())), dwflags).ok()
}
#[inline]
pub unsafe fn CryptCATAdminAddCatalog<P0, P1>(hcatadmin: isize, pwszcatalogfile: P0, pwszselectbasename: P1, dwflags: u32) -> isize
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminAddCatalog(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, pwszselectbasename : windows_core::PCWSTR, dwflags : u32) -> isize);
    CryptCATAdminAddCatalog(hcatadmin, pwszcatalogfile.param().abi(), pwszselectbasename.param().abi(), dwflags)
}
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle<P0>(hfile: P0, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle(hfile : super::super::super::Foundation:: HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminCalcHashFromFileHandle(hfile.param().abi(), pcbhash, core::mem::transmute(pbhash.unwrap_or(std::ptr::null_mut())), dwflags)
}
#[inline]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2<P0>(hcatadmin: isize, hfile: P0, pcbhash: *mut u32, pbhash: Option<*mut u8>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminCalcHashFromFileHandle2(hcatadmin : isize, hfile : super::super::super::Foundation:: HANDLE, pcbhash : *mut u32, pbhash : *mut u8, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminCalcHashFromFileHandle2(hcatadmin, hfile.param().abi(), pcbhash, core::mem::transmute(pbhash.unwrap_or(std::ptr::null_mut())), dwflags).ok()
}
#[inline]
pub unsafe fn CryptCATAdminEnumCatalogFromHash(hcatadmin: isize, pbhash: &[u8], dwflags: u32, phprevcatinfo: Option<*mut isize>) -> isize {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminEnumCatalogFromHash(hcatadmin : isize, pbhash : *const u8, cbhash : u32, dwflags : u32, phprevcatinfo : *mut isize) -> isize);
    CryptCATAdminEnumCatalogFromHash(hcatadmin, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), dwflags, core::mem::transmute(phprevcatinfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn CryptCATAdminPauseServiceForBackup<P0>(dwflags: u32, fresume: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminPauseServiceForBackup(dwflags : u32, fresume : super::super::super::Foundation:: BOOL) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminPauseServiceForBackup(dwflags, fresume.param().abi())
}
#[inline]
pub unsafe fn CryptCATAdminReleaseCatalogContext(hcatadmin: isize, hcatinfo: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminReleaseCatalogContext(hcatadmin : isize, hcatinfo : isize, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminReleaseCatalogContext(hcatadmin, hcatinfo, dwflags)
}
#[inline]
pub unsafe fn CryptCATAdminReleaseContext(hcatadmin: isize, dwflags: u32) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminReleaseContext(hcatadmin : isize, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminReleaseContext(hcatadmin, dwflags)
}
#[inline]
pub unsafe fn CryptCATAdminRemoveCatalog<P0>(hcatadmin: isize, pwszcatalogfile: P0, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminRemoveCatalog(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminRemoveCatalog(hcatadmin, pwszcatalogfile.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn CryptCATAdminResolveCatalogPath<P0>(hcatadmin: isize, pwszcatalogfile: P0, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAdminResolveCatalogPath(hcatadmin : isize, pwszcatalogfile : windows_core::PCWSTR, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATAdminResolveCatalogPath(hcatadmin, pwszcatalogfile.param().abi(), pscatinfo, dwflags).ok()
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATAllocSortedMemberInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATAllocSortedMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    CryptCATAllocSortedMemberInfo(hcatalog.param().abi(), pwszreferencetag.param().abi())
}
#[inline]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCDFClose(pcdf : *mut CRYPTCATCDF) -> super::super::super::Foundation:: BOOL);
    CryptCATCDFClose(pcdf)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumAttributes(pcdf: *mut CRYPTCATCDF, pmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCDFEnumAttributes(pcdf : *mut CRYPTCATCDF, pmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    CryptCATCDFEnumAttributes(pcdf, pmember, pprevattr, pfnparseerror)
}
#[inline]
pub unsafe fn CryptCATCDFEnumCatAttributes(pcdf: *mut CRYPTCATCDF, pprevattr: *mut CRYPTCATATTRIBUTE, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCDFEnumCatAttributes(pcdf : *mut CRYPTCATCDF, pprevattr : *mut CRYPTCATATTRIBUTE, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATATTRIBUTE);
    CryptCATCDFEnumCatAttributes(pcdf, pprevattr, pfnparseerror)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATCDFEnumMembers(pcdf: *mut CRYPTCATCDF, pprevmember: *mut CRYPTCATMEMBER, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCDFEnumMembers(pcdf : *mut CRYPTCATCDF, pprevmember : *mut CRYPTCATMEMBER, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATMEMBER);
    CryptCATCDFEnumMembers(pcdf, pprevmember, pfnparseerror)
}
#[inline]
pub unsafe fn CryptCATCDFOpen<P0>(pwszfilepath: P0, pfnparseerror: PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCDFOpen(pwszfilepath : windows_core::PCWSTR, pfnparseerror : PFN_CDF_PARSE_ERROR_CALLBACK) -> *mut CRYPTCATCDF);
    CryptCATCDFOpen(pwszfilepath.param().abi(), pfnparseerror)
}
#[inline]
pub unsafe fn CryptCATCatalogInfoFromContext(hcatinfo: isize, pscatinfo: *mut CATALOG_INFO, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATCatalogInfoFromContext(hcatinfo : isize, pscatinfo : *mut CATALOG_INFO, dwflags : u32) -> super::super::super::Foundation:: BOOL);
    CryptCATCatalogInfoFromContext(hcatinfo, pscatinfo, dwflags).ok()
}
#[inline]
pub unsafe fn CryptCATClose<P0>(hcatalog: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATClose(hcatalog : super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: BOOL);
    CryptCATClose(hcatalog.param().abi())
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATEnumerateAttr<P0>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATEnumerateAttr(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    CryptCATEnumerateAttr(hcatalog.param().abi(), pcatmember, pprevattr)
}
#[inline]
pub unsafe fn CryptCATEnumerateCatAttr<P0>(hcatalog: P0, pprevattr: *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATEnumerateCatAttr(hcatalog : super::super::super::Foundation:: HANDLE, pprevattr : *mut CRYPTCATATTRIBUTE) -> *mut CRYPTCATATTRIBUTE);
    CryptCATEnumerateCatAttr(hcatalog.param().abi(), pprevattr)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATEnumerateMember<P0>(hcatalog: P0, pprevmember: *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATEnumerateMember(hcatalog : super::super::super::Foundation:: HANDLE, pprevmember : *mut CRYPTCATMEMBER) -> *mut CRYPTCATMEMBER);
    CryptCATEnumerateMember(hcatalog.param().abi(), pprevmember)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATFreeSortedMemberInfo<P0>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER)
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATFreeSortedMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER));
    CryptCATFreeSortedMemberInfo(hcatalog.param().abi(), pcatmember)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATGetAttrInfo<P0, P1>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATGetAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    CryptCATGetAttrInfo(hcatalog.param().abi(), pcatmember, pwszreferencetag.param().abi())
}
#[inline]
pub unsafe fn CryptCATGetCatAttrInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATGetCatAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATATTRIBUTE);
    CryptCATGetCatAttrInfo(hcatalog.param().abi(), pwszreferencetag.param().abi())
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATGetMemberInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1) -> *mut CRYPTCATMEMBER
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATGetMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR) -> *mut CRYPTCATMEMBER);
    CryptCATGetMemberInfo(hcatalog.param().abi(), pwszreferencetag.param().abi())
}
#[inline]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::super::super::Foundation::HANDLE {
    windows_targets::link!("wintrust.dll" "system" fn CryptCATHandleFromStore(pcatstore : *mut CRYPTCATSTORE) -> super::super::super::Foundation:: HANDLE);
    CryptCATHandleFromStore(pcatstore)
}
#[inline]
pub unsafe fn CryptCATOpen<P0>(pwszfilename: P0, fdwopenflags: CRYPTCAT_OPEN_FLAGS, hprov: usize, dwpublicversion: CRYPTCAT_VERSION, dwencodingtype: u32) -> super::super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATOpen(pwszfilename : windows_core::PCWSTR, fdwopenflags : CRYPTCAT_OPEN_FLAGS, hprov : usize, dwpublicversion : CRYPTCAT_VERSION, dwencodingtype : u32) -> super::super::super::Foundation:: HANDLE);
    CryptCATOpen(pwszfilename.param().abi(), fdwopenflags, hprov, dwpublicversion, dwencodingtype)
}
#[inline]
pub unsafe fn CryptCATPersistStore<P0>(hcatalog: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATPersistStore(hcatalog : super::super::super::Foundation:: HANDLE) -> super::super::super::Foundation:: BOOL);
    CryptCATPersistStore(hcatalog.param().abi()).ok()
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATPutAttrInfo<P0, P1>(hcatalog: P0, pcatmember: *mut CRYPTCATMEMBER, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATPutAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pcatmember : *mut CRYPTCATMEMBER, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    CryptCATPutAttrInfo(hcatalog.param().abi(), pcatmember, pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata)
}
#[inline]
pub unsafe fn CryptCATPutCatAttrInfo<P0, P1>(hcatalog: P0, pwszreferencetag: P1, dwattrtypeandaction: u32, cbdata: u32, pbdata: *mut u8) -> *mut CRYPTCATATTRIBUTE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATPutCatAttrInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszreferencetag : windows_core::PCWSTR, dwattrtypeandaction : u32, cbdata : u32, pbdata : *mut u8) -> *mut CRYPTCATATTRIBUTE);
    CryptCATPutCatAttrInfo(hcatalog.param().abi(), pwszreferencetag.param().abi(), dwattrtypeandaction, cbdata, pbdata)
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[inline]
pub unsafe fn CryptCATPutMemberInfo<P0, P1, P2>(hcatalog: P0, pwszfilename: P1, pwszreferencetag: P2, pgsubjecttype: *mut windows_core::GUID, dwcertversion: u32, cbsipindirectdata: u32, pbsipindirectdata: *mut u8) -> *mut CRYPTCATMEMBER
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATPutMemberInfo(hcatalog : super::super::super::Foundation:: HANDLE, pwszfilename : windows_core::PCWSTR, pwszreferencetag : windows_core::PCWSTR, pgsubjecttype : *mut windows_core::GUID, dwcertversion : u32, cbsipindirectdata : u32, pbsipindirectdata : *mut u8) -> *mut CRYPTCATMEMBER);
    CryptCATPutMemberInfo(hcatalog.param().abi(), pwszfilename.param().abi(), pwszreferencetag.param().abi(), pgsubjecttype, dwcertversion, cbsipindirectdata, pbsipindirectdata)
}
#[inline]
pub unsafe fn CryptCATStoreFromHandle<P0>(hcatalog: P0) -> *mut CRYPTCATSTORE
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wintrust.dll" "system" fn CryptCATStoreFromHandle(hcatalog : super::super::super::Foundation:: HANDLE) -> *mut CRYPTCATSTORE);
    CryptCATStoreFromHandle(hcatalog.param().abi())
}
#[inline]
pub unsafe fn IsCatalogFile<P0, P1>(hfile: P0, pwszfilename: P1) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wintrust.dll" "system" fn IsCatalogFile(hfile : super::super::super::Foundation:: HANDLE, pwszfilename : windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    IsCatalogFile(hfile.param().abi(), pwszfilename.param().abi())
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1u32;
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0u32;
pub const CRYPTCAT_ATTR_AUTHENTICATED: u32 = 268435456u32;
pub const CRYPTCAT_ATTR_DATAASCII: u32 = 65536u32;
pub const CRYPTCAT_ATTR_DATABASE64: u32 = 131072u32;
pub const CRYPTCAT_ATTR_DATAREPLACE: u32 = 262144u32;
pub const CRYPTCAT_ATTR_NAMEASCII: u32 = 1u32;
pub const CRYPTCAT_ATTR_NAMEOBJID: u32 = 2u32;
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: u32 = 16777216u32;
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: u32 = 536870912u32;
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
pub const CRYPTCAT_OPEN_FLAGS_MASK: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4294901760u32);
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(131072u32);
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(536870912u32);
pub const CRYPTCAT_OPEN_SORTED: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1073741824u32);
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(268435456u32);
pub const CRYPTCAT_VERSION_1: CRYPTCAT_VERSION = CRYPTCAT_VERSION(256u32);
pub const CRYPTCAT_VERSION_2: CRYPTCAT_VERSION = CRYPTCAT_VERSION(512u32);
pub const szOID_CATALOG_LIST: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.1");
pub const szOID_CATALOG_LIST_MEMBER: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.2");
pub const szOID_CATALOG_LIST_MEMBER2: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.12.1.3");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CRYPTCAT_OPEN_FLAGS(pub u32);
impl windows_core::TypeKind for CRYPTCAT_OPEN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CRYPTCAT_OPEN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CRYPTCAT_OPEN_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CRYPTCAT_VERSION(pub u32);
impl windows_core::TypeKind for CRYPTCAT_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CRYPTCAT_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CRYPTCAT_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl windows_core::TypeKind for CATALOG_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: windows_core::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for CRYPTCATATTRIBUTE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: super::super::super::Foundation::BOOL,
    pub pwszResultDir: windows_core::PWSTR,
    pub hCATStore: super::super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for CRYPTCATCDF {
    type TypeKind = windows_core::CopyType;
}
impl Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for CRYPTCATMEMBER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
impl Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for CRYPTCATSTORE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut CRYPTCATSTORE,
    pub pMember: *mut CRYPTCATMEMBER,
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
impl windows_core::TypeKind for MS_ADDINFO_CATALOGMEMBER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
impl Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_CDF_PARSE_ERROR_CALLBACK = Option<unsafe extern "system" fn(dwerrorarea: u32, dwlocalerror: u32, pwszline: windows_core::PCWSTR)>;
