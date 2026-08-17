pub const DBG_ATTACH: u32 = 14u32;
pub const DBG_BREAK: u32 = 6u32;
pub const DBG_DIVOVERFLOW: u32 = 8u32;
pub const DBG_DLLSTART: u32 = 12u32;
pub const DBG_DLLSTOP: u32 = 13u32;
pub const DBG_GPFAULT: u32 = 7u32;
pub const DBG_GPFAULT2: u32 = 21u32;
pub const DBG_INIT: u32 = 20u32;
pub const DBG_INSTRFAULT: u32 = 9u32;
pub const DBG_MODFREE: u32 = 4u32;
pub const DBG_MODLOAD: u32 = 3u32;
pub const DBG_MODMOVE: u32 = 19u32;
pub const DBG_SEGFREE: u32 = 2u32;
pub const DBG_SEGLOAD: u32 = 0u32;
pub const DBG_SEGMOVE: u32 = 1u32;
pub const DBG_SINGLESTEP: u32 = 5u32;
pub const DBG_STACKFAULT: u32 = 16u32;
pub const DBG_TASKSTART: u32 = 10u32;
pub const DBG_TASKSTOP: u32 = 11u32;
pub const DBG_TEMPBP: u32 = 18u32;
pub const DBG_TOOLHELP: u32 = 15u32;
pub const DBG_WOWINIT: u32 = 17u32;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type DEBUGEVENTPROC = Option<unsafe extern "system" fn(param0: *mut super::Diagnostics::Debug::DEBUG_EVENT, param1: *mut core::ffi::c_void) -> u32>;
pub const GD_ACCELERATORS: u32 = 9u32;
pub const GD_BITMAP: u32 = 2u32;
pub const GD_CURSOR: u32 = 12u32;
pub const GD_CURSORCOMPONENT: u32 = 1u32;
pub const GD_DIALOG: u32 = 5u32;
pub const GD_ERRTABLE: u32 = 11u32;
pub const GD_FONT: u32 = 8u32;
pub const GD_FONTDIR: u32 = 7u32;
pub const GD_ICON: u32 = 14u32;
pub const GD_ICONCOMPONENT: u32 = 3u32;
pub const GD_MAX_RESOURCE: u32 = 15u32;
pub const GD_MENU: u32 = 4u32;
pub const GD_NAMETABLE: u32 = 15u32;
pub const GD_RCDATA: u32 = 10u32;
pub const GD_STRING: u32 = 6u32;
pub const GD_USERDEFINED: u32 = 0u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct GLOBALENTRY {
    pub dwSize: u32,
    pub dwAddress: u32,
    pub dwBlockSize: u32,
    pub hBlock: super::super::Foundation::HANDLE,
    pub wcLock: u16,
    pub wcPageLock: u16,
    pub wFlags: u16,
    pub wHeapPresent: windows_sys::core::BOOL,
    pub hOwner: super::super::Foundation::HANDLE,
    pub wType: u16,
    pub wData: u16,
    pub dwNext: u32,
    pub dwNextAlt: u32,
}
impl Default for GLOBALENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GLOBAL_ALL: u32 = 0u32;
pub const GLOBAL_FREE: u32 = 2u32;
pub const GLOBAL_LRU: u32 = 1u32;
pub const GT_BURGERMASTER: u32 = 10u32;
pub const GT_CODE: u32 = 3u32;
pub const GT_DATA: u32 = 2u32;
pub const GT_DGROUP: u32 = 1u32;
pub const GT_FREE: u32 = 7u32;
pub const GT_INTERNAL: u32 = 8u32;
pub const GT_MODULE: u32 = 6u32;
pub const GT_RESOURCE: u32 = 5u32;
pub const GT_SENTINEL: u32 = 9u32;
pub const GT_TASK: u32 = 4u32;
pub const GT_UNKNOWN: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_NOTE {
    pub Module: [i8; 10],
    pub FileName: [i8; 256],
    pub hModule: u16,
    pub hTask: u16,
}
impl Default for IMAGE_NOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_MODULE_NAME: u32 = 9u32;
pub const MAX_PATH16: u32 = 255u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MODULEENTRY {
    pub dwSize: u32,
    pub szModule: [i8; 10],
    pub hModule: super::super::Foundation::HANDLE,
    pub wcUsage: u16,
    pub szExePath: [i8; 256],
    pub wNext: u16,
}
impl Default for MODULEENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROCESSENUMPROC = Option<unsafe extern "system" fn(dwprocessid: u32, dwattributes: u32, lpuserdefined: super::super::Foundation::LPARAM) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEGMENT_NOTE {
    pub Selector1: u16,
    pub Selector2: u16,
    pub Segment: u16,
    pub Module: [i8; 10],
    pub FileName: [i8; 256],
    pub Type: u16,
    pub Length: u32,
}
impl Default for SEGMENT_NOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SN_CODE: u32 = 0u32;
pub const SN_DATA: u32 = 1u32;
pub const SN_V86: u32 = 2u32;
pub const STATUS_VDM_EVENT: i32 = 1073741829i32;
pub type TASKENUMPROC = Option<unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, lpuserdefined: super::super::Foundation::LPARAM) -> windows_sys::core::BOOL>;
pub type TASKENUMPROCEX = Option<unsafe extern "system" fn(dwthreadid: u32, hmod16: u16, htask16: u16, pszmodname: *mut i8, pszfilename: *mut i8, lpuserdefined: super::super::Foundation::LPARAM) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TEMP_BP_NOTE {
    pub Seg: u16,
    pub Offset: u32,
    pub bPM: windows_sys::core::BOOL,
}
pub const V86FLAGS_ALIGNMENT: u32 = 262144u32;
pub const V86FLAGS_AUXCARRY: u32 = 16u32;
pub const V86FLAGS_CARRY: u32 = 1u32;
pub const V86FLAGS_DIRECTION: u32 = 1024u32;
pub const V86FLAGS_INTERRUPT: u32 = 512u32;
pub const V86FLAGS_IOPL: u32 = 12288u32;
pub const V86FLAGS_IOPL_BITS: u32 = 18u32;
pub const V86FLAGS_OVERFLOW: u32 = 2048u32;
pub const V86FLAGS_PARITY: u32 = 4u32;
pub const V86FLAGS_RESUME: u32 = 65536u32;
pub const V86FLAGS_SIGN: u32 = 128u32;
pub const V86FLAGS_TRACE: u32 = 256u32;
pub const V86FLAGS_V86: u32 = 131072u32;
pub const V86FLAGS_ZERO: u32 = 64u32;
pub const VDMADDR_PM16: u32 = 4u32;
pub const VDMADDR_PM32: u32 = 16u32;
pub const VDMADDR_V86: u32 = 2u32;
pub type VDMBREAKTHREADPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct VDMCONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: super::Kernel::FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
impl Default for VDMCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Default)]
pub struct VDMCONTEXT_WITHOUT_XSAVE {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: super::Kernel::FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
}
pub const VDMCONTEXT_i386: u32 = 65536u32;
pub const VDMCONTEXT_i486: u32 = 65536u32;
pub const VDMDBG_BREAK_DEBUGGER: u32 = 16u32;
pub const VDMDBG_BREAK_DIVIDEBYZERO: u32 = 256u32;
pub const VDMDBG_BREAK_DOSTASK: u32 = 1u32;
pub const VDMDBG_BREAK_EXCEPTIONS: u32 = 8u32;
pub const VDMDBG_BREAK_LOADDLL: u32 = 4u32;
pub const VDMDBG_BREAK_WOWTASK: u32 = 2u32;
pub const VDMDBG_INITIAL_FLAGS: u32 = 256u32;
pub const VDMDBG_MAX_SYMBOL_BUFFER: u32 = 256u32;
pub const VDMDBG_TRACE_HISTORY: u32 = 128u32;
pub type VDMDETECTWOWPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
pub type VDMENUMPROCESSWOWPROC = Option<unsafe extern "system" fn(param0: PROCESSENUMPROC, param1: super::super::Foundation::LPARAM) -> i32>;
pub type VDMENUMTASKWOWEXPROC = Option<unsafe extern "system" fn(param0: u32, param1: TASKENUMPROCEX, param2: super::super::Foundation::LPARAM) -> i32>;
pub type VDMENUMTASKWOWPROC = Option<unsafe extern "system" fn(param0: u32, param1: TASKENUMPROC, param2: super::super::Foundation::LPARAM) -> i32>;
pub const VDMEVENT_ALLFLAGS: u32 = 57344u32;
pub const VDMEVENT_NEEDS_INTERACTIVE: u32 = 32768u32;
pub const VDMEVENT_PE: u32 = 8192u32;
pub const VDMEVENT_PM16: u32 = 2u32;
pub const VDMEVENT_V86: u32 = 1u32;
pub const VDMEVENT_VERBOSE: u32 = 16384u32;
pub type VDMGETADDREXPRESSIONPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: *mut u16, param3: *mut u32, param4: *mut u16) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type VDMGETCONTEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
pub type VDMGETCONTEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> windows_sys::core::BOOL>;
pub type VDMGETDBGFLAGSPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
pub type VDMGETMODULESELECTORPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: windows_sys::core::PCSTR, param4: *mut u16) -> windows_sys::core::BOOL>;
pub type VDMGETPOINTERPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: u32, param4: windows_sys::core::BOOL) -> u32>;
pub type VDMGETSEGMENTINFOPROC = Option<unsafe extern "system" fn(param0: u16, param1: u32, param2: windows_sys::core::BOOL, param3: VDM_SEGINFO) -> windows_sys::core::BOOL>;
pub type VDMGETSELECTORMODULEPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u16, param3: *mut u32, param4: windows_sys::core::PCSTR, param5: u32, param6: windows_sys::core::PCSTR, param7: u32) -> windows_sys::core::BOOL>;
pub type VDMGETSYMBOLPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: u16, param2: u32, param3: windows_sys::core::BOOL, param4: windows_sys::core::BOOL, param5: windows_sys::core::PSTR, param6: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type VDMGETTHREADSELECTORENTRYPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut super::Diagnostics::Debug::LDT_ENTRY) -> windows_sys::core::BOOL>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type VDMGETTHREADSELECTORENTRYPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: u32, param3: *mut VDMLDT_ENTRY) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMGLOBALFIRSTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: DEBUGEVENTPROC, param5: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMGLOBALNEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut GLOBALENTRY, param3: u16, param4: DEBUGEVENTPROC, param5: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type VDMISMODULELOADEDPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type VDMKILLWOWPROC = Option<unsafe extern "system" fn() -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct VDMLDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: VDMLDT_ENTRY_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for VDMLDT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union VDMLDT_ENTRY_0 {
    pub Bytes: VDMLDT_ENTRY_0_0,
    pub Bits: VDMLDT_ENTRY_0_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for VDMLDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct VDMLDT_ENTRY_0_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct VDMLDT_ENTRY_0_0 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMMODULEFIRSTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: DEBUGEVENTPROC, param4: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMMODULENEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut MODULEENTRY, param3: DEBUGEVENTPROC, param4: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Threading"))]
pub type VDMPROCESSEXCEPTIONPROC = Option<unsafe extern "system" fn(param0: *mut super::Diagnostics::Debug::DEBUG_EVENT) -> windows_sys::core::BOOL>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type VDMSETCONTEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut super::Diagnostics::Debug::CONTEXT) -> windows_sys::core::BOOL>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
pub type VDMSETCONTEXTPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: super::super::Foundation::HANDLE, param2: *mut VDMCONTEXT) -> windows_sys::core::BOOL>;
pub type VDMSETDBGFLAGSPROC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32) -> windows_sys::core::BOOL>;
pub type VDMSTARTTASKINWOWPROC = Option<unsafe extern "system" fn(param0: u32, param1: windows_sys::core::PCSTR, param2: u16) -> windows_sys::core::BOOL>;
pub type VDMTERMINATETASKINWOWPROC = Option<unsafe extern "system" fn(param0: u32, param1: u16) -> windows_sys::core::BOOL>;
pub const VDM_KGDT_R3_CODE: u32 = 24u32;
pub const VDM_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDM_SEGINFO {
    pub Selector: u16,
    pub SegNumber: u16,
    pub Length: u32,
    pub Type: u16,
    pub ModuleName: [i8; 9],
    pub FileName: [i8; 255],
}
impl Default for VDM_SEGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WOW_SYSTEM: u32 = 1u32;
