//! The module for the PE optional header ([`OptionalHeader`]) and related items.

use crate::container;
use crate::error;

use crate::pe::data_directories;

use scroll::{ctx, Endian, LE};
use scroll::{Pread, Pwrite, SizeWith};

/// Standard 32-bit COFF fields (for `PE32`).
///
/// In `winnt.h`, this is a subset of [`IMAGE_OPTIONAL_HEADER32`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32).
///
/// * For 64-bit version, see [`StandardFields64`].
/// * For unified version, see [`StandardFields`].
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct StandardFields32 {
    /// See docs for [`StandardFields::magic`](crate::pe::optional_header::StandardFields::magic).
    pub magic: u16,
    /// See docs for [`StandardFields::major_linker_version`].
    pub major_linker_version: u8,
    /// See docs for [`StandardFields::minor_linker_version`].
    pub minor_linker_version: u8,
    /// See docs for [`StandardFields::size_of_code`].
    pub size_of_code: u32,
    /// See docs for [`StandardFields::size_of_initialized_data`].
    pub size_of_initialized_data: u32,
    /// See docs for [`StandardFields::size_of_uninitialized_data`].
    pub size_of_uninitialized_data: u32,
    /// See docs for [`StandardFields::address_of_entry_point`].
    pub address_of_entry_point: u32,
    /// See docs for [`StandardFields::base_of_code`].
    pub base_of_code: u32,
    /// See docs for [`StandardFields::base_of_data`].
    pub base_of_data: u32,
}

/// Convenience constant for `core::mem::size_of::<StandardFields32>()`.
pub const SIZEOF_STANDARD_FIELDS_32: usize = 28;

/// Standard 64-bit COFF fields (for `PE32+`).
///
/// In `winnt.h`, this is a subset of [`IMAGE_OPTIONAL_HEADER64`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64).
///
/// * For 32-bit version, see [`StandardFields32`].
/// * For unified version, see [`StandardFields`].
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct StandardFields64 {
    /// See docs for [`StandardFields::magic`](crate::pe::optional_header::StandardFields::magic).
    pub magic: u16,
    /// See docs for [`StandardFields::major_linker_version`].
    pub major_linker_version: u8,
    /// See docs for [`StandardFields::minor_linker_version`].
    pub minor_linker_version: u8,
    /// See docs for [`StandardFields::size_of_code`].
    pub size_of_code: u32,
    /// See docs for [`StandardFields::size_of_initialized_data`].
    pub size_of_initialized_data: u32,
    /// See docs for [`StandardFields::size_of_uninitialized_data`].
    pub size_of_uninitialized_data: u32,
    /// See docs for [`StandardFields::address_of_entry_point`].
    pub address_of_entry_point: u32,
    /// See docs for [`StandardFields::base_of_code`].
    pub base_of_code: u32,
}

/// Convenience constant for `core::mem::size_of::<StandardFields64>()`.
pub const SIZEOF_STANDARD_FIELDS_64: usize = 24;

/// Unified 32/64-bit standard COFF fields (for `PE32` and `PE32+`).
///
/// Notably, a value of this type is a member of
/// [`goblin::pe::optional_header::OptionalHeader`](crate::pe::optional_header::OptionalHeader),
/// which in turn represents either
/// * [`IMAGE_OPTIONAL_HEADER32`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32); or
/// * [`IMAGE_OPTIONAL_HEADER64`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64)
///
/// from `winnt.h`, depending on the value of [`StandardFields::magic`].
///
/// ## Position in PE binary
///
/// Standard COFF fields are located at the beginning of the [`OptionalHeader`] and before the
/// [`WindowsFields`].
///
/// ## Related structures
///
/// * For 32-bit version, see [`StandardFields32`].
/// * For 64-bit version, see [`StandardFields64`].
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct StandardFields {
    /// The state of the image file. This member can be one of the following values:
    ///
    /// * [`IMAGE_NT_OPTIONAL_HDR32_MAGIC`].
    /// * [`IMAGE_NT_OPTIONAL_HDR64_MAGIC`].
    /// * [`IMAGE_ROM_OPTIONAL_HDR_MAGIC`].
    #[doc(alias = "Magic")]
    pub magic: u16,
    /// The major version number of the linker.
    #[doc(alias = "MajorLinkerVersion")]
    pub major_linker_version: u8,
    /// The minor version number of the linker.
    #[doc(alias = "MinorLinkerVersion")]
    pub minor_linker_version: u8,
    /// The size of the code section (.text), in bytes, or the sum of all such sections if there are multiple code sections.
    #[doc(alias = "SizeOfCode")]
    pub size_of_code: u64,
    /// The size of the initialized data section (.data), in bytes, or the sum of all such sections if there are multiple initialized data sections.
    #[doc(alias = "SizeOfInitializedData")]
    pub size_of_initialized_data: u64,
    /// The size of the uninitialized data section (.bss), in bytes, or the sum of all such sections if there are multiple uninitialized data sections.
    #[doc(alias = "SizeOfUninitializedData")]
    pub size_of_uninitialized_data: u64,
    /// A pointer to the entry point function, relative to the image base address.
    ///
    /// * For executable files, this is the starting address.
    /// * For device drivers, this is the address of the initialization function.
    ///
    /// The entry point function is optional for DLLs. When no entry point is present, this member is zero.
    pub address_of_entry_point: u64,
    /// A pointer to the beginning of the code section (.text), relative to the image base.
    pub base_of_code: u64,
    /// A pointer to the beginning of the data section (.data), relative to the image base. Absent in 64-bit PE32+.
    ///
    /// In other words, it is a Relative virtual address (RVA) of the start of the data (.data) section when the PE
    /// is loaded into memory.
    // Q (JohnScience): Why is this a u32 and not an Option<u32>?
    pub base_of_data: u32,
}

impl From<StandardFields32> for StandardFields {
    fn from(fields: StandardFields32) -> Self {
        StandardFields {
            magic: fields.magic,
            major_linker_version: fields.major_linker_version,
            minor_linker_version: fields.minor_linker_version,
            size_of_code: u64::from(fields.size_of_code),
            size_of_initialized_data: u64::from(fields.size_of_initialized_data),
            size_of_uninitialized_data: u64::from(fields.size_of_uninitialized_data),
            address_of_entry_point: u64::from(fields.address_of_entry_point),
            base_of_code: u64::from(fields.base_of_code),
            base_of_data: fields.base_of_data,
        }
    }
}

impl From<StandardFields> for StandardFields32 {
    fn from(fields: StandardFields) -> Self {
        StandardFields32 {
            magic: fields.magic,
            major_linker_version: fields.major_linker_version,
            minor_linker_version: fields.minor_linker_version,
            size_of_code: fields.size_of_code as u32,
            size_of_initialized_data: fields.size_of_initialized_data as u32,
            size_of_uninitialized_data: fields.size_of_uninitialized_data as u32,
            address_of_entry_point: fields.address_of_entry_point as u32,
            base_of_code: fields.base_of_code as u32,
            base_of_data: fields.base_of_data,
        }
    }
}

impl From<StandardFields64> for StandardFields {
    fn from(fields: StandardFields64) -> Self {
        StandardFields {
            magic: fields.magic,
            major_linker_version: fields.major_linker_version,
            minor_linker_version: fields.minor_linker_version,
            size_of_code: u64::from(fields.size_of_code),
            size_of_initialized_data: u64::from(fields.size_of_initialized_data),
            size_of_uninitialized_data: u64::from(fields.size_of_uninitialized_data),
            address_of_entry_point: u64::from(fields.address_of_entry_point),
            base_of_code: u64::from(fields.base_of_code),
            base_of_data: 0,
        }
    }
}

impl From<StandardFields> for StandardFields64 {
    fn from(fields: StandardFields) -> Self {
        StandardFields64 {
            magic: fields.magic,
            major_linker_version: fields.major_linker_version,
            minor_linker_version: fields.minor_linker_version,
            size_of_code: fields.size_of_code as u32,
            size_of_initialized_data: fields.size_of_initialized_data as u32,
            size_of_uninitialized_data: fields.size_of_uninitialized_data as u32,
            address_of_entry_point: fields.address_of_entry_point as u32,
            base_of_code: fields.base_of_code as u32,
        }
    }
}

/// Standard fields magic number for 32-bit binary (`PE32`).
pub const MAGIC_32: u16 = 0x10b;
/// Standard fields magic number for 64-bit binary (`PE32+`).
pub const MAGIC_64: u16 = 0x20b;

/// Windows specific fields for 32-bit binary (`PE32`). They're also known as "NT additional fields".
///
/// In `winnt.h`, this is a subset of [`IMAGE_OPTIONAL_HEADER32`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32).
///
/// * For 64-bit version, see [`WindowsFields64`].
/// * For unified version, see [`WindowsFields`].
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct WindowsFields32 {
    /// See docs for [`WindowsFields::image_base`].
    pub image_base: u32,
    /// See docs for [`WindowsFields::section_alignment`].
    pub section_alignment: u32,
    /// See docs for [`WindowsFields::file_alignment`].
    pub file_alignment: u32,
    /// See docs for [`WindowsFields::major_operating_system_version`].
    pub major_operating_system_version: u16,
    /// See docs for [`WindowsFields::minor_operating_system_version`].
    pub minor_operating_system_version: u16,
    /// See docs for [`WindowsFields::major_image_version`].
    pub major_image_version: u16,
    /// See docs for [`WindowsFields::minor_image_version`].
    pub minor_image_version: u16,
    /// See docs for [`WindowsFields::major_subsystem_version`].
    pub major_subsystem_version: u16,
    /// See docs for [`WindowsFields::minor_subsystem_version`].
    pub minor_subsystem_version: u16,
    /// See docs for [`WindowsFields::win32_version_value`].
    pub win32_version_value: u32,
    /// See docs for [`WindowsFields::size_of_image`].
    pub size_of_image: u32,
    /// See docs for [`WindowsFields::size_of_headers`].
    pub size_of_headers: u32,
    /// See docs for [`WindowsFields::check_sum`].
    pub check_sum: u32,
    /// See docs for [`WindowsFields::subsystem`].
    pub subsystem: u16,
    /// See docs for [`WindowsFields::dll_characteristics`].
    pub dll_characteristics: u16,
    /// See docs for [`WindowsFields::size_of_stack_reserve`].
    pub size_of_stack_reserve: u32,
    /// See docs for [`WindowsFields::size_of_stack_commit`].
    pub size_of_stack_commit: u32,
    /// See docs for [`WindowsFields::size_of_heap_reserve`].
    pub size_of_heap_reserve: u32,
    /// See docs for [`WindowsFields::size_of_heap_commit`].
    pub size_of_heap_commit: u32,
    /// See docs for [`WindowsFields::loader_flags`].
    pub loader_flags: u32,
    /// See docs for [`WindowsFields::number_of_rva_and_sizes`].
    pub number_of_rva_and_sizes: u32,
}

/// Convenience constant for `core::mem::size_of::<WindowsFields32>()`.
pub const SIZEOF_WINDOWS_FIELDS_32: usize = 68;
/// Offset of the `check_sum` field in [`WindowsFields32`].
pub const OFFSET_WINDOWS_FIELDS_32_CHECKSUM: usize = 36;

/// Windows specific fields for 64-bit binary (`PE32+`). They're also known as "NT additional fields".
///
/// In `winnt.h`, this is a subset of [`IMAGE_OPTIONAL_HEADER64`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64).
///
/// *Note: at the moment of writing, [`WindowsFields`] is an alias for `WindowsFields64`. Though [nominally equivalent](https://en.wikipedia.org/wiki/Nominal_type_system),
/// they're semantically distinct.*
///
/// * For 32-bit version, see [`WindowsFields32`].
/// * For unified version, see [`WindowsFields`].
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct WindowsFields64 {
    /// The *preferred* yet rarely provided address of the first byte of image when loaded into memory; must be a
    /// multiple of 64 K.
    ///
    /// This address is rarely used because Windows uses memory protection mechanisms like Address Space Layout
    /// Randomization (ASLR). As a result, it’s rare to see an image mapped to the preferred address. Instead,
    /// the Windows PE Loader maps the file to a different address with an unused memory range. This process
    /// would create issues because some addresses that would have been constant are now changed. The Loader
    /// addresses this via a process called PE relocation which fixes these constant addresses to work with the
    /// new image base. The relocation section (.reloc) holds data essential to this relocation process.
    /// [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/nt-headers/optional-header/).
    ///
    /// * The default address for DLLs is 0x10000000.
    /// * The default for Windows CE EXEs is 0x00010000.
    /// * The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is 0x00400000.
    ///
    /// ## Position in PE binary
    ///
    /// Windows fields are located inside [`OptionalHeader`] after [`StandardFields`] and before the
    /// [`DataDirectories`](data_directories::DataDirectories).
    ///
    /// ## Related structures
    ///
    /// * For 32-bit version, see [`WindowsFields32`].
    /// * For unified version, see [`WindowsFields`], especially the note on nominal equivalence.
    #[doc(alias = "ImageBase")]
    pub image_base: u64,
    /// Holds a byte value used for section alignment in memory.
    ///
    /// This value must be greater than or equal to
    /// [`file_alignment`](WindowsFields64::file_alignment), which is the next field.
    ///
    /// When loaded into memory, sections are aligned in memory boundaries that are multiples of this value.
    ///
    /// If the value is less than the architecture’s page size, then the value should match
    /// [`file_alignment`](WindowsFields64::file_alignment).
    /// [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/nt-headers/optional-header/).
    ///
    /// The default value is the page size for the architecture.
    #[doc(alias = "SectionAlignment")]
    pub section_alignment: u32,
    /// The alignment factor (in bytes) that is used to align the raw data of sections in the image file.
    ///
    /// The value should be a power of 2 between 512 and 64 K, inclusive.
    ///
    /// If the [`section_alignment`](WindowsFields64::section_alignment) is less than the architecture's page size,
    /// then [`file_alignment`](WindowsFields64::file_alignment) must match [`section_alignment`](WindowsFields64::section_alignment).
    ///
    /// If [`file_alignment`](WindowsFields64::file_alignment) is less than [`section_alignment`](WindowsFields64::section_alignment),
    /// then remainder will be padded with zeroes in order to maintain the alignment boundaries.
    /// [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/nt-headers/optional-header/).
    ///
    /// The default value is 512.
    #[doc(alias = "FileAlignment")]
    pub file_alignment: u32,
    /// The major version number of the required operating system.
    #[doc(alias = "MajorOperatingSystemVersion")]
    pub major_operating_system_version: u16,
    /// The minor version number of the required operating system.
    #[doc(alias = "MinorOperatingSystemVersion")]
    pub minor_operating_system_version: u16,
    /// The major version number of the image.
    #[doc(alias = "MajorImageVersion")]
    pub major_image_version: u16,
    /// The minor version number of the image.
    #[doc(alias = "MinorImageVersion")]
    pub minor_image_version: u16,
    /// The major version number of the subsystem.
    #[doc(alias = "MajorSubsystemVersion")]
    pub major_subsystem_version: u16,
    /// The minor version number of the subsystem.
    #[doc(alias = "MinorSubsystemVersion")]
    pub minor_subsystem_version: u16,
    /// Reserved, must be zero.
    #[doc(alias = "Win32VersionValue")]
    pub win32_version_value: u32,
    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    ///
    /// It must be a multiple of the [`section_alignment`](WindowsFields64::section_alignment).
    #[doc(alias = "SizeOfImage")]
    pub size_of_image: u32,
    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of
    /// [`file_alignment`](WindowsFields64::file_alignment).
    #[doc(alias = "SizeOfHeaders")]
    pub size_of_headers: u32,
    /// The image file checksum. The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    ///
    /// The following are checked for validation at load time:
    /// * all drivers,
    /// * any DLL loaded at boot time, and
    /// * any DLL that is loaded into a critical Windows process.
    #[doc(alias = "CheckSum")]
    pub check_sum: u32,
    /// The subsystem that is required to run this image.
    ///
    /// The subsystem can be one of the values in the [`goblin::pe::subsystem`](crate::pe::subsystem) module.
    #[doc(alias = "Subsystem")]
    pub subsystem: u16,
    /// DLL characteristics of the image.
    ///
    /// DLL characteristics can be one of the values in the
    /// [`goblin::pe::dll_characteristic`](crate::pe::dll_characteristic) module.
    #[doc(alias = "DllCharacteristics")]
    pub dll_characteristics: u16,
    /// The size of the stack to reserve. Only [`WindowsFields::size_of_stack_commit`] is committed;
    /// the rest is made available one page at a time until the reserve size is reached.
    ///
    /// In the context of memory management in operating systems, "commit" refers to the act of allocating physical memory
    /// to back a portion of the virtual memory space.
    ///
    /// When a program requests memory, the operating system typically allocates virtual memory space for it. However,
    /// this virtual memory space doesn't immediately consume physical memory (RAM) resources. Instead, physical memory
    /// is only allocated when the program actually uses (or accesses) that portion of the virtual memory space.
    /// This allocation of physical memory to back virtual memory is called "committing" memory.
    #[doc(alias = "SizeOfStackReserve")]
    pub size_of_stack_reserve: u64,
    /// The size of the stack to commit.
    #[doc(alias = "SizeOfStackCommit")]
    pub size_of_stack_commit: u64,
    ///  The size of the local heap space to reserve. Only [`WindowsFields::size_of_heap_commit`] is committed; the rest
    /// is made available one page at a time until the reserve size is reached.
    #[doc(alias = "SizeOfHeapReserve")]
    pub size_of_heap_reserve: u64,
    /// The size of the local heap space to commit.
    #[doc(alias = "SizeOfHeapCommit")]
    pub size_of_heap_commit: u64,
    /// Reserved, must be zero.
    #[doc(alias = "LoaderFlags")]
    pub loader_flags: u32,
    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    #[doc(alias = "NumberOfRvaAndSizes")]
    pub number_of_rva_and_sizes: u32,
}

/// Convenience constant for `core::mem::size_of::<WindowsFields64>()`.
pub const SIZEOF_WINDOWS_FIELDS_64: usize = 88;
/// Offset of the `check_sum` field in [`WindowsFields64`].
pub const OFFSET_WINDOWS_FIELDS_64_CHECKSUM: usize = 40;

// /// Generic 32/64-bit Windows specific fields
// #[derive(Debug, PartialEq, Copy, Clone, Default)]
// pub struct WindowsFields {
//     pub image_base: u64,
//     pub section_alignment: u32,
//     pub file_alignment: u32,
//     pub major_operating_system_version: u16,
//     pub minor_operating_system_version: u16,
//     pub major_image_version: u16,
//     pub minor_image_version: u16,
//     pub major_subsystem_version: u16,
//     pub minor_subsystem_version: u16,
//     pub win32_version_value: u32,
//     pub size_of_image: u32,
//     pub size_of_headers: u32,
//     pub check_sum: u32,
//     pub subsystem: u16,
//     pub dll_characteristics: u16,
//     pub size_of_stack_reserve: u64,
//     pub size_of_stack_commit:  u64,
//     pub size_of_heap_reserve:  u64,
//     pub size_of_heap_commit:   u64,
//     pub loader_flags: u32,
//     pub number_of_rva_and_sizes: u32,
// }

impl From<WindowsFields32> for WindowsFields {
    fn from(windows: WindowsFields32) -> Self {
        WindowsFields {
            image_base: u64::from(windows.image_base),
            section_alignment: windows.section_alignment,
            file_alignment: windows.file_alignment,
            major_operating_system_version: windows.major_operating_system_version,
            minor_operating_system_version: windows.minor_operating_system_version,
            major_image_version: windows.major_image_version,
            minor_image_version: windows.minor_image_version,
            major_subsystem_version: windows.major_subsystem_version,
            minor_subsystem_version: windows.minor_subsystem_version,
            win32_version_value: windows.win32_version_value,
            size_of_image: windows.size_of_image,
            size_of_headers: windows.size_of_headers,
            check_sum: windows.check_sum,
            subsystem: windows.subsystem,
            dll_characteristics: windows.dll_characteristics,
            size_of_stack_reserve: u64::from(windows.size_of_stack_reserve),
            size_of_stack_commit: u64::from(windows.size_of_stack_commit),
            size_of_heap_reserve: u64::from(windows.size_of_heap_reserve),
            size_of_heap_commit: u64::from(windows.size_of_heap_commit),
            loader_flags: windows.loader_flags,
            number_of_rva_and_sizes: windows.number_of_rva_and_sizes,
        }
    }
}

impl TryFrom<WindowsFields64> for WindowsFields32 {
    type Error = crate::error::Error;

    fn try_from(value: WindowsFields64) -> Result<Self, Self::Error> {
        Ok(WindowsFields32 {
            image_base: value.image_base.try_into()?,
            section_alignment: value.section_alignment,
            file_alignment: value.file_alignment,
            major_operating_system_version: value.major_operating_system_version,
            minor_operating_system_version: value.minor_operating_system_version,
            major_image_version: value.major_image_version,
            minor_image_version: value.minor_image_version,
            major_subsystem_version: value.major_subsystem_version,
            minor_subsystem_version: value.minor_subsystem_version,
            win32_version_value: value.win32_version_value,
            size_of_image: value.size_of_image,
            size_of_headers: value.size_of_headers,
            check_sum: value.check_sum,
            subsystem: value.subsystem,
            dll_characteristics: value.dll_characteristics,
            size_of_stack_reserve: value.size_of_stack_reserve.try_into()?,
            size_of_stack_commit: value.size_of_stack_commit.try_into()?,
            size_of_heap_reserve: value.size_of_heap_reserve.try_into()?,
            size_of_heap_commit: value.size_of_heap_commit.try_into()?,
            loader_flags: value.loader_flags,
            number_of_rva_and_sizes: value.number_of_rva_and_sizes,
        })
    }
}

// impl From<WindowsFields32> for WindowsFields {
//     fn from(windows: WindowsFields32) -> Self {
//         WindowsFields {
//             image_base: windows.image_base,
//             section_alignment: windows.section_alignment,
//             file_alignment: windows.file_alignment,
//             major_operating_system_version: windows.major_operating_system_version,
//             minor_operating_system_version: windows.minor_operating_system_version,
//             major_image_version: windows.major_image_version,
//             minor_image_version: windows.minor_image_version,
//             major_subsystem_version: windows.major_subsystem_version,
//             minor_subsystem_version: windows.minor_subsystem_version,
//             win32_version_value: windows.win32_version_value,
//             size_of_image: windows.size_of_image,
//             size_of_headers: windows.size_of_headers,
//             check_sum: windows.check_sum,
//             subsystem: windows.subsystem,
//             dll_characteristics: windows.dll_characteristics,
//             size_of_stack_reserve: windows.size_of_stack_reserve,
//             size_of_stack_commit: windows.size_of_stack_commit,
//             size_of_heap_reserve: windows.size_of_heap_reserve,
//             size_of_heap_commit: windows.size_of_heap_commit,
//             loader_flags: windows.loader_flags,
//             number_of_rva_and_sizes: windows.number_of_rva_and_sizes,
//         }
//     }
// }

/// Unified 32/64-bit Windows fields (for `PE32` and `PE32+`). Since 64-bit fields are a superset of 32-bit fields,
/// `WindowsFields` is an alias for `WindowsFields64`.
//
// Opinion (JohnScience): even though they're structurally equivalent, it was a questionable idea to make
// them nominally equivalent as well because they're not actually the same thing semantically. WindowsFields is meant to be
// a unified type that can represent either 32-bit or 64-bit Windows fields.
//
// How do you document this effectively and forward-compatibly? `WindowsFields64` and `WindowsFields` need
// different documentation.
pub type WindowsFields = WindowsFields64;

/// Unified 32/64-bit optional header (for `PE32` and `PE32+`).
///
/// Optional header is the most important of the [NT headers](https://offwhitesecurity.dev/malware-development/portable-executable-pe/nt-headers/).
/// Although it's called "optional", it's actually required for PE image files.
///
/// It is meant to represent either
///
/// * [`IMAGE_OPTIONAL_HEADER32`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header32); or
/// * [`IMAGE_OPTIONAL_HEADER64`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_optional_header64).
///
/// Whether it's 32 or 64-bit is determined by the [`StandardFields::magic`] and by the value
/// [`CoffHeader::size_of_optional_header`](crate::pe::header::CoffHeader::size_of_optional_header).
///
/// ## Position in PE binary
///
/// The optional header is located after [`CoffHeader`](crate::pe::header::CoffHeader) and before
/// section table.
#[derive(Debug, PartialEq, Copy, Clone)]
#[doc(alias = "IMAGE_OPTIONAL_HEADER32")]
#[doc(alias = "IMAGE_OPTIONAL_HEADER64")]
pub struct OptionalHeader {
    /// Unified standard (COFF) fields. See [`StandardFields`] to learn more.
    pub standard_fields: StandardFields,
    /// Unified Windows fields. See [`WindowsFields`] to learn more.
    pub windows_fields: WindowsFields,
    /// Data directories. See [`DataDirectories`](data_directories::DataDirectories) to learn more.
    pub data_directories: data_directories::DataDirectories,
}

/// Magic number for 32-bit binary (`PE32`).
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x10b;
/// Magic number for 64-bit binary (`PE32+`).
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x20b;
/// Magic number for a ROM image.
///
/// More info: <https://superuser.com/questions/156994/what-sort-of-program-has-its-pe-executable-header-set-to-rom-image>.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 0x107;

impl OptionalHeader {
    /// Returns the container type of the PE binary.
    pub fn container(&self) -> error::Result<container::Container> {
        match self.standard_fields.magic {
            MAGIC_32 => Ok(container::Container::Little),
            MAGIC_64 => Ok(container::Container::Big),
            magic => Err(error::Error::BadMagic(u64::from(magic))),
        }
    }
}

impl<'a> ctx::TryFromCtx<'a, Endian> for OptionalHeader {
    type Error = crate::error::Error;
    fn try_from_ctx(bytes: &'a [u8], _: Endian) -> error::Result<(Self, usize)> {
        let magic = bytes.pread_with::<u16>(0, LE)?;
        let offset = &mut 0;
        let (standard_fields, windows_fields): (StandardFields, WindowsFields) = match magic {
            MAGIC_32 => {
                let standard_fields = bytes.gread_with::<StandardFields32>(offset, LE)?.into();
                let windows_fields = bytes.gread_with::<WindowsFields32>(offset, LE)?.into();
                (standard_fields, windows_fields)
            }
            MAGIC_64 => {
                let standard_fields = bytes.gread_with::<StandardFields64>(offset, LE)?.into();
                let windows_fields = bytes.gread_with::<WindowsFields64>(offset, LE)?;
                (standard_fields, windows_fields)
            }
            _ => return Err(error::Error::BadMagic(u64::from(magic))),
        };
        let data_directories = data_directories::DataDirectories::parse(
            &bytes,
            windows_fields.number_of_rva_and_sizes as usize,
            offset,
        )?;
        Ok((
            OptionalHeader {
                standard_fields,
                windows_fields,
                data_directories,
            },
            0,
        )) // TODO: FIXME
    }
}

impl ctx::TryIntoCtx<scroll::Endian> for OptionalHeader {
    type Error = error::Error;

    fn try_into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        match self.standard_fields.magic {
            MAGIC_32 => {
                bytes.gwrite_with::<StandardFields32>(self.standard_fields.into(), offset, ctx)?;
                bytes.gwrite_with(WindowsFields32::try_from(self.windows_fields)?, offset, ctx)?;
                bytes.gwrite_with(self.data_directories, offset, ctx)?;
            }
            MAGIC_64 => {
                bytes.gwrite_with::<StandardFields64>(self.standard_fields.into(), offset, ctx)?;
                bytes.gwrite_with(self.windows_fields, offset, ctx)?;
                bytes.gwrite_with(self.data_directories, offset, ctx)?;
            }
            _ => panic!(),
        }
        Ok(*offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sizeof_standards32() {
        assert_eq!(
            ::std::mem::size_of::<StandardFields32>(),
            SIZEOF_STANDARD_FIELDS_32
        );
    }
    #[test]
    fn sizeof_windows32() {
        assert_eq!(
            ::std::mem::size_of::<WindowsFields32>(),
            SIZEOF_WINDOWS_FIELDS_32
        );
    }
    #[test]
    fn sizeof_standards64() {
        assert_eq!(
            ::std::mem::size_of::<StandardFields64>(),
            SIZEOF_STANDARD_FIELDS_64
        );
    }
    #[test]
    fn sizeof_windows64() {
        assert_eq!(
            ::std::mem::size_of::<WindowsFields64>(),
            SIZEOF_WINDOWS_FIELDS_64
        );
    }
}
