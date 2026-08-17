#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotA<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_core::PCSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMailslotA(lpname.param().abi(), nmaxmessagesize, lreadtimeout, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotW<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_core::PCWSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMailslotW(lpname.param().abi(), nmaxmessagesize, lreadtimeout, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetMailslotInfo<P0>(hmailslot: P0, lpmaxmessagesize: Option<*mut u32>, lpnextsize: Option<*mut u32>, lpmessagecount: Option<*mut u32>, lpreadtimeout: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lpmaxmessagesize : *mut u32, lpnextsize : *mut u32, lpmessagecount : *mut u32, lpreadtimeout : *mut u32) -> super::super::Foundation:: BOOL);
    GetMailslotInfo(hmailslot.param().abi(), core::mem::transmute(lpmaxmessagesize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpnextsize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpmessagecount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpreadtimeout.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetMailslotInfo<P0>(hmailslot: P0, lreadtimeout: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lreadtimeout : u32) -> super::super::Foundation:: BOOL);
    SetMailslotInfo(hmailslot.param().abi(), lreadtimeout).ok()
}
