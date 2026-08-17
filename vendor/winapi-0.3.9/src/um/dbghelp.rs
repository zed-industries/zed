// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! DbgHelp include file
use shared::basetsd::{DWORD64, PDWORD64, ULONG64};
use shared::guiddef::GUID;
use shared::minwindef::{
    BOOL, DWORD, HMODULE, LPDWORD, MAX_PATH, PDWORD, PUCHAR, PULONG, UCHAR, ULONG, USHORT, WORD,
};
use um::winnt::{
    BOOLEAN, CHAR, HANDLE, LIST_ENTRY, PCSTR, PCWSTR, PIMAGE_NT_HEADERS, PIMAGE_SECTION_HEADER,
    PSTR, PVOID, PWSTR, WCHAR,
};
#[cfg(target_pointer_width = "32")]
use um::winnt::{
    PFPO_DATA, PIMAGE_COFF_SYMBOLS_HEADER, PIMAGE_DEBUG_DIRECTORY, PIMAGE_FUNCTION_ENTRY,
    PIMAGE_NT_HEADERS32,
};
#[cfg(target_pointer_width = "64")]
use um::winnt::PIMAGE_NT_HEADERS64;
use vc::vcruntime::size_t;
#[cfg(target_pointer_width = "64")]
STRUCT!{struct LOADED_IMAGE {
    ModuleName: PSTR,
    hFile: HANDLE,
    MappedAddress: PUCHAR,
    FileHeader: PIMAGE_NT_HEADERS64,
    LastRvaSection: PIMAGE_SECTION_HEADER,
    NumberOfSections: ULONG,
    Sections: PIMAGE_SECTION_HEADER,
    Characteristics: ULONG,
    fSystemImage: BOOLEAN,
    fDOSImage: BOOLEAN,
    fReadOnly: BOOLEAN,
    Version: UCHAR,
    Links: LIST_ENTRY,
    SizeOfImage: ULONG,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct LOADED_IMAGE {
    ModuleName: PSTR,
    hFile: HANDLE,
    MappedAddress: PUCHAR,
    FileHeader: PIMAGE_NT_HEADERS32,
    LastRvaSection: PIMAGE_SECTION_HEADER,
    NumberOfSections: ULONG,
    Sections: PIMAGE_SECTION_HEADER,
    Characteristics: ULONG,
    fSystemImage: BOOLEAN,
    fDOSImage: BOOLEAN,
    fReadOnly: BOOLEAN,
    Version: UCHAR,
    Links: LIST_ENTRY,
    SizeOfImage: ULONG,
}}
pub const MAX_SYM_NAME: usize = 2000;
pub const ERROR_IMAGE_NOT_STRIPPED: DWORD = 0x8800;
pub const ERROR_NO_DBG_POINTER: DWORD = 0x8801;
pub const ERROR_NO_PDB_POINTER: DWORD = 0x8802;
FN!{stdcall PFIND_DEBUG_FILE_CALLBACK(
    FileHandle: HANDLE,
    FileName: PCSTR,
    CallerData: PVOID,
) -> BOOL}
FN!{stdcall PFIND_DEBUG_FILE_CALLBACKW(
    FileHandle: HANDLE,
    FileName: PCWSTR,
    CallerData: PVOID,
) -> BOOL}
FN!{stdcall PFINDFILEINPATHCALLBACK(
    filename: PCSTR,
    context: PVOID,
) -> BOOL}
FN!{stdcall PFINDFILEINPATHCALLBACKW(
    filename: PCWSTR,
    context: PVOID,
) -> BOOL}
FN!{stdcall PFIND_EXE_FILE_CALLBACK(
    FileHandle: HANDLE,
    FileName: PCSTR,
    CallerData: PVOID,
) -> BOOL}
FN!{stdcall PFIND_EXE_FILE_CALLBACKW(
    FileHandle: HANDLE,
    FileName: PCWSTR,
    CallerData: PVOID,
) -> BOOL}
FN!{stdcall PSYM_ENUMERATESYMBOLS_CALLBACKW(
    pSymInfo: PSYMBOL_INFOW,
    SymbolSize: ULONG,
    CallerData: PVOID,
) -> BOOL}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct IMAGE_DEBUG_INFORMATION {
    List: LIST_ENTRY,
    ReservedSize: DWORD,
    ReservedMappedBase: PVOID,
    ReservedMachine: USHORT,
    ReservedCharacteristics: USHORT,
    ReservedCheckSum: DWORD,
    ImageBase: DWORD,
    SizeOfImage: DWORD,
    ReservedNumberOfSections: DWORD,
    ReservedSections: PIMAGE_SECTION_HEADER,
    ReservedExportedNamesSize: DWORD,
    ReservedExportedNames: PSTR,
    ReservedNumberOfFunctionTableEntries: DWORD,
    ReservedFunctionTableEntries: PIMAGE_FUNCTION_ENTRY,
    ReservedLowestFunctionStartingAddress: DWORD,
    ReservedHighestFunctionEndingAddress: DWORD,
    ReservedNumberOfFpoTableEntries: DWORD,
    ReservedFpoTableEntries: PFPO_DATA,
    SizeOfCoffSymbols: DWORD,
    CoffSymbols: PIMAGE_COFF_SYMBOLS_HEADER,
    ReservedSizeOfCodeViewSymbols: DWORD,
    ReservedCodeViewSymbols: PVOID,
    ImageFilePath: PSTR,
    ImageFileName: PSTR,
    ReservedDebugFilePath: PSTR,
    ReservedTimeDateStamp: DWORD,
    ReservedRomImage: BOOL,
    ReservedDebugDirectory: PIMAGE_DEBUG_DIRECTORY,
    ReservedNumberOfDebugDirectories: DWORD,
    ReservedOriginalFunctionTableBaseAddress: DWORD,
    Reserved: [DWORD; 2],
}}
#[cfg(target_pointer_width = "32")]
pub type PIMAGE_DEBUG_INFORMATION = *mut IMAGE_DEBUG_INFORMATION;
FN!{stdcall PENUMDIRTREE_CALLBACK(
    FilePath: PCSTR,
    CallerData: PVOID,
) -> BOOL}
FN!{stdcall PENUMDIRTREE_CALLBACKW(
    FilePath: PCWSTR,
    CallerData: PVOID,
) -> BOOL}
pub const UNDNAME_COMPLETE: DWORD = 0x0000;
pub const UNDNAME_NO_LEADING_UNDERSCORES: DWORD = 0x0001;
pub const UNDNAME_NO_MS_KEYWORDS: DWORD = 0x0002;
pub const UNDNAME_NO_FUNCTION_RETURNS: DWORD = 0x0004;
pub const UNDNAME_NO_ALLOCATION_MODEL: DWORD = 0x0008;
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: DWORD = 0x0010;
pub const UNDNAME_NO_MS_THISTYPE: DWORD = 0x0020;
pub const UNDNAME_NO_CV_THISTYPE: DWORD = 0x0040;
pub const UNDNAME_NO_THISTYPE: DWORD = 0x0060;
pub const UNDNAME_NO_ACCESS_SPECIFIERS: DWORD = 0x0080;
pub const UNDNAME_NO_THROW_SIGNATURES: DWORD = 0x0100;
pub const UNDNAME_NO_MEMBER_TYPE: DWORD = 0x0200;
pub const UNDNAME_NO_RETURN_UDT_MODEL: DWORD = 0x0400;
pub const UNDNAME_32_BIT_DECODE: DWORD = 0x0800;
pub const UNDNAME_NAME_ONLY: DWORD = 0x1000;
pub const UNDNAME_NO_ARGUMENTS: DWORD = 0x2000;
pub const UNDNAME_NO_SPECIAL_SYMS: DWORD = 0x4000;
pub const DBHHEADER_DEBUGDIRS: DWORD = 0x1;
pub const DBHHEADER_CVMISC: DWORD = 0x2;
pub const DBHHEADER_PDBGUID: DWORD = 0x3;
STRUCT!{struct MODLOAD_DATA {
    ssize: DWORD,
    ssig: DWORD,
    data: PVOID,
    size: DWORD,
    flags: DWORD,
}}
pub type PMODLOAD_DATA = *mut MODLOAD_DATA;
STRUCT!{struct MODLOAD_CVMISC {
    oCV: DWORD,
    cCV: size_t,
    oMisc: DWORD,
    cMisc: size_t,
    dtImage: DWORD,
    cImage: DWORD,
}}
pub type PMODLOAD_CVMISC = *mut MODLOAD_CVMISC;
STRUCT!{struct MODLOAD_PDBGUID_PDBAGE {
    PdbGuid: GUID,
    PdbAge: DWORD,
}}
pub type PMODLOAD_PDBGUID_PDBAGE = *mut MODLOAD_PDBGUID_PDBAGE;
ENUM!{enum ADDRESS_MODE {
    AddrMode1616,
    AddrMode1632,
    AddrModeReal,
    AddrModeFlat,
}}
STRUCT!{struct ADDRESS64 {
    Offset: DWORD64,
    Segment: WORD,
    Mode: ADDRESS_MODE,
}}
pub type LPADDRESS64 = *mut ADDRESS64;
#[cfg(target_pointer_width = "64")]
pub type ADDRESS = ADDRESS64;
#[cfg(target_pointer_width = "64")]
pub type LPADDRESS = LPADDRESS64;
#[cfg(target_pointer_width = "32")]
STRUCT!{struct ADDRESS {
    Offset: DWORD,
    Segment: WORD,
    Mode: ADDRESS_MODE,
}}
#[cfg(target_pointer_width = "32")]
pub type LPADDRESS = *mut ADDRESS;
STRUCT!{struct KDHELP64 {
    Thread: DWORD64,
    ThCallbackStack: DWORD,
    ThCallbackBStore: DWORD,
    NextCallback: DWORD,
    FramePointer: DWORD,
    KiCallUserMode: DWORD64,
    KeUserCallbackDispatcher: DWORD64,
    SystemRangeStart: DWORD64,
    KiUserExceptionDispatcher: DWORD64,
    StackBase: DWORD64,
    StackLimit: DWORD64,
    BuildVersion: DWORD,
    Reserved0: DWORD,
    Reserved1: [DWORD64; 4],
}}
pub type PKDHELP64 = *mut KDHELP64;
#[cfg(target_pointer_width = "64")]
pub type KDHELP = KDHELP64;
#[cfg(target_pointer_width = "64")]
pub type PKDHELP = PKDHELP64;
#[cfg(target_pointer_width = "32")]
STRUCT!{struct KDHELP {
    Thread: DWORD,
    ThCallbackStack: DWORD,
    NextCallback: DWORD,
    FramePointer: DWORD,
    KiCallUserMode: DWORD,
    KeUserCallbackDispatcher: DWORD,
    SystemRangeStart: DWORD,
    ThCallbackBStore: DWORD,
    KiUserExceptionDispatcher: DWORD,
    StackBase: DWORD,
    StackLimit: DWORD,
    Reserved: [DWORD; 5],
}}
#[cfg(target_pointer_width = "32")]
pub type PKDHELP = *mut KDHELP;
STRUCT!{struct STACKFRAME64 {
    AddrPC: ADDRESS64,
    AddrReturn: ADDRESS64,
    AddrFrame: ADDRESS64,
    AddrStack: ADDRESS64,
    AddrBStore: ADDRESS64,
    FuncTableEntry: PVOID,
    Params: [DWORD64; 4],
    Far: BOOL,
    Virtual: BOOL,
    Reserved: [DWORD64; 3],
    KdHelp: KDHELP64,
}}
pub type LPSTACKFRAME64 = *mut STACKFRAME64;
pub const INLINE_FRAME_CONTEXT_INIT: DWORD = 0;
pub const INLINE_FRAME_CONTEXT_IGNORE: DWORD = 0xFFFFFFFF;
STRUCT!{struct STACKFRAME_EX {
    AddrPC: ADDRESS64,
    AddrReturn: ADDRESS64,
    AddrFrame: ADDRESS64,
    AddrStack: ADDRESS64,
    AddrBStore: ADDRESS64,
    FuncTableEntry: PVOID,
    Params: [DWORD64; 4],
    Far: BOOL,
    Virtual: BOOL,
    Reserved: [DWORD64; 3],
    KdHelp: KDHELP64,
    StackFrameSize: DWORD,
    InlineFrameContext: DWORD,
}}
pub type LPSTACKFRAME_EX = *mut STACKFRAME_EX;
#[cfg(target_pointer_width = "64")]
pub type STACKFRAME = STACKFRAME64;
#[cfg(target_pointer_width = "64")]
pub type LPSTACKFRAME = LPSTACKFRAME64;
#[cfg(target_pointer_width = "32")]
STRUCT!{struct STACKFRAME {
    AddrPC: ADDRESS,
    AddrReturn: ADDRESS,
    AddrFrame: ADDRESS,
    AddrStack: ADDRESS,
    FuncTableEntry: PVOID,
    Params: [DWORD; 4],
    Far: BOOL,
    Virtual: BOOL,
    Reserved: [DWORD; 3],
    KdHelp: KDHELP,
    AddrBStore: ADDRESS,
}}
#[cfg(target_pointer_width = "32")]
pub type LPSTACKFRAME = *mut STACKFRAME;
FN!{stdcall PREAD_PROCESS_MEMORY_ROUTINE64(
    hProcess: HANDLE,
    qwBaseAddress: DWORD64,
    lpBuffer: PVOID,
    nSize: DWORD,
    lpNumberOfBytesRead: LPDWORD,
) -> BOOL}
FN!{stdcall PFUNCTION_TABLE_ACCESS_ROUTINE64(
    ahProcess: HANDLE,
    AddrBase: DWORD64,
) -> PVOID}
FN!{stdcall PGET_MODULE_BASE_ROUTINE64(
    hProcess: HANDLE,
    Address: DWORD64,
) -> DWORD64}
FN!{stdcall PTRANSLATE_ADDRESS_ROUTINE64(
    hProcess: HANDLE,
    hThread: HANDLE,
    lpaddr: LPADDRESS64,
) -> DWORD64}
pub const SYM_STKWALK_DEFAULT: DWORD = 0x00000000;
pub const SYM_STKWALK_FORCE_FRAMEPTR: DWORD = 0x00000001;
#[cfg(target_pointer_width = "64")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = PREAD_PROCESS_MEMORY_ROUTINE64;
#[cfg(target_pointer_width = "64")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = PFUNCTION_TABLE_ACCESS_ROUTINE64;
#[cfg(target_pointer_width = "64")]
pub type PGET_MODULE_BASE_ROUTINE = PGET_MODULE_BASE_ROUTINE64;
#[cfg(target_pointer_width = "64")]
pub type PTRANSLATE_ADDRESS_ROUTINE = PTRANSLATE_ADDRESS_ROUTINE64;
#[cfg(target_pointer_width = "32")]
FN!{stdcall PREAD_PROCESS_MEMORY_ROUTINE(
    hProcess: HANDLE,
    qwBaseAddress: DWORD,
    lpBuffer: PVOID,
    nSize: DWORD,
    lpNumberOfBytesRead: PDWORD,
) -> BOOL}
#[cfg(target_pointer_width = "32")]
FN!{stdcall PFUNCTION_TABLE_ACCESS_ROUTINE(
    ahProcess: HANDLE,
    AddrBase: DWORD,
) -> PVOID}
#[cfg(target_pointer_width = "32")]
FN!{stdcall PGET_MODULE_BASE_ROUTINE(
    hProcess: HANDLE,
    Address: DWORD,
) -> DWORD}
#[cfg(target_pointer_width = "32")]
FN!{stdcall PTRANSLATE_ADDRESS_ROUTINE(
    hProcess: HANDLE,
    hThread: HANDLE,
    lpaddr: LPADDRESS,
) -> DWORD}
pub const API_VERSION_NUMBER: USHORT = 12;
STRUCT!{struct API_VERSION {
    MajorVersion: USHORT,
    MinorVersion: USHORT,
    Revision: USHORT,
    Reserved: USHORT,
}}
pub type LPAPI_VERSION = *mut API_VERSION;
STRUCT!{struct SYMBOL_INFOW {
    SizeOfStruct: ULONG,
    TypeIndex: ULONG,
    Reserved: [ULONG64; 2],
    Index: ULONG,
    Size: ULONG,
    ModBase: ULONG64,
    Flags: ULONG,
    Value: ULONG64,
    Address: ULONG64,
    Register: ULONG,
    Scope: ULONG,
    Tag: ULONG,
    NameLen: ULONG,
    MaxNameLen: ULONG,
    Name: [WCHAR; 1],
}}
pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;
ENUM!{enum SYM_TYPE {
    SymNone = 0,
    SymCoff,
    SymCv,
    SymPdb,
    SymExport,
    SymDeferred,
    SymSym,
    SymDia,
    SymVirtual,
    NumSymTypes,
}}
STRUCT!{struct IMAGEHLP_SYMBOL64 {
    SizeOfStruct: DWORD,
    Address: DWORD64,
    Size: DWORD,
    Flags: DWORD,
    MaxNameLength: DWORD,
    Name: [CHAR; 1],
}}
pub type PIMAGEHLP_SYMBOL64 = *mut IMAGEHLP_SYMBOL64;
STRUCT!{struct IMAGEHLP_MODULEW64 {
    SizeOfStruct: DWORD,
    BaseOfImage: DWORD64,
    ImageSize: DWORD,
    TimeDateStamp: DWORD,
    CheckSum: DWORD,
    NumSyms: DWORD,
    SymType: SYM_TYPE,
    ModuleName: [WCHAR; 32],
    ImageName: [WCHAR; 256],
    LoadedImageName: [WCHAR; 256],
    LoadedPdbName: [WCHAR; 256],
    CVSig: DWORD,
    CVData: [WCHAR; MAX_PATH * 3],
    PdbSig: DWORD,
    PdbSig70: GUID,
    PdbAge: DWORD,
    PdbUnmatched: BOOL,
    DbgUnmatched: BOOL,
    LineNumbers: BOOL,
    GlobalSymbols: BOOL,
    TypeInfo: BOOL,
    SourceIndexed: BOOL,
    Publics: BOOL,
    MachineType: DWORD,
    Reserved: DWORD,
}}
pub type PIMAGEHLP_MODULEW64 = *mut IMAGEHLP_MODULEW64;
STRUCT!{struct IMAGEHLP_LINEW64 {
    SizeOfStruct: DWORD,
    Key: PVOID,
    LineNumber: DWORD,
    FileName: PWSTR,
    Address: DWORD64,
}}
pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;
extern "system" {
    pub fn EnumDirTree(
        hProcess: HANDLE,
        RootPath: PCSTR,
        InputPathName: PCSTR,
        OutputPathBuffer: PSTR,
        cb: PENUMDIRTREE_CALLBACK,
        data: PVOID,
    ) -> BOOL;
    pub fn EnumDirTreeW(
        hProcess: HANDLE,
        RootPath: PCWSTR,
        InputPathName: PCWSTR,
        OutputPathBuffer: PWSTR,
        cb: PENUMDIRTREE_CALLBACKW,
        data: PVOID,
    ) -> BOOL;
    pub fn ImagehlpApiVersion() -> LPAPI_VERSION;
    pub fn ImagehlpApiVersionEx(
        AppVersion: LPAPI_VERSION,
    ) -> LPAPI_VERSION;
    pub fn MakeSureDirectoryPathExists(
        DirPath: PCSTR,
    ) -> BOOL;
    pub fn SearchTreeForFile(
        RootPath: PCSTR,
        InputPathName: PCSTR,
        OutputPathBuffer: PSTR,
    ) -> BOOL;
    pub fn SearchTreeForFileW(
        RootPath: PCWSTR,
        InputPathName: PCWSTR,
        OutputPathBuffer: PWSTR,
    ) -> BOOL;
    pub fn FindDebugInfoFile(
        FileName: PCSTR,
        SymbolPath: PCSTR,
        DebugFilePath: PSTR,
    ) -> HANDLE;
    pub fn FindDebugInfoFileEx(
        FileName: PCSTR,
        SymbolPath: PCSTR,
        DebugFilePath: PSTR,
        Callback: PFIND_DEBUG_FILE_CALLBACK,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn FindDebugInfoFileExW(
        FileName: PCWSTR,
        SymbolPath: PCWSTR,
        DebugFilePath: PWSTR,
        Callback: PFIND_DEBUG_FILE_CALLBACKW,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn FindExecutableImage(
        FileName: PCSTR,
        SymbolPath: PCSTR,
        ImageFilePath: PSTR,
    ) -> HANDLE;
    pub fn FindExecutableImageEx(
        FileName: PCSTR,
        SymbolPath: PCSTR,
        ImageFilePath: PSTR,
        Callback: PFIND_EXE_FILE_CALLBACK,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn FindExecutableImageExW(
        FileName: PCWSTR,
        SymbolPath: PCWSTR,
        ImageFilePath: PWSTR,
        Callback: PFIND_EXE_FILE_CALLBACKW,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn StackWalk(
        MachineType: DWORD,
        hProcess: HANDLE,
        hThread: HANDLE,
        StackFrame: LPSTACKFRAME,
        ContextRecord: PVOID,
        ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE,
        FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE,
        GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE,
        TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE,
    ) -> BOOL;
    pub fn StackWalkEx(
        MachineType: DWORD,
        hProcess: HANDLE,
        hThread: HANDLE,
        StackFrame: LPSTACKFRAME_EX,
        ContextRecord: PVOID,
        ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
        FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
        GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
        TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64,
        Flags: DWORD,
    ) -> BOOL;
    pub fn StackWalk64(
        MachineType: DWORD,
        hProcess: HANDLE,
        hThread: HANDLE,
        StackFrame: LPSTACKFRAME64,
        ContextRecord: PVOID,
        ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
        FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
        GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
        TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64,
    ) -> BOOL;
    pub fn UnDecorateSymbolName(
        name: PCSTR,
        outputString: PSTR,
        maxStringLength: DWORD,
        flags: DWORD,
    ) -> DWORD;
    pub fn UnDecorateSymbolNameW(
        name: PCWSTR,
        outputString: PWSTR,
        maxStringLength: DWORD,
        flags: DWORD,
    ) -> DWORD;
    pub fn GetTimestampForLoadedLibrary(
        Module: HMODULE,
    ) -> DWORD;
    pub fn ImageDirectoryEntryToData(
        Base: PVOID,
        MappedAsImage: BOOLEAN,
        DirectoryEntry: USHORT,
        Size: PULONG,
    ) -> PVOID;
    pub fn ImageDirectoryEntryToDataEx(
        Base: PVOID,
        MappedAsImage: BOOLEAN,
        DirectoryEntry: USHORT,
        Size: PULONG,
        FoundHeader: *mut PIMAGE_SECTION_HEADER,
    ) -> PVOID;
    pub fn ImageNtHeader(
        Base: PVOID,
    ) -> PIMAGE_NT_HEADERS;
    pub fn ImageRvaToSection(
        NtHeaders: PIMAGE_NT_HEADERS,
        Base: PVOID,
        Rva: ULONG,
    ) -> PIMAGE_SECTION_HEADER;
    pub fn ImageRvaToVa(
        NtHeaders: PIMAGE_NT_HEADERS,
        Base: PVOID,
        Rva: ULONG,
        LastRvaSection: *mut PIMAGE_SECTION_HEADER,
    ) -> PVOID;
}
pub const SYMOPT_CASE_INSENSITIVE: DWORD = 0x00000001;
pub const SYMOPT_UNDNAME: DWORD = 0x00000002;
pub const SYMOPT_DEFERRED_LOADS: DWORD = 0x00000004;
pub const SYMOPT_NO_CPP: DWORD = 0x00000008;
pub const SYMOPT_LOAD_LINES: DWORD = 0x00000010;
pub const SYMOPT_OMAP_FIND_NEAREST: DWORD = 0x00000020;
pub const SYMOPT_LOAD_ANYTHING: DWORD = 0x00000040;
pub const SYMOPT_IGNORE_CVREC: DWORD = 0x00000080;
pub const SYMOPT_NO_UNQUALIFIED_LOADS: DWORD = 0x00000100;
pub const SYMOPT_FAIL_CRITICAL_ERRORS: DWORD = 0x00000200;
pub const SYMOPT_EXACT_SYMBOLS: DWORD = 0x00000400;
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: DWORD = 0x00000800;
pub const SYMOPT_IGNORE_NT_SYMPATH: DWORD = 0x00001000;
pub const SYMOPT_INCLUDE_32BIT_MODULES: DWORD = 0x00002000;
pub const SYMOPT_PUBLICS_ONLY: DWORD = 0x00004000;
pub const SYMOPT_NO_PUBLICS: DWORD = 0x00008000;
pub const SYMOPT_AUTO_PUBLICS: DWORD = 0x00010000;
pub const SYMOPT_NO_IMAGE_SEARCH: DWORD = 0x00020000;
pub const SYMOPT_SECURE: DWORD = 0x00040000;
pub const SYMOPT_NO_PROMPTS: DWORD = 0x00080000;
pub const SYMOPT_OVERWRITE: DWORD = 0x00100000;
pub const SYMOPT_IGNORE_IMAGEDIR: DWORD = 0x00200000;
pub const SYMOPT_FLAT_DIRECTORY: DWORD = 0x00400000;
pub const SYMOPT_FAVOR_COMPRESSED: DWORD = 0x00800000;
pub const SYMOPT_ALLOW_ZERO_ADDRESS: DWORD = 0x01000000;
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: DWORD = 0x02000000;
pub const SYMOPT_READONLY_CACHE: DWORD = 0x04000000;
pub const SYMOPT_SYMPATH_LAST: DWORD = 0x08000000;
pub const SYMOPT_DISABLE_FAST_SYMBOLS: DWORD = 0x10000000;
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: DWORD = 0x20000000;
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: DWORD = 0x40000000;
pub const SYMOPT_DEBUG: DWORD = 0x80000000;
extern "system" {
    pub fn SymSetOptions(
        SymOptions: DWORD,
    ) -> DWORD;
    pub fn SymGetOptions() -> DWORD;
    pub fn SymCleanup(
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn SymEnumSymbolsW(
        hProcess: HANDLE,
        BaseOfDll: ULONG64,
        Mask: PCWSTR,
        EnumSymbolsCallback: PSYM_ENUMERATESYMBOLS_CALLBACKW,
        CallerData: PVOID,
    ) -> BOOL;
    pub fn SymFindDebugInfoFile(
        hProcess: HANDLE,
        FileName: PCSTR,
        DebugFilePath: PSTR,
        Callback: PFIND_DEBUG_FILE_CALLBACK,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn SymFindDebugInfoFileW(
        hProcess: HANDLE,
        FileName: PCWSTR,
        DebugFilePath: PWSTR,
        Callback: PFIND_DEBUG_FILE_CALLBACKW,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn SymFindExecutableImage(
        hProcess: HANDLE,
        FileName: PCSTR,
        ImageFilePath: PSTR,
        Callback: PFIND_EXE_FILE_CALLBACK,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn SymFindExecutableImageW(
        hProcess: HANDLE,
        FileName: PCWSTR,
        ImageFilePath: PWSTR,
        Callback: PFIND_EXE_FILE_CALLBACKW,
        CallerData: PVOID,
    ) -> HANDLE;
    pub fn SymFindFileInPath(
        hprocess: HANDLE,
        SearchPath: PCSTR,
        FileName: PCSTR,
        id: PVOID,
        two: DWORD,
        three: DWORD,
        flags: DWORD,
        FoundFile: PSTR,
        callback: PFINDFILEINPATHCALLBACK,
        context: PVOID,
    ) -> BOOL;
    pub fn SymFindFileInPathW(
        hprocess: HANDLE,
        SearchPath: PCWSTR,
        FileName: PCWSTR,
        id: PVOID,
        two: DWORD,
        three: DWORD,
        flags: DWORD,
        FoundFile: PWSTR,
        callback: PFINDFILEINPATHCALLBACKW,
        context: PVOID,
    ) -> BOOL;
    pub fn SymFromAddrW(
        hProcess: HANDLE,
        Address: DWORD64,
        Displacement: PDWORD64,
        Symbol: PSYMBOL_INFOW,
    ) -> BOOL;
    pub fn SymFromNameW(
        hProcess: HANDLE,
        Name: PCWSTR,
        Symbol: PSYMBOL_INFOW,
    ) -> BOOL;
    pub fn SymFunctionTableAccess64(
        hProcess: HANDLE,
        AddrBase: DWORD64,
    ) -> PVOID;
    pub fn SymGetLineFromAddrW64(
        hProcess: HANDLE,
        dwAddr: DWORD64,
        pdwDisplacement: PDWORD,
        Line: PIMAGEHLP_LINEW64,
    ) -> BOOL;
    pub fn SymGetModuleInfoW64(
        hProcess: HANDLE,
        qwAddr: DWORD64,
        ModuleInfo: PIMAGEHLP_MODULEW64,
    ) -> BOOL;
    pub fn SymGetModuleBase64(
        hProcess: HANDLE,
        AddrBase: DWORD64,
    ) -> DWORD64;
    pub fn SymGetSymFromAddr64(
        hProcess: HANDLE,
        Address: DWORD64,
        Displacement: PDWORD64,
        Symbol: PIMAGEHLP_SYMBOL64,
    ) -> BOOL;
    pub fn SymInitializeW(
        hProcess: HANDLE,
        UserSearchPath: PCWSTR,
        fInvadeProcess: BOOL,
    ) -> BOOL;
    pub fn SymLoadModuleExW(
        hProcess: HANDLE,
        hFile: HANDLE,
        ImageName: PCWSTR,
        ModuleName: PCWSTR,
        BaseOfDll: DWORD64,
        SizeOfDll: DWORD,
        Data: PMODLOAD_DATA,
        Flags: DWORD,
    ) -> DWORD64;
    pub fn SymUnloadModule(
        hProcess: HANDLE,
        BaseOfDll: DWORD,
    ) -> BOOL;
    pub fn SymUnloadModule64(
        hProcess: HANDLE,
        BaseOfDll: DWORD64,
    ) -> BOOL;
    #[cfg(target_pointer_width = "32")]
    pub fn MapDebugInformation(
        FileHandle: HANDLE,
        FileName: PCSTR,
        SymbolPath: PCSTR,
        ImageBase: ULONG,
    ) -> PIMAGE_DEBUG_INFORMATION;
    #[cfg(target_pointer_width = "32")]
    pub fn UnmapDebugInformation(
        DebugInfo: PIMAGE_DEBUG_INFORMATION,
    ) -> BOOL;
}
