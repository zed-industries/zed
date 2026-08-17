//! Miscellaneous constants used inside of and when constructing, Mach-o binaries
// Convienence constants for return values from dyld_get_sdk_version() and friends.
pub const DYLD_MACOSX_VERSION_10_4: u32 = 0x000A_0400;
pub const DYLD_MACOSX_VERSION_10_5: u32 = 0x000A_0500;
pub const DYLD_MACOSX_VERSION_10_6: u32 = 0x000A_0600;
pub const DYLD_MACOSX_VERSION_10_7: u32 = 0x000A_0700;
pub const DYLD_MACOSX_VERSION_10_8: u32 = 0x000A_0800;
pub const DYLD_MACOSX_VERSION_10_9: u32 = 0x000A_0900;
pub const DYLD_MACOSX_VERSION_10_10: u32 = 0x000A_0A00;
pub const DYLD_MACOSX_VERSION_10_11: u32 = 0x000A_0B00;
pub const DYLD_MACOSX_VERSION_10_12: u32 = 0x000A_0C00;
pub const DYLD_MACOSX_VERSION_10_13: u32 = 0x000A_0D00;

pub const DYLD_IOS_VERSION_2_0: u32 = 0x0002_0000;
pub const DYLD_IOS_VERSION_2_1: u32 = 0x0002_0100;
pub const DYLD_IOS_VERSION_2_2: u32 = 0x0002_0200;
pub const DYLD_IOS_VERSION_3_0: u32 = 0x0003_0000;
pub const DYLD_IOS_VERSION_3_1: u32 = 0x0003_0100;
pub const DYLD_IOS_VERSION_3_2: u32 = 0x0003_0200;
pub const DYLD_IOS_VERSION_4_0: u32 = 0x0004_0000;
pub const DYLD_IOS_VERSION_4_1: u32 = 0x0004_0100;
pub const DYLD_IOS_VERSION_4_2: u32 = 0x0004_0200;
pub const DYLD_IOS_VERSION_4_3: u32 = 0x0004_0300;
pub const DYLD_IOS_VERSION_5_0: u32 = 0x0005_0000;
pub const DYLD_IOS_VERSION_5_1: u32 = 0x0005_0100;
pub const DYLD_IOS_VERSION_6_0: u32 = 0x0006_0000;
pub const DYLD_IOS_VERSION_6_1: u32 = 0x0006_0100;
pub const DYLD_IOS_VERSION_7_0: u32 = 0x0007_0000;
pub const DYLD_IOS_VERSION_7_1: u32 = 0x0007_0100;
pub const DYLD_IOS_VERSION_8_0: u32 = 0x0008_0000;
pub const DYLD_IOS_VERSION_9_0: u32 = 0x0009_0000;
pub const DYLD_IOS_VERSION_10_0: u32 = 0x000A_0000;
pub const DYLD_IOS_VERSION_11_0: u32 = 0x000B_0000;

// Segment and Section Constants

// The flags field of a section structure is separated into two parts a section
// type and section attributes.  The section types are mutually exclusive (it
// can only have one type) but the section attributes are not (it may have more
// than one attribute).
/// 256 section types
pub const SECTION_TYPE: u32 = 0x0000_00ff;
///  24 section attributes
pub const SECTION_ATTRIBUTES: u32 = 0xffff_ff00;

// Constants for the type of a section
/// regular section
pub const S_REGULAR: u32 = 0x0;
/// zero fill on demand section
pub const S_ZEROFILL: u32 = 0x1;
/// section with only literal C strings
pub const S_CSTRING_LITERALS: u32 = 0x2;
/// section with only 4 byte literals
pub const S_4BYTE_LITERALS: u32 = 0x3;
/// section with only 8 byte literals
pub const S_8BYTE_LITERALS: u32 = 0x4;
/// section with only pointers to
pub const S_LITERAL_POINTERS: u32 = 0x5;

// literals
// For the two types of symbol pointers sections and the symbol stubs section
// they have indirect symbol table entries.  For each of the entries in the
// section the indirect symbol table entries, in corresponding order in the
// indirect symbol table, start at the index stored in the reserved1 field
// of the section structure.  Since the indirect symbol table entries
// correspond to the entries in the section the number of indirect symbol table
// entries is inferred from the size of the section divided by the size of the
// entries in the section.  For symbol pointers sections the size of the entries
// in the section is 4 bytes and for symbol stubs sections the byte size of the
// stubs is stored in the reserved2 field of the section structure.
/// section with only non-lazy symbol pointers
pub const S_NON_LAZY_SYMBOL_POINTERS: u32 = 0x6;
/// section with only lazy symbol pointers
pub const S_LAZY_SYMBOL_POINTERS: u32 = 0x7;
/// section with only symbol stubs, byte size of stub in the reserved2 field
pub const S_SYMBOL_STUBS: u32 = 0x8;
/// section with only function pointers for initialization
pub const S_MOD_INIT_FUNC_POINTERS: u32 = 0x9;
/// section with only function pointers for termination
pub const S_MOD_TERM_FUNC_POINTERS: u32 = 0xa;
/// section contains symbols that are to be coalesced
pub const S_COALESCED: u32 = 0xb;
/// zero fill on demand section that can be larger than 4 gigabytes)
pub const S_GB_ZEROFILL: u32 = 0xc;
/// section with only pairs of function pointers for interposing
pub const S_INTERPOSING: u32 = 0xd;
/// section with only 16 byte literals
pub const S_16BYTE_LITERALS: u32 = 0xe;
/// section contains DTrace Object Format
pub const S_DTRACE_DOF: u32 = 0xf;
/// section with only lazy symbol pointers to lazy loaded dylibs
pub const S_LAZY_DYLIB_SYMBOL_POINTERS: u32 = 0x10;

// Section types to support thread local variables
/// template of initial  values for TLVs
pub const S_THREAD_LOCAL_REGULAR: u32 = 0x11;
/// template of initial  values for TLVs
pub const S_THREAD_LOCAL_ZEROFILL: u32 = 0x12;
/// TLV descriptors
pub const S_THREAD_LOCAL_VARIABLES: u32 = 0x13;
/// pointers to TLV  descriptors
pub const S_THREAD_LOCAL_VARIABLE_POINTERS: u32 = 0x14;
/// functions to call to initialize TLV values
pub const S_THREAD_LOCAL_INIT_FUNCTION_POINTERS: u32 = 0x15;

// Constants for the section attributes part of the flags field of a section
// structure.
/// User setable attributes
pub const SECTION_ATTRIBUTES_USR: u32 = 0xff00_0000;
/// section contains only true machine instructions
pub const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
/// section contains coalesced symbols that are not to be in a ranlib table of contents
pub const S_ATTR_NO_TOC: u32 = 0x4000_0000;
/// ok to strip static symbols in this section in files with the MH_DYLDLINK flag
pub const S_ATTR_STRIP_STATIC_SYMS: u32 = 0x2000_0000;
/// no dead stripping
pub const S_ATTR_NO_DEAD_STRIP: u32 = 0x1000_0000;
/// blocks are live if they reference live blocks
pub const S_ATTR_LIVE_SUPPORT: u32 = 0x0800_0000;
/// Used with i386 code stubs written on by dyld
pub const S_ATTR_SELF_MODIFYING_CODE: u32 = 0x0400_0000;

// If a segment contains any sections marked with S_ATTR_DEBUG then all
// sections in that segment must have this attribute.  No section other than
// a section marked with this attribute may reference the contents of this
// section.  A section with this attribute may contain no symbols and must have
// a section type S_REGULAR.  The static linker will not copy section contents
// from sections with this attribute into its output file.  These sections
// generally contain DWARF debugging info.
/// debug section
pub const S_ATTR_DEBUG: u32 = 0x0200_0000;
/// system setable attributes
pub const SECTION_ATTRIBUTES_SYS: u32 = 0x00ff_ff00;
/// section contains some machine instructions
pub const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x0000_0400;
/// section has external relocation entries
pub const S_ATTR_EXT_RELOC: u32 = 0x0000_0200;
/// section has local relocation entries
pub const S_ATTR_LOC_RELOC: u32 = 0x0000_0100;

// The names of segments and sections in them are mostly meaningless to the
// link-editor.  But there are few things to support traditional UNIX
// executables that require the link-editor and assembler to use some names
// agreed upon by convention.
// The initial protection of the "__TEXT" segment has write protection turned
// off (not writeable).
// The link-editor will allocate common symbols at the end of the "__common"
// section in the "__DATA" segment.  It will create the section and segment
// if needed.

// The currently known segment names and the section names in those segments
/// the pagezero segment which has no protections and catches NULL references for MH_EXECUTE files
pub const SEG_PAGEZERO: &str = "__PAGEZERO";
/// the tradition UNIX text segment
pub const SEG_TEXT: &str = "__TEXT";
/// the real text part of the text section no headers, and no padding
pub const SECT_TEXT: &str = "__text";
/// the fvmlib initialization section
pub const SECT_FVMLIB_INIT0: &str = "__fvmlib_init0";
/// the section following the fvmlib initialization section
pub const SECT_FVMLIB_INIT1: &str = "__fvmlib_init1";
/// the tradition UNIX data segment
pub const SEG_DATA: &str = "__DATA";
/// the real initialized data section no padding, no bss overlap
pub const SECT_DATA: &str = "__data";
/// the real uninitialized data sectionno padding
pub const SECT_BSS: &str = "__bss";
/// the section common symbols are allocated in by the link editor
pub const SECT_COMMON: &str = "__common";
/// objective-C runtime segment
pub const SEG_OBJC: &str = "__OBJC";
/// symbol table
pub const SECT_OBJC_SYMBOLS: &str = "__symbol_table";
/// module information
pub const SECT_OBJC_MODULES: &str = "__module_info";
/// string table
pub const SECT_OBJC_STRINGS: &str = "__selector_strs";
/// string table
pub const SECT_OBJC_REFS: &str = "__selector_refs";
/// the icon segment
pub const SEG_ICON: &str = "__ICON";
/// the icon headers
pub const SECT_ICON_HEADER: &str = "__header";
/// the icons in tiff format
pub const SECT_ICON_TIFF: &str = "__tiff";
/// the segment containing all structs created and maintained by the link editor.  Created with -seglinkedit option to ld(1) for MH_EXECUTE and FVMLIB file types only
pub const SEG_LINKEDIT: &str = "__LINKEDIT";
/// the unix stack segment
pub const SEG_UNIXSTACK: &str = "__UNIXSTACK";
/// the segment for the self (dyld) modifing code stubs that has read, write and execute permissions
pub const SEG_IMPORT: &str = "__IMPORT";

/// Segment is readable.
pub const VM_PROT_READ: u32 = 0x1;
/// Segment is writable.
pub const VM_PROT_WRITE: u32 = 0x2;
/// Segment is executable.
pub const VM_PROT_EXECUTE: u32 = 0x4;

pub mod cputype {

    /// An alias for u32
    pub type CpuType = u32;
    /// An alias for u32
    pub type CpuSubType = u32;

    /// the mask for CPU feature flags
    pub const CPU_SUBTYPE_MASK: u32 = 0xff00_0000;
    /// mask for architecture bits
    pub const CPU_ARCH_MASK: CpuType = 0xff00_0000;
    /// the mask for 64 bit ABI
    pub const CPU_ARCH_ABI64: CpuType = 0x0100_0000;
    /// the mask for ILP32 ABI on 64 bit hardware
    pub const CPU_ARCH_ABI64_32: CpuType = 0x0200_0000;

    // CPU Types
    pub const CPU_TYPE_ANY: CpuType = !0;
    pub const CPU_TYPE_VAX: CpuType = 1;
    pub const CPU_TYPE_MC680X0: CpuType = 6;
    pub const CPU_TYPE_X86: CpuType = 7;
    pub const CPU_TYPE_I386: CpuType = CPU_TYPE_X86;
    pub const CPU_TYPE_X86_64: CpuType = CPU_TYPE_X86 | CPU_ARCH_ABI64;
    pub const CPU_TYPE_MIPS: CpuType = 8;
    pub const CPU_TYPE_MC98000: CpuType = 10;
    pub const CPU_TYPE_HPPA: CpuType = 11;
    pub const CPU_TYPE_ARM: CpuType = 12;
    pub const CPU_TYPE_ARM64: CpuType = CPU_TYPE_ARM | CPU_ARCH_ABI64;
    pub const CPU_TYPE_ARM64_32: CpuType = CPU_TYPE_ARM | CPU_ARCH_ABI64_32;
    pub const CPU_TYPE_MC88000: CpuType = 13;
    pub const CPU_TYPE_SPARC: CpuType = 14;
    pub const CPU_TYPE_I860: CpuType = 15;
    pub const CPU_TYPE_ALPHA: CpuType = 16;
    pub const CPU_TYPE_POWERPC: CpuType = 18;
    pub const CPU_TYPE_POWERPC64: CpuType = CPU_TYPE_POWERPC | CPU_ARCH_ABI64;

    // CPU Subtypes
    pub const CPU_SUBTYPE_MULTIPLE: CpuSubType = !0;
    pub const CPU_SUBTYPE_LITTLE_ENDIAN: CpuSubType = 0;
    pub const CPU_SUBTYPE_BIG_ENDIAN: CpuSubType = 1;
    pub const CPU_SUBTYPE_VAX_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_VAX780: CpuSubType = 1;
    pub const CPU_SUBTYPE_VAX785: CpuSubType = 2;
    pub const CPU_SUBTYPE_VAX750: CpuSubType = 3;
    pub const CPU_SUBTYPE_VAX730: CpuSubType = 4;
    pub const CPU_SUBTYPE_UVAXI: CpuSubType = 5;
    pub const CPU_SUBTYPE_UVAXII: CpuSubType = 6;
    pub const CPU_SUBTYPE_VAX8200: CpuSubType = 7;
    pub const CPU_SUBTYPE_VAX8500: CpuSubType = 8;
    pub const CPU_SUBTYPE_VAX8600: CpuSubType = 9;
    pub const CPU_SUBTYPE_VAX8650: CpuSubType = 10;
    pub const CPU_SUBTYPE_VAX8800: CpuSubType = 11;
    pub const CPU_SUBTYPE_UVAXIII: CpuSubType = 12;
    pub const CPU_SUBTYPE_MC680X0_ALL: CpuSubType = 1;
    pub const CPU_SUBTYPE_MC68030: CpuSubType = 1; /* compat */
    pub const CPU_SUBTYPE_MC68040: CpuSubType = 2;
    pub const CPU_SUBTYPE_MC68030_ONLY: CpuSubType = 3;

    macro_rules! CPU_SUBTYPE_INTEL {
        ($f:expr, $m:expr) => {{
            ($f) + (($m) << 4)
        }};
    }

    pub const CPU_SUBTYPE_I386_ALL: CpuSubType = CPU_SUBTYPE_INTEL!(3, 0);
    pub const CPU_SUBTYPE_386: CpuSubType = CPU_SUBTYPE_INTEL!(3, 0);
    pub const CPU_SUBTYPE_486: CpuSubType = CPU_SUBTYPE_INTEL!(4, 0);
    pub const CPU_SUBTYPE_486SX: CpuSubType = CPU_SUBTYPE_INTEL!(4, 8); // 8 << 4 = 128
    pub const CPU_SUBTYPE_586: CpuSubType = CPU_SUBTYPE_INTEL!(5, 0);
    pub const CPU_SUBTYPE_PENT: CpuSubType = CPU_SUBTYPE_INTEL!(5, 0);
    pub const CPU_SUBTYPE_PENTPRO: CpuSubType = CPU_SUBTYPE_INTEL!(6, 1);
    pub const CPU_SUBTYPE_PENTII_M3: CpuSubType = CPU_SUBTYPE_INTEL!(6, 3);
    pub const CPU_SUBTYPE_PENTII_M5: CpuSubType = CPU_SUBTYPE_INTEL!(6, 5);
    pub const CPU_SUBTYPE_CELERON: CpuSubType = CPU_SUBTYPE_INTEL!(7, 6);
    pub const CPU_SUBTYPE_CELERON_MOBILE: CpuSubType = CPU_SUBTYPE_INTEL!(7, 7);
    pub const CPU_SUBTYPE_PENTIUM_3: CpuSubType = CPU_SUBTYPE_INTEL!(8, 0);
    pub const CPU_SUBTYPE_PENTIUM_3_M: CpuSubType = CPU_SUBTYPE_INTEL!(8, 1);
    pub const CPU_SUBTYPE_PENTIUM_3_XEON: CpuSubType = CPU_SUBTYPE_INTEL!(8, 2);
    pub const CPU_SUBTYPE_PENTIUM_M: CpuSubType = CPU_SUBTYPE_INTEL!(9, 0);
    pub const CPU_SUBTYPE_PENTIUM_4: CpuSubType = CPU_SUBTYPE_INTEL!(10, 0);
    pub const CPU_SUBTYPE_PENTIUM_4_M: CpuSubType = CPU_SUBTYPE_INTEL!(10, 1);
    pub const CPU_SUBTYPE_ITANIUM: CpuSubType = CPU_SUBTYPE_INTEL!(11, 0);
    pub const CPU_SUBTYPE_ITANIUM_2: CpuSubType = CPU_SUBTYPE_INTEL!(11, 1);
    pub const CPU_SUBTYPE_XEON: CpuSubType = CPU_SUBTYPE_INTEL!(12, 0);
    pub const CPU_SUBTYPE_XEON_MP: CpuSubType = CPU_SUBTYPE_INTEL!(12, 1);
    pub const CPU_SUBTYPE_INTEL_FAMILY_MAX: CpuSubType = 15;
    pub const CPU_SUBTYPE_INTEL_MODEL_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_X86_ALL: CpuSubType = 3;
    pub const CPU_SUBTYPE_X86_64_ALL: CpuSubType = 3;
    pub const CPU_SUBTYPE_X86_ARCH1: CpuSubType = 4;
    pub const CPU_SUBTYPE_X86_64_H: CpuSubType = 8;
    pub const CPU_SUBTYPE_MIPS_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_MIPS_R2300: CpuSubType = 1;
    pub const CPU_SUBTYPE_MIPS_R2600: CpuSubType = 2;
    pub const CPU_SUBTYPE_MIPS_R2800: CpuSubType = 3;
    pub const CPU_SUBTYPE_MIPS_R2000A: CpuSubType = 4;
    pub const CPU_SUBTYPE_MIPS_R2000: CpuSubType = 5;
    pub const CPU_SUBTYPE_MIPS_R3000A: CpuSubType = 6;
    pub const CPU_SUBTYPE_MIPS_R3000: CpuSubType = 7;
    pub const CPU_SUBTYPE_MC98000_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_MC98601: CpuSubType = 1;
    pub const CPU_SUBTYPE_HPPA_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_HPPA_7100: CpuSubType = 0;
    pub const CPU_SUBTYPE_HPPA_7100LC: CpuSubType = 1;
    pub const CPU_SUBTYPE_MC88000_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_MC88100: CpuSubType = 1;
    pub const CPU_SUBTYPE_MC88110: CpuSubType = 2;
    pub const CPU_SUBTYPE_SPARC_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_I860_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_I860_860: CpuSubType = 1;
    pub const CPU_SUBTYPE_POWERPC_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_POWERPC_601: CpuSubType = 1;
    pub const CPU_SUBTYPE_POWERPC_602: CpuSubType = 2;
    pub const CPU_SUBTYPE_POWERPC_603: CpuSubType = 3;
    pub const CPU_SUBTYPE_POWERPC_603E: CpuSubType = 4;
    pub const CPU_SUBTYPE_POWERPC_603EV: CpuSubType = 5;
    pub const CPU_SUBTYPE_POWERPC_604: CpuSubType = 6;
    pub const CPU_SUBTYPE_POWERPC_604E: CpuSubType = 7;
    pub const CPU_SUBTYPE_POWERPC_620: CpuSubType = 8;
    pub const CPU_SUBTYPE_POWERPC_750: CpuSubType = 9;
    pub const CPU_SUBTYPE_POWERPC_7400: CpuSubType = 10;
    pub const CPU_SUBTYPE_POWERPC_7450: CpuSubType = 11;
    pub const CPU_SUBTYPE_POWERPC_970: CpuSubType = 100;
    pub const CPU_SUBTYPE_ARM_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_ARM_V4T: CpuSubType = 5;
    pub const CPU_SUBTYPE_ARM_V6: CpuSubType = 6;
    pub const CPU_SUBTYPE_ARM_V5TEJ: CpuSubType = 7;
    pub const CPU_SUBTYPE_ARM_XSCALE: CpuSubType = 8;
    pub const CPU_SUBTYPE_ARM_V7: CpuSubType = 9;
    pub const CPU_SUBTYPE_ARM_V7F: CpuSubType = 10;
    pub const CPU_SUBTYPE_ARM_V7S: CpuSubType = 11;
    pub const CPU_SUBTYPE_ARM_V7K: CpuSubType = 12;
    pub const CPU_SUBTYPE_ARM_V6M: CpuSubType = 14;
    pub const CPU_SUBTYPE_ARM_V7M: CpuSubType = 15;
    pub const CPU_SUBTYPE_ARM_V7EM: CpuSubType = 16;
    pub const CPU_SUBTYPE_ARM_V8: CpuSubType = 13;
    pub const CPU_SUBTYPE_ARM64_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_ARM64_V8: CpuSubType = 1;
    pub const CPU_SUBTYPE_ARM64_E: CpuSubType = 2;
    pub const CPU_SUBTYPE_ARM64_32_ALL: CpuSubType = 0;
    pub const CPU_SUBTYPE_ARM64_32_V8: CpuSubType = 1;

    macro_rules! cpu_flag_mapping {
        (
            $(($name:expr, $cputype:ident, $cpusubtype:ident),)*
        ) => {
            fn get_arch_from_flag_no_alias(name: &str) -> Option<(CpuType, CpuSubType)> {
                match name {
                    $($name => Some(($cputype, $cpusubtype)),)*
                    _ => None
                }
            }

            /// Get the architecture name from cputype and cpusubtype
            ///
            /// When using this method to determine the architecture
            /// name of an instance of
            /// [`goblin::mach::header::Header`](/goblin/mach/header/struct.Header.html),
            /// use the provided method
            /// [`cputype()`](/goblin/mach/header/struct.Header.html#method.cputype) and
            /// [`cpusubtype()`](/goblin/mach/header/struct.Header.html#method.cpusubtype)
            /// instead of corresponding field `cputype` and `cpusubtype`.
            ///
            /// For example:
            ///
            /// ```rust
            /// use std::fs::read;
            /// use goblin::mach::constants::cputype::get_arch_name_from_types;
            /// use goblin::mach::Mach;
            ///
            /// read("path/to/macho").and_then(|buf| {
            ///     if let Ok(Mach::Binary(a)) = Mach::parse(&buf) {
            ///         println!("arch name: {}", get_arch_name_from_types(a.header.cputype(), a.header.cpusubtype()).unwrap());
            ///     }
            ///     Ok(())
            /// });
            /// ```
            pub fn get_arch_name_from_types(cputype: CpuType, cpusubtype: CpuSubType)
                -> Option<&'static str> {
                match (cputype, cpusubtype) {
                    $(($cputype, $cpusubtype) => Some($name),)*
                    (_, _) => None
                }
            }
        }
    }

    /// Get the cputype and cpusubtype from a name
    pub fn get_arch_from_flag(name: &str) -> Option<(CpuType, CpuSubType)> {
        get_arch_from_flag_no_alias(name).or_else(|| {
            // we also handle some common aliases
            match name {
                // these are used by apple
                "pentium" => Some((CPU_TYPE_I386, CPU_SUBTYPE_PENT)),
                "pentpro" => Some((CPU_TYPE_I386, CPU_SUBTYPE_PENTPRO)),
                // these are used commonly for consistency
                "x86" => Some((CPU_TYPE_I386, CPU_SUBTYPE_I386_ALL)),
                _ => None,
            }
        })
    }

    cpu_flag_mapping! {
        // generic types
        ("any", CPU_TYPE_ANY, CPU_SUBTYPE_MULTIPLE),
        ("little", CPU_TYPE_ANY, CPU_SUBTYPE_LITTLE_ENDIAN),
        ("big", CPU_TYPE_ANY, CPU_SUBTYPE_BIG_ENDIAN),

        // macho names
        ("ppc64", CPU_TYPE_POWERPC64, CPU_SUBTYPE_POWERPC_ALL),
        ("x86_64", CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL),
        ("x86_64h", CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_H),
        ("arm64", CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_ALL),
        ("arm64_32", CPU_TYPE_ARM64_32, CPU_SUBTYPE_ARM64_32_ALL),
        ("ppc970-64", CPU_TYPE_POWERPC64, CPU_SUBTYPE_POWERPC_970),
        ("ppc", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_ALL),
        ("i386", CPU_TYPE_I386, CPU_SUBTYPE_I386_ALL),
        ("m68k", CPU_TYPE_MC680X0, CPU_SUBTYPE_MC680X0_ALL),
        ("hppa", CPU_TYPE_HPPA, CPU_SUBTYPE_HPPA_ALL),
        ("sparc", CPU_TYPE_SPARC, CPU_SUBTYPE_SPARC_ALL),
        ("m88k", CPU_TYPE_MC88000, CPU_SUBTYPE_MC88000_ALL),
        ("i860", CPU_TYPE_I860, CPU_SUBTYPE_I860_ALL),
        ("arm", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_ALL),
        ("ppc601", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_601),
        ("ppc603", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_603),
        ("ppc603e", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_603E),
        ("ppc603ev", CPU_TYPE_POWERPC,CPU_SUBTYPE_POWERPC_603EV),
        ("ppc604", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_604),
        ("ppc604e",CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_604E),
        ("ppc750", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_750),
        ("ppc7400", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_7400),
        ("ppc7450", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_7450),
        ("ppc970", CPU_TYPE_POWERPC, CPU_SUBTYPE_POWERPC_970),
        ("i486", CPU_TYPE_I386, CPU_SUBTYPE_486),
        ("i486SX", CPU_TYPE_I386, CPU_SUBTYPE_486SX),
        ("i586", CPU_TYPE_I386, CPU_SUBTYPE_586),
        ("i686", CPU_TYPE_I386, CPU_SUBTYPE_PENTPRO),
        ("pentIIm3", CPU_TYPE_I386, CPU_SUBTYPE_PENTII_M3),
        ("pentIIm5", CPU_TYPE_I386, CPU_SUBTYPE_PENTII_M5),
        ("pentium4", CPU_TYPE_I386, CPU_SUBTYPE_PENTIUM_4),
        ("m68030", CPU_TYPE_MC680X0, CPU_SUBTYPE_MC68030_ONLY),
        ("m68040", CPU_TYPE_MC680X0, CPU_SUBTYPE_MC68040),
        ("hppa7100LC", CPU_TYPE_HPPA,  CPU_SUBTYPE_HPPA_7100LC),
        ("armv4t", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V4T),
        ("armv5", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V5TEJ),
        ("xscale", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_XSCALE),
        ("armv6", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V6),
        ("armv6m", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V6M),
        ("armv7", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7),
        ("armv7f", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7F),
        ("armv7s", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7S),
        ("armv7k", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7K),
        ("armv7m", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7M),
        ("armv7em", CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7EM),
        ("arm64v8", CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_V8),
        ("arm64e", CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_E),
        ("arm64_32_v8", CPU_TYPE_ARM64_32, CPU_SUBTYPE_ARM64_32_V8),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_mapping() {
        use super::cputype::*;

        assert_eq!(
            get_arch_from_flag("armv7"),
            Some((CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7))
        );
        assert_eq!(
            get_arch_name_from_types(CPU_TYPE_ARM, CPU_SUBTYPE_ARM_V7),
            Some("armv7")
        );
        assert_eq!(
            get_arch_from_flag("i386"),
            Some((CPU_TYPE_I386, CPU_SUBTYPE_I386_ALL))
        );
        assert_eq!(
            get_arch_from_flag("x86"),
            Some((CPU_TYPE_I386, CPU_SUBTYPE_I386_ALL))
        );
    }
}
