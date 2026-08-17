#[inline]
pub unsafe fn NtCommitRegistryTransaction(transactionhandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCommitRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCommitRegistryTransaction(transactionhandle, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtCreateKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCreateKey(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtCreateKeyTransacted(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, transactionhandle: super::super::super::Win32::Foundation::HANDLE, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCreateKeyTransacted(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, transactionhandle, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtCreateRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, createoptions: Option<u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCreateRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NtDeleteKey(keyhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDeleteKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtDeleteKey(keyhandle) }
}
#[inline]
pub unsafe fn NtDeleteValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDeleteValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtDeleteValueKey(keyhandle, valuename) }
}
#[inline]
pub unsafe fn NtEnumerateKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, index: u32, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtEnumerateKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtEnumerateKey(keyhandle, index, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn NtEnumerateValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, index: u32, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtEnumerateValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtEnumerateValueKey(keyhandle, index, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn NtFlushKey(keyhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtFlushKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtFlushKey(keyhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys(masterkeyhandle: super::super::super::Win32::Foundation::HANDLE, subordinateobjects: Option<&[super::super::Foundation::OBJECT_ATTRIBUTES]>, event: Option<super::super::super::Win32::Foundation::HANDLE>, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: bool, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: bool) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtNotifyChangeMultipleKeys(masterkeyhandle : super::super::super::Win32::Foundation:: HANDLE, count : u32, subordinateobjects : *const super::super::Foundation:: OBJECT_ATTRIBUTES, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : bool, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtNotifyChangeMultipleKeys(masterkeyhandle, subordinateobjects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(subordinateobjects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, completionfilter, watchtree, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, asynchronous) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenKey(keyhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenKeyEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenKeyEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenKeyEx(keyhandle as _, desiredaccess, objectattributes, openoptions) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenKeyTransacted(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, transactionhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenKeyTransacted(keyhandle as _, desiredaccess, objectattributes, transactionhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenKeyTransactedEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32, transactionhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenKeyTransactedEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenKeyTransactedEx(keyhandle as _, desiredaccess, objectattributes, openoptions, transactionhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn NtQueryKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryKey(keyhandle, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn NtQueryMultipleValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryMultipleValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryMultipleValueKey(keyhandle, core::mem::transmute(valueentries.as_ptr()), valueentries.len().try_into().unwrap(), valuebuffer as _, bufferlength as _, requiredbufferlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NtQueryValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryValueKey(keyhandle, valuename, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn NtRenameKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, newname: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRenameKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, newname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtRenameKey(keyhandle, newname) }
}
#[inline]
pub unsafe fn NtRestoreKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: Option<super::super::super::Win32::Foundation::HANDLE>, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRestoreKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtRestoreKey(keyhandle, filehandle.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn NtRollbackRegistryTransaction(transactionhandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollbackRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtRollbackRegistryTransaction(transactionhandle, flags) }
}
#[inline]
pub unsafe fn NtSaveKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSaveKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSaveKey(keyhandle, filehandle) }
}
#[inline]
pub unsafe fn NtSaveKeyEx(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: super::super::super::Win32::Foundation::HANDLE, format: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSaveKeyEx(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, format : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSaveKeyEx(keyhandle, filehandle, format) }
}
#[inline]
pub unsafe fn NtSetInformationKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSetInformationKey(keyhandle, keysetinformationclass, keysetinformation, keysetinformationlength) }
}
#[inline]
pub unsafe fn NtSetValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, titleindex: Option<u32>, r#type: u32, data: Option<*const core::ffi::c_void>, datasize: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, titleindex : u32, r#type : u32, data : *const core::ffi::c_void, datasize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSetValueKey(keyhandle, valuename, titleindex.unwrap_or(core::mem::zeroed()) as _, r#type, data.unwrap_or(core::mem::zeroed()) as _, datasize) }
}
#[inline]
pub unsafe fn ZwCommitRegistryTransaction(transactionhandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCommitRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCommitRegistryTransaction(transactionhandle, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwCreateKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCreateKey(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwCreateKeyTransacted(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::super::super::Win32::Foundation::UNICODE_STRING>, createoptions: u32, transactionhandle: super::super::super::Win32::Foundation::HANDLE, disposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, disposition : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCreateKeyTransacted(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, transactionhandle, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwCreateRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, createoptions: Option<u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCreateRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ZwDeleteKey(keyhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwDeleteKey(keyhandle) }
}
#[inline]
pub unsafe fn ZwDeleteValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwDeleteValueKey(keyhandle, valuename) }
}
#[inline]
pub unsafe fn ZwEnumerateKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, index: u32, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwEnumerateKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwEnumerateKey(keyhandle, index, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn ZwEnumerateValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, index: u32, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwEnumerateValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, index : u32, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwEnumerateValueKey(keyhandle, index, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn ZwFlushKey(keyhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFlushKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwFlushKey(keyhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn ZwNotifyChangeMultipleKeys(masterkeyhandle: super::super::super::Win32::Foundation::HANDLE, subordinateobjects: Option<&[super::super::Foundation::OBJECT_ATTRIBUTES]>, event: Option<super::super::super::Win32::Foundation::HANDLE>, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: bool, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, asynchronous: bool) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwNotifyChangeMultipleKeys(masterkeyhandle : super::super::super::Win32::Foundation:: HANDLE, count : u32, subordinateobjects : *const super::super::Foundation:: OBJECT_ATTRIBUTES, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : bool, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwNotifyChangeMultipleKeys(masterkeyhandle, subordinateobjects.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(subordinateobjects.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, completionfilter, watchtree, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, asynchronous) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenKey(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKey(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenKey(keyhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenKeyEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenKeyEx(keyhandle as _, desiredaccess, objectattributes, openoptions) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenKeyTransacted(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, transactionhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyTransacted(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenKeyTransacted(keyhandle as _, desiredaccess, objectattributes, transactionhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenKeyTransactedEx(keyhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, openoptions: u32, transactionhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyTransactedEx(keyhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, openoptions : u32, transactionhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenKeyTransactedEx(keyhandle as _, desiredaccess, objectattributes, openoptions, transactionhandle) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenRegistryTransaction(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenRegistryTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn ZwQueryKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryKey(keyhandle, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn ZwQueryMultipleValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: Option<*mut u32>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryMultipleValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryMultipleValueKey(keyhandle, core::mem::transmute(valueentries.as_ptr()), valueentries.len().try_into().unwrap(), valuebuffer as _, bufferlength as _, requiredbufferlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ZwQueryValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryValueKey(keyhandle, valuename, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[inline]
pub unsafe fn ZwRenameKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, newname: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRenameKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, newname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwRenameKey(keyhandle, newname) }
}
#[inline]
pub unsafe fn ZwRestoreKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: Option<super::super::super::Win32::Foundation::HANDLE>, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRestoreKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwRestoreKey(keyhandle, filehandle.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn ZwRollbackRegistryTransaction(transactionhandle: super::super::super::Win32::Foundation::HANDLE, flags: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRollbackRegistryTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwRollbackRegistryTransaction(transactionhandle, flags) }
}
#[inline]
pub unsafe fn ZwSaveKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSaveKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSaveKey(keyhandle, filehandle) }
}
#[inline]
pub unsafe fn ZwSaveKeyEx(keyhandle: super::super::super::Win32::Foundation::HANDLE, filehandle: super::super::super::Win32::Foundation::HANDLE, format: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSaveKeyEx(keyhandle : super::super::super::Win32::Foundation:: HANDLE, filehandle : super::super::super::Win32::Foundation:: HANDLE, format : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSaveKeyEx(keyhandle, filehandle, format) }
}
#[inline]
pub unsafe fn ZwSetInformationKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSetInformationKey(keyhandle, keysetinformationclass, keysetinformation, keysetinformationlength) }
}
#[inline]
pub unsafe fn ZwSetValueKey(keyhandle: super::super::super::Win32::Foundation::HANDLE, valuename: *const super::super::super::Win32::Foundation::UNICODE_STRING, titleindex: Option<u32>, r#type: u32, data: Option<*const core::ffi::c_void>, datasize: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, titleindex : u32, r#type : u32, data : *const core::ffi::c_void, datasize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSetValueKey(keyhandle, valuename, titleindex.unwrap_or(core::mem::zeroed()) as _, r#type, data.unwrap_or(core::mem::zeroed()) as _, datasize) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEY_INFORMATION_CLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
impl Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEY_VALUE_INFORMATION_CLASS(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for REG_ENUMERATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for REG_QUERY_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for REG_QUERY_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SET_INFORMATION_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
    pub KeySetInformation: *mut core::ffi::c_void,
    pub KeySetInformationLength: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_SET_INFORMATION_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
