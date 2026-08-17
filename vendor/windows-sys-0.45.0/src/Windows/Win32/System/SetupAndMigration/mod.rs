#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"] fn OOBEComplete ( isoobecomplete : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"] fn RegisterWaitUntilOOBECompleted ( oobecompletedcallback : OOBE_COMPLETED_CALLBACK , callbackcontext : *const ::core::ffi::c_void , waithandle : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`, `\"Win32_Foundation\"`*"] fn UnregisterWaitUntilOOBECompleted ( waithandle : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[doc = "*Required features: `\"Win32_System_SetupAndMigration\"`*"]
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void) -> ()>;
