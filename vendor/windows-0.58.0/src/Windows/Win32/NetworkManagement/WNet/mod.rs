#[inline]
pub unsafe fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    windows_targets::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceA(lpnetresource : *const NETRESOURCEA, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
    MultinetGetConnectionPerformanceA(lpnetresource, lpnetconnectinfostruct)
}
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    windows_targets::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceW(lpnetresource : *const NETRESOURCEW, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
    MultinetGetConnectionPerformanceW(lpnetresource, lpnetconnectinfostruct)
}
#[inline]
pub unsafe fn NPAddConnection<P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPAddConnection(lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR) -> u32);
    NPAddConnection(lpnetresource, lppassword.param().abi(), lpusername.param().abi())
}
#[inline]
pub unsafe fn NPAddConnection3<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: NET_CONNECT_FLAGS) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPAddConnection3(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> u32);
    NPAddConnection3(hwndowner.param().abi(), lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn NPAddConnection4<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lpauthbuffer: Option<*const core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("ntlanman.dll" "system" fn NPAddConnection4(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lpauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
    NPAddConnection4(hwndowner.param().abi(), lpnetresource, core::mem::transmute(lpauthbuffer.unwrap_or(std::ptr::null())), cbauthbuffer, dwflags, core::mem::transmute(lpuseoptions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn NPCancelConnection<P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPCancelConnection(lpname : windows_core::PCWSTR, fforce : super::super::Foundation:: BOOL) -> u32);
    NPCancelConnection(lpname.param().abi(), fforce.param().abi())
}
#[inline]
pub unsafe fn NPCancelConnection2<P0, P1>(lpname: P0, fforce: P1, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ntlanman.dll" "system" fn NPCancelConnection2(lpname : windows_core::PCWSTR, fforce : super::super::Foundation:: BOOL, dwflags : u32) -> u32);
    NPCancelConnection2(lpname.param().abi(), fforce.param().abi(), dwflags)
}
#[inline]
pub unsafe fn NPCloseEnum<P0>(henum: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPCloseEnum(henum : super::super::Foundation:: HANDLE) -> u32);
    NPCloseEnum(henum.param().abi())
}
#[inline]
pub unsafe fn NPEnumResource<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPEnumResource(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    NPEnumResource(henum.param().abi(), lpccount, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn NPFormatNetworkName<P0>(lpremotename: P0, lpformattedname: windows_core::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPFormatNetworkName(lpremotename : windows_core::PCWSTR, lpformattedname : windows_core::PWSTR, lpnlength : *mut u32, dwflags : NETWORK_NAME_FORMAT_FLAGS, dwavecharperline : u32) -> u32);
    NPFormatNetworkName(lpremotename.param().abi(), core::mem::transmute(lpformattedname), lpnlength, dwflags, dwavecharperline)
}
#[inline]
pub unsafe fn NPGetCaps(ndex: u32) -> u32 {
    windows_targets::link!("davclnt.dll" "system" fn NPGetCaps(ndex : u32) -> u32);
    NPGetCaps(ndex)
}
#[inline]
pub unsafe fn NPGetConnection<P0>(lplocalname: P0, lpremotename: windows_core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPGetConnection(lplocalname : windows_core::PCWSTR, lpremotename : windows_core::PWSTR, lpnbufferlen : *mut u32) -> u32);
    NPGetConnection(lplocalname.param().abi(), core::mem::transmute(lpremotename), lpnbufferlen)
}
#[inline]
pub unsafe fn NPGetConnection3<P0>(lplocalname: P0, dwlevel: u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntlanman.dll" "system" fn NPGetConnection3(lplocalname : windows_core::PCWSTR, dwlevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    NPGetConnection3(lplocalname.param().abi(), dwlevel, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn NPGetConnectionPerformance<P0>(lpremotename: P0, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntlanman.dll" "system" fn NPGetConnectionPerformance(lpremotename : windows_core::PCWSTR, lpnetconnectinfo : *mut NETCONNECTINFOSTRUCT) -> u32);
    NPGetConnectionPerformance(lpremotename.param().abi(), lpnetconnectinfo)
}
#[inline]
pub unsafe fn NPGetPersistentUseOptionsForConnection<P0>(lpremotepath: P0, lpreaduseoptions: Option<&[u8]>, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntlanman.dll" "system" fn NPGetPersistentUseOptionsForConnection(lpremotepath : windows_core::PCWSTR, lpreaduseoptions : *const u8, cbreaduseoptions : u32, lpwriteuseoptions : *mut u8, lpsizewriteuseoptions : *mut u32) -> u32);
    NPGetPersistentUseOptionsForConnection(lpremotepath.param().abi(), core::mem::transmute(lpreaduseoptions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpreaduseoptions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpwriteuseoptions, lpsizewriteuseoptions)
}
#[inline]
pub unsafe fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut windows_core::PWSTR) -> u32 {
    windows_targets::link!("davclnt.dll" "system" fn NPGetResourceInformation(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32, lplpsystem : *mut windows_core::PWSTR) -> u32);
    NPGetResourceInformation(lpnetresource, lpbuffer, lpbuffersize, lplpsystem)
}
#[inline]
pub unsafe fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32 {
    windows_targets::link!("davclnt.dll" "system" fn NPGetResourceParent(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    NPGetResourceParent(lpnetresource, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn NPGetUniversalName<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPGetUniversalName(lplocalpath : windows_core::PCWSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
    NPGetUniversalName(lplocalpath.param().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn NPGetUser<P0>(lpname: P0, lpusername: windows_core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("davclnt.dll" "system" fn NPGetUser(lpname : windows_core::PCWSTR, lpusername : windows_core::PWSTR, lpnbufferlen : *mut u32) -> u32);
    NPGetUser(lpname.param().abi(), core::mem::transmute(lpusername), lpnbufferlen)
}
#[inline]
pub unsafe fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: Option<*const NETRESOURCEW>, lphenum: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("davclnt.dll" "system" fn NPOpenEnum(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::super::Foundation:: HANDLE) -> u32);
    NPOpenEnum(dwscope, dwtype, dwusage, core::mem::transmute(lpnetresource.unwrap_or(std::ptr::null())), lphenum)
}
#[inline]
pub unsafe fn WNetAddConnection2A<P0, P1>(lpnetresource: *const NETRESOURCEA, lppassword: P0, lpusername: P1, dwflags: NET_CONNECT_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection2A(lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpusername : windows_core::PCSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection2A(lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn WNetAddConnection2W<P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1, dwflags: NET_CONNECT_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection2W(lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection2W(lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn WNetAddConnection3A<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpusername: P2, dwflags: NET_CONNECT_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection3A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpusername : windows_core::PCSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection3A(hwndowner.param().abi(), lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn WNetAddConnection3W<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: NET_CONNECT_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection3W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpusername : windows_core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection3W(hwndowner.param().abi(), lpnetresource, lppassword.param().abi(), lpusername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn WNetAddConnection4A<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: NET_CONNECT_FLAGS, lpuseoptions: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection4A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : NET_CONNECT_FLAGS, lpuseoptions : *const u8, cbuseoptions : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection4A(hwndowner.param().abi(), lpnetresource, pauthbuffer, cbauthbuffer, dwflags, core::mem::transmute(lpuseoptions.as_ptr()), lpuseoptions.len().try_into().unwrap())
}
#[inline]
pub unsafe fn WNetAddConnection4W<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: NET_CONNECT_FLAGS, lpuseoptions: &[u8]) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnection4W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : NET_CONNECT_FLAGS, lpuseoptions : *const u8, cbuseoptions : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnection4W(hwndowner.param().abi(), lpnetresource, pauthbuffer, cbauthbuffer, dwflags, core::mem::transmute(lpuseoptions.as_ptr()), lpuseoptions.len().try_into().unwrap())
}
#[inline]
pub unsafe fn WNetAddConnectionA<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnectionA(lpremotename : windows_core::PCSTR, lppassword : windows_core::PCSTR, lplocalname : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnectionA(lpremotename.param().abi(), lppassword.param().abi(), lplocalname.param().abi())
}
#[inline]
pub unsafe fn WNetAddConnectionW<P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetAddConnectionW(lpremotename : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, lplocalname : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    WNetAddConnectionW(lpremotename.param().abi(), lppassword.param().abi(), lplocalname.param().abi())
}
#[inline]
pub unsafe fn WNetCancelConnection2A<P0, P1>(lpname: P0, dwflags: NET_CONNECT_FLAGS, fforce: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetCancelConnection2A(lpname : windows_core::PCSTR, dwflags : NET_CONNECT_FLAGS, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    WNetCancelConnection2A(lpname.param().abi(), dwflags, fforce.param().abi())
}
#[inline]
pub unsafe fn WNetCancelConnection2W<P0, P1>(lpname: P0, dwflags: NET_CONNECT_FLAGS, fforce: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetCancelConnection2W(lpname : windows_core::PCWSTR, dwflags : NET_CONNECT_FLAGS, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    WNetCancelConnection2W(lpname.param().abi(), dwflags, fforce.param().abi())
}
#[inline]
pub unsafe fn WNetCancelConnectionA<P0, P1>(lpname: P0, fforce: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetCancelConnectionA(lpname : windows_core::PCSTR, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    WNetCancelConnectionA(lpname.param().abi(), fforce.param().abi())
}
#[inline]
pub unsafe fn WNetCancelConnectionW<P0, P1>(lpname: P0, fforce: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetCancelConnectionW(lpname : windows_core::PCWSTR, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    WNetCancelConnectionW(lpname.param().abi(), fforce.param().abi())
}
#[inline]
pub unsafe fn WNetCloseEnum<P0>(henum: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetCloseEnum(henum : super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    WNetCloseEnum(henum.param().abi())
}
#[inline]
pub unsafe fn WNetConnectionDialog<P0>(hwnd: P0, dwtype: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog(hwnd : super::super::Foundation:: HWND, dwtype : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetConnectionDialog(hwnd.param().abi(), dwtype)
}
#[inline]
pub unsafe fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog1A(lpconndlgstruct : *mut CONNECTDLGSTRUCTA) -> super::super::Foundation:: WIN32_ERROR);
    WNetConnectionDialog1A(lpconndlgstruct)
}
#[inline]
pub unsafe fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog1W(lpconndlgstruct : *mut CONNECTDLGSTRUCTW) -> super::super::Foundation:: WIN32_ERROR);
    WNetConnectionDialog1W(lpconndlgstruct)
}
#[inline]
pub unsafe fn WNetDisconnectDialog<P0>(hwnd: P0, dwtype: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog(hwnd : super::super::Foundation:: HWND, dwtype : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetDisconnectDialog(hwnd.param().abi(), dwtype)
}
#[inline]
pub unsafe fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog1A(lpconndlgstruct : *const DISCDLGSTRUCTA) -> super::super::Foundation:: WIN32_ERROR);
    WNetDisconnectDialog1A(lpconndlgstruct)
}
#[inline]
pub unsafe fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog1W(lpconndlgstruct : *const DISCDLGSTRUCTW) -> super::super::Foundation:: WIN32_ERROR);
    WNetDisconnectDialog1W(lpconndlgstruct)
}
#[inline]
pub unsafe fn WNetEnumResourceA<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetEnumResourceA(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetEnumResourceA(henum.param().abi(), lpccount, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn WNetEnumResourceW<P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetEnumResourceW(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetEnumResourceW(henum.param().abi(), lpccount, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn WNetGetConnectionA<P0>(lplocalname: P0, lpremotename: windows_core::PSTR, lpnlength: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetConnectionA(lplocalname : windows_core::PCSTR, lpremotename : windows_core::PSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetConnectionA(lplocalname.param().abi(), core::mem::transmute(lpremotename), lpnlength)
}
#[inline]
pub unsafe fn WNetGetConnectionW<P0>(lplocalname: P0, lpremotename: windows_core::PWSTR, lpnlength: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetConnectionW(lplocalname : windows_core::PCWSTR, lpremotename : windows_core::PWSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetConnectionW(lplocalname.param().abi(), core::mem::transmute(lpremotename), lpnlength)
}
#[inline]
pub unsafe fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: &mut [u8], lpnamebuf: &mut [u8]) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetLastErrorA(lperror : *mut u32, lperrorbuf : windows_core::PSTR, nerrorbufsize : u32, lpnamebuf : windows_core::PSTR, nnamebufsize : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetLastErrorA(lperror, core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len().try_into().unwrap(), core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len().try_into().unwrap())
}
#[inline]
pub unsafe fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetLastErrorW(lperror : *mut u32, lperrorbuf : windows_core::PWSTR, nerrorbufsize : u32, lpnamebuf : windows_core::PWSTR, nnamebufsize : u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetLastErrorW(lperror, core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len().try_into().unwrap(), core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len().try_into().unwrap())
}
#[inline]
pub unsafe fn WNetGetNetworkInformationA<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetNetworkInformationA(lpprovider : windows_core::PCSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetNetworkInformationA(lpprovider.param().abi(), lpnetinfostruct)
}
#[inline]
pub unsafe fn WNetGetNetworkInformationW<P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetNetworkInformationW(lpprovider : windows_core::PCWSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetNetworkInformationW(lpprovider.param().abi(), lpnetinfostruct)
}
#[inline]
pub unsafe fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: windows_core::PSTR, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetProviderNameA(dwnettype : u32, lpprovidername : windows_core::PSTR, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetProviderNameA(dwnettype, core::mem::transmute(lpprovidername), lpbuffersize)
}
#[inline]
pub unsafe fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: windows_core::PWSTR, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetProviderNameW(dwnettype : u32, lpprovidername : windows_core::PWSTR, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetProviderNameW(dwnettype, core::mem::transmute(lpprovidername), lpbuffersize)
}
#[inline]
pub unsafe fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut windows_core::PSTR) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetResourceInformationA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_core::PSTR) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetResourceInformationA(lpnetresource, lpbuffer, lpcbbuffer, lplpsystem)
}
#[inline]
pub unsafe fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut windows_core::PWSTR) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetResourceInformationW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetResourceInformationW(lpnetresource, lpbuffer, lpcbbuffer, lplpsystem)
}
#[inline]
pub unsafe fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetResourceParentA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetResourceParentA(lpnetresource, lpbuffer, lpcbbuffer)
}
#[inline]
pub unsafe fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpcbbuffer: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetGetResourceParentW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetResourceParentW(lpnetresource, lpbuffer, lpcbbuffer)
}
#[inline]
pub unsafe fn WNetGetUniversalNameA<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetUniversalNameA(lplocalpath : windows_core::PCSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetUniversalNameA(lplocalpath.param().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn WNetGetUniversalNameW<P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetUniversalNameW(lplocalpath : windows_core::PCWSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetUniversalNameW(lplocalpath.param().abi(), dwinfolevel, lpbuffer, lpbuffersize)
}
#[inline]
pub unsafe fn WNetGetUserA<P0>(lpname: P0, lpusername: windows_core::PSTR, lpnlength: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetUserA(lpname : windows_core::PCSTR, lpusername : windows_core::PSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetUserA(lpname.param().abi(), core::mem::transmute(lpusername), lpnlength)
}
#[inline]
pub unsafe fn WNetGetUserW<P0>(lpname: P0, lpusername: windows_core::PWSTR, lpnlength: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetGetUserW(lpname : windows_core::PCWSTR, lpusername : windows_core::PWSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetGetUserW(lpname.param().abi(), core::mem::transmute(lpusername), lpnlength)
}
#[inline]
pub unsafe fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: Option<*const NETRESOURCEA>, lphenum: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetOpenEnumA(dwscope : NET_RESOURCE_SCOPE, dwtype : NET_RESOURCE_TYPE, dwusage : WNET_OPEN_ENUM_USAGE, lpnetresource : *const NETRESOURCEA, lphenum : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    WNetOpenEnumA(dwscope, dwtype, dwusage, core::mem::transmute(lpnetresource.unwrap_or(std::ptr::null())), lphenum)
}
#[inline]
pub unsafe fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: Option<*const NETRESOURCEW>, lphenum: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("mpr.dll" "system" fn WNetOpenEnumW(dwscope : NET_RESOURCE_SCOPE, dwtype : NET_RESOURCE_TYPE, dwusage : WNET_OPEN_ENUM_USAGE, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    WNetOpenEnumW(dwscope, dwtype, dwusage, core::mem::transmute(lpnetresource.unwrap_or(std::ptr::null())), lphenum)
}
#[inline]
pub unsafe fn WNetSetLastErrorA<P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetSetLastErrorA(err : u32, lperror : windows_core::PCSTR, lpproviders : windows_core::PCSTR));
    WNetSetLastErrorA(err, lperror.param().abi(), lpproviders.param().abi())
}
#[inline]
pub unsafe fn WNetSetLastErrorW<P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetSetLastErrorW(err : u32, lperror : windows_core::PCWSTR, lpproviders : windows_core::PCWSTR));
    WNetSetLastErrorW(err, lperror.param().abi(), lpproviders.param().abi())
}
#[inline]
pub unsafe fn WNetUseConnection4A<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: Option<*const core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: Option<&[u8]>, lpaccessname: windows_core::PSTR, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetUseConnection4A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetUseConnection4A(hwndowner.param().abi(), lpnetresource, core::mem::transmute(pauthbuffer.unwrap_or(std::ptr::null())), cbauthbuffer, dwflags, core::mem::transmute(lpuseoptions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpaccessname), core::mem::transmute(lpbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WNetUseConnection4W<P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: Option<*const core::ffi::c_void>, cbauthbuffer: u32, dwflags: u32, lpuseoptions: Option<&[u8]>, lpaccessname: windows_core::PWSTR, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetUseConnection4W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetUseConnection4W(hwndowner.param().abi(), lpnetresource, core::mem::transmute(pauthbuffer.unwrap_or(std::ptr::null())), cbauthbuffer, dwflags, core::mem::transmute(lpuseoptions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpuseoptions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpaccessname), core::mem::transmute(lpbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WNetUseConnectionA<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpuserid: P2, dwflags: NET_CONNECT_FLAGS, lpaccessname: windows_core::PSTR, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetUseConnectionA(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_core::PCSTR, lpuserid : windows_core::PCSTR, dwflags : NET_CONNECT_FLAGS, lpaccessname : windows_core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetUseConnectionA(hwndowner.param().abi(), lpnetresource, lppassword.param().abi(), lpuserid.param().abi(), dwflags, core::mem::transmute(lpaccessname), core::mem::transmute(lpbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpresult.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WNetUseConnectionW<P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpuserid: P2, dwflags: NET_CONNECT_FLAGS, lpaccessname: windows_core::PWSTR, lpbuffersize: Option<*mut u32>, lpresult: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mpr.dll" "system" fn WNetUseConnectionW(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_core::PCWSTR, lpuserid : windows_core::PCWSTR, dwflags : NET_CONNECT_FLAGS, lpaccessname : windows_core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    WNetUseConnectionW(hwndowner.param().abi(), lpnetresource, lppassword.param().abi(), lpuserid.param().abi(), dwflags, core::mem::transmute(lpaccessname), core::mem::transmute(lpbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpresult.unwrap_or(std::ptr::null_mut())))
}
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(2u32);
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(8u32);
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(32u32);
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(16u32);
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(1u32);
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(4u32);
pub const CONNECT_CMD_SAVECRED: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(4096u32);
pub const CONNECT_COMMANDLINE: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(2048u32);
pub const CONNECT_CRED_RESET: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(8192u32);
pub const CONNECT_CURRENT_MEDIA: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(512u32);
pub const CONNECT_DEFERRED: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(1024u32);
pub const CONNECT_GLOBAL_MAPPING: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(262144u32);
pub const CONNECT_INTERACTIVE: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(8u32);
pub const CONNECT_LOCALDRIVE: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(256u32);
pub const CONNECT_NEED_DRIVE: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(32u32);
pub const CONNECT_PROMPT: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(16u32);
pub const CONNECT_REDIRECT: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(128u32);
pub const CONNECT_REFCOUNT: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(64u32);
pub const CONNECT_REQUIRE_INTEGRITY: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(16384u32);
pub const CONNECT_REQUIRE_PRIVACY: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(32768u32);
pub const CONNECT_RESERVED: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(4278190080u32);
pub const CONNECT_TEMPORARY: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(4u32);
pub const CONNECT_UPDATE_PROFILE: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(1u32);
pub const CONNECT_UPDATE_RECENT: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(2u32);
pub const CONNECT_WRITE_THROUGH_SEMANTICS: NET_CONNECT_FLAGS = NET_CONNECT_FLAGS(65536u32);
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(64u32);
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(1u32);
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(4u32);
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(1u32);
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(8u32);
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
pub const NOTIFY_POST: u32 = 2u32;
pub const NOTIFY_PRE: u32 = 1u32;
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(2u32);
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(0u32);
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(1u32);
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(2u32);
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(19u32);
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(16u32);
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(1u32);
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(2u32);
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(0u32);
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(1u32);
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(5u32);
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(2u32);
pub const RESOURCE_RECENT: u32 = 4u32;
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(3u32);
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(1u32);
pub const WNCON_DYNAMIC: u32 = 8u32;
pub const WNCON_FORNETCARD: u32 = 1u32;
pub const WNCON_NOTROUTED: u32 = 2u32;
pub const WNCON_SLOWLINK: u32 = 4u32;
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(1u32);
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(3u32);
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(2u32);
pub const WNDT_NETWORK: u32 = 1u32;
pub const WNDT_NORMAL: u32 = 0u32;
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(2u32);
pub const WNFMT_CONNECTION: u32 = 32u32;
pub const WNFMT_INENUM: u32 = 16u32;
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(1u32);
pub const WNGETCON_CONNECTED: u32 = 0u32;
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
pub const WNNC_ADMIN: u32 = 9u32;
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
pub const WNNC_CONNECTION: u32 = 6u32;
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
pub const WNNC_CON_DEFER: u32 = 128u32;
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
pub const WNNC_DIALOG: u32 = 8u32;
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
pub const WNNC_ENUMERATION: u32 = 11u32;
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
pub const WNNC_NET_NONE: u32 = 0u32;
pub const WNNC_NET_TYPE: u32 = 2u32;
pub const WNNC_SPEC_VERSION: u32 = 1u32;
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
pub const WNNC_START: u32 = 12u32;
pub const WNNC_USER: u32 = 4u32;
pub const WNNC_USR_GETUSER: u32 = 1u32;
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
pub const WNPERMC_AUDIT: u32 = 2u32;
pub const WNPERMC_OWNER: u32 = 4u32;
pub const WNPERMC_PERM: u32 = 1u32;
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = WNPERM_DLG(1u32);
pub const WNPERM_DLG_OWNER: WNPERM_DLG = WNPERM_DLG(2u32);
pub const WNPERM_DLG_PERM: WNPERM_DLG = WNPERM_DLG(0u32);
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(1u32);
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(0u32);
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(2u32);
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
pub const WNTYPE_COMM: u32 = 4u32;
pub const WNTYPE_DRIVE: u32 = 1u32;
pub const WNTYPE_FILE: u32 = 2u32;
pub const WNTYPE_PRINTER: u32 = 3u32;
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
pub const WN_NETWORK_CLASS: u32 = 1u32;
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
pub const WN_SERVICE_CLASS: u32 = 8u32;
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONNECTDLGSTRUCT_FLAGS(pub u32);
impl windows_core::TypeKind for CONNECTDLGSTRUCT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONNECTDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONNECTDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl CONNECTDLGSTRUCT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISCDLGSTRUCT_FLAGS(pub u32);
impl windows_core::TypeKind for DISCDLGSTRUCT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISCDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISCDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl DISCDLGSTRUCT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DISCDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DISCDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETINFOSTRUCT_CHARACTERISTICS(pub u32);
impl windows_core::TypeKind for NETINFOSTRUCT_CHARACTERISTICS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETINFOSTRUCT_CHARACTERISTICS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETINFOSTRUCT_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl NETINFOSTRUCT_CHARACTERISTICS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETWORK_NAME_FORMAT_FLAGS(pub u32);
impl windows_core::TypeKind for NETWORK_NAME_FORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETWORK_NAME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETWORK_NAME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_CONNECT_FLAGS(pub u32);
impl windows_core::TypeKind for NET_CONNECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_CONNECT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_CONNECT_FLAGS").field(&self.0).finish()
    }
}
impl NET_CONNECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_RESOURCE_SCOPE(pub u32);
impl windows_core::TypeKind for NET_RESOURCE_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_RESOURCE_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_RESOURCE_TYPE(pub u32);
impl windows_core::TypeKind for NET_RESOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_RESOURCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_TYPE").field(&self.0).finish()
    }
}
impl NET_RESOURCE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_RESOURCE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_RESOURCE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_RESOURCE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NPDIRECTORY_NOTIFY_OPERATION(pub u32);
impl windows_core::TypeKind for NPDIRECTORY_NOTIFY_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NPDIRECTORY_NOTIFY_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NPDIRECTORY_NOTIFY_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NP_PROPERTY_DIALOG_SELECTION(pub u32);
impl windows_core::TypeKind for NP_PROPERTY_DIALOG_SELECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NP_PROPERTY_DIALOG_SELECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NP_PROPERTY_DIALOG_SELECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UNC_INFO_LEVEL(pub u32);
impl windows_core::TypeKind for UNC_INFO_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UNC_INFO_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UNC_INFO_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WNET_OPEN_ENUM_USAGE(pub u32);
impl windows_core::TypeKind for WNET_OPEN_ENUM_USAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WNET_OPEN_ENUM_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WNET_OPEN_ENUM_USAGE").field(&self.0).finish()
    }
}
impl WNET_OPEN_ENUM_USAGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WNET_OPEN_ENUM_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WNET_OPEN_ENUM_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WNPERM_DLG(pub u32);
impl windows_core::TypeKind for WNPERM_DLG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WNPERM_DLG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WNPERM_DLG").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
impl windows_core::TypeKind for CONNECTDLGSTRUCTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONNECTDLGSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
impl windows_core::TypeKind for CONNECTDLGSTRUCTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONNECTDLGSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: windows_core::PSTR,
    pub lpRemoteName: windows_core::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
impl windows_core::TypeKind for DISCDLGSTRUCTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISCDLGSTRUCTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: windows_core::PWSTR,
    pub lpRemoteName: windows_core::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
impl windows_core::TypeKind for DISCDLGSTRUCTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISCDLGSTRUCTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
impl windows_core::TypeKind for NETCONNECTINFOSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETCONNECTINFOSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
impl windows_core::TypeKind for NETINFOSTRUCT {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETINFOSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_core::PSTR,
    pub lpRemoteName: windows_core::PSTR,
    pub lpComment: windows_core::PSTR,
    pub lpProvider: windows_core::PSTR,
}
impl windows_core::TypeKind for NETRESOURCEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETRESOURCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_core::PWSTR,
    pub lpRemoteName: windows_core::PWSTR,
    pub lpComment: windows_core::PWSTR,
    pub lpProvider: windows_core::PWSTR,
}
impl windows_core::TypeKind for NETRESOURCEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETRESOURCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_CONNECT_FLAGS,
}
impl windows_core::TypeKind for NOTIFYADD {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFYADD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFYCANCEL {
    pub lpName: windows_core::PWSTR,
    pub lpProvider: windows_core::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for NOTIFYCANCEL {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFYCANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for NOTIFYINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for NOTIFYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: windows_core::PSTR,
    pub lpConnectionName: windows_core::PSTR,
    pub lpRemainingPath: windows_core::PSTR,
}
impl windows_core::TypeKind for REMOTE_NAME_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for REMOTE_NAME_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: windows_core::PWSTR,
    pub lpConnectionName: windows_core::PWSTR,
    pub lpRemainingPath: windows_core::PWSTR,
}
impl windows_core::TypeKind for REMOTE_NAME_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for REMOTE_NAME_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: windows_core::PSTR,
}
impl windows_core::TypeKind for UNIVERSAL_NAME_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for UNIVERSAL_NAME_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: windows_core::PWSTR,
}
impl windows_core::TypeKind for UNIVERSAL_NAME_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for UNIVERSAL_NAME_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PF_AddConnectNotify = Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32>;
pub type PF_CancelConnectNotify = Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpcancelinfo: *const NOTIFYCANCEL) -> u32>;
pub type PF_NPAddConnection = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lppassword: windows_core::PCWSTR, lpusername: windows_core::PCWSTR) -> u32>;
pub type PF_NPAddConnection3 = Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: windows_core::PCWSTR, lpusername: windows_core::PCWSTR, dwflags: u32) -> u32>;
pub type PF_NPAddConnection4 = Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32>;
pub type PF_NPCancelConnection = Option<unsafe extern "system" fn(lpname: windows_core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32>;
pub type PF_NPCancelConnection2 = Option<unsafe extern "system" fn(lpname: windows_core::PCWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32>;
pub type PF_NPCloseEnum = Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32>;
pub type PF_NPDeviceMode = Option<unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32>;
pub type PF_NPDirectoryNotify = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpdir: windows_core::PCWSTR, dwoper: u32) -> u32>;
pub type PF_NPEnumResource = Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPFMXEditPerm = Option<unsafe extern "system" fn(lpdrivename: windows_core::PCWSTR, hwndfmx: super::super::Foundation::HWND, ndialogtype: u32) -> u32>;
pub type PF_NPFMXGetPermCaps = Option<unsafe extern "system" fn(lpdrivename: windows_core::PCWSTR) -> u32>;
pub type PF_NPFMXGetPermHelp = Option<unsafe extern "system" fn(lpdrivename: windows_core::PCWSTR, ndialogtype: u32, fdirectory: super::super::Foundation::BOOL, lpfilenamebuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32, lpnhelpcontext: *mut u32) -> u32>;
pub type PF_NPFormatNetworkName = Option<unsafe extern "system" fn(lpremotename: windows_core::PCWSTR, lpformattedname: windows_core::PWSTR, lpnlength: *mut u32, dwflags: u32, dwavecharperline: u32) -> u32>;
pub type PF_NPGetCaps = Option<unsafe extern "system" fn(ndex: u32) -> u32>;
pub type PF_NPGetConnection = Option<unsafe extern "system" fn(lplocalname: windows_core::PCWSTR, lpremotename: windows_core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
pub type PF_NPGetConnection3 = Option<unsafe extern "system" fn(lplocalname: windows_core::PCWSTR, dwlevel: u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPGetConnectionPerformance = Option<unsafe extern "system" fn(lpremotename: windows_core::PCWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32>;
pub type PF_NPGetDirectoryType = Option<unsafe extern "system" fn(lpname: windows_core::PCWSTR, lptype: *const i32, bflushcache: super::super::Foundation::BOOL) -> u32>;
pub type PF_NPGetPersistentUseOptionsForConnection = Option<unsafe extern "system" fn(lpremotepath: windows_core::PCWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32>;
pub type PF_NPGetPropertyText = Option<unsafe extern "system" fn(ibutton: u32, npropsel: u32, lpname: windows_core::PCWSTR, lpbuttonname: windows_core::PWSTR, nbuttonnamelen: u32, ntype: u32) -> u32>;
pub type PF_NPGetResourceInformation = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut windows_core::PWSTR) -> u32>;
pub type PF_NPGetResourceParent = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPGetUniversalName = Option<unsafe extern "system" fn(lplocalpath: windows_core::PCWSTR, dwinfolevel: u32, lpbuffer: *mut core::ffi::c_void, lpnbuffersize: *mut u32) -> u32>;
pub type PF_NPGetUser = Option<unsafe extern "system" fn(lpname: windows_core::PCWSTR, lpusername: windows_core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
pub type PF_NPLogonNotify = Option<unsafe extern "system" fn(lplogonid: *const super::super::Foundation::LUID, lpauthentinfotype: windows_core::PCWSTR, lpauthentinfo: *const core::ffi::c_void, lppreviousauthentinfotype: windows_core::PCWSTR, lppreviousauthentinfo: *const core::ffi::c_void, lpstationname: windows_core::PCWSTR, stationhandle: *const core::ffi::c_void, lplogonscript: *mut windows_core::PWSTR) -> u32>;
pub type PF_NPOpenEnum = Option<unsafe extern "system" fn(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PF_NPPasswordChangeNotify = Option<unsafe extern "system" fn(lpauthentinfotype: windows_core::PCWSTR, lpauthentinfo: *const core::ffi::c_void, lppreviousauthentinfotype: windows_core::PCWSTR, lppreviousauthentinfo: *const core::ffi::c_void, lpstationname: windows_core::PCWSTR, stationhandle: *const core::ffi::c_void, dwchangeinfo: u32) -> u32>;
pub type PF_NPPropertyDialog = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, ibuttondlg: u32, npropsel: u32, lpfilename: windows_core::PCWSTR, ntype: u32) -> u32>;
pub type PF_NPSearchDialog = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, cbbuffer: u32, lpnflags: *mut u32) -> u32>;
