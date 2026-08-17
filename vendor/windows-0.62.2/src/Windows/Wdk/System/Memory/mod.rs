#[inline]
pub unsafe fn NtMapViewOfSection(sectionhandle: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtMapViewOfSection(sectionhandle, processhandle, baseaddress as _, zerobits, commitsize, sectionoffset.unwrap_or(core::mem::zeroed()) as _, viewsize as _, inheritdisposition, allocationtype, win32protect) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenSection(sectionhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn NtUnmapViewOfSection(processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtUnmapViewOfSection(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ZwMapViewOfSection(sectionhandle: super::super::super::Win32::Foundation::HANDLE, processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwMapViewOfSection(sectionhandle, processhandle, baseaddress as _, zerobits, commitsize, sectionoffset.unwrap_or(core::mem::zeroed()) as _, viewsize as _, inheritdisposition, allocationtype, win32protect) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenSection(sectionhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn ZwUnmapViewOfSection(processhandle: super::super::super::Win32::Foundation::HANDLE, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwUnmapViewOfSection(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECTION_INHERIT(pub i32);
pub const ViewShare: SECTION_INHERIT = SECTION_INHERIT(1i32);
pub const ViewUnmap: SECTION_INHERIT = SECTION_INHERIT(2i32);
