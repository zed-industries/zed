#[inline]
pub unsafe fn ActivateActCtx(hactctx: Option<super::super::Foundation::HANDLE>, lpcookie: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ActivateActCtx(hactctx : super::super::Foundation:: HANDLE, lpcookie : *mut usize) -> windows_core::BOOL);
    unsafe { ActivateActCtx(hactctx.unwrap_or(core::mem::zeroed()) as _, lpcookie as _).ok() }
}
#[inline]
pub unsafe fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn AddRefActCtx(hactctx : super::super::Foundation:: HANDLE));
    unsafe { AddRefActCtx(hactctx as _) }
}
#[inline]
pub unsafe fn ApplyDeltaA<P1, P2, P3>(applyflags: i64, lpsourcename: P1, lpdeltaname: P2, lptargetname: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn ApplyDeltaA(applyflags : i64, lpsourcename : windows_core::PCSTR, lpdeltaname : windows_core::PCSTR, lptargetname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ApplyDeltaA(applyflags, lpsourcename.param().abi(), lpdeltaname.param().abi(), lptargetname.param().abi()) }
}
#[inline]
pub unsafe fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn ApplyDeltaB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lptarget : *mut DELTA_OUTPUT) -> windows_core::BOOL);
    unsafe { ApplyDeltaB(applyflags, core::mem::transmute(source), core::mem::transmute(delta), lptarget as _) }
}
#[inline]
pub unsafe fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: Option<*const super::super::Foundation::FILETIME>, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn ApplyDeltaGetReverseB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lpreversefiletime : *const super::super::Foundation:: FILETIME, lptarget : *mut DELTA_OUTPUT, lptargetreverse : *mut DELTA_OUTPUT) -> windows_core::BOOL);
    unsafe { ApplyDeltaGetReverseB(applyflags, core::mem::transmute(source), core::mem::transmute(delta), lpreversefiletime.unwrap_or(core::mem::zeroed()) as _, lptarget as _, lptargetreverse as _) }
}
#[inline]
pub unsafe fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut core::ffi::c_void, utargetsize: usize) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn ApplyDeltaProvidedB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lptarget : *mut core::ffi::c_void, utargetsize : usize) -> windows_core::BOOL);
    unsafe { ApplyDeltaProvidedB(applyflags, core::mem::transmute(source), core::mem::transmute(delta), lptarget as _, utargetsize) }
}
#[inline]
pub unsafe fn ApplyDeltaW<P1, P2, P3>(applyflags: i64, lpsourcename: P1, lpdeltaname: P2, lptargetname: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn ApplyDeltaW(applyflags : i64, lpsourcename : windows_core::PCWSTR, lpdeltaname : windows_core::PCWSTR, lptargetname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ApplyDeltaW(applyflags, lpsourcename.param().abi(), lpdeltaname.param().abi(), lptargetname.param().abi()) }
}
#[inline]
pub unsafe fn ApplyPatchToFileA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(patchfilemapped: &[u8], oldfilemapped: Option<&[u8]>, newfilebuffer: &mut [u8], newfileactualsize: Option<*mut u32>, newfiletime: Option<*mut super::super::Foundation::FILETIME>, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByBuffers(patchfilemapped : *const u8, patchfilesize : u32, oldfilemapped : *const u8, oldfilesize : u32, newfilebuffer : *mut *mut u8, newfilebuffersize : u32, newfileactualsize : *mut u32, newfiletime : *mut super::super::Foundation:: FILETIME, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe {
        ApplyPatchToFileByBuffers(
            core::mem::transmute(patchfilemapped.as_ptr()),
            patchfilemapped.len().try_into().unwrap(),
            core::mem::transmute(oldfilemapped.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            oldfilemapped.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(newfilebuffer.as_ptr()),
            newfilebuffer.len().try_into().unwrap(),
            newfileactualsize.unwrap_or(core::mem::zeroed()) as _,
            newfiletime.unwrap_or(core::mem::zeroed()) as _,
            applyoptionflags,
            progresscallback,
            callbackcontext.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn ApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: Option<super::super::Foundation::HANDLE>, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandles(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags) }
}
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: Option<super::super::Foundation::HANDLE>, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandlesEx(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandlesEx(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileExA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileExW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn CreateActCtxA(pactctx: *const ACTCTXA) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn CreateActCtxA(pactctx : *const ACTCTXA) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateActCtxA(pactctx) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreateActCtxW(pactctx: *const ACTCTXW) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn CreateActCtxW(pactctx : *const ACTCTXW) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateActCtxW(pactctx) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn CreateDeltaA<P3, P4, P5, P6, P10>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P3, lptargetname: P4, lpsourceoptionsname: P5, lptargetoptionsname: P6, globaloptions: DELTA_INPUT, lptargetfiletime: Option<*const super::super::Foundation::FILETIME>, hashalgid: super::super::Security::Cryptography::ALG_ID, lpdeltaname: P10) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P10: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn CreateDeltaA(filetypeset : i64, setflags : i64, resetflags : i64, lpsourcename : windows_core::PCSTR, lptargetname : windows_core::PCSTR, lpsourceoptionsname : windows_core::PCSTR, lptargetoptionsname : windows_core::PCSTR, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : super::super::Security::Cryptography:: ALG_ID, lpdeltaname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { CreateDeltaA(filetypeset, setflags, resetflags, lpsourcename.param().abi(), lptargetname.param().abi(), lpsourceoptionsname.param().abi(), lptargetoptionsname.param().abi(), core::mem::transmute(globaloptions), lptargetfiletime.unwrap_or(core::mem::zeroed()) as _, hashalgid, lpdeltaname.param().abi()) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: Option<*const super::super::Foundation::FILETIME>, hashalgid: super::super::Security::Cryptography::ALG_ID, lpdelta: *mut DELTA_OUTPUT) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn CreateDeltaB(filetypeset : i64, setflags : i64, resetflags : i64, source : DELTA_INPUT, target : DELTA_INPUT, sourceoptions : DELTA_INPUT, targetoptions : DELTA_INPUT, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : super::super::Security::Cryptography:: ALG_ID, lpdelta : *mut DELTA_OUTPUT) -> windows_core::BOOL);
    unsafe { CreateDeltaB(filetypeset, setflags, resetflags, core::mem::transmute(source), core::mem::transmute(target), core::mem::transmute(sourceoptions), core::mem::transmute(targetoptions), core::mem::transmute(globaloptions), lptargetfiletime.unwrap_or(core::mem::zeroed()) as _, hashalgid, lpdelta as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn CreateDeltaW<P3, P4, P5, P6, P10>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P3, lptargetname: P4, lpsourceoptionsname: P5, lptargetoptionsname: P6, globaloptions: DELTA_INPUT, lptargetfiletime: Option<*const super::super::Foundation::FILETIME>, hashalgid: super::super::Security::Cryptography::ALG_ID, lpdeltaname: P10) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P10: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn CreateDeltaW(filetypeset : i64, setflags : i64, resetflags : i64, lpsourcename : windows_core::PCWSTR, lptargetname : windows_core::PCWSTR, lpsourceoptionsname : windows_core::PCWSTR, lptargetoptionsname : windows_core::PCWSTR, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : super::super::Security::Cryptography:: ALG_ID, lpdeltaname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { CreateDeltaW(filetypeset, setflags, resetflags, lpsourcename.param().abi(), lptargetname.param().abi(), lpsourceoptionsname.param().abi(), lptargetoptionsname.param().abi(), core::mem::transmute(globaloptions), lptargetfiletime.unwrap_or(core::mem::zeroed()) as _, hashalgid, lpdeltaname.param().abi()) }
}
#[inline]
pub unsafe fn CreatePatchFileA<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileA(oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileA(oldfilename.param().abi(), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileByHandles(oldfilehandle: Option<super::super::Foundation::HANDLE>, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandles(oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, patchfilehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandles(oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_H], newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandlesEx(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_H, newfilehandle : super::super::Foundation:: HANDLE, patchfilehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandlesEx(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileExA<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_A], newfilename: P2, patchfilename: P3, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExA(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_A, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileExA(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileExW<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_W], newfilename: P2, patchfilename: P3, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExW(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_W, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileExW(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileW<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileW(oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileW(oldfilename.param().abi(), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn DeactivateActCtx(dwflags : u32, ulcookie : usize) -> windows_core::BOOL);
    unsafe { DeactivateActCtx(dwflags, ulcookie).ok() }
}
#[inline]
pub unsafe fn DeltaFree(lpmemory: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn DeltaFree(lpmemory : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeltaFree(lpmemory) }
}
#[inline]
pub unsafe fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut core::ffi::c_void, usourcesize: usize) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn DeltaNormalizeProvidedB(filetypeset : i64, normalizeflags : i64, normalizeoptions : DELTA_INPUT, lpsource : *mut core::ffi::c_void, usourcesize : usize) -> windows_core::BOOL);
    unsafe { DeltaNormalizeProvidedB(filetypeset, normalizeflags, core::mem::transmute(normalizeoptions), lpsource as _, usourcesize) }
}
#[inline]
pub unsafe fn ExtractPatchHeaderToFileA<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileA(patchfilename : windows_core::PCSTR, patchheaderfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileA(patchfilename.param().abi(), patchheaderfilename.param().abi()) }
}
#[inline]
pub unsafe fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, patchheaderfilehandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, patchheaderfilehandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileByHandles(patchfilehandle, patchheaderfilehandle) }
}
#[inline]
pub unsafe fn ExtractPatchHeaderToFileW<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileW(patchfilename : windows_core::PCWSTR, patchheaderfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileW(patchfilename.param().abi(), patchheaderfilename.param().abi()) }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: u32, lpguidtofind: Option<*const windows_core::GUID>, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionGuid(dwflags : u32, lpextensionguid : *const windows_core::GUID, ulsectionid : u32, lpguidtofind : *const windows_core::GUID, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionGuid(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpguidtofind.unwrap_or(core::mem::zeroed()) as _, returneddata as _).ok() }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FindActCtxSectionStringA<P3>(dwflags: u32, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: u32, lpstringtofind: P3, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionStringA(dwflags : u32, lpextensionguid : *const windows_core::GUID, ulsectionid : u32, lpstringtofind : windows_core::PCSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionStringA(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpstringtofind.param().abi(), returneddata as _).ok() }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FindActCtxSectionStringW<P3>(dwflags: u32, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: u32, lpstringtofind: P3, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionStringW(dwflags : u32, lpextensionguid : *const windows_core::GUID, ulsectionid : u32, lpstringtofind : windows_core::PCWSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionStringW(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpstringtofind.param().abi(), returneddata as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentActCtx(lphactctx : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { GetCurrentActCtx(lphactctx as _).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaInfoA<P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn GetDeltaInfoA(lpdeltaname : windows_core::PCSTR, lpheaderinfo : *mut DELTA_HEADER_INFO) -> windows_core::BOOL);
    unsafe { GetDeltaInfoA(lpdeltaname.param().abi(), lpheaderinfo as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn GetDeltaInfoB(delta : DELTA_INPUT, lpheaderinfo : *mut DELTA_HEADER_INFO) -> windows_core::BOOL);
    unsafe { GetDeltaInfoB(core::mem::transmute(delta), lpheaderinfo as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaInfoW<P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn GetDeltaInfoW(lpdeltaname : windows_core::PCWSTR, lpheaderinfo : *mut DELTA_HEADER_INFO) -> windows_core::BOOL);
    unsafe { GetDeltaInfoW(lpdeltaname.param().abi(), lpheaderinfo as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaSignatureA<P2>(filetypeset: i64, hashalgid: super::super::Security::Cryptography::ALG_ID, lpsourcename: P2, lphash: *mut DELTA_HASH) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn GetDeltaSignatureA(filetypeset : i64, hashalgid : super::super::Security::Cryptography:: ALG_ID, lpsourcename : windows_core::PCSTR, lphash : *mut DELTA_HASH) -> windows_core::BOOL);
    unsafe { GetDeltaSignatureA(filetypeset, hashalgid, lpsourcename.param().abi(), lphash as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaSignatureB(filetypeset: i64, hashalgid: super::super::Security::Cryptography::ALG_ID, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> windows_core::BOOL {
    windows_core::link!("msdelta.dll" "system" fn GetDeltaSignatureB(filetypeset : i64, hashalgid : super::super::Security::Cryptography:: ALG_ID, source : DELTA_INPUT, lphash : *mut DELTA_HASH) -> windows_core::BOOL);
    unsafe { GetDeltaSignatureB(filetypeset, hashalgid, core::mem::transmute(source), lphash as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn GetDeltaSignatureW<P2>(filetypeset: i64, hashalgid: super::super::Security::Cryptography::ALG_ID, lpsourcename: P2, lphash: *mut DELTA_HASH) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdelta.dll" "system" fn GetDeltaSignatureW(filetypeset : i64, hashalgid : super::super::Security::Cryptography:: ALG_ID, lpsourcename : windows_core::PCWSTR, lphash : *mut DELTA_HASH) -> windows_core::BOOL);
    unsafe { GetDeltaSignatureW(filetypeset, hashalgid, lpsourcename.param().abi(), lphash as _) }
}
#[inline]
pub unsafe fn GetFilePatchSignatureA<P0>(filename: P0, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn GetFilePatchSignatureA(filename : windows_core::PCSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureA(
            filename.param().abi(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffer.len().try_into().unwrap(),
            core::mem::transmute(signaturebuffer.as_ptr()),
        )
    }
}
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(filebufferwritable: &mut [u8], optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn GetFilePatchSignatureByBuffer(filebufferwritable : *mut u8, filesize : u32, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureByBuffer(
            core::mem::transmute(filebufferwritable.as_ptr()),
            filebufferwritable.len().try_into().unwrap(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffer.len().try_into().unwrap(),
            core::mem::transmute(signaturebuffer.as_ptr()),
        )
    }
}
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle(filehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn GetFilePatchSignatureByHandle(filehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureByHandle(
            filehandle,
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffer.len().try_into().unwrap(),
            core::mem::transmute(signaturebuffer.as_ptr()),
        )
    }
}
#[inline]
pub unsafe fn GetFilePatchSignatureW<P0>(filename: P0, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: u32, signaturebuffer: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn GetFilePatchSignatureW(filename : windows_core::PCWSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureW(
            filename.param().abi(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffersize,
            core::mem::transmute(signaturebuffer),
        )
    }
}
#[inline]
pub unsafe fn MsiAdvertiseProductA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : u16) -> u32);
    unsafe { MsiAdvertiseProductA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[inline]
pub unsafe fn MsiAdvertiseProductExA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : u16, dwplatform : u32, dwoptions : u32) -> u32);
    unsafe { MsiAdvertiseProductExA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[inline]
pub unsafe fn MsiAdvertiseProductExW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : u16, dwplatform : u32, dwoptions : u32) -> u32);
    unsafe { MsiAdvertiseProductExW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[inline]
pub unsafe fn MsiAdvertiseProductW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : u16) -> u32);
    unsafe { MsiAdvertiseProductW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<P0>(szscriptfile: P0, dwflags: u32, phregdata: Option<*const super::Registry::HKEY>, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptA(szscriptfile : windows_core::PCSTR, dwflags : u32, phregdata : *const super::Registry:: HKEY, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiAdvertiseScriptA(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<P0>(szscriptfile: P0, dwflags: u32, phregdata: Option<*const super::Registry::HKEY>, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, dwflags : u32, phregdata : *const super::Registry:: HKEY, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiAdvertiseScriptW(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesA(szpatchpackages : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szpropertieslist : windows_core::PCSTR) -> u32);
    unsafe { MsiApplyMultiplePatchesA(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesW(szpatchpackages : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szpropertieslist : windows_core::PCWSTR) -> u32);
    unsafe { MsiApplyMultiplePatchesW(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyPatchA<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchA(szpatchpackage : windows_core::PCSTR, szinstallpackage : windows_core::PCSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiApplyPatchA(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyPatchW<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchW(szpatchpackage : windows_core::PCWSTR, szinstallpackage : windows_core::PCWSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiApplyPatchW(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiBeginTransactionA<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionA(szname : windows_core::PCSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MsiBeginTransactionA(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[inline]
pub unsafe fn MsiBeginTransactionW<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionW(szname : windows_core::PCWSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MsiBeginTransactionW(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[inline]
pub unsafe fn MsiCloseAllHandles() -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiCloseAllHandles() -> u32);
    unsafe { MsiCloseAllHandles() }
}
#[inline]
pub unsafe fn MsiCloseHandle(hany: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiCloseHandle(hany : MSIHANDLE) -> u32);
    unsafe { MsiCloseHandle(hany) }
}
#[inline]
pub unsafe fn MsiCollectUserInfoA<P0>(szproduct: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoA(szproduct : windows_core::PCSTR) -> u32);
    unsafe { MsiCollectUserInfoA(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiCollectUserInfoW<P0>(szproduct: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoW(szproduct : windows_core::PCWSTR) -> u32);
    unsafe { MsiCollectUserInfoW(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureFeatureA<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureFeatureA(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureFeatureW<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureFeatureW(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureProductA<P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductA(szproduct : windows_core::PCSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureProductA(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureProductExA<P0, P3>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExA(szproduct : windows_core::PCSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiConfigureProductExA(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureProductExW<P0, P3>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExW(szproduct : windows_core::PCWSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiConfigureProductExW(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureProductW<P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductW(szproduct : windows_core::PCWSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureProductW(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[inline]
pub unsafe fn MsiCreateRecord(cparams: u32) -> MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiCreateRecord(cparams : u32) -> MSIHANDLE);
    unsafe { MsiCreateRecord(cparams) }
}
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA<P2>(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoA(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : MSITRANSFORM_ERROR, ivalidation : MSITRANSFORM_VALIDATE) -> u32);
    unsafe { MsiCreateTransformSummaryInfoA(hdatabase, hdatabasereference, sztransformfile.param().abi(), ierrorconditions, ivalidation) }
}
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW<P2>(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoW(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : MSITRANSFORM_ERROR, ivalidation : MSITRANSFORM_VALIDATE) -> u32);
    unsafe { MsiCreateTransformSummaryInfoW(hdatabase, hdatabasereference, sztransformfile.param().abi(), ierrorconditions, ivalidation) }
}
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA<P1>(hdatabase: MSIHANDLE, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformA(hdatabase : MSIHANDLE, sztransformfile : windows_core::PCSTR, ierrorconditions : MSITRANSFORM_ERROR) -> u32);
    unsafe { MsiDatabaseApplyTransformA(hdatabase, sztransformfile.param().abi(), ierrorconditions) }
}
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW<P1>(hdatabase: MSIHANDLE, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseApplyTransformW(hdatabase : MSIHANDLE, sztransformfile : windows_core::PCWSTR, ierrorconditions : MSITRANSFORM_ERROR) -> u32);
    unsafe { MsiDatabaseApplyTransformW(hdatabase, sztransformfile.param().abi(), ierrorconditions) }
}
#[inline]
pub unsafe fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiDatabaseCommit(hdatabase : MSIHANDLE) -> u32);
    unsafe { MsiDatabaseCommit(hdatabase) }
}
#[inline]
pub unsafe fn MsiDatabaseExportA<P1, P2, P3>(hdatabase: MSIHANDLE, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportA(hdatabase : MSIHANDLE, sztablename : windows_core::PCSTR, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseExportA(hdatabase, sztablename.param().abi(), szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseExportW<P1, P2, P3>(hdatabase: MSIHANDLE, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseExportW(hdatabase : MSIHANDLE, sztablename : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseExportW(hdatabase, sztablename.param().abi(), szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA<P2>(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformA(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : windows_core::PCSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    unsafe { MsiDatabaseGenerateTransformA(hdatabase, hdatabasereference, sztransformfile.param().abi(), ireserved1, ireserved2) }
}
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW<P2>(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformW(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : windows_core::PCWSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    unsafe { MsiDatabaseGenerateTransformW(hdatabase, hdatabasereference, sztransformfile.param().abi(), ireserved1, ireserved2) }
}
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA<P1>(hdatabase: MSIHANDLE, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysA(hdatabase : MSIHANDLE, sztablename : windows_core::PCSTR, phrecord : *mut MSIHANDLE) -> u32);
    unsafe { MsiDatabaseGetPrimaryKeysA(hdatabase, sztablename.param().abi(), phrecord as _) }
}
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW<P1>(hdatabase: MSIHANDLE, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysW(hdatabase : MSIHANDLE, sztablename : windows_core::PCWSTR, phrecord : *mut MSIHANDLE) -> u32);
    unsafe { MsiDatabaseGetPrimaryKeysW(hdatabase, sztablename.param().abi(), phrecord as _) }
}
#[inline]
pub unsafe fn MsiDatabaseImportA<P1, P2>(hdatabase: MSIHANDLE, szfolderpath: P1, szfilename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportA(hdatabase : MSIHANDLE, szfolderpath : windows_core::PCSTR, szfilename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseImportA(hdatabase, szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseImportW<P1, P2>(hdatabase: MSIHANDLE, szfolderpath: P1, szfilename: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseImportW(hdatabase : MSIHANDLE, szfolderpath : windows_core::PCWSTR, szfilename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseImportW(hdatabase, szfolderpath.param().abi(), szfilename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA<P1>(hdatabase: MSIHANDLE, sztablename: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentA(hdatabase : MSIHANDLE, sztablename : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentA(hdatabase, sztablename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW<P1>(hdatabase: MSIHANDLE, sztablename: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentW(hdatabase : MSIHANDLE, sztablename : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiDatabaseIsTablePersistentW(hdatabase, sztablename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseMergeA<P2>(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeA(hdatabase : MSIHANDLE, hdatabasemerge : MSIHANDLE, sztablename : windows_core::PCSTR) -> u32);
    unsafe { MsiDatabaseMergeA(hdatabase, hdatabasemerge, sztablename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseMergeW<P2>(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseMergeW(hdatabase : MSIHANDLE, hdatabasemerge : MSIHANDLE, sztablename : windows_core::PCWSTR) -> u32);
    unsafe { MsiDatabaseMergeW(hdatabase, hdatabasemerge, sztablename.param().abi()) }
}
#[inline]
pub unsafe fn MsiDatabaseOpenViewA<P1>(hdatabase: MSIHANDLE, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewA(hdatabase : MSIHANDLE, szquery : windows_core::PCSTR, phview : *mut MSIHANDLE) -> u32);
    unsafe { MsiDatabaseOpenViewA(hdatabase, szquery.param().abi(), phview as _) }
}
#[inline]
pub unsafe fn MsiDatabaseOpenViewW<P1>(hdatabase: MSIHANDLE, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDatabaseOpenViewW(hdatabase : MSIHANDLE, szquery : windows_core::PCWSTR, phview : *mut MSIHANDLE) -> u32);
    unsafe { MsiDatabaseOpenViewW(hdatabase, szquery.param().abi(), phview as _) }
}
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesA(szproductpackagepath : windows_core::PCSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    unsafe { MsiDetermineApplicablePatchesA(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesW(szproductpackagepath : windows_core::PCWSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    unsafe { MsiDetermineApplicablePatchesW(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    unsafe { MsiDeterminePatchSequenceA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    unsafe { MsiDeterminePatchSequenceW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDoActionA<P1>(hinstall: MSIHANDLE, szaction: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDoActionA(hinstall : MSIHANDLE, szaction : windows_core::PCSTR) -> u32);
    unsafe { MsiDoActionA(hinstall, szaction.param().abi()) }
}
#[inline]
pub unsafe fn MsiDoActionW<P1>(hinstall: MSIHANDLE, szaction: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDoActionW(hinstall : MSIHANDLE, szaction : windows_core::PCWSTR) -> u32);
    unsafe { MsiDoActionW(hinstall, szaction.param().abi()) }
}
#[inline]
pub unsafe fn MsiEnableLogA<P1>(dwlogmode: INSTALLLOGMODE, szlogfile: P1, dwlogattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogA(dwlogmode : u32, szlogfile : windows_core::PCSTR, dwlogattributes : u32) -> u32);
    unsafe { MsiEnableLogA(dwlogmode.0 as _, szlogfile.param().abi(), dwlogattributes) }
}
#[inline]
pub unsafe fn MsiEnableLogW<P1>(dwlogmode: INSTALLLOGMODE, szlogfile: P1, dwlogattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogW(dwlogmode : u32, szlogfile : windows_core::PCWSTR, dwlogattributes : u32) -> u32);
    unsafe { MsiEnableLogW(dwlogmode.0 as _, szlogfile.param().abi(), dwlogattributes) }
}
#[inline]
pub unsafe fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnableUIPreview(hdatabase : MSIHANDLE, phpreview : *mut MSIHANDLE) -> u32);
    unsafe { MsiEnableUIPreview(hdatabase, phpreview as _) }
}
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEndTransaction(dwtransactionstate : MSITRANSACTIONSTATE) -> u32);
    unsafe { MsiEndTransaction(dwtransactionstate) }
}
#[inline]
pub unsafe fn MsiEnumClientsA<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsA(szcomponent : windows_core::PCSTR, iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumClientsA(szcomponent.param().abi(), iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumClientsExA<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: Option<&mut [u8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExA(szcomponent : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumClientsExA(szcomponent.param().abi(), szusersid.param().abi(), dwcontext.0 as _, dwproductindex, core::mem::transmute(szproductbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumClientsExW<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExW(szcomponent : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumClientsExW(szcomponent.param().abi(), szusersid.param().abi(), dwcontext.0 as _, dwproductindex, core::mem::transmute(szproductbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumClientsW<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsW(szcomponent : windows_core::PCWSTR, iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumClientsW(szcomponent.param().abi(), iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumComponentCostsA<P1>(hinstall: MSIHANDLE, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: windows_core::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsA(hinstall : MSIHANDLE, szcomponent : windows_core::PCSTR, dwindex : u32, istate : INSTALLSTATE, szdrivebuf : windows_core::PSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    unsafe { MsiEnumComponentCostsA(hinstall, szcomponent.param().abi(), dwindex, istate, core::mem::transmute(szdrivebuf), pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentCostsW<P1>(hinstall: MSIHANDLE, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: windows_core::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentCostsW(hinstall : MSIHANDLE, szcomponent : windows_core::PCWSTR, dwindex : u32, istate : INSTALLSTATE, szdrivebuf : windows_core::PWSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    unsafe { MsiEnumComponentCostsW(hinstall, szcomponent.param().abi(), dwindex, istate, core::mem::transmute(szdrivebuf), pcchdrivebuf as _, picost as _, pitempcost as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: windows_core::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: Option<windows_core::PSTR>, pcchapplicationdatabuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersA(szcomponent : windows_core::PCSTR, iindex : u32, lpqualifierbuf : windows_core::PSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_core::PSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    unsafe { MsiEnumComponentQualifiersA(szcomponent.param().abi(), iindex, core::mem::transmute(lpqualifierbuf), pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: windows_core::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: Option<windows_core::PWSTR>, pcchapplicationdatabuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersW(szcomponent : windows_core::PCWSTR, iindex : u32, lpqualifierbuf : windows_core::PWSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_core::PWSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    unsafe { MsiEnumComponentQualifiersW(szcomponent.param().abi(), iindex, core::mem::transmute(lpqualifierbuf), pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: windows_core::PSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsA(icomponentindex : u32, lpcomponentbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumComponentsA(icomponentindex, core::mem::transmute(lpcomponentbuf)) }
}
#[inline]
pub unsafe fn MsiEnumComponentsExA<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: Option<&mut [u8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExA(szusersid : windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumComponentsExA(szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsExW<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExW(szusersid : windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumComponentsExW(szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: windows_core::PWSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsW(icomponentindex : u32, lpcomponentbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumComponentsW(icomponentindex, core::mem::transmute(lpcomponentbuf)) }
}
#[inline]
pub unsafe fn MsiEnumFeaturesA<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: windows_core::PSTR, lpparentbuf: Option<windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesA(szproduct : windows_core::PCSTR, ifeatureindex : u32, lpfeaturebuf : windows_core::PSTR, lpparentbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumFeaturesA(szproduct.param().abi(), ifeatureindex, core::mem::transmute(lpfeaturebuf), lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumFeaturesW<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: windows_core::PWSTR, lpparentbuf: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesW(szproduct : windows_core::PCWSTR, ifeatureindex : u32, lpfeaturebuf : windows_core::PWSTR, lpparentbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumFeaturesW(szproduct.param().abi(), ifeatureindex, core::mem::transmute(lpfeaturebuf), lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesA<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: windows_core::PSTR, lptransformsbuf: windows_core::PSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesA(szproduct : windows_core::PCSTR, ipatchindex : u32, lppatchbuf : windows_core::PSTR, lptransformsbuf : windows_core::PSTR, pcchtransformsbuf : *mut u32) -> u32);
    unsafe { MsiEnumPatchesA(szproduct.param().abi(), ipatchindex, core::mem::transmute(lppatchbuf), core::mem::transmute(lptransformsbuf), pcchtransformsbuf as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: Option<&mut [u8; 39]>, sztargetproductcode: Option<&mut [u8; 39]>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PSTR>, pcchtargetusersid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : windows_core::PSTR, sztargetproductcode : windows_core::PSTR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PSTR, pcchtargetusersid : *mut u32) -> u32);
    unsafe { MsiEnumPatchesExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, core::mem::transmute(szpatchcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(sztargetproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: Option<&mut [u16; 39]>, sztargetproductcode: Option<&mut [u16; 39]>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PWSTR>, pcchtargetusersid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : windows_core::PWSTR, sztargetproductcode : windows_core::PWSTR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PWSTR, pcchtargetusersid : *mut u32) -> u32);
    unsafe { MsiEnumPatchesExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, core::mem::transmute(szpatchcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(sztargetproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesW<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: windows_core::PWSTR, lptransformsbuf: windows_core::PWSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesW(szproduct : windows_core::PCWSTR, ipatchindex : u32, lppatchbuf : windows_core::PWSTR, lptransformsbuf : windows_core::PWSTR, pcchtransformsbuf : *mut u32) -> u32);
    unsafe { MsiEnumPatchesW(szproduct.param().abi(), ipatchindex, core::mem::transmute(lppatchbuf), core::mem::transmute(lptransformsbuf), pcchtransformsbuf as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsA(iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumProductsA(iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumProductsExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: Option<&mut [u8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumProductsExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumProductsExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsW(iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumProductsW(iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<P0>(lpupgradecode: P0, dwreserved: Option<u32>, iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsA(lpupgradecode : windows_core::PCSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumRelatedProductsA(lpupgradecode.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<P0>(lpupgradecode: P0, dwreserved: Option<u32>, iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsW(lpupgradecode : windows_core::PCWSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumRelatedProductsW(lpupgradecode.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEvaluateConditionA<P1>(hinstall: MSIHANDLE, szcondition: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionA(hinstall : MSIHANDLE, szcondition : windows_core::PCSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionA(hinstall, szcondition.param().abi()) }
}
#[inline]
pub unsafe fn MsiEvaluateConditionW<P1>(hinstall: MSIHANDLE, szcondition: P1) -> MSICONDITION
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEvaluateConditionW(hinstall : MSIHANDLE, szcondition : windows_core::PCWSTR) -> MSICONDITION);
    unsafe { MsiEvaluateConditionW(hinstall, szcondition.param().abi()) }
}
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<P0>(szpatchpath: P0, dwreserved: Option<u32>, szxmldata: Option<windows_core::PSTR>, pcchxmldata: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataA(szpatchpath : windows_core::PCSTR, dwreserved : u32, szxmldata : windows_core::PSTR, pcchxmldata : *mut u32) -> u32);
    unsafe { MsiExtractPatchXMLDataA(szpatchpath.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<P0>(szpatchpath: P0, dwreserved: Option<u32>, szxmldata: Option<windows_core::PWSTR>, pcchxmldata: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataW(szpatchpath : windows_core::PCWSTR, dwreserved : u32, szxmldata : windows_core::PWSTR, pcchxmldata : *mut u32) -> u32);
    unsafe { MsiExtractPatchXMLDataW(szpatchpath.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: Option<windows_core::PSTR>, pcchresultbuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordA(hinstall : MSIHANDLE, hrecord : MSIHANDLE, szresultbuf : windows_core::PSTR, pcchresultbuf : *mut u32) -> u32);
    unsafe { MsiFormatRecordA(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: Option<windows_core::PWSTR>, pcchresultbuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiFormatRecordW(hinstall : MSIHANDLE, hrecord : MSIHANDLE, szresultbuf : windows_core::PWSTR, pcchresultbuf : *mut u32) -> u32);
    unsafe { MsiFormatRecordW(hinstall, hrecord, szresultbuf.unwrap_or(core::mem::zeroed()) as _, pcchresultbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetActiveDatabase(hinstall : MSIHANDLE) -> MSIHANDLE);
    unsafe { MsiGetActiveDatabase(hinstall) }
}
#[inline]
pub unsafe fn MsiGetComponentPathA<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathA(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathExA<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PSTR>, pcchoutpathbuffer: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExA(szproductcode : windows_core::PCSTR, szcomponentcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExA(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathExW<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PWSTR>, pcchoutpathbuffer: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExW(szproductcode : windows_core::PCWSTR, szcomponentcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PWSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExW(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathW<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathW(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentStateA<P1>(hinstall: MSIHANDLE, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateA(hinstall : MSIHANDLE, szcomponent : windows_core::PCSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    unsafe { MsiGetComponentStateA(hinstall, szcomponent.param().abi(), piinstalled as _, piaction as _) }
}
#[inline]
pub unsafe fn MsiGetComponentStateW<P1>(hinstall: MSIHANDLE, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentStateW(hinstall : MSIHANDLE, szcomponent : windows_core::PCWSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    unsafe { MsiGetComponentStateW(hinstall, szcomponent.param().abi(), piinstalled as _, piaction as _) }
}
#[inline]
pub unsafe fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE {
    windows_core::link!("msi.dll" "system" fn MsiGetDatabaseState(hdatabase : MSIHANDLE) -> MSIDBSTATE);
    unsafe { MsiGetDatabaseState(hdatabase) }
}
#[inline]
pub unsafe fn MsiGetFeatureCostA<P1>(hinstall: MSIHANDLE, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostA(hinstall : MSIHANDLE, szfeature : windows_core::PCSTR, icosttree : MSICOSTTREE, istate : INSTALLSTATE, picost : *mut i32) -> u32);
    unsafe { MsiGetFeatureCostA(hinstall, szfeature.param().abi(), icosttree, istate, picost as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureCostW<P1>(hinstall: MSIHANDLE, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureCostW(hinstall : MSIHANDLE, szfeature : windows_core::PCWSTR, icosttree : MSICOSTTREE, istate : INSTALLSTATE, picost : *mut i32) -> u32);
    unsafe { MsiGetFeatureCostW(hinstall, szfeature.param().abi(), icosttree, istate, picost as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureInfoA<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<*mut u32>, lptitlebuf: Option<windows_core::PSTR>, pcchtitlebuf: Option<*mut u32>, lphelpbuf: Option<windows_core::PSTR>, pcchhelpbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoA(hproduct : MSIHANDLE, szfeature : windows_core::PCSTR, lpattributes : *mut u32, lptitlebuf : windows_core::PSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_core::PSTR, pcchhelpbuf : *mut u32) -> u32);
    unsafe { MsiGetFeatureInfoA(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureInfoW<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<*mut u32>, lptitlebuf: Option<windows_core::PWSTR>, pcchtitlebuf: Option<*mut u32>, lphelpbuf: Option<windows_core::PWSTR>, pcchhelpbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoW(hproduct : MSIHANDLE, szfeature : windows_core::PCWSTR, lpattributes : *mut u32, lptitlebuf : windows_core::PWSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_core::PWSTR, pcchhelpbuf : *mut u32) -> u32);
    unsafe { MsiGetFeatureInfoW(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureStateA<P1>(hinstall: MSIHANDLE, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateA(hinstall : MSIHANDLE, szfeature : windows_core::PCSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    unsafe { MsiGetFeatureStateA(hinstall, szfeature.param().abi(), piinstalled as _, piaction as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureStateW<P1>(hinstall: MSIHANDLE, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureStateW(hinstall : MSIHANDLE, szfeature : windows_core::PCWSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    unsafe { MsiGetFeatureStateW(hinstall, szfeature.param().abi(), piinstalled as _, piaction as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureUsageA<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<*mut u32>, pwdateused: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    unsafe { MsiGetFeatureUsageA(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureUsageW<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<*mut u32>, pwdateused: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    unsafe { MsiGetFeatureUsageW(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA<P1>(hinstall: MSIHANDLE, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesA(hinstall : MSIHANDLE, szfeature : windows_core::PCSTR, lpinstallstates : *mut u32) -> u32);
    unsafe { MsiGetFeatureValidStatesA(hinstall, szfeature.param().abi(), lpinstallstates as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW<P1>(hinstall: MSIHANDLE, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureValidStatesW(hinstall : MSIHANDLE, szfeature : windows_core::PCWSTR, lpinstallstates : *mut u32) -> u32);
    unsafe { MsiGetFeatureValidStatesW(hinstall, szfeature.param().abi(), lpinstallstates as _) }
}
#[inline]
pub unsafe fn MsiGetFileHashA<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashA(szfilepath : windows_core::PCSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    unsafe { MsiGetFileHashA(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[inline]
pub unsafe fn MsiGetFileHashW<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashW(szfilepath : windows_core::PCWSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    unsafe { MsiGetFileHashW(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: Option<*mut u8>, pcbhashdata: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationA(szsignedobjectpath : windows_core::PCSTR, dwflags : u32, ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationA(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: Option<*mut u8>, pcbhashdata: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationW(szsignedobjectpath : windows_core::PCWSTR, dwflags : u32, ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationW(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn MsiGetFileVersionA<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PSTR>, pcchversionbuf: Option<*mut u32>, lplangbuf: Option<windows_core::PSTR>, pcchlangbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionA(szfilepath : windows_core::PCSTR, lpversionbuf : windows_core::PSTR, pcchversionbuf : *mut u32, lplangbuf : windows_core::PSTR, pcchlangbuf : *mut u32) -> u32);
    unsafe { MsiGetFileVersionA(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFileVersionW<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PWSTR>, pcchversionbuf: Option<*mut u32>, lplangbuf: Option<windows_core::PWSTR>, pcchlangbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionW(szfilepath : windows_core::PCWSTR, lpversionbuf : windows_core::PWSTR, pcchversionbuf : *mut u32, lplangbuf : windows_core::PWSTR, pcchlangbuf : *mut u32) -> u32);
    unsafe { MsiGetFileVersionW(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16 {
    windows_core::link!("msi.dll" "system" fn MsiGetLanguage(hinstall : MSIHANDLE) -> u16);
    unsafe { MsiGetLanguage(hinstall) }
}
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> MSIHANDLE {
    windows_core::link!("msi.dll" "system" fn MsiGetLastErrorRecord() -> MSIHANDLE);
    unsafe { MsiGetLastErrorRecord() }
}
#[inline]
pub unsafe fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiGetMode(hinstall : MSIHANDLE, erunmode : MSIRUNMODE) -> windows_core::BOOL);
    unsafe { MsiGetMode(hinstall, erunmode) }
}
#[inline]
pub unsafe fn MsiGetPatchFileListA<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListA(szproductcode : windows_core::PCSTR, szpatchpackages : windows_core::PCSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    unsafe { MsiGetPatchFileListA(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[inline]
pub unsafe fn MsiGetPatchFileListW<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListW(szproductcode : windows_core::PCWSTR, szpatchpackages : windows_core::PCWSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    unsafe { MsiGetPatchFileListW(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoA<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoA(szpatch : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoA(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoExA<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExA(szpatchcode : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, lpvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoExA(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoExW<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExW(szpatchcode : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, lpvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoExW(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoW<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoW(szpatch : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoW(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductCodeA<P0>(szcomponent: P0, lpbuf39: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeA(szcomponent : windows_core::PCSTR, lpbuf39 : windows_core::PSTR) -> u32);
    unsafe { MsiGetProductCodeA(szcomponent.param().abi(), core::mem::transmute(lpbuf39)) }
}
#[inline]
pub unsafe fn MsiGetProductCodeW<P0>(szcomponent: P0, lpbuf39: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeW(szcomponent : windows_core::PCWSTR, lpbuf39 : windows_core::PWSTR) -> u32);
    unsafe { MsiGetProductCodeW(szcomponent.param().abi(), core::mem::transmute(lpbuf39)) }
}
#[inline]
pub unsafe fn MsiGetProductInfoA<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoA(szproduct : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoA(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetProductInfoExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetProductInfoExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PSTR>, plgidlanguage: Option<*mut u16>, pdwversion: Option<*mut u32>, lpnamebuf: Option<windows_core::PSTR>, pcchnamebuf: Option<*mut u32>, lppackagebuf: Option<windows_core::PSTR>, pcchpackagebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptA(szscriptfile : windows_core::PCSTR, lpproductbuf39 : windows_core::PSTR, plgidlanguage : *mut u16, pdwversion : *mut u32, lpnamebuf : windows_core::PSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_core::PSTR, pcchpackagebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoFromScriptA(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PWSTR>, plgidlanguage: Option<*mut u16>, pdwversion: Option<*mut u32>, lpnamebuf: Option<windows_core::PWSTR>, pcchnamebuf: Option<*mut u32>, lppackagebuf: Option<windows_core::PWSTR>, pcchpackagebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptW(szscriptfile : windows_core::PCWSTR, lpproductbuf39 : windows_core::PWSTR, plgidlanguage : *mut u16, pdwversion : *mut u32, lpnamebuf : windows_core::PWSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_core::PWSTR, pcchpackagebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoFromScriptW(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoW<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoW(szproduct : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoW(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductPropertyA<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyA(hproduct : MSIHANDLE, szproperty : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductPropertyA(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductPropertyW<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyW(hproduct : MSIHANDLE, szproperty : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductPropertyW(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPropertyA<P1>(hinstall: MSIHANDLE, szname: P1, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyA(hinstall : MSIHANDLE, szname : windows_core::PCSTR, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPropertyA(hinstall, szname.param().abi(), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPropertyW<P1>(hinstall: MSIHANDLE, szname: P1, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPropertyW(hinstall : MSIHANDLE, szname : windows_core::PCWSTR, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPropertyW(hinstall, szname.param().abi(), szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetShortcutTargetA<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PSTR>, szfeatureid: Option<windows_core::PSTR>, szcomponentcode: Option<windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetA(szshortcutpath : windows_core::PCSTR, szproductcode : windows_core::PSTR, szfeatureid : windows_core::PSTR, szcomponentcode : windows_core::PSTR) -> u32);
    unsafe { MsiGetShortcutTargetA(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetShortcutTargetW<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PWSTR>, szfeatureid: Option<windows_core::PWSTR>, szcomponentcode: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetW(szshortcutpath : windows_core::PCWSTR, szproductcode : windows_core::PWSTR, szfeatureid : windows_core::PWSTR, szcomponentcode : windows_core::PWSTR) -> u32);
    unsafe { MsiGetShortcutTargetW(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetSourcePathA<P1>(hinstall: MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathA(hinstall : MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetSourcePathA(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetSourcePathW<P1>(hinstall: MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSourcePathW(hinstall : MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetSourcePathW(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetSummaryInformationA<P1>(hdatabase: MSIHANDLE, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationA(hdatabase : MSIHANDLE, szdatabasepath : windows_core::PCSTR, uiupdatecount : u32, phsummaryinfo : *mut MSIHANDLE) -> u32);
    unsafe { MsiGetSummaryInformationA(hdatabase, szdatabasepath.param().abi(), uiupdatecount, phsummaryinfo as _) }
}
#[inline]
pub unsafe fn MsiGetSummaryInformationW<P1>(hdatabase: MSIHANDLE, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetSummaryInformationW(hdatabase : MSIHANDLE, szdatabasepath : windows_core::PCWSTR, uiupdatecount : u32, phsummaryinfo : *mut MSIHANDLE) -> u32);
    unsafe { MsiGetSummaryInformationW(hdatabase, szdatabasepath.param().abi(), uiupdatecount, phsummaryinfo as _) }
}
#[inline]
pub unsafe fn MsiGetTargetPathA<P1>(hinstall: MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathA(hinstall : MSIHANDLE, szfolder : windows_core::PCSTR, szpathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetTargetPathA(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetTargetPathW<P1>(hinstall: MSIHANDLE, szfolder: P1, szpathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetTargetPathW(hinstall : MSIHANDLE, szfolder : windows_core::PCWSTR, szpathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiGetTargetPathW(hinstall, szfolder.param().abi(), szpathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetUserInfoA<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PSTR>, pcchusernamebuf: Option<*mut u32>, lporgnamebuf: Option<windows_core::PSTR>, pcchorgnamebuf: Option<*mut u32>, lpserialbuf: Option<windows_core::PSTR>, pcchserialbuf: Option<*mut u32>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoA(szproduct : windows_core::PCSTR, lpusernamebuf : windows_core::PSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_core::PSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_core::PSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoA(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetUserInfoW<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PWSTR>, pcchusernamebuf: Option<*mut u32>, lporgnamebuf: Option<windows_core::PWSTR>, pcchorgnamebuf: Option<*mut u32>, lpserialbuf: Option<windows_core::PWSTR>, pcchserialbuf: Option<*mut u32>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoW(szproduct : windows_core::PCWSTR, lpusernamebuf : windows_core::PWSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_core::PWSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_core::PWSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoW(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiInstallMissingComponentA<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiInstallMissingComponentA(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiInstallMissingComponentW<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiInstallMissingComponentW(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiInstallMissingFileA<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileA(szproduct : windows_core::PCSTR, szfile : windows_core::PCSTR) -> u32);
    unsafe { MsiInstallMissingFileA(szproduct.param().abi(), szfile.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallMissingFileW<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileW(szproduct : windows_core::PCWSTR, szfile : windows_core::PCWSTR) -> u32);
    unsafe { MsiInstallMissingFileW(szproduct.param().abi(), szfile.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallProductA<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductA(szpackagepath : windows_core::PCSTR, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiInstallProductA(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallProductW<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductW(szpackagepath : windows_core::PCWSTR, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiInstallProductW(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiIsProductElevatedA<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedA(szproduct : windows_core::PCSTR, pfelevated : *mut windows_core::BOOL) -> u32);
    unsafe { MsiIsProductElevatedA(szproduct.param().abi(), pfelevated as _) }
}
#[inline]
pub unsafe fn MsiIsProductElevatedW<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedW(szproduct : windows_core::PCWSTR, pfelevated : *mut windows_core::BOOL) -> u32);
    unsafe { MsiIsProductElevatedW(szproduct.param().abi(), pfelevated as _) }
}
#[inline]
pub unsafe fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiJoinTransaction(htransactionhandle : MSIHANDLE, dwtransactionattributes : u32, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { MsiJoinTransaction(htransactionhandle, dwtransactionattributes, phchangeofownerevent as _) }
}
#[inline]
pub unsafe fn MsiLocateComponentA<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentA(szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiLocateComponentA(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiLocateComponentW<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentW(szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiLocateComponentW(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiNotifySidChangeA<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeA(poldsid : windows_core::PCSTR, pnewsid : windows_core::PCSTR) -> u32);
    unsafe { MsiNotifySidChangeA(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[inline]
pub unsafe fn MsiNotifySidChangeW<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeW(poldsid : windows_core::PCWSTR, pnewsid : windows_core::PCWSTR) -> u32);
    unsafe { MsiNotifySidChangeW(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[inline]
pub unsafe fn MsiOpenDatabaseA<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseA(szdatabasepath : windows_core::PCSTR, szpersist : windows_core::PCSTR, phdatabase : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenDatabaseA(szdatabasepath.param().abi(), szpersist.param().abi(), phdatabase as _) }
}
#[inline]
pub unsafe fn MsiOpenDatabaseW<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenDatabaseW(szdatabasepath : windows_core::PCWSTR, szpersist : windows_core::PCWSTR, phdatabase : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenDatabaseW(szdatabasepath.param().abi(), szpersist.param().abi(), phdatabase as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageA<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageA(szpackagepath : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageA(szpackagepath.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageExA<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExA(szpackagepath : windows_core::PCSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageExA(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageExW<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExW(szpackagepath : windows_core::PCWSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageExW(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageW<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageW(szpackagepath : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageW(szpackagepath.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenProductA<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductA(szproduct : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenProductA(szproduct.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenProductW<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductW(szproduct : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenProductW(szproduct.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiPreviewBillboardA<P1, P2>(hpreview: MSIHANDLE, szcontrolname: P1, szbillboard: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardA(hpreview : MSIHANDLE, szcontrolname : windows_core::PCSTR, szbillboard : windows_core::PCSTR) -> u32);
    unsafe { MsiPreviewBillboardA(hpreview, szcontrolname.param().abi(), szbillboard.param().abi()) }
}
#[inline]
pub unsafe fn MsiPreviewBillboardW<P1, P2>(hpreview: MSIHANDLE, szcontrolname: P1, szbillboard: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewBillboardW(hpreview : MSIHANDLE, szcontrolname : windows_core::PCWSTR, szbillboard : windows_core::PCWSTR) -> u32);
    unsafe { MsiPreviewBillboardW(hpreview, szcontrolname.param().abi(), szbillboard.param().abi()) }
}
#[inline]
pub unsafe fn MsiPreviewDialogA<P1>(hpreview: MSIHANDLE, szdialogname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogA(hpreview : MSIHANDLE, szdialogname : windows_core::PCSTR) -> u32);
    unsafe { MsiPreviewDialogA(hpreview, szdialogname.param().abi()) }
}
#[inline]
pub unsafe fn MsiPreviewDialogW<P1>(hpreview: MSIHANDLE, szdialogname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiPreviewDialogW(hpreview : MSIHANDLE, szdialogname : windows_core::PCWSTR) -> u32);
    unsafe { MsiPreviewDialogW(hpreview, szdialogname.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::Registry::HKEY>, fshortcuts: bool, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptA(szscriptfile : windows_core::PCSTR, sziconfolder : windows_core::PCSTR, hregdata : super::Registry:: HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiProcessAdvertiseScriptA(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::Registry::HKEY>, fshortcuts: bool, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, sziconfolder : windows_core::PCWSTR, hregdata : super::Registry:: HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiProcessAdvertiseScriptW(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[inline]
pub unsafe fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiProcessMessage(hinstall : MSIHANDLE, emessagetype : INSTALLMESSAGE, hrecord : MSIHANDLE) -> i32);
    unsafe { MsiProcessMessage(hinstall, emessagetype, hrecord) }
}
#[inline]
pub unsafe fn MsiProvideAssemblyA<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyA(szassemblyname : windows_core::PCSTR, szappcontext : windows_core::PCSTR, dwinstallmode : u32, dwassemblyinfo : MSIASSEMBLYINFO, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideAssemblyA(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode.0 as _, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideAssemblyW<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyW(szassemblyname : windows_core::PCWSTR, szappcontext : windows_core::PCWSTR, dwinstallmode : u32, dwassemblyinfo : MSIASSEMBLYINFO, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideAssemblyW(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode.0 as _, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideComponentA<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, szcomponent : windows_core::PCSTR, dwinstallmode : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideComponentA(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode.0 as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideComponentW<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideComponentW(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode.0 as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode.0 as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P3, dwunused1: Option<u32>, dwunused2: Option<u32>, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : u32, szproduct : windows_core::PCSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentExA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode.0 as _, szproduct.param().abi(), dwunused1.unwrap_or(core::mem::zeroed()) as _, dwunused2.unwrap_or(core::mem::zeroed()) as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P3, dwunused1: Option<u32>, dwunused2: Option<u32>, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : u32, szproduct : windows_core::PCWSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentExW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode.0 as _, szproduct.param().abi(), dwunused1.unwrap_or(core::mem::zeroed()) as _, dwunused2.unwrap_or(core::mem::zeroed()) as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode.0 as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryComponentStateA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryComponentStateA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szcomponentcode.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryComponentStateW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryComponentStateW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szcomponentcode.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiQueryFeatureStateA(szproduct.param().abi(), szfeature.param().abi()) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryFeatureStateExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szfeature.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryFeatureStateExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szfeature.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiQueryFeatureStateW(szproduct.param().abi(), szfeature.param().abi()) }
}
#[inline]
pub unsafe fn MsiQueryProductStateA<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryProductStateA(szproduct : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiQueryProductStateA(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiQueryProductStateW<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryProductStateW(szproduct : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiQueryProductStateW(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordClearData(hrecord : MSIHANDLE) -> u32);
    unsafe { MsiRecordClearData(hrecord) }
}
#[inline]
pub unsafe fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordDataSize(hrecord : MSIHANDLE, ifield : u32) -> u32);
    unsafe { MsiRecordDataSize(hrecord, ifield) }
}
#[inline]
pub unsafe fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetFieldCount(hrecord : MSIHANDLE) -> u32);
    unsafe { MsiRecordGetFieldCount(hrecord) }
}
#[inline]
pub unsafe fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetInteger(hrecord : MSIHANDLE, ifield : u32) -> i32);
    unsafe { MsiRecordGetInteger(hrecord, ifield) }
}
#[inline]
pub unsafe fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringA(hrecord : MSIHANDLE, ifield : u32, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiRecordGetStringA(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordGetStringW(hrecord : MSIHANDLE, ifield : u32, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiRecordGetStringW(hrecord, ifield, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> windows_core::BOOL {
    windows_core::link!("msi.dll" "system" fn MsiRecordIsNull(hrecord : MSIHANDLE, ifield : u32) -> windows_core::BOOL);
    unsafe { MsiRecordIsNull(hrecord, ifield) }
}
#[inline]
pub unsafe fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: Option<windows_core::PSTR>, pcbdatabuf: *mut u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordReadStream(hrecord : MSIHANDLE, ifield : u32, szdatabuf : windows_core::PSTR, pcbdatabuf : *mut u32) -> u32);
    unsafe { MsiRecordReadStream(hrecord, ifield, szdatabuf.unwrap_or(core::mem::zeroed()) as _, pcbdatabuf as _) }
}
#[inline]
pub unsafe fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiRecordSetInteger(hrecord : MSIHANDLE, ifield : u32, ivalue : i32) -> u32);
    unsafe { MsiRecordSetInteger(hrecord, ifield, ivalue) }
}
#[inline]
pub unsafe fn MsiRecordSetStreamA<P2>(hrecord: MSIHANDLE, ifield: u32, szfilepath: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamA(hrecord : MSIHANDLE, ifield : u32, szfilepath : windows_core::PCSTR) -> u32);
    unsafe { MsiRecordSetStreamA(hrecord, ifield, szfilepath.param().abi()) }
}
#[inline]
pub unsafe fn MsiRecordSetStreamW<P2>(hrecord: MSIHANDLE, ifield: u32, szfilepath: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStreamW(hrecord : MSIHANDLE, ifield : u32, szfilepath : windows_core::PCWSTR) -> u32);
    unsafe { MsiRecordSetStreamW(hrecord, ifield, szfilepath.param().abi()) }
}
#[inline]
pub unsafe fn MsiRecordSetStringA<P2>(hrecord: MSIHANDLE, ifield: u32, szvalue: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringA(hrecord : MSIHANDLE, ifield : u32, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiRecordSetStringA(hrecord, ifield, szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiRecordSetStringW<P2>(hrecord: MSIHANDLE, ifield: u32, szvalue: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRecordSetStringW(hrecord : MSIHANDLE, ifield : u32, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiRecordSetStringW(hrecord, ifield, szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiReinstallFeatureA<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwreinstallmode : u32) -> u32);
    unsafe { MsiReinstallFeatureA(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode.0 as _) }
}
#[inline]
pub unsafe fn MsiReinstallFeatureW<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwreinstallmode : u32) -> u32);
    unsafe { MsiReinstallFeatureW(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode.0 as _) }
}
#[inline]
pub unsafe fn MsiReinstallProductA<P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductA(szproduct : windows_core::PCSTR, szreinstallmode : u32) -> u32);
    unsafe { MsiReinstallProductA(szproduct.param().abi(), szreinstallmode.0 as _) }
}
#[inline]
pub unsafe fn MsiReinstallProductW<P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductW(szproduct : windows_core::PCWSTR, szreinstallmode : u32) -> u32);
    unsafe { MsiReinstallProductW(szproduct.param().abi(), szreinstallmode.0 as _) }
}
#[inline]
pub unsafe fn MsiRemovePatchesA<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesA(szpatchlist : windows_core::PCSTR, szproductcode : windows_core::PCSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCSTR) -> u32);
    unsafe { MsiRemovePatchesA(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[inline]
pub unsafe fn MsiRemovePatchesW<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesW(szpatchlist : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCWSTR) -> u32);
    unsafe { MsiRemovePatchesW(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[inline]
pub unsafe fn MsiSequenceA<P1>(hinstall: MSIHANDLE, sztable: P1, isequencemode: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSequenceA(hinstall : MSIHANDLE, sztable : windows_core::PCSTR, isequencemode : i32) -> u32);
    unsafe { MsiSequenceA(hinstall, sztable.param().abi(), isequencemode) }
}
#[inline]
pub unsafe fn MsiSequenceW<P1>(hinstall: MSIHANDLE, sztable: P1, isequencemode: i32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSequenceW(hinstall : MSIHANDLE, sztable : windows_core::PCWSTR, isequencemode : i32) -> u32);
    unsafe { MsiSequenceW(hinstall, sztable.param().abi(), isequencemode) }
}
#[inline]
pub unsafe fn MsiSetComponentStateA<P1>(hinstall: MSIHANDLE, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateA(hinstall : MSIHANDLE, szcomponent : windows_core::PCSTR, istate : INSTALLSTATE) -> u32);
    unsafe { MsiSetComponentStateA(hinstall, szcomponent.param().abi(), istate) }
}
#[inline]
pub unsafe fn MsiSetComponentStateW<P1>(hinstall: MSIHANDLE, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetComponentStateW(hinstall : MSIHANDLE, szcomponent : windows_core::PCWSTR, istate : INSTALLSTATE) -> u32);
    unsafe { MsiSetComponentStateW(hinstall, szcomponent.param().abi(), istate) }
}
#[inline]
pub unsafe fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>) -> INSTALLUI_HANDLERA {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIA(puihandler : INSTALLUI_HANDLERA, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERA);
    unsafe { MsiSetExternalUIA(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIRecord(puihandler : PINSTALLUI_HANDLER_RECORD, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void, ppuiprevhandler : PINSTALLUI_HANDLER_RECORD) -> u32);
    unsafe { MsiSetExternalUIRecord(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _, ppuiprevhandler) }
}
#[inline]
pub unsafe fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>) -> INSTALLUI_HANDLERW {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIW(puihandler : INSTALLUI_HANDLERW, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERW);
    unsafe { MsiSetExternalUIW(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSetFeatureAttributesA<P1>(hinstall: MSIHANDLE, szfeature: P1, dwattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesA(hinstall : MSIHANDLE, szfeature : windows_core::PCSTR, dwattributes : u32) -> u32);
    unsafe { MsiSetFeatureAttributesA(hinstall, szfeature.param().abi(), dwattributes) }
}
#[inline]
pub unsafe fn MsiSetFeatureAttributesW<P1>(hinstall: MSIHANDLE, szfeature: P1, dwattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureAttributesW(hinstall : MSIHANDLE, szfeature : windows_core::PCWSTR, dwattributes : u32) -> u32);
    unsafe { MsiSetFeatureAttributesW(hinstall, szfeature.param().abi(), dwattributes) }
}
#[inline]
pub unsafe fn MsiSetFeatureStateA<P1>(hinstall: MSIHANDLE, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateA(hinstall : MSIHANDLE, szfeature : windows_core::PCSTR, istate : INSTALLSTATE) -> u32);
    unsafe { MsiSetFeatureStateA(hinstall, szfeature.param().abi(), istate) }
}
#[inline]
pub unsafe fn MsiSetFeatureStateW<P1>(hinstall: MSIHANDLE, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetFeatureStateW(hinstall : MSIHANDLE, szfeature : windows_core::PCWSTR, istate : INSTALLSTATE) -> u32);
    unsafe { MsiSetFeatureStateW(hinstall, szfeature.param().abi(), istate) }
}
#[inline]
pub unsafe fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetInstallLevel(hinstall : MSIHANDLE, iinstalllevel : i32) -> u32);
    unsafe { MsiSetInstallLevel(hinstall, iinstalllevel) }
}
#[inline]
pub unsafe fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: Option<*mut super::super::Foundation::HWND>) -> INSTALLUILEVEL {
    windows_core::link!("msi.dll" "system" fn MsiSetInternalUI(dwuilevel : INSTALLUILEVEL, phwnd : *mut super::super::Foundation:: HWND) -> INSTALLUILEVEL);
    unsafe { MsiSetInternalUI(dwuilevel, phwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: bool) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetMode(hinstall : MSIHANDLE, erunmode : MSIRUNMODE, fstate : windows_core::BOOL) -> u32);
    unsafe { MsiSetMode(hinstall, erunmode, fstate.into()) }
}
#[inline]
pub unsafe fn MsiSetPropertyA<P1, P2>(hinstall: MSIHANDLE, szname: P1, szvalue: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyA(hinstall : MSIHANDLE, szname : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSetPropertyA(hinstall, szname.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSetPropertyW<P1, P2>(hinstall: MSIHANDLE, szname: P1, szvalue: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetPropertyW(hinstall : MSIHANDLE, szname : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiSetPropertyW(hinstall, szname.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSetTargetPathA<P1, P2>(hinstall: MSIHANDLE, szfolder: P1, szfolderpath: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathA(hinstall : MSIHANDLE, szfolder : windows_core::PCSTR, szfolderpath : windows_core::PCSTR) -> u32);
    unsafe { MsiSetTargetPathA(hinstall, szfolder.param().abi(), szfolderpath.param().abi()) }
}
#[inline]
pub unsafe fn MsiSetTargetPathW<P1, P2>(hinstall: MSIHANDLE, szfolder: P1, szfolderpath: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSetTargetPathW(hinstall : MSIHANDLE, szfolder : windows_core::PCWSTR, szfolderpath : windows_core::PCWSTR) -> u32);
    unsafe { MsiSetTargetPathW(hinstall, szfolder.param().abi(), szfolderpath.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P5, szdiskprompt: P6) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_core::PCSTR, szdiskprompt : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListAddMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P5, szdiskprompt: P6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_core::PCWSTR, szdiskprompt : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListAddMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceA<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: Option<u32>, szsource: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32, szsource : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListAddSourceA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4, dwindex: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCSTR, dwindex : u32) -> u32);
    unsafe { MsiSourceListAddSourceExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4, dwindex: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCWSTR, dwindex : u32) -> u32);
    unsafe { MsiSourceListAddSourceExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceW<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: Option<u32>, szsource: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32, szsource : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListAddSourceW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListClearAllA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListClearAllExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListClearAllExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListClearAllW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    unsafe { MsiSourceListClearMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    unsafe { MsiSourceListClearMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[inline]
pub unsafe fn MsiSourceListClearSourceA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListClearSourceA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListClearSourceW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListClearSourceW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: Option<*mut u32>, szvolumelabel: Option<windows_core::PSTR>, pcchvolumelabel: Option<*mut u32>, szdiskprompt: Option<windows_core::PSTR>, pcchdiskprompt: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_core::PSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_core::PSTR, pcchdiskprompt : *mut u32) -> u32);
    unsafe { MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: Option<*mut u32>, szvolumelabel: Option<windows_core::PWSTR>, pcchvolumelabel: Option<*mut u32>, szdiskprompt: Option<windows_core::PWSTR>, pcchdiskprompt: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_core::PWSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_core::PWSTR, pcchdiskprompt : *mut u32) -> u32);
    unsafe { MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: Option<windows_core::PSTR>, pcchsource: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_core::PSTR, pcchsource : *mut u32) -> u32);
    unsafe { MsiSourceListEnumSourcesA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: Option<windows_core::PWSTR>, pcchsource: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_core::PWSTR, pcchsource : *mut u32) -> u32);
    unsafe { MsiSourceListEnumSourcesW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListForceResolutionA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListForceResolutionExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListForceResolutionExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListForceResolutionW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListGetInfoA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiSourceListGetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListGetInfoW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiSourceListGetInfoW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListSetInfoA<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListSetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListSetInfoW<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListSetInfoW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: Option<*mut super::super::Foundation::FILETIME>, szvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyA(hsummaryinfo : MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyA(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyCount(hsummaryinfo : MSIHANDLE, puipropertycount : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyCount(hsummaryinfo, puipropertycount as _) }
}
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: Option<*mut super::super::Foundation::FILETIME>, szvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyW(hsummaryinfo : MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiSummaryInfoGetPropertyW(hsummaryinfo, uiproperty, puidatatype as _, pivalue as _, pftvalue.unwrap_or(core::mem::zeroed()) as _, szvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoPersist(hsummaryinfo : MSIHANDLE) -> u32);
    unsafe { MsiSummaryInfoPersist(hsummaryinfo) }
}
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA<P5>(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyA(hsummaryinfo : MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSummaryInfoSetPropertyA(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW<P5>(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyW(hsummaryinfo : MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvalue : windows_core::PCWSTR) -> u32);
    unsafe { MsiSummaryInfoSetPropertyW(hsummaryinfo, uiproperty, uidatatype, ivalue, pftvalue as _, szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiUseFeatureA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiUseFeatureA(szproduct.param().abi(), szfeature.param().abi()) }
}
#[inline]
pub unsafe fn MsiUseFeatureExA<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: Option<u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExA(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiUseFeatureExW<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: Option<u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExW(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiUseFeatureW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiUseFeatureW(szproduct.param().abi(), szfeature.param().abi()) }
}
#[inline]
pub unsafe fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiVerifyDiskSpace(hinstall : MSIHANDLE) -> u32);
    unsafe { MsiVerifyDiskSpace(hinstall) }
}
#[inline]
pub unsafe fn MsiVerifyPackageA<P0>(szpackagepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageA(szpackagepath : windows_core::PCSTR) -> u32);
    unsafe { MsiVerifyPackageA(szpackagepath.param().abi()) }
}
#[inline]
pub unsafe fn MsiVerifyPackageW<P0>(szpackagepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageW(szpackagepath : windows_core::PCWSTR) -> u32);
    unsafe { MsiVerifyPackageW(szpackagepath.param().abi()) }
}
#[inline]
pub unsafe fn MsiViewClose(hview: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewClose(hview : MSIHANDLE) -> u32);
    unsafe { MsiViewClose(hview) }
}
#[inline]
pub unsafe fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewExecute(hview : MSIHANDLE, hrecord : MSIHANDLE) -> u32);
    unsafe { MsiViewExecute(hview, hrecord) }
}
#[inline]
pub unsafe fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewFetch(hview : MSIHANDLE, phrecord : *mut MSIHANDLE) -> u32);
    unsafe { MsiViewFetch(hview, phrecord as _) }
}
#[inline]
pub unsafe fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewGetColumnInfo(hview : MSIHANDLE, ecolumninfo : MSICOLINFO, phrecord : *mut MSIHANDLE) -> u32);
    unsafe { MsiViewGetColumnInfo(hview, ecolumninfo, phrecord as _) }
}
#[inline]
pub unsafe fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorA(hview : MSIHANDLE, szcolumnnamebuffer : windows_core::PSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    unsafe { MsiViewGetErrorA(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> MSIDBERROR {
    windows_core::link!("msi.dll" "system" fn MsiViewGetErrorW(hview : MSIHANDLE, szcolumnnamebuffer : windows_core::PWSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    unsafe { MsiViewGetErrorW(hview, szcolumnnamebuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiViewModify(hview : MSIHANDLE, emodifymode : MSIMODIFY, hrecord : MSIHANDLE) -> u32);
    unsafe { MsiViewModify(hview, emodifymode, hrecord) }
}
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(filebuffer: *mut core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, newfilecoffbase: u32, newfilecofftime: u32, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>) -> i32 {
    windows_core::link!("mspatcha.dll" "system" fn NormalizeFileForPatchSignature(filebuffer : *mut core::ffi::c_void, filesize : u32, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, newfilecoffbase : u32, newfilecofftime : u32, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE) -> i32);
    unsafe {
        NormalizeFileForPatchSignature(
            filebuffer as _,
            filesize,
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            newfilecoffbase,
            newfilecofftime,
            ignorerangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
}
#[inline]
pub unsafe fn QueryActCtxSettingsW<P2, P3>(dwflags: Option<u32>, hactctx: Option<super::super::Foundation::HANDLE>, settingsnamespace: P2, settingname: P3, pvbuffer: Option<windows_core::PWSTR>, dwbuffer: usize, pdwwrittenorrequired: Option<*mut usize>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryActCtxSettingsW(dwflags : u32, hactctx : super::super::Foundation:: HANDLE, settingsnamespace : windows_core::PCWSTR, settingname : windows_core::PCWSTR, pvbuffer : windows_core::PWSTR, dwbuffer : usize, pdwwrittenorrequired : *mut usize) -> windows_core::BOOL);
    unsafe { QueryActCtxSettingsW(dwflags.unwrap_or(core::mem::zeroed()) as _, hactctx.unwrap_or(core::mem::zeroed()) as _, settingsnamespace.param().abi(), settingname.param().abi(), pvbuffer.unwrap_or(core::mem::zeroed()) as _, dwbuffer, pdwwrittenorrequired.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn QueryActCtxW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: Option<*const core::ffi::c_void>, ulinfoclass: u32, pvbuffer: Option<*mut core::ffi::c_void>, cbbuffer: usize, pcbwrittenorrequired: Option<*mut usize>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn QueryActCtxW(dwflags : u32, hactctx : super::super::Foundation:: HANDLE, pvsubinstance : *const core::ffi::c_void, ulinfoclass : u32, pvbuffer : *mut core::ffi::c_void, cbbuffer : usize, pcbwrittenorrequired : *mut usize) -> windows_core::BOOL);
    unsafe { QueryActCtxW(dwflags, hactctx, pvsubinstance.unwrap_or(core::mem::zeroed()) as _, ulinfoclass, pvbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer, pcbwrittenorrequired.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseActCtx(hactctx : super::super::Foundation:: HANDLE));
    unsafe { ReleaseActCtx(hactctx as _) }
}
#[inline]
pub unsafe fn SfcGetNextProtectedFile(rpchandle: super::super::Foundation::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> windows_core::Result<()> {
    windows_core::link!("sfc.dll" "system" fn SfcGetNextProtectedFile(rpchandle : super::super::Foundation:: HANDLE, protfiledata : *mut PROTECTED_FILE_DATA) -> windows_core::BOOL);
    unsafe { SfcGetNextProtectedFile(rpchandle, protfiledata as _).ok() }
}
#[inline]
pub unsafe fn SfcIsFileProtected<P1>(rpchandle: super::super::Foundation::HANDLE, protfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("sfc.dll" "system" fn SfcIsFileProtected(rpchandle : super::super::Foundation:: HANDLE, protfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SfcIsFileProtected(rpchandle, protfilename.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SfcIsKeyProtected<P1>(keyhandle: super::Registry::HKEY, subkeyname: P1, keysam: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("sfc.dll" "system" fn SfcIsKeyProtected(keyhandle : super::Registry:: HKEY, subkeyname : windows_core::PCWSTR, keysam : u32) -> windows_core::BOOL);
    unsafe { SfcIsKeyProtected(keyhandle, subkeyname.param().abi(), keysam) }
}
#[inline]
pub unsafe fn SfpVerifyFile<P0>(pszfilename: P0, pszerror: &[u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("sfc.dll" "system" fn SfpVerifyFile(pszfilename : windows_core::PCSTR, pszerror : windows_core::PCSTR, dwerrsize : u32) -> windows_core::BOOL);
    unsafe { SfpVerifyFile(pszfilename.param().abi(), core::mem::transmute(pszerror.as_ptr()), pszerror.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileA<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(patchfilebuffer: &[u8], oldfilebuffer: Option<&[u8]>, newfilesize: Option<*mut u32>, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByBuffers(patchfilebuffer : *const u8, patchfilesize : u32, oldfilebuffer : *const u8, oldfilesize : u32, newfilesize : *mut u32, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByBuffers(core::mem::transmute(patchfilebuffer.as_ptr()), patchfilebuffer.len().try_into().unwrap(), core::mem::transmute(oldfilebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), oldfilebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), newfilesize.unwrap_or(core::mem::zeroed()) as _, applyoptionflags) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByHandles(patchfilehandle, oldfilehandle, applyoptionflags) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileW<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn ZombifyActCtx(hactctx: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ZombifyActCtx(hactctx : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { ZombifyActCtx(hactctx as _).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: windows_core::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: windows_core::PCSTR,
    pub lpResourceName: windows_core::PCSTR,
    pub lpApplicationName: windows_core::PCSTR,
    pub hModule: super::super::Foundation::HMODULE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: windows_core::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: windows_core::PCWSTR,
    pub lpResourceName: windows_core::PCWSTR,
    pub lpApplicationName: windows_core::PCWSTR,
    pub hModule: super::super::Foundation::HMODULE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTX_COMPATIBILITY_ELEMENT_TYPE(pub i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(3i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(2i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(1i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTX_REQUESTED_RUN_LEVEL(pub i32);
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(1i32);
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(2i32);
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(4i32);
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(3i32);
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(0i32);
#[repr(C)]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
impl Default for ACTCTX_SECTION_KEYED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lpAssemblyEncodedAssemblyIdentity: windows_core::PCWSTR,
    pub lpAssemblyManifestPath: windows_core::PCWSTR,
    pub lpAssemblyPolicyPath: windows_core::PCWSTR,
    pub lpAssemblyDirectoryName: windows_core::PCWSTR,
    pub ulFileCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub lpRootManifestPath: windows_core::PCWSTR,
    pub lpRootConfigurationPath: windows_core::PCWSTR,
    pub lpAppDirPath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADVERTISEFLAGS(pub i32);
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(0i32);
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(1i32);
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = ASM_BIND_FLAGS(8i32);
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = ASM_BIND_FLAGS(1i32);
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(32i32);
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(2i32);
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(4i32);
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASM_BIND_FLAGS(pub i32);
impl ASM_BIND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ASM_BIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ASM_BIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ASM_BIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = ASM_CMP_FLAGS(255i32);
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(8i32);
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = ASM_CMP_FLAGS(64i32);
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = ASM_CMP_FLAGS(128i32);
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = ASM_CMP_FLAGS(256i32);
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(2i32);
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(4i32);
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = ASM_CMP_FLAGS(1i32);
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = ASM_CMP_FLAGS(32i32);
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASM_CMP_FLAGS(pub i32);
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(2i32);
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(16i32);
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(64i32);
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(32i32);
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(8i32);
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(4i32);
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASM_DISPLAY_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASM_NAME(pub i32);
pub const ASM_NAME_ALIAS: ASM_NAME = ASM_NAME(12i32);
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = ASM_NAME(6i32);
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = ASM_NAME(14i32);
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = ASM_NAME(13i32);
pub const ASM_NAME_CULTURE: ASM_NAME = ASM_NAME(8i32);
pub const ASM_NAME_CUSTOM: ASM_NAME = ASM_NAME(17i32);
pub const ASM_NAME_HASH_ALGID: ASM_NAME = ASM_NAME(11i32);
pub const ASM_NAME_HASH_VALUE: ASM_NAME = ASM_NAME(2i32);
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = ASM_NAME(4i32);
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = ASM_NAME(20i32);
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = ASM_NAME(5i32);
pub const ASM_NAME_MVID: ASM_NAME = ASM_NAME(19i32);
pub const ASM_NAME_NAME: ASM_NAME = ASM_NAME(3i32);
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = ASM_NAME(18i32);
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = ASM_NAME(15i32);
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(16i32);
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = ASM_NAME(10i32);
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = ASM_NAME(9i32);
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = ASM_NAME(0i32);
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(1i32);
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = ASM_NAME(7i32);
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: windows_core::PCWSTR,
    pub lpFilePath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: windows_core::PWSTR,
    pub cchBuf: u32,
}
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(1i32);
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(2i32);
pub const CLSID_EvalCom2: windows_core::GUID = windows_core::GUID::from_u128(0x6e5e1910_8053_4660_b795_6b612e29bc58);
pub const CLSID_MsmMerge2: windows_core::GUID = windows_core::GUID::from_u128(0xf94985d5_29f9_4743_9805_99bc3f35b678);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: windows_core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_ASM_NAME_OBJ_FLAGS(pub i32);
pub const DEFAULT_DISK_ID: u32 = 2u32;
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl Default for DELTA_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: super::super::Foundation::FILETIME,
    pub TargetHashAlgId: super::super::Security::Cryptography::ALG_ID,
    pub TargetHash: DELTA_HASH,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: windows_core::BOOL,
}
impl Default for DELTA_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DELTA_INPUT_0 {
    pub lpcStart: *const core::ffi::c_void,
    pub lpStart: *mut core::ffi::c_void,
}
impl Default for DELTA_INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut core::ffi::c_void,
    pub uSize: usize,
}
impl Default for DELTA_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: windows_core::GUID,
    pub szIdentifier: windows_core::PCWSTR,
    pub szNonCannonicalData: windows_core::PCWSTR,
}
pub const FUSION_REFCOUNT_FILEPATH_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb02f9d65_fb77_4f7a_afa5_b391309f11c9);
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2ec93463_b0c3_45e1_8364_327e96aea856);
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8cedc215_ac4b_488b_93c0_a50a49cb2fb8);
pub const IACTIONNAME_ADMIN: windows_core::PCWSTR = windows_core::w!("ADMIN");
pub const IACTIONNAME_ADVERTISE: windows_core::PCWSTR = windows_core::w!("ADVERTISE");
pub const IACTIONNAME_COLLECTUSERINFO: windows_core::PCWSTR = windows_core::w!("CollectUserInfo");
pub const IACTIONNAME_FIRSTRUN: windows_core::PCWSTR = windows_core::w!("FirstRun");
pub const IACTIONNAME_INSTALL: windows_core::PCWSTR = windows_core::w!("INSTALL");
pub const IACTIONNAME_SEQUENCE: windows_core::PCWSTR = windows_core::w!("SEQUENCE");
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IASSEMBLYCACHE_UNINSTALL_DISPOSITION(pub u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(3u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(4u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(2u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(1u32);
windows_core::imp::define_interface!(IAssemblyCache, IAssemblyCache_Vtbl, 0xe707dcde_d1cd_11d2_bab9_00c04f8eceae);
windows_core::imp::interface_hierarchy!(IAssemblyCache, windows_core::IUnknown);
impl IAssemblyCache {
    pub unsafe fn UninstallAssembly<P1>(&self, dwflags: u32, pszassemblyname: P1, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UninstallAssembly)(windows_core::Interface::as_raw(self), dwflags, pszassemblyname.param().abi(), prefdata as _, puldisposition as _).ok() }
    }
    pub unsafe fn QueryAssemblyInfo<P1>(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: P1, pasminfo: *mut ASSEMBLY_INFO) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryAssemblyInfo)(windows_core::Interface::as_raw(self), dwflags, pszassemblyname.param().abi(), pasminfo as _).ok() }
    }
    pub unsafe fn CreateAssemblyCacheItem<P3>(&self, dwflags: u32, pvreserved: *mut core::ffi::c_void, ppasmitem: *mut Option<IAssemblyCacheItem>, pszassemblyname: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateAssemblyCacheItem)(windows_core::Interface::as_raw(self), dwflags, pvreserved as _, core::mem::transmute(ppasmitem), pszassemblyname.param().abi()).ok() }
    }
    pub unsafe fn Reserved(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reserved)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstallAssembly<P1>(&self, dwflags: u32, pszmanifestfilepath: P1, prefdata: *mut FUSION_INSTALL_REFERENCE) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstallAssembly)(windows_core::Interface::as_raw(self), dwflags, pszmanifestfilepath.param().abi(), prefdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UninstallAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut FUSION_INSTALL_REFERENCE, *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::HRESULT,
    pub QueryAssemblyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, QUERYASMINFO_FLAGS, windows_core::PCWSTR, *mut ASSEMBLY_INFO) -> windows_core::HRESULT,
    pub CreateAssemblyCacheItem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Reserved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut FUSION_INSTALL_REFERENCE) -> windows_core::HRESULT,
}
pub trait IAssemblyCache_Impl: windows_core::IUnknownImpl {
    fn UninstallAssembly(&self, dwflags: u32, pszassemblyname: &windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::Result<()>;
    fn QueryAssemblyInfo(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: &windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> windows_core::Result<()>;
    fn CreateAssemblyCacheItem(&self, dwflags: u32, pvreserved: *mut core::ffi::c_void, ppasmitem: windows_core::OutRef<IAssemblyCacheItem>, pszassemblyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Reserved(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn InstallAssembly(&self, dwflags: u32, pszmanifestfilepath: &windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> windows_core::Result<()>;
}
impl IAssemblyCache_Vtbl {
    pub const fn new<Identity: IAssemblyCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UninstallAssembly<Identity: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszassemblyname: windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCache_Impl::UninstallAssembly(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszassemblyname), core::mem::transmute_copy(&prefdata), core::mem::transmute_copy(&puldisposition)).into()
            }
        }
        unsafe extern "system" fn QueryAssemblyInfo<Identity: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCache_Impl::QueryAssemblyInfo(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszassemblyname), core::mem::transmute_copy(&pasminfo)).into()
            }
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Identity: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pvreserved: *mut core::ffi::c_void, ppasmitem: *mut *mut core::ffi::c_void, pszassemblyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCache_Impl::CreateAssemblyCacheItem(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&ppasmitem), core::mem::transmute(&pszassemblyname)).into()
            }
        }
        unsafe extern "system" fn Reserved<Identity: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAssemblyCache_Impl::Reserved(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallAssembly<Identity: IAssemblyCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszmanifestfilepath: windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCache_Impl::InstallAssembly(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszmanifestfilepath), core::mem::transmute_copy(&prefdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UninstallAssembly: UninstallAssembly::<Identity, OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Identity, OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Identity, OFFSET>,
            Reserved: Reserved::<Identity, OFFSET>,
            InstallAssembly: InstallAssembly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyCache as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAssemblyCache {}
windows_core::imp::define_interface!(IAssemblyCacheItem, IAssemblyCacheItem_Vtbl, 0x9e3aaeb4_d1cd_11d2_bab9_00c04f8eceae);
windows_core::imp::interface_hierarchy!(IAssemblyCacheItem, windows_core::IUnknown);
impl IAssemblyCacheItem {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream<P1>(&self, dwflags: u32, pszstreamname: P1, dwformat: u32, dwformatflags: u32, ppistream: *mut Option<super::Com::IStream>, pulimaxsize: *mut u64) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), dwflags, pszstreamname.param().abi(), dwformat, dwformatflags, core::mem::transmute(ppistream), pulimaxsize as _).ok() }
    }
    pub unsafe fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), dwflags, puldisposition as _).ok() }
    }
    pub unsafe fn AbortItem(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortItem)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCacheItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, *mut *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub AbortItem: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyCacheItem_Impl: windows_core::IUnknownImpl {
    fn CreateStream(&self, dwflags: u32, pszstreamname: &windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: windows_core::OutRef<super::Com::IStream>, pulimaxsize: *mut u64) -> windows_core::Result<()>;
    fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> windows_core::Result<()>;
    fn AbortItem(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyCacheItem_Vtbl {
    pub const fn new<Identity: IAssemblyCacheItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStream<Identity: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszstreamname: windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut core::ffi::c_void, pulimaxsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCacheItem_Impl::CreateStream(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszstreamname), core::mem::transmute_copy(&dwformat), core::mem::transmute_copy(&dwformatflags), core::mem::transmute_copy(&ppistream), core::mem::transmute_copy(&pulimaxsize)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCacheItem_Impl::Commit(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&puldisposition)).into()
            }
        }
        unsafe extern "system" fn AbortItem<Identity: IAssemblyCacheItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyCacheItem_Impl::AbortItem(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            AbortItem: AbortItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyCacheItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAssemblyCacheItem {}
windows_core::imp::define_interface!(IAssemblyName, IAssemblyName_Vtbl, 0xcd193bc0_b4bc_11d2_9833_00c04fc31d2e);
windows_core::imp::interface_hierarchy!(IAssemblyName, windows_core::IUnknown);
impl IAssemblyName {
    pub unsafe fn SetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, cbproperty: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, pvproperty as _, cbproperty).ok() }
    }
    pub unsafe fn GetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, pcbproperty: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, pvproperty as _, pcbproperty as _).ok() }
    }
    pub unsafe fn Finalize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDisplayName(&self, szdisplayname: Option<windows_core::PWSTR>, pccdisplayname: *mut u32, dwdisplayflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), szdisplayname.unwrap_or(core::mem::zeroed()) as _, pccdisplayname as _, dwdisplayflags).ok() }
    }
    pub unsafe fn Reserved<P1, P2, P3>(&self, refiid: *const windows_core::GUID, punkreserved1: P1, punkreserved2: P2, szreserved: P3, llreserved: i64, pvreserved: *mut core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reserved)(windows_core::Interface::as_raw(self), refiid, punkreserved1.param().abi(), punkreserved2.param().abi(), szreserved.param().abi(), llreserved, pvreserved as _, cbreserved, ppreserved as _).ok() }
    }
    pub unsafe fn GetName(&self, lpcwbuffer: *mut u32, pwzname: Option<windows_core::PWSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), lpcwbuffer as _, pwzname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pdwversionhi as _, pdwversionlow as _).ok() }
    }
    pub unsafe fn IsEqual<P0>(&self, pname: P0, dwcmpflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAssemblyName>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pname.param().abi(), dwcmpflags).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IAssemblyName> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32, u32) -> windows_core::HRESULT,
    pub Reserved: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, i64, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAssemblyName_Impl: windows_core::IUnknownImpl {
    fn SetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, cbproperty: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, pcbproperty: *mut u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn GetDisplayName(&self, szdisplayname: windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> windows_core::Result<()>;
    fn Reserved(&self, refiid: *const windows_core::GUID, punkreserved1: windows_core::Ref<windows_core::IUnknown>, punkreserved2: windows_core::Ref<windows_core::IUnknown>, szreserved: &windows_core::PCWSTR, llreserved: i64, pvreserved: *mut core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetName(&self, lpcwbuffer: *mut u32, pwzname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> windows_core::Result<()>;
    fn IsEqual(&self, pname: windows_core::Ref<IAssemblyName>, dwcmpflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAssemblyName>;
}
impl IAssemblyName_Vtbl {
    pub const fn new<Identity: IAssemblyName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: u32, pvproperty: *mut core::ffi::c_void, cbproperty: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pvproperty), core::mem::transmute_copy(&cbproperty)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: u32, pvproperty: *mut core::ffi::c_void, pcbproperty: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pvproperty), core::mem::transmute_copy(&pcbproperty)).into()
            }
        }
        unsafe extern "system" fn Finalize<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::Finalize(this).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdisplayname: windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::GetDisplayName(this, core::mem::transmute_copy(&szdisplayname), core::mem::transmute_copy(&pccdisplayname), core::mem::transmute_copy(&dwdisplayflags)).into()
            }
        }
        unsafe extern "system" fn Reserved<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, punkreserved1: *mut core::ffi::c_void, punkreserved2: *mut core::ffi::c_void, szreserved: windows_core::PCWSTR, llreserved: i64, pvreserved: *mut core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::Reserved(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&punkreserved1), core::mem::transmute_copy(&punkreserved2), core::mem::transmute(&szreserved), core::mem::transmute_copy(&llreserved), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&cbreserved), core::mem::transmute_copy(&ppreserved)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::GetName(this, core::mem::transmute_copy(&lpcwbuffer), core::mem::transmute_copy(&pwzname)).into()
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::GetVersion(this, core::mem::transmute_copy(&pdwversionhi), core::mem::transmute_copy(&pdwversionlow)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::ffi::c_void, dwcmpflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAssemblyName_Impl::IsEqual(this, core::mem::transmute_copy(&pname), core::mem::transmute_copy(&dwcmpflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IAssemblyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAssemblyName_Impl::Clone(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            Reserved: Reserved::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyName as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAssemblyName {}
windows_core::imp::define_interface!(IEnumMsmDependency, IEnumMsmDependency_Vtbl, 0x0adda82c_2c26_11d2_ad65_00a0c9af11a6);
windows_core::imp::interface_hierarchy!(IEnumMsmDependency, windows_core::IUnknown);
impl IEnumMsmDependency {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmdependencies: *mut Option<IMsmDependency>, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cfetch, core::mem::transmute(rgmsmdependencies), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, cskip: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cskip).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMsmDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cfetch: u32, rgmsmdependencies: windows_core::OutRef<IMsmDependency>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmDependency_Vtbl {
    pub const fn new<Identity: IEnumMsmDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmDependency_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgmsmdependencies), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmDependency_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmDependency_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumMsmDependency_Impl::Clone(this) {
                    Ok(ok__) => {
                        pemsmdependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmDependency as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumMsmDependency {}
windows_core::imp::define_interface!(IEnumMsmError, IEnumMsmError_Vtbl, 0x0adda829_2c26_11d2_ad65_00a0c9af11a6);
windows_core::imp::interface_hierarchy!(IEnumMsmError, windows_core::IUnknown);
impl IEnumMsmError {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmerrors: *mut Option<IMsmError>, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cfetch, core::mem::transmute(rgmsmerrors), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, cskip: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cskip).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMsmError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cfetch: u32, rgmsmerrors: windows_core::OutRef<IMsmError>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmError_Vtbl {
    pub const fn new<Identity: IEnumMsmError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmError_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgmsmerrors), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmError_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmError_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumMsmError_Impl::Clone(this) {
                    Ok(ok__) => {
                        pemsmerrors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumMsmError {}
windows_core::imp::define_interface!(IEnumMsmString, IEnumMsmString_Vtbl, 0x0adda826_2c26_11d2_ad65_00a0c9af11a6);
windows_core::imp::interface_hierarchy!(IEnumMsmString, windows_core::IUnknown);
impl IEnumMsmString {
    pub unsafe fn Next(&self, cfetch: u32, rgbstrstrings: *mut windows_core::BSTR, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cfetch, core::mem::transmute(rgbstrstrings), pcfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, cskip: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cskip).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMsmString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumMsmString_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cfetch: u32, rgbstrstrings: *mut windows_core::BSTR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmString>;
}
impl IEnumMsmString_Vtbl {
    pub const fn new<Identity: IEnumMsmString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmString_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgbstrstrings), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmString_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMsmString_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumMsmString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmstrings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumMsmString_Impl::Clone(this) {
                    Ok(ok__) => {
                        pemsmstrings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumMsmString {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmDependencies, IMsmDependencies_Vtbl, 0x0adda82d_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmDependencies {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmDependencies, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependencies {
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IMsmDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self, count: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), count as _).ok() }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependencies_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmDependencies_Impl: super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> windows_core::Result<IMsmDependency>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmDependencies_Vtbl {
    pub const fn new<Identity: IMsmDependencies_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmDependencies_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                    Ok(ok__) => {
                        r#return.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmDependencies_Impl::Count(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IMsmDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmDependencies_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmDependencies as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmDependencies {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmDependency, IMsmDependency_Vtbl, 0x0adda82b_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmDependency {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmDependency, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependency {
    pub unsafe fn Module(&self, module: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Module)(windows_core::Interface::as_raw(self), core::mem::transmute(module)).ok() }
    }
    pub unsafe fn Language(&self, language: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Language)(windows_core::Interface::as_raw(self), language as _).ok() }
    }
    pub unsafe fn Version(&self, version: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), core::mem::transmute(version)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependency_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Module: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmDependency_Impl: super::Com::IDispatch_Impl {
    fn Module(&self, module: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Language(&self, language: *mut i16) -> windows_core::Result<()>;
    fn Version(&self, version: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmDependency_Vtbl {
    pub const fn new<Identity: IMsmDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Module<Identity: IMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmDependency_Impl::Module(this, core::mem::transmute_copy(&module)).into()
            }
        }
        unsafe extern "system" fn Language<Identity: IMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmDependency_Impl::Language(this, core::mem::transmute_copy(&language)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IMsmDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmDependency_Impl::Version(this, core::mem::transmute_copy(&version)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Module: Module::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmDependency as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmError, IMsmError_Vtbl, 0x0adda828_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmError {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmError, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmError {
    pub unsafe fn Type(&self, errortype: *mut msmErrorType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), errortype as _).ok() }
    }
    pub unsafe fn Path(&self, errorpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), core::mem::transmute(errorpath)).ok() }
    }
    pub unsafe fn Language(&self, errorlanguage: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Language)(windows_core::Interface::as_raw(self), errorlanguage as _).ok() }
    }
    pub unsafe fn DatabaseTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DatabaseTable)(windows_core::Interface::as_raw(self), core::mem::transmute(errortable)).ok() }
    }
    pub unsafe fn DatabaseKeys(&self) -> windows_core::Result<IMsmStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DatabaseKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ModuleTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModuleTable)(windows_core::Interface::as_raw(self), core::mem::transmute(errortable)).ok() }
    }
    pub unsafe fn ModuleKeys(&self) -> windows_core::Result<IMsmStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmError_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut msmErrorType) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub DatabaseTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DatabaseKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModuleTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModuleKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmError_Impl: super::Com::IDispatch_Impl {
    fn Type(&self, errortype: *mut msmErrorType) -> windows_core::Result<()>;
    fn Path(&self, errorpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Language(&self, errorlanguage: *mut i16) -> windows_core::Result<()>;
    fn DatabaseTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DatabaseKeys(&self) -> windows_core::Result<IMsmStrings>;
    fn ModuleTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ModuleKeys(&self) -> windows_core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmError_Vtbl {
    pub const fn new<Identity: IMsmError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortype: *mut msmErrorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmError_Impl::Type(this, core::mem::transmute_copy(&errortype)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmError_Impl::Path(this, core::mem::transmute_copy(&errorpath)).into()
            }
        }
        unsafe extern "system" fn Language<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorlanguage: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmError_Impl::Language(this, core::mem::transmute_copy(&errorlanguage)).into()
            }
        }
        unsafe extern "system" fn DatabaseTable<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmError_Impl::DatabaseTable(this, core::mem::transmute_copy(&errortable)).into()
            }
        }
        unsafe extern "system" fn DatabaseKeys<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmError_Impl::DatabaseKeys(this) {
                    Ok(ok__) => {
                        errorkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModuleTable<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmError_Impl::ModuleTable(this, core::mem::transmute_copy(&errortable)).into()
            }
        }
        unsafe extern "system" fn ModuleKeys<Identity: IMsmError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmError_Impl::ModuleKeys(this) {
                    Ok(ok__) => {
                        errorkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            DatabaseTable: DatabaseTable::<Identity, OFFSET>,
            DatabaseKeys: DatabaseKeys::<Identity, OFFSET>,
            ModuleTable: ModuleTable::<Identity, OFFSET>,
            ModuleKeys: ModuleKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmError as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmError {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmErrors, IMsmErrors_Vtbl, 0x0adda82a_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmErrors {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmErrors, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrors {
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IMsmError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self, count: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), count as _).ok() }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmErrors_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmErrors_Impl: super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> windows_core::Result<IMsmError>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmErrors_Vtbl {
    pub const fn new<Identity: IMsmErrors_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IMsmErrors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmErrors_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                    Ok(ok__) => {
                        r#return.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IMsmErrors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmErrors_Impl::Count(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IMsmErrors_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmErrors_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmErrors as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmErrors {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmGetFiles, IMsmGetFiles_Vtbl, 0x7041ae26_2d78_11d2_888a_00a0c981b015);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmGetFiles {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmGetFiles, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmGetFiles {
    pub unsafe fn ModuleFiles(&self) -> windows_core::Result<IMsmStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmGetFiles_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ModuleFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmGetFiles_Impl: super::Com::IDispatch_Impl {
    fn ModuleFiles(&self) -> windows_core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmGetFiles_Vtbl {
    pub const fn new<Identity: IMsmGetFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleFiles<Identity: IMsmGetFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmGetFiles_Impl::ModuleFiles(this) {
                    Ok(ok__) => {
                        files.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ModuleFiles: ModuleFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmGetFiles as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmGetFiles {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmMerge, IMsmMerge_Vtbl, 0x0adda82e_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmMerge {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmMerge, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmMerge {
    pub unsafe fn OpenDatabase(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenDatabase)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn OpenModule(&self, path: &windows_core::BSTR, language: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenModule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), language).ok() }
    }
    pub unsafe fn CloseDatabase(&self, commit: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseDatabase)(windows_core::Interface::as_raw(self), commit).ok() }
    }
    pub unsafe fn CloseModule(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseModule)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OpenLog(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenLog)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn CloseLog(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseLog)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Log(&self, message: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(message)).ok() }
    }
    pub unsafe fn Errors(&self) -> windows_core::Result<IMsmErrors> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Errors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Dependencies(&self) -> windows_core::Result<IMsmDependencies> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Merge(&self, feature: &windows_core::BSTR, redirectdir: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Merge)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(feature), core::mem::transmute_copy(redirectdir)).ok() }
    }
    pub unsafe fn Connect(&self, feature: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(feature)).ok() }
    }
    pub unsafe fn ExtractCAB(&self, filename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExtractCAB)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)).ok() }
    }
    pub unsafe fn ExtractFiles(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExtractFiles)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmMerge_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OpenDatabase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub CloseDatabase: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CloseModule: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseLog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Errors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtractCAB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtractFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmMerge_Impl: super::Com::IDispatch_Impl {
    fn OpenDatabase(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OpenModule(&self, path: &windows_core::BSTR, language: i16) -> windows_core::Result<()>;
    fn CloseDatabase(&self, commit: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CloseModule(&self) -> windows_core::Result<()>;
    fn OpenLog(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CloseLog(&self) -> windows_core::Result<()>;
    fn Log(&self, message: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Errors(&self) -> windows_core::Result<IMsmErrors>;
    fn Dependencies(&self) -> windows_core::Result<IMsmDependencies>;
    fn Merge(&self, feature: &windows_core::BSTR, redirectdir: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Connect(&self, feature: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtractCAB(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtractFiles(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmMerge_Vtbl {
    pub const fn new<Identity: IMsmMerge_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenDatabase<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::OpenDatabase(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn OpenModule<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, language: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::OpenModule(this, core::mem::transmute(&path), core::mem::transmute_copy(&language)).into()
            }
        }
        unsafe extern "system" fn CloseDatabase<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commit: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::CloseDatabase(this, core::mem::transmute_copy(&commit)).into()
            }
        }
        unsafe extern "system" fn CloseModule<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::CloseModule(this).into()
            }
        }
        unsafe extern "system" fn OpenLog<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::OpenLog(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn CloseLog<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::CloseLog(this).into()
            }
        }
        unsafe extern "system" fn Log<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::Log(this, core::mem::transmute(&message)).into()
            }
        }
        unsafe extern "system" fn Errors<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmMerge_Impl::Errors(this) {
                    Ok(ok__) => {
                        errors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Dependencies<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmMerge_Impl::Dependencies(this) {
                    Ok(ok__) => {
                        dependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Merge<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: *mut core::ffi::c_void, redirectdir: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::Merge(this, core::mem::transmute(&feature), core::mem::transmute(&redirectdir)).into()
            }
        }
        unsafe extern "system" fn Connect<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::Connect(this, core::mem::transmute(&feature)).into()
            }
        }
        unsafe extern "system" fn ExtractCAB<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::ExtractCAB(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn ExtractFiles<Identity: IMsmMerge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmMerge_Impl::ExtractFiles(this, core::mem::transmute(&path)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, OFFSET>,
            OpenModule: OpenModule::<Identity, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, OFFSET>,
            CloseModule: CloseModule::<Identity, OFFSET>,
            OpenLog: OpenLog::<Identity, OFFSET>,
            CloseLog: CloseLog::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
            Errors: Errors::<Identity, OFFSET>,
            Dependencies: Dependencies::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            ExtractCAB: ExtractCAB::<Identity, OFFSET>,
            ExtractFiles: ExtractFiles::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmMerge as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmMerge {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMsmStrings, IMsmStrings_Vtbl, 0x0adda827_2c26_11d2_ad65_00a0c9af11a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMsmStrings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMsmStrings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMsmStrings {
    pub unsafe fn get_Item(&self, item: i32, r#return: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, core::mem::transmute(r#return)).ok() }
    }
    pub unsafe fn Count(&self, count: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), count as _).ok() }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmStrings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmStrings_Impl: super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32, r#return: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMsmStrings_Vtbl {
    pub const fn new<Identity: IMsmStrings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IMsmStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmStrings_Impl::get_Item(this, core::mem::transmute_copy(&item), core::mem::transmute_copy(&r#return)).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IMsmStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMsmStrings_Impl::Count(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IMsmStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMsmStrings_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmStrings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMsmStrings {}
pub const INFO_BASE: u32 = 3222229249u32;
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
pub const INFO_PCP_PATH: u32 = 3222229252u32;
pub const INFO_PROPERTY: u32 = 3222229255u32;
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLFEATUREATTRIBUTE(pub i32);
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(16i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(8i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(1i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(2i32);
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(4i32);
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLLEVEL(pub i32);
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = INSTALLLEVEL(0i32);
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = INSTALLLEVEL(65535i32);
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = INSTALLLEVEL(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLLOGATTRIBUTES(pub i32);
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(1i32);
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLLOGMODE(pub i32);
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLLOGMODE = INSTALLLOGMODE(512i32);
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLLOGMODE = INSTALLLOGMODE(256i32);
pub const INSTALLLOGMODE_COMMONDATA: INSTALLLOGMODE = INSTALLLOGMODE(2048i32);
pub const INSTALLLOGMODE_ERROR: INSTALLLOGMODE = INSTALLLOGMODE(2i32);
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLLOGMODE = INSTALLLOGMODE(8192i32);
pub const INSTALLLOGMODE_FATALEXIT: INSTALLLOGMODE = INSTALLLOGMODE(1i32);
pub const INSTALLLOGMODE_FILESINUSE: INSTALLLOGMODE = INSTALLLOGMODE(32i32);
pub const INSTALLLOGMODE_INFO: INSTALLLOGMODE = INSTALLLOGMODE(16i32);
pub const INSTALLLOGMODE_INITIALIZE: INSTALLLOGMODE = INSTALLLOGMODE(4096i32);
pub const INSTALLLOGMODE_INSTALLEND: INSTALLLOGMODE = INSTALLLOGMODE(134217728i32);
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLLOGMODE = INSTALLLOGMODE(67108864i32);
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLLOGMODE = INSTALLLOGMODE(16384i32);
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLLOGMODE = INSTALLLOGMODE(32768i32);
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLLOGMODE = INSTALLLOGMODE(128i32);
pub const INSTALLLOGMODE_PROGRESS: INSTALLLOGMODE = INSTALLLOGMODE(1024i32);
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLLOGMODE = INSTALLLOGMODE(1024i32);
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLLOGMODE = INSTALLLOGMODE(64i32);
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLLOGMODE = INSTALLLOGMODE(33554432i32);
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLLOGMODE = INSTALLLOGMODE(16384i32);
pub const INSTALLLOGMODE_TERMINATE: INSTALLLOGMODE = INSTALLLOGMODE(8192i32);
pub const INSTALLLOGMODE_USER: INSTALLLOGMODE = INSTALLLOGMODE(8i32);
pub const INSTALLLOGMODE_VERBOSE: INSTALLLOGMODE = INSTALLLOGMODE(4096i32);
pub const INSTALLLOGMODE_WARNING: INSTALLLOGMODE = INSTALLLOGMODE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLMESSAGE(pub i32);
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = INSTALLMESSAGE(150994944i32);
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = INSTALLMESSAGE(134217728i32);
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = INSTALLMESSAGE(184549376i32);
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = INSTALLMESSAGE(16777216i32);
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = INSTALLMESSAGE(0i32);
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(83886080i32);
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = INSTALLMESSAGE(67108864i32);
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = INSTALLMESSAGE(201326592i32);
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = INSTALLMESSAGE(452984832i32);
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = INSTALLMESSAGE(436207616i32);
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = INSTALLMESSAGE(117440512i32);
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = INSTALLMESSAGE(251658240i32);
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = INSTALLMESSAGE(167772160i32);
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = INSTALLMESSAGE(100663296i32);
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(419430400i32);
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = INSTALLMESSAGE(234881024i32);
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = INSTALLMESSAGE(218103808i32);
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = INSTALLMESSAGE(50331648i32);
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = INSTALLMESSAGE(33554432i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLMODE(pub i32);
pub const INSTALLMODE_DEFAULT: INSTALLMODE = INSTALLMODE(0i32);
pub const INSTALLMODE_EXISTING: INSTALLMODE = INSTALLMODE(-1i32);
pub const INSTALLMODE_NODETECTION: INSTALLMODE = INSTALLMODE(-2i32);
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = INSTALLMODE(-4i32);
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = INSTALLMODE(-3i32);
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: windows_core::PCWSTR = windows_core::w!("AssignmentType");
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: windows_core::PCWSTR = windows_core::w!("AuthorizedLUAApp");
pub const INSTALLPROPERTY_DISKPROMPT: windows_core::PCWSTR = windows_core::w!("DiskPrompt");
pub const INSTALLPROPERTY_DISPLAYNAME: windows_core::PCWSTR = windows_core::w!("DisplayName");
pub const INSTALLPROPERTY_HELPLINK: windows_core::PCWSTR = windows_core::w!("HelpLink");
pub const INSTALLPROPERTY_HELPTELEPHONE: windows_core::PCWSTR = windows_core::w!("HelpTelephone");
pub const INSTALLPROPERTY_INSTALLDATE: windows_core::PCWSTR = windows_core::w!("InstallDate");
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: windows_core::PCWSTR = windows_core::w!("InstalledLanguage");
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: windows_core::PCWSTR = windows_core::w!("InstalledProductName");
pub const INSTALLPROPERTY_INSTALLLOCATION: windows_core::PCWSTR = windows_core::w!("InstallLocation");
pub const INSTALLPROPERTY_INSTALLSOURCE: windows_core::PCWSTR = windows_core::w!("InstallSource");
pub const INSTALLPROPERTY_INSTANCETYPE: windows_core::PCWSTR = windows_core::w!("InstanceType");
pub const INSTALLPROPERTY_LANGUAGE: windows_core::PCWSTR = windows_core::w!("Language");
pub const INSTALLPROPERTY_LASTUSEDSOURCE: windows_core::PCWSTR = windows_core::w!("LastUsedSource");
pub const INSTALLPROPERTY_LASTUSEDTYPE: windows_core::PCWSTR = windows_core::w!("LastUsedType");
pub const INSTALLPROPERTY_LOCALPACKAGE: windows_core::PCWSTR = windows_core::w!("LocalPackage");
pub const INSTALLPROPERTY_LUAENABLED: windows_core::PCWSTR = windows_core::w!("LUAEnabled");
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: windows_core::PCWSTR = windows_core::w!("MediaPackagePath");
pub const INSTALLPROPERTY_MOREINFOURL: windows_core::PCWSTR = windows_core::w!("MoreInfoURL");
pub const INSTALLPROPERTY_PACKAGECODE: windows_core::PCWSTR = windows_core::w!("PackageCode");
pub const INSTALLPROPERTY_PACKAGENAME: windows_core::PCWSTR = windows_core::w!("PackageName");
pub const INSTALLPROPERTY_PATCHSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const INSTALLPROPERTY_PATCHTYPE: windows_core::PCWSTR = windows_core::w!("PatchType");
pub const INSTALLPROPERTY_PRODUCTICON: windows_core::PCWSTR = windows_core::w!("ProductIcon");
pub const INSTALLPROPERTY_PRODUCTID: windows_core::PCWSTR = windows_core::w!("ProductID");
pub const INSTALLPROPERTY_PRODUCTNAME: windows_core::PCWSTR = windows_core::w!("ProductName");
pub const INSTALLPROPERTY_PRODUCTSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const INSTALLPROPERTY_PUBLISHER: windows_core::PCWSTR = windows_core::w!("Publisher");
pub const INSTALLPROPERTY_REGCOMPANY: windows_core::PCWSTR = windows_core::w!("RegCompany");
pub const INSTALLPROPERTY_REGOWNER: windows_core::PCWSTR = windows_core::w!("RegOwner");
pub const INSTALLPROPERTY_TRANSFORMS: windows_core::PCWSTR = windows_core::w!("Transforms");
pub const INSTALLPROPERTY_UNINSTALLABLE: windows_core::PCWSTR = windows_core::w!("Uninstallable");
pub const INSTALLPROPERTY_URLINFOABOUT: windows_core::PCWSTR = windows_core::w!("URLInfoAbout");
pub const INSTALLPROPERTY_URLUPDATEINFO: windows_core::PCWSTR = windows_core::w!("URLUpdateInfo");
pub const INSTALLPROPERTY_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const INSTALLPROPERTY_VERSIONMAJOR: windows_core::PCWSTR = windows_core::w!("VersionMajor");
pub const INSTALLPROPERTY_VERSIONMINOR: windows_core::PCWSTR = windows_core::w!("VersionMinor");
pub const INSTALLPROPERTY_VERSIONSTRING: windows_core::PCWSTR = windows_core::w!("VersionString");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLSTATE(pub i32);
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = INSTALLSTATE(2i32);
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = INSTALLSTATE(1i32);
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = INSTALLSTATE(-6i32);
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = INSTALLSTATE(0i32);
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = INSTALLSTATE(5i32);
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = INSTALLSTATE(-5i32);
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = INSTALLSTATE(-2i32);
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = INSTALLSTATE(3i32);
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = INSTALLSTATE(-3i32);
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = INSTALLSTATE(-7i32);
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = INSTALLSTATE(1i32);
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = INSTALLSTATE(4i32);
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = INSTALLSTATE(-4i32);
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = INSTALLSTATE(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLTYPE(pub i32);
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = INSTALLTYPE(0i32);
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = INSTALLTYPE(1i32);
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = INSTALLTYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTALLUILEVEL(pub i32);
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = INSTALLUILEVEL(3i32);
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = INSTALLUILEVEL(1i32);
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = INSTALLUILEVEL(128i32);
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = INSTALLUILEVEL(5i32);
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = INSTALLUILEVEL(32i32);
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = INSTALLUILEVEL(0i32);
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = INSTALLUILEVEL(2i32);
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = INSTALLUILEVEL(64i32);
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = INSTALLUILEVEL(4i32);
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = INSTALLUILEVEL(256i32);
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = INSTALLUILEVEL(512i32);
pub type INSTALLUI_HANDLERA = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_core::PCSTR) -> i32>;
pub type INSTALLUI_HANDLERW = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_core::PCWSTR) -> i32>;
windows_core::imp::define_interface!(IPMApplicationInfo, IPMApplicationInfo_Vtbl, 0x50afb58a_438c_4088_9789_f8c4899829c7);
windows_core::imp::interface_hierarchy!(IPMApplicationInfo, windows_core::IUnknown);
impl IPMApplicationInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstanceID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstanceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OfferID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfferID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultTask(&self, pdefaulttask: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DefaultTask)(windows_core::Interface::as_raw(self), core::mem::transmute(pdefaulttask)).ok() }
    }
    pub unsafe fn AppTitle(&self, papptitle: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AppTitle)(windows_core::Interface::as_raw(self), core::mem::transmute(papptitle)).ok() }
    }
    pub unsafe fn IconPath(&self, pappiconpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IconPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pappiconpath)).ok() }
    }
    pub unsafe fn NotificationState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AppInstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppInstallType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<PM_APPLICATION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsRevoked(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRevoked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateAvailable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstallDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsUninstallable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUninstallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsThemable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThemable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsTrial(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTrial)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstallPath(&self, pinstallpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstallpath)).ok() }
    }
    pub unsafe fn DataRoot(&self, pdataroot: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DataRoot)(windows_core::Interface::as_raw(self), core::mem::transmute(pdataroot)).ok() }
    }
    pub unsafe fn Genre(&self) -> windows_core::Result<PM_APP_GENRE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Genre)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Publisher(&self, ppublisher: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Publisher)(windows_core::Interface::as_raw(self), core::mem::transmute(ppublisher)).ok() }
    }
    pub unsafe fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), core::mem::transmute(pauthor)).ok() }
    }
    pub unsafe fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), core::mem::transmute(pdescription)).ok() }
    }
    pub unsafe fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), core::mem::transmute(pversion)).ok() }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
    pub unsafe fn AppPlatMajorVersion(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppPlatMajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AppPlatMinorVersion(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppPlatMinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsMultiCore(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMultiCore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SID(&self, psid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SID)(windows_core::Interface::as_raw(self), core::mem::transmute(psid)).ok() }
    }
    pub unsafe fn AppPlatMajorVersionLightUp(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppPlatMajorVersionLightUp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AppPlatMinorVersionLightUp(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppPlatMinorVersionLightUp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_UpdateAvailable(&self, isupdateavailable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_UpdateAvailable)(windows_core::Interface::as_raw(self), isupdateavailable.into()).ok() }
    }
    pub unsafe fn set_NotificationState(&self, isnotified: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_NotificationState)(windows_core::Interface::as_raw(self), isnotified.into()).ok() }
    }
    pub unsafe fn set_IconPath(&self, appiconpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IconPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(appiconpath)).ok() }
    }
    pub unsafe fn set_UninstallableState(&self, isuninstallable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_UninstallableState)(windows_core::Interface::as_raw(self), isuninstallable.into()).ok() }
    }
    pub unsafe fn IsPinableOnKidZone(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPinableOnKidZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOriginallyPreInstalled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOriginallyPreInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsInstallOnSD(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInstallOnSD)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOptoutOnSD(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOptoutOnSD)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOptoutBackupRestore(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOptoutBackupRestore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_EnterpriseDisabled(&self, isdisabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_EnterpriseDisabled)(windows_core::Interface::as_raw(self), isdisabled.into()).ok() }
    }
    pub unsafe fn set_EnterpriseUninstallable(&self, isuninstallable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_EnterpriseUninstallable)(windows_core::Interface::as_raw(self), isuninstallable.into()).ok() }
    }
    pub unsafe fn EnterpriseDisabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnterpriseDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnterpriseUninstallable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnterpriseUninstallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsVisibleOnAppList(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVisibleOnAppList)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsInboxApp(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInboxApp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StorageID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StorageID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartAppBlob)(windows_core::Interface::as_raw(self), core::mem::transmute(pblob)).ok() }
    }
    pub unsafe fn IsMovable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMovable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeploymentAppEnumerationHubFilter(&self) -> windows_core::Result<PM_TILE_HUBTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeploymentAppEnumerationHubFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ModifiedDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifiedDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOriginallyRestored(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOriginallyRestored)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShouldDeferMdilBind(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldDeferMdilBind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsFullyPreInstall(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFullyPreInstall)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_IsMdilMaintenanceNeeded(&self, fismdilmaintenanceneeded: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IsMdilMaintenanceNeeded)(windows_core::Interface::as_raw(self), fismdilmaintenanceneeded.into()).ok() }
    }
    pub unsafe fn set_Title(&self, apptitle: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_Title)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(apptitle)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub InstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OfferID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DefaultTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotificationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AppInstallType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_APPLICATION_STATE) -> windows_core::HRESULT,
    pub IsRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub UpdateAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub InstallDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub IsUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsThemable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub InstallPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Genre: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_APP_GENRE) -> windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppPlatMajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AppPlatMinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsMultiCore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppPlatMajorVersionLightUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub AppPlatMinorVersionLightUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub set_UpdateAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_NotificationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub set_UninstallableState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPinableOnKidZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsOriginallyPreInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsInstallOnSD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsOptoutOnSD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsOptoutBackupRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub set_EnterpriseDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_EnterpriseUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub EnterpriseDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub EnterpriseUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsVisibleOnAppList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsInboxApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub StorageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub StartAppBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_STARTAPPBLOB) -> windows_core::HRESULT,
    pub IsMovable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DeploymentAppEnumerationHubFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_TILE_HUBTYPE) -> windows_core::HRESULT,
    pub ModifiedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub IsOriginallyRestored: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ShouldDeferMdilBind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsFullyPreInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IsMdilMaintenanceNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMApplicationInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn InstanceID(&self) -> windows_core::Result<windows_core::GUID>;
    fn OfferID(&self) -> windows_core::Result<windows_core::GUID>;
    fn DefaultTask(&self, pdefaulttask: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppTitle(&self, papptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IconPath(&self, pappiconpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NotificationState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AppInstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(&self) -> windows_core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(&self) -> windows_core::Result<windows_core::BOOL>;
    fn UpdateAvailable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn InstallDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsThemable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsTrial(&self) -> windows_core::Result<windows_core::BOOL>;
    fn InstallPath(&self, pinstallpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DataRoot(&self, pdataroot: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Genre(&self) -> windows_core::Result<PM_APP_GENRE>;
    fn Publisher(&self, ppublisher: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppPlatMajorVersion(&self) -> windows_core::Result<u8>;
    fn AppPlatMinorVersion(&self) -> windows_core::Result<u8>;
    fn PublisherID(&self) -> windows_core::Result<windows_core::GUID>;
    fn IsMultiCore(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SID(&self, psid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppPlatMajorVersionLightUp(&self) -> windows_core::Result<u8>;
    fn AppPlatMinorVersionLightUp(&self) -> windows_core::Result<u8>;
    fn set_UpdateAvailable(&self, isupdateavailable: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_NotificationState(&self, isnotified: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_IconPath(&self, appiconpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn set_UninstallableState(&self, isuninstallable: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsPinableOnKidZone(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsOriginallyPreInstalled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsInstallOnSD(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsOptoutOnSD(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsOptoutBackupRestore(&self) -> windows_core::Result<windows_core::BOOL>;
    fn set_EnterpriseDisabled(&self, isdisabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_EnterpriseUninstallable(&self, isuninstallable: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnterpriseDisabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn EnterpriseUninstallable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsVisibleOnAppList(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsInboxApp(&self) -> windows_core::Result<windows_core::BOOL>;
    fn StorageID(&self) -> windows_core::Result<windows_core::GUID>;
    fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> windows_core::Result<()>;
    fn IsMovable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DeploymentAppEnumerationHubFilter(&self) -> windows_core::Result<PM_TILE_HUBTYPE>;
    fn ModifiedDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsOriginallyRestored(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ShouldDeferMdilBind(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsFullyPreInstall(&self) -> windows_core::Result<windows_core::BOOL>;
    fn set_IsMdilMaintenanceNeeded(&self, fismdilmaintenanceneeded: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_Title(&self, apptitle: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMApplicationInfo_Vtbl {
    pub const fn new<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstanceID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::InstanceID(this) {
                    Ok(ok__) => {
                        pinstanceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OfferID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pofferid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::OfferID(this) {
                    Ok(ok__) => {
                        pofferid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultTask<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaulttask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::DefaultTask(this, core::mem::transmute_copy(&pdefaulttask)).into()
            }
        }
        unsafe extern "system" fn AppTitle<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papptitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::AppTitle(this, core::mem::transmute_copy(&papptitle)).into()
            }
        }
        unsafe extern "system" fn IconPath<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappiconpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::IconPath(this, core::mem::transmute_copy(&pappiconpath)).into()
            }
        }
        unsafe extern "system" fn NotificationState<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisnotified: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::NotificationState(this) {
                    Ok(ok__) => {
                        pisnotified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AppInstallType<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::AppInstallType(this) {
                    Ok(ok__) => {
                        pappinstalltype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRevoked<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrevoked: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsRevoked(this) {
                    Ok(ok__) => {
                        pisrevoked.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateAvailable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisupdateavailable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::UpdateAvailable(this) {
                    Ok(ok__) => {
                        pisupdateavailable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallDate<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::InstallDate(this) {
                    Ok(ok__) => {
                        pinstalldate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisuninstallable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsUninstallable(this) {
                    Ok(ok__) => {
                        pisuninstallable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsThemable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisthemable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsThemable(this) {
                    Ok(ok__) => {
                        pisthemable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsTrial<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistrial: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsTrial(this) {
                    Ok(ok__) => {
                        pistrial.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallPath<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::InstallPath(this, core::mem::transmute_copy(&pinstallpath)).into()
            }
        }
        unsafe extern "system" fn DataRoot<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::DataRoot(this, core::mem::transmute_copy(&pdataroot)).into()
            }
        }
        unsafe extern "system" fn Genre<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::Genre(this) {
                    Ok(ok__) => {
                        pgenre.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Publisher<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublisher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::Publisher(this, core::mem::transmute_copy(&ppublisher)).into()
            }
        }
        unsafe extern "system" fn Author<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::Author(this, core::mem::transmute_copy(&pauthor)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::Version(this, core::mem::transmute_copy(&pversion)).into()
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn AppPlatMajorVersion<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajorver: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::AppPlatMajorVersion(this) {
                    Ok(ok__) => {
                        pmajorver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminorver: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::AppPlatMinorVersion(this) {
                    Ok(ok__) => {
                        pminorver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PublisherID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublisherid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::PublisherID(this) {
                    Ok(ok__) => {
                        ppublisherid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMultiCore<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pismulticore: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsMultiCore(this) {
                    Ok(ok__) => {
                        pismulticore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::SID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajorver: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::AppPlatMajorVersionLightUp(this) {
                    Ok(ok__) => {
                        pmajorver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminorver: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::AppPlatMinorVersionLightUp(this) {
                    Ok(ok__) => {
                        pminorver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isupdateavailable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_UpdateAvailable(this, core::mem::transmute_copy(&isupdateavailable)).into()
            }
        }
        unsafe extern "system" fn set_NotificationState<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnotified: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_NotificationState(this, core::mem::transmute_copy(&isnotified)).into()
            }
        }
        unsafe extern "system" fn set_IconPath<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appiconpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_IconPath(this, core::mem::transmute(&appiconpath)).into()
            }
        }
        unsafe extern "system" fn set_UninstallableState<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_UninstallableState(this, core::mem::transmute_copy(&isuninstallable)).into()
            }
        }
        unsafe extern "system" fn IsPinableOnKidZone<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pispinable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsPinableOnKidZone(this) {
                    Ok(ok__) => {
                        pispinable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pispreinstalled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsOriginallyPreInstalled(this) {
                    Ok(ok__) => {
                        pispreinstalled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinstallonsd: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsInstallOnSD(this) {
                    Ok(ok__) => {
                        pisinstallonsd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptoutonsd: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsOptoutOnSD(this) {
                    Ok(ok__) => {
                        pisoptoutonsd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptoutbackuprestore: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsOptoutBackupRestore(this) {
                    Ok(ok__) => {
                        pisoptoutbackuprestore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdisabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_EnterpriseDisabled(this, core::mem::transmute_copy(&isdisabled)).into()
            }
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_EnterpriseUninstallable(this, core::mem::transmute_copy(&isuninstallable)).into()
            }
        }
        unsafe extern "system" fn EnterpriseDisabled<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdisabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::EnterpriseDisabled(this) {
                    Ok(ok__) => {
                        isdisabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::EnterpriseUninstallable(this) {
                    Ok(ok__) => {
                        isuninstallable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisvisible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsVisibleOnAppList(this) {
                    Ok(ok__) => {
                        pisvisible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInboxApp<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinboxapp: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsInboxApp(this) {
                    Ok(ok__) => {
                        pisinboxapp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StorageID<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorageid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::StorageID(this) {
                    Ok(ok__) => {
                        pstorageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAppBlob<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::StartAppBlob(this, core::mem::transmute_copy(&pblob)).into()
            }
        }
        unsafe extern "system" fn IsMovable<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pismovable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsMovable(this) {
                    Ok(ok__) => {
                        pismovable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::DeploymentAppEnumerationHubFilter(this) {
                    Ok(ok__) => {
                        hubtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifiedDate<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::ModifiedDate(this) {
                    Ok(ok__) => {
                        pmodifieddate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrestored: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsOriginallyRestored(this) {
                    Ok(ok__) => {
                        pisrestored.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdefermdilbind: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::ShouldDeferMdilBind(this) {
                    Ok(ok__) => {
                        pfdefermdilbind.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisfullypreinstall: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfo_Impl::IsFullyPreInstall(this) {
                    Ok(ok__) => {
                        pfisfullypreinstall.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fismdilmaintenanceneeded: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_IsMdilMaintenanceNeeded(this, core::mem::transmute_copy(&fismdilmaintenanceneeded)).into()
            }
        }
        unsafe extern "system" fn set_Title<Identity: IPMApplicationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apptitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMApplicationInfo_Impl::set_Title(this, core::mem::transmute(&apptitle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            InstanceID: InstanceID::<Identity, OFFSET>,
            OfferID: OfferID::<Identity, OFFSET>,
            DefaultTask: DefaultTask::<Identity, OFFSET>,
            AppTitle: AppTitle::<Identity, OFFSET>,
            IconPath: IconPath::<Identity, OFFSET>,
            NotificationState: NotificationState::<Identity, OFFSET>,
            AppInstallType: AppInstallType::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            IsRevoked: IsRevoked::<Identity, OFFSET>,
            UpdateAvailable: UpdateAvailable::<Identity, OFFSET>,
            InstallDate: InstallDate::<Identity, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, OFFSET>,
            IsThemable: IsThemable::<Identity, OFFSET>,
            IsTrial: IsTrial::<Identity, OFFSET>,
            InstallPath: InstallPath::<Identity, OFFSET>,
            DataRoot: DataRoot::<Identity, OFFSET>,
            Genre: Genre::<Identity, OFFSET>,
            Publisher: Publisher::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            AppPlatMajorVersion: AppPlatMajorVersion::<Identity, OFFSET>,
            AppPlatMinorVersion: AppPlatMinorVersion::<Identity, OFFSET>,
            PublisherID: PublisherID::<Identity, OFFSET>,
            IsMultiCore: IsMultiCore::<Identity, OFFSET>,
            SID: SID::<Identity, OFFSET>,
            AppPlatMajorVersionLightUp: AppPlatMajorVersionLightUp::<Identity, OFFSET>,
            AppPlatMinorVersionLightUp: AppPlatMinorVersionLightUp::<Identity, OFFSET>,
            set_UpdateAvailable: set_UpdateAvailable::<Identity, OFFSET>,
            set_NotificationState: set_NotificationState::<Identity, OFFSET>,
            set_IconPath: set_IconPath::<Identity, OFFSET>,
            set_UninstallableState: set_UninstallableState::<Identity, OFFSET>,
            IsPinableOnKidZone: IsPinableOnKidZone::<Identity, OFFSET>,
            IsOriginallyPreInstalled: IsOriginallyPreInstalled::<Identity, OFFSET>,
            IsInstallOnSD: IsInstallOnSD::<Identity, OFFSET>,
            IsOptoutOnSD: IsOptoutOnSD::<Identity, OFFSET>,
            IsOptoutBackupRestore: IsOptoutBackupRestore::<Identity, OFFSET>,
            set_EnterpriseDisabled: set_EnterpriseDisabled::<Identity, OFFSET>,
            set_EnterpriseUninstallable: set_EnterpriseUninstallable::<Identity, OFFSET>,
            EnterpriseDisabled: EnterpriseDisabled::<Identity, OFFSET>,
            EnterpriseUninstallable: EnterpriseUninstallable::<Identity, OFFSET>,
            IsVisibleOnAppList: IsVisibleOnAppList::<Identity, OFFSET>,
            IsInboxApp: IsInboxApp::<Identity, OFFSET>,
            StorageID: StorageID::<Identity, OFFSET>,
            StartAppBlob: StartAppBlob::<Identity, OFFSET>,
            IsMovable: IsMovable::<Identity, OFFSET>,
            DeploymentAppEnumerationHubFilter: DeploymentAppEnumerationHubFilter::<Identity, OFFSET>,
            ModifiedDate: ModifiedDate::<Identity, OFFSET>,
            IsOriginallyRestored: IsOriginallyRestored::<Identity, OFFSET>,
            ShouldDeferMdilBind: ShouldDeferMdilBind::<Identity, OFFSET>,
            IsFullyPreInstall: IsFullyPreInstall::<Identity, OFFSET>,
            set_IsMdilMaintenanceNeeded: set_IsMdilMaintenanceNeeded::<Identity, OFFSET>,
            set_Title: set_Title::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMApplicationInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMApplicationInfo {}
windows_core::imp::define_interface!(IPMApplicationInfoEnumerator, IPMApplicationInfoEnumerator_Vtbl, 0x0ec42a96_4d46_4dc6_a3d9_a7acaac0f5fa);
windows_core::imp::interface_hierarchy!(IPMApplicationInfoEnumerator, windows_core::IUnknown);
impl IPMApplicationInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMApplicationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMApplicationInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMApplicationInfo>;
}
impl IPMApplicationInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMApplicationInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMApplicationInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        ppappinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMApplicationInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMApplicationInfoEnumerator {}
windows_core::imp::define_interface!(IPMBackgroundServiceAgentInfo, IPMBackgroundServiceAgentInfo_Vtbl, 0x3a8b46da_928c_4879_998c_09dc96f3d490);
windows_core::imp::interface_hierarchy!(IPMBackgroundServiceAgentInfo, windows_core::IUnknown);
impl IPMBackgroundServiceAgentInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TaskID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptaskid)).ok() }
    }
    pub unsafe fn BSAID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BSAID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BGSpecifier(&self, pbgspecifier: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BGSpecifier)(windows_core::Interface::as_raw(self), core::mem::transmute(pbgspecifier)).ok() }
    }
    pub unsafe fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BGName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbgname)).ok() }
    }
    pub unsafe fn BGSource(&self, pbgsource: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BGSource)(windows_core::Interface::as_raw(self), core::mem::transmute(pbgsource)).ok() }
    }
    pub unsafe fn BGType(&self, pbgtype: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BGType)(windows_core::Interface::as_raw(self), core::mem::transmute(pbgtype)).ok() }
    }
    pub unsafe fn IsPeriodic(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPeriodic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsScheduled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScheduled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsScheduleAllowed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScheduleAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), core::mem::transmute(pdescription)).ok() }
    }
    pub unsafe fn IsLaunchOnBoot(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLaunchOnBoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_IsScheduled(&self, isscheduled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IsScheduled)(windows_core::Interface::as_raw(self), isscheduled.into()).ok() }
    }
    pub unsafe fn set_IsScheduleAllowed(&self, isscheduleallowed: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IsScheduleAllowed)(windows_core::Interface::as_raw(self), isscheduleallowed.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BSAID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BGSpecifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BGName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BGSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BGType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPeriodic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsScheduled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsScheduleAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLaunchOnBoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IsScheduled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IsScheduleAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMBackgroundServiceAgentInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BSAID(&self) -> windows_core::Result<u32>;
    fn BGSpecifier(&self, pbgspecifier: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGSource(&self, pbgsource: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGType(&self, pbgtype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsPeriodic(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsScheduled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsScheduleAllowed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsLaunchOnBoot(&self) -> windows_core::Result<windows_core::BOOL>;
    fn set_IsScheduled(&self, isscheduled: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_IsScheduleAllowed(&self, isscheduleallowed: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IPMBackgroundServiceAgentInfo_Vtbl {
    pub const fn new<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TaskID<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
            }
        }
        unsafe extern "system" fn BSAID<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsaid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::BSAID(this) {
                    Ok(ok__) => {
                        pbsaid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BGSpecifier<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgspecifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::BGSpecifier(this, core::mem::transmute_copy(&pbgspecifier)).into()
            }
        }
        unsafe extern "system" fn BGName<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::BGName(this, core::mem::transmute_copy(&pbgname)).into()
            }
        }
        unsafe extern "system" fn BGSource<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::BGSource(this, core::mem::transmute_copy(&pbgsource)).into()
            }
        }
        unsafe extern "system" fn BGType<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::BGType(this, core::mem::transmute_copy(&pbgtype)).into()
            }
        }
        unsafe extern "system" fn IsPeriodic<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisperiodic: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::IsPeriodic(this) {
                    Ok(ok__) => {
                        pisperiodic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsScheduled<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisscheduled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::IsScheduled(this) {
                    Ok(ok__) => {
                        pisscheduled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisscheduleallowed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::IsScheduleAllowed(this) {
                    Ok(ok__) => {
                        pisscheduleallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
            }
        }
        unsafe extern "system" fn IsLaunchOnBoot<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plaunchonboot: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfo_Impl::IsLaunchOnBoot(this) {
                    Ok(ok__) => {
                        plaunchonboot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_IsScheduled<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isscheduled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::set_IsScheduled(this, core::mem::transmute_copy(&isscheduled)).into()
            }
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Identity: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isscheduleallowed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundServiceAgentInfo_Impl::set_IsScheduleAllowed(this, core::mem::transmute_copy(&isscheduleallowed)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            BSAID: BSAID::<Identity, OFFSET>,
            BGSpecifier: BGSpecifier::<Identity, OFFSET>,
            BGName: BGName::<Identity, OFFSET>,
            BGSource: BGSource::<Identity, OFFSET>,
            BGType: BGType::<Identity, OFFSET>,
            IsPeriodic: IsPeriodic::<Identity, OFFSET>,
            IsScheduled: IsScheduled::<Identity, OFFSET>,
            IsScheduleAllowed: IsScheduleAllowed::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            IsLaunchOnBoot: IsLaunchOnBoot::<Identity, OFFSET>,
            set_IsScheduled: set_IsScheduled::<Identity, OFFSET>,
            set_IsScheduleAllowed: set_IsScheduleAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMBackgroundServiceAgentInfo {}
windows_core::imp::define_interface!(IPMBackgroundServiceAgentInfoEnumerator, IPMBackgroundServiceAgentInfoEnumerator_Vtbl, 0x18eb2072_ab56_43b3_872c_beafb7a6b391);
windows_core::imp::interface_hierarchy!(IPMBackgroundServiceAgentInfoEnumerator, windows_core::IUnknown);
impl IPMBackgroundServiceAgentInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMBackgroundServiceAgentInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMBackgroundServiceAgentInfo>;
}
impl IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbsainfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundServiceAgentInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        ppbsainfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMBackgroundServiceAgentInfoEnumerator {}
windows_core::imp::define_interface!(IPMBackgroundWorkerInfo, IPMBackgroundWorkerInfo_Vtbl, 0x7dd4531b_d3bf_4b6b_94f3_69c098b1497d);
windows_core::imp::interface_hierarchy!(IPMBackgroundWorkerInfo, windows_core::IUnknown);
impl IPMBackgroundWorkerInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TaskID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptaskid)).ok() }
    }
    pub unsafe fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BGName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbgname)).ok() }
    }
    pub unsafe fn MaxStartupLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxStartupLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpectedRuntime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpectedRuntime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsBootWorker(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBootWorker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BGName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxStartupLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExpectedRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsBootWorker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMBackgroundWorkerInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxStartupLatency(&self) -> windows_core::Result<u32>;
    fn ExpectedRuntime(&self) -> windows_core::Result<u32>;
    fn IsBootWorker(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMBackgroundWorkerInfo_Vtbl {
    pub const fn new<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundWorkerInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TaskID<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundWorkerInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
            }
        }
        unsafe extern "system" fn BGName<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMBackgroundWorkerInfo_Impl::BGName(this, core::mem::transmute_copy(&pbgname)).into()
            }
        }
        unsafe extern "system" fn MaxStartupLatency<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxstartuplatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundWorkerInfo_Impl::MaxStartupLatency(this) {
                    Ok(ok__) => {
                        pmaxstartuplatency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpectedruntime: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundWorkerInfo_Impl::ExpectedRuntime(this) {
                    Ok(ok__) => {
                        pexpectedruntime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsBootWorker<Identity: IPMBackgroundWorkerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisbootworker: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundWorkerInfo_Impl::IsBootWorker(this) {
                    Ok(ok__) => {
                        pisbootworker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            BGName: BGName::<Identity, OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Identity, OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Identity, OFFSET>,
            IsBootWorker: IsBootWorker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMBackgroundWorkerInfo {}
windows_core::imp::define_interface!(IPMBackgroundWorkerInfoEnumerator, IPMBackgroundWorkerInfoEnumerator_Vtbl, 0x87f479f8_90d8_4ec7_92b9_72787e2f636b);
windows_core::imp::interface_hierarchy!(IPMBackgroundWorkerInfoEnumerator, windows_core::IUnknown);
impl IPMBackgroundWorkerInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMBackgroundWorkerInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMBackgroundWorkerInfo>;
}
impl IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbwinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMBackgroundWorkerInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        ppbwinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMBackgroundWorkerInfoEnumerator {}
windows_core::imp::define_interface!(IPMDeploymentManager, IPMDeploymentManager_Vtbl, 0x35f785fa_1979_4a8b_bc8f_fd70eb0d1544);
windows_core::imp::interface_hierarchy!(IPMDeploymentManager, windows_core::IUnknown);
impl IPMDeploymentManager {
    pub unsafe fn ReportDownloadBegin(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportDownloadBegin)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn ReportDownloadProgress(&self, productid: windows_core::GUID, usprogress: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportDownloadProgress)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), usprogress).ok() }
    }
    pub unsafe fn ReportDownloadComplete(&self, productid: windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportDownloadComplete)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), hrresult).ok() }
    }
    pub unsafe fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginInstall)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstallinfo)).ok() }
    }
    pub unsafe fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute(pupdateinfo)).ok() }
    }
    pub unsafe fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginDeployPackage)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstallinfo)).ok() }
    }
    pub unsafe fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdateDeployedPackageLegacy)(windows_core::Interface::as_raw(self), core::mem::transmute(pupdateinfo)).ok() }
    }
    pub unsafe fn BeginUninstall(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUninstall)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEnterpriseAppInstall)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstallinfo)).ok() }
    }
    pub unsafe fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginEnterpriseAppUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute(pupdateinfo)).ok() }
    }
    pub unsafe fn BeginUpdateLicense(&self, productid: windows_core::GUID, offerid: windows_core::GUID, pblicense: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdateLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute(offerid), core::mem::transmute(pblicense.as_ptr()), pblicense.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetLicenseChallenge(&self, packagepath: &windows_core::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: Option<*mut *mut u8>, pcbkid: Option<*mut u32>, ppbdeviceid: Option<*mut *mut u8>, pcbdeviceid: Option<*mut u32>, ppbsaltvalue: Option<*mut *mut u8>, pcbsaltvalue: Option<*mut u32>, ppbkgvvalue: Option<*mut *mut u8>, pcbkgvvalue: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetLicenseChallenge)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(packagepath),
                ppbchallenge as _,
                pcbchallenge as _,
                ppbkid.unwrap_or(core::mem::zeroed()) as _,
                pcbkid.unwrap_or(core::mem::zeroed()) as _,
                ppbdeviceid.unwrap_or(core::mem::zeroed()) as _,
                pcbdeviceid.unwrap_or(core::mem::zeroed()) as _,
                ppbsaltvalue.unwrap_or(core::mem::zeroed()) as _,
                pcbsaltvalue.unwrap_or(core::mem::zeroed()) as _,
                ppbkgvvalue.unwrap_or(core::mem::zeroed()) as _,
                pcbkgvvalue.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetLicenseChallengeByProductID(&self, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLicenseChallengeByProductID)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), ppbchallenge as _, pcblicense as _).ok() }
    }
    pub unsafe fn GetLicenseChallengeByProductID2(&self, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: Option<*mut *mut u8>, pcbkid: Option<*mut u32>, ppbdeviceid: Option<*mut *mut u8>, pcbdeviceid: Option<*mut u32>, ppbsaltvalue: Option<*mut *mut u8>, pcbsaltvalue: Option<*mut u32>, ppbkgvvalue: Option<*mut *mut u8>, pcbkgvvalue: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetLicenseChallengeByProductID2)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(productid),
                ppbchallenge as _,
                pcblicense as _,
                ppbkid.unwrap_or(core::mem::zeroed()) as _,
                pcbkid.unwrap_or(core::mem::zeroed()) as _,
                ppbdeviceid.unwrap_or(core::mem::zeroed()) as _,
                pcbdeviceid.unwrap_or(core::mem::zeroed()) as _,
                ppbsaltvalue.unwrap_or(core::mem::zeroed()) as _,
                pcbsaltvalue.unwrap_or(core::mem::zeroed()) as _,
                ppbkgvvalue.unwrap_or(core::mem::zeroed()) as _,
                pcbkgvvalue.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn RevokeLicense(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RebindMdilBinaries(&self, productid: windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RebindMdilBinaries)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), filenames).ok() }
    }
    pub unsafe fn RebindAllMdilBinaries(&self, productid: windows_core::GUID, instanceid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RebindAllMdilBinaries)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute(instanceid)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegenerateXbf(&self, productid: windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegenerateXbf)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), assemblypaths).ok() }
    }
    pub unsafe fn GenerateXbfForCurrentLocale(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GenerateXbfForCurrentLocale)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn BeginProvision(&self, productid: windows_core::GUID, xmlpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginProvision)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute_copy(xmlpath)).ok() }
    }
    pub unsafe fn BeginDeprovision(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginDeprovision)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn ReindexSQLCEDatabases(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReindexSQLCEDatabases)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetApplicationsNeedMaintenance)(windows_core::Interface::as_raw(self), requiredmaintenanceoperations, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateChamberProfile(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateChamberProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn EnterprisePolicyIsApplicationAllowed<P1>(&self, productid: windows_core::GUID, publishername: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnterprisePolicyIsApplicationAllowed)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), publishername.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdateDeployedPackage)(windows_core::Interface::as_raw(self), core::mem::transmute(pupdateinfo)).ok() }
    }
    pub unsafe fn ReportRestoreCancelled(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportRestoreCancelled)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn ResolveResourceString<P0>(&self, resourcestring: P0, presolvedresourcestring: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveResourceString)(windows_core::Interface::as_raw(self), resourcestring.param().abi(), core::mem::transmute(presolvedresourcestring)).ok() }
    }
    pub unsafe fn UpdateCapabilitiesForModernApps(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateCapabilitiesForModernApps)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReportDownloadStatusUpdate(&self, productid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportDownloadStatusUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute(productid)).ok() }
    }
    pub unsafe fn BeginUninstallWithOptions(&self, productid: windows_core::GUID, removaloptions: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUninstallWithOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), removaloptions).ok() }
    }
    pub unsafe fn BindDeferredMdilBinaries(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindDeferredMdilBinaries)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GenerateXamlLightupXbfForCurrentLocale(&self, packagefamilyname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GenerateXamlLightupXbfForCurrentLocale)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(packagefamilyname)).ok() }
    }
    pub unsafe fn AddLicenseForAppx(&self, productid: windows_core::GUID, pblicense: &[u8], pbplayreadyheader: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddLicenseForAppx)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute(pblicense.as_ptr()), pblicense.len().try_into().unwrap(), core::mem::transmute(pbplayreadyheader.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbplayreadyheader.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn FixJunctionsForAppsOnSDCard(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FixJunctionsForAppsOnSDCard)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMDeploymentManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportDownloadBegin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub ReportDownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u16) -> windows_core::HRESULT,
    pub ReportDownloadComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub BeginInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_INSTALLINFO) -> windows_core::HRESULT,
    pub BeginUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_UPDATEINFO) -> windows_core::HRESULT,
    pub BeginDeployPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_INSTALLINFO) -> windows_core::HRESULT,
    pub BeginUpdateDeployedPackageLegacy: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_UPDATEINFO_LEGACY) -> windows_core::HRESULT,
    pub BeginUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub BeginEnterpriseAppInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_INSTALLINFO) -> windows_core::HRESULT,
    pub BeginEnterpriseAppUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_UPDATEINFO) -> windows_core::HRESULT,
    pub BeginUpdateLicense: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u8, u32) -> windows_core::HRESULT,
    pub GetLicenseChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetLicenseChallengeByProductID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetLicenseChallengeByProductID2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub RevokeLicense: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RebindMdilBinaries: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RebindMdilBinaries: usize,
    pub RebindAllMdilBinaries: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegenerateXbf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegenerateXbf: usize,
    pub GenerateXbfForCurrentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub BeginProvision: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginDeprovision: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub ReindexSQLCEDatabases: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetApplicationsNeedMaintenance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub UpdateChamberProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub EnterprisePolicyIsApplicationAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub BeginUpdateDeployedPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *const PM_UPDATEINFO) -> windows_core::HRESULT,
    pub ReportRestoreCancelled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub ResolveResourceString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateCapabilitiesForModernApps: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportDownloadStatusUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub BeginUninstallWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub BindDeferredMdilBinaries: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateXamlLightupXbfForCurrentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddLicenseForAppx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const u8, u32, *const u8, u32) -> windows_core::HRESULT,
    pub FixJunctionsForAppsOnSDCard: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPMDeploymentManager_Impl: windows_core::IUnknownImpl {
    fn ReportDownloadBegin(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ReportDownloadProgress(&self, productid: &windows_core::GUID, usprogress: u16) -> windows_core::Result<()>;
    fn ReportDownloadComplete(&self, productid: &windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> windows_core::Result<()>;
    fn BeginUninstall(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn BeginUpdateLicense(&self, productid: &windows_core::GUID, offerid: &windows_core::GUID, pblicense: *const u8, cblicense: u32) -> windows_core::Result<()>;
    fn GetLicenseChallenge(&self, packagepath: &windows_core::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::Result<()>;
    fn GetLicenseChallengeByProductID(&self, productid: &windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> windows_core::Result<()>;
    fn GetLicenseChallengeByProductID2(&self, productid: &windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::Result<()>;
    fn RevokeLicense(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RebindMdilBinaries(&self, productid: &windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RebindAllMdilBinaries(&self, productid: &windows_core::GUID, instanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RegenerateXbf(&self, productid: &windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GenerateXbfForCurrentLocale(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginProvision(&self, productid: &windows_core::GUID, xmlpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BeginDeprovision(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ReindexSQLCEDatabases(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> windows_core::Result<u32>;
    fn UpdateChamberProfile(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(&self, productid: &windows_core::GUID, publishername: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn ReportRestoreCancelled(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ResolveResourceString(&self, resourcestring: &windows_core::PCWSTR, presolvedresourcestring: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateCapabilitiesForModernApps(&self) -> windows_core::Result<()>;
    fn ReportDownloadStatusUpdate(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginUninstallWithOptions(&self, productid: &windows_core::GUID, removaloptions: u32) -> windows_core::Result<()>;
    fn BindDeferredMdilBinaries(&self) -> windows_core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(&self, packagefamilyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddLicenseForAppx(&self, productid: &windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> windows_core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPMDeploymentManager_Vtbl {
    pub const fn new<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportDownloadBegin<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReportDownloadBegin(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn ReportDownloadProgress<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, usprogress: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReportDownloadProgress(this, core::mem::transmute(&productid), core::mem::transmute_copy(&usprogress)).into()
            }
        }
        unsafe extern "system" fn ReportDownloadComplete<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReportDownloadComplete(this, core::mem::transmute(&productid), core::mem::transmute_copy(&hrresult)).into()
            }
        }
        unsafe extern "system" fn BeginInstall<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginInstall(this, core::mem::transmute_copy(&pinstallinfo)).into()
            }
        }
        unsafe extern "system" fn BeginUpdate<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUpdate(this, core::mem::transmute_copy(&pupdateinfo)).into()
            }
        }
        unsafe extern "system" fn BeginDeployPackage<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginDeployPackage(this, core::mem::transmute_copy(&pinstallinfo)).into()
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUpdateDeployedPackageLegacy(this, core::mem::transmute_copy(&pupdateinfo)).into()
            }
        }
        unsafe extern "system" fn BeginUninstall<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUninstall(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginEnterpriseAppInstall(this, core::mem::transmute_copy(&pinstallinfo)).into()
            }
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginEnterpriseAppUpdate(this, core::mem::transmute_copy(&pupdateinfo)).into()
            }
        }
        unsafe extern "system" fn BeginUpdateLicense<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, offerid: windows_core::GUID, pblicense: *const u8, cblicense: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUpdateLicense(this, core::mem::transmute(&productid), core::mem::transmute(&offerid), core::mem::transmute_copy(&pblicense), core::mem::transmute_copy(&cblicense)).into()
            }
        }
        unsafe extern "system" fn GetLicenseChallenge<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagepath: *mut core::ffi::c_void, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::GetLicenseChallenge(this, core::mem::transmute(&packagepath), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcbchallenge), core::mem::transmute_copy(&ppbkid), core::mem::transmute_copy(&pcbkid), core::mem::transmute_copy(&ppbdeviceid), core::mem::transmute_copy(&pcbdeviceid), core::mem::transmute_copy(&ppbsaltvalue), core::mem::transmute_copy(&pcbsaltvalue), core::mem::transmute_copy(&ppbkgvvalue), core::mem::transmute_copy(&pcbkgvvalue)).into()
            }
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::GetLicenseChallengeByProductID(this, core::mem::transmute(&productid), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcblicense)).into()
            }
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::GetLicenseChallengeByProductID2(this, core::mem::transmute(&productid), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcblicense), core::mem::transmute_copy(&ppbkid), core::mem::transmute_copy(&pcbkid), core::mem::transmute_copy(&ppbdeviceid), core::mem::transmute_copy(&pcbdeviceid), core::mem::transmute_copy(&ppbsaltvalue), core::mem::transmute_copy(&pcbsaltvalue), core::mem::transmute_copy(&ppbkgvvalue), core::mem::transmute_copy(&pcbkgvvalue)).into()
            }
        }
        unsafe extern "system" fn RevokeLicense<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::RevokeLicense(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn RebindMdilBinaries<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::RebindMdilBinaries(this, core::mem::transmute(&productid), core::mem::transmute_copy(&filenames)).into()
            }
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, instanceid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::RebindAllMdilBinaries(this, core::mem::transmute(&productid), core::mem::transmute(&instanceid)).into()
            }
        }
        unsafe extern "system" fn RegenerateXbf<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::RegenerateXbf(this, core::mem::transmute(&productid), core::mem::transmute_copy(&assemblypaths)).into()
            }
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::GenerateXbfForCurrentLocale(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn BeginProvision<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, xmlpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginProvision(this, core::mem::transmute(&productid), core::mem::transmute(&xmlpath)).into()
            }
        }
        unsafe extern "system" fn BeginDeprovision<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginDeprovision(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReindexSQLCEDatabases(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMDeploymentManager_Impl::SetApplicationsNeedMaintenance(this, core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                    Ok(ok__) => {
                        pcapplications.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::UpdateChamberProfile(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, publishername: windows_core::PCWSTR, pisallowed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMDeploymentManager_Impl::EnterprisePolicyIsApplicationAllowed(this, core::mem::transmute(&productid), core::mem::transmute(&publishername)) {
                    Ok(ok__) => {
                        pisallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUpdateDeployedPackage(this, core::mem::transmute_copy(&pupdateinfo)).into()
            }
        }
        unsafe extern "system" fn ReportRestoreCancelled<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReportRestoreCancelled(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn ResolveResourceString<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcestring: windows_core::PCWSTR, presolvedresourcestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ResolveResourceString(this, core::mem::transmute(&resourcestring), core::mem::transmute_copy(&presolvedresourcestring)).into()
            }
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::UpdateCapabilitiesForModernApps(this).into()
            }
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::ReportDownloadStatusUpdate(this, core::mem::transmute(&productid)).into()
            }
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, removaloptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BeginUninstallWithOptions(this, core::mem::transmute(&productid), core::mem::transmute_copy(&removaloptions)).into()
            }
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::BindDeferredMdilBinaries(this).into()
            }
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::GenerateXamlLightupXbfForCurrentLocale(this, core::mem::transmute(&packagefamilyname)).into()
            }
        }
        unsafe extern "system" fn AddLicenseForAppx<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::AddLicenseForAppx(this, core::mem::transmute(&productid), core::mem::transmute_copy(&pblicense), core::mem::transmute_copy(&cblicense), core::mem::transmute_copy(&pbplayreadyheader), core::mem::transmute_copy(&cbplayreadyheader)).into()
            }
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Identity: IPMDeploymentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMDeploymentManager_Impl::FixJunctionsForAppsOnSDCard(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportDownloadBegin: ReportDownloadBegin::<Identity, OFFSET>,
            ReportDownloadProgress: ReportDownloadProgress::<Identity, OFFSET>,
            ReportDownloadComplete: ReportDownloadComplete::<Identity, OFFSET>,
            BeginInstall: BeginInstall::<Identity, OFFSET>,
            BeginUpdate: BeginUpdate::<Identity, OFFSET>,
            BeginDeployPackage: BeginDeployPackage::<Identity, OFFSET>,
            BeginUpdateDeployedPackageLegacy: BeginUpdateDeployedPackageLegacy::<Identity, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, OFFSET>,
            BeginEnterpriseAppInstall: BeginEnterpriseAppInstall::<Identity, OFFSET>,
            BeginEnterpriseAppUpdate: BeginEnterpriseAppUpdate::<Identity, OFFSET>,
            BeginUpdateLicense: BeginUpdateLicense::<Identity, OFFSET>,
            GetLicenseChallenge: GetLicenseChallenge::<Identity, OFFSET>,
            GetLicenseChallengeByProductID: GetLicenseChallengeByProductID::<Identity, OFFSET>,
            GetLicenseChallengeByProductID2: GetLicenseChallengeByProductID2::<Identity, OFFSET>,
            RevokeLicense: RevokeLicense::<Identity, OFFSET>,
            RebindMdilBinaries: RebindMdilBinaries::<Identity, OFFSET>,
            RebindAllMdilBinaries: RebindAllMdilBinaries::<Identity, OFFSET>,
            RegenerateXbf: RegenerateXbf::<Identity, OFFSET>,
            GenerateXbfForCurrentLocale: GenerateXbfForCurrentLocale::<Identity, OFFSET>,
            BeginProvision: BeginProvision::<Identity, OFFSET>,
            BeginDeprovision: BeginDeprovision::<Identity, OFFSET>,
            ReindexSQLCEDatabases: ReindexSQLCEDatabases::<Identity, OFFSET>,
            SetApplicationsNeedMaintenance: SetApplicationsNeedMaintenance::<Identity, OFFSET>,
            UpdateChamberProfile: UpdateChamberProfile::<Identity, OFFSET>,
            EnterprisePolicyIsApplicationAllowed: EnterprisePolicyIsApplicationAllowed::<Identity, OFFSET>,
            BeginUpdateDeployedPackage: BeginUpdateDeployedPackage::<Identity, OFFSET>,
            ReportRestoreCancelled: ReportRestoreCancelled::<Identity, OFFSET>,
            ResolveResourceString: ResolveResourceString::<Identity, OFFSET>,
            UpdateCapabilitiesForModernApps: UpdateCapabilitiesForModernApps::<Identity, OFFSET>,
            ReportDownloadStatusUpdate: ReportDownloadStatusUpdate::<Identity, OFFSET>,
            BeginUninstallWithOptions: BeginUninstallWithOptions::<Identity, OFFSET>,
            BindDeferredMdilBinaries: BindDeferredMdilBinaries::<Identity, OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale: GenerateXamlLightupXbfForCurrentLocale::<Identity, OFFSET>,
            AddLicenseForAppx: AddLicenseForAppx::<Identity, OFFSET>,
            FixJunctionsForAppsOnSDCard: FixJunctionsForAppsOnSDCard::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMDeploymentManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPMDeploymentManager {}
windows_core::imp::define_interface!(IPMEnumerationManager, IPMEnumerationManager_Vtbl, 0x698d57c2_292d_4cf3_b73c_d95a6922ed9a);
windows_core::imp::interface_hierarchy!(IPMEnumerationManager, windows_core::IUnknown);
impl IPMEnumerationManager {
    pub unsafe fn get_AllApplications(&self, ppappenum: *mut Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllApplications)(windows_core::Interface::as_raw(self), core::mem::transmute(ppappenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_AllTiles(&self, pptileenum: *mut Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllTiles)(windows_core::Interface::as_raw(self), core::mem::transmute(pptileenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_AllTasks(&self, pptaskenum: *mut Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllTasks)(windows_core::Interface::as_raw(self), core::mem::transmute(pptaskenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_AllExtensions(&self, ppextensionenum: *mut Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllExtensions)(windows_core::Interface::as_raw(self), core::mem::transmute(ppextensionenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_AllBackgroundServiceAgents(&self, ppbsaenum: *mut Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllBackgroundServiceAgents)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbsaenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_AllBackgroundWorkers(&self, ppbswenum: *mut Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllBackgroundWorkers)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbswenum), core::mem::transmute_copy(filter)).ok() }
    }
    pub unsafe fn get_ApplicationInfo(&self, productid: windows_core::GUID) -> windows_core::Result<IPMApplicationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ApplicationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TileInfo(&self, productid: windows_core::GUID, tileid: &windows_core::BSTR) -> windows_core::Result<IPMTileInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TileInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TaskInfo(&self, productid: windows_core::GUID, taskid: &windows_core::BSTR) -> windows_core::Result<IPMTaskInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute_copy(taskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TaskInfoEx<P1>(&self, productid: windows_core::GUID, taskid: P1) -> windows_core::Result<IPMTaskInfo>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskInfoEx)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), taskid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> windows_core::Result<IPMBackgroundServiceAgentInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BackgroundServiceAgentInfo)(windows_core::Interface::as_raw(self), bsaid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AllLiveTileJobs(&self) -> windows_core::Result<IPMLiveTileJobInfoEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllLiveTileJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_LiveTileJob(&self, productid: windows_core::GUID, tileid: &windows_core::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> windows_core::Result<IPMLiveTileJobInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LiveTileJob)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), core::mem::transmute_copy(tileid), recurrencetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ApplicationInfoExternal(&self, productid: windows_core::GUID) -> windows_core::Result<IPMApplicationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ApplicationInfoExternal)(windows_core::Interface::as_raw(self), core::mem::transmute(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_FileHandlerGenericLogo(&self, filetype: &windows_core::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_FileHandlerGenericLogo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filetype), logosize, core::mem::transmute(plogo)).ok() }
    }
    pub unsafe fn get_ApplicationInfoFromAccessClaims(&self, sysappid0: &windows_core::BSTR, sysappid1: &windows_core::BSTR) -> windows_core::Result<IPMApplicationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ApplicationInfoFromAccessClaims)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sysappid0), core::mem::transmute_copy(sysappid1), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_StartTileEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_StartTileEnumeratorBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filter), pctiles as _, pptileblobs as _).ok() }
    }
    pub unsafe fn get_StartAppEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_StartAppEnumeratorBlob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filter), pcapps as _, ppappblobs as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMEnumerationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_AllApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_AllTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_AllTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_AllExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_AllBackgroundServiceAgents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_AllBackgroundWorkers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, PM_ENUM_FILTER) -> windows_core::HRESULT,
    pub get_ApplicationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TileInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TaskInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TaskInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_BackgroundServiceAgentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllLiveTileJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LiveTileJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, PM_LIVETILE_RECURRENCE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ApplicationInfoExternal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FileHandlerGenericLogo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PM_LOGO_SIZE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ApplicationInfoFromAccessClaims: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StartTileEnumeratorBlob: unsafe extern "system" fn(*mut core::ffi::c_void, PM_ENUM_FILTER, *mut u32, *mut *mut PM_STARTTILEBLOB) -> windows_core::HRESULT,
    pub get_StartAppEnumeratorBlob: unsafe extern "system" fn(*mut core::ffi::c_void, PM_ENUM_FILTER, *mut u32, *mut *mut PM_STARTAPPBLOB) -> windows_core::HRESULT,
}
pub trait IPMEnumerationManager_Impl: windows_core::IUnknownImpl {
    fn get_AllApplications(&self, ppappenum: windows_core::OutRef<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllTiles(&self, pptileenum: windows_core::OutRef<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllTasks(&self, pptaskenum: windows_core::OutRef<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllExtensions(&self, ppextensionenum: windows_core::OutRef<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllBackgroundServiceAgents(&self, ppbsaenum: windows_core::OutRef<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllBackgroundWorkers(&self, ppbswenum: windows_core::OutRef<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_ApplicationInfo(&self, productid: &windows_core::GUID) -> windows_core::Result<IPMApplicationInfo>;
    fn get_TileInfo(&self, productid: &windows_core::GUID, tileid: &windows_core::BSTR) -> windows_core::Result<IPMTileInfo>;
    fn get_TaskInfo(&self, productid: &windows_core::GUID, taskid: &windows_core::BSTR) -> windows_core::Result<IPMTaskInfo>;
    fn get_TaskInfoEx(&self, productid: &windows_core::GUID, taskid: &windows_core::PCWSTR) -> windows_core::Result<IPMTaskInfo>;
    fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> windows_core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(&self) -> windows_core::Result<IPMLiveTileJobInfoEnumerator>;
    fn get_LiveTileJob(&self, productid: &windows_core::GUID, tileid: &windows_core::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> windows_core::Result<IPMLiveTileJobInfo>;
    fn get_ApplicationInfoExternal(&self, productid: &windows_core::GUID) -> windows_core::Result<IPMApplicationInfo>;
    fn get_FileHandlerGenericLogo(&self, filetype: &windows_core::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ApplicationInfoFromAccessClaims(&self, sysappid0: &windows_core::BSTR, sysappid1: &windows_core::BSTR) -> windows_core::Result<IPMApplicationInfo>;
    fn get_StartTileEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> windows_core::Result<()>;
    fn get_StartAppEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> windows_core::Result<()>;
}
impl IPMEnumerationManager_Vtbl {
    pub const fn new<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllApplications<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllApplications(this, core::mem::transmute_copy(&ppappenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AllTiles<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptileenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllTiles(this, core::mem::transmute_copy(&pptileenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AllTasks<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllTasks(this, core::mem::transmute_copy(&pptaskenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AllExtensions<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensionenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllExtensions(this, core::mem::transmute_copy(&ppextensionenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AllBackgroundServiceAgents<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbsaenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllBackgroundServiceAgents(this, core::mem::transmute_copy(&ppbsaenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AllBackgroundWorkers<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbswenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_AllBackgroundWorkers(this, core::mem::transmute_copy(&ppbswenum), core::mem::transmute(&filter)).into()
            }
        }
        unsafe extern "system" fn get_ApplicationInfo<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_ApplicationInfo(this, core::mem::transmute(&productid)) {
                    Ok(ok__) => {
                        ppappinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TileInfo<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, tileid: *mut core::ffi::c_void, pptileinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_TileInfo(this, core::mem::transmute(&productid), core::mem::transmute(&tileid)) {
                    Ok(ok__) => {
                        pptileinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TaskInfo<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, taskid: *mut core::ffi::c_void, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_TaskInfo(this, core::mem::transmute(&productid), core::mem::transmute(&taskid)) {
                    Ok(ok__) => {
                        pptaskinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TaskInfoEx<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, taskid: windows_core::PCWSTR, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_TaskInfoEx(this, core::mem::transmute(&productid), core::mem::transmute(&taskid)) {
                    Ok(ok__) => {
                        pptaskinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BackgroundServiceAgentInfo<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_BackgroundServiceAgentInfo(this, core::mem::transmute_copy(&bsaid)) {
                    Ok(ok__) => {
                        pptaskinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplivetilejobenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::AllLiveTileJobs(this) {
                    Ok(ok__) => {
                        pplivetilejobenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LiveTileJob<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, tileid: *mut core::ffi::c_void, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_LiveTileJob(this, core::mem::transmute(&productid), core::mem::transmute(&tileid), core::mem::transmute_copy(&recurrencetype)) {
                    Ok(ok__) => {
                        pplivetilejobinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ApplicationInfoExternal<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_ApplicationInfoExternal(this, core::mem::transmute(&productid)) {
                    Ok(ok__) => {
                        ppappinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FileHandlerGenericLogo<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: *mut core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_FileHandlerGenericLogo(this, core::mem::transmute(&filetype), core::mem::transmute_copy(&logosize), core::mem::transmute_copy(&plogo)).into()
            }
        }
        unsafe extern "system" fn get_ApplicationInfoFromAccessClaims<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sysappid0: *mut core::ffi::c_void, sysappid1: *mut core::ffi::c_void, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMEnumerationManager_Impl::get_ApplicationInfoFromAccessClaims(this, core::mem::transmute(&sysappid0), core::mem::transmute(&sysappid1)) {
                    Ok(ok__) => {
                        ppappinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StartTileEnumeratorBlob<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_StartTileEnumeratorBlob(this, core::mem::transmute(&filter), core::mem::transmute_copy(&pctiles), core::mem::transmute_copy(&pptileblobs)).into()
            }
        }
        unsafe extern "system" fn get_StartAppEnumeratorBlob<Identity: IPMEnumerationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMEnumerationManager_Impl::get_StartAppEnumeratorBlob(this, core::mem::transmute(&filter), core::mem::transmute_copy(&pcapps), core::mem::transmute_copy(&ppappblobs)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllApplications: get_AllApplications::<Identity, OFFSET>,
            get_AllTiles: get_AllTiles::<Identity, OFFSET>,
            get_AllTasks: get_AllTasks::<Identity, OFFSET>,
            get_AllExtensions: get_AllExtensions::<Identity, OFFSET>,
            get_AllBackgroundServiceAgents: get_AllBackgroundServiceAgents::<Identity, OFFSET>,
            get_AllBackgroundWorkers: get_AllBackgroundWorkers::<Identity, OFFSET>,
            get_ApplicationInfo: get_ApplicationInfo::<Identity, OFFSET>,
            get_TileInfo: get_TileInfo::<Identity, OFFSET>,
            get_TaskInfo: get_TaskInfo::<Identity, OFFSET>,
            get_TaskInfoEx: get_TaskInfoEx::<Identity, OFFSET>,
            get_BackgroundServiceAgentInfo: get_BackgroundServiceAgentInfo::<Identity, OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Identity, OFFSET>,
            get_LiveTileJob: get_LiveTileJob::<Identity, OFFSET>,
            get_ApplicationInfoExternal: get_ApplicationInfoExternal::<Identity, OFFSET>,
            get_FileHandlerGenericLogo: get_FileHandlerGenericLogo::<Identity, OFFSET>,
            get_ApplicationInfoFromAccessClaims: get_ApplicationInfoFromAccessClaims::<Identity, OFFSET>,
            get_StartTileEnumeratorBlob: get_StartTileEnumeratorBlob::<Identity, OFFSET>,
            get_StartAppEnumeratorBlob: get_StartAppEnumeratorBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMEnumerationManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMEnumerationManager {}
windows_core::imp::define_interface!(IPMExtensionCachedFileUpdaterInfo, IPMExtensionCachedFileUpdaterInfo_Vtbl, 0xe2d77509_4e58_4ba9_af7e_b642e370e1b0);
windows_core::imp::interface_hierarchy!(IPMExtensionCachedFileUpdaterInfo, windows_core::IUnknown);
impl IPMExtensionCachedFileUpdaterInfo {
    pub unsafe fn SupportsUpdates(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SupportsUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: windows_core::IUnknownImpl {
    fn SupportsUpdates(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportsUpdates<Identity: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsupdates: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionCachedFileUpdaterInfo_Impl::SupportsUpdates(this) {
                    Ok(ok__) => {
                        psupportsupdates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SupportsUpdates: SupportsUpdates::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionCachedFileUpdaterInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionCachedFileUpdaterInfo {}
windows_core::imp::define_interface!(IPMExtensionContractInfo, IPMExtensionContractInfo_Vtbl, 0xe5666373_7ba1_467c_b819_b175db1c295b);
windows_core::imp::interface_hierarchy!(IPMExtensionContractInfo, windows_core::IUnknown);
impl IPMExtensionContractInfo {
    pub unsafe fn get_InvocationInfo(&self, paumid: *mut windows_core::BSTR, pargs: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(paumid), core::mem::transmute(pargs)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionContractInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMExtensionContractInfo_Impl: windows_core::IUnknownImpl {
    fn get_InvocationInfo(&self, paumid: *mut windows_core::BSTR, pargs: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMExtensionContractInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionContractInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMExtensionContractInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paumid: *mut *mut core::ffi::c_void, pargs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionContractInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&paumid), core::mem::transmute_copy(&pargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionContractInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionContractInfo {}
windows_core::imp::define_interface!(IPMExtensionFileExtensionInfo, IPMExtensionFileExtensionInfo_Vtbl, 0x6b87cb6c_0b88_4989_a4ec_033714f710d4);
windows_core::imp::interface_hierarchy!(IPMExtensionFileExtensionInfo, windows_core::IUnknown);
impl IPMExtensionFileExtensionInfo {
    pub unsafe fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn DisplayName(&self, pdisplayname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pdisplayname)).ok() }
    }
    pub unsafe fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_Logo)(windows_core::Interface::as_raw(self), logosize, core::mem::transmute(plogo)).ok() }
    }
    pub unsafe fn get_ContentType(&self, filetype: &windows_core::BSTR, pcontenttype: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_ContentType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filetype), core::mem::transmute(pcontenttype)).ok() }
    }
    pub unsafe fn get_FileType(&self, contenttype: &windows_core::BSTR, pfiletype: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_FileType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(contenttype), core::mem::transmute(pfiletype)).ok() }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
    pub unsafe fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllFileTypes)(windows_core::Interface::as_raw(self), pcbtypes as _, pptypes as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileExtensionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Logo: unsafe extern "system" fn(*mut core::ffi::c_void, PM_LOGO_SIZE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMExtensionFileExtensionInfo_Impl: windows_core::IUnknownImpl {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self, pdisplayname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ContentType(&self, filetype: &windows_core::BSTR, pcontenttype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FileType(&self, contenttype: &windows_core::BSTR, pfiletype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMExtensionFileExtensionInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::DisplayName(this, core::mem::transmute_copy(&pdisplayname)).into()
            }
        }
        unsafe extern "system" fn get_Logo<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::get_Logo(this, core::mem::transmute_copy(&logosize), core::mem::transmute_copy(&plogo)).into()
            }
        }
        unsafe extern "system" fn get_ContentType<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: *mut core::ffi::c_void, pcontenttype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::get_ContentType(this, core::mem::transmute(&filetype), core::mem::transmute_copy(&pcontenttype)).into()
            }
        }
        unsafe extern "system" fn get_FileType<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut core::ffi::c_void, pfiletype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::get_FileType(this, core::mem::transmute(&contenttype), core::mem::transmute_copy(&pfiletype)).into()
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn get_AllFileTypes<Identity: IPMExtensionFileExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileExtensionInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pcbtypes), core::mem::transmute_copy(&pptypes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            get_Logo: get_Logo::<Identity, OFFSET>,
            get_ContentType: get_ContentType::<Identity, OFFSET>,
            get_FileType: get_FileType::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileExtensionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionFileExtensionInfo {}
windows_core::imp::define_interface!(IPMExtensionFileOpenPickerInfo, IPMExtensionFileOpenPickerInfo_Vtbl, 0x6dc91d25_9606_420c_9a78_e034a3418345);
windows_core::imp::interface_hierarchy!(IPMExtensionFileOpenPickerInfo, windows_core::IUnknown);
impl IPMExtensionFileOpenPickerInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllFileTypes)(windows_core::Interface::as_raw(self), pctypes as _, pptypes as _).ok() }
    }
    pub unsafe fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsAllFileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileOpenPickerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsAllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMExtensionFileOpenPickerInfo_Impl: windows_core::IUnknownImpl {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMExtensionFileOpenPickerInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllFileTypes<Identity: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileOpenPickerInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionFileOpenPickerInfo_Impl::SupportsAllFileTypes(this) {
                    Ok(ok__) => {
                        psupportsalltypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileOpenPickerInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionFileOpenPickerInfo {}
windows_core::imp::define_interface!(IPMExtensionFileSavePickerInfo, IPMExtensionFileSavePickerInfo_Vtbl, 0x38005cba_f81a_493e_a0f8_922c8680da43);
windows_core::imp::interface_hierarchy!(IPMExtensionFileSavePickerInfo, windows_core::IUnknown);
impl IPMExtensionFileSavePickerInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllFileTypes)(windows_core::Interface::as_raw(self), pctypes as _, pptypes as _).ok() }
    }
    pub unsafe fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsAllFileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileSavePickerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsAllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMExtensionFileSavePickerInfo_Impl: windows_core::IUnknownImpl {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMExtensionFileSavePickerInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllFileTypes<Identity: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionFileSavePickerInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionFileSavePickerInfo_Impl::SupportsAllFileTypes(this) {
                    Ok(ok__) => {
                        psupportsalltypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileSavePickerInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionFileSavePickerInfo {}
windows_core::imp::define_interface!(IPMExtensionInfo, IPMExtensionInfo_Vtbl, 0x49acde79_9788_4d0a_8aa0_1746afdb9e9d);
windows_core::imp::interface_hierarchy!(IPMExtensionInfo, windows_core::IUnknown);
impl IPMExtensionInfo {
    pub unsafe fn SupplierPID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupplierPID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupplierTaskID(&self, psuppliertid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SupplierTaskID)(windows_core::Interface::as_raw(self), core::mem::transmute(psuppliertid)).ok() }
    }
    pub unsafe fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), core::mem::transmute(ptitle)).ok() }
    }
    pub unsafe fn IconPath(&self, piconpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IconPath)(windows_core::Interface::as_raw(self), core::mem::transmute(piconpath)).ok() }
    }
    pub unsafe fn ExtraFile(&self, pfilepath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExtraFile)(windows_core::Interface::as_raw(self), core::mem::transmute(pfilepath)).ok() }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SupplierPID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SupplierTaskID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtraFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMExtensionInfo_Impl: windows_core::IUnknownImpl {
    fn SupplierPID(&self) -> windows_core::Result<windows_core::GUID>;
    fn SupplierTaskID(&self, psuppliertid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IconPath(&self, piconpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtraFile(&self, pfilepath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMExtensionInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupplierPID<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupplierpid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionInfo_Impl::SupplierPID(this) {
                    Ok(ok__) => {
                        psupplierpid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupplierTaskID<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psuppliertid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionInfo_Impl::SupplierTaskID(this, core::mem::transmute_copy(&psuppliertid)).into()
            }
        }
        unsafe extern "system" fn Title<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionInfo_Impl::Title(this, core::mem::transmute_copy(&ptitle)).into()
            }
        }
        unsafe extern "system" fn IconPath<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piconpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionInfo_Impl::IconPath(this, core::mem::transmute_copy(&piconpath)).into()
            }
        }
        unsafe extern "system" fn ExtraFile<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionInfo_Impl::ExtraFile(this, core::mem::transmute_copy(&pfilepath)).into()
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMExtensionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupplierPID: SupplierPID::<Identity, OFFSET>,
            SupplierTaskID: SupplierTaskID::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            IconPath: IconPath::<Identity, OFFSET>,
            ExtraFile: ExtraFile::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionInfo {}
windows_core::imp::define_interface!(IPMExtensionInfoEnumerator, IPMExtensionInfoEnumerator_Vtbl, 0x403b9e82_1171_4573_8e6f_6f33f39b83dd);
windows_core::imp::interface_hierarchy!(IPMExtensionInfoEnumerator, windows_core::IUnknown);
impl IPMExtensionInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMExtensionInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMExtensionInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMExtensionInfo>;
}
impl IPMExtensionInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMExtensionInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        ppextensioninfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionInfoEnumerator {}
windows_core::imp::define_interface!(IPMExtensionProtocolInfo, IPMExtensionProtocolInfo_Vtbl, 0x1e3fa036_51eb_4453_baff_b8d8e4b46c8e);
windows_core::imp::interface_hierarchy!(IPMExtensionProtocolInfo, windows_core::IUnknown);
impl IPMExtensionProtocolInfo {
    pub unsafe fn Protocol(&self, pprotocol: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), core::mem::transmute(pprotocol)).ok() }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionProtocolInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMExtensionProtocolInfo_Impl: windows_core::IUnknownImpl {
    fn Protocol(&self, pprotocol: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMExtensionProtocolInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Protocol<Identity: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionProtocolInfo_Impl::Protocol(this, core::mem::transmute_copy(&pprotocol)).into()
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMExtensionProtocolInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionProtocolInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionProtocolInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionProtocolInfo {}
windows_core::imp::define_interface!(IPMExtensionShareTargetInfo, IPMExtensionShareTargetInfo_Vtbl, 0x5471f48b_c65c_4656_8c70_242e31195fea);
windows_core::imp::interface_hierarchy!(IPMExtensionShareTargetInfo, windows_core::IUnknown);
impl IPMExtensionShareTargetInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllFileTypes)(windows_core::Interface::as_raw(self), pctypes as _, pptypes as _).ok() }
    }
    pub unsafe fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_AllDataFormats)(windows_core::Interface::as_raw(self), pcdataformats as _, ppdataformats as _).ok() }
    }
    pub unsafe fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsAllFileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionShareTargetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AllDataFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsAllFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMExtensionShareTargetInfo_Impl: windows_core::IUnknownImpl {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMExtensionShareTargetInfo_Vtbl {
    pub const fn new<Identity: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllFileTypes<Identity: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionShareTargetInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
            }
        }
        unsafe extern "system" fn get_AllDataFormats<Identity: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMExtensionShareTargetInfo_Impl::get_AllDataFormats(this, core::mem::transmute_copy(&pcdataformats), core::mem::transmute_copy(&ppdataformats)).into()
            }
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: IPMExtensionShareTargetInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMExtensionShareTargetInfo_Impl::SupportsAllFileTypes(this) {
                    Ok(ok__) => {
                        psupportsalltypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            get_AllDataFormats: get_AllDataFormats::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionShareTargetInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMExtensionShareTargetInfo {}
windows_core::imp::define_interface!(IPMLiveTileJobInfo, IPMLiveTileJobInfo_Vtbl, 0x6009a81f_4710_4697_b5f6_2208f6057b8e);
windows_core::imp::interface_hierarchy!(IPMLiveTileJobInfo, windows_core::IUnknown);
impl IPMLiveTileJobInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TileID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptileid)).ok() }
    }
    pub unsafe fn NextSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextSchedule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_NextSchedule(&self, ftnextschedule: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_NextSchedule)(windows_core::Interface::as_raw(self), core::mem::transmute(ftnextschedule)).ok() }
    }
    pub unsafe fn StartSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartSchedule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_StartSchedule(&self, ftstartschedule: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_StartSchedule)(windows_core::Interface::as_raw(self), core::mem::transmute(ftstartschedule)).ok() }
    }
    pub unsafe fn IntervalDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntervalDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_IntervalDuration(&self, ulintervalduration: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IntervalDuration)(windows_core::Interface::as_raw(self), ulintervalduration).ok() }
    }
    pub unsafe fn RunForever(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunForever)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_RunForever(&self, frunforever: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_RunForever)(windows_core::Interface::as_raw(self), frunforever.into()).ok() }
    }
    pub unsafe fn MaxRunCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxRunCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_MaxRunCount(&self, ulmaxruncount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_MaxRunCount)(windows_core::Interface::as_raw(self), ulmaxruncount).ok() }
    }
    pub unsafe fn RunCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_RunCount(&self, ulruncount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_RunCount)(windows_core::Interface::as_raw(self), ulruncount).ok() }
    }
    pub unsafe fn RecurrenceType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecurrenceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_RecurrenceType)(windows_core::Interface::as_raw(self), ulrecurrencetype).ok() }
    }
    pub unsafe fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_TileXML)(windows_core::Interface::as_raw(self), ptilexml as _, pcbtilexml as _).ok() }
    }
    pub unsafe fn set_TileXML(&self, ptilexml: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_TileXML)(windows_core::Interface::as_raw(self), core::mem::transmute(ptilexml.as_ptr()), ptilexml.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_UrlXML)(windows_core::Interface::as_raw(self), purlxml as _, pcburlxml as _).ok() }
    }
    pub unsafe fn set_UrlXML(&self, purlxml: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_UrlXML)(windows_core::Interface::as_raw(self), core::mem::transmute(purlxml.as_ptr()), purlxml.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn AttemptCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttemptCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_AttemptCount(&self, ulattemptcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_AttemptCount)(windows_core::Interface::as_raw(self), ulattemptcount).ok() }
    }
    pub unsafe fn DownloadState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DownloadState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_DownloadState(&self, uldownloadstate: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_DownloadState)(windows_core::Interface::as_raw(self), uldownloadstate).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TileID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub set_NextSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub StartSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub set_StartSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub IntervalDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_IntervalDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RunForever: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub set_RunForever: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub MaxRunCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_MaxRunCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RunCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_RunCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RecurrenceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_RecurrenceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_TileXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub set_TileXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub get_UrlXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub set_UrlXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub AttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_AttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub set_DownloadState: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IPMLiveTileJobInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NextSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_NextSchedule(&self, ftnextschedule: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn StartSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_StartSchedule(&self, ftstartschedule: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn IntervalDuration(&self) -> windows_core::Result<u32>;
    fn set_IntervalDuration(&self, ulintervalduration: u32) -> windows_core::Result<()>;
    fn RunForever(&self) -> windows_core::Result<windows_core::BOOL>;
    fn set_RunForever(&self, frunforever: windows_core::BOOL) -> windows_core::Result<()>;
    fn MaxRunCount(&self) -> windows_core::Result<u32>;
    fn set_MaxRunCount(&self, ulmaxruncount: u32) -> windows_core::Result<()>;
    fn RunCount(&self) -> windows_core::Result<u32>;
    fn set_RunCount(&self, ulruncount: u32) -> windows_core::Result<()>;
    fn RecurrenceType(&self) -> windows_core::Result<u32>;
    fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> windows_core::Result<()>;
    fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> windows_core::Result<()>;
    fn set_TileXML(&self, ptilexml: *const u8, cbtilexml: u32) -> windows_core::Result<()>;
    fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> windows_core::Result<()>;
    fn set_UrlXML(&self, purlxml: *const u8, cburlxml: u32) -> windows_core::Result<()>;
    fn AttemptCount(&self) -> windows_core::Result<u32>;
    fn set_AttemptCount(&self, ulattemptcount: u32) -> windows_core::Result<()>;
    fn DownloadState(&self) -> windows_core::Result<u32>;
    fn set_DownloadState(&self, uldownloadstate: u32) -> windows_core::Result<()>;
}
impl IPMLiveTileJobInfo_Vtbl {
    pub const fn new<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TileID<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptileid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::TileID(this, core::mem::transmute_copy(&ptileid)).into()
            }
        }
        unsafe extern "system" fn NextSchedule<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::NextSchedule(this) {
                    Ok(ok__) => {
                        pnextschedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_NextSchedule<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_NextSchedule(this, core::mem::transmute(&ftnextschedule)).into()
            }
        }
        unsafe extern "system" fn StartSchedule<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::StartSchedule(this) {
                    Ok(ok__) => {
                        pstartschedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_StartSchedule<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_StartSchedule(this, core::mem::transmute(&ftstartschedule)).into()
            }
        }
        unsafe extern "system" fn IntervalDuration<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintervalduration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::IntervalDuration(this) {
                    Ok(ok__) => {
                        pintervalduration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulintervalduration: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_IntervalDuration(this, core::mem::transmute_copy(&ulintervalduration)).into()
            }
        }
        unsafe extern "system" fn RunForever<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isrunforever: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::RunForever(this) {
                    Ok(ok__) => {
                        isrunforever.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_RunForever<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frunforever: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_RunForever(this, core::mem::transmute_copy(&frunforever)).into()
            }
        }
        unsafe extern "system" fn MaxRunCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxruncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::MaxRunCount(this) {
                    Ok(ok__) => {
                        pmaxruncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxruncount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_MaxRunCount(this, core::mem::transmute_copy(&ulmaxruncount)).into()
            }
        }
        unsafe extern "system" fn RunCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruncount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::RunCount(this) {
                    Ok(ok__) => {
                        pruncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_RunCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulruncount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_RunCount(this, core::mem::transmute_copy(&ulruncount)).into()
            }
        }
        unsafe extern "system" fn RecurrenceType<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precurrencetype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::RecurrenceType(this) {
                    Ok(ok__) => {
                        precurrencetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecurrencetype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_RecurrenceType(this, core::mem::transmute_copy(&ulrecurrencetype)).into()
            }
        }
        unsafe extern "system" fn get_TileXML<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::get_TileXML(this, core::mem::transmute_copy(&ptilexml), core::mem::transmute_copy(&pcbtilexml)).into()
            }
        }
        unsafe extern "system" fn set_TileXML<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_TileXML(this, core::mem::transmute_copy(&ptilexml), core::mem::transmute_copy(&cbtilexml)).into()
            }
        }
        unsafe extern "system" fn get_UrlXML<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::get_UrlXML(this, core::mem::transmute_copy(&purlxml), core::mem::transmute_copy(&pcburlxml)).into()
            }
        }
        unsafe extern "system" fn set_UrlXML<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_UrlXML(this, core::mem::transmute_copy(&purlxml), core::mem::transmute_copy(&cburlxml)).into()
            }
        }
        unsafe extern "system" fn AttemptCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattemptcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::AttemptCount(this) {
                    Ok(ok__) => {
                        pattemptcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_AttemptCount<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattemptcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_AttemptCount(this, core::mem::transmute_copy(&ulattemptcount)).into()
            }
        }
        unsafe extern "system" fn DownloadState<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdownloadstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfo_Impl::DownloadState(this) {
                    Ok(ok__) => {
                        pdownloadstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_DownloadState<Identity: IPMLiveTileJobInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldownloadstate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMLiveTileJobInfo_Impl::set_DownloadState(this, core::mem::transmute_copy(&uldownloadstate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TileID: TileID::<Identity, OFFSET>,
            NextSchedule: NextSchedule::<Identity, OFFSET>,
            set_NextSchedule: set_NextSchedule::<Identity, OFFSET>,
            StartSchedule: StartSchedule::<Identity, OFFSET>,
            set_StartSchedule: set_StartSchedule::<Identity, OFFSET>,
            IntervalDuration: IntervalDuration::<Identity, OFFSET>,
            set_IntervalDuration: set_IntervalDuration::<Identity, OFFSET>,
            RunForever: RunForever::<Identity, OFFSET>,
            set_RunForever: set_RunForever::<Identity, OFFSET>,
            MaxRunCount: MaxRunCount::<Identity, OFFSET>,
            set_MaxRunCount: set_MaxRunCount::<Identity, OFFSET>,
            RunCount: RunCount::<Identity, OFFSET>,
            set_RunCount: set_RunCount::<Identity, OFFSET>,
            RecurrenceType: RecurrenceType::<Identity, OFFSET>,
            set_RecurrenceType: set_RecurrenceType::<Identity, OFFSET>,
            get_TileXML: get_TileXML::<Identity, OFFSET>,
            set_TileXML: set_TileXML::<Identity, OFFSET>,
            get_UrlXML: get_UrlXML::<Identity, OFFSET>,
            set_UrlXML: set_UrlXML::<Identity, OFFSET>,
            AttemptCount: AttemptCount::<Identity, OFFSET>,
            set_AttemptCount: set_AttemptCount::<Identity, OFFSET>,
            DownloadState: DownloadState::<Identity, OFFSET>,
            set_DownloadState: set_DownloadState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMLiveTileJobInfo {}
windows_core::imp::define_interface!(IPMLiveTileJobInfoEnumerator, IPMLiveTileJobInfoEnumerator_Vtbl, 0xbc042582_9415_4f36_9f99_06f104c07c03);
windows_core::imp::interface_hierarchy!(IPMLiveTileJobInfoEnumerator, windows_core::IUnknown);
impl IPMLiveTileJobInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMLiveTileJobInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMLiveTileJobInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMLiveTileJobInfo>;
}
impl IPMLiveTileJobInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplivetilejobinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMLiveTileJobInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        pplivetilejobinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMLiveTileJobInfoEnumerator {}
windows_core::imp::define_interface!(IPMTaskInfo, IPMTaskInfo_Vtbl, 0xbf1d8c33_1bf5_4ee0_b549_6b9dd3834942);
windows_core::imp::interface_hierarchy!(IPMTaskInfo, windows_core::IUnknown);
impl IPMTaskInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TaskID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptaskid)).ok() }
    }
    pub unsafe fn NavigationPage(&self, pnavigationpage: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NavigationPage)(windows_core::Interface::as_raw(self), core::mem::transmute(pnavigationpage)).ok() }
    }
    pub unsafe fn TaskTransition(&self) -> windows_core::Result<PM_TASK_TRANSITION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskTransition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RuntimeType(&self) -> windows_core::Result<PACKMAN_RUNTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuntimeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActivationPolicy(&self) -> windows_core::Result<PM_ACTIVATION_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivationPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TaskType(&self) -> windows_core::Result<PM_TASK_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
    pub unsafe fn ImagePath(&self, pimagepath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImagePath)(windows_core::Interface::as_raw(self), core::mem::transmute(pimagepath)).ok() }
    }
    pub unsafe fn ImageParams(&self, pimageparams: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImageParams)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageparams)).ok() }
    }
    pub unsafe fn InstallRootFolder(&self, pinstallrootfolder: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallRootFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(pinstallrootfolder)).ok() }
    }
    pub unsafe fn DataRootFolder(&self, pdatarootfolder: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DataRootFolder)(windows_core::Interface::as_raw(self), core::mem::transmute(pdatarootfolder)).ok() }
    }
    pub unsafe fn IsSingleInstanceHost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSingleInstanceHost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsInteropEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInteropEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ApplicationState(&self) -> windows_core::Result<PM_APPLICATION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_Version)(windows_core::Interface::as_raw(self), ptargetmajorversion as _, ptargetminorversion as _).ok() }
    }
    pub unsafe fn BitsPerPixel(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BitsPerPixel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SuppressesDehydration(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SuppressesDehydration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackgroundExecutionAbilities)(windows_core::Interface::as_raw(self), core::mem::transmute(pbackgroundexecutionabilities)).ok() }
    }
    pub unsafe fn IsOptedForExtendedMem(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOptedForExtendedMem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NavigationPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_TASK_TRANSITION) -> windows_core::HRESULT,
    pub RuntimeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PACKMAN_RUNTIME) -> windows_core::HRESULT,
    pub ActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_ACTIVATION_POLICY) -> windows_core::HRESULT,
    pub TaskType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_TASK_TYPE) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImagePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImageParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallRootFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataRootFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSingleInstanceHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsInteropEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplicationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_APPLICATION_STATE) -> windows_core::HRESULT,
    pub InstallType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT,
    pub get_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u8) -> windows_core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SuppressesDehydration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub BackgroundExecutionAbilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOptedForExtendedMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMTaskInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NavigationPage(&self, pnavigationpage: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskTransition(&self) -> windows_core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(&self) -> windows_core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(&self) -> windows_core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(&self) -> windows_core::Result<PM_TASK_TYPE>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ImagePath(&self, pimagepath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ImageParams(&self, pimageparams: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallRootFolder(&self, pinstallrootfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DataRootFolder(&self, pdatarootfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSingleInstanceHost(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsInteropEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ApplicationState(&self) -> windows_core::Result<PM_APPLICATION_STATE>;
    fn InstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> windows_core::Result<()>;
    fn BitsPerPixel(&self) -> windows_core::Result<u16>;
    fn SuppressesDehydration(&self) -> windows_core::Result<windows_core::BOOL>;
    fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsOptedForExtendedMem(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IPMTaskInfo_Vtbl {
    pub const fn new<Identity: IPMTaskInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TaskID<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
            }
        }
        unsafe extern "system" fn NavigationPage<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnavigationpage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::NavigationPage(this, core::mem::transmute_copy(&pnavigationpage)).into()
            }
        }
        unsafe extern "system" fn TaskTransition<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::TaskTransition(this) {
                    Ok(ok__) => {
                        ptasktransition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RuntimeType<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::RuntimeType(this) {
                    Ok(ok__) => {
                        pruntimetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivationPolicy<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::ActivationPolicy(this) {
                    Ok(ok__) => {
                        pactivationpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TaskType<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::TaskType(this) {
                    Ok(ok__) => {
                        ptasktype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn ImagePath<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimagepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::ImagePath(this, core::mem::transmute_copy(&pimagepath)).into()
            }
        }
        unsafe extern "system" fn ImageParams<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::ImageParams(this, core::mem::transmute_copy(&pimageparams)).into()
            }
        }
        unsafe extern "system" fn InstallRootFolder<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallrootfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::InstallRootFolder(this, core::mem::transmute_copy(&pinstallrootfolder)).into()
            }
        }
        unsafe extern "system" fn DataRootFolder<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatarootfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::DataRootFolder(this, core::mem::transmute_copy(&pdatarootfolder)).into()
            }
        }
        unsafe extern "system" fn IsSingleInstanceHost<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pissingleinstancehost: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::IsSingleInstanceHost(this) {
                    Ok(ok__) => {
                        pissingleinstancehost.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinteropenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::IsInteropEnabled(this) {
                    Ok(ok__) => {
                        pisinteropenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationState<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::ApplicationState(this) {
                    Ok(ok__) => {
                        papplicationstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallType<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::InstallType(this) {
                    Ok(ok__) => {
                        pinstalltype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Version<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::get_Version(this, core::mem::transmute_copy(&ptargetmajorversion), core::mem::transmute_copy(&ptargetminorversion)).into()
            }
        }
        unsafe extern "system" fn BitsPerPixel<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitsperpixel: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::BitsPerPixel(this) {
                    Ok(ok__) => {
                        pbitsperpixel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psuppressesdehydration: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::SuppressesDehydration(this) {
                    Ok(ok__) => {
                        psuppressesdehydration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbackgroundexecutionabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTaskInfo_Impl::BackgroundExecutionAbilities(this, core::mem::transmute_copy(&pbackgroundexecutionabilities)).into()
            }
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Identity: IPMTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptedin: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfo_Impl::IsOptedForExtendedMem(this) {
                    Ok(ok__) => {
                        pisoptedin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            NavigationPage: NavigationPage::<Identity, OFFSET>,
            TaskTransition: TaskTransition::<Identity, OFFSET>,
            RuntimeType: RuntimeType::<Identity, OFFSET>,
            ActivationPolicy: ActivationPolicy::<Identity, OFFSET>,
            TaskType: TaskType::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            ImagePath: ImagePath::<Identity, OFFSET>,
            ImageParams: ImageParams::<Identity, OFFSET>,
            InstallRootFolder: InstallRootFolder::<Identity, OFFSET>,
            DataRootFolder: DataRootFolder::<Identity, OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Identity, OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Identity, OFFSET>,
            ApplicationState: ApplicationState::<Identity, OFFSET>,
            InstallType: InstallType::<Identity, OFFSET>,
            get_Version: get_Version::<Identity, OFFSET>,
            BitsPerPixel: BitsPerPixel::<Identity, OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Identity, OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Identity, OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTaskInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTaskInfo {}
windows_core::imp::define_interface!(IPMTaskInfoEnumerator, IPMTaskInfoEnumerator_Vtbl, 0x0630b0f8_0bbc_4821_be74_c7995166ed2a);
windows_core::imp::interface_hierarchy!(IPMTaskInfoEnumerator, windows_core::IUnknown);
impl IPMTaskInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMTaskInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMTaskInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMTaskInfo>;
}
impl IPMTaskInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMTaskInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTaskInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        pptaskinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTaskInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTaskInfoEnumerator {}
windows_core::imp::define_interface!(IPMTileInfo, IPMTileInfo_Vtbl, 0xd1604833_2b08_4001_82cd_183ad734f752);
windows_core::imp::interface_hierarchy!(IPMTileInfo, windows_core::IUnknown);
impl IPMTileInfo {
    pub unsafe fn ProductID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TileID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptileid)).ok() }
    }
    pub unsafe fn TemplateType(&self) -> windows_core::Result<TILE_TEMPLATE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TemplateType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HubPinnedState)(windows_core::Interface::as_raw(self), hubtype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HubPosition)(windows_core::Interface::as_raw(self), hubtype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsNotified(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNotified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TaskID)(windows_core::Interface::as_raw(self), core::mem::transmute(ptaskid)).ok() }
    }
    pub unsafe fn TileType(&self) -> windows_core::Result<PM_STARTTILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsThemable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThemable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PropertyById(&self, propid: u32) -> windows_core::Result<IPMTilePropertyInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyById)(windows_core::Interface::as_raw(self), propid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pimageurn), core::mem::transmute(pparameters)).ok() }
    }
    pub unsafe fn PropertyEnum(&self) -> windows_core::Result<IPMTilePropertyEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<PM_TILE_SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HubTileSize)(windows_core::Interface::as_raw(self), hubtype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_HubPosition)(windows_core::Interface::as_raw(self), hubtype, position).ok() }
    }
    pub unsafe fn set_NotifiedState(&self, notified: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_NotifiedState)(windows_core::Interface::as_raw(self), notified.into()).ok() }
    }
    pub unsafe fn set_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE, pinned: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_HubPinnedState)(windows_core::Interface::as_raw(self), hubtype, pinned.into()).ok() }
    }
    pub unsafe fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_HubTileSize)(windows_core::Interface::as_raw(self), hubtype, size).ok() }
    }
    pub unsafe fn set_InvocationInfo(&self, taskname: &windows_core::BSTR, taskparameters: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_InvocationInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname), core::mem::transmute_copy(taskparameters)).ok() }
    }
    pub unsafe fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartTileBlob)(windows_core::Interface::as_raw(self), core::mem::transmute(pblob)).ok() }
    }
    pub unsafe fn IsRestoring(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRestoring)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAutoRestoreDisabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAutoRestoreDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn set_IsRestoring(&self, restoring: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IsRestoring)(windows_core::Interface::as_raw(self), restoring.into()).ok() }
    }
    pub unsafe fn set_IsAutoRestoreDisabled(&self, autorestoredisabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_IsAutoRestoreDisabled)(windows_core::Interface::as_raw(self), autorestoredisabled.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TileID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TemplateType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TILE_TEMPLATE_TYPE) -> windows_core::HRESULT,
    pub get_HubPinnedState: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_HubPosition: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, *mut u32) -> windows_core::HRESULT,
    pub IsNotified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_STARTTILE_TYPE) -> windows_core::HRESULT,
    pub IsThemable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_PropertyById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertyEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HubTileSize: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, *mut PM_TILE_SIZE) -> windows_core::HRESULT,
    pub set_HubPosition: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, u32) -> windows_core::HRESULT,
    pub set_NotifiedState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_HubPinnedState: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_HubTileSize: unsafe extern "system" fn(*mut core::ffi::c_void, PM_TILE_HUBTYPE, PM_TILE_SIZE) -> windows_core::HRESULT,
    pub set_InvocationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTileBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PM_STARTTILEBLOB) -> windows_core::HRESULT,
    pub IsRestoring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsAutoRestoreDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IsRestoring: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub set_IsAutoRestoreDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IPMTileInfo_Impl: windows_core::IUnknownImpl {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateType(&self) -> windows_core::Result<TILE_TEMPLATE_TYPE>;
    fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<windows_core::BOOL>;
    fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<u32>;
    fn IsNotified(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TileType(&self) -> windows_core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_PropertyById(&self, propid: u32) -> windows_core::Result<IPMTilePropertyInfo>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn PropertyEnum(&self) -> windows_core::Result<IPMTilePropertyEnumerator>;
    fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> windows_core::Result<()>;
    fn set_NotifiedState(&self, notified: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE, pinned: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> windows_core::Result<()>;
    fn set_InvocationInfo(&self, taskname: &windows_core::BSTR, taskparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> windows_core::Result<()>;
    fn IsRestoring(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsAutoRestoreDisabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn set_IsRestoring(&self, restoring: windows_core::BOOL) -> windows_core::Result<()>;
    fn set_IsAutoRestoreDisabled(&self, autorestoredisabled: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IPMTileInfo_Vtbl {
    pub const fn new<Identity: IPMTileInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductID<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::ProductID(this) {
                    Ok(ok__) => {
                        pproductid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TileID<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptileid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::TileID(this, core::mem::transmute_copy(&ptileid)).into()
            }
        }
        unsafe extern "system" fn TemplateType<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::TemplateType(this) {
                    Ok(ok__) => {
                        ptemplatetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HubPinnedState<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::get_HubPinnedState(this, core::mem::transmute_copy(&hubtype)) {
                    Ok(ok__) => {
                        ppinned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HubPosition<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::get_HubPosition(this, core::mem::transmute_copy(&hubtype)) {
                    Ok(ok__) => {
                        pposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNotified<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisnotified: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::IsNotified(this) {
                    Ok(ok__) => {
                        pisnotified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDefault<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::IsDefault(this) {
                    Ok(ok__) => {
                        pisdefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TaskID<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
            }
        }
        unsafe extern "system" fn TileType<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::TileType(this) {
                    Ok(ok__) => {
                        pstarttiletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsThemable<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisthemable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::IsThemable(this) {
                    Ok(ok__) => {
                        pisthemable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropertyById<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, pppropinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::get_PropertyById(this, core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        pppropinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut *mut core::ffi::c_void, pparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn PropertyEnum<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptilepropenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::PropertyEnum(this) {
                    Ok(ok__) => {
                        pptilepropenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HubTileSize<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::get_HubTileSize(this, core::mem::transmute_copy(&hubtype)) {
                    Ok(ok__) => {
                        psize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_HubPosition<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_HubPosition(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&position)).into()
            }
        }
        unsafe extern "system" fn set_NotifiedState<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notified: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_NotifiedState(this, core::mem::transmute_copy(&notified)).into()
            }
        }
        unsafe extern "system" fn set_HubPinnedState<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_HubPinnedState(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&pinned)).into()
            }
        }
        unsafe extern "system" fn set_HubTileSize<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_HubTileSize(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn set_InvocationInfo<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void, taskparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_InvocationInfo(this, core::mem::transmute(&taskname), core::mem::transmute(&taskparameters)).into()
            }
        }
        unsafe extern "system" fn StartTileBlob<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::StartTileBlob(this, core::mem::transmute_copy(&pblob)).into()
            }
        }
        unsafe extern "system" fn IsRestoring<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrestoring: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::IsRestoring(this) {
                    Ok(ok__) => {
                        pisrestoring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisautorestoredisabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfo_Impl::IsAutoRestoreDisabled(this) {
                    Ok(ok__) => {
                        pisautorestoredisabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn set_IsRestoring<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoring: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_IsRestoring(this, core::mem::transmute_copy(&restoring)).into()
            }
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Identity: IPMTileInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autorestoredisabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTileInfo_Impl::set_IsAutoRestoreDisabled(this, core::mem::transmute_copy(&autorestoredisabled)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TileID: TileID::<Identity, OFFSET>,
            TemplateType: TemplateType::<Identity, OFFSET>,
            get_HubPinnedState: get_HubPinnedState::<Identity, OFFSET>,
            get_HubPosition: get_HubPosition::<Identity, OFFSET>,
            IsNotified: IsNotified::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            TileType: TileType::<Identity, OFFSET>,
            IsThemable: IsThemable::<Identity, OFFSET>,
            get_PropertyById: get_PropertyById::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            PropertyEnum: PropertyEnum::<Identity, OFFSET>,
            get_HubTileSize: get_HubTileSize::<Identity, OFFSET>,
            set_HubPosition: set_HubPosition::<Identity, OFFSET>,
            set_NotifiedState: set_NotifiedState::<Identity, OFFSET>,
            set_HubPinnedState: set_HubPinnedState::<Identity, OFFSET>,
            set_HubTileSize: set_HubTileSize::<Identity, OFFSET>,
            set_InvocationInfo: set_InvocationInfo::<Identity, OFFSET>,
            StartTileBlob: StartTileBlob::<Identity, OFFSET>,
            IsRestoring: IsRestoring::<Identity, OFFSET>,
            IsAutoRestoreDisabled: IsAutoRestoreDisabled::<Identity, OFFSET>,
            set_IsRestoring: set_IsRestoring::<Identity, OFFSET>,
            set_IsAutoRestoreDisabled: set_IsAutoRestoreDisabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTileInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTileInfo {}
windows_core::imp::define_interface!(IPMTileInfoEnumerator, IPMTileInfoEnumerator_Vtbl, 0xded83065_e462_4b2c_acb5_e39cea61c874);
windows_core::imp::interface_hierarchy!(IPMTileInfoEnumerator, windows_core::IUnknown);
impl IPMTileInfoEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMTileInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMTileInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMTileInfo>;
}
impl IPMTileInfoEnumerator_Vtbl {
    pub const fn new<Identity: IPMTileInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMTileInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptileinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTileInfoEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        pptileinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTileInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTileInfoEnumerator {}
windows_core::imp::define_interface!(IPMTilePropertyEnumerator, IPMTilePropertyEnumerator_Vtbl, 0xcc4cd629_9047_4250_aac8_930e47812421);
windows_core::imp::interface_hierarchy!(IPMTilePropertyEnumerator, windows_core::IUnknown);
impl IPMTilePropertyEnumerator {
    pub unsafe fn Next(&self) -> windows_core::Result<IPMTilePropertyInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMTilePropertyEnumerator_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<IPMTilePropertyInfo>;
}
impl IPMTilePropertyEnumerator_Vtbl {
    pub const fn new<Identity: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IPMTilePropertyEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTilePropertyEnumerator_Impl::Next(this) {
                    Ok(ok__) => {
                        pppropinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTilePropertyEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTilePropertyEnumerator {}
windows_core::imp::define_interface!(IPMTilePropertyInfo, IPMTilePropertyInfo_Vtbl, 0x6c2b8017_1efa_42a7_86c0_6d4b640bf528);
windows_core::imp::interface_hierarchy!(IPMTilePropertyInfo, windows_core::IUnknown);
impl IPMTilePropertyInfo {
    pub unsafe fn PropertyID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropertyValue(&self, ppropvalue: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PropertyValue)(windows_core::Interface::as_raw(self), core::mem::transmute(ppropvalue)).ok() }
    }
    pub unsafe fn set_Property(&self, propvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).set_Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PropertyID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub set_Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPMTilePropertyInfo_Impl: windows_core::IUnknownImpl {
    fn PropertyID(&self) -> windows_core::Result<u32>;
    fn PropertyValue(&self, ppropvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn set_Property(&self, propvalue: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IPMTilePropertyInfo_Vtbl {
    pub const fn new<Identity: IPMTilePropertyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyID<Identity: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPMTilePropertyInfo_Impl::PropertyID(this) {
                    Ok(ok__) => {
                        ppropid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTilePropertyInfo_Impl::PropertyValue(this, core::mem::transmute_copy(&ppropvalue)).into()
            }
        }
        unsafe extern "system" fn set_Property<Identity: IPMTilePropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPMTilePropertyInfo_Impl::set_Property(this, core::mem::transmute(&propvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropertyID: PropertyID::<Identity, OFFSET>,
            PropertyValue: PropertyValue::<Identity, OFFSET>,
            set_Property: set_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTilePropertyInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPMTilePropertyInfo {}
pub const IPROPNAME_ACTION: windows_core::PCWSTR = windows_core::w!("ACTION");
pub const IPROPNAME_ADMINTOOLS_FOLDER: windows_core::PCWSTR = windows_core::w!("AdminToolsFolder");
pub const IPROPNAME_ADMINUSER: windows_core::PCWSTR = windows_core::w!("AdminUser");
pub const IPROPNAME_ADMIN_PROPERTIES: windows_core::PCWSTR = windows_core::w!("AdminProperties");
pub const IPROPNAME_AFTERREBOOT: windows_core::PCWSTR = windows_core::w!("AFTERREBOOT");
pub const IPROPNAME_ALLOWEDPROPERTIES: windows_core::PCWSTR = windows_core::w!("SecureCustomProperties");
pub const IPROPNAME_ALLUSERS: windows_core::PCWSTR = windows_core::w!("ALLUSERS");
pub const IPROPNAME_APPDATA_FOLDER: windows_core::PCWSTR = windows_core::w!("AppDataFolder");
pub const IPROPNAME_ARM: windows_core::PCWSTR = windows_core::w!("Arm");
pub const IPROPNAME_ARM64: windows_core::PCWSTR = windows_core::w!("Arm64");
pub const IPROPNAME_ARPAUTHORIZEDCDFPREFIX: windows_core::PCWSTR = windows_core::w!("ARPAUTHORIZEDCDFPREFIX");
pub const IPROPNAME_ARPCOMMENTS: windows_core::PCWSTR = windows_core::w!("ARPCOMMENTS");
pub const IPROPNAME_ARPCONTACT: windows_core::PCWSTR = windows_core::w!("ARPCONTACT");
pub const IPROPNAME_ARPHELPLINK: windows_core::PCWSTR = windows_core::w!("ARPHELPLINK");
pub const IPROPNAME_ARPHELPTELEPHONE: windows_core::PCWSTR = windows_core::w!("ARPHELPTELEPHONE");
pub const IPROPNAME_ARPINSTALLLOCATION: windows_core::PCWSTR = windows_core::w!("ARPINSTALLLOCATION");
pub const IPROPNAME_ARPNOMODIFY: windows_core::PCWSTR = windows_core::w!("ARPNOMODIFY");
pub const IPROPNAME_ARPNOREMOVE: windows_core::PCWSTR = windows_core::w!("ARPNOREMOVE");
pub const IPROPNAME_ARPNOREPAIR: windows_core::PCWSTR = windows_core::w!("ARPNOREPAIR");
pub const IPROPNAME_ARPPRODUCTICON: windows_core::PCWSTR = windows_core::w!("ARPPRODUCTICON");
pub const IPROPNAME_ARPREADME: windows_core::PCWSTR = windows_core::w!("ARPREADME");
pub const IPROPNAME_ARPSETTINGSIDENTIFIER: windows_core::PCWSTR = windows_core::w!("MSIARPSETTINGSIDENTIFIER");
pub const IPROPNAME_ARPSHIMFLAGS: windows_core::PCWSTR = windows_core::w!("SHIMFLAGS");
pub const IPROPNAME_ARPSHIMSERVICEPACKLEVEL: windows_core::PCWSTR = windows_core::w!("SHIMSERVICEPACKLEVEL");
pub const IPROPNAME_ARPSHIMVERSIONNT: windows_core::PCWSTR = windows_core::w!("SHIMVERSIONNT");
pub const IPROPNAME_ARPSIZE: windows_core::PCWSTR = windows_core::w!("ARPSIZE");
pub const IPROPNAME_ARPSYSTEMCOMPONENT: windows_core::PCWSTR = windows_core::w!("ARPSYSTEMCOMPONENT");
pub const IPROPNAME_ARPURLINFOABOUT: windows_core::PCWSTR = windows_core::w!("ARPURLINFOABOUT");
pub const IPROPNAME_ARPURLUPDATEINFO: windows_core::PCWSTR = windows_core::w!("ARPURLUPDATEINFO");
pub const IPROPNAME_AVAILABLEFREEREG: windows_core::PCWSTR = windows_core::w!("AVAILABLEFREEREG");
pub const IPROPNAME_BORDERSIDE: windows_core::PCWSTR = windows_core::w!("BorderSide");
pub const IPROPNAME_BORDERTOP: windows_core::PCWSTR = windows_core::w!("BorderTop");
pub const IPROPNAME_CAPTIONHEIGHT: windows_core::PCWSTR = windows_core::w!("CaptionHeight");
pub const IPROPNAME_CARRYINGNDP: windows_core::PCWSTR = windows_core::w!("CARRYINGNDP");
pub const IPROPNAME_CHECKCRCS: windows_core::PCWSTR = windows_core::w!("MSICHECKCRCS");
pub const IPROPNAME_COLORBITS: windows_core::PCWSTR = windows_core::w!("ColorBits");
pub const IPROPNAME_COMMONAPPDATA_FOLDER: windows_core::PCWSTR = windows_core::w!("CommonAppDataFolder");
pub const IPROPNAME_COMMONFILES64_FOLDER: windows_core::PCWSTR = windows_core::w!("CommonFiles64Folder");
pub const IPROPNAME_COMMONFILES_FOLDER: windows_core::PCWSTR = windows_core::w!("CommonFilesFolder");
pub const IPROPNAME_COMPANYNAME: windows_core::PCWSTR = windows_core::w!("COMPANYNAME");
pub const IPROPNAME_COMPONENTADDDEFAULT: windows_core::PCWSTR = windows_core::w!("COMPADDDEFAULT");
pub const IPROPNAME_COMPONENTADDLOCAL: windows_core::PCWSTR = windows_core::w!("COMPADDLOCAL");
pub const IPROPNAME_COMPONENTADDSOURCE: windows_core::PCWSTR = windows_core::w!("COMPADDSOURCE");
pub const IPROPNAME_COMPUTERNAME: windows_core::PCWSTR = windows_core::w!("ComputerName");
pub const IPROPNAME_COSTINGCOMPLETE: windows_core::PCWSTR = windows_core::w!("CostingComplete");
pub const IPROPNAME_CUSTOMACTIONDATA: windows_core::PCWSTR = windows_core::w!("CustomActionData");
pub const IPROPNAME_DATE: windows_core::PCWSTR = windows_core::w!("Date");
pub const IPROPNAME_DATETIME: windows_core::PCWSTR = windows_core::w!("DateTime");
pub const IPROPNAME_DEFAULTUIFONT: windows_core::PCWSTR = windows_core::w!("DefaultUIFont");
pub const IPROPNAME_DESKTOP_FOLDER: windows_core::PCWSTR = windows_core::w!("DesktopFolder");
pub const IPROPNAME_DISABLEADVTSHORTCUTS: windows_core::PCWSTR = windows_core::w!("DISABLEADVTSHORTCUTS");
pub const IPROPNAME_DISABLEROLLBACK: windows_core::PCWSTR = windows_core::w!("DISABLEROLLBACK");
pub const IPROPNAME_DISKPROMPT: windows_core::PCWSTR = windows_core::w!("DiskPrompt");
pub const IPROPNAME_ENABLEUSERCONTROL: windows_core::PCWSTR = windows_core::w!("EnableUserControl");
pub const IPROPNAME_ENFORCE_UPGRADE_COMPONENT_RULES: windows_core::PCWSTR = windows_core::w!("MSIENFORCEUPGRADECOMPONENTRULES");
pub const IPROPNAME_EXECUTEACTION: windows_core::PCWSTR = windows_core::w!("EXECUTEACTION");
pub const IPROPNAME_EXECUTEMODE: windows_core::PCWSTR = windows_core::w!("EXECUTEMODE");
pub const IPROPNAME_FAVORITES_FOLDER: windows_core::PCWSTR = windows_core::w!("FavoritesFolder");
pub const IPROPNAME_FEATUREADDDEFAULT: windows_core::PCWSTR = windows_core::w!("ADDDEFAULT");
pub const IPROPNAME_FEATUREADDLOCAL: windows_core::PCWSTR = windows_core::w!("ADDLOCAL");
pub const IPROPNAME_FEATUREADDSOURCE: windows_core::PCWSTR = windows_core::w!("ADDSOURCE");
pub const IPROPNAME_FEATUREADVERTISE: windows_core::PCWSTR = windows_core::w!("ADVERTISE");
pub const IPROPNAME_FEATUREREMOVE: windows_core::PCWSTR = windows_core::w!("REMOVE");
pub const IPROPNAME_FILEADDDEFAULT: windows_core::PCWSTR = windows_core::w!("FILEADDDEFAULT");
pub const IPROPNAME_FILEADDLOCAL: windows_core::PCWSTR = windows_core::w!("FILEADDLOCAL");
pub const IPROPNAME_FILEADDSOURCE: windows_core::PCWSTR = windows_core::w!("FILEADDSOURCE");
pub const IPROPNAME_FONTS_FOLDER: windows_core::PCWSTR = windows_core::w!("FontsFolder");
pub const IPROPNAME_HIDDEN_PROPERTIES: windows_core::PCWSTR = windows_core::w!("MsiHiddenProperties");
pub const IPROPNAME_HIDECANCEL: windows_core::PCWSTR = windows_core::w!("MsiUIHideCancel");
pub const IPROPNAME_IA64: windows_core::PCWSTR = windows_core::w!("IA64");
pub const IPROPNAME_INSTALLED: windows_core::PCWSTR = windows_core::w!("Installed");
pub const IPROPNAME_INSTALLLANGUAGE: windows_core::PCWSTR = windows_core::w!("ProductLanguage");
pub const IPROPNAME_INSTALLLEVEL: windows_core::PCWSTR = windows_core::w!("INSTALLLEVEL");
pub const IPROPNAME_INSTALLPERUSER: windows_core::PCWSTR = windows_core::w!("MSIINSTALLPERUSER");
pub const IPROPNAME_INTEL: windows_core::PCWSTR = windows_core::w!("Intel");
pub const IPROPNAME_INTEL64: windows_core::PCWSTR = windows_core::w!("Intel64");
pub const IPROPNAME_INTERNALINSTALLEDPERUSER: windows_core::PCWSTR = windows_core::w!("MSIINTERNALINSTALLEDPERUSER");
pub const IPROPNAME_ISADMINPACKAGE: windows_core::PCWSTR = windows_core::w!("IsAdminPackage");
pub const IPROPNAME_LEFTUNIT: windows_core::PCWSTR = windows_core::w!("LeftUnit");
pub const IPROPNAME_LIMITUI: windows_core::PCWSTR = windows_core::w!("LIMITUI");
pub const IPROPNAME_LOCALAPPDATA_FOLDER: windows_core::PCWSTR = windows_core::w!("LocalAppDataFolder");
pub const IPROPNAME_LOGACTION: windows_core::PCWSTR = windows_core::w!("LOGACTION");
pub const IPROPNAME_LOGONUSER: windows_core::PCWSTR = windows_core::w!("LogonUser");
pub const IPROPNAME_MANUFACTURER: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const IPROPNAME_MSIAMD64: windows_core::PCWSTR = windows_core::w!("MsiAMD64");
pub const IPROPNAME_MSIDISABLEEEUI: windows_core::PCWSTR = windows_core::w!("MSIDISABLEEEUI");
pub const IPROPNAME_MSIDISABLELUAPATCHING: windows_core::PCWSTR = windows_core::w!("MSIDISABLELUAPATCHING");
pub const IPROPNAME_MSIINSTANCEGUID: windows_core::PCWSTR = windows_core::w!("MSIINSTANCEGUID");
pub const IPROPNAME_MSILOGFILELOCATION: windows_core::PCWSTR = windows_core::w!("MsiLogFileLocation");
pub const IPROPNAME_MSILOGGINGMODE: windows_core::PCWSTR = windows_core::w!("MsiLogging");
pub const IPROPNAME_MSINEWINSTANCE: windows_core::PCWSTR = windows_core::w!("MSINEWINSTANCE");
pub const IPROPNAME_MSINODISABLEMEDIA: windows_core::PCWSTR = windows_core::w!("MSINODISABLEMEDIA");
pub const IPROPNAME_MSIPACKAGEDOWNLOADLOCALCOPY: windows_core::PCWSTR = windows_core::w!("MSIPACKAGEDOWNLOADLOCALCOPY");
pub const IPROPNAME_MSIPATCHDOWNLOADLOCALCOPY: windows_core::PCWSTR = windows_core::w!("MSIPATCHDOWNLOADLOCALCOPY");
pub const IPROPNAME_MSIPATCHREMOVE: windows_core::PCWSTR = windows_core::w!("MSIPATCHREMOVE");
pub const IPROPNAME_MSITABLETPC: windows_core::PCWSTR = windows_core::w!("MsiTabletPC");
pub const IPROPNAME_MSIX64: windows_core::PCWSTR = windows_core::w!("Msix64");
pub const IPROPNAME_MSI_FASTINSTALL: windows_core::PCWSTR = windows_core::w!("MSIFASTINSTALL");
pub const IPROPNAME_MSI_REBOOT_PENDING: windows_core::PCWSTR = windows_core::w!("MsiSystemRebootPending");
pub const IPROPNAME_MSI_RM_CONTROL: windows_core::PCWSTR = windows_core::w!("MSIRESTARTMANAGERCONTROL");
pub const IPROPNAME_MSI_RM_DISABLE_RESTART: windows_core::PCWSTR = windows_core::w!("MSIDISABLERMRESTART");
pub const IPROPNAME_MSI_RM_SESSION_KEY: windows_core::PCWSTR = windows_core::w!("MsiRestartManagerSessionKey");
pub const IPROPNAME_MSI_RM_SHUTDOWN: windows_core::PCWSTR = windows_core::w!("MSIRMSHUTDOWN");
pub const IPROPNAME_MSI_UAC_DEPLOYMENT_COMPLIANT: windows_core::PCWSTR = windows_core::w!("MSIDEPLOYMENTCOMPLIANT");
pub const IPROPNAME_MSI_UNINSTALL_SUPERSEDED_COMPONENTS: windows_core::PCWSTR = windows_core::w!("MSIUNINSTALLSUPERSEDEDCOMPONENTS");
pub const IPROPNAME_MSI_USE_REAL_ADMIN_DETECTION: windows_core::PCWSTR = windows_core::w!("MSIUSEREALADMINDETECTION");
pub const IPROPNAME_MYPICTURES_FOLDER: windows_core::PCWSTR = windows_core::w!("MyPicturesFolder");
pub const IPROPNAME_NETASSEMBLYSUPPORT: windows_core::PCWSTR = windows_core::w!("MsiNetAssemblySupport");
pub const IPROPNAME_NETHOOD_FOLDER: windows_core::PCWSTR = windows_core::w!("NetHoodFolder");
pub const IPROPNAME_NOCOMPANYNAME: windows_core::PCWSTR = windows_core::w!("NOCOMPANYNAME");
pub const IPROPNAME_NOUSERNAME: windows_core::PCWSTR = windows_core::w!("NOUSERNAME");
pub const IPROPNAME_NTPRODUCTTYPE: windows_core::PCWSTR = windows_core::w!("MsiNTProductType");
pub const IPROPNAME_NTSUITEBACKOFFICE: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteBackOffice");
pub const IPROPNAME_NTSUITEDATACENTER: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteDataCenter");
pub const IPROPNAME_NTSUITEENTERPRISE: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteEnterprise");
pub const IPROPNAME_NTSUITEPERSONAL: windows_core::PCWSTR = windows_core::w!("MsiNTSuitePersonal");
pub const IPROPNAME_NTSUITESMALLBUSINESS: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteSmallBusiness");
pub const IPROPNAME_NTSUITESMALLBUSINESSRESTRICTED: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteSmallBusinessRestricted");
pub const IPROPNAME_NTSUITEWEBSERVER: windows_core::PCWSTR = windows_core::w!("MsiNTSuiteWebServer");
pub const IPROPNAME_OLEADVTSUPPORT: windows_core::PCWSTR = windows_core::w!("OLEAdvtSupport");
pub const IPROPNAME_OUTOFDISKSPACE: windows_core::PCWSTR = windows_core::w!("OutOfDiskSpace");
pub const IPROPNAME_OUTOFNORBDISKSPACE: windows_core::PCWSTR = windows_core::w!("OutOfNoRbDiskSpace");
pub const IPROPNAME_PATCH: windows_core::PCWSTR = windows_core::w!("PATCH");
pub const IPROPNAME_PATCHNEWPACKAGECODE: windows_core::PCWSTR = windows_core::w!("PATCHNEWPACKAGECODE");
pub const IPROPNAME_PATCHNEWSUMMARYCOMMENTS: windows_core::PCWSTR = windows_core::w!("PATCHNEWSUMMARYCOMMENTS");
pub const IPROPNAME_PATCHNEWSUMMARYSUBJECT: windows_core::PCWSTR = windows_core::w!("PATCHNEWSUMMARYSUBJECT");
pub const IPROPNAME_PERSONAL_FOLDER: windows_core::PCWSTR = windows_core::w!("PersonalFolder");
pub const IPROPNAME_PHYSICALMEMORY: windows_core::PCWSTR = windows_core::w!("PhysicalMemory");
pub const IPROPNAME_PIDKEY: windows_core::PCWSTR = windows_core::w!("PIDKEY");
pub const IPROPNAME_PIDTEMPLATE: windows_core::PCWSTR = windows_core::w!("PIDTemplate");
pub const IPROPNAME_PRESELECTED: windows_core::PCWSTR = windows_core::w!("Preselected");
pub const IPROPNAME_PRIMARYFOLDER: windows_core::PCWSTR = windows_core::w!("PRIMARYFOLDER");
pub const IPROPNAME_PRIMARYFOLDER_PATH: windows_core::PCWSTR = windows_core::w!("PrimaryVolumePath");
pub const IPROPNAME_PRIMARYFOLDER_SPACEAVAILABLE: windows_core::PCWSTR = windows_core::w!("PrimaryVolumeSpaceAvailable");
pub const IPROPNAME_PRIMARYFOLDER_SPACEREMAINING: windows_core::PCWSTR = windows_core::w!("PrimaryVolumeSpaceRemaining");
pub const IPROPNAME_PRIMARYFOLDER_SPACEREQUIRED: windows_core::PCWSTR = windows_core::w!("PrimaryVolumeSpaceRequired");
pub const IPROPNAME_PRINTHOOD_FOLDER: windows_core::PCWSTR = windows_core::w!("PrintHoodFolder");
pub const IPROPNAME_PRIVILEGED: windows_core::PCWSTR = windows_core::w!("Privileged");
pub const IPROPNAME_PRODUCTCODE: windows_core::PCWSTR = windows_core::w!("ProductCode");
pub const IPROPNAME_PRODUCTID: windows_core::PCWSTR = windows_core::w!("ProductID");
pub const IPROPNAME_PRODUCTLANGUAGE: windows_core::PCWSTR = windows_core::w!("PRODUCTLANGUAGE");
pub const IPROPNAME_PRODUCTNAME: windows_core::PCWSTR = windows_core::w!("ProductName");
pub const IPROPNAME_PRODUCTSTATE: windows_core::PCWSTR = windows_core::w!("ProductState");
pub const IPROPNAME_PRODUCTVERSION: windows_core::PCWSTR = windows_core::w!("ProductVersion");
pub const IPROPNAME_PROGRAMFILES64_FOLDER: windows_core::PCWSTR = windows_core::w!("ProgramFiles64Folder");
pub const IPROPNAME_PROGRAMFILES_FOLDER: windows_core::PCWSTR = windows_core::w!("ProgramFilesFolder");
pub const IPROPNAME_PROGRAMMENU_FOLDER: windows_core::PCWSTR = windows_core::w!("ProgramMenuFolder");
pub const IPROPNAME_PROGRESSONLY: windows_core::PCWSTR = windows_core::w!("MsiUIProgressOnly");
pub const IPROPNAME_PROMPTROLLBACKCOST: windows_core::PCWSTR = windows_core::w!("PROMPTROLLBACKCOST");
pub const IPROPNAME_REBOOT: windows_core::PCWSTR = windows_core::w!("REBOOT");
pub const IPROPNAME_REBOOTPROMPT: windows_core::PCWSTR = windows_core::w!("REBOOTPROMPT");
pub const IPROPNAME_RECENT_FOLDER: windows_core::PCWSTR = windows_core::w!("RecentFolder");
pub const IPROPNAME_REDIRECTEDDLLSUPPORT: windows_core::PCWSTR = windows_core::w!("RedirectedDllSupport");
pub const IPROPNAME_REINSTALL: windows_core::PCWSTR = windows_core::w!("REINSTALL");
pub const IPROPNAME_REINSTALLMODE: windows_core::PCWSTR = windows_core::w!("REINSTALLMODE");
pub const IPROPNAME_REMOTEADMINTS: windows_core::PCWSTR = windows_core::w!("RemoteAdminTS");
pub const IPROPNAME_REPLACEDINUSEFILES: windows_core::PCWSTR = windows_core::w!("ReplacedInUseFiles");
pub const IPROPNAME_RESTRICTEDUSERCONTROL: windows_core::PCWSTR = windows_core::w!("RestrictedUserControl");
pub const IPROPNAME_RESUME: windows_core::PCWSTR = windows_core::w!("RESUME");
pub const IPROPNAME_ROLLBACKDISABLED: windows_core::PCWSTR = windows_core::w!("RollbackDisabled");
pub const IPROPNAME_ROOTDRIVE: windows_core::PCWSTR = windows_core::w!("ROOTDRIVE");
pub const IPROPNAME_RUNNINGELEVATED: windows_core::PCWSTR = windows_core::w!("MsiRunningElevated");
pub const IPROPNAME_SCREENX: windows_core::PCWSTR = windows_core::w!("ScreenX");
pub const IPROPNAME_SCREENY: windows_core::PCWSTR = windows_core::w!("ScreenY");
pub const IPROPNAME_SENDTO_FOLDER: windows_core::PCWSTR = windows_core::w!("SendToFolder");
pub const IPROPNAME_SEQUENCE: windows_core::PCWSTR = windows_core::w!("SEQUENCE");
pub const IPROPNAME_SERVICEPACKLEVEL: windows_core::PCWSTR = windows_core::w!("ServicePackLevel");
pub const IPROPNAME_SERVICEPACKLEVELMINOR: windows_core::PCWSTR = windows_core::w!("ServicePackLevelMinor");
pub const IPROPNAME_SHAREDWINDOWS: windows_core::PCWSTR = windows_core::w!("SharedWindows");
pub const IPROPNAME_SHELLADVTSUPPORT: windows_core::PCWSTR = windows_core::w!("ShellAdvtSupport");
pub const IPROPNAME_SHORTFILENAMES: windows_core::PCWSTR = windows_core::w!("SHORTFILENAMES");
pub const IPROPNAME_SOURCEDIR: windows_core::PCWSTR = windows_core::w!("SourceDir");
pub const IPROPNAME_SOURCELIST: windows_core::PCWSTR = windows_core::w!("SOURCELIST");
pub const IPROPNAME_SOURCERESONLY: windows_core::PCWSTR = windows_core::w!("MsiUISourceResOnly");
pub const IPROPNAME_STARTMENU_FOLDER: windows_core::PCWSTR = windows_core::w!("StartMenuFolder");
pub const IPROPNAME_STARTUP_FOLDER: windows_core::PCWSTR = windows_core::w!("StartupFolder");
pub const IPROPNAME_SYSTEM16_FOLDER: windows_core::PCWSTR = windows_core::w!("System16Folder");
pub const IPROPNAME_SYSTEM64_FOLDER: windows_core::PCWSTR = windows_core::w!("System64Folder");
pub const IPROPNAME_SYSTEMLANGUAGEID: windows_core::PCWSTR = windows_core::w!("SystemLanguageID");
pub const IPROPNAME_SYSTEM_FOLDER: windows_core::PCWSTR = windows_core::w!("SystemFolder");
pub const IPROPNAME_TARGETDIR: windows_core::PCWSTR = windows_core::w!("TARGETDIR");
pub const IPROPNAME_TEMPLATE_AMD64: windows_core::PCWSTR = windows_core::w!("AMD64");
pub const IPROPNAME_TEMPLATE_FOLDER: windows_core::PCWSTR = windows_core::w!("TemplateFolder");
pub const IPROPNAME_TEMPLATE_X64: windows_core::PCWSTR = windows_core::w!("x64");
pub const IPROPNAME_TEMP_FOLDER: windows_core::PCWSTR = windows_core::w!("TempFolder");
pub const IPROPNAME_TERMSERVER: windows_core::PCWSTR = windows_core::w!("TerminalServer");
pub const IPROPNAME_TEXTHEIGHT: windows_core::PCWSTR = windows_core::w!("TextHeight");
pub const IPROPNAME_TEXTHEIGHT_CORRECTION: windows_core::PCWSTR = windows_core::w!("TextHeightCorrection");
pub const IPROPNAME_TEXTINTERNALLEADING: windows_core::PCWSTR = windows_core::w!("TextInternalLeading");
pub const IPROPNAME_TIME: windows_core::PCWSTR = windows_core::w!("Time");
pub const IPROPNAME_TRANSFORMS: windows_core::PCWSTR = windows_core::w!("TRANSFORMS");
pub const IPROPNAME_TRANSFORMSATSOURCE: windows_core::PCWSTR = windows_core::w!("TRANSFORMSATSOURCE");
pub const IPROPNAME_TRANSFORMSSECURE: windows_core::PCWSTR = windows_core::w!("TRANSFORMSSECURE");
pub const IPROPNAME_TRUEADMINUSER: windows_core::PCWSTR = windows_core::w!("MsiTrueAdminUser");
pub const IPROPNAME_TTCSUPPORT: windows_core::PCWSTR = windows_core::w!("TTCSupport");
pub const IPROPNAME_UACONLY: windows_core::PCWSTR = windows_core::w!("MsiUIUACOnly");
pub const IPROPNAME_UPDATESTARTED: windows_core::PCWSTR = windows_core::w!("UpdateStarted");
pub const IPROPNAME_UPGRADECODE: windows_core::PCWSTR = windows_core::w!("UpgradeCode");
pub const IPROPNAME_USERLANGUAGEID: windows_core::PCWSTR = windows_core::w!("UserLanguageID");
pub const IPROPNAME_USERNAME: windows_core::PCWSTR = windows_core::w!("USERNAME");
pub const IPROPNAME_USERSID: windows_core::PCWSTR = windows_core::w!("UserSID");
pub const IPROPNAME_VERSION9X: windows_core::PCWSTR = windows_core::w!("Version9X");
pub const IPROPNAME_VERSIONNT: windows_core::PCWSTR = windows_core::w!("VersionNT");
pub const IPROPNAME_VERSIONNT64: windows_core::PCWSTR = windows_core::w!("VersionNT64");
pub const IPROPNAME_VIRTUALMEMORY: windows_core::PCWSTR = windows_core::w!("VirtualMemory");
pub const IPROPNAME_WIN32ASSEMBLYSUPPORT: windows_core::PCWSTR = windows_core::w!("MsiWin32AssemblySupport");
pub const IPROPNAME_WINDOWSBUILD: windows_core::PCWSTR = windows_core::w!("WindowsBuild");
pub const IPROPNAME_WINDOWS_FOLDER: windows_core::PCWSTR = windows_core::w!("WindowsFolder");
pub const IPROPNAME_WINDOWS_VOLUME: windows_core::PCWSTR = windows_core::w!("WindowsVolume");
pub const IPROPVALUE_EXECUTEMODE_NONE: windows_core::PCWSTR = windows_core::w!("NONE");
pub const IPROPVALUE_EXECUTEMODE_SCRIPT: windows_core::PCWSTR = windows_core::w!("SCRIPT");
pub const IPROPVALUE_FEATURE_ALL: windows_core::PCWSTR = windows_core::w!("ALL");
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLE: windows_core::PCWSTR = windows_core::w!("Disable");
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLESHUTDOWN: windows_core::PCWSTR = windows_core::w!("DisableShutdown");
pub const IPROPVALUE_RBCOST_FAIL: windows_core::PCWSTR = windows_core::w!("F");
pub const IPROPVALUE_RBCOST_PROMPT: windows_core::PCWSTR = windows_core::w!("P");
pub const IPROPVALUE_RBCOST_SILENT: windows_core::PCWSTR = windows_core::w!("D");
pub const IPROPVALUE__CARRYINGNDP_URTREINSTALL: windows_core::PCWSTR = windows_core::w!("URTREINSTALL");
pub const IPROPVALUE__CARRYINGNDP_URTUPGRADE: windows_core::PCWSTR = windows_core::w!("URTUPGRADE");
windows_core::imp::define_interface!(IValidate, IValidate_Vtbl, 0xe482e5c6_e31e_4143_a2e6_dbc3d8e4b8d3);
windows_core::imp::interface_hierarchy!(IValidate, windows_core::IUnknown);
impl IValidate {
    pub unsafe fn OpenDatabase<P0>(&self, szdatabase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenDatabase)(windows_core::Interface::as_raw(self), szdatabase.param().abi()).ok() }
    }
    pub unsafe fn OpenCUB<P0>(&self, szcubfile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenCUB)(windows_core::Interface::as_raw(self), szcubfile.param().abi()).ok() }
    }
    pub unsafe fn CloseDatabase(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseDatabase)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CloseCUB(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseCUB)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplay)(windows_core::Interface::as_raw(self), pdisplayfunction, pcontext as _).ok() }
    }
    pub unsafe fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), pstatusfunction, pcontext as _).ok() }
    }
    pub unsafe fn Validate<P0>(&self, wzices: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), wzices.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenDatabase: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OpenCUB: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CloseDatabase: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseCUB: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, LPDISPLAYVAL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, LPEVALCOMCALLBACK, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IValidate_Impl: windows_core::IUnknownImpl {
    fn OpenDatabase(&self, szdatabase: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OpenCUB(&self, szcubfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CloseDatabase(&self) -> windows_core::Result<()>;
    fn CloseCUB(&self) -> windows_core::Result<()>;
    fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Validate(&self, wzices: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IValidate_Vtbl {
    pub const fn new<Identity: IValidate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenDatabase<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdatabase: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::OpenDatabase(this, core::mem::transmute(&szdatabase)).into()
            }
        }
        unsafe extern "system" fn OpenCUB<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcubfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::OpenCUB(this, core::mem::transmute(&szcubfile)).into()
            }
        }
        unsafe extern "system" fn CloseDatabase<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::CloseDatabase(this).into()
            }
        }
        unsafe extern "system" fn CloseCUB<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::CloseCUB(this).into()
            }
        }
        unsafe extern "system" fn SetDisplay<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::SetDisplay(this, core::mem::transmute_copy(&pdisplayfunction), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::SetStatus(this, core::mem::transmute_copy(&pstatusfunction), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Validate<Identity: IValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzices: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidate_Impl::Validate(this, core::mem::transmute(&wzices)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, OFFSET>,
            OpenCUB: OpenCUB::<Identity, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, OFFSET>,
            CloseCUB: CloseCUB::<Identity, OFFSET>,
            SetDisplay: SetDisplay::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValidate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IValidate {}
pub const LIBID_MsmMergeTypeLib: windows_core::GUID = windows_core::GUID::from_u128(0x0adda82f_2c26_11d2_ad65_00a0c9af11a6);
pub const LOGALL: u32 = 15u32;
pub const LOGERR: u32 = 4u32;
pub const LOGINFO: u32 = 1u32;
pub const LOGNONE: u32 = 0u32;
pub const LOGPERFMESSAGES: u32 = 8u32;
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
pub const LOGWARN: u32 = 2u32;
pub type LPDISPLAYVAL = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, uitype: RESULTTYPES, szwval: windows_core::PCWSTR, szwdescription: windows_core::PCWSTR, szwlocation: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type LPEVALCOMCALLBACK = Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const MAX_FEATURE_CHARS: u32 = 38u32;
pub const MAX_GUID_CHARS: u32 = 38u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIADVERTISEOPTIONFLAGS(pub i32);
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = MSIADVERTISEOPTIONFLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIARCHITECTUREFLAGS(pub i32);
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(4i32);
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(8i32);
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(2i32);
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIASSEMBLYINFO(pub u32);
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(0u32);
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSICODE(pub i32);
pub const MSICODE_PATCH: MSICODE = MSICODE(1073741824i32);
pub const MSICODE_PRODUCT: MSICODE = MSICODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSICOLINFO(pub i32);
pub const MSICOLINFO_NAMES: MSICOLINFO = MSICOLINFO(0i32);
pub const MSICOLINFO_TYPES: MSICOLINFO = MSICOLINFO(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSICONDITION(pub i32);
pub const MSICONDITION_ERROR: MSICONDITION = MSICONDITION(3i32);
pub const MSICONDITION_FALSE: MSICONDITION = MSICONDITION(0i32);
pub const MSICONDITION_NONE: MSICONDITION = MSICONDITION(2i32);
pub const MSICONDITION_TRUE: MSICONDITION = MSICONDITION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSICOSTTREE(pub i32);
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = MSICOSTTREE(1i32);
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = MSICOSTTREE(2i32);
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = MSICOSTTREE(3i32);
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = MSICOSTTREE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIDBERROR(pub i32);
pub const MSIDBERROR_BADCABINET: MSIDBERROR = MSIDBERROR(26i32);
pub const MSIDBERROR_BADCASE: MSIDBERROR = MSIDBERROR(8i32);
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = MSIDBERROR(23i32);
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = MSIDBERROR(15i32);
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = MSIDBERROR(20i32);
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = MSIDBERROR(18i32);
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = MSIDBERROR(13i32);
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = MSIDBERROR(16i32);
pub const MSIDBERROR_BADGUID: MSIDBERROR = MSIDBERROR(9i32);
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = MSIDBERROR(11i32);
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = MSIDBERROR(24i32);
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = MSIDBERROR(12i32);
pub const MSIDBERROR_BADLINK: MSIDBERROR = MSIDBERROR(3i32);
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = MSIDBERROR(29i32);
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = MSIDBERROR(25i32);
pub const MSIDBERROR_BADPATH: MSIDBERROR = MSIDBERROR(14i32);
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = MSIDBERROR(21i32);
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = MSIDBERROR(19i32);
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = MSIDBERROR(27i32);
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = MSIDBERROR(17i32);
pub const MSIDBERROR_BADVERSION: MSIDBERROR = MSIDBERROR(7i32);
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = MSIDBERROR(10i32);
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = MSIDBERROR(1i32);
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = MSIDBERROR(-1i32);
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = MSIDBERROR(-3i32);
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = MSIDBERROR(22i32);
pub const MSIDBERROR_MOREDATA: MSIDBERROR = MSIDBERROR(-2i32);
pub const MSIDBERROR_NOERROR: MSIDBERROR = MSIDBERROR(0i32);
pub const MSIDBERROR_NOTINSET: MSIDBERROR = MSIDBERROR(6i32);
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = MSIDBERROR(4i32);
pub const MSIDBERROR_REQUIRED: MSIDBERROR = MSIDBERROR(2i32);
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = MSIDBERROR(28i32);
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = MSIDBERROR(5i32);
pub const MSIDBOPEN_CREATE: windows_core::PCWSTR = windows_core::PCWSTR(3i32 as _);
pub const MSIDBOPEN_CREATEDIRECT: windows_core::PCWSTR = windows_core::PCWSTR(4i32 as _);
pub const MSIDBOPEN_DIRECT: windows_core::PCWSTR = windows_core::PCWSTR(2i32 as _);
pub const MSIDBOPEN_PATCHFILE: i32 = 16i32;
pub const MSIDBOPEN_READONLY: windows_core::PCWSTR = windows_core::PCWSTR(0i32 as _);
pub const MSIDBOPEN_TRANSACT: windows_core::PCWSTR = windows_core::PCWSTR(1i32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIDBSTATE(pub i32);
pub const MSIDBSTATE_ERROR: MSIDBSTATE = MSIDBSTATE(-1i32);
pub const MSIDBSTATE_READ: MSIDBSTATE = MSIDBSTATE(0i32);
pub const MSIDBSTATE_WRITE: MSIDBSTATE = MSIDBSTATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MSIHANDLE(pub u32);
impl MSIHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0
    }
}
impl windows_core::Free for MSIHANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("msi.dll" "system" fn MsiCloseHandle(hany : u32) -> u32);
            unsafe {
                MsiCloseHandle(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIINSTALLCONTEXT(pub i32);
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(7i32);
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(8i32);
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(4i32);
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(1i32);
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIMODIFY(pub i32);
pub const MSIMODIFY_ASSIGN: MSIMODIFY = MSIMODIFY(3i32);
pub const MSIMODIFY_DELETE: MSIMODIFY = MSIMODIFY(6i32);
pub const MSIMODIFY_INSERT: MSIMODIFY = MSIMODIFY(1i32);
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = MSIMODIFY(7i32);
pub const MSIMODIFY_MERGE: MSIMODIFY = MSIMODIFY(5i32);
pub const MSIMODIFY_REFRESH: MSIMODIFY = MSIMODIFY(0i32);
pub const MSIMODIFY_REPLACE: MSIMODIFY = MSIMODIFY(4i32);
pub const MSIMODIFY_SEEK: MSIMODIFY = MSIMODIFY(-1i32);
pub const MSIMODIFY_UPDATE: MSIMODIFY = MSIMODIFY(2i32);
pub const MSIMODIFY_VALIDATE: MSIMODIFY = MSIMODIFY(8i32);
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = MSIMODIFY(11i32);
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = MSIMODIFY(10i32);
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = MSIMODIFY(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIOPENPACKAGEFLAGS(pub i32);
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = MSIOPENPACKAGEFLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIPATCHDATATYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: windows_core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: windows_core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIPATCHSTATE(pub i32);
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = MSIPATCHSTATE(15i32);
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = MSIPATCHSTATE(1i32);
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = MSIPATCHSTATE(0i32);
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = MSIPATCHSTATE(4i32);
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = MSIPATCHSTATE(8i32);
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = MSIPATCHSTATE(2i32);
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = MSIPATCHDATATYPE(0i32);
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = MSIPATCHDATATYPE(2i32);
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = MSIPATCHDATATYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIRUNMODE(pub i32);
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = MSIRUNMODE(0i32);
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = MSIRUNMODE(1i32);
pub const MSIRUNMODE_CABINET: MSIRUNMODE = MSIRUNMODE(8i32);
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = MSIRUNMODE(18i32);
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = MSIRUNMODE(4i32);
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = MSIRUNMODE(2i32);
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = MSIRUNMODE(5i32);
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = MSIRUNMODE(6i32);
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = MSIRUNMODE(7i32);
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = MSIRUNMODE(11i32);
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = MSIRUNMODE(14i32);
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = MSIRUNMODE(15i32);
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = MSIRUNMODE(17i32);
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = MSIRUNMODE(3i32);
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = MSIRUNMODE(16i32);
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = MSIRUNMODE(9i32);
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = MSIRUNMODE(10i32);
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = MSIRUNMODE(12i32);
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = MSIRUNMODE(13i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSISOURCETYPE(pub i32);
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = MSISOURCETYPE(4i32);
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = MSISOURCETYPE(1i32);
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = MSISOURCETYPE(0i32);
pub const MSISOURCETYPE_URL: MSISOURCETYPE = MSISOURCETYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSITRANSACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSITRANSACTIONSTATE(pub u32);
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(1u32);
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(0u32);
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(1i32);
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSITRANSFORM_ERROR(pub i32);
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(1i32);
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(4i32);
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(32i32);
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(2i32);
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(8i32);
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(0i32);
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(16i32);
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(256i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSITRANSFORM_VALIDATE(pub i32);
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1i32);
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(8i32);
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(16i32);
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(256i32);
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1024i32);
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(512i32);
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(64i32);
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(128i32);
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(4i32);
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2i32);
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(32i32);
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2048i32);
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
pub const MsmMerge: windows_core::GUID = windows_core::GUID::from_u128(0x0adda830_2c26_11d2_ad65_00a0c9af11a6);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PACKMAN_RUNTIME(pub i32);
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = PACKMAN_RUNTIME(6i32);
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = PACKMAN_RUNTIME(5i32);
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(4i32);
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(1i32);
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(2i32);
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = PACKMAN_RUNTIME(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: windows_core::PCSTR,
    pub OldFileNameW: windows_core::PCWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
impl Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::super::Foundation::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: windows_core::PCSTR,
    pub OldFileSymbolPathArray: *const windows_core::PCSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
impl Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
pub const PID_APPNAME: u32 = 18u32;
pub const PID_AUTHOR: u32 = 4u32;
pub const PID_CHARCOUNT: u32 = 16u32;
pub const PID_COMMENTS: u32 = 6u32;
pub const PID_CREATE_DTM: u32 = 12u32;
pub const PID_EDITTIME: u32 = 10u32;
pub const PID_KEYWORDS: u32 = 5u32;
pub const PID_LASTAUTHOR: u32 = 8u32;
pub const PID_LASTPRINTED: u32 = 11u32;
pub const PID_LASTSAVE_DTM: u32 = 13u32;
pub const PID_MSIRESTRICT: u32 = 16u32;
pub const PID_MSISOURCE: u32 = 15u32;
pub const PID_MSIVERSION: u32 = 14u32;
pub const PID_PAGECOUNT: u32 = 14u32;
pub const PID_REVNUMBER: u32 = 9u32;
pub const PID_SUBJECT: u32 = 3u32;
pub const PID_TEMPLATE: u32 = 7u32;
pub const PID_THUMBNAIL: u32 = 17u32;
pub const PID_TITLE: u32 = 2u32;
pub const PID_WORDCOUNT: u32 = 15u32;
pub type PINSTALLUI_HANDLER_RECORD = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
pub const PMSvc: windows_core::GUID = windows_core::GUID::from_u128(0xb9e511fc_e364_497a_a121_b7b3612cedce);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ACTIVATION_POLICY(pub i32);
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(7i32);
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(4i32);
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(2i32);
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(3i32);
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(5i32);
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(0i32);
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(1i32);
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_APPLICATION_HUBTYPE(pub i32);
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(2i32);
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(1i32);
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(0i32);
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(3i32);
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(4i32);
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(5i32);
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(1i32);
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(0i32);
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_APPLICATION_INSTALL_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_APPLICATION_STATE(pub i32);
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = PM_APPLICATION_STATE(9i32);
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = PM_APPLICATION_STATE(8i32);
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = PM_APPLICATION_STATE(7i32);
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = PM_APPLICATION_STATE(1i32);
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(2i32);
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = PM_APPLICATION_STATE(11i32);
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(5i32);
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = PM_APPLICATION_STATE(0i32);
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(6i32);
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(4i32);
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PM_APPTASKTYPE {
    pub ProductID: windows_core::GUID,
    pub TaskType: PM_TASK_TYPE,
}
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(0i32);
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(6i32);
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(7i32);
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(2i32);
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(4i32);
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(8i32);
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(3i32);
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(5i32);
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_APP_GENRE(pub i32);
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = PM_APP_GENRE(0i32);
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = PM_APP_GENRE(2i32);
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = PM_APP_GENRE(1i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PM_BSATASKID {
    pub ProductID: windows_core::GUID,
    pub TaskID: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PM_BWTASKID {
    pub ProductID: windows_core::GUID,
    pub TaskID: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_APP_FILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_BSA_FILTER(pub i32);
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(26i32);
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(30i32);
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(29i32);
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(28i32);
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(27i32);
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(31i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_BW_FILTER(pub i32);
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(31i32);
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(32i32);
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(33i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_EXTENSION_FILTER(pub i32);
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(25i32);
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(23i32);
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(24i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(21i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(20i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(19i32);
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(26i32);
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(18i32);
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(22i32);
#[repr(C)]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
impl Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PM_ENUM_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: windows_core::GUID,
    pub TileProductID: windows_core::GUID,
    pub AppTaskType: PM_APPTASKTYPE,
    pub Consumer: core::mem::ManuallyDrop<PM_EXTENSIONCONSUMER>,
    pub BSATask: core::mem::ManuallyDrop<PM_BSATASKID>,
    pub BSAProductID: windows_core::GUID,
    pub BWTask: core::mem::ManuallyDrop<PM_BWTASKID>,
    pub ProtocolName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub FileType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub ContentType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub AppSupportedFileExtPID: windows_core::GUID,
    pub ShareTargetFileType: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PM_ENUM_FILTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_TASK_FILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_ENUM_TILE_FILTER(pub i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: windows_core::GUID,
    pub ExtensionID: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct PM_INSTALLINFO {
    pub ProductID: windows_core::GUID,
    pub PackagePath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub InstanceID: windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: windows_core::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: windows_core::GUID,
    pub MarketplaceAppVersion: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Default for PM_INSTALLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub URIFragmentOrArgs: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_LIVETILE_RECURRENCE_TYPE(pub i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(0i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_LOGO_SIZE(pub i32);
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = PM_LOGO_SIZE(3i32);
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = PM_LOGO_SIZE(2i32);
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = PM_LOGO_SIZE(1i32);
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = PM_LOGO_SIZE(0i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: windows_core::GUID,
    pub AppTitle: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub IconPath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub IsUninstallable: windows_core::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: windows_core::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: windows_core::BOOL,
    pub IsModernLightUp: windows_core::BOOL,
    pub LightUpSupportMask: u16,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: windows_core::GUID,
    pub TileID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: windows_core::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: windows_core::BOOL,
    pub IsModern: windows_core::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
impl Default for PM_STARTTILEBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_STARTTILE_TYPE(pub i32);
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(3i32);
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(4i32);
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(5i32);
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(1i32);
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(2i32);
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(12i32);
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(15i32);
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(16i32);
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(14i32);
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(17i32);
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(13i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_TASK_TRANSITION(pub i32);
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = PM_TASK_TRANSITION(6i32);
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = PM_TASK_TRANSITION(0i32);
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = PM_TASK_TRANSITION(7i32);
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(1i32);
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = PM_TASK_TRANSITION(5i32);
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(3i32);
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = PM_TASK_TRANSITION(4i32);
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_TASK_TYPE(pub i32);
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = PM_TASK_TYPE(3i32);
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = PM_TASK_TYPE(4i32);
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = PM_TASK_TYPE(1i32);
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = PM_TASK_TYPE(5i32);
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = PM_TASK_TYPE(0i32);
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = PM_TASK_TYPE(2i32);
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(8i32);
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(11i32);
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(10i32);
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(12i32);
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_TILE_HUBTYPE(pub i32);
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1073741824i32);
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108864i32);
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(536870912i32);
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108865i32);
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(33554432i32);
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(16777216i32);
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(268435456i32);
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1i32);
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(-2147483648i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PM_TILE_SIZE(pub i32);
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = PM_TILE_SIZE(5i32);
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = PM_TILE_SIZE(2i32);
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = PM_TILE_SIZE(1i32);
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = PM_TILE_SIZE(0i32);
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = PM_TILE_SIZE(3i32);
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = PM_TILE_SIZE(4i32);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct PM_UPDATEINFO {
    pub ProductID: windows_core::GUID,
    pub PackagePath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub InstanceID: windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub DeploymentOptions: u32,
}
impl Default for PM_UPDATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: windows_core::GUID,
    pub PackagePath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub InstanceID: windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: core::mem::ManuallyDrop<windows_core::BSTR>,
}
impl Default for PM_UPDATEINFO_LEGACY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PPATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void, currentposition: u32, maximumposition: u32) -> windows_core::BOOL>;
pub type PPATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: windows_core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl Default for PROTECTED_FILE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERYASMINFO_FLAGS(pub u32);
impl QUERYASMINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for QUERYASMINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for QUERYASMINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = QUERYASMINFO_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REINSTALLMODE(pub i32);
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = REINSTALLMODE(8i32);
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = REINSTALLMODE(16i32);
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = REINSTALLMODE(2i32);
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = REINSTALLMODE(4i32);
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = REINSTALLMODE(64i32);
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = REINSTALLMODE(32i32);
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = REINSTALLMODE(128i32);
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = REINSTALLMODE(1024i32);
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = REINSTALLMODE(1i32);
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = REINSTALLMODE(512i32);
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = REINSTALLMODE(256i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESULTTYPES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTFLAGS(pub i32);
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = SCRIPTFLAGS(1i32);
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = SCRIPTFLAGS(8i32);
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = SCRIPTFLAGS(416i32);
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = SCRIPTFLAGS(384i32);
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = SCRIPTFLAGS(128i32);
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = SCRIPTFLAGS(32i32);
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = SCRIPTFLAGS(256i32);
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = SCRIPTFLAGS(4i32);
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = SCRIPTFLAGS(64i32);
pub const SFC_DISABLE_ASK: u32 = 1u32;
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
pub const SFC_DISABLE_ONCE: u32 = 2u32;
pub const SFC_DISABLE_SETUP: u32 = 3u32;
pub const SFC_IDLE_TRIGGER: windows_core::PCWSTR = windows_core::w!("WFP_IDLE_TRIGGER");
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
pub const SFC_SCAN_NORMAL: u32 = 0u32;
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STATUSTYPES(pub i32);
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(2i32);
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(100i32);
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(16i32);
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(17i32);
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(33i32);
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(34i32);
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(4i32);
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(11i32);
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(14i32);
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(13i32);
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(15i32);
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(5i32);
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(59i32);
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(3i32);
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(12i32);
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(29i32);
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(31i32);
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(32i32);
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(30i32);
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(0i32);
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(1i32);
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(56i32);
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(7i32);
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(39i32);
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(40i32);
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(41i32);
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(42i32);
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(43i32);
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(44i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(35i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(36i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(37i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(38i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(45i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(46i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(47i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(48i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(49i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(50i32);
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(10i32);
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(57i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(51i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(52i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(53i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(54i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(55i32);
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(18i32);
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(19i32);
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(20i32);
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(21i32);
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(22i32);
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(23i32);
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(24i32);
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(25i32);
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(26i32);
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(27i32);
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(28i32);
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(58i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TILE_TEMPLATE_TYPE(pub i32);
pub const TXTLOG_BACKUP: u32 = 128u32;
pub const TXTLOG_CMI: u32 = 268435456u32;
pub const TXTLOG_COPYFILES: u32 = 8u32;
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
pub const TXTLOG_DETAILS: u32 = 5u32;
pub const TXTLOG_DEVINST: u32 = 1u32;
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
pub const TXTLOG_ERROR: u32 = 1u32;
pub const TXTLOG_FILEQ: u32 = 4u32;
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
pub const TXTLOG_INF: u32 = 2u32;
pub const TXTLOG_INFDB: u32 = 1024u32;
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
pub const TXTLOG_POLICY: u32 = 8388608u32;
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
pub const TXTLOG_SETUP: u32 = 134217728u32;
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
pub const TXTLOG_SIGVERIF: u32 = 32u32;
pub const TXTLOG_SUMMARY: u32 = 4u32;
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
pub const TXTLOG_TAB_1: u32 = 524288u32;
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
pub const TXTLOG_UI: u32 = 256u32;
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
pub const TXTLOG_UTIL: u32 = 512u32;
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
pub const TXTLOG_VERBOSE: u32 = 6u32;
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
pub const TXTLOG_WARNING: u32 = 2u32;
pub const UIALL: u32 = 32768u32;
pub const UILOGBITS: u32 = 15u32;
pub const UINONE: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USERINFOSTATE(pub i32);
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = USERINFOSTATE(0i32);
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = USERINFOSTATE(-2i32);
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = USERINFOSTATE(-3i32);
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = USERINFOSTATE(1i32);
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = USERINFOSTATE(-1i32);
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
pub const WARN_BASE: u32 = 3222294785u32;
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
pub const _WIN32_MSI: u32 = 500u32;
pub const _WIN32_MSM: u32 = 100u32;
pub const cchMaxInteger: i32 = 12i32;
pub const ieError: RESULTTYPES = RESULTTYPES(1i32);
pub const ieInfo: RESULTTYPES = RESULTTYPES(3i32);
pub const ieStatusCancel: STATUSTYPES = STATUSTYPES(10i32);
pub const ieStatusCreateEngine: STATUSTYPES = STATUSTYPES(4i32);
pub const ieStatusFail: STATUSTYPES = STATUSTYPES(9i32);
pub const ieStatusGetCUB: STATUSTYPES = STATUSTYPES(0i32);
pub const ieStatusICECount: STATUSTYPES = STATUSTYPES(1i32);
pub const ieStatusMerge: STATUSTYPES = STATUSTYPES(2i32);
pub const ieStatusRunICE: STATUSTYPES = STATUSTYPES(6i32);
pub const ieStatusShutdown: STATUSTYPES = STATUSTYPES(7i32);
pub const ieStatusStarting: STATUSTYPES = STATUSTYPES(5i32);
pub const ieStatusSuccess: STATUSTYPES = STATUSTYPES(8i32);
pub const ieStatusSummaryInfo: STATUSTYPES = STATUSTYPES(3i32);
pub const ieUnknown: RESULTTYPES = RESULTTYPES(0i32);
pub const ieWarning: RESULTTYPES = RESULTTYPES(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbAssemblyAttributes(pub i32);
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = msidbAssemblyAttributes(0i32);
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = msidbAssemblyAttributes(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbClassAttributes(pub i32);
pub const msidbClassAttributesRelativePath: msidbClassAttributes = msidbClassAttributes(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbComponentAttributes(pub i32);
pub const msidbComponentAttributes64bit: msidbComponentAttributes = msidbComponentAttributes(256i32);
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = msidbComponentAttributes(512i32);
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = msidbComponentAttributes(0i32);
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = msidbComponentAttributes(128i32);
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = msidbComponentAttributes(32i32);
pub const msidbComponentAttributesOptional: msidbComponentAttributes = msidbComponentAttributes(2i32);
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = msidbComponentAttributes(16i32);
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = msidbComponentAttributes(4i32);
pub const msidbComponentAttributesShared: msidbComponentAttributes = msidbComponentAttributes(2048i32);
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = msidbComponentAttributes(8i32);
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = msidbComponentAttributes(1i32);
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = msidbComponentAttributes(64i32);
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = msidbComponentAttributes(1024i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbControlAttributes(pub i32);
pub const msidbControlAttributesBiDi: msidbControlAttributes = msidbControlAttributes(224i32);
pub const msidbControlAttributesBitmap: msidbControlAttributes = msidbControlAttributes(262144i32);
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = msidbControlAttributes(524288i32);
pub const msidbControlAttributesComboList: msidbControlAttributes = msidbControlAttributes(131072i32);
pub const msidbControlAttributesElevationShield: msidbControlAttributes = msidbControlAttributes(8388608i32);
pub const msidbControlAttributesEnabled: msidbControlAttributes = msidbControlAttributes(2i32);
pub const msidbControlAttributesFixedSize: msidbControlAttributes = msidbControlAttributes(1048576i32);
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = msidbControlAttributes(131072i32);
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = msidbControlAttributes(2097152i32);
pub const msidbControlAttributesFormatSize: msidbControlAttributes = msidbControlAttributes(524288i32);
pub const msidbControlAttributesHasBorder: msidbControlAttributes = msidbControlAttributes(16777216i32);
pub const msidbControlAttributesIcon: msidbControlAttributes = msidbControlAttributes(524288i32);
pub const msidbControlAttributesIconSize16: msidbControlAttributes = msidbControlAttributes(2097152i32);
pub const msidbControlAttributesIconSize32: msidbControlAttributes = msidbControlAttributes(4194304i32);
pub const msidbControlAttributesIconSize48: msidbControlAttributes = msidbControlAttributes(6291456i32);
pub const msidbControlAttributesImageHandle: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesIndirect: msidbControlAttributes = msidbControlAttributes(8i32);
pub const msidbControlAttributesInteger: msidbControlAttributes = msidbControlAttributes(16i32);
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = msidbControlAttributes(128i32);
pub const msidbControlAttributesMultiline: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = msidbControlAttributes(131072i32);
pub const msidbControlAttributesNoWrap: msidbControlAttributes = msidbControlAttributes(262144i32);
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = msidbControlAttributes(2097152i32);
pub const msidbControlAttributesProgress95: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesPushLike: msidbControlAttributes = msidbControlAttributes(131072i32);
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = msidbControlAttributes(1048576i32);
pub const msidbControlAttributesRTLRO: msidbControlAttributes = msidbControlAttributes(32i32);
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = msidbControlAttributes(262144i32);
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesRightAligned: msidbControlAttributes = msidbControlAttributes(64i32);
pub const msidbControlAttributesSorted: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesSunken: msidbControlAttributes = msidbControlAttributes(4i32);
pub const msidbControlAttributesTransparent: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = msidbControlAttributes(1048576i32);
pub const msidbControlAttributesVisible: msidbControlAttributes = msidbControlAttributes(1i32);
pub const msidbControlShowRollbackCost: msidbControlAttributes = msidbControlAttributes(4194304i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbCustomActionType(pub i32);
pub const msidbCustomActionType64BitScript: msidbCustomActionType = msidbCustomActionType(4096i32);
pub const msidbCustomActionTypeAsync: msidbCustomActionType = msidbCustomActionType(128i32);
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = msidbCustomActionType(0i32);
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = msidbCustomActionType(768i32);
pub const msidbCustomActionTypeCommit: msidbCustomActionType = msidbCustomActionType(512i32);
pub const msidbCustomActionTypeContinue: msidbCustomActionType = msidbCustomActionType(64i32);
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = msidbCustomActionType(32i32);
pub const msidbCustomActionTypeDll: msidbCustomActionType = msidbCustomActionType(1i32);
pub const msidbCustomActionTypeExe: msidbCustomActionType = msidbCustomActionType(2i32);
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = msidbCustomActionType(256i32);
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = msidbCustomActionType(8192i32);
pub const msidbCustomActionTypeInScript: msidbCustomActionType = msidbCustomActionType(1024i32);
pub const msidbCustomActionTypeInstall: msidbCustomActionType = msidbCustomActionType(7i32);
pub const msidbCustomActionTypeJScript: msidbCustomActionType = msidbCustomActionType(5i32);
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = msidbCustomActionType(2048i32);
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = msidbCustomActionType(512i32);
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = msidbCustomActionType(32768i32);
pub const msidbCustomActionTypeProperty: msidbCustomActionType = msidbCustomActionType(48i32);
pub const msidbCustomActionTypeRollback: msidbCustomActionType = msidbCustomActionType(256i32);
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = msidbCustomActionType(16i32);
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = msidbCustomActionType(16384i32);
pub const msidbCustomActionTypeTextData: msidbCustomActionType = msidbCustomActionType(3i32);
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = msidbCustomActionType(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbDialogAttributes(pub i32);
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = msidbDialogAttributes(896i32);
pub const msidbDialogAttributesError: msidbDialogAttributes = msidbDialogAttributes(65536i32);
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = msidbDialogAttributes(16i32);
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = msidbDialogAttributes(512i32);
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = msidbDialogAttributes(4i32);
pub const msidbDialogAttributesModal: msidbDialogAttributes = msidbDialogAttributes(2i32);
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = msidbDialogAttributes(128i32);
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = msidbDialogAttributes(256i32);
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = msidbDialogAttributes(8i32);
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = msidbDialogAttributes(32i32);
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = msidbDialogAttributes(64i32);
pub const msidbDialogAttributesVisible: msidbDialogAttributes = msidbDialogAttributes(1i32);
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(2i32);
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbEmbeddedUIAttributes(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbFeatureAttributes(pub i32);
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(8i32);
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(4i32);
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = msidbFeatureAttributes(0i32);
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = msidbFeatureAttributes(1i32);
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = msidbFeatureAttributes(2i32);
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(32i32);
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = msidbFeatureAttributes(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbFileAttributes(pub i32);
pub const msidbFileAttributesChecksum: msidbFileAttributes = msidbFileAttributes(1024i32);
pub const msidbFileAttributesCompressed: msidbFileAttributes = msidbFileAttributes(16384i32);
pub const msidbFileAttributesHidden: msidbFileAttributes = msidbFileAttributes(2i32);
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = msidbFileAttributes(16i32);
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = msidbFileAttributes(8192i32);
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = msidbFileAttributes(4096i32);
pub const msidbFileAttributesReadOnly: msidbFileAttributes = msidbFileAttributes(1i32);
pub const msidbFileAttributesReserved0: msidbFileAttributes = msidbFileAttributes(8i32);
pub const msidbFileAttributesReserved1: msidbFileAttributes = msidbFileAttributes(64i32);
pub const msidbFileAttributesReserved2: msidbFileAttributes = msidbFileAttributes(128i32);
pub const msidbFileAttributesReserved3: msidbFileAttributes = msidbFileAttributes(256i32);
pub const msidbFileAttributesReserved4: msidbFileAttributes = msidbFileAttributes(32768i32);
pub const msidbFileAttributesSystem: msidbFileAttributes = msidbFileAttributes(4i32);
pub const msidbFileAttributesVital: msidbFileAttributes = msidbFileAttributes(512i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbIniFileAction(pub i32);
pub const msidbIniFileActionAddLine: msidbIniFileAction = msidbIniFileAction(0i32);
pub const msidbIniFileActionAddTag: msidbIniFileAction = msidbIniFileAction(3i32);
pub const msidbIniFileActionCreateLine: msidbIniFileAction = msidbIniFileAction(1i32);
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = msidbIniFileAction(2i32);
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = msidbIniFileAction(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbLocatorType(pub i32);
pub const msidbLocatorType64bit: msidbLocatorType = msidbLocatorType(16i32);
pub const msidbLocatorTypeDirectory: msidbLocatorType = msidbLocatorType(0i32);
pub const msidbLocatorTypeFileName: msidbLocatorType = msidbLocatorType(1i32);
pub const msidbLocatorTypeRawValue: msidbLocatorType = msidbLocatorType(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbMoveFileOptions(pub i32);
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = msidbMoveFileOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbODBCDataSourceRegistration(pub i32);
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(0i32);
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbPatchAttributes(pub i32);
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = msidbPatchAttributes(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbRegistryRoot(pub i32);
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = msidbRegistryRoot(0i32);
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = msidbRegistryRoot(1i32);
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = msidbRegistryRoot(2i32);
pub const msidbRegistryRootUsers: msidbRegistryRoot = msidbRegistryRoot(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbRemoveFileInstallMode(pub i32);
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(3i32);
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(1i32);
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbServiceConfigEvent(pub i32);
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = msidbServiceConfigEvent(1i32);
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = msidbServiceConfigEvent(4i32);
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = msidbServiceConfigEvent(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbServiceControlEvent(pub i32);
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = msidbServiceControlEvent(8i32);
pub const msidbServiceControlEventStart: msidbServiceControlEvent = msidbServiceControlEvent(1i32);
pub const msidbServiceControlEventStop: msidbServiceControlEvent = msidbServiceControlEvent(2i32);
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = msidbServiceControlEvent(128i32);
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = msidbServiceControlEvent(16i32);
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = msidbServiceControlEvent(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbServiceInstallErrorControl(pub i32);
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = msidbServiceInstallErrorControl(32768i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbSumInfoSourceType(pub i32);
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = msidbSumInfoSourceType(4i32);
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = msidbSumInfoSourceType(2i32);
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = msidbSumInfoSourceType(8i32);
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = msidbSumInfoSourceType(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbTextStyleStyleBits(pub i32);
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = msidbTextStyleStyleBits(1i32);
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = msidbTextStyleStyleBits(2i32);
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = msidbTextStyleStyleBits(8i32);
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = msidbTextStyleStyleBits(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msidbUpgradeAttributes(pub i32);
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = msidbUpgradeAttributes(4i32);
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(1024i32);
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = msidbUpgradeAttributes(1i32);
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = msidbUpgradeAttributes(2i32);
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(512i32);
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(256i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msifiFastInstallBits(pub i32);
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = msifiFastInstallBits(4i32);
pub const msifiFastInstallNoSR: msifiFastInstallBits = msifiFastInstallBits(1i32);
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = msifiFastInstallBits(2i32);
pub const msirbRebootCustomActionReason: msirbRebootReason = msirbRebootReason(4i32);
pub const msirbRebootDeferred: msirbRebootType = msirbRebootType(2i32);
pub const msirbRebootForceRebootReason: msirbRebootReason = msirbRebootReason(3i32);
pub const msirbRebootImmediate: msirbRebootType = msirbRebootType(1i32);
pub const msirbRebootInUseFilesReason: msirbRebootReason = msirbRebootReason(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msirbRebootReason(pub i32);
pub const msirbRebootScheduleRebootReason: msirbRebootReason = msirbRebootReason(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msirbRebootType(pub i32);
pub const msirbRebootUndeterminedReason: msirbRebootReason = msirbRebootReason(0i32);
pub const msmErrorDirCreate: msmErrorType = msmErrorType(7i32);
pub const msmErrorExclusion: msmErrorType = msmErrorType(3i32);
pub const msmErrorFeatureRequired: msmErrorType = msmErrorType(8i32);
pub const msmErrorFileCreate: msmErrorType = msmErrorType(6i32);
pub const msmErrorLanguageFailed: msmErrorType = msmErrorType(2i32);
pub const msmErrorLanguageUnsupported: msmErrorType = msmErrorType(1i32);
pub const msmErrorResequenceMerge: msmErrorType = msmErrorType(5i32);
pub const msmErrorTableMerge: msmErrorType = msmErrorType(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct msmErrorType(pub i32);
