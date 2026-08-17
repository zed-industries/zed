windows_targets::link!("kernel32.dll" "system" fn OOBEComplete(isoobecomplete : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn RegisterWaitUntilOOBECompleted(oobecompletedcallback : OOBE_COMPLETED_CALLBACK, callbackcontext : *const core::ffi::c_void, waithandle : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn UnregisterWaitUntilOOBECompleted(waithandle : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
pub type OOBE_COMPLETED_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void)>;
