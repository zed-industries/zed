#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ActivateActCtx ( hactctx : super::super::Foundation:: HANDLE , lpcookie : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn AddRefActCtx ( hactctx : super::super::Foundation:: HANDLE ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyDeltaA ( applyflags : i64 , lpsourcename : :: windows_sys::core::PCSTR , lpdeltaname : :: windows_sys::core::PCSTR , lptargetname : :: windows_sys::core::PCSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyDeltaB ( applyflags : i64 , source : DELTA_INPUT , delta : DELTA_INPUT , lptarget : *mut DELTA_OUTPUT ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyDeltaGetReverseB ( applyflags : i64 , source : DELTA_INPUT , delta : DELTA_INPUT , lpreversefiletime : *const super::super::Foundation:: FILETIME , lptarget : *mut DELTA_OUTPUT , lptargetreverse : *mut DELTA_OUTPUT ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyDeltaProvidedB ( applyflags : i64 , source : DELTA_INPUT , delta : DELTA_INPUT , lptarget : *mut ::core::ffi::c_void , utargetsize : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyDeltaW ( applyflags : i64 , lpsourcename : :: windows_sys::core::PCWSTR , lpdeltaname : :: windows_sys::core::PCWSTR , lptargetname : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileA ( patchfilename : :: windows_sys::core::PCSTR , oldfilename : :: windows_sys::core::PCSTR , newfilename : :: windows_sys::core::PCSTR , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileByBuffers ( patchfilemapped : *const u8 , patchfilesize : u32 , oldfilemapped : *const u8 , oldfilesize : u32 , newfilebuffer : *mut *mut u8 , newfilebuffersize : u32 , newfileactualsize : *mut u32 , newfiletime : *mut super::super::Foundation:: FILETIME , applyoptionflags : u32 , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileByHandles ( patchfilehandle : super::super::Foundation:: HANDLE , oldfilehandle : super::super::Foundation:: HANDLE , newfilehandle : super::super::Foundation:: HANDLE , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileByHandlesEx ( patchfilehandle : super::super::Foundation:: HANDLE , oldfilehandle : super::super::Foundation:: HANDLE , newfilehandle : super::super::Foundation:: HANDLE , applyoptionflags : u32 , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileExA ( patchfilename : :: windows_sys::core::PCSTR , oldfilename : :: windows_sys::core::PCSTR , newfilename : :: windows_sys::core::PCSTR , applyoptionflags : u32 , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileExW ( patchfilename : :: windows_sys::core::PCWSTR , oldfilename : :: windows_sys::core::PCWSTR , newfilename : :: windows_sys::core::PCWSTR , applyoptionflags : u32 , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ApplyPatchToFileW ( patchfilename : :: windows_sys::core::PCWSTR , oldfilename : :: windows_sys::core::PCWSTR , newfilename : :: windows_sys::core::PCWSTR , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreateActCtxA ( pactctx : *const ACTCTXA ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreateActCtxW ( pactctx : *const ACTCTXW ) -> super::super::Foundation:: HANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreateDeltaA ( filetypeset : i64 , setflags : i64 , resetflags : i64 , lpsourcename : :: windows_sys::core::PCSTR , lptargetname : :: windows_sys::core::PCSTR , lpsourceoptionsname : :: windows_sys::core::PCSTR , lptargetoptionsname : :: windows_sys::core::PCSTR , globaloptions : DELTA_INPUT , lptargetfiletime : *const super::super::Foundation:: FILETIME , hashalgid : u32 , lpdeltaname : :: windows_sys::core::PCSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreateDeltaB ( filetypeset : i64 , setflags : i64 , resetflags : i64 , source : DELTA_INPUT , target : DELTA_INPUT , sourceoptions : DELTA_INPUT , targetoptions : DELTA_INPUT , globaloptions : DELTA_INPUT , lptargetfiletime : *const super::super::Foundation:: FILETIME , hashalgid : u32 , lpdelta : *mut DELTA_OUTPUT ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreateDeltaW ( filetypeset : i64 , setflags : i64 , resetflags : i64 , lpsourcename : :: windows_sys::core::PCWSTR , lptargetname : :: windows_sys::core::PCWSTR , lpsourceoptionsname : :: windows_sys::core::PCWSTR , lptargetoptionsname : :: windows_sys::core::PCWSTR , globaloptions : DELTA_INPUT , lptargetfiletime : *const super::super::Foundation:: FILETIME , hashalgid : u32 , lpdeltaname : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileA ( oldfilename : :: windows_sys::core::PCSTR , newfilename : :: windows_sys::core::PCSTR , patchfilename : :: windows_sys::core::PCSTR , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileByHandles ( oldfilehandle : super::super::Foundation:: HANDLE , newfilehandle : super::super::Foundation:: HANDLE , patchfilehandle : super::super::Foundation:: HANDLE , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileByHandlesEx ( oldfilecount : u32 , oldfileinfoarray : *const PATCH_OLD_FILE_INFO_H , newfilehandle : super::super::Foundation:: HANDLE , patchfilehandle : super::super::Foundation:: HANDLE , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileExA ( oldfilecount : u32 , oldfileinfoarray : *const PATCH_OLD_FILE_INFO_A , newfilename : :: windows_sys::core::PCSTR , patchfilename : :: windows_sys::core::PCSTR , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileExW ( oldfilecount : u32 , oldfileinfoarray : *const PATCH_OLD_FILE_INFO_W , newfilename : :: windows_sys::core::PCWSTR , patchfilename : :: windows_sys::core::PCWSTR , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA , progresscallback : PPATCH_PROGRESS_CALLBACK , callbackcontext : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn CreatePatchFileW ( oldfilename : :: windows_sys::core::PCWSTR , newfilename : :: windows_sys::core::PCWSTR , patchfilename : :: windows_sys::core::PCWSTR , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn DeactivateActCtx ( dwflags : u32 , ulcookie : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn DeltaFree ( lpmemory : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn DeltaNormalizeProvidedB ( filetypeset : i64 , normalizeflags : i64 , normalizeoptions : DELTA_INPUT , lpsource : *mut ::core::ffi::c_void , usourcesize : usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ExtractPatchHeaderToFileA ( patchfilename : :: windows_sys::core::PCSTR , patchheaderfilename : :: windows_sys::core::PCSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ExtractPatchHeaderToFileByHandles ( patchfilehandle : super::super::Foundation:: HANDLE , patchheaderfilehandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatchc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ExtractPatchHeaderToFileW ( patchfilename : :: windows_sys::core::PCWSTR , patchheaderfilename : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FindActCtxSectionGuid ( dwflags : u32 , lpextensionguid : *const :: windows_sys::core::GUID , ulsectionid : u32 , lpguidtofind : *const :: windows_sys::core::GUID , returneddata : *mut ACTCTX_SECTION_KEYED_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FindActCtxSectionStringA ( dwflags : u32 , lpextensionguid : *const :: windows_sys::core::GUID , ulsectionid : u32 , lpstringtofind : :: windows_sys::core::PCSTR , returneddata : *mut ACTCTX_SECTION_KEYED_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"] fn FindActCtxSectionStringW ( dwflags : u32 , lpextensionguid : *const :: windows_sys::core::GUID , ulsectionid : u32 , lpstringtofind : :: windows_sys::core::PCWSTR , returneddata : *mut ACTCTX_SECTION_KEYED_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetCurrentActCtx ( lphactctx : *mut super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaInfoA ( lpdeltaname : :: windows_sys::core::PCSTR , lpheaderinfo : *mut DELTA_HEADER_INFO ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaInfoB ( delta : DELTA_INPUT , lpheaderinfo : *mut DELTA_HEADER_INFO ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaInfoW ( lpdeltaname : :: windows_sys::core::PCWSTR , lpheaderinfo : *mut DELTA_HEADER_INFO ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaSignatureA ( filetypeset : i64 , hashalgid : u32 , lpsourcename : :: windows_sys::core::PCSTR , lphash : *mut DELTA_HASH ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaSignatureB ( filetypeset : i64 , hashalgid : u32 , source : DELTA_INPUT , lphash : *mut DELTA_HASH ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msdelta.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetDeltaSignatureW ( filetypeset : i64 , hashalgid : u32 , lpsourcename : :: windows_sys::core::PCWSTR , lphash : *mut DELTA_HASH ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetFilePatchSignatureA ( filename : :: windows_sys::core::PCSTR , optionflags : u32 , optiondata : *const ::core::ffi::c_void , ignorerangecount : u32 , ignorerangearray : *const PATCH_IGNORE_RANGE , retainrangecount : u32 , retainrangearray : *const PATCH_RETAIN_RANGE , signaturebuffersize : u32 , signaturebuffer : :: windows_sys::core::PSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetFilePatchSignatureByBuffer ( filebufferwritable : *mut u8 , filesize : u32 , optionflags : u32 , optiondata : *const ::core::ffi::c_void , ignorerangecount : u32 , ignorerangearray : *const PATCH_IGNORE_RANGE , retainrangecount : u32 , retainrangearray : *const PATCH_RETAIN_RANGE , signaturebuffersize : u32 , signaturebuffer : :: windows_sys::core::PSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetFilePatchSignatureByHandle ( filehandle : super::super::Foundation:: HANDLE , optionflags : u32 , optiondata : *const ::core::ffi::c_void , ignorerangecount : u32 , ignorerangearray : *const PATCH_IGNORE_RANGE , retainrangecount : u32 , retainrangearray : *const PATCH_RETAIN_RANGE , signaturebuffersize : u32 , signaturebuffer : :: windows_sys::core::PSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn GetFilePatchSignatureW ( filename : :: windows_sys::core::PCWSTR , optionflags : u32 , optiondata : *const ::core::ffi::c_void , ignorerangecount : u32 , ignorerangearray : *const PATCH_IGNORE_RANGE , retainrangecount : u32 , retainrangearray : *const PATCH_RETAIN_RANGE , signaturebuffersize : u32 , signaturebuffer : :: windows_sys::core::PWSTR ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiAdvertiseProductA ( szpackagepath : :: windows_sys::core::PCSTR , szscriptfilepath : :: windows_sys::core::PCSTR , sztransforms : :: windows_sys::core::PCSTR , lgidlanguage : u16 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiAdvertiseProductExA ( szpackagepath : :: windows_sys::core::PCSTR , szscriptfilepath : :: windows_sys::core::PCSTR , sztransforms : :: windows_sys::core::PCSTR , lgidlanguage : u16 , dwplatform : u32 , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiAdvertiseProductExW ( szpackagepath : :: windows_sys::core::PCWSTR , szscriptfilepath : :: windows_sys::core::PCWSTR , sztransforms : :: windows_sys::core::PCWSTR , lgidlanguage : u16 , dwplatform : u32 , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiAdvertiseProductW ( szpackagepath : :: windows_sys::core::PCWSTR , szscriptfilepath : :: windows_sys::core::PCWSTR , sztransforms : :: windows_sys::core::PCWSTR , lgidlanguage : u16 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn MsiAdvertiseScriptA ( szscriptfile : :: windows_sys::core::PCSTR , dwflags : u32 , phregdata : *const super::Registry:: HKEY , fremoveitems : super::super::Foundation:: BOOL ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn MsiAdvertiseScriptW ( szscriptfile : :: windows_sys::core::PCWSTR , dwflags : u32 , phregdata : *const super::Registry:: HKEY , fremoveitems : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiApplyMultiplePatchesA ( szpatchpackages : :: windows_sys::core::PCSTR , szproductcode : :: windows_sys::core::PCSTR , szpropertieslist : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiApplyMultiplePatchesW ( szpatchpackages : :: windows_sys::core::PCWSTR , szproductcode : :: windows_sys::core::PCWSTR , szpropertieslist : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiApplyPatchA ( szpatchpackage : :: windows_sys::core::PCSTR , szinstallpackage : :: windows_sys::core::PCSTR , einstalltype : INSTALLTYPE , szcommandline : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiApplyPatchW ( szpatchpackage : :: windows_sys::core::PCWSTR , szinstallpackage : :: windows_sys::core::PCWSTR , einstalltype : INSTALLTYPE , szcommandline : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiBeginTransactionA ( szname : :: windows_sys::core::PCSTR , dwtransactionattributes : u32 , phtransactionhandle : *mut MSIHANDLE , phchangeofownerevent : *mut super::super::Foundation:: HANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiBeginTransactionW ( szname : :: windows_sys::core::PCWSTR , dwtransactionattributes : u32 , phtransactionhandle : *mut MSIHANDLE , phchangeofownerevent : *mut super::super::Foundation:: HANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCloseAllHandles ( ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCloseHandle ( hany : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCollectUserInfoA ( szproduct : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCollectUserInfoW ( szproduct : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureFeatureA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureFeatureW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureProductA ( szproduct : :: windows_sys::core::PCSTR , iinstalllevel : INSTALLLEVEL , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureProductExA ( szproduct : :: windows_sys::core::PCSTR , iinstalllevel : INSTALLLEVEL , einstallstate : INSTALLSTATE , szcommandline : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureProductExW ( szproduct : :: windows_sys::core::PCWSTR , iinstalllevel : INSTALLLEVEL , einstallstate : INSTALLSTATE , szcommandline : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiConfigureProductW ( szproduct : :: windows_sys::core::PCWSTR , iinstalllevel : INSTALLLEVEL , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCreateRecord ( cparams : u32 ) -> MSIHANDLE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCreateTransformSummaryInfoA ( hdatabase : MSIHANDLE , hdatabasereference : MSIHANDLE , sztransformfile : :: windows_sys::core::PCSTR , ierrorconditions : MSITRANSFORM_ERROR , ivalidation : MSITRANSFORM_VALIDATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiCreateTransformSummaryInfoW ( hdatabase : MSIHANDLE , hdatabasereference : MSIHANDLE , sztransformfile : :: windows_sys::core::PCWSTR , ierrorconditions : MSITRANSFORM_ERROR , ivalidation : MSITRANSFORM_VALIDATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseApplyTransformA ( hdatabase : MSIHANDLE , sztransformfile : :: windows_sys::core::PCSTR , ierrorconditions : MSITRANSFORM_ERROR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseApplyTransformW ( hdatabase : MSIHANDLE , sztransformfile : :: windows_sys::core::PCWSTR , ierrorconditions : MSITRANSFORM_ERROR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseCommit ( hdatabase : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseExportA ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCSTR , szfolderpath : :: windows_sys::core::PCSTR , szfilename : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseExportW ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCWSTR , szfolderpath : :: windows_sys::core::PCWSTR , szfilename : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseGenerateTransformA ( hdatabase : MSIHANDLE , hdatabasereference : MSIHANDLE , sztransformfile : :: windows_sys::core::PCSTR , ireserved1 : i32 , ireserved2 : i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseGenerateTransformW ( hdatabase : MSIHANDLE , hdatabasereference : MSIHANDLE , sztransformfile : :: windows_sys::core::PCWSTR , ireserved1 : i32 , ireserved2 : i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseGetPrimaryKeysA ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCSTR , phrecord : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseGetPrimaryKeysW ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCWSTR , phrecord : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseImportA ( hdatabase : MSIHANDLE , szfolderpath : :: windows_sys::core::PCSTR , szfilename : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseImportW ( hdatabase : MSIHANDLE , szfolderpath : :: windows_sys::core::PCWSTR , szfilename : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseIsTablePersistentA ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCSTR ) -> MSICONDITION );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseIsTablePersistentW ( hdatabase : MSIHANDLE , sztablename : :: windows_sys::core::PCWSTR ) -> MSICONDITION );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseMergeA ( hdatabase : MSIHANDLE , hdatabasemerge : MSIHANDLE , sztablename : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseMergeW ( hdatabase : MSIHANDLE , hdatabasemerge : MSIHANDLE , sztablename : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseOpenViewA ( hdatabase : MSIHANDLE , szquery : :: windows_sys::core::PCSTR , phview : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDatabaseOpenViewW ( hdatabase : MSIHANDLE , szquery : :: windows_sys::core::PCWSTR , phview : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDetermineApplicablePatchesA ( szproductpackagepath : :: windows_sys::core::PCSTR , cpatchinfo : u32 , ppatchinfo : *mut MSIPATCHSEQUENCEINFOA ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDetermineApplicablePatchesW ( szproductpackagepath : :: windows_sys::core::PCWSTR , cpatchinfo : u32 , ppatchinfo : *mut MSIPATCHSEQUENCEINFOW ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDeterminePatchSequenceA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , cpatchinfo : u32 , ppatchinfo : *mut MSIPATCHSEQUENCEINFOA ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDeterminePatchSequenceW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , cpatchinfo : u32 , ppatchinfo : *mut MSIPATCHSEQUENCEINFOW ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDoActionA ( hinstall : MSIHANDLE , szaction : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiDoActionW ( hinstall : MSIHANDLE , szaction : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnableLogA ( dwlogmode : INSTALLLOGMODE , szlogfile : :: windows_sys::core::PCSTR , dwlogattributes : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnableLogW ( dwlogmode : INSTALLLOGMODE , szlogfile : :: windows_sys::core::PCWSTR , dwlogattributes : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnableUIPreview ( hdatabase : MSIHANDLE , phpreview : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEndTransaction ( dwtransactionstate : MSITRANSACTIONSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumClientsA ( szcomponent : :: windows_sys::core::PCSTR , iproductindex : u32 , lpproductbuf : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumClientsExA ( szcomponent : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwproductindex : u32 , szproductbuf : :: windows_sys::core::PSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumClientsExW ( szcomponent : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwproductindex : u32 , szproductbuf : :: windows_sys::core::PWSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PWSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumClientsW ( szcomponent : :: windows_sys::core::PCWSTR , iproductindex : u32 , lpproductbuf : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentCostsA ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCSTR , dwindex : u32 , istate : INSTALLSTATE , szdrivebuf : :: windows_sys::core::PSTR , pcchdrivebuf : *mut u32 , picost : *mut i32 , pitempcost : *mut i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentCostsW ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCWSTR , dwindex : u32 , istate : INSTALLSTATE , szdrivebuf : :: windows_sys::core::PWSTR , pcchdrivebuf : *mut u32 , picost : *mut i32 , pitempcost : *mut i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentQualifiersA ( szcomponent : :: windows_sys::core::PCSTR , iindex : u32 , lpqualifierbuf : :: windows_sys::core::PSTR , pcchqualifierbuf : *mut u32 , lpapplicationdatabuf : :: windows_sys::core::PSTR , pcchapplicationdatabuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentQualifiersW ( szcomponent : :: windows_sys::core::PCWSTR , iindex : u32 , lpqualifierbuf : :: windows_sys::core::PWSTR , pcchqualifierbuf : *mut u32 , lpapplicationdatabuf : :: windows_sys::core::PWSTR , pcchapplicationdatabuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentsA ( icomponentindex : u32 , lpcomponentbuf : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentsExA ( szusersid : :: windows_sys::core::PCSTR , dwcontext : u32 , dwindex : u32 , szinstalledcomponentcode : :: windows_sys::core::PSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentsExW ( szusersid : :: windows_sys::core::PCWSTR , dwcontext : u32 , dwindex : u32 , szinstalledcomponentcode : :: windows_sys::core::PWSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PWSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumComponentsW ( icomponentindex : u32 , lpcomponentbuf : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumFeaturesA ( szproduct : :: windows_sys::core::PCSTR , ifeatureindex : u32 , lpfeaturebuf : :: windows_sys::core::PSTR , lpparentbuf : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumFeaturesW ( szproduct : :: windows_sys::core::PCWSTR , ifeatureindex : u32 , lpfeaturebuf : :: windows_sys::core::PWSTR , lpparentbuf : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumPatchesA ( szproduct : :: windows_sys::core::PCSTR , ipatchindex : u32 , lppatchbuf : :: windows_sys::core::PSTR , lptransformsbuf : :: windows_sys::core::PSTR , pcchtransformsbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumPatchesExA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : u32 , dwfilter : u32 , dwindex : u32 , szpatchcode : :: windows_sys::core::PSTR , sztargetproductcode : :: windows_sys::core::PSTR , pdwtargetproductcontext : *mut MSIINSTALLCONTEXT , sztargetusersid : :: windows_sys::core::PSTR , pcchtargetusersid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumPatchesExW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : u32 , dwfilter : u32 , dwindex : u32 , szpatchcode : :: windows_sys::core::PWSTR , sztargetproductcode : :: windows_sys::core::PWSTR , pdwtargetproductcontext : *mut MSIINSTALLCONTEXT , sztargetusersid : :: windows_sys::core::PWSTR , pcchtargetusersid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumPatchesW ( szproduct : :: windows_sys::core::PCWSTR , ipatchindex : u32 , lppatchbuf : :: windows_sys::core::PWSTR , lptransformsbuf : :: windows_sys::core::PWSTR , pcchtransformsbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumProductsA ( iproductindex : u32 , lpproductbuf : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumProductsExA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : u32 , dwindex : u32 , szinstalledproductcode : :: windows_sys::core::PSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumProductsExW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : u32 , dwindex : u32 , szinstalledproductcode : :: windows_sys::core::PWSTR , pdwinstalledcontext : *mut MSIINSTALLCONTEXT , szsid : :: windows_sys::core::PWSTR , pcchsid : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumProductsW ( iproductindex : u32 , lpproductbuf : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumRelatedProductsA ( lpupgradecode : :: windows_sys::core::PCSTR , dwreserved : u32 , iproductindex : u32 , lpproductbuf : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEnumRelatedProductsW ( lpupgradecode : :: windows_sys::core::PCWSTR , dwreserved : u32 , iproductindex : u32 , lpproductbuf : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEvaluateConditionA ( hinstall : MSIHANDLE , szcondition : :: windows_sys::core::PCSTR ) -> MSICONDITION );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiEvaluateConditionW ( hinstall : MSIHANDLE , szcondition : :: windows_sys::core::PCWSTR ) -> MSICONDITION );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiExtractPatchXMLDataA ( szpatchpath : :: windows_sys::core::PCSTR , dwreserved : u32 , szxmldata : :: windows_sys::core::PSTR , pcchxmldata : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiExtractPatchXMLDataW ( szpatchpath : :: windows_sys::core::PCWSTR , dwreserved : u32 , szxmldata : :: windows_sys::core::PWSTR , pcchxmldata : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiFormatRecordA ( hinstall : MSIHANDLE , hrecord : MSIHANDLE , szresultbuf : :: windows_sys::core::PSTR , pcchresultbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiFormatRecordW ( hinstall : MSIHANDLE , hrecord : MSIHANDLE , szresultbuf : :: windows_sys::core::PWSTR , pcchresultbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetActiveDatabase ( hinstall : MSIHANDLE ) -> MSIHANDLE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentPathA ( szproduct : :: windows_sys::core::PCSTR , szcomponent : :: windows_sys::core::PCSTR , lppathbuf : :: windows_sys::core::PSTR , pcchbuf : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentPathExA ( szproductcode : :: windows_sys::core::PCSTR , szcomponentcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , lpoutpathbuffer : :: windows_sys::core::PSTR , pcchoutpathbuffer : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentPathExW ( szproductcode : :: windows_sys::core::PCWSTR , szcomponentcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , lpoutpathbuffer : :: windows_sys::core::PWSTR , pcchoutpathbuffer : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentPathW ( szproduct : :: windows_sys::core::PCWSTR , szcomponent : :: windows_sys::core::PCWSTR , lppathbuf : :: windows_sys::core::PWSTR , pcchbuf : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentStateA ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCSTR , piinstalled : *mut INSTALLSTATE , piaction : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetComponentStateW ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCWSTR , piinstalled : *mut INSTALLSTATE , piaction : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetDatabaseState ( hdatabase : MSIHANDLE ) -> MSIDBSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureCostA ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , icosttree : MSICOSTTREE , istate : INSTALLSTATE , picost : *mut i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureCostW ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , icosttree : MSICOSTTREE , istate : INSTALLSTATE , picost : *mut i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureInfoA ( hproduct : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , lpattributes : *mut u32 , lptitlebuf : :: windows_sys::core::PSTR , pcchtitlebuf : *mut u32 , lphelpbuf : :: windows_sys::core::PSTR , pcchhelpbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureInfoW ( hproduct : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , lpattributes : *mut u32 , lptitlebuf : :: windows_sys::core::PWSTR , pcchtitlebuf : *mut u32 , lphelpbuf : :: windows_sys::core::PWSTR , pcchhelpbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureStateA ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , piinstalled : *mut INSTALLSTATE , piaction : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureStateW ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , piinstalled : *mut INSTALLSTATE , piaction : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureUsageA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR , pdwusecount : *mut u32 , pwdateused : *mut u16 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureUsageW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR , pdwusecount : *mut u32 , pwdateused : *mut u16 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureValidStatesA ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , lpinstallstates : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFeatureValidStatesW ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , lpinstallstates : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFileHashA ( szfilepath : :: windows_sys::core::PCSTR , dwoptions : u32 , phash : *mut MSIFILEHASHINFO ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFileHashW ( szfilepath : :: windows_sys::core::PCWSTR , dwoptions : u32 , phash : *mut MSIFILEHASHINFO ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"] fn MsiGetFileSignatureInformationA ( szsignedobjectpath : :: windows_sys::core::PCSTR , dwflags : u32 , ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT , pbhashdata : *mut u8 , pcbhashdata : *mut u32 ) -> :: windows_sys::core::HRESULT );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"] fn MsiGetFileSignatureInformationW ( szsignedobjectpath : :: windows_sys::core::PCWSTR , dwflags : u32 , ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT , pbhashdata : *mut u8 , pcbhashdata : *mut u32 ) -> :: windows_sys::core::HRESULT );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFileVersionA ( szfilepath : :: windows_sys::core::PCSTR , lpversionbuf : :: windows_sys::core::PSTR , pcchversionbuf : *mut u32 , lplangbuf : :: windows_sys::core::PSTR , pcchlangbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetFileVersionW ( szfilepath : :: windows_sys::core::PCWSTR , lpversionbuf : :: windows_sys::core::PWSTR , pcchversionbuf : *mut u32 , lplangbuf : :: windows_sys::core::PWSTR , pcchlangbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetLanguage ( hinstall : MSIHANDLE ) -> u16 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetLastErrorRecord ( ) -> MSIHANDLE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiGetMode ( hinstall : MSIHANDLE , erunmode : MSIRUNMODE ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchFileListA ( szproductcode : :: windows_sys::core::PCSTR , szpatchpackages : :: windows_sys::core::PCSTR , pcfiles : *mut u32 , pphfilerecords : *mut *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchFileListW ( szproductcode : :: windows_sys::core::PCWSTR , szpatchpackages : :: windows_sys::core::PCWSTR , pcfiles : *mut u32 , pphfilerecords : *mut *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchInfoA ( szpatch : :: windows_sys::core::PCSTR , szattribute : :: windows_sys::core::PCSTR , lpvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchInfoExA ( szpatchcode : :: windows_sys::core::PCSTR , szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , szproperty : :: windows_sys::core::PCSTR , lpvalue : :: windows_sys::core::PSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchInfoExW ( szpatchcode : :: windows_sys::core::PCWSTR , szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , szproperty : :: windows_sys::core::PCWSTR , lpvalue : :: windows_sys::core::PWSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPatchInfoW ( szpatch : :: windows_sys::core::PCWSTR , szattribute : :: windows_sys::core::PCWSTR , lpvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductCodeA ( szcomponent : :: windows_sys::core::PCSTR , lpbuf39 : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductCodeW ( szcomponent : :: windows_sys::core::PCWSTR , lpbuf39 : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoA ( szproduct : :: windows_sys::core::PCSTR , szattribute : :: windows_sys::core::PCSTR , lpvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoExA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , szproperty : :: windows_sys::core::PCSTR , szvalue : :: windows_sys::core::PSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoExW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , szproperty : :: windows_sys::core::PCWSTR , szvalue : :: windows_sys::core::PWSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoFromScriptA ( szscriptfile : :: windows_sys::core::PCSTR , lpproductbuf39 : :: windows_sys::core::PSTR , plgidlanguage : *mut u16 , pdwversion : *mut u32 , lpnamebuf : :: windows_sys::core::PSTR , pcchnamebuf : *mut u32 , lppackagebuf : :: windows_sys::core::PSTR , pcchpackagebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoFromScriptW ( szscriptfile : :: windows_sys::core::PCWSTR , lpproductbuf39 : :: windows_sys::core::PWSTR , plgidlanguage : *mut u16 , pdwversion : *mut u32 , lpnamebuf : :: windows_sys::core::PWSTR , pcchnamebuf : *mut u32 , lppackagebuf : :: windows_sys::core::PWSTR , pcchpackagebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductInfoW ( szproduct : :: windows_sys::core::PCWSTR , szattribute : :: windows_sys::core::PCWSTR , lpvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductPropertyA ( hproduct : MSIHANDLE , szproperty : :: windows_sys::core::PCSTR , lpvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetProductPropertyW ( hproduct : MSIHANDLE , szproperty : :: windows_sys::core::PCWSTR , lpvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPropertyA ( hinstall : MSIHANDLE , szname : :: windows_sys::core::PCSTR , szvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetPropertyW ( hinstall : MSIHANDLE , szname : :: windows_sys::core::PCWSTR , szvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetShortcutTargetA ( szshortcutpath : :: windows_sys::core::PCSTR , szproductcode : :: windows_sys::core::PSTR , szfeatureid : :: windows_sys::core::PSTR , szcomponentcode : :: windows_sys::core::PSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetShortcutTargetW ( szshortcutpath : :: windows_sys::core::PCWSTR , szproductcode : :: windows_sys::core::PWSTR , szfeatureid : :: windows_sys::core::PWSTR , szcomponentcode : :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetSourcePathA ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCSTR , szpathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetSourcePathW ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCWSTR , szpathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetSummaryInformationA ( hdatabase : MSIHANDLE , szdatabasepath : :: windows_sys::core::PCSTR , uiupdatecount : u32 , phsummaryinfo : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetSummaryInformationW ( hdatabase : MSIHANDLE , szdatabasepath : :: windows_sys::core::PCWSTR , uiupdatecount : u32 , phsummaryinfo : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetTargetPathA ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCSTR , szpathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetTargetPathW ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCWSTR , szpathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetUserInfoA ( szproduct : :: windows_sys::core::PCSTR , lpusernamebuf : :: windows_sys::core::PSTR , pcchusernamebuf : *mut u32 , lporgnamebuf : :: windows_sys::core::PSTR , pcchorgnamebuf : *mut u32 , lpserialbuf : :: windows_sys::core::PSTR , pcchserialbuf : *mut u32 ) -> USERINFOSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiGetUserInfoW ( szproduct : :: windows_sys::core::PCWSTR , lpusernamebuf : :: windows_sys::core::PWSTR , pcchusernamebuf : *mut u32 , lporgnamebuf : :: windows_sys::core::PWSTR , pcchorgnamebuf : *mut u32 , lpserialbuf : :: windows_sys::core::PWSTR , pcchserialbuf : *mut u32 ) -> USERINFOSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallMissingComponentA ( szproduct : :: windows_sys::core::PCSTR , szcomponent : :: windows_sys::core::PCSTR , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallMissingComponentW ( szproduct : :: windows_sys::core::PCWSTR , szcomponent : :: windows_sys::core::PCWSTR , einstallstate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallMissingFileA ( szproduct : :: windows_sys::core::PCSTR , szfile : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallMissingFileW ( szproduct : :: windows_sys::core::PCWSTR , szfile : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallProductA ( szpackagepath : :: windows_sys::core::PCSTR , szcommandline : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiInstallProductW ( szpackagepath : :: windows_sys::core::PCWSTR , szcommandline : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiIsProductElevatedA ( szproduct : :: windows_sys::core::PCSTR , pfelevated : *mut super::super::Foundation:: BOOL ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiIsProductElevatedW ( szproduct : :: windows_sys::core::PCWSTR , pfelevated : *mut super::super::Foundation:: BOOL ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiJoinTransaction ( htransactionhandle : MSIHANDLE , dwtransactionattributes : u32 , phchangeofownerevent : *mut super::super::Foundation:: HANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiLocateComponentA ( szcomponent : :: windows_sys::core::PCSTR , lppathbuf : :: windows_sys::core::PSTR , pcchbuf : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiLocateComponentW ( szcomponent : :: windows_sys::core::PCWSTR , lppathbuf : :: windows_sys::core::PWSTR , pcchbuf : *mut u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiNotifySidChangeA ( poldsid : :: windows_sys::core::PCSTR , pnewsid : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiNotifySidChangeW ( poldsid : :: windows_sys::core::PCWSTR , pnewsid : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenDatabaseA ( szdatabasepath : :: windows_sys::core::PCSTR , szpersist : :: windows_sys::core::PCSTR , phdatabase : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenDatabaseW ( szdatabasepath : :: windows_sys::core::PCWSTR , szpersist : :: windows_sys::core::PCWSTR , phdatabase : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenPackageA ( szpackagepath : :: windows_sys::core::PCSTR , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenPackageExA ( szpackagepath : :: windows_sys::core::PCSTR , dwoptions : u32 , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenPackageExW ( szpackagepath : :: windows_sys::core::PCWSTR , dwoptions : u32 , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenPackageW ( szpackagepath : :: windows_sys::core::PCWSTR , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenProductA ( szproduct : :: windows_sys::core::PCSTR , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiOpenProductW ( szproduct : :: windows_sys::core::PCWSTR , hproduct : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiPreviewBillboardA ( hpreview : MSIHANDLE , szcontrolname : :: windows_sys::core::PCSTR , szbillboard : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiPreviewBillboardW ( hpreview : MSIHANDLE , szcontrolname : :: windows_sys::core::PCWSTR , szbillboard : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiPreviewDialogA ( hpreview : MSIHANDLE , szdialogname : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiPreviewDialogW ( hpreview : MSIHANDLE , szdialogname : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn MsiProcessAdvertiseScriptA ( szscriptfile : :: windows_sys::core::PCSTR , sziconfolder : :: windows_sys::core::PCSTR , hregdata : super::Registry:: HKEY , fshortcuts : super::super::Foundation:: BOOL , fremoveitems : super::super::Foundation:: BOOL ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn MsiProcessAdvertiseScriptW ( szscriptfile : :: windows_sys::core::PCWSTR , sziconfolder : :: windows_sys::core::PCWSTR , hregdata : super::Registry:: HKEY , fshortcuts : super::super::Foundation:: BOOL , fremoveitems : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProcessMessage ( hinstall : MSIHANDLE , emessagetype : INSTALLMESSAGE , hrecord : MSIHANDLE ) -> i32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideAssemblyA ( szassemblyname : :: windows_sys::core::PCSTR , szappcontext : :: windows_sys::core::PCSTR , dwinstallmode : INSTALLMODE , dwassemblyinfo : MSIASSEMBLYINFO , lppathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideAssemblyW ( szassemblyname : :: windows_sys::core::PCWSTR , szappcontext : :: windows_sys::core::PCWSTR , dwinstallmode : INSTALLMODE , dwassemblyinfo : MSIASSEMBLYINFO , lppathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideComponentA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR , szcomponent : :: windows_sys::core::PCSTR , dwinstallmode : INSTALLMODE , lppathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideComponentW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR , szcomponent : :: windows_sys::core::PCWSTR , dwinstallmode : INSTALLMODE , lppathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideQualifiedComponentA ( szcategory : :: windows_sys::core::PCSTR , szqualifier : :: windows_sys::core::PCSTR , dwinstallmode : INSTALLMODE , lppathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideQualifiedComponentExA ( szcategory : :: windows_sys::core::PCSTR , szqualifier : :: windows_sys::core::PCSTR , dwinstallmode : INSTALLMODE , szproduct : :: windows_sys::core::PCSTR , dwunused1 : u32 , dwunused2 : u32 , lppathbuf : :: windows_sys::core::PSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideQualifiedComponentExW ( szcategory : :: windows_sys::core::PCWSTR , szqualifier : :: windows_sys::core::PCWSTR , dwinstallmode : INSTALLMODE , szproduct : :: windows_sys::core::PCWSTR , dwunused1 : u32 , dwunused2 : u32 , lppathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiProvideQualifiedComponentW ( szcategory : :: windows_sys::core::PCWSTR , szqualifier : :: windows_sys::core::PCWSTR , dwinstallmode : INSTALLMODE , lppathbuf : :: windows_sys::core::PWSTR , pcchpathbuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryComponentStateA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , szcomponentcode : :: windows_sys::core::PCSTR , pdwstate : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryComponentStateW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , szcomponentcode : :: windows_sys::core::PCWSTR , pdwstate : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryFeatureStateA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryFeatureStateExA ( szproductcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , szfeature : :: windows_sys::core::PCSTR , pdwstate : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryFeatureStateExW ( szproductcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , szfeature : :: windows_sys::core::PCWSTR , pdwstate : *mut INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryFeatureStateW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryProductStateA ( szproduct : :: windows_sys::core::PCSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiQueryProductStateW ( szproduct : :: windows_sys::core::PCWSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordClearData ( hrecord : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordDataSize ( hrecord : MSIHANDLE , ifield : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordGetFieldCount ( hrecord : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordGetInteger ( hrecord : MSIHANDLE , ifield : u32 ) -> i32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordGetStringA ( hrecord : MSIHANDLE , ifield : u32 , szvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordGetStringW ( hrecord : MSIHANDLE , ifield : u32 , szvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiRecordIsNull ( hrecord : MSIHANDLE , ifield : u32 ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordReadStream ( hrecord : MSIHANDLE , ifield : u32 , szdatabuf : :: windows_sys::core::PSTR , pcbdatabuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordSetInteger ( hrecord : MSIHANDLE , ifield : u32 , ivalue : i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordSetStreamA ( hrecord : MSIHANDLE , ifield : u32 , szfilepath : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordSetStreamW ( hrecord : MSIHANDLE , ifield : u32 , szfilepath : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordSetStringA ( hrecord : MSIHANDLE , ifield : u32 , szvalue : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRecordSetStringW ( hrecord : MSIHANDLE , ifield : u32 , szvalue : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiReinstallFeatureA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR , dwreinstallmode : REINSTALLMODE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiReinstallFeatureW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR , dwreinstallmode : REINSTALLMODE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiReinstallProductA ( szproduct : :: windows_sys::core::PCSTR , szreinstallmode : REINSTALLMODE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiReinstallProductW ( szproduct : :: windows_sys::core::PCWSTR , szreinstallmode : REINSTALLMODE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRemovePatchesA ( szpatchlist : :: windows_sys::core::PCSTR , szproductcode : :: windows_sys::core::PCSTR , euninstalltype : INSTALLTYPE , szpropertylist : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiRemovePatchesW ( szpatchlist : :: windows_sys::core::PCWSTR , szproductcode : :: windows_sys::core::PCWSTR , euninstalltype : INSTALLTYPE , szpropertylist : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSequenceA ( hinstall : MSIHANDLE , sztable : :: windows_sys::core::PCSTR , isequencemode : i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSequenceW ( hinstall : MSIHANDLE , sztable : :: windows_sys::core::PCWSTR , isequencemode : i32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetComponentStateA ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCSTR , istate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetComponentStateW ( hinstall : MSIHANDLE , szcomponent : :: windows_sys::core::PCWSTR , istate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetExternalUIA ( puihandler : INSTALLUI_HANDLERA , dwmessagefilter : u32 , pvcontext : *const ::core::ffi::c_void ) -> INSTALLUI_HANDLERA );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetExternalUIRecord ( puihandler : PINSTALLUI_HANDLER_RECORD , dwmessagefilter : u32 , pvcontext : *const ::core::ffi::c_void , ppuiprevhandler : PINSTALLUI_HANDLER_RECORD ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetExternalUIW ( puihandler : INSTALLUI_HANDLERW , dwmessagefilter : u32 , pvcontext : *const ::core::ffi::c_void ) -> INSTALLUI_HANDLERW );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetFeatureAttributesA ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , dwattributes : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetFeatureAttributesW ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , dwattributes : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetFeatureStateA ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCSTR , istate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetFeatureStateW ( hinstall : MSIHANDLE , szfeature : :: windows_sys::core::PCWSTR , istate : INSTALLSTATE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetInstallLevel ( hinstall : MSIHANDLE , iinstalllevel : i32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSetInternalUI ( dwuilevel : INSTALLUILEVEL , phwnd : *mut super::super::Foundation:: HWND ) -> INSTALLUILEVEL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSetMode ( hinstall : MSIHANDLE , erunmode : MSIRUNMODE , fstate : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetPropertyA ( hinstall : MSIHANDLE , szname : :: windows_sys::core::PCSTR , szvalue : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetPropertyW ( hinstall : MSIHANDLE , szname : :: windows_sys::core::PCWSTR , szvalue : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetTargetPathA ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCSTR , szfolderpath : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSetTargetPathW ( hinstall : MSIHANDLE , szfolder : :: windows_sys::core::PCWSTR , szfolderpath : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddMediaDiskA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwdiskid : u32 , szvolumelabel : :: windows_sys::core::PCSTR , szdiskprompt : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddMediaDiskW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwdiskid : u32 , szvolumelabel : :: windows_sys::core::PCWSTR , szdiskprompt : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddSourceA ( szproduct : :: windows_sys::core::PCSTR , szusername : :: windows_sys::core::PCSTR , dwreserved : u32 , szsource : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddSourceExA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szsource : :: windows_sys::core::PCSTR , dwindex : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddSourceExW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szsource : :: windows_sys::core::PCWSTR , dwindex : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListAddSourceW ( szproduct : :: windows_sys::core::PCWSTR , szusername : :: windows_sys::core::PCWSTR , dwreserved : u32 , szsource : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearAllA ( szproduct : :: windows_sys::core::PCSTR , szusername : :: windows_sys::core::PCSTR , dwreserved : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearAllExA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearAllExW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearAllW ( szproduct : :: windows_sys::core::PCWSTR , szusername : :: windows_sys::core::PCWSTR , dwreserved : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearMediaDiskA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwdiskid : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearMediaDiskW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwdiskid : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearSourceA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szsource : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListClearSourceW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szsource : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListEnumMediaDisksA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwindex : u32 , pdwdiskid : *mut u32 , szvolumelabel : :: windows_sys::core::PSTR , pcchvolumelabel : *mut u32 , szdiskprompt : :: windows_sys::core::PSTR , pcchdiskprompt : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListEnumMediaDisksW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwindex : u32 , pdwdiskid : *mut u32 , szvolumelabel : :: windows_sys::core::PWSTR , pcchvolumelabel : *mut u32 , szdiskprompt : :: windows_sys::core::PWSTR , pcchdiskprompt : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListEnumSourcesA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwindex : u32 , szsource : :: windows_sys::core::PSTR , pcchsource : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListEnumSourcesW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , dwindex : u32 , szsource : :: windows_sys::core::PWSTR , pcchsource : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListForceResolutionA ( szproduct : :: windows_sys::core::PCSTR , szusername : :: windows_sys::core::PCSTR , dwreserved : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListForceResolutionExA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListForceResolutionExW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListForceResolutionW ( szproduct : :: windows_sys::core::PCWSTR , szusername : :: windows_sys::core::PCWSTR , dwreserved : u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListGetInfoA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szproperty : :: windows_sys::core::PCSTR , szvalue : :: windows_sys::core::PSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListGetInfoW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szproperty : :: windows_sys::core::PCWSTR , szvalue : :: windows_sys::core::PWSTR , pcchvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListSetInfoA ( szproductcodeorpatchcode : :: windows_sys::core::PCSTR , szusersid : :: windows_sys::core::PCSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szproperty : :: windows_sys::core::PCSTR , szvalue : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSourceListSetInfoW ( szproductcodeorpatchcode : :: windows_sys::core::PCWSTR , szusersid : :: windows_sys::core::PCWSTR , dwcontext : MSIINSTALLCONTEXT , dwoptions : u32 , szproperty : :: windows_sys::core::PCWSTR , szvalue : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSummaryInfoGetPropertyA ( hsummaryinfo : MSIHANDLE , uiproperty : u32 , puidatatype : *mut u32 , pivalue : *mut i32 , pftvalue : *mut super::super::Foundation:: FILETIME , szvaluebuf : :: windows_sys::core::PSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSummaryInfoGetPropertyCount ( hsummaryinfo : MSIHANDLE , puipropertycount : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSummaryInfoGetPropertyW ( hsummaryinfo : MSIHANDLE , uiproperty : u32 , puidatatype : *mut u32 , pivalue : *mut i32 , pftvalue : *mut super::super::Foundation:: FILETIME , szvaluebuf : :: windows_sys::core::PWSTR , pcchvaluebuf : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiSummaryInfoPersist ( hsummaryinfo : MSIHANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSummaryInfoSetPropertyA ( hsummaryinfo : MSIHANDLE , uiproperty : u32 , uidatatype : u32 , ivalue : i32 , pftvalue : *mut super::super::Foundation:: FILETIME , szvalue : :: windows_sys::core::PCSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn MsiSummaryInfoSetPropertyW ( hsummaryinfo : MSIHANDLE , uiproperty : u32 , uidatatype : u32 , ivalue : i32 , pftvalue : *mut super::super::Foundation:: FILETIME , szvalue : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiUseFeatureA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiUseFeatureExA ( szproduct : :: windows_sys::core::PCSTR , szfeature : :: windows_sys::core::PCSTR , dwinstallmode : u32 , dwreserved : u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiUseFeatureExW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR , dwinstallmode : u32 , dwreserved : u32 ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiUseFeatureW ( szproduct : :: windows_sys::core::PCWSTR , szfeature : :: windows_sys::core::PCWSTR ) -> INSTALLSTATE );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiVerifyDiskSpace ( hinstall : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiVerifyPackageA ( szpackagepath : :: windows_sys::core::PCSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiVerifyPackageW ( szpackagepath : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewClose ( hview : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewExecute ( hview : MSIHANDLE , hrecord : MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewFetch ( hview : MSIHANDLE , phrecord : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewGetColumnInfo ( hview : MSIHANDLE , ecolumninfo : MSICOLINFO , phrecord : *mut MSIHANDLE ) -> u32 );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewGetErrorA ( hview : MSIHANDLE , szcolumnnamebuffer : :: windows_sys::core::PSTR , pcchbuf : *mut u32 ) -> MSIDBERROR );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewGetErrorW ( hview : MSIHANDLE , szcolumnnamebuffer : :: windows_sys::core::PWSTR , pcchbuf : *mut u32 ) -> MSIDBERROR );
::windows_sys::core::link ! ( "msi.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"] fn MsiViewModify ( hview : MSIHANDLE , emodifymode : MSIMODIFY , hrecord : MSIHANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn NormalizeFileForPatchSignature ( filebuffer : *mut ::core::ffi::c_void , filesize : u32 , optionflags : u32 , optiondata : *const PATCH_OPTION_DATA , newfilecoffbase : u32 , newfilecofftime : u32 , ignorerangecount : u32 , ignorerangearray : *const PATCH_IGNORE_RANGE , retainrangecount : u32 , retainrangearray : *const PATCH_RETAIN_RANGE ) -> i32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn QueryActCtxSettingsW ( dwflags : u32 , hactctx : super::super::Foundation:: HANDLE , settingsnamespace : :: windows_sys::core::PCWSTR , settingname : :: windows_sys::core::PCWSTR , pvbuffer : :: windows_sys::core::PWSTR , dwbuffer : usize , pdwwrittenorrequired : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn QueryActCtxW ( dwflags : u32 , hactctx : super::super::Foundation:: HANDLE , pvsubinstance : *const ::core::ffi::c_void , ulinfoclass : u32 , pvbuffer : *mut ::core::ffi::c_void , cbbuffer : usize , pcbwrittenorrequired : *mut usize ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ReleaseActCtx ( hactctx : super::super::Foundation:: HANDLE ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "sfc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn SfcGetNextProtectedFile ( rpchandle : super::super::Foundation:: HANDLE , protfiledata : *mut PROTECTED_FILE_DATA ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "sfc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn SfcIsFileProtected ( rpchandle : super::super::Foundation:: HANDLE , protfilename : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "sfc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn SfcIsKeyProtected ( keyhandle : super::Registry:: HKEY , subkeyname : :: windows_sys::core::PCWSTR , keysam : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "sfc.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn SfpVerifyFile ( pszfilename : :: windows_sys::core::PCSTR , pszerror : :: windows_sys::core::PCSTR , dwerrsize : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn TestApplyPatchToFileA ( patchfilename : :: windows_sys::core::PCSTR , oldfilename : :: windows_sys::core::PCSTR , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn TestApplyPatchToFileByBuffers ( patchfilebuffer : *const u8 , patchfilesize : u32 , oldfilebuffer : *const u8 , oldfilesize : u32 , newfilesize : *mut u32 , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn TestApplyPatchToFileByHandles ( patchfilehandle : super::super::Foundation:: HANDLE , oldfilehandle : super::super::Foundation:: HANDLE , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "mspatcha.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn TestApplyPatchToFileW ( patchfilename : :: windows_sys::core::PCWSTR , oldfilename : :: windows_sys::core::PCWSTR , applyoptionflags : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "kernel32.dll""system" #[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"] fn ZombifyActCtx ( hactctx : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
pub type IAssemblyCache = *mut ::core::ffi::c_void;
pub type IAssemblyCacheItem = *mut ::core::ffi::c_void;
pub type IAssemblyName = *mut ::core::ffi::c_void;
pub type IEnumMsmDependency = *mut ::core::ffi::c_void;
pub type IEnumMsmError = *mut ::core::ffi::c_void;
pub type IEnumMsmString = *mut ::core::ffi::c_void;
pub type IMsmDependencies = *mut ::core::ffi::c_void;
pub type IMsmDependency = *mut ::core::ffi::c_void;
pub type IMsmError = *mut ::core::ffi::c_void;
pub type IMsmErrors = *mut ::core::ffi::c_void;
pub type IMsmGetFiles = *mut ::core::ffi::c_void;
pub type IMsmMerge = *mut ::core::ffi::c_void;
pub type IMsmStrings = *mut ::core::ffi::c_void;
pub type IPMApplicationInfo = *mut ::core::ffi::c_void;
pub type IPMApplicationInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMDeploymentManager = *mut ::core::ffi::c_void;
pub type IPMEnumerationManager = *mut ::core::ffi::c_void;
pub type IPMExtensionCachedFileUpdaterInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionContractInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileOpenPickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileSavePickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMExtensionProtocolInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionShareTargetInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTaskInfo = *mut ::core::ffi::c_void;
pub type IPMTaskInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTileInfo = *mut ::core::ffi::c_void;
pub type IPMTileInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyInfo = *mut ::core::ffi::c_void;
pub type IValidate = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CLSID_EvalCom2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6e5e1910_8053_4660_b795_6b612e29bc58);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CLSID_MsmMerge2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf94985d5_29f9_4743_9805_99bc3f35b678);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_DISK_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb02f9d65_fb77_4f7a_afa5_b391309f11c9);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2ec93463_b0c3_45e1_8364_327e96aea856);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cedc215_ac4b_488b_93c0_a50a49cb2fb8);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADMIN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADMIN");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADVERTISE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADVERTISE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_COLLECTUSERINFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CollectUserInfo");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_FIRSTRUN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FirstRun");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_INSTALL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("INSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_SEQUENCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SEQUENCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_BASE: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PCP_PATH: u32 = 3222229252u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PROPERTY: u32 = 3222229255u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AssignmentType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AuthorizedLUAApp");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISKPROMPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskPrompt");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISPLAYNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPLINK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HelpLink");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPTELEPHONE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HelpTelephone");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLDATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstallDate");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstalledLanguage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstalledProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLLOCATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstallLocation");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstallSource");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTANCETYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InstanceType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LANGUAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Language");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LastUsedSource");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LastUsedType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LOCALPACKAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LocalPackage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LUAENABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LUAEnabled");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MediaPackagePath");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MOREINFOURL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MoreInfoURL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGECODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PackageCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGENAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PackageName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHSTATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("State");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PatchType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTICON: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductIcon");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTSTATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("State");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PUBLISHER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Publisher");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGCOMPANY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RegCompany");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGOWNER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RegOwner");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_TRANSFORMS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Transforms");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_UNINSTALLABLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Uninstallable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLINFOABOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("URLInfoAbout");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLUPDATEINFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("URLUpdateInfo");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Version");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMAJOR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VersionMajor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMINOR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VersionMinor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONSTRING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VersionString");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINTOOLS_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdminToolsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINUSER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdminUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMIN_PROPERTIES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdminProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AFTERREBOOT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AFTERREBOOT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLOWEDPROPERTIES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SecureCustomProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLUSERS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ALLUSERS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_APPDATA_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Arm");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Arm64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPAUTHORIZEDCDFPREFIX: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPAUTHORIZEDCDFPREFIX");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCOMMENTS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPCOMMENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCONTACT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPCONTACT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPLINK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPHELPLINK");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPTELEPHONE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPHELPTELEPHONE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPINSTALLLOCATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPINSTALLLOCATION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOMODIFY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPNOMODIFY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREMOVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPNOREMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREPAIR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPNOREPAIR");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPPRODUCTICON: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPPRODUCTICON");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPREADME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPREADME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSETTINGSIDENTIFIER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIARPSETTINGSIDENTIFIER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMFLAGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SHIMFLAGS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMSERVICEPACKLEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SHIMSERVICEPACKLEVEL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMVERSIONNT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SHIMVERSIONNT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSIZE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPSIZE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSYSTEMCOMPONENT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPSYSTEMCOMPONENT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLINFOABOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPURLINFOABOUT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLUPDATEINFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ARPURLUPDATEINFO");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AVAILABLEFREEREG: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AVAILABLEFREEREG");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERSIDE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BorderSide");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERTOP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BorderTop");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CAPTIONHEIGHT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CaptionHeight");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CARRYINGNDP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CARRYINGNDP");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CHECKCRCS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSICHECKCRCS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COLORBITS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ColorBits");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONAPPDATA_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CommonAppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES64_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CommonFiles64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CommonFilesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPANYNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("COMPANYNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDDEFAULT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("COMPADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDLOCAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("COMPADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("COMPADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPUTERNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COSTINGCOMPLETE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CostingComplete");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CUSTOMACTIONDATA: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CustomActionData");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Date");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATETIME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DateTime");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DEFAULTUIFONT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DefaultUIFont");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DESKTOP_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DesktopFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEADVTSHORTCUTS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DISABLEADVTSHORTCUTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEROLLBACK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DISABLEROLLBACK");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISKPROMPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskPrompt");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENABLEUSERCONTROL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnableUserControl");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENFORCE_UPGRADE_COMPONENT_RULES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIENFORCEUPGRADECOMPONENTRULES");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EXECUTEACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEMODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EXECUTEMODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FAVORITES_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FavoritesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDDEFAULT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDLOCAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADVERTISE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADVERTISE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREREMOVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("REMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDDEFAULT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FILEADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDLOCAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FILEADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FILEADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FONTS_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FontsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDDEN_PROPERTIES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiHiddenProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDECANCEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiUIHideCancel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_IA64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IA64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Installed");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLANGUAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductLanguage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("INSTALLLEVEL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLPERUSER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIINSTALLPERUSER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Intel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Intel64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTERNALINSTALLEDPERUSER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIINTERNALINSTALLEDPERUSER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ISADMINPACKAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsAdminPackage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LEFTUNIT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LeftUnit");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LIMITUI: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LIMITUI");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOCALAPPDATA_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LocalAppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LOGACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGONUSER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LogonUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MANUFACTURER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIAMD64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiAMD64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLEEEUI: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIDISABLEEEUI");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLELUAPATCHING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIDISABLELUAPATCHING");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIINSTANCEGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIINSTANCEGUID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGFILELOCATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiLogFileLocation");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGGINGMODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiLogging");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINEWINSTANCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSINEWINSTANCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINODISABLEMEDIA: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSINODISABLEMEDIA");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPACKAGEDOWNLOADLOCALCOPY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIPACKAGEDOWNLOADLOCALCOPY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHDOWNLOADLOCALCOPY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIPATCHDOWNLOADLOCALCOPY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHREMOVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIPATCHREMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSITABLETPC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiTabletPC");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIX64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Msix64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_FASTINSTALL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIFASTINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_REBOOT_PENDING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiSystemRebootPending");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_CONTROL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIRESTARTMANAGERCONTROL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_DISABLE_RESTART: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIDISABLERMRESTART");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SESSION_KEY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiRestartManagerSessionKey");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SHUTDOWN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIRMSHUTDOWN");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UAC_DEPLOYMENT_COMPLIANT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIDEPLOYMENTCOMPLIANT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UNINSTALL_SUPERSEDED_COMPONENTS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIUNINSTALLSUPERSEDEDCOMPONENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_USE_REAL_ADMIN_DETECTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSIUSEREALADMINDETECTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MYPICTURES_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MyPicturesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETASSEMBLYSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNetAssemblySupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETHOOD_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NetHoodFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOCOMPANYNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NOCOMPANYNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOUSERNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NOUSERNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTPRODUCTTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTProductType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEBACKOFFICE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteBackOffice");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEDATACENTER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteDataCenter");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEENTERPRISE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteEnterprise");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEPERSONAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuitePersonal");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteSmallBusiness");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESSRESTRICTED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteSmallBusinessRestricted");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEWEBSERVER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiNTSuiteWebServer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OLEADVTSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("OLEAdvtSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFDISKSPACE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("OutOfDiskSpace");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFNORBDISKSPACE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("OutOfNoRbDiskSpace");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PATCH");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWPACKAGECODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PATCHNEWPACKAGECODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYCOMMENTS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PATCHNEWSUMMARYCOMMENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYSUBJECT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PATCHNEWSUMMARYSUBJECT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PERSONAL_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PersonalFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PHYSICALMEMORY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PhysicalMemory");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDKEY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PIDKEY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDTEMPLATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PIDTemplate");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRESELECTED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Preselected");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PRIMARYFOLDER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryVolumePath");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEAVAILABLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryVolumeSpaceAvailable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREMAINING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryVolumeSpaceRemaining");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREQUIRED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryVolumeSpaceRequired");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRINTHOOD_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrintHoodFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIVILEGED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Privileged");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTCODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTLANGUAGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PRODUCTLANGUAGE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTSTATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductState");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTVERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProductVersion");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES64_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProgramFiles64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProgramFilesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMMENU_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProgramMenuFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRESSONLY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiUIProgressOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROMPTROLLBACKCOST: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PROMPTROLLBACKCOST");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("REBOOT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOTPROMPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("REBOOTPROMPT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RECENT_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RecentFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REDIRECTEDDLLSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RedirectedDllSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("REINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALLMODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("REINSTALLMODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REMOTEADMINTS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RemoteAdminTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REPLACEDINUSEFILES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ReplacedInUseFiles");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESTRICTEDUSERCONTROL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RestrictedUserControl");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESUME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RESUME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROLLBACKDISABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RollbackDisabled");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROOTDRIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ROOTDRIVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RUNNINGELEVATED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiRunningElevated");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENX: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ScreenX");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ScreenY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SENDTO_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SendToFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SEQUENCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SEQUENCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ServicePackLevel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVELMINOR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ServicePackLevelMinor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHAREDWINDOWS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedWindows");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHELLADVTSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ShellAdvtSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHORTFILENAMES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SHORTFILENAMES");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCEDIR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SourceDir");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCELIST: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SOURCELIST");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCERESONLY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiUISourceResOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTMENU_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartMenuFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTUP_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartupFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM16_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("System16Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM64_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("System64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEMLANGUAGEID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SystemLanguageID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SystemFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TARGETDIR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TARGETDIR");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_AMD64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AMD64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TemplateFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_X64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("x64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMP_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TempFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TERMSERVER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TerminalServer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TextHeight");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT_CORRECTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TextHeightCorrection");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTINTERNALLEADING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TextInternalLeading");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TIME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Time");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TRANSFORMS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSATSOURCE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TRANSFORMSATSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSSECURE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TRANSFORMSSECURE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRUEADMINUSER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiTrueAdminUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TTCSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TTCSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UACONLY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiUIUACOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPDATESTARTED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UpdateStarted");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPGRADECODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UpgradeCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERLANGUAGEID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UserLanguageID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("USERNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERSID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UserSID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSION9X: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Version9X");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VersionNT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT64: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VersionNT64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VIRTUALMEMORY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualMemory");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WIN32ASSEMBLYSUPPORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MsiWin32AssemblySupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWSBUILD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WindowsBuild");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_FOLDER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WindowsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_VOLUME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WindowsVolume");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_NONE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NONE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_SCRIPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SCRIPT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_FEATURE_ALL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Disable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLESHUTDOWN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DisableShutdown");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_FAIL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("F");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_PROMPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("P");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_SILENT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("D");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTREINSTALL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("URTREINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTUPGRADE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("URTUPGRADE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LIBID_MsmMergeTypeLib: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0adda82f_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGERR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGNONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGPERFMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGWARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_FEATURE_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_GUID_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MsmMerge: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0adda830_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_APPNAME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_AUTHOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CHARCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_COMMENTS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CREATE_DTM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_EDITTIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_KEYWORDS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTAUTHOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTPRINTED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTSAVE_DTM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIRESTRICT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSISOURCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIVERSION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_PAGECOUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_REVNUMBER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_SUBJECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TEMPLATE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_THUMBNAIL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_WORDCOUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PMSvc: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb9e511fc_e364_497a_a121_b7b3612cedce);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_SETUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_IDLE_TRIGGER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WFP_IDLE_TRIGGER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_BACKUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_CMI: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_COPYFILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DETAILS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVINST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FILEQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INFDB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_POLICY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SIGVERIF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SUMMARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TAB_1: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UI: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UTIL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERBOSE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UIALL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UILOGBITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UINONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BASE: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSI: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const cchMaxInteger: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ACTCTX_COMPATIBILITY_ELEMENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ACTCTX_REQUESTED_RUN_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ADVERTISEFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ASM_BIND_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ASM_CMP_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = 255i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ASM_DISPLAY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type ASM_NAME = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_VALUE: ASM_NAME = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NAME: ASM_NAME = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CULTURE: ASM_NAME = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_ALGID: ASM_NAME = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_ALIAS: ASM_NAME = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = 13i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = 14i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CUSTOM: ASM_NAME = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = 18i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MVID: ASM_NAME = 19i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = 20i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type CREATE_ASM_NAME_OBJ_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type IASSEMBLYCACHE_UNINSTALL_DISPOSITION = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLFEATUREATTRIBUTE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = 65535i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLLOGATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLLOGMODE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FATALEXIT: INSTALLLOGMODE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ERROR: INSTALLLOGMODE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_WARNING: INSTALLLOGMODE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_USER: INSTALLLOGMODE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INFO: INSTALLLOGMODE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLLOGMODE = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLLOGMODE = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLLOGMODE = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLLOGMODE = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_COMMONDATA: INSTALLLOGMODE = 2048i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLLOGMODE = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_VERBOSE: INSTALLLOGMODE = 4096i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLLOGMODE = 8192i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLLOGMODE = 16384i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLLOGMODE = 32768i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROGRESS: INSTALLLOGMODE = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INITIALIZE: INSTALLLOGMODE = 4096i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_TERMINATE: INSTALLLOGMODE = 8192i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLLOGMODE = 16384i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FILESINUSE: INSTALLLOGMODE = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLLOGMODE = 33554432i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLLOGMODE = 67108864i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLEND: INSTALLLOGMODE = 134217728i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLMESSAGE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = 16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = 33554432i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = 50331648i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = 67108864i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = 83886080i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = 100663296i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = 117440512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = 134217728i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = 150994944i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = 167772160i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = 184549376i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = 201326592i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = 218103808i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = 234881024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = 251658240i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = 419430400i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = 436207616i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = 452984832i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLMODE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = -4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = -3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION: INSTALLMODE = -2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_EXISTING: INSTALLMODE = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_DEFAULT: INSTALLMODE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLSTATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = -7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = -6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = -5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = -4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = -3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = -2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUILEVEL = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIADVERTISEOPTIONFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIARCHITECTUREFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIASSEMBLYINFO = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSICODE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PRODUCT: MSICODE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PATCH: MSICODE = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSICOLINFO = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_NAMES: MSICOLINFO = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_TYPES: MSICOLINFO = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSICONDITION = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_FALSE: MSICONDITION = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_TRUE: MSICONDITION = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_NONE: MSICONDITION = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_ERROR: MSICONDITION = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSICOSTTREE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIDBERROR = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = -3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MOREDATA: MSIDBERROR = -2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOERROR: MSIDBERROR = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_REQUIRED: MSIDBERROR = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLINK: MSIDBERROR = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOTINSET: MSIDBERROR = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADVERSION: MSIDBERROR = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCASE: MSIDBERROR = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADGUID: MSIDBERROR = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = 13i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPATH: MSIDBERROR = 14i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = 18i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = 19i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = 20i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = 21i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = 22i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = 23i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = 24i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = 25i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCABINET: MSIDBERROR = 26i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = 27i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = 28i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = 29i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIDBSTATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_ERROR: MSIDBSTATE = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_READ: MSIDBSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_WRITE: MSIDBSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIINSTALLCONTEXT = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIMODIFY = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_SEEK: MSIMODIFY = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REFRESH: MSIMODIFY = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT: MSIMODIFY = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_UPDATE: MSIMODIFY = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_ASSIGN: MSIMODIFY = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REPLACE: MSIMODIFY = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_MERGE: MSIMODIFY = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_DELETE: MSIMODIFY = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE: MSIMODIFY = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIOPENPACKAGEFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIPATCHDATATYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIPATCHSTATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSIRUNMODE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_CABINET: MSIRUNMODE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = 13i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = 14i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = 18i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSISOURCETYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_URL: MSISOURCETYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSITRANSACTION = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSITRANSACTIONSTATE = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSITRANSFORM_ERROR = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type MSITRANSFORM_VALIDATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = 2048i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PACKMAN_RUNTIME = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ACTIVATION_POLICY = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_APPLICATION_HUBTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_APPLICATION_INSTALL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_APPLICATION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_APP_GENRE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_APP_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_BSA_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = 26i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = 27i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = 28i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = 29i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = 30i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = 31i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_BW_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = 31i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = 33i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_EXTENSION_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = 18i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = 19i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = 20i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = 21i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = 22i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 23i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 24i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = 25i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = 26i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_TASK_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = 13i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = 14i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_ENUM_TILE_FILTER = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_LIVETILE_RECURRENCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_LOGO_SIZE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_STARTTILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_TASK_TRANSITION = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_TASK_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_TILE_HUBTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = 268435456i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = 536870912i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = 16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = 33554432i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = 67108864i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = 67108865i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PM_TILE_SIZE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type QUERYASMINFO_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type REINSTALLMODE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type RESULTTYPES = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieUnknown: RESULTTYPES = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieError: RESULTTYPES = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieWarning: RESULTTYPES = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieInfo: RESULTTYPES = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type SCRIPTFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = 384i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = 416i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type STATUSTYPES = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusGetCUB: STATUSTYPES = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusICECount: STATUSTYPES = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusMerge: STATUSTYPES = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSummaryInfo: STATUSTYPES = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCreateEngine: STATUSTYPES = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusStarting: STATUSTYPES = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusRunICE: STATUSTYPES = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusShutdown: STATUSTYPES = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSuccess: STATUSTYPES = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusFail: STATUSTYPES = 9i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCancel: STATUSTYPES = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type TILE_TEMPLATE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = 39i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = 40i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = 41i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = 42i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = 43i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = 44i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = 45i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = 46i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = 47i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = 48i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = 49i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = 50i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 51i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 52i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 53i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 54i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = 55i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = 56i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = 57i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = 58i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = 59i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type USERINFOSTATE = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = -3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = -2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = -1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbAssemblyAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbClassAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbClassAttributesRelativePath: msidbClassAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbComponentAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesOptional: msidbComponentAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributes64bit: msidbComponentAttributes = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesShared: msidbComponentAttributes = 2048i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbControlAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesVisible: msidbControlAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesEnabled: msidbControlAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSunken: msidbControlAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIndirect: msidbControlAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesInteger: msidbControlAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRTLRO: msidbControlAttributes = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRightAligned: msidbControlAttributes = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBiDi: msidbControlAttributes = 224i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesTransparent: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = 131072i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoWrap: msidbControlAttributes = 262144i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFormatSize: msidbControlAttributes = 524288i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesMultiline: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesProgress95: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = 131072i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = 262144i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = 524288i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlShowRollbackCost: msidbControlAttributes = 4194304i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSorted: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesComboList: msidbControlAttributes = 131072i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesImageHandle: msidbControlAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPushLike: msidbControlAttributes = 131072i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBitmap: msidbControlAttributes = 262144i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIcon: msidbControlAttributes = 524288i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedSize: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize16: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize32: msidbControlAttributes = 4194304i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize48: msidbControlAttributes = 6291456i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesElevationShield: msidbControlAttributes = 8388608i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesHasBorder: msidbControlAttributes = 16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbCustomActionType = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDll: msidbCustomActionType = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeExe: msidbCustomActionType = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTextData: msidbCustomActionType = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeJScript: msidbCustomActionType = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInstall: msidbCustomActionType = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeProperty: msidbCustomActionType = 48i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeContinue: msidbCustomActionType = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeAsync: msidbCustomActionType = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = 768i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInScript: msidbCustomActionType = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeRollback: msidbCustomActionType = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeCommit: msidbCustomActionType = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = 2048i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = 16384i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionType64BitScript: msidbCustomActionType = 4096i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = 8192i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = 32768i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbDialogAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesVisible: msidbDialogAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesModal: msidbDialogAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = 896i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesError: msidbDialogAttributes = 65536i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbEmbeddedUIAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbFeatureAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbFileAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReadOnly: msidbFileAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesHidden: msidbFileAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesSystem: msidbFileAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved0: msidbFileAttributes = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved1: msidbFileAttributes = 64i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved2: msidbFileAttributes = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved3: msidbFileAttributes = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesVital: msidbFileAttributes = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesChecksum: msidbFileAttributes = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = 4096i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = 8192i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesCompressed: msidbFileAttributes = 16384i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved4: msidbFileAttributes = 32768i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbIniFileAction = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddLine: msidbIniFileAction = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionCreateLine: msidbIniFileAction = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddTag: msidbIniFileAction = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbLocatorType = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeDirectory: msidbLocatorType = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeFileName: msidbLocatorType = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeRawValue: msidbLocatorType = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorType64bit: msidbLocatorType = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbMoveFileOptions = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbODBCDataSourceRegistration = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbPatchAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbRegistryRoot = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootUsers: msidbRegistryRoot = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbRemoveFileInstallMode = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbServiceConfigEvent = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbServiceControlEvent = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStart: msidbServiceControlEvent = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStop: msidbServiceControlEvent = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = 32i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = 128i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbServiceInstallErrorControl = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = 32768i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbSumInfoSourceType = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbTextStyleStyleBits = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = 8i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msidbUpgradeAttributes = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = 256i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = 512i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = 1024i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msifiFastInstallBits = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallNoSR: msifiFastInstallBits = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msirbRebootReason = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootUndeterminedReason: msirbRebootReason = 0i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootInUseFilesReason: msirbRebootReason = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootScheduleRebootReason: msirbRebootReason = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootForceRebootReason: msirbRebootReason = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootCustomActionReason: msirbRebootReason = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msirbRebootType = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootImmediate: msirbRebootType = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootDeferred: msirbRebootType = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type msmErrorType = i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageUnsupported: msmErrorType = 1i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageFailed: msmErrorType = 2i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorExclusion: msmErrorType = 3i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorTableMerge: msmErrorType = 4i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorResequenceMerge: msmErrorType = 5i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFileCreate: msmErrorType = 6i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorDirCreate: msmErrorType = 7i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFeatureRequired: msmErrorType = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_sys::core::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_sys::core::PCSTR,
    pub lpResourceName: ::windows_sys::core::PCSTR,
    pub lpApplicationName: ::windows_sys::core::PCSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_sys::core::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_sys::core::PCWSTR,
    pub lpResourceName: ::windows_sys::core::PCWSTR,
    pub lpApplicationName: ::windows_sys::core::PCWSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: ::windows_sys::core::PCWSTR,
    pub lpAssemblyManifestPath: ::windows_sys::core::PCWSTR,
    pub lpAssemblyPolicyPath: ::windows_sys::core::PCWSTR,
    pub lpAssemblyDirectoryName: ::windows_sys::core::PCWSTR,
    pub ulFileCount: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: *mut COMPATIBILITY_CONTEXT_ELEMENT,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: ::windows_sys::core::PCWSTR,
    pub lpRootConfigurationPath: ::windows_sys::core::PCWSTR,
    pub lpAppDirPath: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: ::windows_sys::core::PCWSTR,
    pub lpFilePath: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: ::windows_sys::core::PWSTR,
    pub cchBuf: u32,
}
impl ::core::marker::Copy for ASSEMBLY_INFO {}
impl ::core::clone::Clone for ASSEMBLY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows_sys::core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::clone::Clone for COMPATIBILITY_CONTEXT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl ::core::marker::Copy for DELTA_HASH {}
impl ::core::clone::Clone for DELTA_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: super::super::Foundation::FILETIME,
    pub TargetHashAlgId: u32,
    pub TargetHash: DELTA_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_HEADER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DELTA_INPUT_0 {
    pub lpcStart: *const ::core::ffi::c_void,
    pub lpStart: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::core::ffi::c_void,
    pub uSize: usize,
}
impl ::core::marker::Copy for DELTA_OUTPUT {}
impl ::core::clone::Clone for DELTA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows_sys::core::GUID,
    pub szIdentifier: ::windows_sys::core::PCWSTR,
    pub szNonCannonicalData: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for FUSION_INSTALL_REFERENCE {}
impl ::core::clone::Clone for FUSION_INSTALL_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl ::core::marker::Copy for MSIFILEHASHINFO {}
impl ::core::clone::Clone for MSIFILEHASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MSIHANDLE = u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: ::windows_sys::core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOA {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: ::windows_sys::core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOW {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl ::core::marker::Copy for PATCH_IGNORE_RANGE {}
impl ::core::clone::Clone for PATCH_IGNORE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: ::windows_sys::core::PCSTR,
    pub OldFileNameW: ::windows_sys::core::PCWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_sys::core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_A {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::super::Foundation::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_H {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_sys::core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_W {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: ::windows_sys::core::PCSTR,
    pub OldFileSymbolPathArray: *mut ::windows_sys::core::PSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut ::core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl ::core::marker::Copy for PATCH_RETAIN_RANGE {}
impl ::core::clone::Clone for PATCH_RETAIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_APPTASKTYPE {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl ::core::marker::Copy for PM_APPTASKTYPE {}
impl ::core::clone::Clone for PM_APPTASKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_BSATASKID {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskID: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_BSATASKID {}
impl ::core::clone::Clone for PM_BSATASKID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_BWTASKID {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskID: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_BWTASKID {}
impl ::core::clone::Clone for PM_BWTASKID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
impl ::core::marker::Copy for PM_ENUM_FILTER {}
impl ::core::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows_sys::core::GUID,
    pub TileProductID: ::windows_sys::core::GUID,
    pub AppTaskType: PM_APPTASKTYPE,
    pub Consumer: PM_EXTENSIONCONSUMER,
    pub BSATask: PM_BSATASKID,
    pub BSAProductID: ::windows_sys::core::GUID,
    pub BWTask: PM_BWTASKID,
    pub ProtocolName: ::windows_sys::core::BSTR,
    pub FileType: ::windows_sys::core::BSTR,
    pub ContentType: ::windows_sys::core::BSTR,
    pub AppSupportedFileExtPID: ::windows_sys::core::GUID,
    pub ShareTargetFileType: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_ENUM_FILTER_0 {}
impl ::core::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows_sys::core::GUID,
    pub ExtensionID: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_EXTENSIONCONSUMER {}
impl ::core::clone::Clone for PM_EXTENSIONCONSUMER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: ::windows_sys::core::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: super::super::Foundation::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows_sys::core::GUID,
    pub MarketplaceAppVersion: ::windows_sys::core::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INSTALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: ::windows_sys::core::BSTR,
    pub URIFragmentOrArgs: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_INVOCATIONINFO {}
impl ::core::clone::Clone for PM_INVOCATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_sys::core::GUID,
    pub AppTitle: ::windows_sys::core::BSTR,
    pub IconPath: ::windows_sys::core::BSTR,
    pub IsUninstallable: super::super::Foundation::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows_sys::core::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: super::super::Foundation::BOOL,
    pub IsModernLightUp: super::super::Foundation::BOOL,
    pub LightUpSupportMask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTAPPBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_sys::core::GUID,
    pub TileID: ::windows_sys::core::BSTR,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: super::super::Foundation::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: super::super::Foundation::BOOL,
    pub IsModern: super::super::Foundation::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTTILEBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: ::windows_sys::core::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::windows_sys::core::BSTR,
    pub DeploymentOptions: u32,
}
impl ::core::marker::Copy for PM_UPDATEINFO {}
impl ::core::clone::Clone for PM_UPDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: ::windows_sys::core::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for PM_UPDATEINFO_LEGACY {}
impl ::core::clone::Clone for PM_UPDATEINFO_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl ::core::marker::Copy for PROTECTED_FILE_DATA {}
impl ::core::clone::Clone for PROTECTED_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERA = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_sys::core::PCSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERW = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_sys::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: ::windows_sys::core::PCWSTR, szwdescription: ::windows_sys::core::PCWSTR, szwlocation: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = ::core::option::Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: ::windows_sys::core::PCWSTR, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PINSTALLUI_HANDLER_RECORD = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: ::windows_sys::core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
