#[inline]
pub unsafe fn AppCacheCheckManifest<P0, P1>(pwszmasterurl: P0, pwszmanifesturl: P1, pbmanifestdata: &[u8], pbmanifestresponseheaders: &[u8], pestate: *mut APP_CACHE_STATE, phnewappcache: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheCheckManifest(pwszmasterurl : windows_core::PCWSTR, pwszmanifesturl : windows_core::PCWSTR, pbmanifestdata : *const u8, dwmanifestdatasize : u32, pbmanifestresponseheaders : *const u8, dwmanifestresponseheaderssize : u32, pestate : *mut APP_CACHE_STATE, phnewappcache : *mut *mut core::ffi::c_void) -> u32);
    unsafe { AppCacheCheckManifest(pwszmasterurl.param().abi(), pwszmanifesturl.param().abi(), core::mem::transmute(pbmanifestdata.as_ptr()), pbmanifestdata.len().try_into().unwrap(), core::mem::transmute(pbmanifestresponseheaders.as_ptr()), pbmanifestresponseheaders.len().try_into().unwrap(), pestate as _, phnewappcache as _) }
}
#[inline]
pub unsafe fn AppCacheCloseHandle(happcache: *const core::ffi::c_void) {
    windows_core::link!("wininet.dll" "system" fn AppCacheCloseHandle(happcache : *const core::ffi::c_void));
    unsafe { AppCacheCloseHandle(happcache) }
}
#[inline]
pub unsafe fn AppCacheCreateAndCommitFile<P1, P2>(happcache: *const core::ffi::c_void, pwszsourcefilepath: P1, pwszurl: P2, pbresponseheaders: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheCreateAndCommitFile(happcache : *const core::ffi::c_void, pwszsourcefilepath : windows_core::PCWSTR, pwszurl : windows_core::PCWSTR, pbresponseheaders : *const u8, dwresponseheaderssize : u32) -> u32);
    unsafe { AppCacheCreateAndCommitFile(happcache, pwszsourcefilepath.param().abi(), pwszurl.param().abi(), core::mem::transmute(pbresponseheaders.as_ptr()), pbresponseheaders.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn AppCacheDeleteGroup<P0>(pwszmanifesturl: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheDeleteGroup(pwszmanifesturl : windows_core::PCWSTR) -> u32);
    unsafe { AppCacheDeleteGroup(pwszmanifesturl.param().abi()) }
}
#[inline]
pub unsafe fn AppCacheDeleteIEGroup<P0>(pwszmanifesturl: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheDeleteIEGroup(pwszmanifesturl : windows_core::PCWSTR) -> u32);
    unsafe { AppCacheDeleteIEGroup(pwszmanifesturl.param().abi()) }
}
#[inline]
pub unsafe fn AppCacheDuplicateHandle(happcache: *const core::ffi::c_void, phduplicatedappcache: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheDuplicateHandle(happcache : *const core::ffi::c_void, phduplicatedappcache : *mut *mut core::ffi::c_void) -> u32);
    unsafe { AppCacheDuplicateHandle(happcache, phduplicatedappcache as _) }
}
#[inline]
pub unsafe fn AppCacheFinalize(happcache: *const core::ffi::c_void, pbmanifestdata: &[u8], pestate: *mut APP_CACHE_FINALIZE_STATE) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheFinalize(happcache : *const core::ffi::c_void, pbmanifestdata : *const u8, dwmanifestdatasize : u32, pestate : *mut APP_CACHE_FINALIZE_STATE) -> u32);
    unsafe { AppCacheFinalize(happcache, core::mem::transmute(pbmanifestdata.as_ptr()), pbmanifestdata.len().try_into().unwrap(), pestate as _) }
}
#[inline]
pub unsafe fn AppCacheFreeDownloadList(pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) {
    windows_core::link!("wininet.dll" "system" fn AppCacheFreeDownloadList(pdownloadlist : *mut APP_CACHE_DOWNLOAD_LIST));
    unsafe { AppCacheFreeDownloadList(pdownloadlist as _) }
}
#[inline]
pub unsafe fn AppCacheFreeGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) {
    windows_core::link!("wininet.dll" "system" fn AppCacheFreeGroupList(pappcachegrouplist : *mut APP_CACHE_GROUP_LIST));
    unsafe { AppCacheFreeGroupList(pappcachegrouplist as _) }
}
#[inline]
pub unsafe fn AppCacheFreeIESpace(ftcutoff: super::super::Foundation::FILETIME) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheFreeIESpace(ftcutoff : super::super::Foundation:: FILETIME) -> u32);
    unsafe { AppCacheFreeIESpace(core::mem::transmute(ftcutoff)) }
}
#[inline]
pub unsafe fn AppCacheFreeSpace(ftcutoff: super::super::Foundation::FILETIME) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheFreeSpace(ftcutoff : super::super::Foundation:: FILETIME) -> u32);
    unsafe { AppCacheFreeSpace(core::mem::transmute(ftcutoff)) }
}
#[inline]
pub unsafe fn AppCacheGetDownloadList(happcache: *const core::ffi::c_void, pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheGetDownloadList(happcache : *const core::ffi::c_void, pdownloadlist : *mut APP_CACHE_DOWNLOAD_LIST) -> u32);
    unsafe { AppCacheGetDownloadList(happcache, pdownloadlist as _) }
}
#[inline]
pub unsafe fn AppCacheGetFallbackUrl<P1>(happcache: *const core::ffi::c_void, pwszurl: P1, ppwszfallbackurl: *mut windows_core::PWSTR) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheGetFallbackUrl(happcache : *const core::ffi::c_void, pwszurl : windows_core::PCWSTR, ppwszfallbackurl : *mut windows_core::PWSTR) -> u32);
    unsafe { AppCacheGetFallbackUrl(happcache, pwszurl.param().abi(), ppwszfallbackurl as _) }
}
#[inline]
pub unsafe fn AppCacheGetGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheGetGroupList(pappcachegrouplist : *mut APP_CACHE_GROUP_LIST) -> u32);
    unsafe { AppCacheGetGroupList(pappcachegrouplist as _) }
}
#[inline]
pub unsafe fn AppCacheGetIEGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheGetIEGroupList(pappcachegrouplist : *mut APP_CACHE_GROUP_LIST) -> u32);
    unsafe { AppCacheGetIEGroupList(pappcachegrouplist as _) }
}
#[inline]
pub unsafe fn AppCacheGetInfo(happcache: *const core::ffi::c_void, pappcacheinfo: *mut APP_CACHE_GROUP_INFO) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheGetInfo(happcache : *const core::ffi::c_void, pappcacheinfo : *mut APP_CACHE_GROUP_INFO) -> u32);
    unsafe { AppCacheGetInfo(happcache, pappcacheinfo as _) }
}
#[inline]
pub unsafe fn AppCacheGetManifestUrl(happcache: *const core::ffi::c_void, ppwszmanifesturl: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("wininet.dll" "system" fn AppCacheGetManifestUrl(happcache : *const core::ffi::c_void, ppwszmanifesturl : *mut windows_core::PWSTR) -> u32);
    unsafe { AppCacheGetManifestUrl(happcache, ppwszmanifesturl as _) }
}
#[inline]
pub unsafe fn AppCacheLookup<P0>(pwszurl: P0, dwflags: u32, phappcache: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn AppCacheLookup(pwszurl : windows_core::PCWSTR, dwflags : u32, phappcache : *mut *mut core::ffi::c_void) -> u32);
    unsafe { AppCacheLookup(pwszurl.param().abi(), dwflags, phappcache as _) }
}
#[inline]
pub unsafe fn CommitUrlCacheEntryA<P0, P1, P7, P8>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpheaderinfo: Option<&[u8]>, lpszfileextension: P7, lpszoriginalurl: P8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CommitUrlCacheEntryA(lpszurlname : windows_core::PCSTR, lpszlocalfilename : windows_core::PCSTR, expiretime : super::super::Foundation:: FILETIME, lastmodifiedtime : super::super::Foundation:: FILETIME, cacheentrytype : u32, lpheaderinfo : *const u8, cchheaderinfo : u32, lpszfileextension : windows_core::PCSTR, lpszoriginalurl : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CommitUrlCacheEntryA(lpszurlname.param().abi(), lpszlocalfilename.param().abi(), core::mem::transmute(expiretime), core::mem::transmute(lastmodifiedtime), cacheentrytype, core::mem::transmute(lpheaderinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpheaderinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszfileextension.param().abi(), lpszoriginalurl.param().abi()).ok() }
}
#[inline]
pub unsafe fn CommitUrlCacheEntryBinaryBlob<P0>(pwszurlname: P0, dwtype: u32, ftexpiretime: super::super::Foundation::FILETIME, ftmodifiedtime: super::super::Foundation::FILETIME, pbblob: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CommitUrlCacheEntryBinaryBlob(pwszurlname : windows_core::PCWSTR, dwtype : u32, ftexpiretime : super::super::Foundation:: FILETIME, ftmodifiedtime : super::super::Foundation:: FILETIME, pbblob : *const u8, cbblob : u32) -> u32);
    unsafe { CommitUrlCacheEntryBinaryBlob(pwszurlname.param().abi(), dwtype, core::mem::transmute(ftexpiretime), core::mem::transmute(ftmodifiedtime), core::mem::transmute(pbblob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbblob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn CommitUrlCacheEntryW<P0, P1, P7, P8>(lpszurlname: P0, lpszlocalfilename: P1, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpszheaderinfo: Option<&[u16]>, lpszfileextension: P7, lpszoriginalurl: P8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CommitUrlCacheEntryW(lpszurlname : windows_core::PCWSTR, lpszlocalfilename : windows_core::PCWSTR, expiretime : super::super::Foundation:: FILETIME, lastmodifiedtime : super::super::Foundation:: FILETIME, cacheentrytype : u32, lpszheaderinfo : windows_core::PCWSTR, cchheaderinfo : u32, lpszfileextension : windows_core::PCWSTR, lpszoriginalurl : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CommitUrlCacheEntryW(lpszurlname.param().abi(), lpszlocalfilename.param().abi(), core::mem::transmute(expiretime), core::mem::transmute(lastmodifiedtime), cacheentrytype, core::mem::transmute(lpszheaderinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaderinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszfileextension.param().abi(), lpszoriginalurl.param().abi()).ok() }
}
#[inline]
pub unsafe fn CreateMD5SSOHash<P0, P1, P2>(pszchallengeinfo: P0, pwszrealm: P1, pwsztarget: P2, pbhexhash: *mut u8) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateMD5SSOHash(pszchallengeinfo : windows_core::PCWSTR, pwszrealm : windows_core::PCWSTR, pwsztarget : windows_core::PCWSTR, pbhexhash : *mut u8) -> windows_core::BOOL);
    unsafe { CreateMD5SSOHash(pszchallengeinfo.param().abi(), pwszrealm.param().abi(), pwsztarget.param().abi(), pbhexhash as _) }
}
#[inline]
pub unsafe fn CreateUrlCacheContainerA<P0, P1, P2>(name: P0, lpcacheprefix: P1, lpszcachepath: P2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: Option<*const core::ffi::c_void>, cbbuffer: Option<*const u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheContainerA(name : windows_core::PCSTR, lpcacheprefix : windows_core::PCSTR, lpszcachepath : windows_core::PCSTR, kbcachelimit : u32, dwcontainertype : u32, dwoptions : u32, pvbuffer : *const core::ffi::c_void, cbbuffer : *const u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheContainerA(name.param().abi(), lpcacheprefix.param().abi(), lpszcachepath.param().abi(), kbcachelimit, dwcontainertype, dwoptions, pvbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CreateUrlCacheContainerW<P0, P1, P2>(name: P0, lpcacheprefix: P1, lpszcachepath: P2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: Option<*const core::ffi::c_void>, cbbuffer: Option<*const u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheContainerW(name : windows_core::PCWSTR, lpcacheprefix : windows_core::PCWSTR, lpszcachepath : windows_core::PCWSTR, kbcachelimit : u32, dwcontainertype : u32, dwoptions : u32, pvbuffer : *const core::ffi::c_void, cbbuffer : *const u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheContainerW(name.param().abi(), lpcacheprefix.param().abi(), lpszcachepath.param().abi(), kbcachelimit, dwcontainertype, dwoptions, pvbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CreateUrlCacheEntryA<P0, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P2, lpszfilename: &mut [u8; 260], dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheEntryA(lpszurlname : windows_core::PCSTR, dwexpectedfilesize : u32, lpszfileextension : windows_core::PCSTR, lpszfilename : windows_core::PSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheEntryA(lpszurlname.param().abi(), dwexpectedfilesize, lpszfileextension.param().abi(), core::mem::transmute(lpszfilename.as_ptr()), dwreserved).ok() }
}
#[inline]
pub unsafe fn CreateUrlCacheEntryExW<P0, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P2, lpszfilename: &mut [u16; 260], dwreserved: u32, fpreserveincomingfilename: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheEntryExW(lpszurlname : windows_core::PCWSTR, dwexpectedfilesize : u32, lpszfileextension : windows_core::PCWSTR, lpszfilename : windows_core::PWSTR, dwreserved : u32, fpreserveincomingfilename : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CreateUrlCacheEntryExW(lpszurlname.param().abi(), dwexpectedfilesize, lpszfileextension.param().abi(), core::mem::transmute(lpszfilename.as_ptr()), dwreserved, fpreserveincomingfilename.into()) }
}
#[inline]
pub unsafe fn CreateUrlCacheEntryW<P0, P2>(lpszurlname: P0, dwexpectedfilesize: u32, lpszfileextension: P2, lpszfilename: &mut [u16; 260], dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheEntryW(lpszurlname : windows_core::PCWSTR, dwexpectedfilesize : u32, lpszfileextension : windows_core::PCWSTR, lpszfilename : windows_core::PWSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { CreateUrlCacheEntryW(lpszurlname.param().abi(), dwexpectedfilesize, lpszfileextension.param().abi(), core::mem::transmute(lpszfilename.as_ptr()), dwreserved).ok() }
}
#[inline]
pub unsafe fn CreateUrlCacheGroup(dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> i64 {
    windows_core::link!("wininet.dll" "system" fn CreateUrlCacheGroup(dwflags : u32, lpreserved : *const core::ffi::c_void) -> i64);
    unsafe { CreateUrlCacheGroup(dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteIE3Cache<P2>(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: P2, ncmdshow: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteIE3Cache(hwnd : super::super::Foundation:: HWND, hinst : super::super::Foundation:: HINSTANCE, lpszcmd : windows_core::PCSTR, ncmdshow : i32) -> u32);
    unsafe { DeleteIE3Cache(hwnd, hinst, lpszcmd.param().abi(), ncmdshow) }
}
#[inline]
pub unsafe fn DeleteUrlCacheContainerA<P0>(name: P0, dwoptions: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheContainerA(name : windows_core::PCSTR, dwoptions : u32) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheContainerA(name.param().abi(), dwoptions).ok() }
}
#[inline]
pub unsafe fn DeleteUrlCacheContainerW<P0>(name: P0, dwoptions: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheContainerW(name : windows_core::PCWSTR, dwoptions : u32) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheContainerW(name.param().abi(), dwoptions).ok() }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntry<P0>(lpszurlname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntry(lpszurlname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntry(lpszurlname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntryA<P0>(lpszurlname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntryA(lpszurlname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntryA(lpszurlname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteUrlCacheEntryW<P0>(lpszurlname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheEntryW(lpszurlname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheEntryW(lpszurlname.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeleteUrlCacheGroup(groupid: i64, dwflags: u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn DeleteUrlCacheGroup(groupid : i64, dwflags : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeleteUrlCacheGroup(groupid, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn DeleteWpadCacheForNetworks(param0 : WPAD_CACHE_DELETE) -> windows_core::BOOL);
    unsafe { DeleteWpadCacheForNetworks(param0) }
}
#[inline]
pub unsafe fn DetectAutoProxyUrl(pszautoproxyurl: &mut [u8], dwdetectflags: PROXY_AUTO_DETECT_TYPE) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn DetectAutoProxyUrl(pszautoproxyurl : windows_core::PSTR, cchautoproxyurl : u32, dwdetectflags : PROXY_AUTO_DETECT_TYPE) -> windows_core::BOOL);
    unsafe { DetectAutoProxyUrl(core::mem::transmute(pszautoproxyurl.as_ptr()), pszautoproxyurl.len().try_into().unwrap(), dwdetectflags).ok() }
}
#[inline]
pub unsafe fn DoConnectoidsExist() -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn DoConnectoidsExist() -> windows_core::BOOL);
    unsafe { DoConnectoidsExist() }
}
#[inline]
pub unsafe fn ExportCookieFileA<P0>(szfilename: P0, fappend: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn ExportCookieFileA(szfilename : windows_core::PCSTR, fappend : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ExportCookieFileA(szfilename.param().abi(), fappend.into()) }
}
#[inline]
pub unsafe fn ExportCookieFileW<P0>(szfilename: P0, fappend: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn ExportCookieFileW(szfilename : windows_core::PCWSTR, fappend : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ExportCookieFileW(szfilename.param().abi(), fappend.into()) }
}
#[inline]
pub unsafe fn FindCloseUrlCache(henumhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindCloseUrlCache(henumhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FindCloseUrlCache(henumhandle).ok() }
}
#[inline]
pub unsafe fn FindFirstUrlCacheContainerA(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheContainerA(pdwmodified : *mut u32, lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo : *mut u32, dwoptions : u32) -> super::super::Foundation:: HANDLE);
    unsafe { FindFirstUrlCacheContainerA(pdwmodified as _, lpcontainerinfo as _, lpcbcontainerinfo as _, dwoptions) }
}
#[inline]
pub unsafe fn FindFirstUrlCacheContainerW(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheContainerW(pdwmodified : *mut u32, lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo : *mut u32, dwoptions : u32) -> super::super::Foundation:: HANDLE);
    unsafe { FindFirstUrlCacheContainerW(pdwmodified as _, lpcontainerinfo as _, lpcbcontainerinfo as _, dwoptions) }
}
#[inline]
pub unsafe fn FindFirstUrlCacheEntryA<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryA(lpszurlsearchpattern : windows_core::PCSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { FindFirstUrlCacheEntryA(lpszurlsearchpattern.param().abi(), lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExA<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExA(lpszurlsearchpattern : windows_core::PCSTR, dwflags : u32, dwfilter : u32, groupid : i64, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { FindFirstUrlCacheEntryExA(lpszurlsearchpattern.param().abi(), dwflags, dwfilter, groupid, lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExW<P0>(lpszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryExW(lpszurlsearchpattern : windows_core::PCWSTR, dwflags : u32, dwfilter : u32, groupid : i64, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { FindFirstUrlCacheEntryExW(lpszurlsearchpattern.param().abi(), dwflags, dwfilter, groupid, lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn FindFirstUrlCacheEntryW<P0>(lpszurlsearchpattern: P0, lpfirstcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheEntryW(lpszurlsearchpattern : windows_core::PCWSTR, lpfirstcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { FindFirstUrlCacheEntryW(lpszurlsearchpattern.param().abi(), lpfirstcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: Option<*const core::ffi::c_void>, dwsearchcondition: Option<u32>, lpgroupid: *mut i64, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("wininet.dll" "system" fn FindFirstUrlCacheGroup(dwflags : u32, dwfilter : u32, lpsearchcondition : *const core::ffi::c_void, dwsearchcondition : u32, lpgroupid : *mut i64, lpreserved : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { FindFirstUrlCacheGroup(dwflags, dwfilter, lpsearchcondition.unwrap_or(core::mem::zeroed()) as _, dwsearchcondition.unwrap_or(core::mem::zeroed()) as _, lpgroupid as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn FindNextUrlCacheContainerA(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheContainerA(henumhandle : super::super::Foundation:: HANDLE, lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheContainerA(henumhandle, lpcontainerinfo as _, lpcbcontainerinfo as _) }
}
#[inline]
pub unsafe fn FindNextUrlCacheContainerW(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheContainerW(henumhandle : super::super::Foundation:: HANDLE, lpcontainerinfo : *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheContainerW(henumhandle, lpcontainerinfo as _, lpcbcontainerinfo as _) }
}
#[inline]
pub unsafe fn FindNextUrlCacheEntryA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryA(henumhandle : super::super::Foundation:: HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryA(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _).ok() }
}
#[inline]
pub unsafe fn FindNextUrlCacheEntryExA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExA(henumhandle : super::super::Foundation:: HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryExA(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FindNextUrlCacheEntryExW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, lpgroupattributes: Option<*const core::ffi::c_void>, lpcbgroupattributes: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryExW(henumhandle : super::super::Foundation:: HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpgroupattributes : *const core::ffi::c_void, lpcbgroupattributes : *const u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryExW(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, lpgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpcbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FindNextUrlCacheEntryW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheEntryW(henumhandle : super::super::Foundation:: HANDLE, lpnextcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheEntryW(henumhandle, lpnextcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _).ok() }
}
#[inline]
pub unsafe fn FindNextUrlCacheGroup(hfind: super::super::Foundation::HANDLE, lpgroupid: *mut i64, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FindNextUrlCacheGroup(hfind : super::super::Foundation:: HANDLE, lpgroupid : *mut i64, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextUrlCacheGroup(hfind, lpgroupid as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FindP3PPolicySymbol<P0>(pszsymbol: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FindP3PPolicySymbol(pszsymbol : windows_core::PCSTR) -> i32);
    unsafe { FindP3PPolicySymbol(pszsymbol.param().abi()) }
}
#[inline]
pub unsafe fn FreeUrlCacheSpaceA<P0>(lpszcachepath: P0, dwsize: u32, dwfilter: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FreeUrlCacheSpaceA(lpszcachepath : windows_core::PCSTR, dwsize : u32, dwfilter : u32) -> windows_core::BOOL);
    unsafe { FreeUrlCacheSpaceA(lpszcachepath.param().abi(), dwsize, dwfilter).ok() }
}
#[inline]
pub unsafe fn FreeUrlCacheSpaceW<P0>(lpszcachepath: P0, dwsize: u32, dwfilter: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FreeUrlCacheSpaceW(lpszcachepath : windows_core::PCWSTR, dwsize : u32, dwfilter : u32) -> windows_core::BOOL);
    unsafe { FreeUrlCacheSpaceW(lpszcachepath.param().abi(), dwsize, dwfilter).ok() }
}
#[inline]
pub unsafe fn FtpCommandA<P3>(hconnect: *const core::ffi::c_void, fexpectresponse: bool, dwflags: FTP_FLAGS, lpszcommand: P3, dwcontext: Option<usize>, phftpcommand: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCommandA(hconnect : *const core::ffi::c_void, fexpectresponse : windows_core::BOOL, dwflags : FTP_FLAGS, lpszcommand : windows_core::PCSTR, dwcontext : usize, phftpcommand : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FtpCommandA(hconnect, fexpectresponse.into(), dwflags, lpszcommand.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, phftpcommand.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpCommandW<P3>(hconnect: *const core::ffi::c_void, fexpectresponse: bool, dwflags: FTP_FLAGS, lpszcommand: P3, dwcontext: Option<usize>, phftpcommand: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCommandW(hconnect : *const core::ffi::c_void, fexpectresponse : windows_core::BOOL, dwflags : FTP_FLAGS, lpszcommand : windows_core::PCWSTR, dwcontext : usize, phftpcommand : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FtpCommandW(hconnect, fexpectresponse.into(), dwflags, lpszcommand.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, phftpcommand.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpCreateDirectoryA<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCreateDirectoryA(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpCreateDirectoryA(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpCreateDirectoryW<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpCreateDirectoryW(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpCreateDirectoryW(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpDeleteFileA<P1>(hconnect: *const core::ffi::c_void, lpszfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpDeleteFileA(hconnect : *const core::ffi::c_void, lpszfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpDeleteFileA(hconnect, lpszfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpDeleteFileW<P1>(hconnect: *const core::ffi::c_void, lpszfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpDeleteFileW(hconnect : *const core::ffi::c_void, lpszfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpDeleteFileW(hconnect, lpszfilename.param().abi()).ok() }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn FtpFindFirstFileA<P1>(hconnect: *const core::ffi::c_void, lpszsearchfile: P1, lpfindfiledata: Option<*mut super::super::Storage::FileSystem::WIN32_FIND_DATAA>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpFindFirstFileA(hconnect : *const core::ffi::c_void, lpszsearchfile : windows_core::PCSTR, lpfindfiledata : *mut super::super::Storage::FileSystem:: WIN32_FIND_DATAA, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { FtpFindFirstFileA(hconnect, lpszsearchfile.param().abi(), lpfindfiledata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn FtpFindFirstFileW<P1>(hconnect: *const core::ffi::c_void, lpszsearchfile: P1, lpfindfiledata: Option<*mut super::super::Storage::FileSystem::WIN32_FIND_DATAW>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpFindFirstFileW(hconnect : *const core::ffi::c_void, lpszsearchfile : windows_core::PCWSTR, lpfindfiledata : *mut super::super::Storage::FileSystem:: WIN32_FIND_DATAW, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { FtpFindFirstFileW(hconnect, lpszsearchfile.param().abi(), lpfindfiledata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpGetCurrentDirectoryA(hconnect: *const core::ffi::c_void, lpszcurrentdirectory: windows_core::PSTR, lpdwcurrentdirectory: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryA(hconnect : *const core::ffi::c_void, lpszcurrentdirectory : windows_core::PSTR, lpdwcurrentdirectory : *mut u32) -> windows_core::BOOL);
    unsafe { FtpGetCurrentDirectoryA(hconnect, core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory as _).ok() }
}
#[inline]
pub unsafe fn FtpGetCurrentDirectoryW(hconnect: *const core::ffi::c_void, lpszcurrentdirectory: windows_core::PWSTR, lpdwcurrentdirectory: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn FtpGetCurrentDirectoryW(hconnect : *const core::ffi::c_void, lpszcurrentdirectory : windows_core::PWSTR, lpdwcurrentdirectory : *mut u32) -> windows_core::BOOL);
    unsafe { FtpGetCurrentDirectoryW(hconnect, core::mem::transmute(lpszcurrentdirectory), lpdwcurrentdirectory as _).ok() }
}
#[inline]
pub unsafe fn FtpGetFileA<P1, P2>(hconnect: *const core::ffi::c_void, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileA(hconnect : *const core::ffi::c_void, lpszremotefile : windows_core::PCSTR, lpsznewfile : windows_core::PCSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileA(hconnect, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpGetFileEx<P1, P2>(hftpsession: *const core::ffi::c_void, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileEx(hftpsession : *const core::ffi::c_void, lpszremotefile : windows_core::PCSTR, lpsznewfile : windows_core::PCWSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileEx(hftpsession, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpGetFileSize(hfile: *const core::ffi::c_void, lpdwfilesizehigh: Option<*mut u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn FtpGetFileSize(hfile : *const core::ffi::c_void, lpdwfilesizehigh : *mut u32) -> u32);
    unsafe { FtpGetFileSize(hfile, lpdwfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpGetFileW<P1, P2>(hconnect: *const core::ffi::c_void, lpszremotefile: P1, lpsznewfile: P2, ffailifexists: bool, dwflagsandattributes: u32, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpGetFileW(hconnect : *const core::ffi::c_void, lpszremotefile : windows_core::PCWSTR, lpsznewfile : windows_core::PCWSTR, ffailifexists : windows_core::BOOL, dwflagsandattributes : u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpGetFileW(hconnect, lpszremotefile.param().abi(), lpsznewfile.param().abi(), ffailifexists.into(), dwflagsandattributes, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpOpenFileA<P1>(hconnect: *const core::ffi::c_void, lpszfilename: P1, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpOpenFileA(hconnect : *const core::ffi::c_void, lpszfilename : windows_core::PCSTR, dwaccess : u32, dwflags : FTP_FLAGS, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { FtpOpenFileA(hconnect, lpszfilename.param().abi(), dwaccess, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpOpenFileW<P1>(hconnect: *const core::ffi::c_void, lpszfilename: P1, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpOpenFileW(hconnect : *const core::ffi::c_void, lpszfilename : windows_core::PCWSTR, dwaccess : u32, dwflags : FTP_FLAGS, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { FtpOpenFileW(hconnect, lpszfilename.param().abi(), dwaccess, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpPutFileA<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: FTP_FLAGS, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileA(hconnect : *const core::ffi::c_void, lpszlocalfile : windows_core::PCSTR, lpsznewremotefile : windows_core::PCSTR, dwflags : FTP_FLAGS, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileA(hconnect, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpPutFileEx<P1, P2>(hftpsession: *const core::ffi::c_void, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileEx(hftpsession : *const core::ffi::c_void, lpszlocalfile : windows_core::PCWSTR, lpsznewremotefile : windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileEx(hftpsession, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FtpPutFileW<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocalfile: P1, lpsznewremotefile: P2, dwflags: FTP_FLAGS, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpPutFileW(hconnect : *const core::ffi::c_void, lpszlocalfile : windows_core::PCWSTR, lpsznewremotefile : windows_core::PCWSTR, dwflags : FTP_FLAGS, dwcontext : usize) -> windows_core::BOOL);
    unsafe { FtpPutFileW(hconnect, lpszlocalfile.param().abi(), lpsznewremotefile.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn FtpRemoveDirectoryA<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRemoveDirectoryA(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpRemoveDirectoryA(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpRemoveDirectoryW<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRemoveDirectoryW(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpRemoveDirectoryW(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpRenameFileA<P1, P2>(hconnect: *const core::ffi::c_void, lpszexisting: P1, lpsznew: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRenameFileA(hconnect : *const core::ffi::c_void, lpszexisting : windows_core::PCSTR, lpsznew : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpRenameFileA(hconnect, lpszexisting.param().abi(), lpsznew.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpRenameFileW<P1, P2>(hconnect: *const core::ffi::c_void, lpszexisting: P1, lpsznew: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpRenameFileW(hconnect : *const core::ffi::c_void, lpszexisting : windows_core::PCWSTR, lpsznew : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpRenameFileW(hconnect, lpszexisting.param().abi(), lpsznew.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpSetCurrentDirectoryA<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryA(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { FtpSetCurrentDirectoryA(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn FtpSetCurrentDirectoryW<P1>(hconnect: *const core::ffi::c_void, lpszdirectory: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn FtpSetCurrentDirectoryW(hconnect : *const core::ffi::c_void, lpszdirectory : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FtpSetCurrentDirectoryW(hconnect, lpszdirectory.param().abi()).ok() }
}
#[inline]
pub unsafe fn GetDiskInfoA<P0>(pszpath: P0, pdwclustersize: Option<*mut u32>, pdlavail: Option<*mut u64>, pdltotal: Option<*mut u64>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetDiskInfoA(pszpath : windows_core::PCSTR, pdwclustersize : *mut u32, pdlavail : *mut u64, pdltotal : *mut u64) -> windows_core::BOOL);
    unsafe { GetDiskInfoA(pszpath.param().abi(), pdwclustersize.unwrap_or(core::mem::zeroed()) as _, pdlavail.unwrap_or(core::mem::zeroed()) as _, pdltotal.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetUrlCacheConfigInfoA(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo: Option<*const u32>, dwfieldcontrol: CACHE_CONFIG) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheConfigInfoA(lpcacheconfiginfo : *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo : *const u32, dwfieldcontrol : CACHE_CONFIG) -> windows_core::BOOL);
    unsafe { GetUrlCacheConfigInfoA(lpcacheconfiginfo as _, lpcbcacheconfiginfo.unwrap_or(core::mem::zeroed()) as _, dwfieldcontrol).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheConfigInfoW(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo: Option<*const u32>, dwfieldcontrol: CACHE_CONFIG) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheConfigInfoW(lpcacheconfiginfo : *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo : *const u32, dwfieldcontrol : CACHE_CONFIG) -> windows_core::BOOL);
    unsafe { GetUrlCacheConfigInfoW(lpcacheconfiginfo as _, lpcbcacheconfiginfo.unwrap_or(core::mem::zeroed()) as _, dwfieldcontrol).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheEntryBinaryBlob<P0>(pwszurlname: P0, dwtype: *mut u32, pftexpiretime: *mut super::super::Foundation::FILETIME, pftaccesstime: *mut super::super::Foundation::FILETIME, pftmodifiedtime: *mut super::super::Foundation::FILETIME, ppbblob: *mut *mut u8, pcbblob: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryBinaryBlob(pwszurlname : windows_core::PCWSTR, dwtype : *mut u32, pftexpiretime : *mut super::super::Foundation:: FILETIME, pftaccesstime : *mut super::super::Foundation:: FILETIME, pftmodifiedtime : *mut super::super::Foundation:: FILETIME, ppbblob : *mut *mut u8, pcbblob : *mut u32) -> u32);
    unsafe { GetUrlCacheEntryBinaryBlob(pwszurlname.param().abi(), dwtype as _, pftexpiretime as _, pftaccesstime as _, pftmodifiedtime as _, ppbblob as _, pcbblob as _) }
}
#[inline]
pub unsafe fn GetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExA<P0, P3>(lpszurl: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: Option<*mut u32>, lpszredirecturl: P3, lpcbredirecturl: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExA(lpszurl : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_core::PCSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoExA(lpszurl.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpszredirecturl.param().abi(), lpcbredirecturl.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExW<P0, P3>(lpszurl: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: Option<*mut u32>, lpszredirecturl: P3, lpcbredirecturl: Option<*const u32>, lpreserved: Option<*const core::ffi::c_void>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoExW(lpszurl : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, lpszredirecturl : windows_core::PCWSTR, lpcbredirecturl : *const u32, lpreserved : *const core::ffi::c_void, dwflags : u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoExW(lpszurl.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpszredirecturl.param().abi(), lpcbredirecturl.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheEntryInfoW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheEntryInfoW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeA(gid: i64, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeA(gid : i64, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetUrlCacheGroupAttributeA(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo as _, lpcbgroupinfo as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeW(gid: i64, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheGroupAttributeW(gid : i64, dwflags : u32, dwattributes : u32, lpgroupinfo : *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetUrlCacheGroupAttributeW(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo as _, lpcbgroupinfo as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn GetUrlCacheHeaderData(nidx : u32, lpdwdata : *mut u32) -> windows_core::BOOL);
    unsafe { GetUrlCacheHeaderData(nidx, lpdwdata as _) }
}
#[inline]
pub unsafe fn GopherCreateLocatorA<P0, P2, P3>(lpszhost: P0, nserverport: u16, lpszdisplaystring: P2, lpszselectorstring: P3, dwgophertype: u32, lpszlocator: Option<windows_core::PSTR>, lpdwbufferlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherCreateLocatorA(lpszhost : windows_core::PCSTR, nserverport : u16, lpszdisplaystring : windows_core::PCSTR, lpszselectorstring : windows_core::PCSTR, dwgophertype : u32, lpszlocator : windows_core::PSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { GopherCreateLocatorA(lpszhost.param().abi(), nserverport, lpszdisplaystring.param().abi(), lpszselectorstring.param().abi(), dwgophertype, lpszlocator.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn GopherCreateLocatorW<P0, P2, P3>(lpszhost: P0, nserverport: u16, lpszdisplaystring: P2, lpszselectorstring: P3, dwgophertype: u32, lpszlocator: Option<windows_core::PWSTR>, lpdwbufferlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherCreateLocatorW(lpszhost : windows_core::PCWSTR, nserverport : u16, lpszdisplaystring : windows_core::PCWSTR, lpszselectorstring : windows_core::PCWSTR, dwgophertype : u32, lpszlocator : windows_core::PWSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { GopherCreateLocatorW(lpszhost.param().abi(), nserverport, lpszdisplaystring.param().abi(), lpszselectorstring.param().abi(), dwgophertype, lpszlocator.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn GopherFindFirstFileA<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszsearchstring: P2, lpfinddata: Option<*mut GOPHER_FIND_DATAA>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherFindFirstFileA(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCSTR, lpszsearchstring : windows_core::PCSTR, lpfinddata : *mut GOPHER_FIND_DATAA, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { GopherFindFirstFileA(hconnect, lpszlocator.param().abi(), lpszsearchstring.param().abi(), lpfinddata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GopherFindFirstFileW<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszsearchstring: P2, lpfinddata: Option<*mut GOPHER_FIND_DATAW>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherFindFirstFileW(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCWSTR, lpszsearchstring : windows_core::PCWSTR, lpfinddata : *mut GOPHER_FIND_DATAW, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { GopherFindFirstFileW(hconnect, lpszlocator.param().abi(), lpszsearchstring.param().abi(), lpfinddata.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GopherGetAttributeA<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszattributename: P2, lpbuffer: &mut [u8], lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetAttributeA(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCSTR, lpszattributename : windows_core::PCSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_core::BOOL);
    unsafe { GopherGetAttributeA(hconnect, lpszlocator.param().abi(), lpszattributename.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpdwcharactersreturned as _, lpfnenumerator, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GopherGetAttributeW<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszattributename: P2, lpbuffer: &mut [u8], lpdwcharactersreturned: *mut u32, lpfnenumerator: GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext: Option<usize>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetAttributeW(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCWSTR, lpszattributename : windows_core::PCWSTR, lpbuffer : *mut u8, dwbufferlength : u32, lpdwcharactersreturned : *mut u32, lpfnenumerator : GOPHER_ATTRIBUTE_ENUMERATOR, dwcontext : usize) -> windows_core::BOOL);
    unsafe { GopherGetAttributeW(hconnect, lpszlocator.param().abi(), lpszattributename.param().abi(), core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpdwcharactersreturned as _, lpfnenumerator, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GopherGetLocatorTypeA<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetLocatorTypeA(lpszlocator : windows_core::PCSTR, lpdwgophertype : *mut u32) -> windows_core::BOOL);
    unsafe { GopherGetLocatorTypeA(lpszlocator.param().abi(), lpdwgophertype as _).ok() }
}
#[inline]
pub unsafe fn GopherGetLocatorTypeW<P0>(lpszlocator: P0, lpdwgophertype: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherGetLocatorTypeW(lpszlocator : windows_core::PCWSTR, lpdwgophertype : *mut u32) -> windows_core::BOOL);
    unsafe { GopherGetLocatorTypeW(lpszlocator.param().abi(), lpdwgophertype as _).ok() }
}
#[inline]
pub unsafe fn GopherOpenFileA<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszview: P2, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherOpenFileA(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCSTR, lpszview : windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { GopherOpenFileA(hconnect, lpszlocator.param().abi(), lpszview.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GopherOpenFileW<P1, P2>(hconnect: *const core::ffi::c_void, lpszlocator: P1, lpszview: P2, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn GopherOpenFileW(hconnect : *const core::ffi::c_void, lpszlocator : windows_core::PCWSTR, lpszview : windows_core::PCWSTR, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { GopherOpenFileW(hconnect, lpszlocator.param().abi(), lpszview.param().abi(), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HttpAddRequestHeadersA(hrequest: *const core::ffi::c_void, lpszheaders: &[u8], dwmodifiers: HTTP_ADDREQ_FLAG) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpAddRequestHeadersA(hrequest : *const core::ffi::c_void, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, dwmodifiers : HTTP_ADDREQ_FLAG) -> windows_core::BOOL);
    unsafe { HttpAddRequestHeadersA(hrequest, core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len().try_into().unwrap(), dwmodifiers).ok() }
}
#[inline]
pub unsafe fn HttpAddRequestHeadersW(hrequest: *const core::ffi::c_void, lpszheaders: &[u16], dwmodifiers: HTTP_ADDREQ_FLAG) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpAddRequestHeadersW(hrequest : *const core::ffi::c_void, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, dwmodifiers : HTTP_ADDREQ_FLAG) -> windows_core::BOOL);
    unsafe { HttpAddRequestHeadersW(hrequest, core::mem::transmute(lpszheaders.as_ptr()), lpszheaders.len().try_into().unwrap(), dwmodifiers).ok() }
}
#[inline]
pub unsafe fn HttpCheckDavComplianceA<P0, P1>(lpszurl: P0, lpszcompliancetoken: P1, lpffound: *mut windows_core::BOOL, hwnd: super::super::Foundation::HWND, lpvreserved: *const core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpCheckDavComplianceA(lpszurl : windows_core::PCSTR, lpszcompliancetoken : windows_core::PCSTR, lpffound : *mut windows_core::BOOL, hwnd : super::super::Foundation:: HWND, lpvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HttpCheckDavComplianceA(lpszurl.param().abi(), lpszcompliancetoken.param().abi(), lpffound as _, hwnd, lpvreserved) }
}
#[inline]
pub unsafe fn HttpCheckDavComplianceW<P0, P1>(lpszurl: P0, lpszcompliancetoken: P1, lpffound: *mut windows_core::BOOL, hwnd: super::super::Foundation::HWND, lpvreserved: *const core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpCheckDavComplianceW(lpszurl : windows_core::PCWSTR, lpszcompliancetoken : windows_core::PCWSTR, lpffound : *mut windows_core::BOOL, hwnd : super::super::Foundation:: HWND, lpvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HttpCheckDavComplianceW(lpszurl.param().abi(), lpszcompliancetoken.param().abi(), lpffound as _, hwnd, lpvreserved) }
}
#[inline]
pub unsafe fn HttpCloseDependencyHandle(hdependencyhandle: *const core::ffi::c_void) {
    windows_core::link!("wininet.dll" "system" fn HttpCloseDependencyHandle(hdependencyhandle : *const core::ffi::c_void));
    unsafe { HttpCloseDependencyHandle(hdependencyhandle) }
}
#[inline]
pub unsafe fn HttpDuplicateDependencyHandle(hdependencyhandle: *const core::ffi::c_void, phduplicateddependencyhandle: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("wininet.dll" "system" fn HttpDuplicateDependencyHandle(hdependencyhandle : *const core::ffi::c_void, phduplicateddependencyhandle : *mut *mut core::ffi::c_void) -> u32);
    unsafe { HttpDuplicateDependencyHandle(hdependencyhandle, phduplicateddependencyhandle as _) }
}
#[inline]
pub unsafe fn HttpEndRequestA(hrequest: *const core::ffi::c_void, lpbuffersout: Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpEndRequestA(hrequest : *const core::ffi::c_void, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpEndRequestA(hrequest, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpEndRequestW(hrequest: *const core::ffi::c_void, lpbuffersout: Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpEndRequestW(hrequest : *const core::ffi::c_void, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpEndRequestW(hrequest, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpGetServerCredentials<P0>(pwszurl: P0, ppwszusername: *mut windows_core::PWSTR, ppwszpassword: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpGetServerCredentials(pwszurl : windows_core::PCWSTR, ppwszusername : *mut windows_core::PWSTR, ppwszpassword : *mut windows_core::PWSTR) -> u32);
    unsafe { HttpGetServerCredentials(pwszurl.param().abi(), ppwszusername as _, ppwszpassword as _) }
}
#[inline]
pub unsafe fn HttpIndicatePageLoadComplete(hdependencyhandle: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wininet.dll" "system" fn HttpIndicatePageLoadComplete(hdependencyhandle : *const core::ffi::c_void) -> u32);
    unsafe { HttpIndicatePageLoadComplete(hdependencyhandle) }
}
#[inline]
pub unsafe fn HttpIsHostHstsEnabled<P0>(pcwszurl: P0, pfishsts: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpIsHostHstsEnabled(pcwszurl : windows_core::PCWSTR, pfishsts : *mut windows_core::BOOL) -> u32);
    unsafe { HttpIsHostHstsEnabled(pcwszurl.param().abi(), pfishsts as _) }
}
#[inline]
pub unsafe fn HttpOpenDependencyHandle(hrequesthandle: *const core::ffi::c_void, fbackground: bool, phdependencyhandle: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("wininet.dll" "system" fn HttpOpenDependencyHandle(hrequesthandle : *const core::ffi::c_void, fbackground : windows_core::BOOL, phdependencyhandle : *mut *mut core::ffi::c_void) -> u32);
    unsafe { HttpOpenDependencyHandle(hrequesthandle, fbackground.into(), phdependencyhandle as _) }
}
#[inline]
pub unsafe fn HttpOpenRequestA<P1, P2, P3, P4>(hconnect: *const core::ffi::c_void, lpszverb: P1, lpszobjectname: P2, lpszversion: P3, lpszreferrer: P4, lplpszaccepttypes: Option<*const windows_core::PCSTR>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpOpenRequestA(hconnect : *const core::ffi::c_void, lpszverb : windows_core::PCSTR, lpszobjectname : windows_core::PCSTR, lpszversion : windows_core::PCSTR, lpszreferrer : windows_core::PCSTR, lplpszaccepttypes : *const windows_core::PCSTR, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { HttpOpenRequestA(hconnect, lpszverb.param().abi(), lpszobjectname.param().abi(), lpszversion.param().abi(), lpszreferrer.param().abi(), lplpszaccepttypes.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HttpOpenRequestW<P1, P2, P3, P4>(hconnect: *const core::ffi::c_void, lpszverb: P1, lpszobjectname: P2, lpszversion: P3, lpszreferrer: P4, lplpszaccepttypes: Option<*const windows_core::PCWSTR>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn HttpOpenRequestW(hconnect : *const core::ffi::c_void, lpszverb : windows_core::PCWSTR, lpszobjectname : windows_core::PCWSTR, lpszversion : windows_core::PCWSTR, lpszreferrer : windows_core::PCWSTR, lplpszaccepttypes : *const windows_core::PCWSTR, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { HttpOpenRequestW(hconnect, lpszverb.param().abi(), lpszobjectname.param().abi(), lpszversion.param().abi(), lpszreferrer.param().abi(), lplpszaccepttypes.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HttpPushClose(hwait: HTTP_PUSH_WAIT_HANDLE) {
    windows_core::link!("wininet.dll" "system" fn HttpPushClose(hwait : HTTP_PUSH_WAIT_HANDLE));
    unsafe { HttpPushClose(hwait) }
}
#[inline]
pub unsafe fn HttpPushEnable(hrequest: *const core::ffi::c_void, ptransportsetting: *const HTTP_PUSH_TRANSPORT_SETTING, phwait: *mut HTTP_PUSH_WAIT_HANDLE) -> u32 {
    windows_core::link!("wininet.dll" "system" fn HttpPushEnable(hrequest : *const core::ffi::c_void, ptransportsetting : *const HTTP_PUSH_TRANSPORT_SETTING, phwait : *mut HTTP_PUSH_WAIT_HANDLE) -> u32);
    unsafe { HttpPushEnable(hrequest, ptransportsetting, phwait as _) }
}
#[inline]
pub unsafe fn HttpPushWait(hwait: HTTP_PUSH_WAIT_HANDLE, etype: HTTP_PUSH_WAIT_TYPE, pnotificationstatus: Option<*mut HTTP_PUSH_NOTIFICATION_STATUS>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn HttpPushWait(hwait : HTTP_PUSH_WAIT_HANDLE, etype : HTTP_PUSH_WAIT_TYPE, pnotificationstatus : *mut HTTP_PUSH_NOTIFICATION_STATUS) -> u32);
    unsafe { HttpPushWait(hwait, etype, pnotificationstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HttpQueryInfoA(hrequest: *const core::ffi::c_void, dwinfolevel: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpQueryInfoA(hrequest : *const core::ffi::c_void, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_core::BOOL);
    unsafe { HttpQueryInfoA(hrequest, dwinfolevel, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpQueryInfoW(hrequest: *const core::ffi::c_void, dwinfolevel: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32, lpdwindex: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpQueryInfoW(hrequest : *const core::ffi::c_void, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32, lpdwindex : *mut u32) -> windows_core::BOOL);
    unsafe { HttpQueryInfoW(hrequest, dwinfolevel, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _, lpdwindex.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpSendRequestA(hrequest: *const core::ffi::c_void, lpszheaders: Option<&[u8]>, lpoptional: Option<*const core::ffi::c_void>, dwoptionallength: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestA(hrequest : *const core::ffi::c_void, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_core::BOOL);
    unsafe { HttpSendRequestA(hrequest, core::mem::transmute(lpszheaders.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength).ok() }
}
#[inline]
pub unsafe fn HttpSendRequestExA(hrequest: *const core::ffi::c_void, lpbuffersin: Option<*const INTERNET_BUFFERSA>, lpbuffersout: Option<*mut INTERNET_BUFFERSA>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestExA(hrequest : *const core::ffi::c_void, lpbuffersin : *const INTERNET_BUFFERSA, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpSendRequestExA(hrequest, lpbuffersin.unwrap_or(core::mem::zeroed()) as _, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpSendRequestExW(hrequest: *const core::ffi::c_void, lpbuffersin: Option<*const INTERNET_BUFFERSW>, lpbuffersout: Option<*mut INTERNET_BUFFERSW>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestExW(hrequest : *const core::ffi::c_void, lpbuffersin : *const INTERNET_BUFFERSW, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { HttpSendRequestExW(hrequest, lpbuffersin.unwrap_or(core::mem::zeroed()) as _, lpbuffersout.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HttpSendRequestW(hrequest: *const core::ffi::c_void, lpszheaders: Option<&[u16]>, lpoptional: Option<*const core::ffi::c_void>, dwoptionallength: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn HttpSendRequestW(hrequest : *const core::ffi::c_void, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, lpoptional : *const core::ffi::c_void, dwoptionallength : u32) -> windows_core::BOOL);
    unsafe { HttpSendRequestW(hrequest, core::mem::transmute(lpszheaders.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpoptional.unwrap_or(core::mem::zeroed()) as _, dwoptionallength).ok() }
}
#[inline]
pub unsafe fn HttpWebSocketClose(hwebsocket: *const core::ffi::c_void, usstatus: u16, pvreason: Option<*const core::ffi::c_void>, dwreasonlength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketClose(hwebsocket : *const core::ffi::c_void, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> windows_core::BOOL);
    unsafe { HttpWebSocketClose(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[inline]
pub unsafe fn HttpWebSocketCompleteUpgrade(hrequest: *const core::ffi::c_void, dwcontext: usize) -> *mut core::ffi::c_void {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketCompleteUpgrade(hrequest : *const core::ffi::c_void, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { HttpWebSocketCompleteUpgrade(hrequest, dwcontext) }
}
#[inline]
pub unsafe fn HttpWebSocketQueryCloseStatus(hwebsocket: *const core::ffi::c_void, pusstatus: *mut u16, pvreason: Option<*mut core::ffi::c_void>, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketQueryCloseStatus(hwebsocket : *const core::ffi::c_void, pusstatus : *mut u16, pvreason : *mut core::ffi::c_void, dwreasonlength : u32, pdwreasonlengthconsumed : *mut u32) -> windows_core::BOOL);
    unsafe { HttpWebSocketQueryCloseStatus(hwebsocket, pusstatus as _, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength, pdwreasonlengthconsumed as _) }
}
#[inline]
pub unsafe fn HttpWebSocketReceive(hwebsocket: *const core::ffi::c_void, pvbuffer: *mut core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pbuffertype: *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketReceive(hwebsocket : *const core::ffi::c_void, pvbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbytesread : *mut u32, pbuffertype : *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> windows_core::BOOL);
    unsafe { HttpWebSocketReceive(hwebsocket, pvbuffer as _, dwbufferlength, pdwbytesread as _, pbuffertype as _) }
}
#[inline]
pub unsafe fn HttpWebSocketSend(hwebsocket: *const core::ffi::c_void, buffertype: HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketSend(hwebsocket : *const core::ffi::c_void, buffertype : HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { HttpWebSocketSend(hwebsocket, buffertype, pvbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength) }
}
#[inline]
pub unsafe fn HttpWebSocketShutdown(hwebsocket: *const core::ffi::c_void, usstatus: u16, pvreason: Option<*const core::ffi::c_void>, dwreasonlength: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn HttpWebSocketShutdown(hwebsocket : *const core::ffi::c_void, usstatus : u16, pvreason : *const core::ffi::c_void, dwreasonlength : u32) -> windows_core::BOOL);
    unsafe { HttpWebSocketShutdown(hwebsocket, usstatus, pvreason.unwrap_or(core::mem::zeroed()) as _, dwreasonlength) }
}
#[inline]
pub unsafe fn ImportCookieFileA<P0>(szfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn ImportCookieFileA(szfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImportCookieFileA(szfilename.param().abi()) }
}
#[inline]
pub unsafe fn ImportCookieFileW<P0>(szfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn ImportCookieFileW(szfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImportCookieFileW(szfilename.param().abi()) }
}
#[inline]
pub unsafe fn IncrementUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn IncrementUrlCacheHeaderData(nidx : u32, lpdwdata : *mut u32) -> windows_core::BOOL);
    unsafe { IncrementUrlCacheHeaderData(nidx, lpdwdata as _) }
}
#[inline]
pub unsafe fn InternalInternetGetCookie<P0>(lpszurl: P0, lpszcookiedata: windows_core::PSTR, lpdwdatasize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternalInternetGetCookie(lpszurl : windows_core::PCSTR, lpszcookiedata : windows_core::PSTR, lpdwdatasize : *mut u32) -> u32);
    unsafe { InternalInternetGetCookie(lpszurl.param().abi(), core::mem::transmute(lpszcookiedata), lpdwdatasize as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn InternetAlgIdToStringA(ai: super::super::Security::Cryptography::ALG_ID, lpstr: windows_core::PSTR, lpdwstrlength: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetAlgIdToStringA(ai : super::super::Security::Cryptography:: ALG_ID, lpstr : windows_core::PSTR, lpdwstrlength : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetAlgIdToStringA(ai, core::mem::transmute(lpstr), lpdwstrlength as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn InternetAlgIdToStringW(ai: super::super::Security::Cryptography::ALG_ID, lpstr: windows_core::PWSTR, lpdwstrlength: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetAlgIdToStringW(ai : super::super::Security::Cryptography:: ALG_ID, lpstr : windows_core::PWSTR, lpdwstrlength : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetAlgIdToStringW(ai, core::mem::transmute(lpstr), lpdwstrlength as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetAttemptConnect(dwreserved: u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetAttemptConnect(dwreserved : u32) -> u32);
    unsafe { InternetAttemptConnect(dwreserved) }
}
#[inline]
pub unsafe fn InternetAutodial(dwflags: INTERNET_AUTODIAL, hwndparent: Option<super::super::Foundation::HWND>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetAutodial(dwflags : INTERNET_AUTODIAL, hwndparent : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { InternetAutodial(dwflags, hwndparent.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetAutodialHangup(dwreserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetAutodialHangup(dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetAutodialHangup(dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetCanonicalizeUrlA<P0>(lpszurl: P0, lpszbuffer: windows_core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCanonicalizeUrlA(lpszurl : windows_core::PCSTR, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCanonicalizeUrlA(lpszurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetCanonicalizeUrlW<P0>(lpszurl: P0, lpszbuffer: windows_core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCanonicalizeUrlW(lpszurl : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCanonicalizeUrlW(lpszurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetCheckConnectionA<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCheckConnectionA(lpszurl : windows_core::PCSTR, dwflags : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetCheckConnectionA(lpszurl.param().abi(), dwflags, dwreserved).ok() }
}
#[inline]
pub unsafe fn InternetCheckConnectionW<P0>(lpszurl: P0, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCheckConnectionW(lpszurl : windows_core::PCWSTR, dwflags : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetCheckConnectionW(lpszurl.param().abi(), dwflags, dwreserved).ok() }
}
#[inline]
pub unsafe fn InternetClearAllPerSiteCookieDecisions() -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetClearAllPerSiteCookieDecisions() -> windows_core::BOOL);
    unsafe { InternetClearAllPerSiteCookieDecisions() }
}
#[inline]
pub unsafe fn InternetCloseHandle(hinternet: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetCloseHandle(hinternet : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetCloseHandle(hinternet).ok() }
}
#[inline]
pub unsafe fn InternetCombineUrlA<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: windows_core::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCombineUrlA(lpszbaseurl : windows_core::PCSTR, lpszrelativeurl : windows_core::PCSTR, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCombineUrlA(lpszbaseurl.param().abi(), lpszrelativeurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetCombineUrlW<P0, P1>(lpszbaseurl: P0, lpszrelativeurl: P1, lpszbuffer: windows_core::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetCombineUrlW(lpszbaseurl : windows_core::PCWSTR, lpszrelativeurl : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetCombineUrlW(lpszbaseurl.param().abi(), lpszrelativeurl.param().abi(), core::mem::transmute(lpszbuffer), lpdwbufferlength as _, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetConfirmZoneCrossing<P1, P2>(hwnd: super::super::Foundation::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossing(hwnd : super::super::Foundation:: HWND, szurlprev : windows_core::PCSTR, szurlnew : windows_core::PCSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossing(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[inline]
pub unsafe fn InternetConfirmZoneCrossingA<P1, P2>(hwnd: super::super::Foundation::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingA(hwnd : super::super::Foundation:: HWND, szurlprev : windows_core::PCSTR, szurlnew : windows_core::PCSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossingA(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[inline]
pub unsafe fn InternetConfirmZoneCrossingW<P1, P2>(hwnd: super::super::Foundation::HWND, szurlprev: P1, szurlnew: P2, bpost: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConfirmZoneCrossingW(hwnd : super::super::Foundation:: HWND, szurlprev : windows_core::PCWSTR, szurlnew : windows_core::PCWSTR, bpost : windows_core::BOOL) -> u32);
    unsafe { InternetConfirmZoneCrossingW(hwnd, szurlprev.param().abi(), szurlnew.param().abi(), bpost.into()) }
}
#[inline]
pub unsafe fn InternetConnectA<P1, P3, P4>(hinternet: *const core::ffi::c_void, lpszservername: P1, nserverport: u16, lpszusername: P3, lpszpassword: P4, dwservice: u32, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConnectA(hinternet : *const core::ffi::c_void, lpszservername : windows_core::PCSTR, nserverport : u16, lpszusername : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { InternetConnectA(hinternet, lpszservername.param().abi(), nserverport, lpszusername.param().abi(), lpszpassword.param().abi(), dwservice, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetConnectW<P1, P3, P4>(hinternet: *const core::ffi::c_void, lpszservername: P1, nserverport: u16, lpszusername: P3, lpszpassword: P4, dwservice: u32, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConnectW(hinternet : *const core::ffi::c_void, lpszservername : windows_core::PCWSTR, nserverport : u16, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwservice : u32, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { InternetConnectW(hinternet, lpszservername.param().abi(), nserverport, lpszusername.param().abi(), lpszpassword.param().abi(), dwservice, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetConvertUrlFromWireToWideChar<P2>(pcszurl: &[u8], pcwszbaseurl: P2, dwcodepagehost: u32, dwcodepagepath: u32, fencodepathextra: bool, dwcodepageextra: u32, ppwszconvertedurl: *mut windows_core::PWSTR) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetConvertUrlFromWireToWideChar(pcszurl : windows_core::PCSTR, cchurl : u32, pcwszbaseurl : windows_core::PCWSTR, dwcodepagehost : u32, dwcodepagepath : u32, fencodepathextra : windows_core::BOOL, dwcodepageextra : u32, ppwszconvertedurl : *mut windows_core::PWSTR) -> u32);
    unsafe { InternetConvertUrlFromWireToWideChar(core::mem::transmute(pcszurl.as_ptr()), pcszurl.len().try_into().unwrap(), pcwszbaseurl.param().abi(), dwcodepagehost, dwcodepagepath, fencodepathextra.into(), dwcodepageextra, ppwszconvertedurl as _) }
}
#[cfg(feature = "Win32_Networking_WinHttp")]
#[inline]
pub unsafe fn InternetCrackUrlA(lpszurl: &[u8], dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSA) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetCrackUrlA(lpszurl : windows_core::PCSTR, dwurllength : u32, dwflags : super::WinHttp:: WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents : *mut URL_COMPONENTSA) -> windows_core::BOOL);
    unsafe { InternetCrackUrlA(core::mem::transmute(lpszurl.as_ptr()), lpszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _).ok() }
}
#[cfg(feature = "Win32_Networking_WinHttp")]
#[inline]
pub unsafe fn InternetCrackUrlW(lpszurl: &[u16], dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSW) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetCrackUrlW(lpszurl : windows_core::PCWSTR, dwurllength : u32, dwflags : super::WinHttp:: WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents : *mut URL_COMPONENTSW) -> windows_core::BOOL);
    unsafe { InternetCrackUrlW(core::mem::transmute(lpszurl.as_ptr()), lpszurl.len().try_into().unwrap(), dwflags, lpurlcomponents as _).ok() }
}
#[inline]
pub unsafe fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: Option<windows_core::PSTR>, lpdwurllength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetCreateUrlA(lpurlcomponents : *const URL_COMPONENTSA, dwflags : u32, lpszurl : windows_core::PSTR, lpdwurllength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetCreateUrlA(lpurlcomponents, dwflags, lpszurl.unwrap_or(core::mem::zeroed()) as _, lpdwurllength as _).ok() }
}
#[inline]
pub unsafe fn InternetCreateUrlW(lpurlcomponents: *const URL_COMPONENTSW, dwflags: u32, lpszurl: Option<windows_core::PWSTR>, lpdwurllength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetCreateUrlW(lpurlcomponents : *const URL_COMPONENTSW, dwflags : u32, lpszurl : windows_core::PWSTR, lpdwurllength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetCreateUrlW(lpurlcomponents, dwflags, lpszurl.unwrap_or(core::mem::zeroed()) as _, lpdwurllength as _).ok() }
}
#[inline]
pub unsafe fn InternetDial<P1>(hwndparent: super::super::Foundation::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDial(hwndparent : super::super::Foundation:: HWND, lpszconnectoid : windows_core::PCSTR, dwflags : u32, lpdwconnection : *mut u32, dwreserved : u32) -> u32);
    unsafe { InternetDial(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved) }
}
#[inline]
pub unsafe fn InternetDialA<P1>(hwndparent: super::super::Foundation::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: Option<u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDialA(hwndparent : super::super::Foundation:: HWND, lpszconnectoid : windows_core::PCSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
    unsafe { InternetDialA(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetDialW<P1>(hwndparent: super::super::Foundation::HWND, lpszconnectoid: P1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: Option<u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetDialW(hwndparent : super::super::Foundation:: HWND, lpszconnectoid : windows_core::PCWSTR, dwflags : u32, lpdwconnection : *mut usize, dwreserved : u32) -> u32);
    unsafe { InternetDialW(hwndparent, lpszconnectoid.param().abi(), dwflags, lpdwconnection as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionA(pszsitename: windows_core::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionA(pszsitename : windows_core::PSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_core::BOOL);
    unsafe { InternetEnumPerSiteCookieDecisionA(core::mem::transmute(pszsitename), pcsitenamesize as _, pdwdecision as _, dwindex) }
}
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionW(pszsitename: windows_core::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetEnumPerSiteCookieDecisionW(pszsitename : windows_core::PWSTR, pcsitenamesize : *mut u32, pdwdecision : *mut u32, dwindex : u32) -> windows_core::BOOL);
    unsafe { InternetEnumPerSiteCookieDecisionW(core::mem::transmute(pszsitename), pcsitenamesize as _, pdwdecision as _, dwindex) }
}
#[inline]
pub unsafe fn InternetErrorDlg(hwnd: super::super::Foundation::HWND, hrequest: Option<*mut core::ffi::c_void>, dwerror: u32, dwflags: u32, lppvdata: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetErrorDlg(hwnd : super::super::Foundation:: HWND, hrequest : *mut core::ffi::c_void, dwerror : u32, dwflags : u32, lppvdata : *mut *mut core::ffi::c_void) -> u32);
    unsafe { InternetErrorDlg(hwnd, hrequest.unwrap_or(core::mem::zeroed()) as _, dwerror, dwflags, lppvdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetFindNextFileA(hfind: *const core::ffi::c_void, lpvfinddata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetFindNextFileA(hfind : *const core::ffi::c_void, lpvfinddata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetFindNextFileA(hfind, lpvfinddata as _).ok() }
}
#[inline]
pub unsafe fn InternetFindNextFileW(hfind: *const core::ffi::c_void, lpvfinddata: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetFindNextFileW(hfind : *const core::ffi::c_void, lpvfinddata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetFindNextFileW(hfind, lpvfinddata as _).ok() }
}
#[inline]
pub unsafe fn InternetFortezzaCommand(dwcommand: u32, hwnd: super::super::Foundation::HWND, dwreserved: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetFortezzaCommand(dwcommand : u32, hwnd : super::super::Foundation:: HWND, dwreserved : usize) -> windows_core::BOOL);
    unsafe { InternetFortezzaCommand(dwcommand, hwnd, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetFreeCookies(pcookies: Option<*mut INTERNET_COOKIE2>, dwcookiecount: u32) {
    windows_core::link!("wininet.dll" "system" fn InternetFreeCookies(pcookies : *mut INTERNET_COOKIE2, dwcookiecount : u32));
    unsafe { InternetFreeCookies(pcookies.unwrap_or(core::mem::zeroed()) as _, dwcookiecount) }
}
#[inline]
pub unsafe fn InternetFreeProxyInfoList(pproxyinfolist: *mut WININET_PROXY_INFO_LIST) {
    windows_core::link!("wininet.dll" "system" fn InternetFreeProxyInfoList(pproxyinfolist : *mut WININET_PROXY_INFO_LIST));
    unsafe { InternetFreeProxyInfoList(pproxyinfolist as _) }
}
#[inline]
pub unsafe fn InternetGetConnectedState(lpdwflags: *mut INTERNET_CONNECTION, dwreserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedState(lpdwflags : *mut INTERNET_CONNECTION, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedState(lpdwflags as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetGetConnectedStateEx(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: Option<&mut [u8]>, dwreserved: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateEx(lpdwflags : *mut INTERNET_CONNECTION, lpszconnectionname : windows_core::PSTR, dwnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateEx(lpdwflags as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved).ok() }
}
#[inline]
pub unsafe fn InternetGetConnectedStateExA(lpdwflags: Option<*mut INTERNET_CONNECTION>, lpszconnectionname: Option<&mut [u8]>, dwreserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateExA(lpdwflags : *mut INTERNET_CONNECTION, lpszconnectionname : windows_core::PSTR, cchnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateExA(lpdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetGetConnectedStateExW(lpdwflags: Option<*mut INTERNET_CONNECTION>, lpszconnectionname: Option<&mut [u16]>, dwreserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetConnectedStateExW(lpdwflags : *mut INTERNET_CONNECTION, lpszconnectionname : windows_core::PWSTR, cchnamelen : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetGetConnectedStateExW(lpdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpszconnectionname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszconnectionname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetGetCookieA<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: Option<windows_core::PSTR>, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetCookieA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _).ok() }
}
#[inline]
pub unsafe fn InternetGetCookieEx2<P0, P1>(pcwszurl: P0, pcwszcookiename: P1, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieEx2(pcwszurl : windows_core::PCWSTR, pcwszcookiename : windows_core::PCWSTR, dwflags : u32, ppcookies : *mut *mut INTERNET_COOKIE2, pdwcookiecount : *mut u32) -> u32);
    unsafe { InternetGetCookieEx2(pcwszurl.param().abi(), pcwszcookiename.param().abi(), dwflags, ppcookies as _, pdwcookiecount as _) }
}
#[inline]
pub unsafe fn InternetGetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieExA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR, lpdwsize : *mut u32, dwflags : INTERNET_COOKIE_FLAGS, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetGetCookieExA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), lpdwsize as _, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetGetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieExW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, lpdwsize : *mut u32, dwflags : INTERNET_COOKIE_FLAGS, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InternetGetCookieExW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), lpdwsize as _, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetGetCookieW<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: Option<windows_core::PWSTR>, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetCookieW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PWSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetCookieW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _).ok() }
}
#[inline]
pub unsafe fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: Option<windows_core::PSTR>, lpdwbufferlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetLastResponseInfoA(lpdwerror : *mut u32, lpszbuffer : windows_core::PSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetLastResponseInfoA(lpdwerror as _, lpszbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: Option<windows_core::PWSTR>, lpdwbufferlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetGetLastResponseInfoW(lpdwerror : *mut u32, lpszbuffer : windows_core::PWSTR, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetLastResponseInfoW(lpdwerror as _, lpszbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionA<P0>(pchhostname: P0, presult: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionA(pchhostname : windows_core::PCSTR, presult : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetPerSiteCookieDecisionA(pchhostname.param().abi(), presult as _) }
}
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionW<P0>(pchhostname: P0, presult: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetPerSiteCookieDecisionW(pchhostname : windows_core::PCWSTR, presult : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetPerSiteCookieDecisionW(pchhostname.param().abi(), presult as _) }
}
#[inline]
pub unsafe fn InternetGetProxyForUrl<P1>(hinternet: *const core::ffi::c_void, pcwszurl: P1, pproxyinfolist: *mut WININET_PROXY_INFO_LIST) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetProxyForUrl(hinternet : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pproxyinfolist : *mut WININET_PROXY_INFO_LIST) -> u32);
    unsafe { InternetGetProxyForUrl(hinternet, pcwszurl.param().abi(), pproxyinfolist as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURL<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetSecurityInfoByURL(lpszurl : windows_core::PCSTR, ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT, pdwsecureflags : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetSecurityInfoByURL(lpszurl.param().abi(), ppcertchain as _, pdwsecureflags as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLA<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetSecurityInfoByURLA(lpszurl : windows_core::PCSTR, ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT, pdwsecureflags : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetSecurityInfoByURLA(lpszurl.param().abi(), ppcertchain as _, pdwsecureflags as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLW<P0>(lpszurl: P0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGetSecurityInfoByURLW(lpszurl : windows_core::PCWSTR, ppcertchain : *mut *mut super::super::Security::Cryptography:: CERT_CHAIN_CONTEXT, pdwsecureflags : *mut u32) -> windows_core::BOOL);
    unsafe { InternetGetSecurityInfoByURLW(lpszurl.param().abi(), ppcertchain as _, pdwsecureflags as _) }
}
#[inline]
pub unsafe fn InternetGoOnline<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnline(lpszurl : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnline(lpszurl.param().abi(), hwndparent, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetGoOnlineA<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnlineA(lpszurl : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnlineA(lpszurl.param().abi(), hwndparent, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetGoOnlineW<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetGoOnlineW(lpszurl : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetGoOnlineW(lpszurl.param().abi(), hwndparent, dwflags).ok() }
}
#[inline]
pub unsafe fn InternetHangUp(dwconnection: usize, dwreserved: Option<u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetHangUp(dwconnection : usize, dwreserved : u32) -> u32);
    unsafe { InternetHangUp(dwconnection, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetInitializeAutoProxyDll(dwreserved: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetInitializeAutoProxyDll(dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetInitializeAutoProxyDll(dwreserved).ok() }
}
#[inline]
pub unsafe fn InternetLockRequestFile(hinternet: *const core::ffi::c_void, lphlockrequestinfo: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetLockRequestFile(hinternet : *const core::ffi::c_void, lphlockrequestinfo : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { InternetLockRequestFile(hinternet, lphlockrequestinfo as _).ok() }
}
#[inline]
pub unsafe fn InternetOpenA<P0, P2, P3>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P2, lpszproxybypass: P3, dwflags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenA(lpszagent : windows_core::PCSTR, dwaccesstype : u32, lpszproxy : windows_core::PCSTR, lpszproxybypass : windows_core::PCSTR, dwflags : u32) -> *mut core::ffi::c_void);
    unsafe { InternetOpenA(lpszagent.param().abi(), dwaccesstype, lpszproxy.param().abi(), lpszproxybypass.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn InternetOpenUrlA<P1>(hinternet: *const core::ffi::c_void, lpszurl: P1, lpszheaders: Option<&[u8]>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenUrlA(hinternet : *const core::ffi::c_void, lpszurl : windows_core::PCSTR, lpszheaders : windows_core::PCSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { InternetOpenUrlA(hinternet, lpszurl.param().abi(), core::mem::transmute(lpszheaders.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetOpenUrlW<P1>(hinternet: *const core::ffi::c_void, lpszurl: P1, lpszheaders: Option<&[u16]>, dwflags: u32, dwcontext: Option<usize>) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenUrlW(hinternet : *const core::ffi::c_void, lpszurl : windows_core::PCWSTR, lpszheaders : windows_core::PCWSTR, dwheaderslength : u32, dwflags : u32, dwcontext : usize) -> *mut core::ffi::c_void);
    unsafe { InternetOpenUrlW(hinternet, lpszurl.param().abi(), core::mem::transmute(lpszheaders.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetOpenW<P0, P2, P3>(lpszagent: P0, dwaccesstype: u32, lpszproxy: P2, lpszproxybypass: P3, dwflags: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetOpenW(lpszagent : windows_core::PCWSTR, dwaccesstype : u32, lpszproxy : windows_core::PCWSTR, lpszproxybypass : windows_core::PCWSTR, dwflags : u32) -> *mut core::ffi::c_void);
    unsafe { InternetOpenW(lpszagent.param().abi(), dwaccesstype, lpszproxy.param().abi(), lpszproxybypass.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn InternetQueryDataAvailable(hfile: *const core::ffi::c_void, lpdwnumberofbytesavailable: Option<*mut u32>, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetQueryDataAvailable(hfile : *const core::ffi::c_void, lpdwnumberofbytesavailable : *mut u32, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetQueryDataAvailable(hfile, lpdwnumberofbytesavailable.unwrap_or(core::mem::zeroed()) as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetQueryFortezzaStatus(pdwstatus: *mut u32, dwreserved: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetQueryFortezzaStatus(pdwstatus : *mut u32, dwreserved : usize) -> windows_core::BOOL);
    unsafe { InternetQueryFortezzaStatus(pdwstatus as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetQueryOptionA(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetQueryOptionA(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetQueryOptionA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn InternetQueryOptionW(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*mut core::ffi::c_void>, lpdwbufferlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetQueryOptionW(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *mut core::ffi::c_void, lpdwbufferlength : *mut u32) -> windows_core::BOOL);
    unsafe { InternetQueryOptionW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _).ok() }
}
#[inline]
pub unsafe fn InternetReadFile(hfile: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetReadFile(hfile : *const core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, dwnumberofbytestoread : u32, lpdwnumberofbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { InternetReadFile(hfile, lpbuffer as _, dwnumberofbytestoread, lpdwnumberofbytesread as _).ok() }
}
#[inline]
pub unsafe fn InternetReadFileExA(hfile: *const core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetReadFileExA(hfile : *const core::ffi::c_void, lpbuffersout : *mut INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetReadFileExA(hfile, lpbuffersout as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetReadFileExW(hfile: *const core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: Option<usize>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetReadFileExW(hfile : *const core::ffi::c_void, lpbuffersout : *mut INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetReadFileExW(hfile, lpbuffersout as _, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetSecurityProtocolToStringA(dwprotocol: u32, lpstr: Option<windows_core::PSTR>, lpdwstrlength: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSecurityProtocolToStringA(dwprotocol : u32, lpstr : windows_core::PSTR, lpdwstrlength : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSecurityProtocolToStringA(dwprotocol, lpstr.unwrap_or(core::mem::zeroed()) as _, lpdwstrlength as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSecurityProtocolToStringW(dwprotocol: u32, lpstr: Option<windows_core::PWSTR>, lpdwstrlength: *mut u32, dwreserved: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSecurityProtocolToStringW(dwprotocol : u32, lpstr : windows_core::PWSTR, lpdwstrlength : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSecurityProtocolToStringW(dwprotocol, lpstr.unwrap_or(core::mem::zeroed()) as _, lpdwstrlength as _, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { InternetSetCookieA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi()).ok() }
}
#[inline]
pub unsafe fn InternetSetCookieEx2<P0, P2>(pcwszurl: P0, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: P2, dwflags: u32, pdwcookiestate: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieEx2(pcwszurl : windows_core::PCWSTR, pcookie : *const INTERNET_COOKIE2, pcwszp3ppolicy : windows_core::PCWSTR, dwflags : u32, pdwcookiestate : *mut u32) -> u32);
    unsafe { InternetSetCookieEx2(pcwszurl.param().abi(), pcookie, pcwszp3ppolicy.param().abi(), dwflags, pdwcookiestate as _) }
}
#[inline]
pub unsafe fn InternetSetCookieExA<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: Option<usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieExA(lpszurl : windows_core::PCSTR, lpszcookiename : windows_core::PCSTR, lpszcookiedata : windows_core::PCSTR, dwflags : u32, dwreserved : usize) -> u32);
    unsafe { InternetSetCookieExA(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieExW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32, dwreserved: Option<usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieExW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, dwflags : u32, dwreserved : usize) -> u32);
    unsafe { InternetSetCookieExW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetCookieW<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetCookieW(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { InternetSetCookieW(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi()).ok() }
}
#[inline]
pub unsafe fn InternetSetDialState<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialState(lpszconnectoid : windows_core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialState(lpszconnectoid.param().abi(), dwstate, dwreserved) }
}
#[inline]
pub unsafe fn InternetSetDialStateA<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialStateA(lpszconnectoid : windows_core::PCSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialStateA(lpszconnectoid.param().abi(), dwstate, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetDialStateW<P0>(lpszconnectoid: P0, dwstate: u32, dwreserved: Option<u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetDialStateW(lpszconnectoid : windows_core::PCWSTR, dwstate : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetSetDialStateW(lpszconnectoid.param().abi(), dwstate, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetFilePointer(hfile: *const core::ffi::c_void, ldistancetomove: i32, lpdistancetomovehigh: Option<*mut i32>, dwmovemethod: u32, dwcontext: Option<usize>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn InternetSetFilePointer(hfile : *const core::ffi::c_void, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : u32, dwcontext : usize) -> u32);
    unsafe { InternetSetFilePointer(hfile, ldistancetomove, lpdistancetomovehigh.unwrap_or(core::mem::zeroed()) as _, dwmovemethod, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetSetOptionA(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionA(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength).ok() }
}
#[inline]
pub unsafe fn InternetSetOptionExA(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionExA(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionExA(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, dwflags) }
}
#[inline]
pub unsafe fn InternetSetOptionExW(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionExW(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionExW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength, dwflags) }
}
#[inline]
pub unsafe fn InternetSetOptionW(hinternet: Option<*const core::ffi::c_void>, dwoption: u32, lpbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetSetOptionW(hinternet : *const core::ffi::c_void, dwoption : u32, lpbuffer : *const core::ffi::c_void, dwbufferlength : u32) -> windows_core::BOOL);
    unsafe { InternetSetOptionW(hinternet.unwrap_or(core::mem::zeroed()) as _, dwoption, lpbuffer.unwrap_or(core::mem::zeroed()) as _, dwbufferlength).ok() }
}
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionA<P0>(pchhostname: P0, dwdecision: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionA(pchhostname : windows_core::PCSTR, dwdecision : u32) -> windows_core::BOOL);
    unsafe { InternetSetPerSiteCookieDecisionA(pchhostname.param().abi(), dwdecision) }
}
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionW<P0>(pchhostname: P0, dwdecision: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetSetPerSiteCookieDecisionW(pchhostname : windows_core::PCWSTR, dwdecision : u32) -> windows_core::BOOL);
    unsafe { InternetSetPerSiteCookieDecisionW(pchhostname.param().abi(), dwdecision) }
}
#[inline]
pub unsafe fn InternetSetStatusCallback(hinternet: *const core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallback(hinternet : *const core::ffi::c_void, lpfninternetcallback : LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallback(hinternet, lpfninternetcallback) }
}
#[inline]
pub unsafe fn InternetSetStatusCallbackA(hinternet: *const core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallbackA(hinternet : *const core::ffi::c_void, lpfninternetcallback : LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallbackA(hinternet, lpfninternetcallback) }
}
#[inline]
pub unsafe fn InternetSetStatusCallbackW(hinternet: *const core::ffi::c_void, lpfninternetcallback: LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK {
    windows_core::link!("wininet.dll" "system" fn InternetSetStatusCallbackW(hinternet : *const core::ffi::c_void, lpfninternetcallback : LPINTERNET_STATUS_CALLBACK) -> LPINTERNET_STATUS_CALLBACK);
    unsafe { InternetSetStatusCallbackW(hinternet, lpfninternetcallback) }
}
#[inline]
pub unsafe fn InternetShowSecurityInfoByURL<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetShowSecurityInfoByURL(lpszurl : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { InternetShowSecurityInfoByURL(lpszurl.param().abi(), hwndparent) }
}
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLA<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetShowSecurityInfoByURLA(lpszurl : windows_core::PCSTR, hwndparent : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { InternetShowSecurityInfoByURLA(lpszurl.param().abi(), hwndparent) }
}
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLW<P0>(lpszurl: P0, hwndparent: super::super::Foundation::HWND) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetShowSecurityInfoByURLW(lpszurl : windows_core::PCWSTR, hwndparent : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { InternetShowSecurityInfoByURLW(lpszurl.param().abi(), hwndparent) }
}
#[inline]
pub unsafe fn InternetTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTime(pst : *const super::super::Foundation:: SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTime(pst, dwrfc, core::mem::transmute(lpsztime.as_ptr()), lpsztime.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn InternetTimeFromSystemTimeA(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: &mut [u8]) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeA(pst : *const super::super::Foundation:: SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTimeA(pst, dwrfc, core::mem::transmute(lpsztime.as_ptr()), lpsztime.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn InternetTimeFromSystemTimeW(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: windows_core::PWSTR, cbtime: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetTimeFromSystemTimeW(pst : *const super::super::Foundation:: SYSTEMTIME, dwrfc : u32, lpsztime : windows_core::PWSTR, cbtime : u32) -> windows_core::BOOL);
    unsafe { InternetTimeFromSystemTimeW(pst, dwrfc, core::mem::transmute(lpsztime), cbtime).ok() }
}
#[inline]
pub unsafe fn InternetTimeToSystemTime<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTime(lpsztime : windows_core::PCSTR, pst : *mut super::super::Foundation:: SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTime(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetTimeToSystemTimeA<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTimeA(lpsztime : windows_core::PCSTR, pst : *mut super::super::Foundation:: SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTimeA(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetTimeToSystemTimeW<P0>(lpsztime: P0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn InternetTimeToSystemTimeW(lpsztime : windows_core::PCWSTR, pst : *mut super::super::Foundation:: SYSTEMTIME, dwreserved : u32) -> windows_core::BOOL);
    unsafe { InternetTimeToSystemTimeW(lpsztime.param().abi(), pst as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn InternetUnlockRequestFile(hlockrequestinfo: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetUnlockRequestFile(hlockrequestinfo : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { InternetUnlockRequestFile(hlockrequestinfo as _).ok() }
}
#[inline]
pub unsafe fn InternetWriteFile(hfile: *const core::ffi::c_void, lpbuffer: *const core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn InternetWriteFile(hfile : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, dwnumberofbytestowrite : u32, lpdwnumberofbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { InternetWriteFile(hfile, lpbuffer, dwnumberofbytestowrite, lpdwnumberofbyteswritten as _).ok() }
}
#[inline]
pub unsafe fn InternetWriteFileExA(hfile: *const core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetWriteFileExA(hfile : *const core::ffi::c_void, lpbuffersin : *const INTERNET_BUFFERSA, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetWriteFileExA(hfile, lpbuffersin, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn InternetWriteFileExW(hfile: *const core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, dwflags: u32, dwcontext: Option<usize>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn InternetWriteFileExW(hfile : *const core::ffi::c_void, lpbuffersin : *const INTERNET_BUFFERSW, dwflags : u32, dwcontext : usize) -> windows_core::BOOL);
    unsafe { InternetWriteFileExW(hfile, lpbuffersin, dwflags, dwcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsDomainLegalCookieDomainA<P0, P1>(pchdomain: P0, pchfulldomain: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn IsDomainLegalCookieDomainA(pchdomain : windows_core::PCSTR, pchfulldomain : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { IsDomainLegalCookieDomainA(pchdomain.param().abi(), pchfulldomain.param().abi()) }
}
#[inline]
pub unsafe fn IsDomainLegalCookieDomainW<P0, P1>(pchdomain: P0, pchfulldomain: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn IsDomainLegalCookieDomainW(pchdomain : windows_core::PCWSTR, pchfulldomain : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IsDomainLegalCookieDomainW(pchdomain.param().abi(), pchfulldomain.param().abi()) }
}
#[inline]
pub unsafe fn IsHostInProxyBypassList(tscheme: INTERNET_SCHEME, lpszhost: &[u8]) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn IsHostInProxyBypassList(tscheme : INTERNET_SCHEME, lpszhost : windows_core::PCSTR, cchhost : u32) -> windows_core::BOOL);
    unsafe { IsHostInProxyBypassList(tscheme, core::mem::transmute(lpszhost.as_ptr()), lpszhost.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn IsProfilesEnabled() -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn IsProfilesEnabled() -> windows_core::BOOL);
    unsafe { IsProfilesEnabled() }
}
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredA<P0>(lpszurlname: P0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn IsUrlCacheEntryExpiredA(lpszurlname : windows_core::PCSTR, dwflags : u32, pftlastmodified : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { IsUrlCacheEntryExpiredA(lpszurlname.param().abi(), dwflags, pftlastmodified as _) }
}
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredW<P0>(lpszurlname: P0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn IsUrlCacheEntryExpiredW(lpszurlname : windows_core::PCWSTR, dwflags : u32, pftlastmodified : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { IsUrlCacheEntryExpiredW(lpszurlname.param().abi(), dwflags, pftlastmodified as _) }
}
#[inline]
pub unsafe fn LoadUrlCacheContent() -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn LoadUrlCacheContent() -> windows_core::BOOL);
    unsafe { LoadUrlCacheContent() }
}
#[inline]
pub unsafe fn ParseX509EncodedCertificateForListBoxEntry(lpcert: &[u8], lpszlistboxentry: Option<windows_core::PSTR>, lpdwlistboxentry: *mut u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn ParseX509EncodedCertificateForListBoxEntry(lpcert : *const u8, cbcert : u32, lpszlistboxentry : windows_core::PSTR, lpdwlistboxentry : *mut u32) -> u32);
    unsafe { ParseX509EncodedCertificateForListBoxEntry(core::mem::transmute(lpcert.as_ptr()), lpcert.len().try_into().unwrap(), lpszlistboxentry.unwrap_or(core::mem::zeroed()) as _, lpdwlistboxentry as _) }
}
#[inline]
pub unsafe fn PerformOperationOverUrlCacheA<P0>(pszurlsearchpattern: P0, dwflags: u32, dwfilter: u32, groupid: i64, preserved1: Option<*const core::ffi::c_void>, pdwreserved2: Option<*const u32>, preserved3: Option<*const core::ffi::c_void>, op: CACHE_OPERATOR, poperatordata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn PerformOperationOverUrlCacheA(pszurlsearchpattern : windows_core::PCSTR, dwflags : u32, dwfilter : u32, groupid : i64, preserved1 : *const core::ffi::c_void, pdwreserved2 : *const u32, preserved3 : *const core::ffi::c_void, op : CACHE_OPERATOR, poperatordata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { PerformOperationOverUrlCacheA(pszurlsearchpattern.param().abi(), dwflags, dwfilter, groupid, preserved1.unwrap_or(core::mem::zeroed()) as _, pdwreserved2.unwrap_or(core::mem::zeroed()) as _, preserved3.unwrap_or(core::mem::zeroed()) as _, op, poperatordata as _) }
}
#[inline]
pub unsafe fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: Option<*mut u32>, pszbuffer: Option<windows_core::PWSTR>, pdwbufferlength: Option<*mut u32>) -> u32 {
    windows_core::link!("wininet.dll" "system" fn PrivacyGetZonePreferenceW(dwzone : u32, dwtype : u32, pdwtemplate : *mut u32, pszbuffer : windows_core::PWSTR, pdwbufferlength : *mut u32) -> u32);
    unsafe { PrivacyGetZonePreferenceW(dwzone, dwtype, pdwtemplate.unwrap_or(core::mem::zeroed()) as _, pszbuffer.unwrap_or(core::mem::zeroed()) as _, pdwbufferlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PrivacySetZonePreferenceW<P3>(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn PrivacySetZonePreferenceW(dwzone : u32, dwtype : u32, dwtemplate : u32, pszpreference : windows_core::PCWSTR) -> u32);
    unsafe { PrivacySetZonePreferenceW(dwzone, dwtype, dwtemplate, pszpreference.param().abi()) }
}
#[inline]
pub unsafe fn ReadGuidsForConnectedNetworks(pcnetworks: Option<*mut u32>, pppwsznetworkguids: Option<*mut *mut windows_core::PWSTR>, pppbstrnetworknames: Option<*mut *mut windows_core::BSTR>, pppwszgwmacs: Option<*mut *mut windows_core::PWSTR>, pcgatewaymacs: Option<*mut u32>, pdwflags: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn ReadGuidsForConnectedNetworks(pcnetworks : *mut u32, pppwsznetworkguids : *mut *mut windows_core::PWSTR, pppbstrnetworknames : *mut *mut * mut core::ffi::c_void, pppwszgwmacs : *mut *mut windows_core::PWSTR, pcgatewaymacs : *mut u32, pdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { ReadGuidsForConnectedNetworks(pcnetworks.unwrap_or(core::mem::zeroed()) as _, pppwsznetworkguids.unwrap_or(core::mem::zeroed()) as _, pppbstrnetworknames.unwrap_or(core::mem::zeroed()) as _, pppwszgwmacs.unwrap_or(core::mem::zeroed()) as _, pcgatewaymacs.unwrap_or(core::mem::zeroed()) as _, pdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ReadUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, dwlocation: u32, lpbuffer: *mut core::ffi::c_void, lpdwlen: *mut u32, reserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn ReadUrlCacheEntryStream(hurlcachestream : super::super::Foundation:: HANDLE, dwlocation : u32, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32, reserved : u32) -> windows_core::BOOL);
    unsafe { ReadUrlCacheEntryStream(hurlcachestream, dwlocation, lpbuffer as _, lpdwlen as _, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ReadUrlCacheEntryStreamEx(hurlcachestream: super::super::Foundation::HANDLE, qwlocation: u64, lpbuffer: *mut core::ffi::c_void, lpdwlen: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn ReadUrlCacheEntryStreamEx(hurlcachestream : super::super::Foundation:: HANDLE, qwlocation : u64, lpbuffer : *mut core::ffi::c_void, lpdwlen : *mut u32) -> windows_core::BOOL);
    unsafe { ReadUrlCacheEntryStreamEx(hurlcachestream, qwlocation, lpbuffer as _, lpdwlen as _) }
}
#[inline]
pub unsafe fn RegisterUrlCacheNotification(hwnd: Option<super::super::Foundation::HWND>, umsg: u32, gid: i64, dwopsfilter: u32, dwreserved: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn RegisterUrlCacheNotification(hwnd : super::super::Foundation:: HWND, umsg : u32, gid : i64, dwopsfilter : u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { RegisterUrlCacheNotification(hwnd.unwrap_or(core::mem::zeroed()) as _, umsg, gid, dwopsfilter, dwreserved) }
}
#[inline]
pub unsafe fn ResumeSuspendedDownload(hrequest: *const core::ffi::c_void, dwresultcode: u32) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn ResumeSuspendedDownload(hrequest : *const core::ffi::c_void, dwresultcode : u32) -> windows_core::BOOL);
    unsafe { ResumeSuspendedDownload(hrequest, dwresultcode).ok() }
}
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { RetrieveUrlCacheEntryFileA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryFileW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, dwreserved : u32) -> windows_core::BOOL);
    unsafe { RetrieveUrlCacheEntryFileW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamA<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOA>, lpcbcacheentryinfo: *mut u32, frandomread: bool, dwreserved: Option<u32>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo : *mut u32, frandomread : windows_core::BOOL, dwreserved : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { RetrieveUrlCacheEntryStreamA(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, frandomread.into(), dwreserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamW<P0>(lpszurlname: P0, lpcacheentryinfo: Option<*mut INTERNET_CACHE_ENTRY_INFOW>, lpcbcacheentryinfo: *mut u32, frandomread: bool, dwreserved: Option<u32>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RetrieveUrlCacheEntryStreamW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo : *mut u32, frandomread : windows_core::BOOL, dwreserved : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { RetrieveUrlCacheEntryStreamW(lpszurlname.param().abi(), lpcacheentryinfo.unwrap_or(core::mem::zeroed()) as _, lpcbcacheentryinfo as _, frandomread.into(), dwreserved.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn RunOnceUrlCache<P2>(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: P2, ncmdshow: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn RunOnceUrlCache(hwnd : super::super::Foundation:: HWND, hinst : super::super::Foundation:: HINSTANCE, lpszcmd : windows_core::PCSTR, ncmdshow : i32) -> u32);
    unsafe { RunOnceUrlCache(hwnd, hinst, lpszcmd.param().abi(), ncmdshow) }
}
#[inline]
pub unsafe fn SetUrlCacheConfigInfoA(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheConfigInfoA(lpcacheconfiginfo : *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheConfigInfoA(lpcacheconfiginfo, dwfieldcontrol) }
}
#[inline]
pub unsafe fn SetUrlCacheConfigInfoW(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheConfigInfoW(lpcacheconfiginfo : *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheConfigInfoW(lpcacheconfiginfo, dwfieldcontrol) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroup<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroup(lpszurlname : windows_core::PCSTR, dwflags : u32, groupid : i64, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroup(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroupA<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupA(lpszurlname : windows_core::PCSTR, dwflags : u32, groupid : i64, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroupA(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryGroupW<P0>(lpszurlname: P0, dwflags: u32, groupid: i64, pbgroupattributes: Option<*const u8>, cbgroupattributes: Option<u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryGroupW(lpszurlname : windows_core::PCWSTR, dwflags : u32, groupid : i64, pbgroupattributes : *const u8, cbgroupattributes : u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryGroupW(lpszurlname.param().abi(), dwflags, groupid, pbgroupattributes.unwrap_or(core::mem::zeroed()) as _, cbgroupattributes.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetUrlCacheEntryInfoA<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoA(lpszurlname : windows_core::PCSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryInfoA(lpszurlname.param().abi(), lpcacheentryinfo, dwfieldcontrol).ok() }
}
#[inline]
pub unsafe fn SetUrlCacheEntryInfoW<P0>(lpszurlname: P0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheEntryInfoW(lpszurlname : windows_core::PCWSTR, lpcacheentryinfo : *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheEntryInfoW(lpszurlname.param().abi(), lpcacheentryinfo, dwfieldcontrol).ok() }
}
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeA(gid: i64, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeA(gid : i64, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOA, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheGroupAttributeA(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeW(gid: i64, dwflags: Option<u32>, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheGroupAttributeW(gid : i64, dwflags : u32, dwattributes : u32, lpgroupinfo : *const INTERNET_CACHE_GROUP_INFOW, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetUrlCacheGroupAttributeW(gid, dwflags.unwrap_or(core::mem::zeroed()) as _, dwattributes, lpgroupinfo, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetUrlCacheHeaderData(nidx: u32, dwdata: u32) -> windows_core::BOOL {
    windows_core::link!("wininet.dll" "system" fn SetUrlCacheHeaderData(nidx : u32, dwdata : u32) -> windows_core::BOOL);
    unsafe { SetUrlCacheHeaderData(nidx, dwdata) }
}
#[inline]
pub unsafe fn ShowClientAuthCerts(hwndparent: super::super::Foundation::HWND) -> u32 {
    windows_core::link!("wininet.dll" "system" fn ShowClientAuthCerts(hwndparent : super::super::Foundation:: HWND) -> u32);
    unsafe { ShowClientAuthCerts(hwndparent) }
}
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn ShowSecurityInfo(hwndparent: super::super::Foundation::HWND, psecurityinfo: *const INTERNET_SECURITY_INFO) -> u32 {
    windows_core::link!("wininet.dll" "system" fn ShowSecurityInfo(hwndparent : super::super::Foundation:: HWND, psecurityinfo : *const INTERNET_SECURITY_INFO) -> u32);
    unsafe { ShowSecurityInfo(hwndparent, psecurityinfo) }
}
#[inline]
pub unsafe fn ShowX509EncodedCertificate(hwndparent: super::super::Foundation::HWND, lpcert: &[u8]) -> u32 {
    windows_core::link!("wininet.dll" "system" fn ShowX509EncodedCertificate(hwndparent : super::super::Foundation:: HWND, lpcert : *const u8, cbcert : u32) -> u32);
    unsafe { ShowX509EncodedCertificate(hwndparent, core::mem::transmute(lpcert.as_ptr()), lpcert.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFile<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFile(lpszurlname : windows_core::PCSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFile(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileA<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileA(lpszurlname : windows_core::PCSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFileA(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileW<P0>(lpszurlname: P0, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryFileW(lpszurlname : windows_core::PCWSTR, dwreserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryFileW(lpszurlname.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UnlockUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, reserved: Option<u32>) -> windows_core::Result<()> {
    windows_core::link!("wininet.dll" "system" fn UnlockUrlCacheEntryStream(hurlcachestream : super::super::Foundation:: HANDLE, reserved : u32) -> windows_core::BOOL);
    unsafe { UnlockUrlCacheEntryStream(hurlcachestream, reserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn UpdateUrlCacheContentPath<P0>(sznewpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UpdateUrlCacheContentPath(sznewpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { UpdateUrlCacheContentPath(sznewpath.param().abi()) }
}
#[inline]
pub unsafe fn UrlCacheCheckEntriesExist(rgpwszurls: *const windows_core::PCWSTR, centries: u32, rgfexist: *mut windows_core::BOOL) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheCheckEntriesExist(rgpwszurls : *const windows_core::PCWSTR, centries : u32, rgfexist : *mut windows_core::BOOL) -> u32);
    unsafe { UrlCacheCheckEntriesExist(rgpwszurls, centries, rgfexist as _) }
}
#[inline]
pub unsafe fn UrlCacheCloseEntryHandle(hentryfile: *const core::ffi::c_void) {
    windows_core::link!("wininet.dll" "system" fn UrlCacheCloseEntryHandle(hentryfile : *const core::ffi::c_void));
    unsafe { UrlCacheCloseEntryHandle(hentryfile) }
}
#[inline]
pub unsafe fn UrlCacheContainerSetEntryMaximumAge<P0>(pwszprefix: P0, dwentrymaxage: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheContainerSetEntryMaximumAge(pwszprefix : windows_core::PCWSTR, dwentrymaxage : u32) -> u32);
    unsafe { UrlCacheContainerSetEntryMaximumAge(pwszprefix.param().abi(), dwentrymaxage) }
}
#[inline]
pub unsafe fn UrlCacheCreateContainer<P0, P1, P2>(pwszname: P0, pwszprefix: P1, pwszdirectory: P2, ulllimit: u64, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheCreateContainer(pwszname : windows_core::PCWSTR, pwszprefix : windows_core::PCWSTR, pwszdirectory : windows_core::PCWSTR, ulllimit : u64, dwoptions : u32) -> u32);
    unsafe { UrlCacheCreateContainer(pwszname.param().abi(), pwszprefix.param().abi(), pwszdirectory.param().abi(), ulllimit, dwoptions) }
}
#[inline]
pub unsafe fn UrlCacheFindFirstEntry<P0>(pwszprefix: P0, dwflags: u32, dwfilter: u32, groupid: i64, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phfind: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheFindFirstEntry(pwszprefix : windows_core::PCWSTR, dwflags : u32, dwfilter : u32, groupid : i64, pcacheentryinfo : *mut URLCACHE_ENTRY_INFO, phfind : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { UrlCacheFindFirstEntry(pwszprefix.param().abi(), dwflags, dwfilter, groupid, pcacheentryinfo as _, phfind as _) }
}
#[inline]
pub unsafe fn UrlCacheFindNextEntry(hfind: super::super::Foundation::HANDLE, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheFindNextEntry(hfind : super::super::Foundation:: HANDLE, pcacheentryinfo : *mut URLCACHE_ENTRY_INFO) -> u32);
    unsafe { UrlCacheFindNextEntry(hfind, pcacheentryinfo as _) }
}
#[inline]
pub unsafe fn UrlCacheFreeEntryInfo(pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) {
    windows_core::link!("wininet.dll" "system" fn UrlCacheFreeEntryInfo(pcacheentryinfo : *mut URLCACHE_ENTRY_INFO));
    unsafe { UrlCacheFreeEntryInfo(pcacheentryinfo as _) }
}
#[inline]
pub unsafe fn UrlCacheFreeGlobalSpace(ulltargetsize: u64, dwfilter: u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheFreeGlobalSpace(ulltargetsize : u64, dwfilter : u32) -> u32);
    unsafe { UrlCacheFreeGlobalSpace(ulltargetsize, dwfilter) }
}
#[inline]
pub unsafe fn UrlCacheGetContentPaths(pppwszdirectories: *mut *mut windows_core::PWSTR, pcdirectories: *mut u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheGetContentPaths(pppwszdirectories : *mut *mut windows_core::PWSTR, pcdirectories : *mut u32) -> u32);
    unsafe { UrlCacheGetContentPaths(pppwszdirectories as _, pcdirectories as _) }
}
#[inline]
pub unsafe fn UrlCacheGetEntryInfo<P1>(happcache: Option<*const core::ffi::c_void>, pcwszurl: P1, pcacheentryinfo: Option<*mut URLCACHE_ENTRY_INFO>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheGetEntryInfo(happcache : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pcacheentryinfo : *mut URLCACHE_ENTRY_INFO) -> u32);
    unsafe { UrlCacheGetEntryInfo(happcache.unwrap_or(core::mem::zeroed()) as _, pcwszurl.param().abi(), pcacheentryinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UrlCacheGetGlobalCacheSize(dwfilter: u32, pullsize: *mut u64, pulllimit: *mut u64) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheGetGlobalCacheSize(dwfilter : u32, pullsize : *mut u64, pulllimit : *mut u64) -> u32);
    unsafe { UrlCacheGetGlobalCacheSize(dwfilter, pullsize as _, pulllimit as _) }
}
#[inline]
pub unsafe fn UrlCacheGetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, pulllimit: *mut u64) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheGetGlobalLimit(limittype : URL_CACHE_LIMIT_TYPE, pulllimit : *mut u64) -> u32);
    unsafe { UrlCacheGetGlobalLimit(limittype, pulllimit as _) }
}
#[inline]
pub unsafe fn UrlCacheReadEntryStream(hurlcachestream: *const core::ffi::c_void, ulllocation: u64, pbuffer: *mut core::ffi::c_void, dwbufferlen: u32, pdwbufferlen: *mut u32) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheReadEntryStream(hurlcachestream : *const core::ffi::c_void, ulllocation : u64, pbuffer : *mut core::ffi::c_void, dwbufferlen : u32, pdwbufferlen : *mut u32) -> u32);
    unsafe { UrlCacheReadEntryStream(hurlcachestream, ulllocation, pbuffer as _, dwbufferlen, pdwbufferlen as _) }
}
#[inline]
pub unsafe fn UrlCacheReloadSettings() -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheReloadSettings() -> u32);
    unsafe { UrlCacheReloadSettings() }
}
#[inline]
pub unsafe fn UrlCacheRetrieveEntryFile<P1>(happcache: Option<*const core::ffi::c_void>, pcwszurl: P1, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentryfile: *mut *mut core::ffi::c_void) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheRetrieveEntryFile(happcache : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pcacheentryinfo : *mut URLCACHE_ENTRY_INFO, phentryfile : *mut *mut core::ffi::c_void) -> u32);
    unsafe { UrlCacheRetrieveEntryFile(happcache.unwrap_or(core::mem::zeroed()) as _, pcwszurl.param().abi(), pcacheentryinfo as _, phentryfile as _) }
}
#[inline]
pub unsafe fn UrlCacheRetrieveEntryStream<P1>(happcache: Option<*const core::ffi::c_void>, pcwszurl: P1, frandomread: bool, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentrystream: *mut *mut core::ffi::c_void) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheRetrieveEntryStream(happcache : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, frandomread : windows_core::BOOL, pcacheentryinfo : *mut URLCACHE_ENTRY_INFO, phentrystream : *mut *mut core::ffi::c_void) -> u32);
    unsafe { UrlCacheRetrieveEntryStream(happcache.unwrap_or(core::mem::zeroed()) as _, pcwszurl.param().abi(), frandomread.into(), pcacheentryinfo as _, phentrystream as _) }
}
#[inline]
pub unsafe fn UrlCacheServer() -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheServer() -> u32);
    unsafe { UrlCacheServer() }
}
#[inline]
pub unsafe fn UrlCacheSetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, ulllimit: u64) -> u32 {
    windows_core::link!("wininet.dll" "system" fn UrlCacheSetGlobalLimit(limittype : URL_CACHE_LIMIT_TYPE, ulllimit : u64) -> u32);
    unsafe { UrlCacheSetGlobalLimit(limittype, ulllimit) }
}
#[inline]
pub unsafe fn UrlCacheUpdateEntryExtraData<P1>(happcache: Option<*const core::ffi::c_void>, pcwszurl: P1, pbextradata: &[u8]) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wininet.dll" "system" fn UrlCacheUpdateEntryExtraData(happcache : *const core::ffi::c_void, pcwszurl : windows_core::PCWSTR, pbextradata : *const u8, cbextradata : u32) -> u32);
    unsafe { UrlCacheUpdateEntryExtraData(happcache.unwrap_or(core::mem::zeroed()) as _, pcwszurl.param().abi(), core::mem::transmute(pbextradata.as_ptr()), pbextradata.len().try_into().unwrap()) }
}
pub const ANY_CACHE_ENTRY: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APP_CACHE_DOWNLOAD_ENTRY {
    pub pwszUrl: windows_core::PWSTR,
    pub dwEntryType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APP_CACHE_DOWNLOAD_LIST {
    pub dwEntryCount: u32,
    pub pEntries: *mut APP_CACHE_DOWNLOAD_ENTRY,
}
impl Default for APP_CACHE_DOWNLOAD_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APP_CACHE_ENTRY_TYPE_EXPLICIT: u32 = 2u32;
pub const APP_CACHE_ENTRY_TYPE_FALLBACK: u32 = 4u32;
pub const APP_CACHE_ENTRY_TYPE_FOREIGN: u32 = 8u32;
pub const APP_CACHE_ENTRY_TYPE_MANIFEST: u32 = 16u32;
pub const APP_CACHE_ENTRY_TYPE_MASTER: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APP_CACHE_FINALIZE_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APP_CACHE_GROUP_INFO {
    pub pwszManifestUrl: windows_core::PWSTR,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ullSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APP_CACHE_GROUP_LIST {
    pub dwAppCacheGroupCount: u32,
    pub pAppCacheGroups: *mut APP_CACHE_GROUP_INFO,
}
impl Default for APP_CACHE_GROUP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const APP_CACHE_LOOKUP_NO_MASTER_ONLY: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APP_CACHE_STATE(pub i32);
pub const AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL: u32 = 4u32;
pub const AUTH_FLAG_DISABLE_NEGOTIATE: u32 = 1u32;
pub const AUTH_FLAG_DISABLE_SERVER_AUTH: u32 = 8u32;
pub const AUTH_FLAG_ENABLE_NEGOTIATE: u32 = 2u32;
pub const AUTH_FLAG_RESET: u32 = 0u32;
pub const AUTODIAL_MODE_ALWAYS: u32 = 2u32;
pub const AUTODIAL_MODE_NEVER: u32 = 1u32;
pub const AUTODIAL_MODE_NO_NETWORK_PRESENT: u32 = 4u32;
pub const AUTO_PROXY_FLAG_ALWAYS_DETECT: u32 = 2u32;
pub const AUTO_PROXY_FLAG_CACHE_INIT_RUN: u32 = 32u32;
pub const AUTO_PROXY_FLAG_DETECTION_RUN: u32 = 4u32;
pub const AUTO_PROXY_FLAG_DETECTION_SUSPECT: u32 = 64u32;
pub const AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT: u32 = 16u32;
pub const AUTO_PROXY_FLAG_MIGRATED: u32 = 8u32;
pub const AUTO_PROXY_FLAG_USER_SET: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTO_PROXY_SCRIPT_BUFFER {
    pub dwStructSize: u32,
    pub lpszScriptBuffer: windows_core::PSTR,
    pub dwScriptBufferSize: u32,
}
pub const AppCacheFinalizeStateComplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(2i32);
pub const AppCacheFinalizeStateIncomplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(0i32);
pub const AppCacheFinalizeStateManifestChange: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(1i32);
pub const AppCacheStateNoUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(0i32);
pub const AppCacheStateUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(1i32);
pub const AppCacheStateUpdateNeededMasterOnly: APP_CACHE_STATE = APP_CACHE_STATE(3i32);
pub const AppCacheStateUpdateNeededNew: APP_CACHE_STATE = APP_CACHE_STATE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoProxyHelperFunctions {
    pub lpVtbl: *const AutoProxyHelperVtbl,
}
impl Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AutoProxyHelperVtbl {
    pub IsResolvable: isize,
    pub GetIPAddress: isize,
    pub ResolveHostName: isize,
    pub IsInNet: isize,
    pub IsResolvableEx: isize,
    pub GetIPAddressEx: isize,
    pub ResolveHostNameEx: isize,
    pub IsInNetEx: isize,
    pub SortIpList: isize,
}
pub const CACHEGROUP_ATTRIBUTE_BASIC: u32 = 1u32;
pub const CACHEGROUP_ATTRIBUTE_FLAG: u32 = 2u32;
pub const CACHEGROUP_ATTRIBUTE_GET_ALL: u32 = 4294967295u32;
pub const CACHEGROUP_ATTRIBUTE_GROUPNAME: u32 = 16u32;
pub const CACHEGROUP_ATTRIBUTE_QUOTA: u32 = 8u32;
pub const CACHEGROUP_ATTRIBUTE_STORAGE: u32 = 32u32;
pub const CACHEGROUP_ATTRIBUTE_TYPE: u32 = 4u32;
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: u32 = 2u32;
pub const CACHEGROUP_FLAG_GIDONLY: u32 = 4u32;
pub const CACHEGROUP_FLAG_NONPURGEABLE: u32 = 1u32;
pub const CACHEGROUP_FLAG_VALID: u32 = 7u32;
pub const CACHEGROUP_ID_BUILTIN_STICKY: u64 = 1152921504606846983u64;
pub const CACHEGROUP_SEARCH_ALL: u32 = 0u32;
pub const CACHEGROUP_SEARCH_BYURL: u32 = 1u32;
pub const CACHEGROUP_TYPE_INVALID: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACHE_CONFIG(pub u32);
pub const CACHE_CONFIG_APPCONTAINER_CONTENT_QUOTA_FC: u32 = 131072u32;
pub const CACHE_CONFIG_APPCONTAINER_TOTAL_CONTENT_QUOTA_FC: u32 = 262144u32;
pub const CACHE_CONFIG_CONTENT_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(256u32);
pub const CACHE_CONFIG_CONTENT_QUOTA_FC: u32 = 32768u32;
pub const CACHE_CONFIG_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(8192u32);
pub const CACHE_CONFIG_COOKIES_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(512u32);
pub const CACHE_CONFIG_DISK_CACHE_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(64u32);
pub const CACHE_CONFIG_FORCE_CLEANUP_FC: CACHE_CONFIG = CACHE_CONFIG(32u32);
pub const CACHE_CONFIG_HISTORY_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(1024u32);
pub const CACHE_CONFIG_QUOTA_FC: CACHE_CONFIG = CACHE_CONFIG(2048u32);
pub const CACHE_CONFIG_STICKY_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(16384u32);
pub const CACHE_CONFIG_SYNC_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(128u32);
pub const CACHE_CONFIG_TOTAL_CONTENT_QUOTA_FC: u32 = 65536u32;
pub const CACHE_CONFIG_USER_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(4096u32);
pub const CACHE_ENTRY_ACCTIME_FC: u32 = 256u32;
pub const CACHE_ENTRY_ATTRIBUTE_FC: u32 = 4u32;
pub const CACHE_ENTRY_EXEMPT_DELTA_FC: u32 = 2048u32;
pub const CACHE_ENTRY_EXPTIME_FC: u32 = 128u32;
pub const CACHE_ENTRY_HEADERINFO_FC: u32 = 1024u32;
pub const CACHE_ENTRY_HITRATE_FC: u32 = 16u32;
pub const CACHE_ENTRY_MODIFY_DATA_FC: u32 = 2147483648u32;
pub const CACHE_ENTRY_MODTIME_FC: u32 = 64u32;
pub const CACHE_ENTRY_SYNCTIME_FC: u32 = 512u32;
pub const CACHE_ENTRY_TYPE_FC: u32 = 4096u32;
pub const CACHE_FIND_CONTAINER_RETURN_NOCHANGE: u32 = 1u32;
pub const CACHE_HEADER_DATA_CACHE_READ_COUNT_SINCE_LAST_SCAVENGE: u32 = 9u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_12: u32 = 12u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_13: u32 = 13u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_15: u32 = 15u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_16: u32 = 16u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_17: u32 = 17u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_18: u32 = 18u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_19: u32 = 19u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_20: u32 = 20u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_23: u32 = 23u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_24: u32 = 24u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_25: u32 = 25u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_26: u32 = 26u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_28: u32 = 28u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_29: u32 = 29u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_30: u32 = 30u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_31: u32 = 31u32;
pub const CACHE_HEADER_DATA_CACHE_WRITE_COUNT_SINCE_LAST_SCAVENGE: u32 = 10u32;
pub const CACHE_HEADER_DATA_CONLIST_CHANGE_COUNT: u32 = 1u32;
pub const CACHE_HEADER_DATA_COOKIE_CHANGE_COUNT: u32 = 2u32;
pub const CACHE_HEADER_DATA_CURRENT_SETTINGS_VERSION: u32 = 0u32;
pub const CACHE_HEADER_DATA_DOWNLOAD_PARTIAL: u32 = 14u32;
pub const CACHE_HEADER_DATA_GID_HIGH: u32 = 7u32;
pub const CACHE_HEADER_DATA_GID_LOW: u32 = 6u32;
pub const CACHE_HEADER_DATA_HSTS_CHANGE_COUNT: u32 = 11u32;
pub const CACHE_HEADER_DATA_LAST: u32 = 31u32;
pub const CACHE_HEADER_DATA_LAST_SCAVENGE_TIMESTAMP: u32 = 8u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_FILTER: u32 = 21u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_HWND: u32 = 3u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_MESG: u32 = 4u32;
pub const CACHE_HEADER_DATA_ROOTGROUP_OFFSET: u32 = 5u32;
pub const CACHE_HEADER_DATA_ROOT_GROUPLIST_OFFSET: u32 = 27u32;
pub const CACHE_HEADER_DATA_ROOT_LEAK_OFFSET: u32 = 22u32;
pub const CACHE_HEADER_DATA_SSL_STATE_COUNT: u32 = 14u32;
pub const CACHE_NOTIFY_ADD_URL: u32 = 1u32;
pub const CACHE_NOTIFY_DELETE_ALL: u32 = 8u32;
pub const CACHE_NOTIFY_DELETE_URL: u32 = 2u32;
pub const CACHE_NOTIFY_FILTER_CHANGED: u32 = 268435456u32;
pub const CACHE_NOTIFY_SET_OFFLINE: u32 = 512u32;
pub const CACHE_NOTIFY_SET_ONLINE: u32 = 256u32;
pub const CACHE_NOTIFY_UPDATE_URL: u32 = 4u32;
pub const CACHE_NOTIFY_URL_SET_STICKY: u32 = 16u32;
pub const CACHE_NOTIFY_URL_UNSET_STICKY: u32 = 32u32;
pub type CACHE_OPERATOR = Option<unsafe extern "system" fn(pcei: *mut INTERNET_CACHE_ENTRY_INFOA, pcbcei: *mut u32, popdata: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const COOKIE_ACCEPTED_CACHE_ENTRY: u32 = 4096u32;
pub const COOKIE_ALLOW: u32 = 2u32;
pub const COOKIE_ALLOW_ALL: u32 = 4u32;
pub const COOKIE_CACHE_ENTRY: u32 = 1048576u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COOKIE_DLG_INFO {
    pub pszServer: windows_core::PWSTR,
    pub pic: *mut INTERNET_COOKIE,
    pub dwStopWarning: u32,
    pub cx: i32,
    pub cy: i32,
    pub pszHeader: windows_core::PWSTR,
    pub dwOperation: u32,
}
impl Default for COOKIE_DLG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COOKIE_DONT_ALLOW: u32 = 1u32;
pub const COOKIE_DONT_ALLOW_ALL: u32 = 8u32;
pub const COOKIE_DOWNGRADED_CACHE_ENTRY: u32 = 16384u32;
pub const COOKIE_LEASHED_CACHE_ENTRY: u32 = 8192u32;
pub const COOKIE_OP_3RD_PARTY: u32 = 32u32;
pub const COOKIE_OP_GET: u32 = 4u32;
pub const COOKIE_OP_MODIFY: u32 = 2u32;
pub const COOKIE_OP_PERSISTENT: u32 = 16u32;
pub const COOKIE_OP_SESSION: u32 = 8u32;
pub const COOKIE_OP_SET: u32 = 1u32;
pub const COOKIE_REJECTED_CACHE_ENTRY: u32 = 32768u32;
pub const COOKIE_STATE_ACCEPT: InternetCookieState = InternetCookieState(1i32);
pub const COOKIE_STATE_DOWNGRADE: InternetCookieState = InternetCookieState(4i32);
pub const COOKIE_STATE_LB: u32 = 0u32;
pub const COOKIE_STATE_LEASH: InternetCookieState = InternetCookieState(3i32);
pub const COOKIE_STATE_MAX: InternetCookieState = InternetCookieState(5i32);
pub const COOKIE_STATE_PROMPT: InternetCookieState = InternetCookieState(2i32);
pub const COOKIE_STATE_REJECT: InternetCookieState = InternetCookieState(5i32);
pub const COOKIE_STATE_UB: u32 = 5u32;
pub const COOKIE_STATE_UNKNOWN: InternetCookieState = InternetCookieState(0i32);
pub const ConnectionEstablishmentEnd: REQUEST_TIMES = REQUEST_TIMES(3i32);
pub const ConnectionEstablishmentStart: REQUEST_TIMES = REQUEST_TIMES(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CookieDecision {
    pub dwCookieState: u32,
    pub fAllowSession: windows_core::BOOL,
}
pub const DIALENG_OperationComplete: u32 = 65536u32;
pub const DIALENG_RedialAttempt: u32 = 65537u32;
pub const DIALENG_RedialWait: u32 = 65538u32;
pub const DIALPROP_DOMAIN: windows_core::PCWSTR = windows_core::w!("Domain");
pub const DIALPROP_LASTERROR: windows_core::PCWSTR = windows_core::w!("LastError");
pub const DIALPROP_PASSWORD: windows_core::PCWSTR = windows_core::w!("Password");
pub const DIALPROP_PHONENUMBER: windows_core::PCWSTR = windows_core::w!("PhoneNumber");
pub const DIALPROP_REDIALCOUNT: windows_core::PCWSTR = windows_core::w!("RedialCount");
pub const DIALPROP_REDIALINTERVAL: windows_core::PCWSTR = windows_core::w!("RedialInterval");
pub const DIALPROP_RESOLVEDPHONE: windows_core::PCWSTR = windows_core::w!("ResolvedPhone");
pub const DIALPROP_SAVEPASSWORD: windows_core::PCWSTR = windows_core::w!("SavePassword");
pub const DIALPROP_USERNAME: windows_core::PCWSTR = windows_core::w!("UserName");
pub const DLG_FLAGS_INSECURE_FALLBACK: u32 = 4194304u32;
pub const DLG_FLAGS_INVALID_CA: u32 = 16777216u32;
pub const DLG_FLAGS_SEC_CERT_CN_INVALID: u32 = 33554432u32;
pub const DLG_FLAGS_SEC_CERT_DATE_INVALID: u32 = 67108864u32;
pub const DLG_FLAGS_SEC_CERT_REV_FAILED: u32 = 8388608u32;
pub const DLG_FLAGS_WEAK_SIGNATURE: u32 = 2097152u32;
pub const DOWNLOAD_CACHE_ENTRY: u32 = 1024u32;
pub const DUO_PROTOCOL_FLAG_SPDY3: u32 = 1u32;
pub const DUO_PROTOCOL_MASK: u32 = 1u32;
pub const EDITED_CACHE_ENTRY: u32 = 8u32;
pub const ERROR_FTP_DROPPED: u32 = 12111u32;
pub const ERROR_FTP_NO_PASSIVE_MODE: u32 = 12112u32;
pub const ERROR_FTP_TRANSFER_IN_PROGRESS: u32 = 12110u32;
pub const ERROR_GOPHER_ATTRIBUTE_NOT_FOUND: u32 = 12137u32;
pub const ERROR_GOPHER_DATA_ERROR: u32 = 12132u32;
pub const ERROR_GOPHER_END_OF_DATA: u32 = 12133u32;
pub const ERROR_GOPHER_INCORRECT_LOCATOR_TYPE: u32 = 12135u32;
pub const ERROR_GOPHER_INVALID_LOCATOR: u32 = 12134u32;
pub const ERROR_GOPHER_NOT_FILE: u32 = 12131u32;
pub const ERROR_GOPHER_NOT_GOPHER_PLUS: u32 = 12136u32;
pub const ERROR_GOPHER_PROTOCOL_ERROR: u32 = 12130u32;
pub const ERROR_GOPHER_UNKNOWN_LOCATOR: u32 = 12138u32;
pub const ERROR_HTTP_COOKIE_DECLINED: u32 = 12162u32;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION: u32 = 12161u32;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION_EX: u32 = 12907u32;
pub const ERROR_HTTP_DOWNLEVEL_SERVER: u32 = 12151u32;
pub const ERROR_HTTP_HEADER_ALREADY_EXISTS: u32 = 12155u32;
pub const ERROR_HTTP_HEADER_NOT_FOUND: u32 = 12150u32;
pub const ERROR_HTTP_HSTS_REDIRECT_REQUIRED: u32 = 12060u32;
pub const ERROR_HTTP_INVALID_HEADER: u32 = 12153u32;
pub const ERROR_HTTP_INVALID_QUERY_REQUEST: u32 = 12154u32;
pub const ERROR_HTTP_INVALID_SERVER_RESPONSE: u32 = 12152u32;
pub const ERROR_HTTP_NOT_REDIRECTED: u32 = 12160u32;
pub const ERROR_HTTP_PUSH_ENABLE_FAILED: u32 = 12149u32;
pub const ERROR_HTTP_PUSH_RETRY_NOT_SUPPORTED: u32 = 12148u32;
pub const ERROR_HTTP_PUSH_STATUS_CODE_NOT_SUPPORTED: u32 = 12147u32;
pub const ERROR_HTTP_REDIRECT_FAILED: u32 = 12156u32;
pub const ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION: u32 = 12168u32;
pub const ERROR_INTERNET_ASYNC_THREAD_FAILED: u32 = 12047u32;
pub const ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT: u32 = 12166u32;
pub const ERROR_INTERNET_BAD_OPTION_LENGTH: u32 = 12010u32;
pub const ERROR_INTERNET_BAD_REGISTRY_PARAMETER: u32 = 12022u32;
pub const ERROR_INTERNET_CACHE_SUCCESS: u32 = 12906u32;
pub const ERROR_INTERNET_CANNOT_CONNECT: u32 = 12029u32;
pub const ERROR_INTERNET_CHG_POST_IS_NON_SECURE: u32 = 12042u32;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED: u32 = 12044u32;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187u32;
pub const ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP: u32 = 12046u32;
pub const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030u32;
pub const ERROR_INTERNET_CONNECTION_AVAILABLE: u32 = 12902u32;
pub const ERROR_INTERNET_CONNECTION_RESET: u32 = 12031u32;
pub const ERROR_INTERNET_DECODING_FAILED: u32 = 12175u32;
pub const ERROR_INTERNET_DIALOG_PENDING: u32 = 12049u32;
pub const ERROR_INTERNET_DISALLOW_INPRIVATE: u32 = 12189u32;
pub const ERROR_INTERNET_DISCONNECTED: u32 = 12163u32;
pub const ERROR_INTERNET_EXTENDED_ERROR: u32 = 12003u32;
pub const ERROR_INTERNET_FAILED_DUETOSECURITYCHECK: u32 = 12171u32;
pub const ERROR_INTERNET_FEATURE_DISABLED: u32 = 12192u32;
pub const ERROR_INTERNET_FORCE_RETRY: u32 = 12032u32;
pub const ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED: u32 = 12054u32;
pub const ERROR_INTERNET_GLOBAL_CALLBACK_FAILED: u32 = 12191u32;
pub const ERROR_INTERNET_HANDLE_EXISTS: u32 = 12036u32;
pub const ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR: u32 = 12052u32;
pub const ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR: u32 = 12040u32;
pub const ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH: u32 = 12190u32;
pub const ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR: u32 = 12039u32;
pub const ERROR_INTERNET_INCORRECT_FORMAT: u32 = 12027u32;
pub const ERROR_INTERNET_INCORRECT_HANDLE_STATE: u32 = 12019u32;
pub const ERROR_INTERNET_INCORRECT_HANDLE_TYPE: u32 = 12018u32;
pub const ERROR_INTERNET_INCORRECT_PASSWORD: u32 = 12014u32;
pub const ERROR_INTERNET_INCORRECT_USER_NAME: u32 = 12013u32;
pub const ERROR_INTERNET_INSECURE_FALLBACK_REQUIRED: u32 = 12059u32;
pub const ERROR_INTERNET_INSERT_CDROM: u32 = 12053u32;
pub const ERROR_INTERNET_INTERNAL_ERROR: u32 = 12004u32;
pub const ERROR_INTERNET_INTERNAL_SOCKET_ERROR: u32 = 12901u32;
pub const ERROR_INTERNET_INVALID_CA: u32 = 12045u32;
pub const ERROR_INTERNET_INVALID_OPERATION: u32 = 12016u32;
pub const ERROR_INTERNET_INVALID_OPTION: u32 = 12009u32;
pub const ERROR_INTERNET_INVALID_PROXY_REQUEST: u32 = 12033u32;
pub const ERROR_INTERNET_INVALID_URL: u32 = 12005u32;
pub const ERROR_INTERNET_ITEM_NOT_FOUND: u32 = 12028u32;
pub const ERROR_INTERNET_LOGIN_FAILURE: u32 = 12015u32;
pub const ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 12174u32;
pub const ERROR_INTERNET_MIXED_SECURITY: u32 = 12041u32;
pub const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007u32;
pub const ERROR_INTERNET_NEED_MSN_SSPI_PKG: u32 = 12173u32;
pub const ERROR_INTERNET_NEED_UI: u32 = 12034u32;
pub const ERROR_INTERNET_NOT_INITIALIZED: u32 = 12172u32;
pub const ERROR_INTERNET_NOT_PROXY_REQUEST: u32 = 12020u32;
pub const ERROR_INTERNET_NO_CALLBACK: u32 = 12025u32;
pub const ERROR_INTERNET_NO_CM_CONNECTION: u32 = 12080u32;
pub const ERROR_INTERNET_NO_CONTEXT: u32 = 12024u32;
pub const ERROR_INTERNET_NO_DIRECT_ACCESS: u32 = 12023u32;
pub const ERROR_INTERNET_NO_KNOWN_SERVERS: u32 = 12903u32;
pub const ERROR_INTERNET_NO_NEW_CONTAINERS: u32 = 12051u32;
pub const ERROR_INTERNET_NO_PING_SUPPORT: u32 = 12905u32;
pub const ERROR_INTERNET_OFFLINE: u32 = 12163u32;
pub const ERROR_INTERNET_OPERATION_CANCELLED: u32 = 12017u32;
pub const ERROR_INTERNET_OPTION_NOT_SETTABLE: u32 = 12011u32;
pub const ERROR_INTERNET_OUT_OF_HANDLES: u32 = 12001u32;
pub const ERROR_INTERNET_PING_FAILED: u32 = 12904u32;
pub const ERROR_INTERNET_POST_IS_NON_SECURE: u32 = 12043u32;
pub const ERROR_INTERNET_PROTOCOL_NOT_FOUND: u32 = 12008u32;
pub const ERROR_INTERNET_PROXY_ALERT: u32 = 12061u32;
pub const ERROR_INTERNET_PROXY_SERVER_UNREACHABLE: u32 = 12165u32;
pub const ERROR_INTERNET_REDIRECT_SCHEME_CHANGE: u32 = 12048u32;
pub const ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND: u32 = 12021u32;
pub const ERROR_INTERNET_REQUEST_PENDING: u32 = 12026u32;
pub const ERROR_INTERNET_RETRY_DIALOG: u32 = 12050u32;
pub const ERROR_INTERNET_SECURE_FAILURE_PROXY: u32 = 12188u32;
pub const ERROR_INTERNET_SECURITY_CHANNEL_ERROR: u32 = 12157u32;
pub const ERROR_INTERNET_SEC_CERT_CN_INVALID: u32 = 12038u32;
pub const ERROR_INTERNET_SEC_CERT_DATE_INVALID: u32 = 12037u32;
pub const ERROR_INTERNET_SEC_CERT_ERRORS: u32 = 12055u32;
pub const ERROR_INTERNET_SEC_CERT_NO_REV: u32 = 12056u32;
pub const ERROR_INTERNET_SEC_CERT_REVOKED: u32 = 12170u32;
pub const ERROR_INTERNET_SEC_CERT_REV_FAILED: u32 = 12057u32;
pub const ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE: u32 = 12062u32;
pub const ERROR_INTERNET_SEC_INVALID_CERT: u32 = 12169u32;
pub const ERROR_INTERNET_SERVER_UNREACHABLE: u32 = 12164u32;
pub const ERROR_INTERNET_SHUTDOWN: u32 = 12012u32;
pub const ERROR_INTERNET_SOURCE_PORT_IN_USE: u32 = 12058u32;
pub const ERROR_INTERNET_TCPIP_NOT_INSTALLED: u32 = 12159u32;
pub const ERROR_INTERNET_TIMEOUT: u32 = 12002u32;
pub const ERROR_INTERNET_UNABLE_TO_CACHE_FILE: u32 = 12158u32;
pub const ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167u32;
pub const ERROR_INTERNET_UNRECOGNIZED_SCHEME: u32 = 12006u32;
pub const FLAGS_ERROR_UI_FILTER_FOR_ERRORS: u32 = 1u32;
pub const FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS: u32 = 2u32;
pub const FLAGS_ERROR_UI_FLAGS_GENERATE_DATA: u32 = 4u32;
pub const FLAGS_ERROR_UI_FLAGS_NO_UI: u32 = 8u32;
pub const FLAGS_ERROR_UI_SERIALIZE_DIALOGS: u32 = 16u32;
pub const FLAGS_ERROR_UI_SHOW_IDN_HOSTNAME: u32 = 32u32;
pub const FLAG_ICC_FORCE_CONNECTION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FORTCMD(pub i32);
pub const FORTCMD_CHG_PERSONALITY: FORTCMD = FORTCMD(3i32);
pub const FORTCMD_LOGOFF: FORTCMD = FORTCMD(2i32);
pub const FORTCMD_LOGON: FORTCMD = FORTCMD(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FORTSTAT(pub i32);
pub const FORTSTAT_INSTALLED: FORTSTAT = FORTSTAT(1i32);
pub const FORTSTAT_LOGGEDON: FORTSTAT = FORTSTAT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FTP_FLAGS(pub u32);
pub const FTP_TRANSFER_TYPE_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
pub const FTP_TRANSFER_TYPE_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
pub const FTP_TRANSFER_TYPE_UNKNOWN: FTP_FLAGS = FTP_FLAGS(0u32);
pub const GOPHER_ABSTRACT_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Abstract");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub ShortAbstract: *mut i8,
    pub AbstractFile: *mut i8,
}
impl Default for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_ABSTRACT_CATEGORY: windows_core::PCWSTR = windows_core::w!("+ABSTRACT");
pub const GOPHER_ADMIN_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Admin");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub Comment: *mut i8,
    pub EmailAddress: *mut i8,
}
impl Default for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_ADMIN_CATEGORY: windows_core::PCWSTR = windows_core::w!("+ADMIN");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_ASK_ATTRIBUTE_TYPE {
    pub QuestionType: *mut i8,
    pub QuestionText: *mut i8,
}
impl Default for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GOPHER_ATTRIBUTE_ENUMERATOR = Option<unsafe extern "system" fn(lpattributeinfo: *const GOPHER_ATTRIBUTE_TYPE, dwerror: u32) -> windows_core::BOOL>;
pub const GOPHER_ATTRIBUTE_ID_ABSTRACT: u32 = 2882325526u32;
pub const GOPHER_ATTRIBUTE_ID_ADMIN: u32 = 2882325514u32;
pub const GOPHER_ATTRIBUTE_ID_ALL: u32 = 2882325513u32;
pub const GOPHER_ATTRIBUTE_ID_BASE: u32 = 2882325504u32;
pub const GOPHER_ATTRIBUTE_ID_GEOG: u32 = 2882325522u32;
pub const GOPHER_ATTRIBUTE_ID_LOCATION: u32 = 2882325521u32;
pub const GOPHER_ATTRIBUTE_ID_MOD_DATE: u32 = 2882325515u32;
pub const GOPHER_ATTRIBUTE_ID_ORG: u32 = 2882325520u32;
pub const GOPHER_ATTRIBUTE_ID_PROVIDER: u32 = 2882325524u32;
pub const GOPHER_ATTRIBUTE_ID_RANGE: u32 = 2882325518u32;
pub const GOPHER_ATTRIBUTE_ID_SCORE: u32 = 2882325517u32;
pub const GOPHER_ATTRIBUTE_ID_SITE: u32 = 2882325519u32;
pub const GOPHER_ATTRIBUTE_ID_TIMEZONE: u32 = 2882325523u32;
pub const GOPHER_ATTRIBUTE_ID_TREEWALK: u32 = 2882325528u32;
pub const GOPHER_ATTRIBUTE_ID_TTL: u32 = 2882325516u32;
pub const GOPHER_ATTRIBUTE_ID_UNKNOWN: u32 = 2882325529u32;
pub const GOPHER_ATTRIBUTE_ID_VERSION: u32 = 2882325525u32;
pub const GOPHER_ATTRIBUTE_ID_VIEW: u32 = 2882325527u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GOPHER_ATTRIBUTE_TYPE {
    pub CategoryId: u32,
    pub AttributeId: u32,
    pub AttributeType: GOPHER_ATTRIBUTE_TYPE_0,
}
impl Default for GOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GOPHER_ATTRIBUTE_TYPE_0 {
    pub Admin: GOPHER_ADMIN_ATTRIBUTE_TYPE,
    pub ModDate: GOPHER_MOD_DATE_ATTRIBUTE_TYPE,
    pub Ttl: GOPHER_TTL_ATTRIBUTE_TYPE,
    pub Score: GOPHER_SCORE_ATTRIBUTE_TYPE,
    pub ScoreRange: GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE,
    pub Site: GOPHER_SITE_ATTRIBUTE_TYPE,
    pub Organization: GOPHER_ORGANIZATION_ATTRIBUTE_TYPE,
    pub Location: GOPHER_LOCATION_ATTRIBUTE_TYPE,
    pub GeographicalLocation: GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE,
    pub TimeZone: GOPHER_TIMEZONE_ATTRIBUTE_TYPE,
    pub Provider: GOPHER_PROVIDER_ATTRIBUTE_TYPE,
    pub Version: GOPHER_VERSION_ATTRIBUTE_TYPE,
    pub Abstract: GOPHER_ABSTRACT_ATTRIBUTE_TYPE,
    pub View: GOPHER_VIEW_ATTRIBUTE_TYPE,
    pub Veronica: GOPHER_VERONICA_ATTRIBUTE_TYPE,
    pub Ask: GOPHER_ASK_ATTRIBUTE_TYPE,
    pub Unknown: GOPHER_UNKNOWN_ATTRIBUTE_TYPE,
}
impl Default for GOPHER_ATTRIBUTE_TYPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_CATEGORY_ID_ABSTRACT: u32 = 2882325509u32;
pub const GOPHER_CATEGORY_ID_ADMIN: u32 = 2882325507u32;
pub const GOPHER_CATEGORY_ID_ALL: u32 = 2882325505u32;
pub const GOPHER_CATEGORY_ID_ASK: u32 = 2882325511u32;
pub const GOPHER_CATEGORY_ID_INFO: u32 = 2882325506u32;
pub const GOPHER_CATEGORY_ID_UNKNOWN: u32 = 2882325512u32;
pub const GOPHER_CATEGORY_ID_VERONICA: u32 = 2882325510u32;
pub const GOPHER_CATEGORY_ID_VIEWS: u32 = 2882325508u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_FIND_DATAA {
    pub DisplayString: [i8; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [i8; 654],
}
impl Default for GOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_FIND_DATAW {
    pub DisplayString: [u16; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [u16; 654],
}
impl Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    pub DegreesNorth: i32,
    pub MinutesNorth: i32,
    pub SecondsNorth: i32,
    pub DegreesEast: i32,
    pub MinutesEast: i32,
    pub SecondsEast: i32,
}
pub const GOPHER_GEOG_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Geog");
pub const GOPHER_INFO_CATEGORY: windows_core::PCWSTR = windows_core::w!("+INFO");
pub const GOPHER_LOCATION_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Loc");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub Location: *mut i8,
}
impl Default for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_MOD_DATE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Mod-Date");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub DateAndTime: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub Organization: *mut i8,
}
impl Default for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_ORG_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Org");
pub const GOPHER_PROVIDER_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Provider");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub Provider: *mut i8,
}
impl Default for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_RANGE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Score-range");
pub const GOPHER_SCORE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Score");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_SCORE_ATTRIBUTE_TYPE {
    pub Score: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub LowerBound: i32,
    pub UpperBound: i32,
}
pub const GOPHER_SITE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Site");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_SITE_ATTRIBUTE_TYPE {
    pub Site: *mut i8,
}
impl Default for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_TIMEZONE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("TZ");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub Zone: i32,
}
pub const GOPHER_TREEWALK_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("treewalk");
pub const GOPHER_TTL_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("TTL");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_TTL_ATTRIBUTE_TYPE {
    pub Ttl: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GOPHER_TYPE(pub u32);
pub const GOPHER_TYPE_ASK: GOPHER_TYPE = GOPHER_TYPE(1073741824u32);
pub const GOPHER_TYPE_BINARY: GOPHER_TYPE = GOPHER_TYPE(512u32);
pub const GOPHER_TYPE_BITMAP: GOPHER_TYPE = GOPHER_TYPE(16384u32);
pub const GOPHER_TYPE_CALENDAR: GOPHER_TYPE = GOPHER_TYPE(524288u32);
pub const GOPHER_TYPE_CSO: GOPHER_TYPE = GOPHER_TYPE(4u32);
pub const GOPHER_TYPE_DIRECTORY: GOPHER_TYPE = GOPHER_TYPE(2u32);
pub const GOPHER_TYPE_DOS_ARCHIVE: GOPHER_TYPE = GOPHER_TYPE(32u32);
pub const GOPHER_TYPE_ERROR: GOPHER_TYPE = GOPHER_TYPE(8u32);
pub const GOPHER_TYPE_GIF: GOPHER_TYPE = GOPHER_TYPE(4096u32);
pub const GOPHER_TYPE_GOPHER_PLUS: GOPHER_TYPE = GOPHER_TYPE(2147483648u32);
pub const GOPHER_TYPE_HTML: GOPHER_TYPE = GOPHER_TYPE(131072u32);
pub const GOPHER_TYPE_IMAGE: GOPHER_TYPE = GOPHER_TYPE(8192u32);
pub const GOPHER_TYPE_INDEX_SERVER: GOPHER_TYPE = GOPHER_TYPE(128u32);
pub const GOPHER_TYPE_INLINE: GOPHER_TYPE = GOPHER_TYPE(1048576u32);
pub const GOPHER_TYPE_MAC_BINHEX: GOPHER_TYPE = GOPHER_TYPE(16u32);
pub const GOPHER_TYPE_MOVIE: GOPHER_TYPE = GOPHER_TYPE(32768u32);
pub const GOPHER_TYPE_PDF: GOPHER_TYPE = GOPHER_TYPE(262144u32);
pub const GOPHER_TYPE_REDUNDANT: GOPHER_TYPE = GOPHER_TYPE(1024u32);
pub const GOPHER_TYPE_SOUND: GOPHER_TYPE = GOPHER_TYPE(65536u32);
pub const GOPHER_TYPE_TELNET: GOPHER_TYPE = GOPHER_TYPE(256u32);
pub const GOPHER_TYPE_TEXT_FILE: GOPHER_TYPE = GOPHER_TYPE(1u32);
pub const GOPHER_TYPE_TN3270: GOPHER_TYPE = GOPHER_TYPE(2048u32);
pub const GOPHER_TYPE_UNIX_UUENCODED: GOPHER_TYPE = GOPHER_TYPE(64u32);
pub const GOPHER_TYPE_UNKNOWN: GOPHER_TYPE = GOPHER_TYPE(536870912u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub Text: *mut i8,
}
impl Default for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub TreeWalk: windows_core::BOOL,
}
pub const GOPHER_VERONICA_CATEGORY: windows_core::PCWSTR = windows_core::w!("+VERONICA");
pub const GOPHER_VERSION_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("Version");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_VERSION_ATTRIBUTE_TYPE {
    pub Version: *mut i8,
}
impl Default for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GOPHER_VIEWS_CATEGORY: windows_core::PCWSTR = windows_core::w!("+VIEWS");
pub const GOPHER_VIEW_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("View");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GOPHER_VIEW_ATTRIBUTE_TYPE {
    pub ContentType: *mut i8,
    pub Language: *mut i8,
    pub Size: u32,
}
impl Default for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GROUPNAME_MAX_LENGTH: u32 = 120u32;
pub const GROUP_OWNER_STORAGE_SIZE: u32 = 4u32;
pub const HSR_ASYNC: u32 = 1u32;
pub const HSR_CHUNKED: u32 = 32u32;
pub const HSR_DOWNLOAD: u32 = 16u32;
pub const HSR_INITIATE: u32 = 8u32;
pub const HSR_SYNC: u32 = 4u32;
pub const HSR_USE_CONTEXT: u32 = 8u32;
pub const HTTP_1_1_CACHE_ENTRY: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_ADDREQ_FLAG(pub u32);
impl HTTP_ADDREQ_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HTTP_ADDREQ_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HTTP_ADDREQ_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760u32;
pub const HTTP_ADDREQ_FLAG_ADD: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(536870912u32);
pub const HTTP_ADDREQ_FLAG_ADD_IF_NEW: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(268435456u32);
pub const HTTP_ADDREQ_FLAG_ALLOW_EMPTY_VALUES: u32 = 67108864u32;
pub const HTTP_ADDREQ_FLAG_COALESCE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(16777216u32);
pub const HTTP_ADDREQ_FLAG_REPLACE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(2147483648u32);
pub const HTTP_ADDREQ_FLAG_RESPONSE_HEADERS: u32 = 33554432u32;
pub const HTTP_ADDREQ_INDEX_MASK: u32 = 65535u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE: u32 = 3u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX: u32 = 2u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_MAX: u32 = 3u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE: u32 = 1u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN: u32 = 0u32;
pub const HTTP_MAJOR_VERSION: u32 = 1u32;
pub const HTTP_MINOR_VERSION: u32 = 0u32;
pub type HTTP_POLICY_EXTENSION_INIT = Option<unsafe extern "system" fn(version: HTTP_POLICY_EXTENSION_VERSION, r#type: HTTP_POLICY_EXTENSION_TYPE, pvdata: *const core::ffi::c_void, cbdata: u32) -> u32>;
pub type HTTP_POLICY_EXTENSION_SHUTDOWN = Option<unsafe extern "system" fn(r#type: HTTP_POLICY_EXTENSION_TYPE) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_POLICY_EXTENSION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_POLICY_EXTENSION_VERSION(pub i32);
pub const HTTP_PROTOCOL_FLAG_HTTP2: u32 = 2u32;
pub const HTTP_PROTOCOL_MASK: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_PUSH_NOTIFICATION_STATUS {
    pub ChannelStatusValid: windows_core::BOOL,
    pub ChannelStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_PUSH_TRANSPORT_SETTING {
    pub TransportSettingId: windows_core::GUID,
    pub BrokerEventId: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTTP_PUSH_WAIT_HANDLE(pub *mut core::ffi::c_void);
impl HTTP_PUSH_WAIT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HTTP_PUSH_WAIT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_PUSH_WAIT_TYPE(pub i32);
pub const HTTP_QUERY_ACCEPT: u32 = 24u32;
pub const HTTP_QUERY_ACCEPT_CHARSET: u32 = 25u32;
pub const HTTP_QUERY_ACCEPT_ENCODING: u32 = 26u32;
pub const HTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27u32;
pub const HTTP_QUERY_ACCEPT_RANGES: u32 = 42u32;
pub const HTTP_QUERY_AGE: u32 = 48u32;
pub const HTTP_QUERY_ALLOW: u32 = 7u32;
pub const HTTP_QUERY_AUTHENTICATION_INFO: u32 = 76u32;
pub const HTTP_QUERY_AUTHORIZATION: u32 = 28u32;
pub const HTTP_QUERY_CACHE_CONTROL: u32 = 49u32;
pub const HTTP_QUERY_CONNECTION: u32 = 23u32;
pub const HTTP_QUERY_CONTENT_BASE: u32 = 50u32;
pub const HTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4u32;
pub const HTTP_QUERY_CONTENT_DISPOSITION: u32 = 47u32;
pub const HTTP_QUERY_CONTENT_ENCODING: u32 = 29u32;
pub const HTTP_QUERY_CONTENT_ID: u32 = 3u32;
pub const HTTP_QUERY_CONTENT_LANGUAGE: u32 = 6u32;
pub const HTTP_QUERY_CONTENT_LENGTH: u32 = 5u32;
pub const HTTP_QUERY_CONTENT_LOCATION: u32 = 51u32;
pub const HTTP_QUERY_CONTENT_MD5: u32 = 52u32;
pub const HTTP_QUERY_CONTENT_RANGE: u32 = 53u32;
pub const HTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2u32;
pub const HTTP_QUERY_CONTENT_TYPE: u32 = 1u32;
pub const HTTP_QUERY_COOKIE: u32 = 44u32;
pub const HTTP_QUERY_COST: u32 = 15u32;
pub const HTTP_QUERY_CUSTOM: u32 = 65535u32;
pub const HTTP_QUERY_DATE: u32 = 9u32;
pub const HTTP_QUERY_DEFAULT_STYLE: u32 = 84u32;
pub const HTTP_QUERY_DERIVED_FROM: u32 = 14u32;
pub const HTTP_QUERY_DO_NOT_TRACK: u32 = 88u32;
pub const HTTP_QUERY_ECHO_HEADERS: u32 = 73u32;
pub const HTTP_QUERY_ECHO_HEADERS_CRLF: u32 = 74u32;
pub const HTTP_QUERY_ECHO_REPLY: u32 = 72u32;
pub const HTTP_QUERY_ECHO_REQUEST: u32 = 71u32;
pub const HTTP_QUERY_ETAG: u32 = 54u32;
pub const HTTP_QUERY_EXPECT: u32 = 68u32;
pub const HTTP_QUERY_EXPIRES: u32 = 10u32;
pub const HTTP_QUERY_FLAG_COALESCE: u32 = 268435456u32;
pub const HTTP_QUERY_FLAG_COALESCE_WITH_COMMA: u32 = 67108864u32;
pub const HTTP_QUERY_FLAG_NUMBER: u32 = 536870912u32;
pub const HTTP_QUERY_FLAG_NUMBER64: u32 = 134217728u32;
pub const HTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648u32;
pub const HTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824u32;
pub const HTTP_QUERY_FORWARDED: u32 = 30u32;
pub const HTTP_QUERY_FROM: u32 = 31u32;
pub const HTTP_QUERY_HOST: u32 = 55u32;
pub const HTTP_QUERY_HTTP2_SETTINGS: u32 = 90u32;
pub const HTTP_QUERY_IF_MATCH: u32 = 56u32;
pub const HTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32u32;
pub const HTTP_QUERY_IF_NONE_MATCH: u32 = 57u32;
pub const HTTP_QUERY_IF_RANGE: u32 = 58u32;
pub const HTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59u32;
pub const HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID: u32 = 93u32;
pub const HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID: u32 = 93u32;
pub const HTTP_QUERY_KEEP_ALIVE: u32 = 89u32;
pub const HTTP_QUERY_LAST_MODIFIED: u32 = 11u32;
pub const HTTP_QUERY_LINK: u32 = 16u32;
pub const HTTP_QUERY_LOCATION: u32 = 33u32;
pub const HTTP_QUERY_MAX: u32 = 95u32;
pub const HTTP_QUERY_MAX_FORWARDS: u32 = 60u32;
pub const HTTP_QUERY_MESSAGE_ID: u32 = 12u32;
pub const HTTP_QUERY_MIME_VERSION: u32 = 0u32;
pub const HTTP_QUERY_ORIG_URI: u32 = 34u32;
pub const HTTP_QUERY_P3P: u32 = 80u32;
pub const HTTP_QUERY_PASSPORT_CONFIG: u32 = 78u32;
pub const HTTP_QUERY_PASSPORT_URLS: u32 = 77u32;
pub const HTTP_QUERY_PRAGMA: u32 = 17u32;
pub const HTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41u32;
pub const HTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61u32;
pub const HTTP_QUERY_PROXY_CONNECTION: u32 = 69u32;
pub const HTTP_QUERY_PROXY_SUPPORT: u32 = 75u32;
pub const HTTP_QUERY_PUBLIC: u32 = 8u32;
pub const HTTP_QUERY_PUBLIC_KEY_PINS: u32 = 94u32;
pub const HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY: u32 = 95u32;
pub const HTTP_QUERY_RANGE: u32 = 62u32;
pub const HTTP_QUERY_RAW_HEADERS: u32 = 21u32;
pub const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22u32;
pub const HTTP_QUERY_REFERER: u32 = 35u32;
pub const HTTP_QUERY_REFRESH: u32 = 46u32;
pub const HTTP_QUERY_REQUEST_METHOD: u32 = 45u32;
pub const HTTP_QUERY_RETRY_AFTER: u32 = 36u32;
pub const HTTP_QUERY_SERVER: u32 = 37u32;
pub const HTTP_QUERY_SET_COOKIE: u32 = 43u32;
pub const HTTP_QUERY_SET_COOKIE2: u32 = 87u32;
pub const HTTP_QUERY_STATUS_CODE: u32 = 19u32;
pub const HTTP_QUERY_STATUS_TEXT: u32 = 20u32;
pub const HTTP_QUERY_STRICT_TRANSPORT_SECURITY: u32 = 91u32;
pub const HTTP_QUERY_TITLE: u32 = 38u32;
pub const HTTP_QUERY_TOKEN_BINDING: u32 = 92u32;
pub const HTTP_QUERY_TRANSFER_ENCODING: u32 = 63u32;
pub const HTTP_QUERY_TRANSLATE: u32 = 82u32;
pub const HTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70u32;
pub const HTTP_QUERY_UPGRADE: u32 = 64u32;
pub const HTTP_QUERY_URI: u32 = 13u32;
pub const HTTP_QUERY_USER_AGENT: u32 = 39u32;
pub const HTTP_QUERY_VARY: u32 = 65u32;
pub const HTTP_QUERY_VERSION: u32 = 18u32;
pub const HTTP_QUERY_VIA: u32 = 66u32;
pub const HTTP_QUERY_WARNING: u32 = 67u32;
pub const HTTP_QUERY_WWW_AUTHENTICATE: u32 = 40u32;
pub const HTTP_QUERY_X_CONTENT_TYPE_OPTIONS: u32 = 79u32;
pub const HTTP_QUERY_X_FRAME_OPTIONS: u32 = 85u32;
pub const HTTP_QUERY_X_P2P_PEERDIST: u32 = 81u32;
pub const HTTP_QUERY_X_UA_COMPATIBLE: u32 = 83u32;
pub const HTTP_QUERY_X_XSS_PROTECTION: u32 = 86u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgTimes: [u64; 32],
}
impl Default for HTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HTTP_STATUS_MISDIRECTED_REQUEST: u32 = 421u32;
pub const HTTP_VERSIONA: windows_core::PCSTR = windows_core::s!("HTTP/1.0");
pub const HTTP_VERSIONW: windows_core::PCWSTR = windows_core::w!("HTTP/1.0");
pub const HTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1006i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: INTERNET_ASYNC_RESULT,
    pub Operation: HTTP_WEB_SOCKET_OPERATION,
    pub BufferType: HTTP_WEB_SOCKET_BUFFER_TYPE,
    pub dwBytesTransferred: u32,
}
pub const HTTP_WEB_SOCKET_BINARY_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(1i32);
pub const HTTP_WEB_SOCKET_BINARY_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_WEB_SOCKET_BUFFER_TYPE(pub i32);
pub const HTTP_WEB_SOCKET_CLOSE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_WEB_SOCKET_CLOSE_STATUS(pub i32);
pub const HTTP_WEB_SOCKET_CLOSE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(4i32);
pub const HTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const HTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1001i32);
pub const HTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const HTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
pub const HTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const HTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: u32 = 10000u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HTTP_WEB_SOCKET_OPERATION(pub i32);
pub const HTTP_WEB_SOCKET_PING_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(5i32);
pub const HTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1008i32);
pub const HTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const HTTP_WEB_SOCKET_RECEIVE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(1i32);
pub const HTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1015i32);
pub const HTTP_WEB_SOCKET_SEND_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(0i32);
pub const HTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const HTTP_WEB_SOCKET_SHUTDOWN_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(3i32);
pub const HTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const HTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const HTTP_WEB_SOCKET_UTF8_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(3i32);
pub const HTTP_WEB_SOCKET_UTF8_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(2i32);
pub const HttpPushWaitEnableComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(0i32);
pub const HttpPushWaitReceiveComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(1i32);
pub const HttpPushWaitSendComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(2i32);
pub const HttpRequestTimeMax: REQUEST_TIMES = REQUEST_TIMES(32i32);
pub const ICU_USERNAME: u32 = 1073741824u32;
pub const IDENTITY_CACHE_ENTRY: u32 = 2147483648u32;
pub const IDSI_FLAG_KEEP_ALIVE: u32 = 1u32;
pub const IDSI_FLAG_PROXY: u32 = 4u32;
pub const IDSI_FLAG_SECURE: u32 = 2u32;
pub const IDSI_FLAG_TUNNEL: u32 = 8u32;
windows_core::imp::define_interface!(IDialBranding, IDialBranding_Vtbl, 0x8aecafa9_4306_43cc_8c5a_765f2979cc16);
windows_core::imp::interface_hierarchy!(IDialBranding, windows_core::IUnknown);
impl IDialBranding {
    pub unsafe fn Initialize<P0>(&self, pwzconnectoid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pwzconnectoid.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self, dwindex: u32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialBranding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetBitmap: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDialBranding_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pwzconnectoid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBitmap(&self, dwindex: u32) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDialBranding_Vtbl {
    pub const fn new<Identity: IDialBranding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDialBranding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzconnectoid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialBranding_Impl::Initialize(this, core::mem::transmute(&pwzconnectoid)).into()
            }
        }
        unsafe extern "system" fn GetBitmap<Identity: IDialBranding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDialBranding_Impl::GetBitmap(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        phbitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetBitmap: GetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialBranding as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDialBranding {}
windows_core::imp::define_interface!(IDialEngine, IDialEngine_Vtbl, 0x39fd782b_7905_40d5_9148_3c9b190423d5);
windows_core::imp::interface_hierarchy!(IDialEngine, windows_core::IUnknown);
impl IDialEngine {
    pub unsafe fn Initialize<P0, P1>(&self, pwzconnectoid: P0, pides: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDialEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pwzconnectoid.param().abi(), pides.param().abi()).ok() }
    }
    pub unsafe fn GetProperty<P0, P1>(&self, pwzproperty: P0, pwzvalue: P1, dwbufsize: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwzproperty.param().abi(), pwzvalue.param().abi(), dwbufsize).ok() }
    }
    pub unsafe fn SetProperty<P0, P1>(&self, pwzproperty: P0, pwzvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwzproperty.param().abi(), pwzvalue.param().abi()).ok() }
    }
    pub unsafe fn Dial(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Dial)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn HangUp(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HangUp)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetConnectedState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectHandle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Dial: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HangUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnectHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
pub trait IDialEngine_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pwzconnectoid: &windows_core::PCWSTR, pides: windows_core::Ref<IDialEventSink>) -> windows_core::Result<()>;
    fn GetProperty(&self, pwzproperty: &windows_core::PCWSTR, pwzvalue: &windows_core::PCWSTR, dwbufsize: u32) -> windows_core::Result<()>;
    fn SetProperty(&self, pwzproperty: &windows_core::PCWSTR, pwzvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Dial(&self) -> windows_core::Result<()>;
    fn HangUp(&self) -> windows_core::Result<()>;
    fn GetConnectedState(&self) -> windows_core::Result<u32>;
    fn GetConnectHandle(&self) -> windows_core::Result<usize>;
}
impl IDialEngine_Vtbl {
    pub const fn new<Identity: IDialEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzconnectoid: windows_core::PCWSTR, pides: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEngine_Impl::Initialize(this, core::mem::transmute(&pwzconnectoid), core::mem::transmute_copy(&pides)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzproperty: windows_core::PCWSTR, pwzvalue: windows_core::PCWSTR, dwbufsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEngine_Impl::GetProperty(this, core::mem::transmute(&pwzproperty), core::mem::transmute(&pwzvalue), core::mem::transmute_copy(&dwbufsize)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzproperty: windows_core::PCWSTR, pwzvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEngine_Impl::SetProperty(this, core::mem::transmute(&pwzproperty), core::mem::transmute(&pwzvalue)).into()
            }
        }
        unsafe extern "system" fn Dial<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEngine_Impl::Dial(this).into()
            }
        }
        unsafe extern "system" fn HangUp<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEngine_Impl::HangUp(this).into()
            }
        }
        unsafe extern "system" fn GetConnectedState<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDialEngine_Impl::GetConnectedState(this) {
                    Ok(ok__) => {
                        pdwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectHandle<Identity: IDialEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDialEngine_Impl::GetConnectHandle(this) {
                    Ok(ok__) => {
                        pdwhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Dial: Dial::<Identity, OFFSET>,
            HangUp: HangUp::<Identity, OFFSET>,
            GetConnectedState: GetConnectedState::<Identity, OFFSET>,
            GetConnectHandle: GetConnectHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDialEngine {}
windows_core::imp::define_interface!(IDialEventSink, IDialEventSink_Vtbl, 0x2d86f4ff_6e2d_4488_b2e9_6934afd41bea);
windows_core::imp::interface_hierarchy!(IDialEventSink, windows_core::IUnknown);
impl IDialEventSink {
    pub unsafe fn OnEvent(&self, dwevent: u32, dwstatus: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dwevent, dwstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IDialEventSink_Impl: windows_core::IUnknownImpl {
    fn OnEvent(&self, dwevent: u32, dwstatus: u32) -> windows_core::Result<()>;
}
impl IDialEventSink_Vtbl {
    pub const fn new<Identity: IDialEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEvent<Identity: IDialEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwevent: u32, dwstatus: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDialEventSink_Impl::OnEvent(this, core::mem::transmute_copy(&dwevent), core::mem::transmute_copy(&dwstatus)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDialEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDialEventSink {}
pub const IMMUTABLE_CACHE_ENTRY: u32 = 524288u32;
pub const INSTALLED_CACHE_ENTRY: u32 = 268435456u32;
pub const INTERENT_GOONLINE_MASK: u32 = 3u32;
pub const INTERENT_GOONLINE_NOPROMPT: u32 = 2u32;
pub const INTERENT_GOONLINE_REFRESH: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_ACCESS_TYPE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct INTERNET_AUTH_NOTIFY_DATA {
    pub cbStruct: u32,
    pub dwOptions: u32,
    pub pfnNotify: PFN_AUTH_NOTIFY,
    pub dwContext: usize,
}
pub const INTERNET_AUTH_SCHEME_BASIC: u32 = 0u32;
pub const INTERNET_AUTH_SCHEME_DIGEST: u32 = 1u32;
pub const INTERNET_AUTH_SCHEME_KERBEROS: u32 = 3u32;
pub const INTERNET_AUTH_SCHEME_NEGOTIATE: u32 = 4u32;
pub const INTERNET_AUTH_SCHEME_NTLM: u32 = 2u32;
pub const INTERNET_AUTH_SCHEME_PASSPORT: u32 = 5u32;
pub const INTERNET_AUTH_SCHEME_UNKNOWN: u32 = 6u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_AUTODIAL(pub u32);
pub const INTERNET_AUTODIAL_FAILIFSECURITYCHECK: INTERNET_AUTODIAL = INTERNET_AUTODIAL(4u32);
pub const INTERNET_AUTODIAL_FORCE_ONLINE: INTERNET_AUTODIAL = INTERNET_AUTODIAL(1u32);
pub const INTERNET_AUTODIAL_FORCE_UNATTENDED: INTERNET_AUTODIAL = INTERNET_AUTODIAL(2u32);
pub const INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT: INTERNET_AUTODIAL = INTERNET_AUTODIAL(8u32);
pub const INTERNET_AUTOPROXY_INIT_DEFAULT: u32 = 1u32;
pub const INTERNET_AUTOPROXY_INIT_DOWNLOADSYNC: u32 = 2u32;
pub const INTERNET_AUTOPROXY_INIT_ONLYQUERY: u32 = 8u32;
pub const INTERNET_AUTOPROXY_INIT_QUERYSTATE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_BUFFERSA {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSA,
    pub lpcszHeader: windows_core::PCSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl Default for INTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_BUFFERSW {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSW,
    pub lpcszHeader: windows_core::PCWSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
impl Default for INTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_CONFIG_INFOA {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: windows_core::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
impl Default for INTERNET_CACHE_CONFIG_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_CONFIG_INFOA_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYA; 1],
}
impl Default for INTERNET_CACHE_CONFIG_INFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_CONFIG_INFOA_0_0 {
    pub CachePath: [i8; 260],
    pub dwCacheSize: u32,
}
impl Default for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_CONFIG_INFOW {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: windows_core::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
impl Default for INTERNET_CACHE_CONFIG_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_CONFIG_INFOW_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYW; 1],
}
impl Default for INTERNET_CACHE_CONFIG_INFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_CONFIG_INFOW_0_0 {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
impl Default for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    pub CachePath: [i8; 260],
    pub dwCacheSize: u32,
}
impl Default for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
impl Default for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_CACHE_CONTAINER_AUTODELETE: u32 = 2u32;
pub const INTERNET_CACHE_CONTAINER_BLOOM_FILTER: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CACHE_CONTAINER_INFOA {
    pub dwCacheVersion: u32,
    pub lpszName: windows_core::PSTR,
    pub lpszCachePrefix: windows_core::PSTR,
    pub lpszVolumeLabel: windows_core::PSTR,
    pub lpszVolumeTitle: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CACHE_CONTAINER_INFOW {
    pub dwCacheVersion: u32,
    pub lpszName: windows_core::PWSTR,
    pub lpszCachePrefix: windows_core::PWSTR,
    pub lpszVolumeLabel: windows_core::PWSTR,
    pub lpszVolumeTitle: windows_core::PWSTR,
}
pub const INTERNET_CACHE_CONTAINER_MAP_ENABLED: u32 = 16u32;
pub const INTERNET_CACHE_CONTAINER_NODESKTOPINIT: u32 = 8u32;
pub const INTERNET_CACHE_CONTAINER_NOSUBDIRS: u32 = 1u32;
pub const INTERNET_CACHE_CONTAINER_RESERVED1: u32 = 4u32;
pub const INTERNET_CACHE_CONTAINER_SHARE_READ: u32 = 256u32;
pub const INTERNET_CACHE_CONTAINER_SHARE_READ_WRITE: u32 = 768u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_ENTRY_INFOA {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: windows_core::PSTR,
    pub lpszLocalFileName: windows_core::PSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: windows_core::PSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_core::PSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOA_0,
}
impl Default for INTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_ENTRY_INFOA_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
impl Default for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_CACHE_ENTRY_INFOW {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: windows_core::PWSTR,
    pub lpszLocalFileName: windows_core::PWSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: windows_core::PWSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: windows_core::PWSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOW_0,
}
impl Default for INTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_CACHE_ENTRY_INFOW_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
impl Default for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_CACHE_FLAG_ADD_FILENAME_ONLY: u32 = 2048u32;
pub const INTERNET_CACHE_FLAG_ALLOW_COLLISIONS: u32 = 256u32;
pub const INTERNET_CACHE_FLAG_ENTRY_OR_MAPPING: u32 = 1024u32;
pub const INTERNET_CACHE_FLAG_GET_STRUCT_ONLY: u32 = 4096u32;
pub const INTERNET_CACHE_FLAG_INSTALLED_ENTRY: u32 = 512u32;
pub const INTERNET_CACHE_GROUP_ADD: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_GROUP_INFOA {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [i8; 120],
}
impl Default for INTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CACHE_GROUP_INFOW {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [u16; 120],
}
impl Default for INTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_CACHE_GROUP_REMOVE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CACHE_TIMESTAMPS {
    pub ftExpires: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CALLBACK_COOKIE {
    pub pcwszName: windows_core::PCWSTR,
    pub pcwszValue: windows_core::PCWSTR,
    pub pcwszDomain: windows_core::PCWSTR,
    pub pcwszPath: windows_core::PCWSTR,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_CERTIFICATE_INFO {
    pub ftExpiry: super::super::Foundation::FILETIME,
    pub ftStart: super::super::Foundation::FILETIME,
    pub lpszSubjectInfo: *mut i8,
    pub lpszIssuerInfo: *mut i8,
    pub lpszProtocolName: *mut i8,
    pub lpszSignatureAlgName: *mut i8,
    pub lpszEncryptionAlgName: *mut i8,
    pub dwKeySize: u32,
}
impl Default for INTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CONNECTED_INFO {
    pub dwConnectedState: INTERNET_STATE,
    pub dwFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_CONNECTION(pub u32);
impl INTERNET_CONNECTION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INTERNET_CONNECTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INTERNET_CONNECTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INTERNET_CONNECTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INTERNET_CONNECTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INTERNET_CONNECTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_CONNECTION_CONFIGURED: INTERNET_CONNECTION = INTERNET_CONNECTION(64u32);
pub const INTERNET_CONNECTION_LAN: INTERNET_CONNECTION = INTERNET_CONNECTION(2u32);
pub const INTERNET_CONNECTION_MODEM: INTERNET_CONNECTION = INTERNET_CONNECTION(1u32);
pub const INTERNET_CONNECTION_MODEM_BUSY: INTERNET_CONNECTION = INTERNET_CONNECTION(8u32);
pub const INTERNET_CONNECTION_OFFLINE: INTERNET_CONNECTION = INTERNET_CONNECTION(32u32);
pub const INTERNET_CONNECTION_PROXY: INTERNET_CONNECTION = INTERNET_CONNECTION(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_COOKIE {
    pub cbSize: u32,
    pub pszName: windows_core::PSTR,
    pub pszData: windows_core::PSTR,
    pub pszDomain: windows_core::PSTR,
    pub pszPath: windows_core::PSTR,
    pub pftExpires: *mut super::super::Foundation::FILETIME,
    pub dwFlags: u32,
    pub pszUrl: windows_core::PSTR,
    pub pszP3PPolicy: windows_core::PSTR,
}
impl Default for INTERNET_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_COOKIE2 {
    pub pwszName: windows_core::PWSTR,
    pub pwszValue: windows_core::PWSTR,
    pub pwszDomain: windows_core::PWSTR,
    pub pwszPath: windows_core::PWSTR,
    pub dwFlags: u32,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub fExpiresSet: windows_core::BOOL,
}
pub const INTERNET_COOKIE_ALL_COOKIES: u32 = 536870912u32;
pub const INTERNET_COOKIE_APPLY_HOST_ONLY: u32 = 32768u32;
pub const INTERNET_COOKIE_APPLY_P3P: u32 = 128u32;
pub const INTERNET_COOKIE_ECTX_3RDPARTY: u32 = 2147483648u32;
pub const INTERNET_COOKIE_EDGE_COOKIES: u32 = 262144u32;
pub const INTERNET_COOKIE_EVALUATE_P3P: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_COOKIE_FLAGS(pub u32);
pub const INTERNET_COOKIE_HOST_ONLY: u32 = 16384u32;
pub const INTERNET_COOKIE_HOST_ONLY_APPLIED: u32 = 524288u32;
pub const INTERNET_COOKIE_HTTPONLY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(8192u32);
pub const INTERNET_COOKIE_IE6: u32 = 1024u32;
pub const INTERNET_COOKIE_IS_LEGACY: u32 = 2048u32;
pub const INTERNET_COOKIE_IS_RESTRICTED: u32 = 512u32;
pub const INTERNET_COOKIE_IS_SECURE: u32 = 1u32;
pub const INTERNET_COOKIE_IS_SESSION: u32 = 2u32;
pub const INTERNET_COOKIE_NON_SCRIPT: u32 = 4096u32;
pub const INTERNET_COOKIE_NO_CALLBACK: u32 = 1073741824u32;
pub const INTERNET_COOKIE_P3P_ENABLED: u32 = 256u32;
pub const INTERNET_COOKIE_PERSISTENT_HOST_ONLY: u32 = 65536u32;
pub const INTERNET_COOKIE_PROMPT_REQUIRED: u32 = 32u32;
pub const INTERNET_COOKIE_RESTRICTED_ZONE: u32 = 131072u32;
pub const INTERNET_COOKIE_SAME_SITE_LAX: u32 = 2097152u32;
pub const INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE: u32 = 4194304u32;
pub const INTERNET_COOKIE_SAME_SITE_STRICT: u32 = 1048576u32;
pub const INTERNET_COOKIE_THIRD_PARTY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(16u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_CREDENTIALS {
    pub lpcwszHostName: windows_core::PCWSTR,
    pub dwPort: u32,
    pub dwScheme: u32,
    pub lpcwszUrl: windows_core::PCWSTR,
    pub lpcwszRealm: windows_core::PCWSTR,
    pub fAuthIdentity: windows_core::BOOL,
    pub Anonymous: INTERNET_CREDENTIALS_0,
}
impl Default for INTERNET_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_CREDENTIALS_0 {
    pub Anonymous: INTERNET_CREDENTIALS_0_0,
    pub pAuthIdentityOpaque: *mut core::ffi::c_void,
}
impl Default for INTERNET_CREDENTIALS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_CREDENTIALS_0_0 {
    pub lpcwszUserName: windows_core::PCWSTR,
    pub lpcwszPassword: windows_core::PCWSTR,
}
pub const INTERNET_CUSTOMDIAL_CAN_HANGUP: u32 = 4u32;
pub const INTERNET_CUSTOMDIAL_CONNECT: u32 = 0u32;
pub const INTERNET_CUSTOMDIAL_DISCONNECT: u32 = 2u32;
pub const INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED: u32 = 1u32;
pub const INTERNET_CUSTOMDIAL_SHOWOFFLINE: u32 = 4u32;
pub const INTERNET_CUSTOMDIAL_UNATTENDED: u32 = 1u32;
pub const INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE: u32 = 2u32;
pub const INTERNET_DEFAULT_FTP_PORT: u16 = 21u16;
pub const INTERNET_DEFAULT_GOPHER_PORT: u16 = 70u16;
pub const INTERNET_DEFAULT_SOCKS_PORT: u16 = 1080u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub Socket: usize,
    pub SourcePort: u32,
    pub DestPort: u32,
    pub Flags: u32,
}
pub const INTERNET_DIALSTATE_DISCONNECTED: u32 = 1u32;
pub const INTERNET_DIAL_FORCE_PROMPT: u32 = 8192u32;
pub const INTERNET_DIAL_SHOW_OFFLINE: u32 = 16384u32;
pub const INTERNET_DIAL_UNATTENDED: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_DOWNLOAD_MODE_HANDLE {
    pub pcwszFileName: windows_core::PCWSTR,
    pub phFile: *mut super::super::Foundation::HANDLE,
}
impl Default for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_END_BROWSER_SESSION_DATA {
    pub lpBuffer: *mut core::ffi::c_void,
    pub dwBufferLength: u32,
}
impl Default for INTERNET_END_BROWSER_SESSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_ERROR_BASE: u32 = 12000u32;
pub const INTERNET_ERROR_LAST: u32 = 12192u32;
pub const INTERNET_ERROR_MASK_COMBINED_SEC_CERT: u32 = 2u32;
pub const INTERNET_ERROR_MASK_INSERT_CDROM: u32 = 1u32;
pub const INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 8u32;
pub const INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG: u32 = 4u32;
pub const INTERNET_FIRST_OPTION: u32 = 1u32;
pub const INTERNET_FLAG_ASYNC: u32 = 268435456u32;
pub const INTERNET_FLAG_BGUPDATE: u32 = 8u32;
pub const INTERNET_FLAG_CACHE_ASYNC: u32 = 128u32;
pub const INTERNET_FLAG_CACHE_IF_NET_FAIL: u32 = 65536u32;
pub const INTERNET_FLAG_DONT_CACHE: u32 = 67108864u32;
pub const INTERNET_FLAG_EXISTING_CONNECT: u32 = 536870912u32;
pub const INTERNET_FLAG_FORMS_SUBMIT: u32 = 64u32;
pub const INTERNET_FLAG_FROM_CACHE: u32 = 16777216u32;
pub const INTERNET_FLAG_FTP_FOLDER_VIEW: u32 = 4u32;
pub const INTERNET_FLAG_FWD_BACK: u32 = 32u32;
pub const INTERNET_FLAG_HYPERLINK: u32 = 1024u32;
pub const INTERNET_FLAG_IDN_DIRECT: u32 = 1u32;
pub const INTERNET_FLAG_IDN_PROXY: u32 = 2u32;
pub const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096u32;
pub const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192u32;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
pub const INTERNET_FLAG_KEEP_CONNECTION: u32 = 4194304u32;
pub const INTERNET_FLAG_MAKE_PERSISTENT: u32 = 33554432u32;
pub const INTERNET_FLAG_MUST_CACHE_REQUEST: u32 = 16u32;
pub const INTERNET_FLAG_NEED_FILE: u32 = 16u32;
pub const INTERNET_FLAG_NO_AUTH: u32 = 262144u32;
pub const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 2097152u32;
pub const INTERNET_FLAG_NO_CACHE_WRITE: u32 = 67108864u32;
pub const INTERNET_FLAG_NO_COOKIES: u32 = 524288u32;
pub const INTERNET_FLAG_NO_UI: u32 = 512u32;
pub const INTERNET_FLAG_OFFLINE: u32 = 16777216u32;
pub const INTERNET_FLAG_PASSIVE: u32 = 134217728u32;
pub const INTERNET_FLAG_PRAGMA_NOCACHE: u32 = 256u32;
pub const INTERNET_FLAG_RAW_DATA: u32 = 1073741824u32;
pub const INTERNET_FLAG_READ_PREFETCH: u32 = 1048576u32;
pub const INTERNET_FLAG_RELOAD: u32 = 2147483648u32;
pub const INTERNET_FLAG_RESTRICTED_ZONE: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(131072u32);
pub const INTERNET_FLAG_RESYNCHRONIZE: u32 = 2048u32;
pub const INTERNET_FLAG_SECURE: u32 = 8388608u32;
pub const INTERNET_FLAG_TRANSFER_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
pub const INTERNET_FLAG_TRANSFER_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
pub const INTERNET_GLOBAL_CALLBACK_DETECTING_PROXY: u32 = 2u32;
pub const INTERNET_GLOBAL_CALLBACK_SENDING_HTTP_HEADERS: u32 = 1u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_FTP: u32 = 2u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_GOPHER: u32 = 3u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: u32 = 4u32;
pub const INTERNET_HANDLE_TYPE_FILE_REQUEST: u32 = 14u32;
pub const INTERNET_HANDLE_TYPE_FTP_FILE: u32 = 7u32;
pub const INTERNET_HANDLE_TYPE_FTP_FILE_HTML: u32 = 8u32;
pub const INTERNET_HANDLE_TYPE_FTP_FIND: u32 = 5u32;
pub const INTERNET_HANDLE_TYPE_FTP_FIND_HTML: u32 = 6u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE: u32 = 11u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML: u32 = 12u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND: u32 = 9u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML: u32 = 10u32;
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: u32 = 13u32;
pub const INTERNET_HANDLE_TYPE_INTERNET: u32 = 1u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_CONTENT: u32 = 32u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_COOKIES: u32 = 8u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_DATA: u32 = 4u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_HISTORY: u32 = 16u32;
pub const INTERNET_IDENTITY_FLAG_PRIVATE_CACHE: u32 = 1u32;
pub const INTERNET_IDENTITY_FLAG_SHARED_CACHE: u32 = 2u32;
pub const INTERNET_INTERNAL_ERROR_BASE: u32 = 12900u32;
pub const INTERNET_INVALID_PORT_NUMBER: u32 = 0u32;
pub const INTERNET_KEEP_ALIVE_DISABLED: u32 = 0u32;
pub const INTERNET_KEEP_ALIVE_ENABLED: u32 = 1u32;
pub const INTERNET_KEEP_ALIVE_UNKNOWN: u32 = 4294967295u32;
pub const INTERNET_LAST_OPTION: u32 = 193u32;
pub const INTERNET_LAST_OPTION_INTERNAL: u32 = 193u32;
pub const INTERNET_MAX_HOST_NAME_LENGTH: u32 = 256u32;
pub const INTERNET_MAX_PASSWORD_LENGTH: u32 = 128u32;
pub const INTERNET_MAX_PORT_NUMBER_LENGTH: u32 = 5u32;
pub const INTERNET_MAX_PORT_NUMBER_VALUE: u32 = 65535u32;
pub const INTERNET_MAX_USER_NAME_LENGTH: u32 = 128u32;
pub const INTERNET_NO_CALLBACK: u32 = 0u32;
pub const INTERNET_OPEN_TYPE_DIRECT: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(1u32);
pub const INTERNET_OPEN_TYPE_PRECONFIG: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(0u32);
pub const INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY: u32 = 4u32;
pub const INTERNET_OPEN_TYPE_PROXY: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(3u32);
pub const INTERNET_OPTION_ACTIVATE_WORKER_THREADS: u32 = 92u32;
pub const INTERNET_OPTION_ACTIVITY_ID: u32 = 185u32;
pub const INTERNET_OPTION_ALLOW_FAILED_CONNECT_CONTENT: u32 = 110u32;
pub const INTERNET_OPTION_ALLOW_INSECURE_FALLBACK: u32 = 161u32;
pub const INTERNET_OPTION_ALTER_IDENTITY: u32 = 80u32;
pub const INTERNET_OPTION_APP_CACHE: u32 = 130u32;
pub const INTERNET_OPTION_ASYNC: u32 = 30u32;
pub const INTERNET_OPTION_ASYNC_ID: u32 = 15u32;
pub const INTERNET_OPTION_ASYNC_PRIORITY: u32 = 16u32;
pub const INTERNET_OPTION_AUTH_FLAGS: u32 = 85u32;
pub const INTERNET_OPTION_AUTH_SCHEME_SELECTED: u32 = 183u32;
pub const INTERNET_OPTION_AUTODIAL_CONNECTION: u32 = 83u32;
pub const INTERNET_OPTION_AUTODIAL_HWND: u32 = 112u32;
pub const INTERNET_OPTION_AUTODIAL_MODE: u32 = 82u32;
pub const INTERNET_OPTION_BACKGROUND_CONNECTIONS: u32 = 121u32;
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: u32 = 64u32;
pub const INTERNET_OPTION_CACHE_ENTRY_EXTRA_DATA: u32 = 139u32;
pub const INTERNET_OPTION_CACHE_PARTITION: u32 = 111u32;
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: u32 = 27u32;
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: u32 = 69u32;
pub const INTERNET_OPTION_CALLBACK: u32 = 1u32;
pub const INTERNET_OPTION_CALLBACK_FILTER: u32 = 54u32;
pub const INTERNET_OPTION_CALLER_MODULE: u32 = 192u32;
pub const INTERNET_OPTION_CANCEL_CACHE_WRITE: u32 = 182u32;
pub const INTERNET_OPTION_CERT_ERROR_FLAGS: u32 = 98u32;
pub const INTERNET_OPTION_CHUNK_ENCODE_REQUEST: u32 = 150u32;
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: u32 = 84u32;
pub const INTERNET_OPTION_CLIENT_CERT_ISSUER_LIST: u32 = 153u32;
pub const INTERNET_OPTION_CM_HANDLE_COPY_REF: u32 = 118u32;
pub const INTERNET_OPTION_CODEPAGE: u32 = 68u32;
pub const INTERNET_OPTION_CODEPAGE_EXTRA: u32 = 101u32;
pub const INTERNET_OPTION_CODEPAGE_PATH: u32 = 100u32;
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: u32 = 147u32;
pub const INTERNET_OPTION_CONNECTED_STATE: u32 = 50u32;
pub const INTERNET_OPTION_CONNECTION_FILTER: u32 = 162u32;
pub const INTERNET_OPTION_CONNECTION_INFO: u32 = 120u32;
pub const INTERNET_OPTION_CONNECT_BACKOFF: u32 = 4u32;
pub const INTERNET_OPTION_CONNECT_LIMIT: u32 = 46u32;
pub const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3u32;
pub const INTERNET_OPTION_CONNECT_TIME: u32 = 55u32;
pub const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2u32;
pub const INTERNET_OPTION_CONTEXT_VALUE: u32 = 45u32;
pub const INTERNET_OPTION_CONTEXT_VALUE_OLD: u32 = 10u32;
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: u32 = 6u32;
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: u32 = 5u32;
pub const INTERNET_OPTION_COOKIES_3RD_PARTY: u32 = 86u32;
pub const INTERNET_OPTION_COOKIES_APPLY_HOST_ONLY: u32 = 179u32;
pub const INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL: u32 = 187u32;
pub const INTERNET_OPTION_DATAFILE_EXT: u32 = 96u32;
pub const INTERNET_OPTION_DATAFILE_NAME: u32 = 33u32;
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: u32 = 8u32;
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: u32 = 7u32;
pub const INTERNET_OPTION_DEPENDENCY_HANDLE: u32 = 131u32;
pub const INTERNET_OPTION_DETECT_POST_SEND: u32 = 71u32;
pub const INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO: u32 = 67u32;
pub const INTERNET_OPTION_DIGEST_AUTH_UNLOAD: u32 = 76u32;
pub const INTERNET_OPTION_DISABLE_AUTODIAL: u32 = 70u32;
pub const INTERNET_OPTION_DISABLE_INSECURE_FALLBACK: u32 = 160u32;
pub const INTERNET_OPTION_DISABLE_NTLM_PREAUTH: u32 = 72u32;
pub const INTERNET_OPTION_DISABLE_PASSPORT_AUTH: u32 = 87u32;
pub const INTERNET_OPTION_DISABLE_PROXY_LINK_LOCAL_NAME_RESOLUTION: u32 = 190u32;
pub const INTERNET_OPTION_DISALLOW_PREMATURE_EOF: u32 = 137u32;
pub const INTERNET_OPTION_DISCONNECTED_TIMEOUT: u32 = 49u32;
pub const INTERNET_OPTION_DOWNLOAD_MODE: u32 = 116u32;
pub const INTERNET_OPTION_DOWNLOAD_MODE_HANDLE: u32 = 165u32;
pub const INTERNET_OPTION_DO_NOT_TRACK: u32 = 123u32;
pub const INTERNET_OPTION_DUO_USED: u32 = 149u32;
pub const INTERNET_OPTION_EDGE_COOKIES: u32 = 166u32;
pub const INTERNET_OPTION_EDGE_COOKIES_TEMP: u32 = 175u32;
pub const INTERNET_OPTION_EDGE_MODE: u32 = 180u32;
pub const INTERNET_OPTION_ENABLE_DUO: u32 = 148u32;
pub const INTERNET_OPTION_ENABLE_HEADER_CALLBACKS: u32 = 168u32;
pub const INTERNET_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 148u32;
pub const INTERNET_OPTION_ENABLE_PASSPORT_AUTH: u32 = 90u32;
pub const INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ: u32 = 122u32;
pub const INTERNET_OPTION_ENABLE_TEST_SIGNING: u32 = 189u32;
pub const INTERNET_OPTION_ENABLE_WBOEXT: u32 = 158u32;
pub const INTERNET_OPTION_ENABLE_ZLIB_DEFLATE: u32 = 173u32;
pub const INTERNET_OPTION_ENCODE_EXTRA: u32 = 155u32;
pub const INTERNET_OPTION_ENCODE_FALLBACK_FOR_REDIRECT_URI: u32 = 174u32;
pub const INTERNET_OPTION_END_BROWSER_SESSION: u32 = 42u32;
pub const INTERNET_OPTION_ENTERPRISE_CONTEXT: u32 = 159u32;
pub const INTERNET_OPTION_ERROR_MASK: u32 = 62u32;
pub const INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT: u32 = 89u32;
pub const INTERNET_OPTION_EXTENDED_CALLBACKS: u32 = 108u32;
pub const INTERNET_OPTION_EXTENDED_ERROR: u32 = 24u32;
pub const INTERNET_OPTION_FAIL_ON_CACHE_WRITE_ERROR: u32 = 115u32;
pub const INTERNET_OPTION_FALSE_START: u32 = 141u32;
pub const INTERNET_OPTION_FLUSH_STATE: u32 = 135u32;
pub const INTERNET_OPTION_FORCE_DECODE: u32 = 178u32;
pub const INTERNET_OPTION_FROM_CACHE_TIMEOUT: u32 = 63u32;
pub const INTERNET_OPTION_GLOBAL_CALLBACK: u32 = 188u32;
pub const INTERNET_OPTION_HANDLE_TYPE: u32 = 9u32;
pub const INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS: u32 = 91u32;
pub const INTERNET_OPTION_HSTS: u32 = 157u32;
pub const INTERNET_OPTION_HTTP_09: u32 = 191u32;
pub const INTERNET_OPTION_HTTP_DECODING: u32 = 65u32;
pub const INTERNET_OPTION_HTTP_PROTOCOL_USED: u32 = 149u32;
pub const INTERNET_OPTION_HTTP_VERSION: u32 = 59u32;
pub const INTERNET_OPTION_IDENTITY: u32 = 78u32;
pub const INTERNET_OPTION_IDLE_STATE: u32 = 51u32;
pub const INTERNET_OPTION_IDN: u32 = 102u32;
pub const INTERNET_OPTION_IGNORE_CERT_ERROR_FLAGS: u32 = 99u32;
pub const INTERNET_OPTION_IGNORE_OFFLINE: u32 = 77u32;
pub const INTERNET_OPTION_KEEP_CONNECTION: u32 = 22u32;
pub const INTERNET_OPTION_LINE_STATE: u32 = 50u32;
pub const INTERNET_OPTION_LISTEN_TIMEOUT: u32 = 11u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_PROXY: u32 = 103u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_SERVER: u32 = 73u32;
pub const INTERNET_OPTION_MAX_QUERY_BUFFER_SIZE: u32 = 140u32;
pub const INTERNET_OPTION_NET_SPEED: u32 = 61u32;
pub const INTERNET_OPTION_NOCACHE_WRITE_IN_PRIVATE: u32 = 184u32;
pub const INTERNET_OPTION_NOTIFY_SENDING_COOKIE: u32 = 152u32;
pub const INTERNET_OPTION_NO_HTTP_SERVER_AUTH: u32 = 167u32;
pub const INTERNET_OPTION_OFFLINE_MODE: u32 = 26u32;
pub const INTERNET_OPTION_OFFLINE_SEMANTICS: u32 = 52u32;
pub const INTERNET_OPTION_OFFLINE_TIMEOUT: u32 = 49u32;
pub const INTERNET_OPTION_OPT_IN_WEAK_SIGNATURE: u32 = 176u32;
pub const INTERNET_OPTION_ORIGINAL_CONNECT_FLAGS: u32 = 97u32;
pub const INTERNET_OPTION_PARENT_HANDLE: u32 = 21u32;
pub const INTERNET_OPTION_PARSE_LINE_FOLDING: u32 = 177u32;
pub const INTERNET_OPTION_PASSWORD: u32 = 29u32;
pub const INTERNET_OPTION_PER_CONNECTION_OPTION: u32 = 75u32;
pub const INTERNET_OPTION_POLICY: u32 = 48u32;
pub const INTERNET_OPTION_PRESERVE_REFERER_ON_HTTPS_TO_HTTP_REDIRECT: u32 = 170u32;
pub const INTERNET_OPTION_PRESERVE_REQUEST_SERVER_CREDENTIALS_ON_REDIRECT: u32 = 169u32;
pub const INTERNET_OPTION_PROXY: u32 = 38u32;
pub const INTERNET_OPTION_PROXY_AUTH_SCHEME: u32 = 144u32;
pub const INTERNET_OPTION_PROXY_CREDENTIALS: u32 = 107u32;
pub const INTERNET_OPTION_PROXY_FROM_REQUEST: u32 = 109u32;
pub const INTERNET_OPTION_PROXY_PASSWORD: u32 = 44u32;
pub const INTERNET_OPTION_PROXY_SETTINGS_CHANGED: u32 = 95u32;
pub const INTERNET_OPTION_PROXY_USERNAME: u32 = 43u32;
pub const INTERNET_OPTION_READ_BUFFER_SIZE: u32 = 12u32;
pub const INTERNET_OPTION_RECEIVE_THROUGHPUT: u32 = 57u32;
pub const INTERNET_OPTION_RECEIVE_TIMEOUT: u32 = 6u32;
pub const INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 163u32;
pub const INTERNET_OPTION_REFRESH: u32 = 37u32;
pub const INTERNET_OPTION_REMOVE_IDENTITY: u32 = 79u32;
pub const INTERNET_OPTION_REQUEST_ANNOTATION: u32 = 193u32;
pub const INTERNET_OPTION_REQUEST_ANNOTATION_MAX_LENGTH: u32 = 64000u32;
pub const INTERNET_OPTION_REQUEST_FLAGS: u32 = 23u32;
pub const INTERNET_OPTION_REQUEST_PRIORITY: u32 = 58u32;
pub const INTERNET_OPTION_REQUEST_TIMES: u32 = 186u32;
pub const INTERNET_OPTION_RESET: u32 = 154u32;
pub const INTERNET_OPTION_RESET_URLCACHE_SESSION: u32 = 60u32;
pub const INTERNET_OPTION_RESPONSE_RESUMABLE: u32 = 117u32;
pub const INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS: u32 = 93u32;
pub const INTERNET_OPTION_SECONDARY_CACHE_KEY: u32 = 53u32;
pub const INTERNET_OPTION_SECURE_FAILURE: u32 = 151u32;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE: u32 = 35u32;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32u32;
pub const INTERNET_OPTION_SECURITY_CONNECTION_INFO: u32 = 66u32;
pub const INTERNET_OPTION_SECURITY_FLAGS: u32 = 31u32;
pub const INTERNET_OPTION_SECURITY_KEY_BITNESS: u32 = 36u32;
pub const INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT: u32 = 47u32;
pub const INTERNET_OPTION_SEND_THROUGHPUT: u32 = 56u32;
pub const INTERNET_OPTION_SEND_TIMEOUT: u32 = 5u32;
pub const INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY: u32 = 88u32;
pub const INTERNET_OPTION_SERVER_ADDRESS_INFO: u32 = 156u32;
pub const INTERNET_OPTION_SERVER_AUTH_SCHEME: u32 = 143u32;
pub const INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 105u32;
pub const INTERNET_OPTION_SERVER_CREDENTIALS: u32 = 113u32;
pub const INTERNET_OPTION_SESSION_START_TIME: u32 = 106u32;
pub const INTERNET_OPTION_SETTINGS_CHANGED: u32 = 39u32;
pub const INTERNET_OPTION_SET_IN_PRIVATE: u32 = 164u32;
pub const INTERNET_OPTION_SOCKET_NODELAY: u32 = 129u32;
pub const INTERNET_OPTION_SOCKET_NOTIFICATION_IOCTL: u32 = 138u32;
pub const INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH: u32 = 94u32;
pub const INTERNET_OPTION_SOURCE_PORT: u32 = 146u32;
pub const INTERNET_OPTION_SUPPRESS_BEHAVIOR: u32 = 81u32;
pub const INTERNET_OPTION_SUPPRESS_SERVER_AUTH: u32 = 104u32;
pub const INTERNET_OPTION_SYNC_MODE_AUTOMATIC_SESSION_DISABLED: u32 = 172u32;
pub const INTERNET_OPTION_TCP_FAST_OPEN: u32 = 171u32;
pub const INTERNET_OPTION_TIMED_CONNECTION_LIMIT_BYPASS: u32 = 133u32;
pub const INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 181u32;
pub const INTERNET_OPTION_TUNNEL_ONLY: u32 = 145u32;
pub const INTERNET_OPTION_UNLOAD_NOTIFY_EVENT: u32 = 128u32;
pub const INTERNET_OPTION_UPGRADE_TO_WEB_SOCKET: u32 = 126u32;
pub const INTERNET_OPTION_URL: u32 = 34u32;
pub const INTERNET_OPTION_USERNAME: u32 = 28u32;
pub const INTERNET_OPTION_USER_AGENT: u32 = 41u32;
pub const INTERNET_OPTION_USER_PASS_SERVER_ONLY: u32 = 142u32;
pub const INTERNET_OPTION_USE_FIRST_AVAILABLE_CONNECTION: u32 = 132u32;
pub const INTERNET_OPTION_USE_MODIFIED_HEADER_FILTER: u32 = 124u32;
pub const INTERNET_OPTION_VERSION: u32 = 40u32;
pub const INTERNET_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: u32 = 134u32;
pub const INTERNET_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: u32 = 127u32;
pub const INTERNET_OPTION_WPAD_SLEEP: u32 = 114u32;
pub const INTERNET_OPTION_WRITE_BUFFER_SIZE: u32 = 13u32;
pub const INTERNET_OPTION_WWA_MODE: u32 = 125u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_PER_CONN(pub u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME: INTERNET_PER_CONN = INTERNET_PER_CONN(8u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(9u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS: INTERNET_PER_CONN = INTERNET_PER_CONN(7u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(6u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(4u32);
pub const INTERNET_PER_CONN_AUTODISCOVERY_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(5u32);
pub const INTERNET_PER_CONN_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(1u32);
pub const INTERNET_PER_CONN_FLAGS_UI: u32 = 10u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTIONA {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONA_0,
}
impl Default for INTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_PER_CONN_OPTIONA_0 {
    pub dwValue: u32,
    pub pszValue: windows_core::PSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
impl Default for INTERNET_PER_CONN_OPTIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERNET_PER_CONN_OPTIONW {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONW_0,
}
impl Default for INTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTERNET_PER_CONN_OPTIONW_0 {
    pub dwValue: u32,
    pub pszValue: windows_core::PWSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
impl Default for INTERNET_PER_CONN_OPTIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_PER_CONN_OPTION_LISTA {
    pub dwSize: u32,
    pub pszConnection: windows_core::PSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONA,
}
impl Default for INTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_PER_CONN_OPTION_LISTW {
    pub dwSize: u32,
    pub pszConnection: windows_core::PWSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONW,
}
impl Default for INTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_PER_CONN_PROXY_BYPASS: INTERNET_PER_CONN = INTERNET_PER_CONN(3u32);
pub const INTERNET_PER_CONN_PROXY_SERVER: INTERNET_PER_CONN = INTERNET_PER_CONN(2u32);
pub const INTERNET_PREFETCH_ABORTED: u32 = 2u32;
pub const INTERNET_PREFETCH_COMPLETE: u32 = 1u32;
pub const INTERNET_PREFETCH_PROGRESS: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_PREFETCH_STATUS {
    pub dwStatus: u32,
    pub dwSize: u32,
}
pub const INTERNET_PRIORITY_FOREGROUND: u32 = 1000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_PROXY_INFO {
    pub dwAccessType: INTERNET_ACCESS_TYPE,
    pub lpszProxy: *mut i8,
    pub lpszProxyBypass: *mut i8,
}
impl Default for INTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INTERNET_RAS_INSTALLED: INTERNET_CONNECTION = INTERNET_CONNECTION(16u32);
pub const INTERNET_REQFLAG_ASYNC: u32 = 2u32;
pub const INTERNET_REQFLAG_CACHE_WRITE_DISABLED: u32 = 64u32;
pub const INTERNET_REQFLAG_FROM_APP_CACHE: u32 = 256u32;
pub const INTERNET_REQFLAG_FROM_CACHE: u32 = 1u32;
pub const INTERNET_REQFLAG_NET_TIMEOUT: u32 = 128u32;
pub const INTERNET_REQFLAG_NO_HEADERS: u32 = 8u32;
pub const INTERNET_REQFLAG_PASSIVE: u32 = 16u32;
pub const INTERNET_REQFLAG_VIA_PROXY: u32 = 4u32;
pub const INTERNET_RFC1123_BUFSIZE: u32 = 30u32;
pub const INTERNET_RFC1123_FORMAT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_SCHEME(pub i32);
pub const INTERNET_SCHEME_DEFAULT: INTERNET_SCHEME = INTERNET_SCHEME(0i32);
pub const INTERNET_SCHEME_FILE: INTERNET_SCHEME = INTERNET_SCHEME(5i32);
pub const INTERNET_SCHEME_FIRST: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
pub const INTERNET_SCHEME_FTP: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
pub const INTERNET_SCHEME_GOPHER: INTERNET_SCHEME = INTERNET_SCHEME(2i32);
pub const INTERNET_SCHEME_HTTP: INTERNET_SCHEME = INTERNET_SCHEME(3i32);
pub const INTERNET_SCHEME_HTTPS: INTERNET_SCHEME = INTERNET_SCHEME(4i32);
pub const INTERNET_SCHEME_JAVASCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(9i32);
pub const INTERNET_SCHEME_LAST: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
pub const INTERNET_SCHEME_MAILTO: INTERNET_SCHEME = INTERNET_SCHEME(7i32);
pub const INTERNET_SCHEME_NEWS: INTERNET_SCHEME = INTERNET_SCHEME(6i32);
pub const INTERNET_SCHEME_PARTIAL: INTERNET_SCHEME = INTERNET_SCHEME(-2i32);
pub const INTERNET_SCHEME_RES: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
pub const INTERNET_SCHEME_SOCKS: INTERNET_SCHEME = INTERNET_SCHEME(8i32);
pub const INTERNET_SCHEME_UNKNOWN: INTERNET_SCHEME = INTERNET_SCHEME(-1i32);
pub const INTERNET_SCHEME_VBSCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(10i32);
#[repr(C)]
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_SECURITY_CONNECTION_INFO {
    pub dwSize: u32,
    pub fSecure: windows_core::BOOL,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERNET_SECURITY_INFO {
    pub dwSize: u32,
    pub pCertificate: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub pcCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
    pub pcUnverifiedCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub channelBindingToken: super::super::Security::Authentication::Identity::SecPkgContext_Bindings,
}
#[cfg(all(feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl Default for INTERNET_SECURITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_SERVER_CONNECTION_STATE {
    pub lpcwszHostName: windows_core::PCWSTR,
    pub fProxy: windows_core::BOOL,
    pub dwCounter: u32,
    pub dwConnectionLimit: u32,
    pub dwAvailableCreates: u32,
    pub dwAvailableKeepAlives: u32,
    pub dwActiveConnections: u32,
    pub dwWaiters: u32,
}
pub const INTERNET_SERVICE_FTP: u32 = 1u32;
pub const INTERNET_SERVICE_GOPHER: u32 = 2u32;
pub const INTERNET_SERVICE_HTTP: u32 = 3u32;
pub const INTERNET_SERVICE_URL: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNET_STATE(pub u32);
pub const INTERNET_STATE_BUSY: INTERNET_STATE = INTERNET_STATE(512u32);
pub const INTERNET_STATE_CONNECTED: INTERNET_STATE = INTERNET_STATE(1u32);
pub const INTERNET_STATE_DISCONNECTED: INTERNET_STATE = INTERNET_STATE(2u32);
pub const INTERNET_STATE_DISCONNECTED_BY_USER: INTERNET_STATE = INTERNET_STATE(16u32);
pub const INTERNET_STATE_IDLE: INTERNET_STATE = INTERNET_STATE(256u32);
pub const INTERNET_STATUS_CLOSING_CONNECTION: u32 = 50u32;
pub const INTERNET_STATUS_CONNECTED_TO_SERVER: u32 = 21u32;
pub const INTERNET_STATUS_CONNECTING_TO_SERVER: u32 = 20u32;
pub const INTERNET_STATUS_CONNECTION_CLOSED: u32 = 51u32;
pub const INTERNET_STATUS_COOKIE: u32 = 430u32;
pub const INTERNET_STATUS_COOKIE_HISTORY: u32 = 327u32;
pub const INTERNET_STATUS_COOKIE_RECEIVED: u32 = 321u32;
pub const INTERNET_STATUS_COOKIE_SENT: u32 = 320u32;
pub const INTERNET_STATUS_CTL_RESPONSE_RECEIVED: u32 = 42u32;
pub const INTERNET_STATUS_DETECTING_PROXY: u32 = 80u32;
pub const INTERNET_STATUS_END_BROWSER_SESSION: u32 = 420u32;
pub const INTERNET_STATUS_FILTER_CLOSED: u32 = 512u32;
pub const INTERNET_STATUS_FILTER_CLOSING: u32 = 256u32;
pub const INTERNET_STATUS_FILTER_CONNECTED: u32 = 8u32;
pub const INTERNET_STATUS_FILTER_CONNECTING: u32 = 4u32;
pub const INTERNET_STATUS_FILTER_HANDLE_CLOSING: u32 = 2048u32;
pub const INTERNET_STATUS_FILTER_HANDLE_CREATED: u32 = 1024u32;
pub const INTERNET_STATUS_FILTER_PREFETCH: u32 = 4096u32;
pub const INTERNET_STATUS_FILTER_RECEIVED: u32 = 128u32;
pub const INTERNET_STATUS_FILTER_RECEIVING: u32 = 64u32;
pub const INTERNET_STATUS_FILTER_REDIRECT: u32 = 8192u32;
pub const INTERNET_STATUS_FILTER_RESOLVED: u32 = 2u32;
pub const INTERNET_STATUS_FILTER_RESOLVING: u32 = 1u32;
pub const INTERNET_STATUS_FILTER_SENDING: u32 = 16u32;
pub const INTERNET_STATUS_FILTER_SENT: u32 = 32u32;
pub const INTERNET_STATUS_FILTER_STATE_CHANGE: u32 = 16384u32;
pub const INTERNET_STATUS_HANDLE_CLOSING: u32 = 70u32;
pub const INTERNET_STATUS_HANDLE_CREATED: u32 = 60u32;
pub const INTERNET_STATUS_INTERMEDIATE_RESPONSE: u32 = 120u32;
pub const INTERNET_STATUS_NAME_RESOLVED: u32 = 11u32;
pub const INTERNET_STATUS_P3P_HEADER: u32 = 325u32;
pub const INTERNET_STATUS_P3P_POLICYREF: u32 = 326u32;
pub const INTERNET_STATUS_PREFETCH: u32 = 43u32;
pub const INTERNET_STATUS_PRIVACY_IMPACTED: u32 = 324u32;
pub const INTERNET_STATUS_PROXY_CREDENTIALS: u32 = 400u32;
pub const INTERNET_STATUS_RECEIVING_RESPONSE: u32 = 40u32;
pub const INTERNET_STATUS_REDIRECT: u32 = 110u32;
pub const INTERNET_STATUS_REQUEST_COMPLETE: u32 = 100u32;
pub const INTERNET_STATUS_REQUEST_HEADERS_SET: u32 = 329u32;
pub const INTERNET_STATUS_REQUEST_SENT: u32 = 31u32;
pub const INTERNET_STATUS_RESOLVING_NAME: u32 = 10u32;
pub const INTERNET_STATUS_RESPONSE_HEADERS_SET: u32 = 330u32;
pub const INTERNET_STATUS_RESPONSE_RECEIVED: u32 = 41u32;
pub const INTERNET_STATUS_SENDING_COOKIE: u32 = 328u32;
pub const INTERNET_STATUS_SENDING_REQUEST: u32 = 30u32;
pub const INTERNET_STATUS_SERVER_CONNECTION_STATE: u32 = 410u32;
pub const INTERNET_STATUS_SERVER_CREDENTIALS: u32 = 401u32;
pub const INTERNET_STATUS_STATE_CHANGE: u32 = 200u32;
pub const INTERNET_STATUS_USER_INPUT_REQUIRED: u32 = 140u32;
pub const INTERNET_SUPPRESS_COOKIE_PERSIST: u32 = 3u32;
pub const INTERNET_SUPPRESS_COOKIE_PERSIST_RESET: u32 = 4u32;
pub const INTERNET_SUPPRESS_COOKIE_POLICY: u32 = 1u32;
pub const INTERNET_SUPPRESS_COOKIE_POLICY_RESET: u32 = 2u32;
pub const INTERNET_SUPPRESS_RESET_ALL: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERNET_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
windows_core::imp::define_interface!(IProofOfPossessionCookieInfoManager, IProofOfPossessionCookieInfoManager_Vtbl, 0xcdaece56_4edf_43df_b113_88e4556fa1bb);
windows_core::imp::interface_hierarchy!(IProofOfPossessionCookieInfoManager, windows_core::IUnknown);
impl IProofOfPossessionCookieInfoManager {
    pub unsafe fn GetCookieInfoForUri<P0>(&self, uri: P0, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCookieInfoForUri)(windows_core::Interface::as_raw(self), uri.param().abi(), cookieinfocount as _, cookieinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCookieInfoForUri: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT,
}
pub trait IProofOfPossessionCookieInfoManager_Impl: windows_core::IUnknownImpl {
    fn GetCookieInfoForUri(&self, uri: &windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>;
}
impl IProofOfPossessionCookieInfoManager_Vtbl {
    pub const fn new<Identity: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCookieInfoForUri<Identity: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProofOfPossessionCookieInfoManager_Impl::GetCookieInfoForUri(this, core::mem::transmute(&uri), core::mem::transmute_copy(&cookieinfocount), core::mem::transmute_copy(&cookieinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCookieInfoForUri: GetCookieInfoForUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProofOfPossessionCookieInfoManager {}
windows_core::imp::define_interface!(IProofOfPossessionCookieInfoManager2, IProofOfPossessionCookieInfoManager2_Vtbl, 0x15e41407_b42f_4ae7_9966_34a087b2d713);
windows_core::imp::interface_hierarchy!(IProofOfPossessionCookieInfoManager2, windows_core::IUnknown);
impl IProofOfPossessionCookieInfoManager2 {
    pub unsafe fn GetCookieInfoWithUriForAccount<P0, P1>(&self, webaccount: P0, uri: P1, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCookieInfoWithUriForAccount)(windows_core::Interface::as_raw(self), webaccount.param().abi(), uri.param().abi(), cookieinfocount as _, cookieinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCookieInfoWithUriForAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT,
}
pub trait IProofOfPossessionCookieInfoManager2_Impl: windows_core::IUnknownImpl {
    fn GetCookieInfoWithUriForAccount(&self, webaccount: windows_core::Ref<windows_core::IInspectable>, uri: &windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::Result<()>;
}
impl IProofOfPossessionCookieInfoManager2_Vtbl {
    pub const fn new<Identity: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Identity: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, webaccount: *mut core::ffi::c_void, uri: windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProofOfPossessionCookieInfoManager2_Impl::GetCookieInfoWithUriForAccount(this, core::mem::transmute_copy(&webaccount), core::mem::transmute(&uri), core::mem::transmute_copy(&cookieinfocount), core::mem::transmute_copy(&cookieinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCookieInfoWithUriForAccount: GetCookieInfoWithUriForAccount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProofOfPossessionCookieInfoManager2 {}
pub const IRF_ASYNC: u32 = 1u32;
pub const IRF_NO_WAIT: u32 = 8u32;
pub const IRF_SYNC: u32 = 4u32;
pub const IRF_USE_CONTEXT: u32 = 8u32;
pub const ISO_FORCE_DISCONNECTED: u32 = 1u32;
pub const ISO_FORCE_OFFLINE: u32 = 1u32;
pub const ISO_GLOBAL: u32 = 1u32;
pub const ISO_REGISTRY: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IncomingCookieState {
    pub cSession: i32,
    pub cPersistent: i32,
    pub cAccepted: i32,
    pub cLeashed: i32,
    pub cDowngraded: i32,
    pub cBlocked: i32,
    pub pszLocation: windows_core::PCSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InternetCookieHistory {
    pub fAccepted: windows_core::BOOL,
    pub fLeashed: windows_core::BOOL,
    pub fDowngraded: windows_core::BOOL,
    pub fRejected: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InternetCookieState(pub i32);
pub const LOCAL_NAMESPACE_PREFIX: windows_core::PCSTR = windows_core::s!("Local\\");
pub const LOCAL_NAMESPACE_PREFIX_W: windows_core::PCWSTR = windows_core::w!("Local\\");
pub type LPINTERNET_STATUS_CALLBACK = Option<unsafe extern "system" fn(hinternet: *const core::ffi::c_void, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *const core::ffi::c_void, dwstatusinformationlength: u32)>;
pub const MAX_CACHE_ENTRY_INFO_SIZE: u32 = 4096u32;
pub const MAX_GOPHER_ATTRIBUTE_NAME: u32 = 128u32;
pub const MAX_GOPHER_CATEGORY_NAME: u32 = 128u32;
pub const MAX_GOPHER_DISPLAY_TEXT: u32 = 128u32;
pub const MAX_GOPHER_HOST_NAME: u32 = 256u32;
pub const MAX_GOPHER_SELECTOR_TEXT: u32 = 256u32;
pub const MIN_GOPHER_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const MUST_REVALIDATE_CACHE_ENTRY: u32 = 256u32;
pub const MaxPrivacySettings: u32 = 16384u32;
pub const NORMAL_CACHE_ENTRY: u32 = 1u32;
pub const NameResolutionEnd: REQUEST_TIMES = REQUEST_TIMES(1i32);
pub const NameResolutionStart: REQUEST_TIMES = REQUEST_TIMES(0i32);
pub const OTHER_USER_CACHE_ENTRY: u32 = 8388608u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OutgoingCookieState {
    pub cSent: i32,
    pub cSuppressed: i32,
    pub pszLocation: windows_core::PCSTR,
}
pub const PENDING_DELETE_CACHE_ENTRY: u32 = 4194304u32;
pub type PFN_AUTH_NOTIFY = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
pub type PFN_DIAL_HANDLER = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCSTR, param2: u32, param3: *mut u32) -> u32>;
pub const POLICY_EXTENSION_TYPE_NONE: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(0i32);
pub const POLICY_EXTENSION_TYPE_WINHTTP: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(1i32);
pub const POLICY_EXTENSION_TYPE_WININET: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(2i32);
pub const POLICY_EXTENSION_VERSION1: HTTP_POLICY_EXTENSION_VERSION = HTTP_POLICY_EXTENSION_VERSION(1i32);
pub const POST_CHECK_CACHE_ENTRY: u32 = 536870912u32;
pub const POST_RESPONSE_CACHE_ENTRY: u32 = 67108864u32;
pub const PRIVACY_IMPACTED_CACHE_ENTRY: u32 = 33554432u32;
pub const PRIVACY_MODE_CACHE_ENTRY: u32 = 131072u32;
pub const PRIVACY_TEMPLATE_ADVANCED: u32 = 101u32;
pub const PRIVACY_TEMPLATE_CUSTOM: u32 = 100u32;
pub const PRIVACY_TEMPLATE_HIGH: u32 = 1u32;
pub const PRIVACY_TEMPLATE_LOW: u32 = 5u32;
pub const PRIVACY_TEMPLATE_MAX: u32 = 5u32;
pub const PRIVACY_TEMPLATE_MEDIUM: u32 = 3u32;
pub const PRIVACY_TEMPLATE_MEDIUM_HIGH: u32 = 2u32;
pub const PRIVACY_TEMPLATE_MEDIUM_LOW: u32 = 4u32;
pub const PRIVACY_TEMPLATE_NO_COOKIES: u32 = 0u32;
pub const PRIVACY_TYPE_FIRST_PARTY: u32 = 0u32;
pub const PRIVACY_TYPE_THIRD_PARTY: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROXY_AUTO_DETECT_TYPE(pub u32);
impl PROXY_AUTO_DETECT_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROXY_AUTO_DETECT_TYPE_DHCP: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(1u32);
pub const PROXY_AUTO_DETECT_TYPE_DNS_A: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(2u32);
pub const PROXY_TYPE_AUTO_DETECT: u32 = 8u32;
pub const PROXY_TYPE_AUTO_PROXY_URL: u32 = 4u32;
pub const PROXY_TYPE_DIRECT: u32 = 1u32;
pub const PROXY_TYPE_PROXY: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ProofOfPossessionCookieInfo {
    pub name: windows_core::PWSTR,
    pub data: windows_core::PWSTR,
    pub flags: u32,
    pub p3pHeader: windows_core::PWSTR,
}
pub const ProofOfPossessionCookieInfoManager: windows_core::GUID = windows_core::GUID::from_u128(0xa9927f85_a304_4390_8b23_a75f1c668600);
pub const REDIRECT_CACHE_ENTRY: u32 = 2048u32;
pub const REGSTR_DIAL_AUTOCONNECT: windows_core::PCSTR = windows_core::s!("AutoConnect");
pub const REGSTR_LEASH_LEGACY_COOKIES: windows_core::PCSTR = windows_core::s!("LeashLegacyCookies");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REQUEST_TIMES(pub i32);
pub const SECURITY_FLAG_128BIT: u32 = 536870912u32;
pub const SECURITY_FLAG_40BIT: u32 = 268435456u32;
pub const SECURITY_FLAG_56BIT: u32 = 1073741824u32;
pub const SECURITY_FLAG_FORTEZZA: u32 = 134217728u32;
pub const SECURITY_FLAG_IETFSSL4: u32 = 32u32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
pub const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 128u32;
pub const SECURITY_FLAG_IGNORE_WEAK_SIGNATURE: u32 = 65536u32;
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: u32 = 512u32;
pub const SECURITY_FLAG_NORMALBITNESS: u32 = 268435456u32;
pub const SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE: u32 = 131072u32;
pub const SECURITY_FLAG_PCT: u32 = 8u32;
pub const SECURITY_FLAG_PCT4: u32 = 16u32;
pub const SECURITY_FLAG_SSL: u32 = 2u32;
pub const SECURITY_FLAG_SSL3: u32 = 4u32;
pub const SECURITY_FLAG_UNKNOWNBIT: u32 = 2147483648u32;
pub const SHORTPATH_CACHE_ENTRY: u32 = 512u32;
pub const SPARSE_CACHE_ENTRY: u32 = 65536u32;
pub const STATIC_CACHE_ENTRY: u32 = 128u32;
pub const STICKY_CACHE_ENTRY: u32 = 4u32;
pub const TLSHandshakeEnd: REQUEST_TIMES = REQUEST_TIMES(5i32);
pub const TLSHandshakeStart: REQUEST_TIMES = REQUEST_TIMES(4i32);
pub const TRACK_OFFLINE_CACHE_ENTRY: u32 = 16u32;
pub const TRACK_ONLINE_CACHE_ENTRY: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct URLCACHE_ENTRY_INFO {
    pub pwszSourceUrlName: windows_core::PWSTR,
    pub pwszLocalFileName: windows_core::PWSTR,
    pub dwCacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub ftLastModifiedTime: super::super::Foundation::FILETIME,
    pub ftExpireTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastSyncTime: super::super::Foundation::FILETIME,
    pub pbHeaderInfo: *mut u8,
    pub cbHeaderInfoSize: u32,
    pub pbExtraData: *mut u8,
    pub cbExtraDataSize: u32,
}
impl Default for URLCACHE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const URLHISTORY_CACHE_ENTRY: u32 = 2097152u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URL_CACHE_LIMIT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URL_COMPONENTSA {
    pub dwStructSize: u32,
    pub lpszScheme: windows_core::PSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: windows_core::PSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: windows_core::PSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_core::PSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_core::PSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_core::PSTR,
    pub dwExtraInfoLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct URL_COMPONENTSW {
    pub dwStructSize: u32,
    pub lpszScheme: windows_core::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: windows_core::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: windows_core::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: windows_core::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: windows_core::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: windows_core::PWSTR,
    pub dwExtraInfoLength: u32,
}
pub const UrlCacheLimitTypeAppContainer: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(2i32);
pub const UrlCacheLimitTypeAppContainerTotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(3i32);
pub const UrlCacheLimitTypeIE: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(0i32);
pub const UrlCacheLimitTypeIETotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(1i32);
pub const UrlCacheLimitTypeNum: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(4i32);
pub const WININET_API_FLAG_ASYNC: u32 = 1u32;
pub const WININET_API_FLAG_SYNC: u32 = 4u32;
pub const WININET_API_FLAG_USE_CONTEXT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WININET_PROXY_INFO {
    pub fProxy: windows_core::BOOL,
    pub fBypass: windows_core::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: windows_core::PWSTR,
    pub ProxyPort: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WININET_PROXY_INFO_LIST {
    pub dwProxyInfoCount: u32,
    pub pProxyInfo: *mut WININET_PROXY_INFO,
}
impl Default for WININET_PROXY_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WININET_SYNC_MODE(pub i32);
pub const WININET_SYNC_MODE_ALWAYS: WININET_SYNC_MODE = WININET_SYNC_MODE(3i32);
pub const WININET_SYNC_MODE_AUTOMATIC: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
pub const WININET_SYNC_MODE_DEFAULT: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
pub const WININET_SYNC_MODE_NEVER: WININET_SYNC_MODE = WININET_SYNC_MODE(0i32);
pub const WININET_SYNC_MODE_ONCE_PER_SESSION: WININET_SYNC_MODE = WININET_SYNC_MODE(2i32);
pub const WININET_SYNC_MODE_ON_EXPIRY: WININET_SYNC_MODE = WININET_SYNC_MODE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WPAD_CACHE_DELETE(pub i32);
pub const WPAD_CACHE_DELETE_ALL: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(1i32);
pub const WPAD_CACHE_DELETE_CURRENT: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(0i32);
pub const XDR_CACHE_ENTRY: u32 = 262144u32;
pub type pfnInternetDeInitializeAutoProxyDll = Option<unsafe extern "system" fn(lpszmime: windows_core::PCSTR, dwreserved: u32) -> windows_core::BOOL>;
pub type pfnInternetGetProxyInfo = Option<unsafe extern "system" fn(lpszurl: windows_core::PCSTR, dwurllength: u32, lpszurlhostname: windows_core::PCSTR, dwurlhostnamelength: u32, lplpszproxyhostname: *mut windows_core::PSTR, lpdwproxyhostnamelength: *mut u32) -> windows_core::BOOL>;
pub type pfnInternetInitializeAutoProxyDll = Option<unsafe extern "system" fn(dwversion: u32, lpszdownloadedtempfile: windows_core::PCSTR, lpszmime: windows_core::PCSTR, lpautoproxycallbacks: *mut AutoProxyHelperFunctions, lpautoproxyscriptbuffer: *mut AUTO_PROXY_SCRIPT_BUFFER) -> windows_core::BOOL>;
