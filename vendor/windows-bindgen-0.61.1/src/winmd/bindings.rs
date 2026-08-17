#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type CorElementType = u8;
pub const ELEMENT_TYPE_ARRAY: CorElementType = 20u8;
pub const ELEMENT_TYPE_BOOLEAN: CorElementType = 2u8;
pub const ELEMENT_TYPE_BYREF: CorElementType = 16u8;
pub const ELEMENT_TYPE_CHAR: CorElementType = 3u8;
pub const ELEMENT_TYPE_CLASS: CorElementType = 18u8;
pub const ELEMENT_TYPE_CMOD_OPT: CorElementType = 32u8;
pub const ELEMENT_TYPE_CMOD_REQD: CorElementType = 31u8;
pub const ELEMENT_TYPE_GENERICINST: CorElementType = 21u8;
pub const ELEMENT_TYPE_I: CorElementType = 24u8;
pub const ELEMENT_TYPE_I1: CorElementType = 4u8;
pub const ELEMENT_TYPE_I2: CorElementType = 6u8;
pub const ELEMENT_TYPE_I4: CorElementType = 8u8;
pub const ELEMENT_TYPE_I8: CorElementType = 10u8;
pub const ELEMENT_TYPE_OBJECT: CorElementType = 28u8;
pub const ELEMENT_TYPE_PTR: CorElementType = 15u8;
pub const ELEMENT_TYPE_R4: CorElementType = 12u8;
pub const ELEMENT_TYPE_R8: CorElementType = 13u8;
pub const ELEMENT_TYPE_STRING: CorElementType = 14u8;
pub const ELEMENT_TYPE_SZARRAY: CorElementType = 29u8;
pub const ELEMENT_TYPE_U: CorElementType = 25u8;
pub const ELEMENT_TYPE_U1: CorElementType = 5u8;
pub const ELEMENT_TYPE_U2: CorElementType = 7u8;
pub const ELEMENT_TYPE_U4: CorElementType = 9u8;
pub const ELEMENT_TYPE_U8: CorElementType = 11u8;
pub const ELEMENT_TYPE_VALUETYPE: CorElementType = 17u8;
pub const ELEMENT_TYPE_VAR: CorElementType = 19u8;
pub const ELEMENT_TYPE_VOID: CorElementType = 1u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR20_HEADER {
    pub cb: u32,
    pub MajorRuntimeVersion: u16,
    pub MinorRuntimeVersion: u16,
    pub MetaData: IMAGE_DATA_DIRECTORY,
    pub Flags: u32,
    pub Anonymous: IMAGE_COR20_HEADER_0,
    pub Resources: IMAGE_DATA_DIRECTORY,
    pub StrongNameSignature: IMAGE_DATA_DIRECTORY,
    pub CodeManagerTable: IMAGE_DATA_DIRECTORY,
    pub VTableFixups: IMAGE_DATA_DIRECTORY,
    pub ExportAddressTableJumps: IMAGE_DATA_DIRECTORY,
    pub ManagedNativeHeader: IMAGE_DATA_DIRECTORY,
}
impl Default for IMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR20_HEADER_0 {
    pub EntryPointToken: u32,
    pub EntryPointRVA: u32,
}
impl Default for IMAGE_COR20_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}
pub type IMAGE_DIRECTORY_ENTRY = u16;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: IMAGE_DIRECTORY_ENTRY = 14u16;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: IMAGE_DLL_CHARACTERISTICS = 64u16;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: IMAGE_DLL_CHARACTERISTICS = 1024u16;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: IMAGE_DLL_CHARACTERISTICS = 256u16;
pub type IMAGE_DLL_CHARACTERISTICS = u16;
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}
impl Default for IMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DOS_SIGNATURE: u16 = 23117u16;
pub const IMAGE_FILE_32BIT_MACHINE: IMAGE_FILE_CHARACTERISTICS = 256u16;
pub type IMAGE_FILE_CHARACTERISTICS = u16;
pub const IMAGE_FILE_DLL: IMAGE_FILE_CHARACTERISTICS = 8192u16;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: IMAGE_FILE_CHARACTERISTICS = 2u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: IMAGE_FILE_MACHINE,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: IMAGE_FILE_CHARACTERISTICS,
}
pub type IMAGE_FILE_MACHINE = u16;
pub const IMAGE_FILE_MACHINE_I386: IMAGE_FILE_MACHINE = 332u16;
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = 267u16;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = 523u16;
pub const IMAGE_NT_SIGNATURE: u32 = 17744u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: IMAGE_OPTIONAL_HEADER_MAGIC,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub ImageBase: u32,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: IMAGE_SUBSYSTEM,
    pub DllCharacteristics: IMAGE_DLL_CHARACTERISTICS,
    pub SizeOfStackReserve: u32,
    pub SizeOfStackCommit: u32,
    pub SizeOfHeapReserve: u32,
    pub SizeOfHeapCommit: u32,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: IMAGE_OPTIONAL_HEADER_MAGIC,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: IMAGE_SUBSYSTEM,
    pub DllCharacteristics: IMAGE_DLL_CHARACTERISTICS,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGE_OPTIONAL_HEADER_MAGIC = u16;
pub type IMAGE_SECTION_CHARACTERISTICS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    pub Misc: IMAGE_SECTION_HEADER_0,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: IMAGE_SECTION_CHARACTERISTICS,
}
impl Default for IMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_SECTION_HEADER_0 {
    pub PhysicalAddress: u32,
    pub VirtualSize: u32,
}
impl Default for IMAGE_SECTION_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGE_SUBSYSTEM = u16;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: IMAGE_SUBSYSTEM = 3u16;
