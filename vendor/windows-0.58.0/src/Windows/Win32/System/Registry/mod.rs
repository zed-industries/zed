#[inline]
pub unsafe fn GetRegistryValueWithFallbackW<P0, P1, P2, P3, P4>(hkeyprimary: P0, pwszprimarysubkey: P1, hkeyfallback: P2, pwszfallbacksubkey: P3, pwszvalue: P4, dwflags: u32, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, cbdatain: u32, pcbdataout: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HKEY>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-state-helpers-l1-1-0.dll" "system" fn GetRegistryValueWithFallbackW(hkeyprimary : HKEY, pwszprimarysubkey : windows_core::PCWSTR, hkeyfallback : HKEY, pwszfallbacksubkey : windows_core::PCWSTR, pwszvalue : windows_core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, cbdatain : u32, pcbdataout : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetRegistryValueWithFallbackW(hkeyprimary.param().abi(), pwszprimarysubkey.param().abi(), hkeyfallback.param().abi(), pwszfallbacksubkey.param().abi(), pwszvalue.param().abi(), dwflags, core::mem::transmute(pdwtype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvdata.unwrap_or(std::ptr::null_mut())), cbdatain, core::mem::transmute(pcbdataout.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegCloseKey<P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCloseKey(hkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCloseKey(hkey.param().abi())
}
#[inline]
pub unsafe fn RegConnectRegistryA<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryA(lpmachinename : windows_core::PCSTR, hkey : HKEY, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegConnectRegistryA(lpmachinename.param().abi(), hkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn RegConnectRegistryExA<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryExA(lpmachinename : windows_core::PCSTR, hkey : HKEY, flags : u32, phkresult : *mut HKEY) -> i32);
    RegConnectRegistryExA(lpmachinename.param().abi(), hkey.param().abi(), flags, phkresult)
}
#[inline]
pub unsafe fn RegConnectRegistryExW<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryExW(lpmachinename : windows_core::PCWSTR, hkey : HKEY, flags : u32, phkresult : *mut HKEY) -> i32);
    RegConnectRegistryExW(lpmachinename.param().abi(), hkey.param().abi(), flags, phkresult)
}
#[inline]
pub unsafe fn RegConnectRegistryW<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryW(lpmachinename : windows_core::PCWSTR, hkey : HKEY, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegConnectRegistryW(lpmachinename.param().abi(), hkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn RegCopyTreeA<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCopyTreeA(hkeysrc : HKEY, lpsubkey : windows_core::PCSTR, hkeydest : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCopyTreeA(hkeysrc.param().abi(), lpsubkey.param().abi(), hkeydest.param().abi())
}
#[inline]
pub unsafe fn RegCopyTreeW<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCopyTreeW(hkeysrc : HKEY, lpsubkey : windows_core::PCWSTR, hkeydest : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCopyTreeW(hkeysrc.param().abi(), lpsubkey.param().abi(), hkeydest.param().abi())
}
#[inline]
pub unsafe fn RegCreateKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyA(hkey.param().abi(), lpsubkey.param().abi(), phkresult)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegCreateKeyExA<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: Option<*mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyExA(hkey : HKEY, lpsubkey : windows_core::PCSTR, reserved : u32, lpclass : windows_core::PCSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyExA(hkey.param().abi(), lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegCreateKeyExW<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: Option<*mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyExW(hkey.param().abi(), lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyTransactedA(hkey : HKEY, lpsubkey : windows_core::PCSTR, reserved : u32, lpclass : windows_core::PCSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyTransactedA(hkey.param().abi(), lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())), htransaction.param().abi(), core::mem::transmute(pextendedparemeter.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyTransactedW(hkey.param().abi(), lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, core::mem::transmute(lpdwdisposition.unwrap_or(std::ptr::null_mut())), htransaction.param().abi(), core::mem::transmute(pextendedparemeter.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RegCreateKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyW(hkey.param().abi(), lpsubkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn RegDeleteKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyA(hkey.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn RegDeleteKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyExA(hkey : HKEY, lpsubkey : windows_core::PCSTR, samdesired : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyExA(hkey.param().abi(), lpsubkey.param().abi(), samdesired, reserved)
}
#[inline]
pub unsafe fn RegDeleteKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyExW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, samdesired : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyExW(hkey.param().abi(), lpsubkey.param().abi(), samdesired, reserved)
}
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedA(hkey : HKEY, lpsubkey : windows_core::PCSTR, samdesired : u32, reserved : u32, htransaction : super::super::Foundation:: HANDLE, pextendedparameter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyTransactedA(hkey.param().abi(), lpsubkey.param().abi(), samdesired, reserved, htransaction.param().abi(), core::mem::transmute(pextendedparameter.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, samdesired : u32, reserved : u32, htransaction : super::super::Foundation:: HANDLE, pextendedparameter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyTransactedW(hkey.param().abi(), lpsubkey.param().abi(), samdesired, reserved, htransaction.param().abi(), core::mem::transmute(pextendedparameter.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RegDeleteKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyValueA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpvaluename : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyValueA(hkey.param().abi(), lpsubkey.param().abi(), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn RegDeleteKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyValueW(hkey.param().abi(), lpsubkey.param().abi(), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn RegDeleteKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyW(hkey.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn RegDeleteTreeA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteTreeA(hkey : HKEY, lpsubkey : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteTreeA(hkey.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn RegDeleteTreeW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : HKEY, lpsubkey : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteTreeW(hkey.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn RegDeleteValueA<P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteValueA(hkey : HKEY, lpvaluename : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteValueA(hkey.param().abi(), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn RegDeleteValueW<P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : HKEY, lpvaluename : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteValueW(hkey.param().abi(), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn RegDisablePredefinedCache() -> super::super::Foundation:: WIN32_ERROR);
    RegDisablePredefinedCache()
}
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn RegDisablePredefinedCacheEx() -> super::super::Foundation:: WIN32_ERROR);
    RegDisablePredefinedCacheEx()
}
#[inline]
pub unsafe fn RegDisableReflectionKey<P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegDisableReflectionKey(hbase : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegDisableReflectionKey(hbase.param().abi())
}
#[inline]
pub unsafe fn RegEnableReflectionKey<P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnableReflectionKey(hbase : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegEnableReflectionKey(hbase.param().abi())
}
#[inline]
pub unsafe fn RegEnumKeyA<P0>(hkey: P0, dwindex: u32, lpname: Option<&mut [u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyA(hkey : HKEY, dwindex : u32, lpname : windows_core::PSTR, cchname : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyA(hkey.param().abi(), dwindex, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RegEnumKeyExA<P0>(hkey: P0, dwindex: u32, lpname: windows_core::PSTR, lpcchname: *mut u32, lpreserved: Option<*const u32>, lpclass: windows_core::PSTR, lpcchclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyExA(hkey : HKEY, dwindex : u32, lpname : windows_core::PSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_core::PSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyExA(hkey.param().abi(), dwindex, core::mem::transmute(lpname), lpcchname, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lpclass), core::mem::transmute(lpcchclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegEnumKeyExW<P0>(hkey: P0, dwindex: u32, lpname: windows_core::PWSTR, lpcchname: *mut u32, lpreserved: Option<*const u32>, lpclass: windows_core::PWSTR, lpcchclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : HKEY, dwindex : u32, lpname : windows_core::PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_core::PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyExW(hkey.param().abi(), dwindex, core::mem::transmute(lpname), lpcchname, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lpclass), core::mem::transmute(lpcchclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegEnumKeyW<P0>(hkey: P0, dwindex: u32, lpname: Option<&mut [u16]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyW(hkey : HKEY, dwindex : u32, lpname : windows_core::PWSTR, cchname : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyW(hkey.param().abi(), dwindex, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RegEnumValueA<P0>(hkey: P0, dwindex: u32, lpvaluename: windows_core::PSTR, lpcchvaluename: *mut u32, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumValueA(hkey : HKEY, dwindex : u32, lpvaluename : windows_core::PSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumValueA(hkey.param().abi(), dwindex, core::mem::transmute(lpvaluename), lpcchvaluename, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegEnumValueW<P0>(hkey: P0, dwindex: u32, lpvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : HKEY, dwindex : u32, lpvaluename : windows_core::PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumValueW(hkey.param().abi(), dwindex, core::mem::transmute(lpvaluename), lpcchvaluename, core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegFlushKey<P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegFlushKey(hkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegFlushKey(hkey.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegGetKeySecurity<P0>(hkey: P0, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegGetKeySecurity(hkey : HKEY, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetKeySecurity(hkey.param().abi(), securityinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[inline]
pub unsafe fn RegGetValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: Option<*mut REG_VALUE_TYPE>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegGetValueA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpvalue : windows_core::PCSTR, dwflags : REG_ROUTINE_FLAGS, pdwtype : *mut REG_VALUE_TYPE, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetValueA(hkey.param().abi(), lpsubkey.param().abi(), lpvalue.param().abi(), dwflags, core::mem::transmute(pdwtype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegGetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: Option<*mut REG_VALUE_TYPE>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegGetValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, dwflags : REG_ROUTINE_FLAGS, pdwtype : *mut REG_VALUE_TYPE, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetValueW(hkey.param().abi(), lpsubkey.param().abi(), lpvalue.param().abi(), dwflags, core::mem::transmute(pdwtype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegLoadAppKeyA<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadAppKeyA(lpfile : windows_core::PCSTR, phkresult : *mut HKEY, samdesired : u32, dwoptions : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadAppKeyA(lpfile.param().abi(), phkresult, samdesired, dwoptions, reserved)
}
#[inline]
pub unsafe fn RegLoadAppKeyW<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadAppKeyW(lpfile : windows_core::PCWSTR, phkresult : *mut HKEY, samdesired : u32, dwoptions : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadAppKeyW(lpfile.param().abi(), phkresult, samdesired, dwoptions, reserved)
}
#[inline]
pub unsafe fn RegLoadKeyA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpfile : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadKeyA(hkey.param().abi(), lpsubkey.param().abi(), lpfile.param().abi())
}
#[inline]
pub unsafe fn RegLoadKeyW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpfile : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadKeyW(hkey.param().abi(), lpsubkey.param().abi(), lpfile.param().abi())
}
#[inline]
pub unsafe fn RegLoadMUIStringA<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: Option<&mut [u8]>, pcbdata: Option<*mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadMUIStringA(hkey : HKEY, pszvalue : windows_core::PCSTR, pszoutbuf : windows_core::PSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadMUIStringA(hkey.param().abi(), pszvalue.param().abi(), core::mem::transmute(pszoutbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbdata.unwrap_or(std::ptr::null_mut())), flags, pszdirectory.param().abi())
}
#[inline]
pub unsafe fn RegLoadMUIStringW<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: windows_core::PWSTR, cboutbuf: u32, pcbdata: Option<*mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegLoadMUIStringW(hkey : HKEY, pszvalue : windows_core::PCWSTR, pszoutbuf : windows_core::PWSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadMUIStringW(hkey.param().abi(), pszvalue.param().abi(), core::mem::transmute(pszoutbuf), cboutbuf, core::mem::transmute(pcbdata.unwrap_or(std::ptr::null_mut())), flags, pszdirectory.param().abi())
}
#[inline]
pub unsafe fn RegNotifyChangeKeyValue<P0, P1, P2, P3>(hkey: P0, bwatchsubtree: P1, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: P2, fasynchronous: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegNotifyChangeKeyValue(hkey : HKEY, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : REG_NOTIFY_FILTER, hevent : super::super::Foundation:: HANDLE, fasynchronous : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    RegNotifyChangeKeyValue(hkey.param().abi(), bwatchsubtree.param().abi(), dwnotifyfilter, hevent.param().abi(), fasynchronous.param().abi())
}
#[inline]
pub unsafe fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("advapi32.dll" "system" fn RegOpenCurrentUser(samdesired : u32, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenCurrentUser(samdesired, phkresult)
}
#[inline]
pub unsafe fn RegOpenKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyA(hkey.param().abi(), lpsubkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn RegOpenKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyExA(hkey : HKEY, lpsubkey : windows_core::PCSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyExA(hkey.param().abi(), lpsubkey.param().abi(), uloptions, samdesired, phkresult)
}
#[inline]
pub unsafe fn RegOpenKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyExW(hkey.param().abi(), lpsubkey.param().abi(), uloptions, samdesired, phkresult)
}
#[inline]
pub unsafe fn RegOpenKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyTransactedA(hkey : HKEY, lpsubkey : windows_core::PCSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyTransactedA(hkey.param().abi(), lpsubkey.param().abi(), uloptions, samdesired, phkresult, htransaction.param().abi(), core::mem::transmute(pextendedparemeter.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RegOpenKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: Option<*const core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyTransactedW(hkey.param().abi(), lpsubkey.param().abi(), uloptions, samdesired, phkresult, htransaction.param().abi(), core::mem::transmute(pextendedparemeter.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn RegOpenKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyW(hkey.param().abi(), lpsubkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn RegOpenUserClassesRoot<P0>(htoken: P0, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOpenUserClassesRoot(htoken : super::super::Foundation:: HANDLE, dwoptions : u32, samdesired : u32, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenUserClassesRoot(htoken.param().abi(), dwoptions, samdesired, phkresult)
}
#[inline]
pub unsafe fn RegOverridePredefKey<P0, P1>(hkey: P0, hnewhkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegOverridePredefKey(hkey : HKEY, hnewhkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOverridePredefKey(hkey.param().abi(), hnewhkey.param().abi())
}
#[inline]
pub unsafe fn RegQueryInfoKeyA<P0>(hkey: P0, lpclass: windows_core::PSTR, lpcchclass: Option<*mut u32>, lpreserved: Option<*const u32>, lpcsubkeys: Option<*mut u32>, lpcbmaxsubkeylen: Option<*mut u32>, lpcbmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcbmaxvaluenamelen: Option<*mut u32>, lpcbmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryInfoKeyA(hkey : HKEY, lpclass : windows_core::PSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryInfoKeyA(
        hkey.param().abi(),
        core::mem::transmute(lpclass),
        core::mem::transmute(lpcchclass.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())),
        core::mem::transmute(lpcsubkeys.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxsubkeylen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxclasslen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcvalues.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxvaluenamelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxvaluelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn RegQueryInfoKeyW<P0>(hkey: P0, lpclass: windows_core::PWSTR, lpcchclass: Option<*mut u32>, lpreserved: Option<*const u32>, lpcsubkeys: Option<*mut u32>, lpcbmaxsubkeylen: Option<*mut u32>, lpcbmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcbmaxvaluenamelen: Option<*mut u32>, lpcbmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : HKEY, lpclass : windows_core::PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryInfoKeyW(
        hkey.param().abi(),
        core::mem::transmute(lpclass),
        core::mem::transmute(lpcchclass.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())),
        core::mem::transmute(lpcsubkeys.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxsubkeylen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxclasslen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcvalues.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxvaluenamelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbmaxvaluelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn RegQueryMultipleValuesA<P0>(hkey: P0, val_list: &mut [VALENTA], lpvaluebuf: windows_core::PSTR, ldwtotsize: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryMultipleValuesA(hkey : HKEY, val_list : *mut VALENTA, num_vals : u32, lpvaluebuf : windows_core::PSTR, ldwtotsize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryMultipleValuesA(hkey.param().abi(), core::mem::transmute(val_list.as_ptr()), val_list.len().try_into().unwrap(), core::mem::transmute(lpvaluebuf), core::mem::transmute(ldwtotsize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegQueryMultipleValuesW<P0>(hkey: P0, val_list: &mut [VALENTW], lpvaluebuf: windows_core::PWSTR, ldwtotsize: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryMultipleValuesW(hkey : HKEY, val_list : *mut VALENTW, num_vals : u32, lpvaluebuf : windows_core::PWSTR, ldwtotsize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryMultipleValuesW(hkey.param().abi(), core::mem::transmute(val_list.as_ptr()), val_list.len().try_into().unwrap(), core::mem::transmute(lpvaluebuf), core::mem::transmute(ldwtotsize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegQueryReflectionKey<P0>(hbase: P0, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryReflectionKey(hbase : HKEY, bisreflectiondisabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryReflectionKey(hbase.param().abi(), bisreflectiondisabled)
}
#[inline]
pub unsafe fn RegQueryValueA<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: windows_core::PSTR, lpcbdata: Option<*mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryValueA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpdata : windows_core::PSTR, lpcbdata : *mut i32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueA(hkey.param().abi(), lpsubkey.param().abi(), core::mem::transmute(lpdata), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegQueryValueExA<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: Option<*const u32>, lptype: Option<*mut REG_VALUE_TYPE>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryValueExA(hkey : HKEY, lpvaluename : windows_core::PCSTR, lpreserved : *const u32, lptype : *mut REG_VALUE_TYPE, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueExA(hkey.param().abi(), lpvaluename.param().abi(), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegQueryValueExW<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: Option<*const u32>, lptype: Option<*mut REG_VALUE_TYPE>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : HKEY, lpvaluename : windows_core::PCWSTR, lpreserved : *const u32, lptype : *mut REG_VALUE_TYPE, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueExW(hkey.param().abi(), lpvaluename.param().abi(), core::mem::transmute(lpreserved.unwrap_or(std::ptr::null())), core::mem::transmute(lptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegQueryValueW<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: windows_core::PWSTR, lpcbdata: Option<*mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegQueryValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpdata : windows_core::PWSTR, lpcbdata : *mut i32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueW(hkey.param().abi(), lpsubkey.param().abi(), core::mem::transmute(lpdata), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RegRenameKey<P0, P1, P2>(hkey: P0, lpsubkeyname: P1, lpnewkeyname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegRenameKey(hkey : HKEY, lpsubkeyname : windows_core::PCWSTR, lpnewkeyname : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegRenameKey(hkey.param().abi(), lpsubkeyname.param().abi(), lpnewkeyname.param().abi())
}
#[inline]
pub unsafe fn RegReplaceKeyA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegReplaceKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpnewfile : windows_core::PCSTR, lpoldfile : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegReplaceKeyA(hkey.param().abi(), lpsubkey.param().abi(), lpnewfile.param().abi(), lpoldfile.param().abi())
}
#[inline]
pub unsafe fn RegReplaceKeyW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegReplaceKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpnewfile : windows_core::PCWSTR, lpoldfile : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegReplaceKeyW(hkey.param().abi(), lpsubkey.param().abi(), lpnewfile.param().abi(), lpoldfile.param().abi())
}
#[inline]
pub unsafe fn RegRestoreKeyA<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegRestoreKeyA(hkey : HKEY, lpfile : windows_core::PCSTR, dwflags : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegRestoreKeyA(hkey.param().abi(), lpfile.param().abi(), dwflags.0 as _)
}
#[inline]
pub unsafe fn RegRestoreKeyW<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegRestoreKeyW(hkey : HKEY, lpfile : windows_core::PCWSTR, dwflags : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegRestoreKeyW(hkey.param().abi(), lpfile.param().abi(), dwflags.0 as _)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegSaveKeyA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyA(hkey : HKEY, lpfile : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyA(hkey.param().abi(), lpfile.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegSaveKeyExA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyExA(hkey : HKEY, lpfile : windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flags : REG_SAVE_FORMAT) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyExA(hkey.param().abi(), lpfile.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), flags)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegSaveKeyExW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyExW(hkey : HKEY, lpfile : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flags : REG_SAVE_FORMAT) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyExW(hkey.param().abi(), lpfile.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), flags)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegSaveKeyW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyW(hkey : HKEY, lpfile : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyW(hkey.param().abi(), lpfile.param().abi(), core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn RegSetKeySecurity<P0, P1>(hkey: P0, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetKeySecurity(hkey : HKEY, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeySecurity(hkey.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn RegSetKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetKeyValueA(hkey : HKEY, lpsubkey : windows_core::PCSTR, lpvaluename : windows_core::PCSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeyValueA(hkey.param().abi(), lpsubkey.param().abi(), lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), cbdata)
}
#[inline]
pub unsafe fn RegSetKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeyValueW(hkey.param().abi(), lpsubkey.param().abi(), lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), cbdata)
}
#[inline]
pub unsafe fn RegSetValueA<P0, P1>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetValueA(hkey : HKEY, lpsubkey : windows_core::PCSTR, dwtype : REG_VALUE_TYPE, lpdata : windows_core::PCSTR, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueA(hkey.param().abi(), lpsubkey.param().abi(), dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RegSetValueExA<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetValueExA(hkey : HKEY, lpvaluename : windows_core::PCSTR, reserved : u32, dwtype : REG_VALUE_TYPE, lpdata : *const u8, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueExA(hkey.param().abi(), lpvaluename.param().abi(), reserved, dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RegSetValueExW<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : HKEY, lpvaluename : windows_core::PCWSTR, reserved : u32, dwtype : REG_VALUE_TYPE, lpdata : *const u8, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueExW(hkey.param().abi(), lpvaluename.param().abi(), reserved, dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn RegSetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: P2, cbdata: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegSetValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, dwtype : REG_VALUE_TYPE, lpdata : windows_core::PCWSTR, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueW(hkey.param().abi(), lpsubkey.param().abi(), dwtype, lpdata.param().abi(), cbdata)
}
#[inline]
pub unsafe fn RegUnLoadKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegUnLoadKeyA(hkey : HKEY, lpsubkey : windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegUnLoadKeyA(hkey.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn RegUnLoadKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<HKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegUnLoadKeyW(hkey : HKEY, lpsubkey : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegUnLoadKeyW(hkey.param().abi(), lpsubkey.param().abi())
}
pub const AGP_FLAG_NO_1X_RATE: i32 = 1i32;
pub const AGP_FLAG_NO_2X_RATE: i32 = 2i32;
pub const AGP_FLAG_NO_4X_RATE: i32 = 4i32;
pub const AGP_FLAG_NO_8X_RATE: i32 = 8i32;
pub const AGP_FLAG_NO_FW_ENABLE: i32 = 512i32;
pub const AGP_FLAG_NO_SBA_ENABLE: i32 = 256i32;
pub const AGP_FLAG_REVERSE_INITIALIZATION: i32 = 128i32;
pub const AGP_FLAG_SPECIAL_RESERVE: i32 = 1015808i32;
pub const AGP_FLAG_SPECIAL_TARGET: i32 = 1048575i32;
pub const APMMENUSUSPEND_DISABLED: u32 = 0u32;
pub const APMMENUSUSPEND_ENABLED: u32 = 1u32;
pub const APMMENUSUSPEND_NOCHANGE: u32 = 128u32;
pub const APMMENUSUSPEND_UNDOCKED: u32 = 2u32;
pub const APMTIMEOUT_DISABLED: u32 = 0u32;
pub const BIF_RAWDEVICENEEDSDRIVER: u32 = 2u32;
pub const BIF_SHOWSIMILARDRIVERS: u32 = 1u32;
pub const CSCONFIGFLAG_BITS: u32 = 7u32;
pub const CSCONFIGFLAG_DISABLED: u32 = 1u32;
pub const CSCONFIGFLAG_DO_NOT_CREATE: u32 = 2u32;
pub const CSCONFIGFLAG_DO_NOT_START: u32 = 4u32;
pub const DMSTATEFLAG_APPLYTOALL: u32 = 1u32;
pub const DOSOPTF_ALWAYSUSE: i32 = 4i32;
pub const DOSOPTF_DEFAULT: i32 = 1i32;
pub const DOSOPTF_INDOSSTART: i32 = 64i32;
pub const DOSOPTF_MULTIPLE: i32 = 128i32;
pub const DOSOPTF_NEEDSETUP: i32 = 32i32;
pub const DOSOPTF_PROVIDESUMB: i32 = 16i32;
pub const DOSOPTF_SUPPORTED: i32 = 2i32;
pub const DOSOPTF_USESPMODE: i32 = 8i32;
pub const DOSOPTGF_DEFCLEAN: i32 = 1i32;
pub const DRIVERSIGN_BLOCKING: u32 = 2u32;
pub const DRIVERSIGN_NONE: u32 = 0u32;
pub const DRIVERSIGN_WARNING: u32 = 1u32;
pub const DTRESULTFIX: u32 = 1u32;
pub const DTRESULTOK: u32 = 0u32;
pub const DTRESULTPART: u32 = 3u32;
pub const DTRESULTPROB: u32 = 2u32;
pub const EISAFLAG_NO_IO_MERGE: u32 = 1u32;
pub const EISAFLAG_SLOT_IO_FIRST: u32 = 2u32;
pub const EISA_NO_MAX_FUNCTION: u32 = 255u32;
pub const HKEY_CLASSES_ROOT: HKEY = HKEY(-2147483648i32 as _);
pub const HKEY_CURRENT_CONFIG: HKEY = HKEY(-2147483643i32 as _);
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647i32 as _);
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = HKEY(-2147483641i32 as _);
pub const HKEY_DYN_DATA: HKEY = HKEY(-2147483642i32 as _);
pub const HKEY_LOCAL_MACHINE: HKEY = HKEY(-2147483646i32 as _);
pub const HKEY_PERFORMANCE_DATA: HKEY = HKEY(-2147483644i32 as _);
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = HKEY(-2147483552i32 as _);
pub const HKEY_PERFORMANCE_TEXT: HKEY = HKEY(-2147483568i32 as _);
pub const HKEY_USERS: HKEY = HKEY(-2147483645i32 as _);
pub const IT_COMPACT: u32 = 0u32;
pub const IT_CUSTOM: u32 = 3u32;
pub const IT_PORTABLE: u32 = 2u32;
pub const IT_TYPICAL: u32 = 1u32;
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = REG_SAM_FLAGS(983103u32);
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = REG_SAM_FLAGS(32u32);
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(4u32);
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = REG_SAM_FLAGS(8u32);
pub const KEY_EXECUTE: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_NOTIFY: REG_SAM_FLAGS = REG_SAM_FLAGS(16u32);
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(1u32);
pub const KEY_READ: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_SET_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(2u32);
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(512u32);
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(256u32);
pub const KEY_WOW64_RES: REG_SAM_FLAGS = REG_SAM_FLAGS(768u32);
pub const KEY_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(131078u32);
pub const LASTGOOD_OPERATION: u32 = 255u32;
pub const LASTGOOD_OPERATION_DELETE: u32 = 1u32;
pub const LASTGOOD_OPERATION_NOPOSTPROC: u32 = 0u32;
pub const MF_FLAGS_CREATE_BUT_NO_SHOW_DISABLED: u32 = 8u32;
pub const MF_FLAGS_EVEN_IF_NO_RESOURCE: u32 = 1u32;
pub const MF_FLAGS_FILL_IN_UNKNOWN_RESOURCE: u32 = 4u32;
pub const MF_FLAGS_NO_CREATE_IF_NO_RESOURCE: u32 = 2u32;
pub const NUM_EISA_RANGES: u32 = 4u32;
pub const NUM_RESOURCE_MAP: u32 = 256u32;
pub const PCIC_DEFAULT_IRQMASK: u32 = 20152u32;
pub const PCIC_DEFAULT_NUMSOCKETS: u32 = 0u32;
pub const PCI_OPTIONS_USE_BIOS: i32 = 1i32;
pub const PCI_OPTIONS_USE_IRQ_STEERING: i32 = 2i32;
pub const PCMCIA_DEF_MEMBEGIN: u32 = 786432u32;
pub const PCMCIA_DEF_MEMEND: u32 = 16777215u32;
pub const PCMCIA_DEF_MEMLEN: u32 = 4096u32;
pub const PCMCIA_DEF_MIN_REGION: u32 = 65536u32;
pub const PCMCIA_OPT_AUTOMEM: i32 = 4i32;
pub const PCMCIA_OPT_HAVE_SOCKET: i32 = 1i32;
pub const PCMCIA_OPT_NO_APMREMOVE: i32 = 32i32;
pub const PCMCIA_OPT_NO_AUDIO: i32 = 16i32;
pub const PCMCIA_OPT_NO_SOUND: i32 = 8i32;
pub const PIR_OPTION_DEFAULT: u32 = 15u32;
pub const PIR_OPTION_ENABLED: u32 = 1u32;
pub const PIR_OPTION_MSSPEC: u32 = 4u32;
pub const PIR_OPTION_REALMODE: u32 = 8u32;
pub const PIR_OPTION_REGISTRY: u32 = 2u32;
pub const PIR_STATUS_DISABLED: u32 = 2u32;
pub const PIR_STATUS_ENABLED: u32 = 1u32;
pub const PIR_STATUS_ERROR: u32 = 0u32;
pub const PIR_STATUS_MAX: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_COMPATIBLE: u32 = 1u32;
pub const PIR_STATUS_MINIPORT_ERROR: u32 = 4u32;
pub const PIR_STATUS_MINIPORT_INVALID: u32 = 7u32;
pub const PIR_STATUS_MINIPORT_MAX: u32 = 8u32;
pub const PIR_STATUS_MINIPORT_NOKEY: u32 = 5u32;
pub const PIR_STATUS_MINIPORT_NONE: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_NORMAL: u32 = 0u32;
pub const PIR_STATUS_MINIPORT_OVERRIDE: u32 = 2u32;
pub const PIR_STATUS_MINIPORT_SUCCESS: u32 = 6u32;
pub const PIR_STATUS_TABLE_BAD: u32 = 5u32;
pub const PIR_STATUS_TABLE_ERROR: u32 = 4u32;
pub const PIR_STATUS_TABLE_MAX: u32 = 7u32;
pub const PIR_STATUS_TABLE_MSSPEC: u32 = 1u32;
pub const PIR_STATUS_TABLE_NONE: u32 = 3u32;
pub const PIR_STATUS_TABLE_REALMODE: u32 = 2u32;
pub const PIR_STATUS_TABLE_REGISTRY: u32 = 0u32;
pub const PIR_STATUS_TABLE_SUCCESS: u32 = 6u32;
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1u32;
pub const REGDF_CONFLICTDMA: u32 = 524288u32;
pub const REGDF_CONFLICTIO: u32 = 65536u32;
pub const REGDF_CONFLICTIRQ: u32 = 262144u32;
pub const REGDF_CONFLICTMEM: u32 = 131072u32;
pub const REGDF_GENFORCEDCONFIG: u32 = 32u32;
pub const REGDF_MAPIRQ2TO9: u32 = 1048576u32;
pub const REGDF_NEEDFULLCONFIG: u32 = 16u32;
pub const REGDF_NODETCONFIG: u32 = 32768u32;
pub const REGDF_NOTDETDMA: u32 = 8u32;
pub const REGDF_NOTDETIO: u32 = 1u32;
pub const REGDF_NOTDETIRQ: u32 = 4u32;
pub const REGDF_NOTDETMEM: u32 = 2u32;
pub const REGDF_NOTVERIFIED: u32 = 2147483648u32;
pub const REGSTR_DATA_NETOS_IPX: windows_core::PCWSTR = windows_core::w!("IPX");
pub const REGSTR_DATA_NETOS_NDIS: windows_core::PCWSTR = windows_core::w!("NDIS");
pub const REGSTR_DATA_NETOS_ODI: windows_core::PCWSTR = windows_core::w!("ODI");
pub const REGSTR_DEFAULT_INSTANCE: windows_core::PCWSTR = windows_core::w!("0000");
pub const REGSTR_KEY_ACPIENUM: windows_core::PCWSTR = windows_core::w!("ACPI");
pub const REGSTR_KEY_APM: windows_core::PCWSTR = windows_core::w!("*PNP0C05");
pub const REGSTR_KEY_BIOSENUM: windows_core::PCWSTR = windows_core::w!("BIOS");
pub const REGSTR_KEY_CLASS: windows_core::PCWSTR = windows_core::w!("Class");
pub const REGSTR_KEY_CONFIG: windows_core::PCWSTR = windows_core::w!("Config");
pub const REGSTR_KEY_CONTROL: windows_core::PCWSTR = windows_core::w!("Control");
pub const REGSTR_KEY_CRASHES: windows_core::PCWSTR = windows_core::w!("Crashes");
pub const REGSTR_KEY_CURRENT: windows_core::PCWSTR = windows_core::w!("Current");
pub const REGSTR_KEY_CURRENT_ENV: windows_core::PCWSTR = windows_core::w!("\\Windows 4.0");
pub const REGSTR_KEY_DANGERS: windows_core::PCWSTR = windows_core::w!("Dangers");
pub const REGSTR_KEY_DEFAULT: windows_core::PCWSTR = windows_core::w!("Default");
pub const REGSTR_KEY_DETMODVARS: windows_core::PCWSTR = windows_core::w!("DetModVars");
pub const REGSTR_KEY_DEVICEPARAMETERS: windows_core::PCWSTR = windows_core::w!("Device Parameters");
pub const REGSTR_KEY_DEVICE_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const REGSTR_KEY_DISPLAY_CLASS: windows_core::PCWSTR = windows_core::w!("Display");
pub const REGSTR_KEY_DOSOPTCDROM: windows_core::PCWSTR = windows_core::w!("CD-ROM");
pub const REGSTR_KEY_DOSOPTMOUSE: windows_core::PCWSTR = windows_core::w!("MOUSE");
pub const REGSTR_KEY_DRIVERPARAMETERS: windows_core::PCWSTR = windows_core::w!("Driver Parameters");
pub const REGSTR_KEY_DRIVERS: windows_core::PCWSTR = windows_core::w!("\\Drivers");
pub const REGSTR_KEY_EBDAUTOEXECBATKEYBOARD: windows_core::PCWSTR = windows_core::w!("EBDAutoexecBatKeyboard");
pub const REGSTR_KEY_EBDAUTOEXECBATLOCAL: windows_core::PCWSTR = windows_core::w!("EBDAutoexecBatLocale");
pub const REGSTR_KEY_EBDCONFIGSYSKEYBOARD: windows_core::PCWSTR = windows_core::w!("EBDConfigSysKeyboard");
pub const REGSTR_KEY_EBDCONFIGSYSLOCAL: windows_core::PCWSTR = windows_core::w!("EBDConfigSysLocale");
pub const REGSTR_KEY_EBDFILESKEYBOARD: windows_core::PCWSTR = windows_core::w!("EBDFilesKeyboard");
pub const REGSTR_KEY_EBDFILESLOCAL: windows_core::PCWSTR = windows_core::w!("EBDFilesLocale");
pub const REGSTR_KEY_EISAENUM: windows_core::PCWSTR = windows_core::w!("EISA");
pub const REGSTR_KEY_ENUM: windows_core::PCWSTR = windows_core::w!("Enum");
pub const REGSTR_KEY_EXPLORER: windows_core::PCWSTR = windows_core::w!("Explorer");
pub const REGSTR_KEY_FILTERS: windows_core::PCWSTR = windows_core::w!("Filters");
pub const REGSTR_KEY_INIUPDATE: windows_core::PCWSTR = windows_core::w!("IniUpdate");
pub const REGSTR_KEY_ISAENUM: windows_core::PCWSTR = windows_core::w!("ISAPnP");
pub const REGSTR_KEY_JOYCURR: windows_core::PCWSTR = windows_core::w!("CurrentJoystickSettings");
pub const REGSTR_KEY_JOYSETTINGS: windows_core::PCWSTR = windows_core::w!("JoystickSettings");
pub const REGSTR_KEY_KEYBOARD_CLASS: windows_core::PCWSTR = windows_core::w!("Keyboard");
pub const REGSTR_KEY_KNOWNDOCKINGSTATES: windows_core::PCWSTR = windows_core::w!("Hardware Profiles");
pub const REGSTR_KEY_LOGCONFIG: windows_core::PCWSTR = windows_core::w!("LogConfig");
pub const REGSTR_KEY_LOGON: windows_core::PCWSTR = windows_core::w!("\\Logon");
pub const REGSTR_KEY_LOWER_FILTER_LEVEL_DEFAULT: windows_core::PCWSTR = windows_core::w!("*Lower");
pub const REGSTR_KEY_MEDIA_CLASS: windows_core::PCWSTR = windows_core::w!("MEDIA");
pub const REGSTR_KEY_MODEM_CLASS: windows_core::PCWSTR = windows_core::w!("Modem");
pub const REGSTR_KEY_MODES: windows_core::PCWSTR = windows_core::w!("Modes");
pub const REGSTR_KEY_MONITOR_CLASS: windows_core::PCWSTR = windows_core::w!("Monitor");
pub const REGSTR_KEY_MOUSE_CLASS: windows_core::PCWSTR = windows_core::w!("Mouse");
pub const REGSTR_KEY_NDISINFO: windows_core::PCWSTR = windows_core::w!("NDISInfo");
pub const REGSTR_KEY_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const REGSTR_KEY_NETWORKPROVIDER: windows_core::PCWSTR = windows_core::w!("\\NetworkProvider");
pub const REGSTR_KEY_NETWORK_PERSISTENT: windows_core::PCWSTR = windows_core::w!("\\Persistent");
pub const REGSTR_KEY_NETWORK_RECENT: windows_core::PCWSTR = windows_core::w!("\\Recent");
pub const REGSTR_KEY_OVERRIDE: windows_core::PCWSTR = windows_core::w!("Override");
pub const REGSTR_KEY_PCIENUM: windows_core::PCWSTR = windows_core::w!("PCI");
pub const REGSTR_KEY_PCMCIA: windows_core::PCWSTR = windows_core::w!("PCMCIA\\");
pub const REGSTR_KEY_PCMCIAENUM: windows_core::PCWSTR = windows_core::w!("PCMCIA");
pub const REGSTR_KEY_PCMCIA_CLASS: windows_core::PCWSTR = windows_core::w!("PCMCIA");
pub const REGSTR_KEY_PCMTD: windows_core::PCWSTR = windows_core::w!("MTD-");
pub const REGSTR_KEY_PCUNKNOWN: windows_core::PCWSTR = windows_core::w!("UNKNOWN_MANUFACTURER");
pub const REGSTR_KEY_POL_COMPUTERS: windows_core::PCWSTR = windows_core::w!("Computers");
pub const REGSTR_KEY_POL_DEFAULT: windows_core::PCWSTR = windows_core::w!(".default");
pub const REGSTR_KEY_POL_USERGROUPDATA: windows_core::PCWSTR = windows_core::w!("GroupData\\UserGroups\\Priority");
pub const REGSTR_KEY_POL_USERGROUPS: windows_core::PCWSTR = windows_core::w!("UserGroups");
pub const REGSTR_KEY_POL_USERS: windows_core::PCWSTR = windows_core::w!("Users");
pub const REGSTR_KEY_PORTS_CLASS: windows_core::PCWSTR = windows_core::w!("ports");
pub const REGSTR_KEY_PRINTERS: windows_core::PCWSTR = windows_core::w!("Printers");
pub const REGSTR_KEY_PRINT_PROC: windows_core::PCWSTR = windows_core::w!("\\Print Processors");
pub const REGSTR_KEY_ROOTENUM: windows_core::PCWSTR = windows_core::w!("Root");
pub const REGSTR_KEY_RUNHISTORY: windows_core::PCWSTR = windows_core::w!("RunHistory");
pub const REGSTR_KEY_SCSI_CLASS: windows_core::PCWSTR = windows_core::w!("SCSIAdapter");
pub const REGSTR_KEY_SETUP: windows_core::PCWSTR = windows_core::w!("\\Setup");
pub const REGSTR_KEY_SHARES: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\LanMan");
pub const REGSTR_KEY_SYSTEM: windows_core::PCWSTR = windows_core::w!("System");
pub const REGSTR_KEY_SYSTEMBOARD: windows_core::PCWSTR = windows_core::w!("*PNP0C01");
pub const REGSTR_KEY_UPPER_FILTER_LEVEL_DEFAULT: windows_core::PCWSTR = windows_core::w!("*Upper");
pub const REGSTR_KEY_USER: windows_core::PCWSTR = windows_core::w!("User");
pub const REGSTR_KEY_VPOWERDENUM: windows_core::PCWSTR = windows_core::w!("VPOWERD");
pub const REGSTR_KEY_WINOLDAPP: windows_core::PCWSTR = windows_core::w!("WinOldApp");
pub const REGSTR_MACHTYPE_ATT_PC: windows_core::PCWSTR = windows_core::w!("AT&T PC");
pub const REGSTR_MACHTYPE_HP_VECTRA: windows_core::PCWSTR = windows_core::w!("HP Vectra");
pub const REGSTR_MACHTYPE_IBMPC: windows_core::PCWSTR = windows_core::w!("IBM PC");
pub const REGSTR_MACHTYPE_IBMPCAT: windows_core::PCWSTR = windows_core::w!("IBM PC/AT");
pub const REGSTR_MACHTYPE_IBMPCCONV: windows_core::PCWSTR = windows_core::w!("IBM PC Convertible");
pub const REGSTR_MACHTYPE_IBMPCJR: windows_core::PCWSTR = windows_core::w!("IBM PCjr");
pub const REGSTR_MACHTYPE_IBMPCXT: windows_core::PCWSTR = windows_core::w!("IBM PC/XT");
pub const REGSTR_MACHTYPE_IBMPCXT_286: windows_core::PCWSTR = windows_core::w!("IBM PC/XT 286");
pub const REGSTR_MACHTYPE_IBMPS1: windows_core::PCWSTR = windows_core::w!("IBM PS/1");
pub const REGSTR_MACHTYPE_IBMPS2_25: windows_core::PCWSTR = windows_core::w!("IBM PS/2-25");
pub const REGSTR_MACHTYPE_IBMPS2_30: windows_core::PCWSTR = windows_core::w!("IBM PS/2-30");
pub const REGSTR_MACHTYPE_IBMPS2_30_286: windows_core::PCWSTR = windows_core::w!("IBM PS/2-30 286");
pub const REGSTR_MACHTYPE_IBMPS2_50: windows_core::PCWSTR = windows_core::w!("IBM PS/2-50");
pub const REGSTR_MACHTYPE_IBMPS2_50Z: windows_core::PCWSTR = windows_core::w!("IBM PS/2-50Z");
pub const REGSTR_MACHTYPE_IBMPS2_55SX: windows_core::PCWSTR = windows_core::w!("IBM PS/2-55SX");
pub const REGSTR_MACHTYPE_IBMPS2_60: windows_core::PCWSTR = windows_core::w!("IBM PS/2-60");
pub const REGSTR_MACHTYPE_IBMPS2_65SX: windows_core::PCWSTR = windows_core::w!("IBM PS/2-65SX");
pub const REGSTR_MACHTYPE_IBMPS2_70: windows_core::PCWSTR = windows_core::w!("IBM PS/2-70");
pub const REGSTR_MACHTYPE_IBMPS2_70_80: windows_core::PCWSTR = windows_core::w!("IBM PS/2-70/80");
pub const REGSTR_MACHTYPE_IBMPS2_80: windows_core::PCWSTR = windows_core::w!("IBM PS/2-80");
pub const REGSTR_MACHTYPE_IBMPS2_90: windows_core::PCWSTR = windows_core::w!("IBM PS/2-90");
pub const REGSTR_MACHTYPE_IBMPS2_P70: windows_core::PCWSTR = windows_core::w!("IBM PS/2-P70");
pub const REGSTR_MACHTYPE_PHOENIX_PCAT: windows_core::PCWSTR = windows_core::w!("Phoenix PC/AT Compatible");
pub const REGSTR_MACHTYPE_UNKNOWN: windows_core::PCWSTR = windows_core::w!("Unknown");
pub const REGSTR_MACHTYPE_ZENITH_PC: windows_core::PCWSTR = windows_core::w!("Zenith PC");
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
pub const REGSTR_PATH_ADDRARB: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\AddrArb");
pub const REGSTR_PATH_AEDEBUG: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\AeDebug");
pub const REGSTR_PATH_APPEARANCE: windows_core::PCWSTR = windows_core::w!("Control Panel\\Appearance");
pub const REGSTR_PATH_APPPATCH: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\AppPatches");
pub const REGSTR_PATH_APPPATHS: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\App Paths");
pub const REGSTR_PATH_BIOSINFO: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\BiosInfo");
pub const REGSTR_PATH_BUSINFORMATION: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\PnP\\BusInformation");
pub const REGSTR_PATH_CDFS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem\\CDFS");
pub const REGSTR_PATH_CHECKBADAPPS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps");
pub const REGSTR_PATH_CHECKBADAPPS400: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps400");
pub const REGSTR_PATH_CHECKDISK: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive");
pub const REGSTR_PATH_CHECKDISKSET: windows_core::PCWSTR = windows_core::w!("Settings");
pub const REGSTR_PATH_CHECKDISKUDRVS: windows_core::PCWSTR = windows_core::w!("NoUnknownDDErrDrvs");
pub const REGSTR_PATH_CHECKVERDLLS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckVerDLLs");
pub const REGSTR_PATH_CHILD_PREFIX: windows_core::PCWSTR = windows_core::w!("Child");
pub const REGSTR_PATH_CHKLASTCHECK: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastCheck");
pub const REGSTR_PATH_CHKLASTSURFAN: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastSurfaceAnalysis");
pub const REGSTR_PATH_CLASS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\Class");
pub const REGSTR_PATH_CLASS_NT: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Class");
pub const REGSTR_PATH_CODEPAGE: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Nls\\Codepage");
pub const REGSTR_PATH_CODEVICEINSTALLERS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\CoDeviceInstallers");
pub const REGSTR_PATH_COLORS: windows_core::PCWSTR = windows_core::w!("Control Panel\\Colors");
pub const REGSTR_PATH_COMPUTRNAME: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\ComputerName\\ComputerName");
pub const REGSTR_PATH_CONTROLPANEL: windows_core::PCWSTR = windows_core::w!("Control Panel");
pub const REGSTR_PATH_CONTROLSFOLDER: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Controls Folder");
pub const REGSTR_PATH_CRITICALDEVICEDATABASE: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\CriticalDeviceDatabase");
pub const REGSTR_PATH_CURRENTCONTROLSET: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet");
pub const REGSTR_PATH_CURRENT_CONTROL_SET: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control");
pub const REGSTR_PATH_CURSORS: windows_core::PCWSTR = windows_core::w!("Control Panel\\Cursors");
pub const REGSTR_PATH_CVNETWORK: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network");
pub const REGSTR_PATH_DESKTOP: windows_core::PCWSTR = windows_core::w!("Control Panel\\Desktop");
pub const REGSTR_PATH_DETECT: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Detect");
pub const REGSTR_PATH_DEVICEINSTALLER: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Device Installer");
pub const REGSTR_PATH_DEVICE_CLASSES: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\DeviceClasses");
pub const REGSTR_PATH_DIFX: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\DIFX");
pub const REGSTR_PATH_DISPLAYSETTINGS: windows_core::PCWSTR = windows_core::w!("Display\\Settings");
pub const REGSTR_PATH_DMAARB: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\DMAArb");
pub const REGSTR_PATH_DRIVERSIGN: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Driver Signing");
pub const REGSTR_PATH_DRIVERSIGN_POLICY: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Driver Signing");
pub const REGSTR_PATH_ENUM: windows_core::PCWSTR = windows_core::w!("Enum");
pub const REGSTR_PATH_ENVIRONMENTS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Environments");
pub const REGSTR_PATH_EVENTLABELS: windows_core::PCWSTR = windows_core::w!("AppEvents\\EventLabels");
pub const REGSTR_PATH_EXPLORER: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer");
pub const REGSTR_PATH_FAULT: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Fault");
pub const REGSTR_PATH_FILESYSTEM: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem");
pub const REGSTR_PATH_FILESYSTEM_NOVOLTRACK: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem\\NoVolTrack");
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR: windows_core::PCWSTR = windows_core::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor");
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR0: windows_core::PCWSTR = windows_core::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor\\0");
pub const REGSTR_PATH_FONTS: windows_core::PCWSTR = windows_core::w!("Display\\Fonts");
pub const REGSTR_PATH_GRPCONV: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\GrpConv");
pub const REGSTR_PATH_HACKINIFILE: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\HackIniFiles");
pub const REGSTR_PATH_HWPROFILES: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Hardware Profiles");
pub const REGSTR_PATH_HWPROFILESCURRENT: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Hardware Profiles\\Current");
pub const REGSTR_PATH_ICONS: windows_core::PCWSTR = windows_core::w!("Control Panel\\Icons");
pub const REGSTR_PATH_IDCONFIGDB: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\IDConfigDB");
pub const REGSTR_PATH_INSTALLEDFILES: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\InstalledFiles");
pub const REGSTR_PATH_IOARB: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IOArb");
pub const REGSTR_PATH_IOS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\IOS");
pub const REGSTR_PATH_IRQARB: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IRQArb");
pub const REGSTR_PATH_KEYBOARD: windows_core::PCWSTR = windows_core::w!("Control Panel\\Keyboard");
pub const REGSTR_PATH_KNOWN16DLLS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\Known16DLLs");
pub const REGSTR_PATH_KNOWNDLLS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownDLLs");
pub const REGSTR_PATH_KNOWNVXDS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownVxDs");
pub const REGSTR_PATH_LASTBACKUP: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastBackup");
pub const REGSTR_PATH_LASTCHECK: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastCheck");
pub const REGSTR_PATH_LASTGOOD: windows_core::PCWSTR = windows_core::w!("System\\LastKnownGoodRecovery\\LastGood");
pub const REGSTR_PATH_LASTGOODTMP: windows_core::PCWSTR = windows_core::w!("System\\LastKnownGoodRecovery\\LastGood.Tmp");
pub const REGSTR_PATH_LASTOPTIMIZE: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastOptimize");
pub const REGSTR_PATH_LOOKSCHEMES: windows_core::PCWSTR = windows_core::w!("Control Panel\\Appearance\\Schemes");
pub const REGSTR_PATH_METRICS: windows_core::PCWSTR = windows_core::w!("Control Panel\\Desktop\\WindowMetrics");
pub const REGSTR_PATH_MONITORS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Monitors");
pub const REGSTR_PATH_MOUSE: windows_core::PCWSTR = windows_core::w!("Control Panel\\Mouse");
pub const REGSTR_PATH_MSDOSOPTS: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSOptions");
pub const REGSTR_PATH_MULTIMEDIA_AUDIO: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Multimedia\\Audio");
pub const REGSTR_PATH_MULTI_FUNCTION: windows_core::PCWSTR = windows_core::w!("MF");
pub const REGSTR_PATH_NCPSERVER: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\NcpServer\\Parameters");
pub const REGSTR_PATH_NETEQUIV: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Equivalent");
pub const REGSTR_PATH_NETWORK_USERSETTINGS: windows_core::PCWSTR = windows_core::w!("Network");
pub const REGSTR_PATH_NEWDOSBOX: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSSpecialConfig");
pub const REGSTR_PATH_NONDRIVERSIGN: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Non-Driver Signing");
pub const REGSTR_PATH_NONDRIVERSIGN_POLICY: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Non-Driver Signing");
pub const REGSTR_PATH_NOSUGGMSDOS: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\NoMSDOSWarn");
pub const REGSTR_PATH_NT_CURRENTVERSION: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion");
pub const REGSTR_PATH_NWREDIR: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\NWREDIR");
pub const REGSTR_PATH_PCIIR: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Pnp\\PciIrqRouting");
pub const REGSTR_PATH_PER_HW_ID_STORAGE: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\PerHwIdStorage");
pub const REGSTR_PATH_PIFCONVERT: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PIFConvert");
pub const REGSTR_PATH_POLICIES: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Policies");
pub const REGSTR_PATH_PRINT: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Print");
pub const REGSTR_PATH_PRINTERS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Printers");
pub const REGSTR_PATH_PROPERTYSYSTEM: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PropertySystem");
pub const REGSTR_PATH_PROVIDERS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Providers");
pub const REGSTR_PATH_PWDPROVIDER: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\PwdProvider");
pub const REGSTR_PATH_REALMODENET: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Real Mode Net");
pub const REGSTR_PATH_REINSTALL: windows_core::PCWSTR = windows_core::w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Reinstall");
pub const REGSTR_PATH_RELIABILITY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Reliability");
pub const REGSTR_PATH_RELIABILITY_POLICY: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Reliability");
pub const REGSTR_PATH_RELIABILITY_POLICY_REPORTSNAPSHOT: windows_core::PCWSTR = windows_core::w!("ReportSnapshot");
pub const REGSTR_PATH_RELIABILITY_POLICY_SHUTDOWNREASONUI: windows_core::PCWSTR = windows_core::w!("ShutdownReasonUI");
pub const REGSTR_PATH_RELIABILITY_POLICY_SNAPSHOT: windows_core::PCWSTR = windows_core::w!("Snapshot");
pub const REGSTR_PATH_ROOT: windows_core::PCWSTR = windows_core::w!("Enum\\Root");
pub const REGSTR_PATH_RUN: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
pub const REGSTR_PATH_RUNONCE: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce");
pub const REGSTR_PATH_RUNONCEEX: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx");
pub const REGSTR_PATH_RUNSERVICES: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServices");
pub const REGSTR_PATH_RUNSERVICESONCE: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServicesOnce");
pub const REGSTR_PATH_SCHEMES: windows_core::PCWSTR = windows_core::w!("AppEvents\\Schemes");
pub const REGSTR_PATH_SCREENSAVE: windows_core::PCWSTR = windows_core::w!("Control Panel\\Desktop");
pub const REGSTR_PATH_SERVICES: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services");
pub const REGSTR_PATH_SETUP: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion");
pub const REGSTR_PATH_SHUTDOWN: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Shutdown");
pub const REGSTR_PATH_SOUND: windows_core::PCWSTR = windows_core::w!("Control Panel\\Sound");
pub const REGSTR_PATH_SYSTEMENUM: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Enum");
pub const REGSTR_PATH_SYSTRAY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\SysTray");
pub const REGSTR_PATH_TIMEZONE: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\TimeZoneInformation");
pub const REGSTR_PATH_UNINSTALL: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall");
pub const REGSTR_PATH_UPDATE: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Update");
pub const REGSTR_PATH_VCOMM: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VCOMM");
pub const REGSTR_PATH_VMM: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VMM");
pub const REGSTR_PATH_VMM32FILES: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\VMM32Files");
pub const REGSTR_PATH_VNETSUP: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VNETSUP");
pub const REGSTR_PATH_VOLUMECACHE: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VolumeCaches");
pub const REGSTR_PATH_VPOWERD: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VPOWERD");
pub const REGSTR_PATH_VXD: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Services\\VxD");
pub const REGSTR_PATH_WARNVERDLLS: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\WarnVerDLLs");
pub const REGSTR_PATH_WINBOOT: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\WinBoot");
pub const REGSTR_PATH_WINDOWSAPPLETS: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets");
pub const REGSTR_PATH_WINLOGON: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Winlogon");
pub const REGSTR_PATH_WMI_SECURITY: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\Wmi\\Security");
pub const REGSTR_PCI_DUAL_IDE: windows_core::PCWSTR = windows_core::w!("PCIDualIDE");
pub const REGSTR_PCI_OPTIONS: windows_core::PCWSTR = windows_core::w!("Options");
pub const REGSTR_VALUE_DEFAULTLOC: windows_core::PCWSTR = windows_core::w!("UseDefaultNetLocation");
pub const REGSTR_VALUE_ENABLE: windows_core::PCWSTR = windows_core::w!("Enable");
pub const REGSTR_VALUE_LOWPOWERACTIVE: windows_core::PCWSTR = windows_core::w!("ScreenSaveLowPowerActive");
pub const REGSTR_VALUE_LOWPOWERTIMEOUT: windows_core::PCWSTR = windows_core::w!("ScreenSaveLowPowerTimeout");
pub const REGSTR_VALUE_NETPATH: windows_core::PCWSTR = windows_core::w!("NetworkPath");
pub const REGSTR_VALUE_POWEROFFACTIVE: windows_core::PCWSTR = windows_core::w!("ScreenSavePowerOffActive");
pub const REGSTR_VALUE_POWEROFFTIMEOUT: windows_core::PCWSTR = windows_core::w!("ScreenSavePowerOffTimeout");
pub const REGSTR_VALUE_SCRPASSWORD: windows_core::PCWSTR = windows_core::w!("ScreenSave_Data");
pub const REGSTR_VALUE_USESCRPASSWORD: windows_core::PCWSTR = windows_core::w!("ScreenSaveUsePassword");
pub const REGSTR_VALUE_VERBOSE: windows_core::PCWSTR = windows_core::w!("Verbose");
pub const REGSTR_VAL_ACDRIVESPINDOWN: windows_core::PCWSTR = windows_core::w!("ACDriveSpinDown");
pub const REGSTR_VAL_ACSPINDOWNPREVIOUS: windows_core::PCWSTR = windows_core::w!("ACSpinDownPrevious");
pub const REGSTR_VAL_ACTIVESERVICE: windows_core::PCWSTR = windows_core::w!("ActiveService");
pub const REGSTR_VAL_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const REGSTR_VAL_AEDEBUG_AUTO: windows_core::PCWSTR = windows_core::w!("Auto");
pub const REGSTR_VAL_AEDEBUG_DEBUGGER: windows_core::PCWSTR = windows_core::w!("Debugger");
pub const REGSTR_VAL_ALPHANUMPWDS: windows_core::PCWSTR = windows_core::w!("AlphanumPwds");
pub const REGSTR_VAL_APISUPPORT: windows_core::PCWSTR = windows_core::w!("APISupport");
pub const REGSTR_VAL_APMACTIMEOUT: windows_core::PCWSTR = windows_core::w!("APMACTimeout");
pub const REGSTR_VAL_APMBATTIMEOUT: windows_core::PCWSTR = windows_core::w!("APMBatTimeout");
pub const REGSTR_VAL_APMBIOSVER: windows_core::PCWSTR = windows_core::w!("APMBiosVer");
pub const REGSTR_VAL_APMFLAGS: windows_core::PCWSTR = windows_core::w!("APMFlags");
pub const REGSTR_VAL_APMMENUSUSPEND: windows_core::PCWSTR = windows_core::w!("APMMenuSuspend");
pub const REGSTR_VAL_APMSHUTDOWNPOWER: windows_core::PCWSTR = windows_core::w!("APMShutDownPower");
pub const REGSTR_VAL_APPINSTPATH: windows_core::PCWSTR = windows_core::w!("AppInstallPath");
pub const REGSTR_VAL_ASKFORCONFIG: windows_core::PCWSTR = windows_core::w!("AskForConfig");
pub const REGSTR_VAL_ASKFORCONFIGFUNC: windows_core::PCWSTR = windows_core::w!("AskForConfigFunc");
pub const REGSTR_VAL_ASYNCFILECOMMIT: windows_core::PCWSTR = windows_core::w!("AsyncFileCommit");
pub const REGSTR_VAL_AUDIO_BITMAP: windows_core::PCWSTR = windows_core::w!("bitmap");
pub const REGSTR_VAL_AUDIO_ICON: windows_core::PCWSTR = windows_core::w!("icon");
pub const REGSTR_VAL_AUTHENT_AGENT: windows_core::PCWSTR = windows_core::w!("AuthenticatingAgent");
pub const REGSTR_VAL_AUTOEXEC: windows_core::PCWSTR = windows_core::w!("Autoexec.Bat");
pub const REGSTR_VAL_AUTOINSNOTE: windows_core::PCWSTR = windows_core::w!("AutoInsertNotification");
pub const REGSTR_VAL_AUTOLOGON: windows_core::PCWSTR = windows_core::w!("AutoLogon");
pub const REGSTR_VAL_AUTOMOUNT: windows_core::PCWSTR = windows_core::w!("AutoMountDrives");
pub const REGSTR_VAL_AUTOSTART: windows_core::PCWSTR = windows_core::w!("AutoStart");
pub const REGSTR_VAL_BASICPROPERTIES: windows_core::PCWSTR = windows_core::w!("BasicProperties");
pub const REGSTR_VAL_BASICPROPERTIES_32: windows_core::PCWSTR = windows_core::w!("BasicProperties32");
pub const REGSTR_VAL_BATDRIVESPINDOWN: windows_core::PCWSTR = windows_core::w!("BatDriveSpinDown");
pub const REGSTR_VAL_BATSPINDOWNPREVIOUS: windows_core::PCWSTR = windows_core::w!("BatSpinDownPrevious");
pub const REGSTR_VAL_BEHAVIOR_ON_FAILED_VERIFY: windows_core::PCWSTR = windows_core::w!("BehaviorOnFailedVerify");
pub const REGSTR_VAL_BIOSDATE: windows_core::PCWSTR = windows_core::w!("BIOSDate");
pub const REGSTR_VAL_BIOSNAME: windows_core::PCWSTR = windows_core::w!("BIOSName");
pub const REGSTR_VAL_BIOSVERSION: windows_core::PCWSTR = windows_core::w!("BIOSVersion");
pub const REGSTR_VAL_BITSPERPIXEL: windows_core::PCWSTR = windows_core::w!("BitsPerPixel");
pub const REGSTR_VAL_BOOTCONFIG: windows_core::PCWSTR = windows_core::w!("BootConfig");
pub const REGSTR_VAL_BOOTCOUNT: windows_core::PCWSTR = windows_core::w!("BootCount");
pub const REGSTR_VAL_BOOTDIR: windows_core::PCWSTR = windows_core::w!("BootDir");
pub const REGSTR_VAL_BPP: windows_core::PCWSTR = windows_core::w!("BPP");
pub const REGSTR_VAL_BT: windows_core::PCWSTR = windows_core::w!("6005BT");
pub const REGSTR_VAL_BUFFAGETIMEOUT: windows_core::PCWSTR = windows_core::w!("BufferAgeTimeout");
pub const REGSTR_VAL_BUFFIDLETIMEOUT: windows_core::PCWSTR = windows_core::w!("BufferIdleTimeout");
pub const REGSTR_VAL_BUSTYPE: windows_core::PCWSTR = windows_core::w!("BusType");
pub const REGSTR_VAL_CAPABILITIES: windows_core::PCWSTR = windows_core::w!("Capabilities");
pub const REGSTR_VAL_CARDSPECIFIC: windows_core::PCWSTR = windows_core::w!("CardSpecific");
pub const REGSTR_VAL_CDCACHESIZE: windows_core::PCWSTR = windows_core::w!("CacheSize");
pub const REGSTR_VAL_CDCOMPATNAMES: windows_core::PCWSTR = windows_core::w!("MSCDEXCompatNames");
pub const REGSTR_VAL_CDEXTERRORS: windows_core::PCWSTR = windows_core::w!("ExtendedErrors");
pub const REGSTR_VAL_CDNOREADAHEAD: windows_core::PCWSTR = windows_core::w!("NoReadAhead");
pub const REGSTR_VAL_CDPREFETCH: windows_core::PCWSTR = windows_core::w!("Prefetch");
pub const REGSTR_VAL_CDPREFETCHTAIL: windows_core::PCWSTR = windows_core::w!("PrefetchTail");
pub const REGSTR_VAL_CDRAWCACHE: windows_core::PCWSTR = windows_core::w!("RawCache");
pub const REGSTR_VAL_CDROM: windows_core::PCWSTR = windows_core::w!("GenCD");
pub const REGSTR_VAL_CDROMCLASSNAME: windows_core::PCWSTR = windows_core::w!("CDROM");
pub const REGSTR_VAL_CDSHOWVERSIONS: windows_core::PCWSTR = windows_core::w!("ShowVersions");
pub const REGSTR_VAL_CDSVDSENSE: windows_core::PCWSTR = windows_core::w!("SVDSense");
pub const REGSTR_VAL_CHECKSUM: windows_core::PCWSTR = windows_core::w!("CurrentChecksum");
pub const REGSTR_VAL_CLASS: windows_core::PCWSTR = windows_core::w!("Class");
pub const REGSTR_VAL_CLASSDESC: windows_core::PCWSTR = windows_core::w!("ClassDesc");
pub const REGSTR_VAL_CLASSGUID: windows_core::PCWSTR = windows_core::w!("ClassGUID");
pub const REGSTR_VAL_CMDRIVFLAGS: windows_core::PCWSTR = windows_core::w!("CMDrivFlags");
pub const REGSTR_VAL_CMENUMFLAGS: windows_core::PCWSTR = windows_core::w!("CMEnumFlags");
pub const REGSTR_VAL_COINSTALLERS_32: windows_core::PCWSTR = windows_core::w!("CoInstallers32");
pub const REGSTR_VAL_COMINFO: windows_core::PCWSTR = windows_core::w!("ComInfo");
pub const REGSTR_VAL_COMMENT: windows_core::PCWSTR = windows_core::w!("Comment");
pub const REGSTR_VAL_COMPATIBLEIDS: windows_core::PCWSTR = windows_core::w!("CompatibleIDs");
pub const REGSTR_VAL_COMPRESSIONMETHOD: windows_core::PCWSTR = windows_core::w!("CompressionAlgorithm");
pub const REGSTR_VAL_COMPRESSIONTHRESHOLD: windows_core::PCWSTR = windows_core::w!("CompressionThreshold");
pub const REGSTR_VAL_COMPUTERNAME: windows_core::PCWSTR = windows_core::w!("ComputerName");
pub const REGSTR_VAL_COMPUTRNAME: windows_core::PCWSTR = windows_core::w!("ComputerName");
pub const REGSTR_VAL_COMVERIFYBASE: windows_core::PCWSTR = windows_core::w!("COMVerifyBase");
pub const REGSTR_VAL_CONFIG: windows_core::PCWSTR = windows_core::w!("ConfigPath");
pub const REGSTR_VAL_CONFIGFLAGS: windows_core::PCWSTR = windows_core::w!("ConfigFlags");
pub const REGSTR_VAL_CONFIGMG: windows_core::PCWSTR = windows_core::w!("CONFIGMG");
pub const REGSTR_VAL_CONFIGSYS: windows_core::PCWSTR = windows_core::w!("Config.Sys");
pub const REGSTR_VAL_CONNECTION_TYPE: windows_core::PCWSTR = windows_core::w!("ConnectionType");
pub const REGSTR_VAL_CONTAINERID: windows_core::PCWSTR = windows_core::w!("ContainerID");
pub const REGSTR_VAL_CONTIGFILEALLOC: windows_core::PCWSTR = windows_core::w!("ContigFileAllocSize");
pub const REGSTR_VAL_CONVMEM: windows_core::PCWSTR = windows_core::w!("ConvMem");
pub const REGSTR_VAL_CPU: windows_core::PCWSTR = windows_core::w!("CPU");
pub const REGSTR_VAL_CRASHFUNCS: windows_core::PCWSTR = windows_core::w!("CrashFuncs");
pub const REGSTR_VAL_CSCONFIGFLAGS: windows_core::PCWSTR = windows_core::w!("CSConfigFlags");
pub const REGSTR_VAL_CURCONFIG: windows_core::PCWSTR = windows_core::w!("CurrentConfig");
pub const REGSTR_VAL_CURDRVLET: windows_core::PCWSTR = windows_core::w!("CurrentDriveLetterAssignment");
pub const REGSTR_VAL_CURRENTCONFIG: windows_core::PCWSTR = windows_core::w!("CurrentConfig");
pub const REGSTR_VAL_CURRENT_BUILD: windows_core::PCWSTR = windows_core::w!("CurrentBuildNumber");
pub const REGSTR_VAL_CURRENT_CSDVERSION: windows_core::PCWSTR = windows_core::w!("CSDVersion");
pub const REGSTR_VAL_CURRENT_TYPE: windows_core::PCWSTR = windows_core::w!("CurrentType");
pub const REGSTR_VAL_CURRENT_USER: windows_core::PCWSTR = windows_core::w!("Current User");
pub const REGSTR_VAL_CURRENT_VERSION: windows_core::PCWSTR = windows_core::w!("CurrentVersion");
pub const REGSTR_VAL_CUSTOMCOLORS: windows_core::PCWSTR = windows_core::w!("CustomColors");
pub const REGSTR_VAL_CUSTOM_PROPERTY_CACHE_DATE: windows_core::PCWSTR = windows_core::w!("CustomPropertyCacheDate");
pub const REGSTR_VAL_CUSTOM_PROPERTY_HW_ID_KEY: windows_core::PCWSTR = windows_core::w!("CustomPropertyHwIdKey");
pub const REGSTR_VAL_DEFAULT: windows_core::PCWSTR = windows_core::w!("Default");
pub const REGSTR_VAL_DETCONFIG: windows_core::PCWSTR = windows_core::w!("DetConfig");
pub const REGSTR_VAL_DETECT: windows_core::PCWSTR = windows_core::w!("Detect");
pub const REGSTR_VAL_DETECTFUNC: windows_core::PCWSTR = windows_core::w!("DetectFunc");
pub const REGSTR_VAL_DETFLAGS: windows_core::PCWSTR = windows_core::w!("DetFlags");
pub const REGSTR_VAL_DETFUNC: windows_core::PCWSTR = windows_core::w!("DetFunc");
pub const REGSTR_VAL_DEVDESC: windows_core::PCWSTR = windows_core::w!("DeviceDesc");
pub const REGSTR_VAL_DEVICEDRIVER: windows_core::PCWSTR = windows_core::w!("DeviceDriver");
pub const REGSTR_VAL_DEVICEPATH: windows_core::PCWSTR = windows_core::w!("DevicePath");
pub const REGSTR_VAL_DEVICE_CHARACTERISTICS: windows_core::PCWSTR = windows_core::w!("DeviceCharacteristics");
pub const REGSTR_VAL_DEVICE_EXCLUSIVE: windows_core::PCWSTR = windows_core::w!("Exclusive");
pub const REGSTR_VAL_DEVICE_INSTANCE: windows_core::PCWSTR = windows_core::w!("DeviceInstance");
pub const REGSTR_VAL_DEVICE_SECURITY_DESCRIPTOR: windows_core::PCWSTR = windows_core::w!("Security");
pub const REGSTR_VAL_DEVICE_TYPE: windows_core::PCWSTR = windows_core::w!("DeviceType");
pub const REGSTR_VAL_DEVLOADER: windows_core::PCWSTR = windows_core::w!("DevLoader");
pub const REGSTR_VAL_DEVTYPE: windows_core::PCWSTR = windows_core::w!("DeviceType");
pub const REGSTR_VAL_DIRECTHOST: windows_core::PCWSTR = windows_core::w!("DirectHost");
pub const REGSTR_VAL_DIRTYSHUTDOWN: windows_core::PCWSTR = windows_core::w!("DirtyShutdown");
pub const REGSTR_VAL_DIRTYSHUTDOWNTIME: windows_core::PCWSTR = windows_core::w!("DirtyShutdownTime");
pub const REGSTR_VAL_DISABLECOUNT: windows_core::PCWSTR = windows_core::w!("DisableCount");
pub const REGSTR_VAL_DISABLEPWDCACHING: windows_core::PCWSTR = windows_core::w!("DisablePwdCaching");
pub const REGSTR_VAL_DISABLEREGTOOLS: windows_core::PCWSTR = windows_core::w!("DisableRegistryTools");
pub const REGSTR_VAL_DISCONNECT: windows_core::PCWSTR = windows_core::w!("Disconnect");
pub const REGSTR_VAL_DISK: windows_core::PCWSTR = windows_core::w!("GenDisk");
pub const REGSTR_VAL_DISKCLASSNAME: windows_core::PCWSTR = windows_core::w!("DiskDrive");
pub const REGSTR_VAL_DISPCPL_NOAPPEARANCEPAGE: windows_core::PCWSTR = windows_core::w!("NoDispAppearancePage");
pub const REGSTR_VAL_DISPCPL_NOBACKGROUNDPAGE: windows_core::PCWSTR = windows_core::w!("NoDispBackgroundPage");
pub const REGSTR_VAL_DISPCPL_NODISPCPL: windows_core::PCWSTR = windows_core::w!("NoDispCPL");
pub const REGSTR_VAL_DISPCPL_NOSCRSAVPAGE: windows_core::PCWSTR = windows_core::w!("NoDispScrSavPage");
pub const REGSTR_VAL_DISPCPL_NOSETTINGSPAGE: windows_core::PCWSTR = windows_core::w!("NoDispSettingsPage");
pub const REGSTR_VAL_DISPLAY: windows_core::PCWSTR = windows_core::w!("display");
pub const REGSTR_VAL_DISPLAYFLAGS: windows_core::PCWSTR = windows_core::w!("DisplayFlags");
pub const REGSTR_VAL_DOCKED: windows_core::PCWSTR = windows_core::w!("CurrentDockedState");
pub const REGSTR_VAL_DOCKSTATE: windows_core::PCWSTR = windows_core::w!("DockState");
pub const REGSTR_VAL_DOES_POLLING: windows_core::PCWSTR = windows_core::w!("PollingSupportNeeded");
pub const REGSTR_VAL_DONTLOADIFCONFLICT: windows_core::PCWSTR = windows_core::w!("DontLoadIfConflict");
pub const REGSTR_VAL_DONTUSEMEM: windows_core::PCWSTR = windows_core::w!("DontAllocLastMem");
pub const REGSTR_VAL_DOSCP: windows_core::PCWSTR = windows_core::w!("OEMCP");
pub const REGSTR_VAL_DOSOPTFLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const REGSTR_VAL_DOSOPTGLOBALFLAGS: windows_core::PCWSTR = windows_core::w!("GlobalFlags");
pub const REGSTR_VAL_DOSOPTTIP: windows_core::PCWSTR = windows_core::w!("TipText");
pub const REGSTR_VAL_DOSPAGER: windows_core::PCWSTR = windows_core::w!("DOSPager");
pub const REGSTR_VAL_DOS_SPOOL_MASK: windows_core::PCWSTR = windows_core::w!("DOSSpoolMask");
pub const REGSTR_VAL_DOUBLEBUFFER: windows_core::PCWSTR = windows_core::w!("DoubleBuffer");
pub const REGSTR_VAL_DPI: windows_core::PCWSTR = windows_core::w!("dpi");
pub const REGSTR_VAL_DPILOGICALX: windows_core::PCWSTR = windows_core::w!("DPILogicalX");
pub const REGSTR_VAL_DPILOGICALY: windows_core::PCWSTR = windows_core::w!("DPILogicalY");
pub const REGSTR_VAL_DPIPHYSICALX: windows_core::PCWSTR = windows_core::w!("DPIPhysicalX");
pub const REGSTR_VAL_DPIPHYSICALY: windows_core::PCWSTR = windows_core::w!("DPIPhysicalY");
pub const REGSTR_VAL_DPMS: windows_core::PCWSTR = windows_core::w!("DPMS");
pub const REGSTR_VAL_DRIVER: windows_core::PCWSTR = windows_core::w!("Driver");
pub const REGSTR_VAL_DRIVERCACHEPATH: windows_core::PCWSTR = windows_core::w!("DriverCachePath");
pub const REGSTR_VAL_DRIVERDATE: windows_core::PCWSTR = windows_core::w!("DriverDate");
pub const REGSTR_VAL_DRIVERDATEDATA: windows_core::PCWSTR = windows_core::w!("DriverDateData");
pub const REGSTR_VAL_DRIVERVERSION: windows_core::PCWSTR = windows_core::w!("DriverVersion");
pub const REGSTR_VAL_DRIVESPINDOWN: windows_core::PCWSTR = windows_core::w!("DriveSpinDown");
pub const REGSTR_VAL_DRIVEWRITEBEHIND: windows_core::PCWSTR = windows_core::w!("DriveWriteBehind");
pub const REGSTR_VAL_DRIVE_SPINDOWN: windows_core::PCWSTR = windows_core::w!("NoDispSpinDown");
pub const REGSTR_VAL_DRV: windows_core::PCWSTR = windows_core::w!("drv");
pub const REGSTR_VAL_DRVDESC: windows_core::PCWSTR = windows_core::w!("DriverDesc");
pub const REGSTR_VAL_DYNAMIC: windows_core::PCWSTR = windows_core::w!("Dynamic");
pub const REGSTR_VAL_EISA_FLAGS: windows_core::PCWSTR = windows_core::w!("EISAFlags");
pub const REGSTR_VAL_EISA_FUNCTIONS: windows_core::PCWSTR = windows_core::w!("EISAFunctions");
pub const REGSTR_VAL_EISA_FUNCTIONS_MASK: windows_core::PCWSTR = windows_core::w!("EISAFunctionsMask");
pub const REGSTR_VAL_EISA_RANGES: windows_core::PCWSTR = windows_core::w!("EISARanges");
pub const REGSTR_VAL_EISA_SIMULATE_INT15: windows_core::PCWSTR = windows_core::w!("EISASimulateInt15");
pub const REGSTR_VAL_EJECT_PRIORITY: windows_core::PCWSTR = windows_core::w!("EjectPriority");
pub const REGSTR_VAL_ENABLEINTS: windows_core::PCWSTR = windows_core::w!("EnableInts");
pub const REGSTR_VAL_ENUMERATOR: windows_core::PCWSTR = windows_core::w!("Enumerator");
pub const REGSTR_VAL_ENUMPROPPAGES: windows_core::PCWSTR = windows_core::w!("EnumPropPages");
pub const REGSTR_VAL_ENUMPROPPAGES_32: windows_core::PCWSTR = windows_core::w!("EnumPropPages32");
pub const REGSTR_VAL_ESDI: windows_core::PCWSTR = windows_core::w!("ESDI\\");
pub const REGSTR_VAL_EXISTS: windows_core::PCWSTR = windows_core::w!("Exists");
pub const REGSTR_VAL_EXTMEM: windows_core::PCWSTR = windows_core::w!("ExtMem");
pub const REGSTR_VAL_FAULT_LOGFILE: windows_core::PCWSTR = windows_core::w!("LogFile");
pub const REGSTR_VAL_FIFODEPTH: windows_core::PCWSTR = windows_core::w!("FIFODepth");
pub const REGSTR_VAL_FILESHARING: windows_core::PCWSTR = windows_core::w!("FileSharing");
pub const REGSTR_VAL_FIRSTINSTALLDATETIME: windows_core::PCWSTR = windows_core::w!("FirstInstallDateTime");
pub const REGSTR_VAL_FIRSTNETDRIVE: windows_core::PCWSTR = windows_core::w!("FirstNetworkDrive");
pub const REGSTR_VAL_FLOP: windows_core::PCWSTR = windows_core::w!("FLOP\\");
pub const REGSTR_VAL_FLOPPY: windows_core::PCWSTR = windows_core::w!("FLOPPY");
pub const REGSTR_VAL_FONTSIZE: windows_core::PCWSTR = windows_core::w!("FontSize");
pub const REGSTR_VAL_FORCECL: windows_core::PCWSTR = windows_core::w!("ForceChangeLine");
pub const REGSTR_VAL_FORCEDCONFIG: windows_core::PCWSTR = windows_core::w!("ForcedConfig");
pub const REGSTR_VAL_FORCEFIFO: windows_core::PCWSTR = windows_core::w!("ForceFIFO");
pub const REGSTR_VAL_FORCELOAD: windows_core::PCWSTR = windows_core::w!("ForceLoadPD");
pub const REGSTR_VAL_FORCEPMIO: windows_core::PCWSTR = windows_core::w!("ForcePMIO");
pub const REGSTR_VAL_FORCEREBOOT: windows_core::PCWSTR = windows_core::w!("ForceReboot");
pub const REGSTR_VAL_FORCERMIO: windows_core::PCWSTR = windows_core::w!("ForceRMIO");
pub const REGSTR_VAL_FREESPACERATIO: windows_core::PCWSTR = windows_core::w!("FreeSpaceRatio");
pub const REGSTR_VAL_FRIENDLYNAME: windows_core::PCWSTR = windows_core::w!("FriendlyName");
pub const REGSTR_VAL_FSFILTERCLASS: windows_core::PCWSTR = windows_core::w!("FSFilterClass");
pub const REGSTR_VAL_FULLTRACE: windows_core::PCWSTR = windows_core::w!("FullTrace");
pub const REGSTR_VAL_FUNCDESC: windows_core::PCWSTR = windows_core::w!("FunctionDesc");
pub const REGSTR_VAL_GAPTIME: windows_core::PCWSTR = windows_core::w!("GapTime");
pub const REGSTR_VAL_GRB: windows_core::PCWSTR = windows_core::w!("grb");
pub const REGSTR_VAL_HARDWAREID: windows_core::PCWSTR = windows_core::w!("HardwareID");
pub const REGSTR_VAL_HIDESHAREPWDS: windows_core::PCWSTR = windows_core::w!("HideSharePwds");
pub const REGSTR_VAL_HRES: windows_core::PCWSTR = windows_core::w!("HRes");
pub const REGSTR_VAL_HWDETECT: windows_core::PCWSTR = windows_core::w!("HardwareDetect");
pub const REGSTR_VAL_HWMECHANISM: windows_core::PCWSTR = windows_core::w!("HWMechanism");
pub const REGSTR_VAL_HWREV: windows_core::PCWSTR = windows_core::w!("HWRevision");
pub const REGSTR_VAL_ID: windows_core::PCWSTR = windows_core::w!("CurrentID");
pub const REGSTR_VAL_IDE_FORCE_SERIALIZE: windows_core::PCWSTR = windows_core::w!("ForceSerialization");
pub const REGSTR_VAL_IDE_NO_SERIALIZE: windows_core::PCWSTR = windows_core::w!("IDENoSerialize");
pub const REGSTR_VAL_INFNAME: windows_core::PCWSTR = windows_core::w!("InfName");
pub const REGSTR_VAL_INFPATH: windows_core::PCWSTR = windows_core::w!("InfPath");
pub const REGSTR_VAL_INFSECTION: windows_core::PCWSTR = windows_core::w!("InfSection");
pub const REGSTR_VAL_INFSECTIONEXT: windows_core::PCWSTR = windows_core::w!("InfSectionExt");
pub const REGSTR_VAL_INHIBITRESULTS: windows_core::PCWSTR = windows_core::w!("InhibitResults");
pub const REGSTR_VAL_INSICON: windows_core::PCWSTR = windows_core::w!("Icon");
pub const REGSTR_VAL_INSTALLER: windows_core::PCWSTR = windows_core::w!("Installer");
pub const REGSTR_VAL_INSTALLER_32: windows_core::PCWSTR = windows_core::w!("Installer32");
pub const REGSTR_VAL_INSTALLTYPE: windows_core::PCWSTR = windows_core::w!("InstallType");
pub const REGSTR_VAL_INT13: windows_core::PCWSTR = windows_core::w!("Int13");
pub const REGSTR_VAL_ISAPNP: windows_core::PCWSTR = windows_core::w!("ISAPNP");
pub const REGSTR_VAL_ISAPNP_RDP_OVERRIDE: windows_core::PCWSTR = windows_core::w!("RDPOverRide");
pub const REGSTR_VAL_JOYCALLOUT: windows_core::PCWSTR = windows_core::w!("JoystickCallout");
pub const REGSTR_VAL_JOYNCONFIG: windows_core::PCWSTR = windows_core::w!("Joystick%dConfiguration");
pub const REGSTR_VAL_JOYNOEMCALLOUT: windows_core::PCWSTR = windows_core::w!("Joystick%dOEMCallout");
pub const REGSTR_VAL_JOYNOEMNAME: windows_core::PCWSTR = windows_core::w!("Joystick%dOEMName");
pub const REGSTR_VAL_JOYOEMCAL1: windows_core::PCWSTR = windows_core::w!("OEMCal1");
pub const REGSTR_VAL_JOYOEMCAL10: windows_core::PCWSTR = windows_core::w!("OEMCal10");
pub const REGSTR_VAL_JOYOEMCAL11: windows_core::PCWSTR = windows_core::w!("OEMCal11");
pub const REGSTR_VAL_JOYOEMCAL12: windows_core::PCWSTR = windows_core::w!("OEMCal12");
pub const REGSTR_VAL_JOYOEMCAL2: windows_core::PCWSTR = windows_core::w!("OEMCal2");
pub const REGSTR_VAL_JOYOEMCAL3: windows_core::PCWSTR = windows_core::w!("OEMCal3");
pub const REGSTR_VAL_JOYOEMCAL4: windows_core::PCWSTR = windows_core::w!("OEMCal4");
pub const REGSTR_VAL_JOYOEMCAL5: windows_core::PCWSTR = windows_core::w!("OEMCal5");
pub const REGSTR_VAL_JOYOEMCAL6: windows_core::PCWSTR = windows_core::w!("OEMCal6");
pub const REGSTR_VAL_JOYOEMCAL7: windows_core::PCWSTR = windows_core::w!("OEMCal7");
pub const REGSTR_VAL_JOYOEMCAL8: windows_core::PCWSTR = windows_core::w!("OEMCal8");
pub const REGSTR_VAL_JOYOEMCAL9: windows_core::PCWSTR = windows_core::w!("OEMCal9");
pub const REGSTR_VAL_JOYOEMCALCAP: windows_core::PCWSTR = windows_core::w!("OEMCalCap");
pub const REGSTR_VAL_JOYOEMCALLOUT: windows_core::PCWSTR = windows_core::w!("OEMCallout");
pub const REGSTR_VAL_JOYOEMCALWINCAP: windows_core::PCWSTR = windows_core::w!("OEMCalWinCap");
pub const REGSTR_VAL_JOYOEMDATA: windows_core::PCWSTR = windows_core::w!("OEMData");
pub const REGSTR_VAL_JOYOEMNAME: windows_core::PCWSTR = windows_core::w!("OEMName");
pub const REGSTR_VAL_JOYOEMPOVLABEL: windows_core::PCWSTR = windows_core::w!("OEMPOVLabel");
pub const REGSTR_VAL_JOYOEMRLABEL: windows_core::PCWSTR = windows_core::w!("OEMRLabel");
pub const REGSTR_VAL_JOYOEMTESTBUTTONCAP: windows_core::PCWSTR = windows_core::w!("OEMTestButtonCap");
pub const REGSTR_VAL_JOYOEMTESTBUTTONDESC: windows_core::PCWSTR = windows_core::w!("OEMTestButtonDesc");
pub const REGSTR_VAL_JOYOEMTESTMOVECAP: windows_core::PCWSTR = windows_core::w!("OEMTestMoveCap");
pub const REGSTR_VAL_JOYOEMTESTMOVEDESC: windows_core::PCWSTR = windows_core::w!("OEMTestMoveDesc");
pub const REGSTR_VAL_JOYOEMTESTWINCAP: windows_core::PCWSTR = windows_core::w!("OEMTestWinCap");
pub const REGSTR_VAL_JOYOEMULABEL: windows_core::PCWSTR = windows_core::w!("OEMULabel");
pub const REGSTR_VAL_JOYOEMVLABEL: windows_core::PCWSTR = windows_core::w!("OEMVLabel");
pub const REGSTR_VAL_JOYOEMXYLABEL: windows_core::PCWSTR = windows_core::w!("OEMXYLabel");
pub const REGSTR_VAL_JOYOEMZLABEL: windows_core::PCWSTR = windows_core::w!("OEMZLabel");
pub const REGSTR_VAL_JOYUSERVALUES: windows_core::PCWSTR = windows_core::w!("JoystickUserValues");
pub const REGSTR_VAL_LASTALIVEBT: windows_core::PCWSTR = windows_core::w!("LastAliveBT");
pub const REGSTR_VAL_LASTALIVEINTERVAL: windows_core::PCWSTR = windows_core::w!("TimeStampInterval");
pub const REGSTR_VAL_LASTALIVEPMPOLICY: windows_core::PCWSTR = windows_core::w!("LastAlivePMPolicy");
pub const REGSTR_VAL_LASTALIVESTAMP: windows_core::PCWSTR = windows_core::w!("LastAliveStamp");
pub const REGSTR_VAL_LASTALIVESTAMPFORCED: windows_core::PCWSTR = windows_core::w!("LastAliveStampForced");
pub const REGSTR_VAL_LASTALIVESTAMPINTERVAL: windows_core::PCWSTR = windows_core::w!("LastAliveStampInterval");
pub const REGSTR_VAL_LASTALIVESTAMPPOLICYINTERVAL: windows_core::PCWSTR = windows_core::w!("LastAliveStampPolicyInterval");
pub const REGSTR_VAL_LASTALIVEUPTIME: windows_core::PCWSTR = windows_core::w!("LastAliveUptime");
pub const REGSTR_VAL_LASTBOOTPMDRVS: windows_core::PCWSTR = windows_core::w!("LastBootPMDrvs");
pub const REGSTR_VAL_LASTCOMPUTERNAME: windows_core::PCWSTR = windows_core::w!("LastComputerName");
pub const REGSTR_VAL_LASTPCIBUSNUM: windows_core::PCWSTR = windows_core::w!("LastPCIBusNum");
pub const REGSTR_VAL_LAST_UPDATE_TIME: windows_core::PCWSTR = windows_core::w!("LastUpdateTime");
pub const REGSTR_VAL_LEGALNOTICECAPTION: windows_core::PCWSTR = windows_core::w!("LegalNoticeCaption");
pub const REGSTR_VAL_LEGALNOTICETEXT: windows_core::PCWSTR = windows_core::w!("LegalNoticeText");
pub const REGSTR_VAL_LICENSINGINFO: windows_core::PCWSTR = windows_core::w!("LicensingInfo");
pub const REGSTR_VAL_LINKED: windows_core::PCWSTR = windows_core::w!("Linked");
pub const REGSTR_VAL_LOADHI: windows_core::PCWSTR = windows_core::w!("LoadHi");
pub const REGSTR_VAL_LOADRMDRIVERS: windows_core::PCWSTR = windows_core::w!("LoadRMDrivers");
pub const REGSTR_VAL_LOCATION_INFORMATION: windows_core::PCWSTR = windows_core::w!("LocationInformation");
pub const REGSTR_VAL_LOCATION_INFORMATION_OVERRIDE: windows_core::PCWSTR = windows_core::w!("LocationInformationOverride");
pub const REGSTR_VAL_LOWERFILTERS: windows_core::PCWSTR = windows_core::w!("LowerFilters");
pub const REGSTR_VAL_LOWER_FILTER_DEFAULT_LEVEL: windows_core::PCWSTR = windows_core::w!("LowerFilterDefaultLevel");
pub const REGSTR_VAL_LOWER_FILTER_LEVELS: windows_core::PCWSTR = windows_core::w!("LowerFilterLevels");
pub const REGSTR_VAL_MACHINETYPE: windows_core::PCWSTR = windows_core::w!("MachineType");
pub const REGSTR_VAL_MANUFACTURER: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const REGSTR_VAL_MAP: windows_core::PCWSTR = windows_core::w!("Map");
pub const REGSTR_VAL_MATCHINGDEVID: windows_core::PCWSTR = windows_core::w!("MatchingDeviceId");
pub const REGSTR_VAL_MAXCONNECTIONS: windows_core::PCWSTR = windows_core::w!("MaxConnections");
pub const REGSTR_VAL_MAXLIP: windows_core::PCWSTR = windows_core::w!("MaxLIP");
pub const REGSTR_VAL_MAXRES: windows_core::PCWSTR = windows_core::w!("MaxResolution");
pub const REGSTR_VAL_MAXRETRY: windows_core::PCWSTR = windows_core::w!("MaxRetry");
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
pub const REGSTR_VAL_MEDIA: windows_core::PCWSTR = windows_core::w!("MediaPath");
pub const REGSTR_VAL_MFG: windows_core::PCWSTR = windows_core::w!("Mfg");
pub const REGSTR_VAL_MF_FLAGS: windows_core::PCWSTR = windows_core::w!("MFFlags");
pub const REGSTR_VAL_MINIPORT_STAT: windows_core::PCWSTR = windows_core::w!("MiniportStatus");
pub const REGSTR_VAL_MINPWDLEN: windows_core::PCWSTR = windows_core::w!("MinPwdLen");
pub const REGSTR_VAL_MINRETRY: windows_core::PCWSTR = windows_core::w!("MinRetry");
pub const REGSTR_VAL_MODE: windows_core::PCWSTR = windows_core::w!("Mode");
pub const REGSTR_VAL_MODEL: windows_core::PCWSTR = windows_core::w!("Model");
pub const REGSTR_VAL_MSDOSMODE: windows_core::PCWSTR = windows_core::w!("MSDOSMode");
pub const REGSTR_VAL_MSDOSMODEDISCARD: windows_core::PCWSTR = windows_core::w!("Discard");
pub const REGSTR_VAL_MUSTBEVALIDATED: windows_core::PCWSTR = windows_core::w!("MustBeValidated");
pub const REGSTR_VAL_NAMECACHECOUNT: windows_core::PCWSTR = windows_core::w!("NameCache");
pub const REGSTR_VAL_NAMENUMERICTAIL: windows_core::PCWSTR = windows_core::w!("NameNumericTail");
pub const REGSTR_VAL_NCP_BROWSEMASTER: windows_core::PCWSTR = windows_core::w!("BrowseMaster");
pub const REGSTR_VAL_NCP_USEPEERBROWSING: windows_core::PCWSTR = windows_core::w!("Use_PeerBrowsing");
pub const REGSTR_VAL_NCP_USESAP: windows_core::PCWSTR = windows_core::w!("Use_Sap");
pub const REGSTR_VAL_NDP: windows_core::PCWSTR = windows_core::w!("NDP");
pub const REGSTR_VAL_NETCARD: windows_core::PCWSTR = windows_core::w!("Netcard");
pub const REGSTR_VAL_NETCLEAN: windows_core::PCWSTR = windows_core::w!("NetClean");
pub const REGSTR_VAL_NETOSTYPE: windows_core::PCWSTR = windows_core::w!("NetOSType");
pub const REGSTR_VAL_NETSETUP_DISABLE: windows_core::PCWSTR = windows_core::w!("NoNetSetup");
pub const REGSTR_VAL_NETSETUP_NOCONFIGPAGE: windows_core::PCWSTR = windows_core::w!("NoNetSetupConfigPage");
pub const REGSTR_VAL_NETSETUP_NOIDPAGE: windows_core::PCWSTR = windows_core::w!("NoNetSetupIDPage");
pub const REGSTR_VAL_NETSETUP_NOSECURITYPAGE: windows_core::PCWSTR = windows_core::w!("NoNetSetupSecurityPage");
pub const REGSTR_VAL_NOCMOSORFDPT: windows_core::PCWSTR = windows_core::w!("NoCMOSorFDPT");
pub const REGSTR_VAL_NODISPLAYCLASS: windows_core::PCWSTR = windows_core::w!("NoDisplayClass");
pub const REGSTR_VAL_NOENTIRENETWORK: windows_core::PCWSTR = windows_core::w!("NoEntireNetwork");
pub const REGSTR_VAL_NOFILESHARING: windows_core::PCWSTR = windows_core::w!("NoFileSharing");
pub const REGSTR_VAL_NOFILESHARINGCTRL: windows_core::PCWSTR = windows_core::w!("NoFileSharingControl");
pub const REGSTR_VAL_NOIDE: windows_core::PCWSTR = windows_core::w!("NoIDE");
pub const REGSTR_VAL_NOINSTALLCLASS: windows_core::PCWSTR = windows_core::w!("NoInstallClass");
pub const REGSTR_VAL_NONSTANDARD_ATAPI: windows_core::PCWSTR = windows_core::w!("NonStandardATAPI");
pub const REGSTR_VAL_NOPRINTSHARING: windows_core::PCWSTR = windows_core::w!("NoPrintSharing");
pub const REGSTR_VAL_NOPRINTSHARINGCTRL: windows_core::PCWSTR = windows_core::w!("NoPrintSharingControl");
pub const REGSTR_VAL_NOUSECLASS: windows_core::PCWSTR = windows_core::w!("NoUseClass");
pub const REGSTR_VAL_NOWORKGROUPCONTENTS: windows_core::PCWSTR = windows_core::w!("NoWorkgroupContents");
pub const REGSTR_VAL_OLDMSDOSVER: windows_core::PCWSTR = windows_core::w!("OldMSDOSVer");
pub const REGSTR_VAL_OLDWINDIR: windows_core::PCWSTR = windows_core::w!("OldWinDir");
pub const REGSTR_VAL_OPTIMIZESFN: windows_core::PCWSTR = windows_core::w!("OptimizeSFN");
pub const REGSTR_VAL_OPTIONS: windows_core::PCWSTR = windows_core::w!("Options");
pub const REGSTR_VAL_OPTORDER: windows_core::PCWSTR = windows_core::w!("Order");
pub const REGSTR_VAL_P1284MDL: windows_core::PCWSTR = windows_core::w!("Model");
pub const REGSTR_VAL_P1284MFG: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const REGSTR_VAL_PATHCACHECOUNT: windows_core::PCWSTR = windows_core::w!("PathCache");
pub const REGSTR_VAL_PCCARD_POWER: windows_core::PCWSTR = windows_core::w!("EnablePowerManagement");
pub const REGSTR_VAL_PCI: windows_core::PCWSTR = windows_core::w!("PCI");
pub const REGSTR_VAL_PCIBIOSVER: windows_core::PCWSTR = windows_core::w!("PCIBIOSVer");
pub const REGSTR_VAL_PCICIRQMAP: windows_core::PCWSTR = windows_core::w!("PCICIRQMap");
pub const REGSTR_VAL_PCICOPTIONS: windows_core::PCWSTR = windows_core::w!("PCICOptions");
pub const REGSTR_VAL_PCMCIA_ALLOC: windows_core::PCWSTR = windows_core::w!("AllocMemWin");
pub const REGSTR_VAL_PCMCIA_ATAD: windows_core::PCWSTR = windows_core::w!("ATADelay");
pub const REGSTR_VAL_PCMCIA_MEM: windows_core::PCWSTR = windows_core::w!("Memory");
pub const REGSTR_VAL_PCMCIA_OPT: windows_core::PCWSTR = windows_core::w!("Options");
pub const REGSTR_VAL_PCMCIA_SIZ: windows_core::PCWSTR = windows_core::w!("MinRegionSize");
pub const REGSTR_VAL_PCMTDRIVER: windows_core::PCWSTR = windows_core::w!("MTD");
pub const REGSTR_VAL_PCSSDRIVER: windows_core::PCWSTR = windows_core::w!("Driver");
pub const REGSTR_VAL_PHYSICALDEVICEOBJECT: windows_core::PCWSTR = windows_core::w!("PhysicalDeviceObject");
pub const REGSTR_VAL_PMODE_INT13: windows_core::PCWSTR = windows_core::w!("PModeInt13");
pub const REGSTR_VAL_PNPBIOSVER: windows_core::PCWSTR = windows_core::w!("PnPBIOSVer");
pub const REGSTR_VAL_PNPSTRUCOFFSET: windows_core::PCWSTR = windows_core::w!("PnPStrucOffset");
pub const REGSTR_VAL_POLICY: windows_core::PCWSTR = windows_core::w!("Policy");
pub const REGSTR_VAL_POLLING: windows_core::PCWSTR = windows_core::w!("Polling");
pub const REGSTR_VAL_PORTNAME: windows_core::PCWSTR = windows_core::w!("PortName");
pub const REGSTR_VAL_PORTSUBCLASS: windows_core::PCWSTR = windows_core::w!("PortSubClass");
pub const REGSTR_VAL_PREFREDIR: windows_core::PCWSTR = windows_core::w!("PreferredRedir");
pub const REGSTR_VAL_PRESERVECASE: windows_core::PCWSTR = windows_core::w!("PreserveCase");
pub const REGSTR_VAL_PRESERVELONGNAMES: windows_core::PCWSTR = windows_core::w!("PreserveLongNames");
pub const REGSTR_VAL_PRINTERS_HIDETABS: windows_core::PCWSTR = windows_core::w!("NoPrinterTabs");
pub const REGSTR_VAL_PRINTERS_MASK: windows_core::PCWSTR = windows_core::w!("PrintersMask");
pub const REGSTR_VAL_PRINTERS_NOADD: windows_core::PCWSTR = windows_core::w!("NoAddPrinter");
pub const REGSTR_VAL_PRINTERS_NODELETE: windows_core::PCWSTR = windows_core::w!("NoDeletePrinter");
pub const REGSTR_VAL_PRINTSHARING: windows_core::PCWSTR = windows_core::w!("PrintSharing");
pub const REGSTR_VAL_PRIORITY: windows_core::PCWSTR = windows_core::w!("Priority");
pub const REGSTR_VAL_PRIVATE: windows_core::PCWSTR = windows_core::w!("Private");
pub const REGSTR_VAL_PRIVATEFUNC: windows_core::PCWSTR = windows_core::w!("PrivateFunc");
pub const REGSTR_VAL_PRIVATEPROBLEM: windows_core::PCWSTR = windows_core::w!("PrivateProblem");
pub const REGSTR_VAL_PRODUCTID: windows_core::PCWSTR = windows_core::w!("ProductId");
pub const REGSTR_VAL_PRODUCTTYPE: windows_core::PCWSTR = windows_core::w!("ProductType");
pub const REGSTR_VAL_PROFILEFLAGS: windows_core::PCWSTR = windows_core::w!("ProfileFlags");
pub const REGSTR_VAL_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const REGSTR_VAL_PROTINIPATH: windows_core::PCWSTR = windows_core::w!("ProtIniPath");
pub const REGSTR_VAL_PROVIDER_NAME: windows_core::PCWSTR = windows_core::w!("ProviderName");
pub const REGSTR_VAL_PWDEXPIRATION: windows_core::PCWSTR = windows_core::w!("PwdExpiration");
pub const REGSTR_VAL_PWDPROVIDER_CHANGEORDER: windows_core::PCWSTR = windows_core::w!("ChangeOrder");
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWD: windows_core::PCWSTR = windows_core::w!("ChangePassword");
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWDHWND: windows_core::PCWSTR = windows_core::w!("ChangePasswordHwnd");
pub const REGSTR_VAL_PWDPROVIDER_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const REGSTR_VAL_PWDPROVIDER_GETPWDSTATUS: windows_core::PCWSTR = windows_core::w!("GetPasswordStatus");
pub const REGSTR_VAL_PWDPROVIDER_ISNP: windows_core::PCWSTR = windows_core::w!("NetworkProvider");
pub const REGSTR_VAL_PWDPROVIDER_PATH: windows_core::PCWSTR = windows_core::w!("ProviderPath");
pub const REGSTR_VAL_RDINTTHRESHOLD: windows_core::PCWSTR = windows_core::w!("RDIntThreshold");
pub const REGSTR_VAL_READAHEADTHRESHOLD: windows_core::PCWSTR = windows_core::w!("ReadAheadThreshold");
pub const REGSTR_VAL_READCACHING: windows_core::PCWSTR = windows_core::w!("ReadCaching");
pub const REGSTR_VAL_REALNETSTART: windows_core::PCWSTR = windows_core::w!("RealNetStart");
pub const REGSTR_VAL_REASONCODE: windows_core::PCWSTR = windows_core::w!("ReasonCode");
pub const REGSTR_VAL_REFRESHRATE: windows_core::PCWSTR = windows_core::w!("RefreshRate");
pub const REGSTR_VAL_REGITEMDELETEMESSAGE: windows_core::PCWSTR = windows_core::w!("Removal Message");
pub const REGSTR_VAL_REGORGANIZATION: windows_core::PCWSTR = windows_core::w!("RegisteredOrganization");
pub const REGSTR_VAL_REGOWNER: windows_core::PCWSTR = windows_core::w!("RegisteredOwner");
pub const REGSTR_VAL_REINSTALL_DEVICEINSTANCEIDS: windows_core::PCWSTR = windows_core::w!("DeviceInstanceIds");
pub const REGSTR_VAL_REINSTALL_DISPLAYNAME: windows_core::PCWSTR = windows_core::w!("DisplayName");
pub const REGSTR_VAL_REINSTALL_STRING: windows_core::PCWSTR = windows_core::w!("ReinstallString");
pub const REGSTR_VAL_REMOTE_PATH: windows_core::PCWSTR = windows_core::w!("RemotePath");
pub const REGSTR_VAL_REMOVABLE: windows_core::PCWSTR = windows_core::w!("Removable");
pub const REGSTR_VAL_REMOVAL_POLICY: windows_core::PCWSTR = windows_core::w!("RemovalPolicy");
pub const REGSTR_VAL_REMOVEROMOKAY: windows_core::PCWSTR = windows_core::w!("RemoveRomOkay");
pub const REGSTR_VAL_REMOVEROMOKAYFUNC: windows_core::PCWSTR = windows_core::w!("RemoveRomOkayFunc");
pub const REGSTR_VAL_RESERVED_DEVNODE: windows_core::PCWSTR = windows_core::w!("HTREE\\RESERVED\\0");
pub const REGSTR_VAL_RESOLUTION: windows_core::PCWSTR = windows_core::w!("Resolution");
pub const REGSTR_VAL_RESOURCES: windows_core::PCWSTR = windows_core::w!("Resources");
pub const REGSTR_VAL_RESOURCE_MAP: windows_core::PCWSTR = windows_core::w!("ResourceMap");
pub const REGSTR_VAL_RESOURCE_PICKER_EXCEPTIONS: windows_core::PCWSTR = windows_core::w!("ResourcePickerExceptions");
pub const REGSTR_VAL_RESOURCE_PICKER_TAGS: windows_core::PCWSTR = windows_core::w!("ResourcePickerTags");
pub const REGSTR_VAL_RESTRICTRUN: windows_core::PCWSTR = windows_core::w!("RestrictRun");
pub const REGSTR_VAL_RESUMERESET: windows_core::PCWSTR = windows_core::w!("ResumeReset");
pub const REGSTR_VAL_REVISION: windows_core::PCWSTR = windows_core::w!("Revision");
pub const REGSTR_VAL_REVLEVEL: windows_core::PCWSTR = windows_core::w!("RevisionLevel");
pub const REGSTR_VAL_ROOT_DEVNODE: windows_core::PCWSTR = windows_core::w!("HTREE\\ROOT\\0");
pub const REGSTR_VAL_RUNLOGINSCRIPT: windows_core::PCWSTR = windows_core::w!("ProcessLoginScript");
pub const REGSTR_VAL_SCANNER: windows_core::PCWSTR = windows_core::w!("SCANNER");
pub const REGSTR_VAL_SCAN_ONLY_FIRST: windows_core::PCWSTR = windows_core::w!("ScanOnlyFirstDrive");
pub const REGSTR_VAL_SCSI: windows_core::PCWSTR = windows_core::w!("SCSI\\");
pub const REGSTR_VAL_SCSILUN: windows_core::PCWSTR = windows_core::w!("SCSILUN");
pub const REGSTR_VAL_SCSITID: windows_core::PCWSTR = windows_core::w!("SCSITargetID");
pub const REGSTR_VAL_SEARCHMODE: windows_core::PCWSTR = windows_core::w!("SearchMode");
pub const REGSTR_VAL_SEARCHOPTIONS: windows_core::PCWSTR = windows_core::w!("SearchOptions");
pub const REGSTR_VAL_SECCPL_NOADMINPAGE: windows_core::PCWSTR = windows_core::w!("NoAdminPage");
pub const REGSTR_VAL_SECCPL_NOPROFILEPAGE: windows_core::PCWSTR = windows_core::w!("NoProfilePage");
pub const REGSTR_VAL_SECCPL_NOPWDPAGE: windows_core::PCWSTR = windows_core::w!("NoPwdPage");
pub const REGSTR_VAL_SECCPL_NOSECCPL: windows_core::PCWSTR = windows_core::w!("NoSecCPL");
pub const REGSTR_VAL_SERVICE: windows_core::PCWSTR = windows_core::w!("Service");
pub const REGSTR_VAL_SETUPFLAGS: windows_core::PCWSTR = windows_core::w!("SetupFlags");
pub const REGSTR_VAL_SETUPMACHINETYPE: windows_core::PCWSTR = windows_core::w!("SetupMachineType");
pub const REGSTR_VAL_SETUPN: windows_core::PCWSTR = windows_core::w!("SetupN");
pub const REGSTR_VAL_SETUPNPATH: windows_core::PCWSTR = windows_core::w!("SetupNPath");
pub const REGSTR_VAL_SETUPPROGRAMRAN: windows_core::PCWSTR = windows_core::w!("SetupProgramRan");
pub const REGSTR_VAL_SHARES_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const REGSTR_VAL_SHARES_PATH: windows_core::PCWSTR = windows_core::w!("Path");
pub const REGSTR_VAL_SHARES_REMARK: windows_core::PCWSTR = windows_core::w!("Remark");
pub const REGSTR_VAL_SHARES_RO_PASS: windows_core::PCWSTR = windows_core::w!("Parm2");
pub const REGSTR_VAL_SHARES_RW_PASS: windows_core::PCWSTR = windows_core::w!("Parm1");
pub const REGSTR_VAL_SHARES_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const REGSTR_VAL_SHARE_IRQ: windows_core::PCWSTR = windows_core::w!("ForceIRQSharing");
pub const REGSTR_VAL_SHELLVERSION: windows_core::PCWSTR = windows_core::w!("ShellVersion");
pub const REGSTR_VAL_SHOWDOTS: windows_core::PCWSTR = windows_core::w!("ShowDots");
pub const REGSTR_VAL_SHOWREASONUI: windows_core::PCWSTR = windows_core::w!("ShutdownReasonUI");
pub const REGSTR_VAL_SHUTDOWNREASON: windows_core::PCWSTR = windows_core::w!("ShutdownReason");
pub const REGSTR_VAL_SHUTDOWNREASON_CODE: windows_core::PCWSTR = windows_core::w!("ShutdownReasonCode");
pub const REGSTR_VAL_SHUTDOWNREASON_COMMENT: windows_core::PCWSTR = windows_core::w!("ShutdownReasonComment");
pub const REGSTR_VAL_SHUTDOWNREASON_PROCESS: windows_core::PCWSTR = windows_core::w!("ShutdownReasonProcess");
pub const REGSTR_VAL_SHUTDOWNREASON_USERNAME: windows_core::PCWSTR = windows_core::w!("ShutdownReasonUserName");
pub const REGSTR_VAL_SHUTDOWN_FLAGS: windows_core::PCWSTR = windows_core::w!("ShutdownFlags");
pub const REGSTR_VAL_SHUTDOWN_IGNORE_PREDEFINED: windows_core::PCWSTR = windows_core::w!("ShutdownIgnorePredefinedReasons");
pub const REGSTR_VAL_SHUTDOWN_STATE_SNAPSHOT: windows_core::PCWSTR = windows_core::w!("ShutdownStateSnapshot");
pub const REGSTR_VAL_SILENTINSTALL: windows_core::PCWSTR = windows_core::w!("SilentInstall");
pub const REGSTR_VAL_SLSUPPORT: windows_core::PCWSTR = windows_core::w!("SLSupport");
pub const REGSTR_VAL_SOFTCOMPATMODE: windows_core::PCWSTR = windows_core::w!("SoftCompatMode");
pub const REGSTR_VAL_SRCPATH: windows_core::PCWSTR = windows_core::w!("SourcePath");
pub const REGSTR_VAL_SRVNAMECACHE: windows_core::PCWSTR = windows_core::w!("ServerNameCache");
pub const REGSTR_VAL_SRVNAMECACHECOUNT: windows_core::PCWSTR = windows_core::w!("ServerNameCacheMax");
pub const REGSTR_VAL_SRVNAMECACHENETPROV: windows_core::PCWSTR = windows_core::w!("ServerNameCacheNumNets");
pub const REGSTR_VAL_START_ON_BOOT: windows_core::PCWSTR = windows_core::w!("StartOnBoot");
pub const REGSTR_VAL_STAT: windows_core::PCWSTR = windows_core::w!("Status");
pub const REGSTR_VAL_STATICDRIVE: windows_core::PCWSTR = windows_core::w!("StaticDrive");
pub const REGSTR_VAL_STATICVXD: windows_core::PCWSTR = windows_core::w!("StaticVxD");
pub const REGSTR_VAL_STDDOSOPTION: windows_core::PCWSTR = windows_core::w!("StdOption");
pub const REGSTR_VAL_SUBMODEL: windows_core::PCWSTR = windows_core::w!("Submodel");
pub const REGSTR_VAL_SUPPORTBURST: windows_core::PCWSTR = windows_core::w!("SupportBurst");
pub const REGSTR_VAL_SUPPORTLFN: windows_core::PCWSTR = windows_core::w!("SupportLFN");
pub const REGSTR_VAL_SUPPORTTUNNELLING: windows_core::PCWSTR = windows_core::w!("SupportTunnelling");
pub const REGSTR_VAL_SYMBOLIC_LINK: windows_core::PCWSTR = windows_core::w!("SymbolicLink");
pub const REGSTR_VAL_SYNCDATAXFER: windows_core::PCWSTR = windows_core::w!("SyncDataXfer");
pub const REGSTR_VAL_SYSDM: windows_core::PCWSTR = windows_core::w!("SysDM");
pub const REGSTR_VAL_SYSDMFUNC: windows_core::PCWSTR = windows_core::w!("SysDMFunc");
pub const REGSTR_VAL_SYSTEMCPL_NOCONFIGPAGE: windows_core::PCWSTR = windows_core::w!("NoConfigPage");
pub const REGSTR_VAL_SYSTEMCPL_NODEVMGRPAGE: windows_core::PCWSTR = windows_core::w!("NoDevMgrPage");
pub const REGSTR_VAL_SYSTEMCPL_NOFILESYSPAGE: windows_core::PCWSTR = windows_core::w!("NoFileSysPage");
pub const REGSTR_VAL_SYSTEMCPL_NOVIRTMEMPAGE: windows_core::PCWSTR = windows_core::w!("NoVirtMemPage");
pub const REGSTR_VAL_SYSTEMROOT: windows_core::PCWSTR = windows_core::w!("SystemRoot");
pub const REGSTR_VAL_SYSTRAYBATFLAGS: windows_core::PCWSTR = windows_core::w!("PowerFlags");
pub const REGSTR_VAL_SYSTRAYPCCARDFLAGS: windows_core::PCWSTR = windows_core::w!("PCMCIAFlags");
pub const REGSTR_VAL_SYSTRAYSVCS: windows_core::PCWSTR = windows_core::w!("Services");
pub const REGSTR_VAL_TABLE_STAT: windows_core::PCWSTR = windows_core::w!("TableStatus");
pub const REGSTR_VAL_TAPE: windows_core::PCWSTR = windows_core::w!("TAPE");
pub const REGSTR_VAL_TRANSITION: windows_core::PCWSTR = windows_core::w!("Transition");
pub const REGSTR_VAL_TRANSPORT: windows_core::PCWSTR = windows_core::w!("Transport");
pub const REGSTR_VAL_TZACTBIAS: windows_core::PCWSTR = windows_core::w!("ActiveTimeBias");
pub const REGSTR_VAL_TZBIAS: windows_core::PCWSTR = windows_core::w!("Bias");
pub const REGSTR_VAL_TZDLTBIAS: windows_core::PCWSTR = windows_core::w!("DaylightBias");
pub const REGSTR_VAL_TZDLTFLAG: windows_core::PCWSTR = windows_core::w!("DaylightFlag");
pub const REGSTR_VAL_TZDLTNAME: windows_core::PCWSTR = windows_core::w!("DaylightName");
pub const REGSTR_VAL_TZDLTSTART: windows_core::PCWSTR = windows_core::w!("DaylightStart");
pub const REGSTR_VAL_TZNOAUTOTIME: windows_core::PCWSTR = windows_core::w!("DisableAutoDaylightTimeSet");
pub const REGSTR_VAL_TZNOCHANGEEND: windows_core::PCWSTR = windows_core::w!("NoChangeEnd");
pub const REGSTR_VAL_TZNOCHANGESTART: windows_core::PCWSTR = windows_core::w!("NoChangeStart");
pub const REGSTR_VAL_TZSTDBIAS: windows_core::PCWSTR = windows_core::w!("StandardBias");
pub const REGSTR_VAL_TZSTDNAME: windows_core::PCWSTR = windows_core::w!("StandardName");
pub const REGSTR_VAL_TZSTDSTART: windows_core::PCWSTR = windows_core::w!("StandardStart");
pub const REGSTR_VAL_UI_NUMBER: windows_core::PCWSTR = windows_core::w!("UINumber");
pub const REGSTR_VAL_UI_NUMBER_DESC_FORMAT: windows_core::PCWSTR = windows_core::w!("UINumberDescFormat");
pub const REGSTR_VAL_UNDOCK_WITHOUT_LOGON: windows_core::PCWSTR = windows_core::w!("UndockWithoutLogon");
pub const REGSTR_VAL_UNINSTALLER_COMMANDLINE: windows_core::PCWSTR = windows_core::w!("UninstallString");
pub const REGSTR_VAL_UNINSTALLER_DISPLAYNAME: windows_core::PCWSTR = windows_core::w!("DisplayName");
pub const REGSTR_VAL_UPGRADE: windows_core::PCWSTR = windows_core::w!("Upgrade");
pub const REGSTR_VAL_UPPERFILTERS: windows_core::PCWSTR = windows_core::w!("UpperFilters");
pub const REGSTR_VAL_UPPER_FILTER_DEFAULT_LEVEL: windows_core::PCWSTR = windows_core::w!("UpperFilterDefaultLevel");
pub const REGSTR_VAL_UPPER_FILTER_LEVELS: windows_core::PCWSTR = windows_core::w!("UpperFilterLevels");
pub const REGSTR_VAL_USERSETTINGS: windows_core::PCWSTR = windows_core::w!("AdapterSettings");
pub const REGSTR_VAL_USER_NAME: windows_core::PCWSTR = windows_core::w!("UserName");
pub const REGSTR_VAL_USRDRVLET: windows_core::PCWSTR = windows_core::w!("UserDriveLetterAssignment");
pub const REGSTR_VAL_VDD: windows_core::PCWSTR = windows_core::w!("vdd");
pub const REGSTR_VAL_VER: windows_core::PCWSTR = windows_core::w!("Ver");
pub const REGSTR_VAL_VERIFYKEY: windows_core::PCWSTR = windows_core::w!("VerifyKey");
pub const REGSTR_VAL_VIRTUALHDIRQ: windows_core::PCWSTR = windows_core::w!("VirtualHDIRQ");
pub const REGSTR_VAL_VOLIDLETIMEOUT: windows_core::PCWSTR = windows_core::w!("VolumeIdleTimeout");
pub const REGSTR_VAL_VPOWERDFLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const REGSTR_VAL_VRES: windows_core::PCWSTR = windows_core::w!("VRes");
pub const REGSTR_VAL_VXDGROUPS: windows_core::PCWSTR = windows_core::w!("VXDGroups");
pub const REGSTR_VAL_WAITFORUNDOCK: windows_core::PCWSTR = windows_core::w!("WaitForUndock");
pub const REGSTR_VAL_WAITFORUNDOCKFUNC: windows_core::PCWSTR = windows_core::w!("WaitForUndockFunc");
pub const REGSTR_VAL_WIN31FILESYSTEM: windows_core::PCWSTR = windows_core::w!("Win31FileSystem");
pub const REGSTR_VAL_WIN31PROVIDER: windows_core::PCWSTR = windows_core::w!("Win31Provider");
pub const REGSTR_VAL_WINBOOTDIR: windows_core::PCWSTR = windows_core::w!("WinbootDir");
pub const REGSTR_VAL_WINCP: windows_core::PCWSTR = windows_core::w!("ACP");
pub const REGSTR_VAL_WINDIR: windows_core::PCWSTR = windows_core::w!("WinDir");
pub const REGSTR_VAL_WINOLDAPP_DISABLED: windows_core::PCWSTR = windows_core::w!("Disabled");
pub const REGSTR_VAL_WINOLDAPP_NOREALMODE: windows_core::PCWSTR = windows_core::w!("NoRealMode");
pub const REGSTR_VAL_WORKGROUP: windows_core::PCWSTR = windows_core::w!("Workgroup");
pub const REGSTR_VAL_WRAPPER: windows_core::PCWSTR = windows_core::w!("Wrapper");
pub const REGSTR_VAL_WRINTTHRESHOLD: windows_core::PCWSTR = windows_core::w!("WRIntThreshold");
pub const REGSTR_VAL_WRKGRP_FORCEMAPPING: windows_core::PCWSTR = windows_core::w!("WrkgrpForceMapping");
pub const REGSTR_VAL_WRKGRP_REQUIRED: windows_core::PCWSTR = windows_core::w!("WrkgrpRequired");
pub const REG_BINARY: REG_VALUE_TYPE = REG_VALUE_TYPE(3u32);
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
pub const REG_DWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(5u32);
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(2u32);
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = REG_VALUE_TYPE(9u32);
pub const REG_KEY_INSTDEV: windows_core::PCWSTR = windows_core::w!("Installed");
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
pub const REG_LINK: REG_VALUE_TYPE = REG_VALUE_TYPE(6u32);
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
pub const REG_MULTI_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(7u32);
pub const REG_NONE: REG_VALUE_TYPE = REG_VALUE_TYPE(0u32);
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
pub const REG_PROCESS_APPKEY: u32 = 1u32;
pub const REG_QWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(8u32);
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(10u32);
pub const REG_SECURE_CONNECTION: u32 = 1u32;
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
pub const REG_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(1u32);
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
pub const RRF_NOEXPAND: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(268435456u32);
pub const RRF_RT_ANY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65535u32);
pub const RRF_RT_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(24u32);
pub const RRF_RT_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(72u32);
pub const RRF_RT_REG_BINARY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(8u32);
pub const RRF_RT_REG_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(16u32);
pub const RRF_RT_REG_EXPAND_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(4u32);
pub const RRF_RT_REG_MULTI_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(32u32);
pub const RRF_RT_REG_NONE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(1u32);
pub const RRF_RT_REG_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(64u32);
pub const RRF_RT_REG_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(2u32);
pub const RRF_SUBKEY_WOW6432KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(131072u32);
pub const RRF_SUBKEY_WOW6464KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65536u32);
pub const RRF_WOW64_MASK: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(196608u32);
pub const RRF_ZEROONFAILURE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(536870912u32);
pub const SUF_BATCHINF: i32 = 4i32;
pub const SUF_CLEAN: i32 = 8i32;
pub const SUF_EXPRESS: i32 = 2i32;
pub const SUF_FIRSTTIME: i32 = 1i32;
pub const SUF_INSETUP: i32 = 16i32;
pub const SUF_NETHDBOOT: i32 = 64i32;
pub const SUF_NETRPLBOOT: i32 = 128i32;
pub const SUF_NETSETUP: i32 = 32i32;
pub const SUF_SBSCOPYOK: i32 = 256i32;
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_CREATE_KEY_DISPOSITION(pub u32);
impl windows_core::TypeKind for REG_CREATE_KEY_DISPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_CREATE_KEY_DISPOSITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_CREATE_KEY_DISPOSITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_NOTIFY_FILTER(pub u32);
impl windows_core::TypeKind for REG_NOTIFY_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_NOTIFY_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_NOTIFY_FILTER").field(&self.0).finish()
    }
}
impl REG_NOTIFY_FILTER {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REG_NOTIFY_FILTER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REG_NOTIFY_FILTER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REG_NOTIFY_FILTER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_OPEN_CREATE_OPTIONS(pub u32);
impl windows_core::TypeKind for REG_OPEN_CREATE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_OPEN_CREATE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_OPEN_CREATE_OPTIONS").field(&self.0).finish()
    }
}
impl REG_OPEN_CREATE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_RESTORE_KEY_FLAGS(pub i32);
impl windows_core::TypeKind for REG_RESTORE_KEY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_RESTORE_KEY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_RESTORE_KEY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_ROUTINE_FLAGS(pub u32);
impl windows_core::TypeKind for REG_ROUTINE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_ROUTINE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_ROUTINE_FLAGS").field(&self.0).finish()
    }
}
impl REG_ROUTINE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REG_ROUTINE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REG_ROUTINE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REG_ROUTINE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_SAM_FLAGS(pub u32);
impl windows_core::TypeKind for REG_SAM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_SAM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_SAM_FLAGS").field(&self.0).finish()
    }
}
impl REG_SAM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REG_SAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REG_SAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REG_SAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REG_SAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REG_SAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_SAVE_FORMAT(pub u32);
impl windows_core::TypeKind for REG_SAVE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_SAVE_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_SAVE_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REG_VALUE_TYPE(pub u32);
impl windows_core::TypeKind for REG_VALUE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REG_VALUE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REG_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSKTLSYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
impl windows_core::TypeKind for DSKTLSYSTEMTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSKTLSYSTEMTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEY(pub *mut core::ffi::c_void);
impl HKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HKEY {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = RegCloseKey(*self);
        }
    }
}
impl Default for HKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PVALUEA {
    pub pv_valuename: windows_core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl windows_core::TypeKind for PVALUEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PVALUEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PVALUEW {
    pub pv_valuename: windows_core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl windows_core::TypeKind for PVALUEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for PVALUEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct REG_PROVIDER {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VALENTA {
    pub ve_valuename: windows_core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl windows_core::TypeKind for VALENTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for VALENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VALENTW {
    pub ve_valuename: windows_core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl windows_core::TypeKind for VALENTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for VALENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut core::ffi::c_void,
    pub val_buff_ptr: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for val_context {
    type TypeKind = windows_core::CopyType;
}
impl Default for val_context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PQUERYHANDLER = Option<unsafe extern "system" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
