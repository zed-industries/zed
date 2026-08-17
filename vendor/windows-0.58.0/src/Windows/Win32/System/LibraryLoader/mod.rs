#[inline]
pub unsafe fn AddDllDirectory<P0>(newdirectory: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_core::PCWSTR) -> *mut core::ffi::c_void);
    AddDllDirectory(newdirectory.param().abi())
}
#[inline]
pub unsafe fn BeginUpdateResourceA<P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn BeginUpdateResourceA(pfilename : windows_core::PCSTR, bdeleteexistingresources : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
    let result__ = BeginUpdateResourceA(pfilename.param().abi(), bdeleteexistingresources.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn BeginUpdateResourceW<P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn BeginUpdateResourceW(pfilename : windows_core::PCWSTR, bdeleteexistingresources : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
    let result__ = BeginUpdateResourceW(pfilename.param().abi(), bdeleteexistingresources.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DisableThreadLibraryCalls<P0>(hlibmodule: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::super::Foundation:: HMODULE) -> super::super::Foundation:: BOOL);
    DisableThreadLibraryCalls(hlibmodule.param().abi()).ok()
}
#[inline]
pub unsafe fn EndUpdateResourceA<P0, P1>(hupdate: P0, fdiscard: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn EndUpdateResourceA(hupdate : super::super::Foundation:: HANDLE, fdiscard : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    EndUpdateResourceA(hupdate.param().abi(), fdiscard.param().abi()).ok()
}
#[inline]
pub unsafe fn EndUpdateResourceW<P0, P1>(hupdate: P0, fdiscard: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn EndUpdateResourceW(hupdate : super::super::Foundation:: HANDLE, fdiscard : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    EndUpdateResourceW(hupdate.param().abi(), fdiscard.param().abi()).ok()
}
#[inline]
pub unsafe fn EnumResourceLanguagesA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceLanguagesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceLanguagesA(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam).ok()
}
#[inline]
pub unsafe fn EnumResourceLanguagesExA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceLanguagesExA(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceLanguagesExW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceLanguagesExW(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceLanguagesW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceLanguagesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceLanguagesW(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam).ok()
}
#[inline]
pub unsafe fn EnumResourceNamesA<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceNamesA(hmodule.param().abi(), lptype.param().abi(), lpenumfunc, lparam).ok()
}
#[inline]
pub unsafe fn EnumResourceNamesExA<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceNamesExA(hmodule.param().abi(), lptype.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceNamesExW<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceNamesExW(hmodule.param().abi(), lptype.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceNamesW<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceNamesW(hmodule.param().abi(), lptype.param().abi(), lpenumfunc, lparam)
}
#[inline]
pub unsafe fn EnumResourceTypesA<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceTypesA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceTypesA(hmodule.param().abi(), lpenumfunc, lparam).ok()
}
#[inline]
pub unsafe fn EnumResourceTypesExA<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceTypesExA(hmodule.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceTypesExW<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize, dwflags : u32, langid : u16) -> super::super::Foundation:: BOOL);
    EnumResourceTypesExW(hmodule.param().abi(), lpenumfunc, lparam, dwflags, langid).ok()
}
#[inline]
pub unsafe fn EnumResourceTypesW<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnumResourceTypesW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize) -> super::super::Foundation:: BOOL);
    EnumResourceTypesW(hmodule.param().abi(), lpenumfunc, lparam).ok()
}
#[inline]
pub unsafe fn FindResourceA<P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> windows_core::Result<super::super::Foundation::HRSRC>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindResourceA(hmodule : super::super::Foundation:: HMODULE, lpname : windows_core::PCSTR, lptype : windows_core::PCSTR) -> super::super::Foundation:: HRSRC);
    let result__ = FindResourceA(hmodule.param().abi(), lpname.param().abi(), lptype.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindResourceExA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> windows_core::Result<super::super::Foundation::HRSRC>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindResourceExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
    let result__ = FindResourceExA(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), wlanguage);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn FindResourceExW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> super::super::Foundation::HRSRC
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
    FindResourceExW(hmodule.param().abi(), lptype.param().abi(), lpname.param().abi(), wlanguage)
}
#[inline]
pub unsafe fn FindResourceW<P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> super::super::Foundation::HRSRC
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::super::Foundation:: HMODULE, lpname : windows_core::PCWSTR, lptype : windows_core::PCWSTR) -> super::super::Foundation:: HRSRC);
    FindResourceW(hmodule.param().abi(), lpname.param().abi(), lptype.param().abi())
}
#[inline]
pub unsafe fn FreeLibraryAndExitThread<P0>(hlibmodule: P0, dwexitcode: u32) -> !
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::super::Foundation:: HMODULE, dwexitcode : u32) -> !);
    FreeLibraryAndExitThread(hlibmodule.param().abi(), dwexitcode)
}
#[inline]
pub unsafe fn FreeResource<P0>(hresdata: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HGLOBAL>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::super::Foundation:: HGLOBAL) -> super::super::Foundation:: BOOL);
    FreeResource(hresdata.param().abi())
}
#[inline]
pub unsafe fn GetDllDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetDllDirectoryA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    GetDllDirectoryA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetDllDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetDllDirectoryW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    GetDllDirectoryW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn GetModuleFileNameA<P0>(hmodule: P0, lpfilename: &mut [u8]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetModuleFileNameA(hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleFileNameW<P0>(hmodule: P0, lpfilename: &mut [u16]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetModuleFileNameW(hmodule.param().abi(), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleHandleA<P0>(lpmodulename: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_core::PCSTR) -> super::super::Foundation:: HMODULE);
    let result__ = GetModuleHandleA(lpmodulename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetModuleHandleExA<P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : u32, lpmodulename : windows_core::PCSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> super::super::Foundation:: BOOL);
    GetModuleHandleExA(dwflags, lpmodulename.param().abi(), phmodule).ok()
}
#[inline]
pub unsafe fn GetModuleHandleExW<P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_core::PCWSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> super::super::Foundation:: BOOL);
    GetModuleHandleExW(dwflags, lpmodulename.param().abi(), phmodule).ok()
}
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> super::super::Foundation:: HMODULE);
    let result__ = GetModuleHandleW(lpmodulename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetProcAddress<P0, P1>(hmodule: P0, lpprocname: P1) -> super::super::Foundation::FARPROC
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::super::Foundation:: HMODULE, lpprocname : windows_core::PCSTR) -> super::super::Foundation:: FARPROC);
    GetProcAddress(hmodule.param().abi(), lpprocname.param().abi())
}
#[inline]
pub unsafe fn LoadLibraryA<P0>(lplibfilename: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_core::PCSTR) -> super::super::Foundation:: HMODULE);
    let result__ = LoadLibraryA(lplibfilename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadLibraryExA<P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_core::PCSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
    let result__ = LoadLibraryExA(lplibfilename.param().abi(), hfile.param().abi(), dwflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadLibraryExW<P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_core::PCWSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
    let result__ = LoadLibraryExW(lplibfilename.param().abi(), hfile.param().abi(), dwflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> super::super::Foundation:: HMODULE);
    let result__ = LoadLibraryW(lplibfilename.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadModule<P0>(lpmodulename: P0, lpparameterblock: *const core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadModule(lpmodulename : windows_core::PCSTR, lpparameterblock : *const core::ffi::c_void) -> u32);
    LoadModule(lpmodulename.param().abi(), lpparameterblock)
}
#[inline]
pub unsafe fn LoadPackagedLibrary<P0>(lpwlibfilename: P0, reserved: u32) -> windows_core::Result<super::super::Foundation::HMODULE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadPackagedLibrary(lpwlibfilename : windows_core::PCWSTR, reserved : u32) -> super::super::Foundation:: HMODULE);
    let result__ = LoadPackagedLibrary(lpwlibfilename.param().abi(), reserved);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LoadResource<P0, P1>(hmodule: P0, hresinfo: P1) -> windows_core::Result<super::super::Foundation::HGLOBAL>
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<super::super::Foundation::HRSRC>,
{
    windows_targets::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> super::super::Foundation:: HGLOBAL);
    let result__ = LoadResource(hmodule.param().abi(), hresinfo.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LockResource<P0>(hresdata: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<super::super::Foundation::HGLOBAL>,
{
    windows_targets::link!("kernel32.dll" "system" fn LockResource(hresdata : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
    LockResource(hresdata.param().abi())
}
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    RemoveDllDirectory(cookie).ok()
}
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: BOOL);
    SetDefaultDllDirectories(directoryflags).ok()
}
#[inline]
pub unsafe fn SetDllDirectoryA<P0>(lppathname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetDllDirectoryA(lppathname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetDllDirectoryA(lppathname.param().abi()).ok()
}
#[inline]
pub unsafe fn SetDllDirectoryW<P0>(lppathname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetDllDirectoryW(lppathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetDllDirectoryW(lppathname.param().abi()).ok()
}
#[inline]
pub unsafe fn SizeofResource<P0, P1>(hmodule: P0, hresinfo: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HMODULE>,
    P1: windows_core::Param<super::super::Foundation::HRSRC>,
{
    windows_targets::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> u32);
    SizeofResource(hmodule.param().abi(), hresinfo.param().abi())
}
#[inline]
pub unsafe fn UpdateResourceA<P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: Option<*const core::ffi::c_void>, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn UpdateResourceA(hupdate : super::super::Foundation:: HANDLE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    UpdateResourceA(hupdate.param().abi(), lptype.param().abi(), lpname.param().abi(), wlanguage, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), cb).ok()
}
#[inline]
pub unsafe fn UpdateResourceW<P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: Option<*const core::ffi::c_void>, cb: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn UpdateResourceW(hupdate : super::super::Foundation:: HANDLE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    UpdateResourceW(hupdate.param().abi(), lptype.param().abi(), lpname.param().abi(), wlanguage, core::mem::transmute(lpdata.unwrap_or(std::ptr::null())), cb).ok()
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16384u32);
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
impl windows_core::TypeKind for LOAD_LIBRARY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOAD_LIBRARY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOAD_LIBRARY_FLAGS").field(&self.0).finish()
    }
}
impl LOAD_LIBRARY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl windows_core::TypeKind for ENUMUILANG {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl windows_core::TypeKind for REDIRECTION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_core::PCSTR,
    pub FunctionName: windows_core::PCSTR,
    pub RedirectionTarget: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for REDIRECTION_FUNCTION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCSTR, phmodule: *mut super::super::Foundation::HMODULE) -> super::super::Foundation::BOOL>;
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCWSTR, phmodule: *mut super::super::Foundation::HMODULE) -> super::super::Foundation::BOOL>;
