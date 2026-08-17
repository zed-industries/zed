#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys<P0, P1, P2, P3>(masterkeyhandle: P0, subordinateobjects: Option<&[super::super::Foundation::OBJECT_ATTRIBUTES]>, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtNotifyChangeMultipleKeys(masterkeyhandle : super::super::super::Win32::Foundation:: HANDLE, count : u32, subordinateobjects : *const super::super::Foundation:: OBJECT_ATTRIBUTES, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtNotifyChangeMultipleKeys(masterkeyhandle.param().abi(), subordinateobjects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(subordinateobjects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, completionfilter, watchtree.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, asynchronous.param().abi())
}
#[inline]
pub unsafe fn NtQueryMultipleValueKey<P0>(keyhandle: P0, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryMultipleValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryMultipleValueKey(keyhandle.param().abi(), core::mem::transmute(valueentries.as_ptr()), valueentries.len().try_into().unwrap(), valuebuffer, bufferlength, core::mem::transmute(requiredbufferlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NtRenameKey<P0>(keyhandle: P0, newname: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtRenameKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, newname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtRenameKey(keyhandle.param().abi(), newname)
}
#[inline]
pub unsafe fn NtSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationKey(keyhandle.param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength)
}
#[inline]
pub unsafe fn ZwSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationKey(keyhandle.param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength)
}
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
pub const KeySetLayerInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(7i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_SET_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
impl windows_core::TypeKind for KEY_VALUE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueEntries: *mut KEY_VALUE_ENTRY,
    pub EntryCount: u32,
    pub ValueBuffer: *mut core::ffi::c_void,
    pub BufferLength: *mut u32,
    pub RequiredBufferLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REG_SET_INFORMATION_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
    pub KeySetInformation: *mut core::ffi::c_void,
    pub KeySetInformationLength: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_SET_INFORMATION_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_SET_INFORMATION_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
