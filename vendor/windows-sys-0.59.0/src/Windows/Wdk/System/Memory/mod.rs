windows_targets::link!("ntdll.dll" "system" fn NtMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
pub const ViewShare: SECTION_INHERIT = 1i32;
pub const ViewUnmap: SECTION_INHERIT = 2i32;
pub type SECTION_INHERIT = i32;
