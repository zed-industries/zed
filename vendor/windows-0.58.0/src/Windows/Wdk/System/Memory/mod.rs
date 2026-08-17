#[inline]
pub unsafe fn NtMapViewOfSection<P0, P1>(sectionhandle: P0, processhandle: P1, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtMapViewOfSection(sectionhandle.param().abi(), processhandle.param().abi(), baseaddress, zerobits, commitsize, core::mem::transmute(sectionoffset.unwrap_or(std::ptr::null_mut())), viewsize, inheritdisposition, allocationtype, win32protect)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenSection(sectionhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn NtUnmapViewOfSection<P0>(processhandle: P0, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtUnmapViewOfSection(processhandle.param().abi(), core::mem::transmute(baseaddress.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn ZwMapViewOfSection<P0, P1>(sectionhandle: P0, processhandle: P1, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwMapViewOfSection(sectionhandle : super::super::super::Win32::Foundation:: HANDLE, processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwMapViewOfSection(sectionhandle.param().abi(), processhandle.param().abi(), baseaddress, zerobits, commitsize, core::mem::transmute(sectionoffset.unwrap_or(std::ptr::null_mut())), viewsize, inheritdisposition, allocationtype, win32protect)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenSection(sectionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenSection(sectionhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn ZwUnmapViewOfSection<P0>(processhandle: P0, baseaddress: Option<*const core::ffi::c_void>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwUnmapViewOfSection(processhandle : super::super::super::Win32::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwUnmapViewOfSection(processhandle.param().abi(), core::mem::transmute(baseaddress.unwrap_or(std::ptr::null())))
}
pub const ViewShare: SECTION_INHERIT = SECTION_INHERIT(1i32);
pub const ViewUnmap: SECTION_INHERIT = SECTION_INHERIT(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECTION_INHERIT(pub i32);
impl windows_core::TypeKind for SECTION_INHERIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECTION_INHERIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECTION_INHERIT").field(&self.0).finish()
    }
}
