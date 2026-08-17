#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut windows_core::BOOL) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn OOBEComplete(isoobecomplete : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { OOBEComplete(isoobecomplete as _).ok() }
}
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>, waithandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn RegisterWaitUntilOOBECompleted(oobecompletedcallback : OOBE_COMPLETED_CALLBACK, callbackcontext : *const core::ffi::c_void, waithandle : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RegisterWaitUntilOOBECompleted(oobecompletedcallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _, waithandle as _).ok() }
}
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn UnregisterWaitUntilOOBECompleted(waithandle : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UnregisterWaitUntilOOBECompleted(waithandle).ok() }
}
pub type OOBE_COMPLETED_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void)>;
