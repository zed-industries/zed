#[inline]
pub unsafe fn NtCommitRegistryTransaction<P0>(transactionhandle: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCommitRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCommitRegistryTransaction(transactionhandle.param().abi(), flags)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtCreateKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: u32, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtCreateKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateKey(keyhandle, desiredaccess, objectattributes, titleindex, core::mem::transmute(class.unwrap_or(std::ptr::null())), createoptions, core::mem::transmute(disposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtCreateKeyTransacted<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: u32, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, transactionhandle: P0, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCreateKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateKeyTransacted(keyhandle, desiredaccess, objectattributes, titleindex, core::mem::transmute(class.unwrap_or(std::ptr::null())), createoptions, transactionhandle.param().abi(), core::mem::transmute(disposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtCreateRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, createoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtCreateRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateRegistryTransaction(transactionhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), createoptions)
}
#[inline]
pub unsafe fn NtDeleteKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDeleteKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeleteKey(keyhandle.param().abi())
}
#[inline]
pub unsafe fn NtDeleteValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtDeleteValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeleteValueKey(keyhandle.param().abi(), valuename)
}
#[inline]
pub unsafe fn NtEnumerateKey<P0>(keyhandle: P0, index: u32, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtEnumerateKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtEnumerateKey(keyhandle.param().abi(), index, keyinformationclass, core::mem::transmute(keyinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn NtEnumerateValueKey<P0>(keyhandle: P0, index: u32, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtEnumerateValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtEnumerateValueKey(keyhandle.param().abi(), index, keyvalueinformationclass, core::mem::transmute(keyvalueinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn NtFlushKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtFlushKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtFlushKey(keyhandle.param().abi())
}
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
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenKey(keyhandle, desiredaccess, objectattributes)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenKeyEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenKeyEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenKeyEx(keyhandle, desiredaccess, objectattributes, openoptions)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenKeyTransacted<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, transactionhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenKeyTransacted(keyhandle, desiredaccess, objectattributes, transactionhandle.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenKeyTransactedEx<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32, transactionhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtOpenKeyTransactedEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenKeyTransactedEx(keyhandle, desiredaccess, objectattributes, openoptions, transactionhandle.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenRegistryTransaction(transactionhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn NtQueryKey<P0>(keyhandle: P0, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryKey(keyhandle.param().abi(), keyinformationclass, core::mem::transmute(keyinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
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
pub unsafe fn NtQueryValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryValueKey(keyhandle.param().abi(), valuename, keyvalueinformationclass, core::mem::transmute(keyvalueinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
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
pub unsafe fn NtRestoreKey<P0, P1>(keyhandle: P0, filehandle: P1, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtRestoreKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtRestoreKey(keyhandle.param().abi(), filehandle.param().abi(), flags)
}
#[inline]
pub unsafe fn NtRollbackRegistryTransaction<P0>(transactionhandle: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtRollbackRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtRollbackRegistryTransaction(transactionhandle.param().abi(), flags)
}
#[inline]
pub unsafe fn NtSaveKey<P0, P1>(keyhandle: P0, filehandle: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSaveKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSaveKey(keyhandle.param().abi(), filehandle.param().abi())
}
#[inline]
pub unsafe fn NtSaveKeyEx<P0, P1>(keyhandle: P0, filehandle: P1, format: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSaveKeyEx(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, format : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSaveKeyEx(keyhandle.param().abi(), filehandle.param().abi(), format)
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
pub unsafe fn NtSetValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, titleindex: u32, r#type: u32, data: Option<*const core::ffi::c_void>, datasize: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, titleindex : u32, r#type : u32, data : *const core::ffi::c_void, datasize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetValueKey(keyhandle.param().abi(), valuename, titleindex, r#type, core::mem::transmute(data.unwrap_or(std::ptr::null())), datasize)
}
#[inline]
pub unsafe fn ZwCommitRegistryTransaction<P0>(transactionhandle: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCommitRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCommitRegistryTransaction(transactionhandle.param().abi(), flags)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwCreateKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: u32, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateKey(keyhandle, desiredaccess, objectattributes, titleindex, core::mem::transmute(class.unwrap_or(std::ptr::null())), createoptions, core::mem::transmute(disposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwCreateKeyTransacted<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: u32, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, transactionhandle: P0, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateKeyTransacted(keyhandle, desiredaccess, objectattributes, titleindex, core::mem::transmute(class.unwrap_or(std::ptr::null())), createoptions, transactionhandle.param().abi(), core::mem::transmute(disposition.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwCreateRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, createoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateRegistryTransaction(transactionhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), createoptions)
}
#[inline]
pub unsafe fn ZwDeleteKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwDeleteKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDeleteKey(keyhandle.param().abi())
}
#[inline]
pub unsafe fn ZwDeleteValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwDeleteValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwDeleteValueKey(keyhandle.param().abi(), valuename)
}
#[inline]
pub unsafe fn ZwEnumerateKey<P0>(keyhandle: P0, index: u32, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwEnumerateKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwEnumerateKey(keyhandle.param().abi(), index, keyinformationclass, core::mem::transmute(keyinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn ZwEnumerateValueKey<P0>(keyhandle: P0, index: u32, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwEnumerateValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwEnumerateValueKey(keyhandle.param().abi(), index, keyvalueinformationclass, core::mem::transmute(keyvalueinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn ZwFlushKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwFlushKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwFlushKey(keyhandle.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ZwNotifyChangeMultipleKeys<P0, P1, P2, P3>(masterkeyhandle: P0, subordinateobjects: Option<&[super::super::Foundation::OBJECT_ATTRIBUTES]>, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwNotifyChangeMultipleKeys(masterkeyhandle : super::super::super::Win32::Foundation:: HANDLE, count : u32, subordinateobjects : *const super::super::Foundation:: OBJECT_ATTRIBUTES, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwNotifyChangeMultipleKeys(masterkeyhandle.param().abi(), subordinateobjects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(subordinateobjects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), event.param().abi(), apcroutine, core::mem::transmute(apccontext.unwrap_or(std::ptr::null())), iostatusblock, completionfilter, watchtree.param().abi(), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, asynchronous.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenKey(keyhandle, desiredaccess, objectattributes)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenKeyEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenKeyEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenKeyEx(keyhandle, desiredaccess, objectattributes, openoptions)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenKeyTransacted<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, transactionhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenKeyTransacted(keyhandle, desiredaccess, objectattributes, transactionhandle.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenKeyTransactedEx<P0>(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32, transactionhandle: P0) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenKeyTransactedEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenKeyTransactedEx(keyhandle, desiredaccess, objectattributes, openoptions, transactionhandle.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenRegistryTransaction(transactionhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn ZwQueryKey<P0>(keyhandle: P0, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryKey(keyhandle.param().abi(), keyinformationclass, core::mem::transmute(keyinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn ZwQueryMultipleValueKey<P0>(keyhandle: P0, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryMultipleValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryMultipleValueKey(keyhandle.param().abi(), core::mem::transmute(valueentries.as_ptr()), valueentries.len().try_into().unwrap(), valuebuffer, bufferlength, core::mem::transmute(requiredbufferlength.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ZwQueryValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryValueKey(keyhandle.param().abi(), valuename, keyvalueinformationclass, core::mem::transmute(keyvalueinformation.unwrap_or(std::ptr::null_mut())), length, resultlength)
}
#[inline]
pub unsafe fn ZwRenameKey<P0>(keyhandle: P0, newname: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwRenameKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, newname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwRenameKey(keyhandle.param().abi(), newname)
}
#[inline]
pub unsafe fn ZwRestoreKey<P0, P1>(keyhandle: P0, filehandle: P1, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwRestoreKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwRestoreKey(keyhandle.param().abi(), filehandle.param().abi(), flags)
}
#[inline]
pub unsafe fn ZwRollbackRegistryTransaction<P0>(transactionhandle: P0, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwRollbackRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwRollbackRegistryTransaction(transactionhandle.param().abi(), flags)
}
#[inline]
pub unsafe fn ZwSaveKey<P0, P1>(keyhandle: P0, filehandle: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSaveKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSaveKey(keyhandle.param().abi(), filehandle.param().abi())
}
#[inline]
pub unsafe fn ZwSaveKeyEx<P0, P1>(keyhandle: P0, filehandle: P1, format: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSaveKeyEx(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, format : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSaveKeyEx(keyhandle.param().abi(), filehandle.param().abi(), format)
}
#[inline]
pub unsafe fn ZwSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationKey(keyhandle.param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength)
}
#[inline]
pub unsafe fn ZwSetValueKey<P0>(keyhandle: P0, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, titleindex: u32, r#type: u32, data: Option<*const core::ffi::c_void>, datasize: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, titleindex : u32, r#type : u32, data : *const core::ffi::c_void, datasize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetValueKey(keyhandle.param().abi(), valuename, titleindex, r#type, core::mem::transmute(data.unwrap_or(std::ptr::null())), datasize)
}
pub const KeyBasicInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(0i32);
pub const KeyCachedInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(4i32);
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
pub const KeyFlagsInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(5i32);
pub const KeyFullInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(2i32);
pub const KeyHandleTagsInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(7i32);
pub const KeyLayerInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(9i32);
pub const KeyNameInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(3i32);
pub const KeyNodeInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(1i32);
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
pub const KeySetLayerInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
pub const KeyTrustInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(8i32);
pub const KeyValueBasicInformation: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(0i32);
pub const KeyValueFullInformation: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(1i32);
pub const KeyValueFullInformationAlign64: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(3i32);
pub const KeyValueLayerInformation: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(5i32);
pub const KeyValuePartialInformation: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(2i32);
pub const KeyValuePartialInformationAlign64: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(4i32);
pub const KeyVirtualizationInformation: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(6i32);
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
pub const MaxKeyInfoClass: KEY_INFORMATION_CLASS = KEY_INFORMATION_CLASS(10i32);
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(7i32);
pub const MaxKeyValueInfoClass: KEY_VALUE_INFORMATION_CLASS = KEY_VALUE_INFORMATION_CLASS(6i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KEY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KEY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KEY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KEY_VALUE_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_VALUE_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KEY_VALUE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KEY_VALUE_INFORMATION_CLASS").field(&self.0).finish()
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
pub struct REG_ENUMERATE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_ENUMERATE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_ENUMERATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REG_ENUMERATE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REG_QUERY_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_QUERY_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_QUERY_KEY_INFORMATION {
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
pub struct REG_QUERY_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REG_QUERY_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for REG_QUERY_VALUE_KEY_INFORMATION {
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
