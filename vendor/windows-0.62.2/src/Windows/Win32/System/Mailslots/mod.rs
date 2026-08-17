#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotA<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_core::PCSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateMailslotA(lpname.param().abi(), nmaxmessagesize, lreadtimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotW<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_core::PCWSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateMailslotW(lpname.param().abi(), nmaxmessagesize, lreadtimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: Option<*mut u32>, lpnextsize: Option<*mut u32>, lpmessagecount: Option<*mut u32>, lpreadtimeout: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lpmaxmessagesize : *mut u32, lpnextsize : *mut u32, lpmessagecount : *mut u32, lpreadtimeout : *mut u32) -> windows_core::BOOL);
    unsafe { GetMailslotInfo(hmailslot, lpmaxmessagesize.unwrap_or(core::mem::zeroed()) as _, lpnextsize.unwrap_or(core::mem::zeroed()) as _, lpmessagecount.unwrap_or(core::mem::zeroed()) as _, lpreadtimeout.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lreadtimeout : u32) -> windows_core::BOOL);
    unsafe { SetMailslotInfo(hmailslot, lreadtimeout).ok() }
}
