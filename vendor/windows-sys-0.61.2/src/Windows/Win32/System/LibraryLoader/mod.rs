windows_link::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_sys::core::PCWSTR) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn BeginUpdateResourceA(pfilename : windows_sys::core::PCSTR, bdeleteexistingresources : windows_sys::core::BOOL) -> super::super::Foundation:: HANDLE);
windows_link::link!("kernel32.dll" "system" fn BeginUpdateResourceW(pfilename : windows_sys::core::PCWSTR, bdeleteexistingresources : windows_sys::core::BOOL) -> super::super::Foundation:: HANDLE);
windows_link::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::super::Foundation:: HMODULE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EndUpdateResourceA(hupdate : super::super::Foundation:: HANDLE, fdiscard : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EndUpdateResourceW(hupdate : super::super::Foundation:: HANDLE, fdiscard : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize, dwflags : u32, langid : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesW(hmodule : super::super::Foundation:: HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FindResourceA(hmodule : super::super::Foundation:: HMODULE, lpname : windows_sys::core::PCSTR, lptype : windows_sys::core::PCSTR) -> super::super::Foundation:: HRSRC);
windows_link::link!("kernel32.dll" "system" fn FindResourceExA(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
windows_link::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::super::Foundation:: HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, wlanguage : u16) -> super::super::Foundation:: HRSRC);
windows_link::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::super::Foundation:: HMODULE, lpname : windows_sys::core::PCWSTR, lptype : windows_sys::core::PCWSTR) -> super::super::Foundation:: HRSRC);
windows_link::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::super::Foundation:: HMODULE, dwexitcode : u32) -> !);
windows_link::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::super::Foundation:: HGLOBAL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDllDirectoryA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetDllDirectoryW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_sys::core::PCSTR) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : u32, lpmodulename : windows_sys::core::PCSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_sys::core::PCWSTR, phmodule : *mut super::super::Foundation:: HMODULE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_sys::core::PCWSTR) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::super::Foundation:: HMODULE, lpprocname : windows_sys::core::PCSTR) -> super::super::Foundation:: FARPROC);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_sys::core::PCSTR) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_sys::core::PCSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_sys::core::PCWSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_sys::core::PCWSTR) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn LoadModule(lpmodulename : windows_sys::core::PCSTR, lpparameterblock : *const core::ffi::c_void) -> u32);
windows_link::link!("kernel32.dll" "system" fn LoadPackagedLibrary(lpwlibfilename : windows_sys::core::PCWSTR, reserved : u32) -> super::super::Foundation:: HMODULE);
windows_link::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> super::super::Foundation:: HGLOBAL);
windows_link::link!("kernel32.dll" "system" fn LockResource(hresdata : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
windows_link::link!("api-ms-win-core-libraryloader-l2-1-0.dll" "system" fn QueryOptionalDelayLoadedAPI(hparentmodule : super::super::Foundation:: HMODULE, lpdllname : windows_sys::core::PCSTR, lpprocname : windows_sys::core::PCSTR, reserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : LOAD_LIBRARY_FLAGS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDllDirectoryA(lppathname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDllDirectoryW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::super::Foundation:: HMODULE, hresinfo : super::super::Foundation:: HRSRC) -> u32);
windows_link::link!("kernel32.dll" "system" fn UpdateResourceA(hupdate : super::super::Foundation:: HANDLE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn UpdateResourceW(hupdate : super::super::Foundation:: HANDLE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = 1u32;
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, wlanguage: u16, lparam: isize) -> windows_sys::core::BOOL>;
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, wlanguage: u16, lparam: isize) -> windows_sys::core::BOOL>;
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, lparam: isize) -> windows_sys::core::BOOL>;
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, lparam: isize) -> windows_sys::core::BOOL>;
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCSTR, lparam: isize) -> windows_sys::core::BOOL>;
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_sys::core::PCWSTR, lparam: isize) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = 16u32;
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = 2u32;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = 64u32;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32u32;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = 128u32;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = 8192u32;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = 512u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = 256u32;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = 2048u32;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = 16384u32;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = 1024u32;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = 8u32;
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_sys::core::PCSTR, phmodule: *mut super::super::Foundation::HMODULE) -> windows_sys::core::BOOL>;
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_sys::core::PCWSTR, phmodule: *mut super::super::Foundation::HMODULE) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_sys::core::PCSTR,
    pub FunctionName: windows_sys::core::PCSTR,
    pub RedirectionTarget: *mut core::ffi::c_void,
}
impl Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
