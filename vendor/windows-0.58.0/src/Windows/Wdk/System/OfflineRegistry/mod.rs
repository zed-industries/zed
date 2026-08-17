#[inline]
pub unsafe fn ORCloseHive<P0>(handle: P0) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCloseHive(handle.param().abi())
}
#[inline]
pub unsafe fn ORCloseKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCloseKey(keyhandle.param().abi())
}
#[inline]
pub unsafe fn ORCreateHive(horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORCreateHive(horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCreateHive(horkey)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORCreateKey<P0, P1, P2, P3>(keyhandle: P0, lpsubkey: P1, lpclass: P2, dwoptions: u32, psecuritydescriptor: P3, phkresult: *mut ORHKEY, pdwdisposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpclass : windows_core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, phkresult : *mut ORHKEY, pdwdisposition : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCreateKey(keyhandle.param().abi(), lpsubkey.param().abi(), lpclass.param().abi(), dwoptions, psecuritydescriptor.param().abi(), phkresult, core::mem::transmute(pdwdisposition.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ORDeleteKey<P0, P1>(handle: P0, lpsubkey: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORDeleteKey(handle.param().abi(), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn ORDeleteValue<P0, P1>(handle: P0, lpvaluename: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORDeleteValue(handle.param().abi(), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn OREnumKey<P0>(handle: P0, dwindex: u32, lpname: windows_core::PWSTR, lpcname: *mut u32, lpclass: windows_core::PWSTR, lpcclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_core::PWSTR, lpcname : *mut u32, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OREnumKey(handle.param().abi(), dwindex, core::mem::transmute(lpname), lpcname, core::mem::transmute(lpclass), core::mem::transmute(lpcclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OREnumValue<P0>(handle: P0, dwindex: u32, lpvaluename: windows_core::PWSTR, lpcvaluename: *mut u32, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_core::PWSTR, lpcvaluename : *mut u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OREnumValue(handle.param().abi(), dwindex, core::mem::transmute(lpvaluename), lpcvaluename, core::mem::transmute(lptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcbdata.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORGetKeySecurity<P0>(handle: P0, securityinformation: u32, psecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetKeySecurity(handle.param().abi(), securityinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[inline]
pub unsafe fn ORGetValue<P0, P1, P2>(handle: P0, lpsubkey: P1, lpvalue: P2, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetValue(handle.param().abi(), lpsubkey.param().abi(), lpvalue.param().abi(), core::mem::transmute(pdwtype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvdata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ORGetVersion(pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : *mut u32, pdwminorversion : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetVersion(pdwmajorversion, pdwminorversion)
}
#[inline]
pub unsafe fn ORGetVirtualFlags<P0>(handle: P0, pdwflags: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetVirtualFlags(handle.param().abi(), pdwflags)
}
#[inline]
pub unsafe fn ORMergeHives(hivehandles: &[ORHKEY], phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORMergeHives(core::mem::transmute(hivehandles.as_ptr()), hivehandles.len().try_into().unwrap(), phkresult)
}
#[inline]
pub unsafe fn OROpenHive<P0>(filepath: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_core::PCWSTR, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenHive(filepath.param().abi(), horkey)
}
#[inline]
pub unsafe fn OROpenHiveByHandle<P0>(filehandle: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::super::super::Win32::Foundation:: HANDLE, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenHiveByHandle(filehandle.param().abi(), horkey)
}
#[inline]
pub unsafe fn OROpenKey<P0, P1>(handle: P0, lpsubkey: P1, phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenKey(handle.param().abi(), lpsubkey.param().abi(), phkresult)
}
#[inline]
pub unsafe fn ORQueryInfoKey<P0>(handle: P0, lpclass: windows_core::PWSTR, lpcclass: Option<*mut u32>, lpcsubkeys: Option<*mut u32>, lpcmaxsubkeylen: Option<*mut u32>, lpcmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcmaxvaluenamelen: Option<*mut u32>, lpcmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpcsubkeys : *mut u32, lpcmaxsubkeylen : *mut u32, lpcmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcmaxvaluenamelen : *mut u32, lpcmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORQueryInfoKey(
        handle.param().abi(),
        core::mem::transmute(lpclass),
        core::mem::transmute(lpcclass.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcsubkeys.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcmaxsubkeylen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcmaxclasslen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcvalues.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcmaxvaluenamelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcmaxvaluelen.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(lpftlastwritetime.unwrap_or(std::ptr::null_mut())),
    )
}
#[inline]
pub unsafe fn ORRenameKey<P0, P1>(handle: P0, lpnewname: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORRenameKey(handle.param().abi(), lpnewname.param().abi())
}
#[inline]
pub unsafe fn ORSaveHive<P0, P1>(horkey: P0, hivepath: P1, osmajorversion: u32, osminorversion: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSaveHive(horkey.param().abi(), hivepath.param().abi(), osmajorversion, osminorversion)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORSetKeySecurity<P0, P1>(handle: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetKeySecurity(handle.param().abi(), securityinformation, psecuritydescriptor.param().abi())
}
#[inline]
pub unsafe fn ORSetValue<P0, P1>(handle: P0, lpvaluename: P1, dwtype: u32, lpdata: Option<&[u8]>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetValue(handle.param().abi(), lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn ORSetVirtualFlags<P0>(handle: P0, dwflags: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<ORHKEY>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetVirtualFlags(handle.param().abi(), dwflags)
}
#[inline]
pub unsafe fn ORShutdown() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORShutdown() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORShutdown()
}
#[inline]
pub unsafe fn ORStart() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORStart() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORStart()
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ORHKEY(pub *mut core::ffi::c_void);
impl ORHKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for ORHKEY {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = ORCloseKey(*self);
        }
    }
}
impl Default for ORHKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ORHKEY {
    type TypeKind = windows_core::CopyType;
}
