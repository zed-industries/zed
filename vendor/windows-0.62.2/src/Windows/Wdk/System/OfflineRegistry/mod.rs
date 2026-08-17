#[inline]
pub unsafe fn ORCloseHive(handle: ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORCloseHive(handle) }
}
#[inline]
pub unsafe fn ORCloseKey(keyhandle: ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORCloseKey(keyhandle) }
}
#[inline]
pub unsafe fn ORCreateHive(horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORCreateHive(horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORCreateHive(horkey as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORCreateKey<P1, P2>(keyhandle: ORHKEY, lpsubkey: P1, lpclass: P2, dwoptions: Option<u32>, psecuritydescriptor: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, phkresult: *mut ORHKEY, pdwdisposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpclass : windows_core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, phkresult : *mut ORHKEY, pdwdisposition : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORCreateKey(keyhandle, lpsubkey.param().abi(), lpclass.param().abi(), dwoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, phkresult as _, pdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ORDeleteKey<P1>(handle: ORHKEY, lpsubkey: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORDeleteKey(handle, lpsubkey.param().abi()) }
}
#[inline]
pub unsafe fn ORDeleteValue<P1>(handle: ORHKEY, lpvaluename: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORDeleteValue(handle, lpvaluename.param().abi()) }
}
#[inline]
pub unsafe fn OREnumKey(handle: ORHKEY, dwindex: u32, lpname: windows_core::PWSTR, lpcname: *mut u32, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_core::PWSTR, lpcname : *mut u32, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { OREnumKey(handle, dwindex, core::mem::transmute(lpname), lpcname as _, lpclass.unwrap_or(core::mem::zeroed()) as _, lpcclass.unwrap_or(core::mem::zeroed()) as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OREnumValue(handle: ORHKEY, dwindex: u32, lpvaluename: windows_core::PWSTR, lpcvaluename: *mut u32, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_core::PWSTR, lpcvaluename : *mut u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { OREnumValue(handle, dwindex, core::mem::transmute(lpvaluename), lpcvaluename as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORGetKeySecurity(handle: ORHKEY, securityinformation: u32, psecuritydescriptor: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, lpcbsecuritydescriptor: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORGetKeySecurity(handle, securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpcbsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn ORGetValue<P1, P2>(handle: ORHKEY, lpsubkey: P1, lpvalue: P2, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORGetValue(handle, lpsubkey.param().abi(), lpvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ORGetVersion(pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : *mut u32, pdwminorversion : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORGetVersion(pdwmajorversion as _, pdwminorversion as _) }
}
#[inline]
pub unsafe fn ORGetVirtualFlags(handle: ORHKEY, pdwflags: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORGetVirtualFlags(handle, pdwflags as _) }
}
#[inline]
pub unsafe fn ORMergeHives(hivehandles: &[ORHKEY], phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORMergeHives(core::mem::transmute(hivehandles.as_ptr()), hivehandles.len().try_into().unwrap(), phkresult as _) }
}
#[inline]
pub unsafe fn OROpenHive<P0>(filepath: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_core::PCWSTR, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { OROpenHive(filepath.param().abi(), horkey as _) }
}
#[inline]
pub unsafe fn OROpenHiveByHandle(filehandle: super::super::super::Win32::Foundation::HANDLE, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::super::super::Win32::Foundation:: HANDLE, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { OROpenHiveByHandle(filehandle, horkey as _) }
}
#[inline]
pub unsafe fn OROpenKey<P1>(handle: ORHKEY, lpsubkey: P1, phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { OROpenKey(handle, lpsubkey.param().abi(), phkresult as _) }
}
#[inline]
pub unsafe fn ORQueryInfoKey(handle: ORHKEY, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<*mut u32>, lpcsubkeys: Option<*mut u32>, lpcmaxsubkeylen: Option<*mut u32>, lpcmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcmaxvaluenamelen: Option<*mut u32>, lpcmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpcsubkeys : *mut u32, lpcmaxsubkeylen : *mut u32, lpcmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcmaxvaluenamelen : *mut u32, lpcmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe {
        ORQueryInfoKey(
            handle,
            lpclass.unwrap_or(core::mem::zeroed()) as _,
            lpcclass.unwrap_or(core::mem::zeroed()) as _,
            lpcsubkeys.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxclasslen.unwrap_or(core::mem::zeroed()) as _,
            lpcvalues.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _,
            lpcmaxvaluelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _,
            lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn ORRenameKey<P1>(handle: ORHKEY, lpnewname: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORRenameKey(handle, lpnewname.param().abi()) }
}
#[inline]
pub unsafe fn ORSaveHive<P1>(horkey: ORHKEY, hivepath: P1, osmajorversion: u32, osminorversion: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORSaveHive(horkey, hivepath.param().abi(), osmajorversion, osminorversion) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORSetKeySecurity(handle: ORHKEY, securityinformation: u32, psecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORSetKeySecurity(handle, securityinformation, psecuritydescriptor) }
}
#[inline]
pub unsafe fn ORSetValue<P1>(handle: ORHKEY, lpvaluename: P1, dwtype: u32, lpdata: Option<&[u8]>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORSetValue(handle, lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ORSetVirtualFlags(handle: ORHKEY, dwflags: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORSetVirtualFlags(handle, dwflags) }
}
#[inline]
pub unsafe fn ORShutdown() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORShutdown() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORShutdown() }
}
#[inline]
pub unsafe fn ORStart() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_core::link!("offreg.dll" "system" fn ORStart() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    unsafe { ORStart() }
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
            windows_core::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : *mut core::ffi::c_void) -> u32);
            unsafe {
                ORCloseKey(self.0);
            }
        }
    }
}
impl Default for ORHKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
