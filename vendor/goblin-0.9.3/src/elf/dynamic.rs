macro_rules! elf_dyn {
    ($size:ty) => {
        // XXX: Do not import scroll traits here.
        // See: https://github.com/rust-lang/rust/issues/65090#issuecomment-538668155

        #[repr(C)]
        #[derive(Copy, Clone, PartialEq, Default)]
        #[cfg_attr(
            feature = "alloc",
            derive(scroll::Pread, scroll::Pwrite, scroll::SizeWith)
        )]
        /// An entry in the dynamic array
        pub struct Dyn {
            /// Dynamic entry type
            pub d_tag: $size,
            /// Integer value
            pub d_val: $size,
        }

        use plain;
        unsafe impl plain::Plain for Dyn {}
    };
}

// TODO: figure out what's the best, most friendly + safe API choice here - u32s or u64s
// remember that DT_TAG is "pointer sized"/used as address sometimes Original rationale: I
// decided to use u64 instead of u32 due to pattern matching use case seems safer to cast the
// elf32's d_tag from u32 -> u64 at runtime instead of casting the elf64's d_tag from u64 ->
// u32 at runtime

/// Marks end of dynamic section
pub const DT_NULL: u64 = 0;
/// Name of needed library
pub const DT_NEEDED: u64 = 1;
/// Size in bytes of PLT relocs
pub const DT_PLTRELSZ: u64 = 2;
/// Processor defined value
pub const DT_PLTGOT: u64 = 3;
/// Address of symbol hash table
pub const DT_HASH: u64 = 4;
/// Address of string table
pub const DT_STRTAB: u64 = 5;
/// Address of symbol table
pub const DT_SYMTAB: u64 = 6;
/// Address of Rela relocs
pub const DT_RELA: u64 = 7;
/// Total size of Rela relocs
pub const DT_RELASZ: u64 = 8;
/// Size of one Rela reloc
pub const DT_RELAENT: u64 = 9;
/// Size of string table
pub const DT_STRSZ: u64 = 10;
/// Size of one symbol table entry
pub const DT_SYMENT: u64 = 11;
/// Address of init function
pub const DT_INIT: u64 = 12;
/// Address of termination function
pub const DT_FINI: u64 = 13;
/// Name of shared object
pub const DT_SONAME: u64 = 14;
/// Library search path (deprecated)
pub const DT_RPATH: u64 = 15;
/// Start symbol search here
pub const DT_SYMBOLIC: u64 = 16;
/// Address of Rel relocs
pub const DT_REL: u64 = 17;
/// Total size of Rel relocs
pub const DT_RELSZ: u64 = 18;
/// Size of one Rel reloc
pub const DT_RELENT: u64 = 19;
/// Type of reloc in PLT
pub const DT_PLTREL: u64 = 20;
/// For debugging; unspecified
pub const DT_DEBUG: u64 = 21;
/// Reloc might modify .text
pub const DT_TEXTREL: u64 = 22;
/// Address of PLT relocs
pub const DT_JMPREL: u64 = 23;
/// Process relocations of object
pub const DT_BIND_NOW: u64 = 24;
/// Array with addresses of init fct
pub const DT_INIT_ARRAY: u64 = 25;
/// Array with addresses of fini fct
pub const DT_FINI_ARRAY: u64 = 26;
/// Size in bytes of DT_INIT_ARRAY
pub const DT_INIT_ARRAYSZ: u64 = 27;
/// Size in bytes of DT_FINI_ARRAY
pub const DT_FINI_ARRAYSZ: u64 = 28;
/// Library search path
pub const DT_RUNPATH: u64 = 29;
/// Flags for the object being loaded
pub const DT_FLAGS: u64 = 30;
/// Start of encoded range
pub const DT_ENCODING: u64 = 32;
/// Array with addresses of preinit fct
pub const DT_PREINIT_ARRAY: u64 = 32;
/// size in bytes of DT_PREINIT_ARRAY
pub const DT_PREINIT_ARRAYSZ: u64 = 33;
/// Number used
pub const DT_NUM: u64 = 34;
/// Start of OS-specific
pub const DT_LOOS: u64 = 0x6000_000d;
/// End of OS-specific
pub const DT_HIOS: u64 = 0x6fff_f000;
/// Start of processor-specific
pub const DT_LOPROC: u64 = 0x7000_0000;
/// End of processor-specific
pub const DT_HIPROC: u64 = 0x7fff_ffff;
// Most used by any processor
// pub const DT_PROCNUM: u64 = DT_MIPS_NUM;

/// DT_* entries which fall between DT_ADDRRNGHI & DT_ADDRRNGLO use the
/// Dyn.d_un.d_ptr field of the Elf*_Dyn structure.
///
/// If any adjustment is made to the ELF object after it has been
/// built these entries will need to be adjusted.
pub const DT_ADDRRNGLO: u64 = 0x6fff_fe00;
/// GNU-style hash table
pub const DT_GNU_HASH: u64 = 0x6fff_fef5;
///
pub const DT_TLSDESC_PLT: u64 = 0x6fff_fef6;
///
pub const DT_TLSDESC_GOT: u64 = 0x6fff_fef7;
/// Start of conflict section
pub const DT_GNU_CONFLICT: u64 = 0x6fff_fef8;
/// Library list
pub const DT_GNU_LIBLIST: u64 = 0x6fff_fef9;
/// Configuration information
pub const DT_CONFIG: u64 = 0x6fff_fefa;
/// Dependency auditing
pub const DT_DEPAUDIT: u64 = 0x6fff_fefb;
/// Object auditing
pub const DT_AUDIT: u64 = 0x6fff_fefc;
/// PLT padding
pub const DT_PLTPAD: u64 = 0x6fff_fefd;
/// Move table
pub const DT_MOVETAB: u64 = 0x6fff_fefe;
/// Syminfo table
pub const DT_SYMINFO: u64 = 0x6fff_feff;
///
pub const DT_ADDRRNGHI: u64 = 0x6fff_feff;

//DT_ADDRTAGIDX(tag)	(DT_ADDRRNGHI - (tag))	/* Reverse order! */
pub const DT_ADDRNUM: u64 = 11;

/// The versioning entry types. The next are defined as part of the GNU extension
pub const DT_VERSYM: u64 = 0x6fff_fff0;
pub const DT_RELACOUNT: u64 = 0x6fff_fff9;
pub const DT_RELCOUNT: u64 = 0x6fff_fffa;
/// State flags, see DF_1_* below
pub const DT_FLAGS_1: u64 = 0x6fff_fffb;
/// Address of version definition table
pub const DT_VERDEF: u64 = 0x6fff_fffc;
/// Number of version definitions
pub const DT_VERDEFNUM: u64 = 0x6fff_fffd;
/// Address of table with needed versions
pub const DT_VERNEED: u64 = 0x6fff_fffe;
/// Number of needed versions
pub const DT_VERNEEDNUM: u64 = 0x6fff_ffff;

/// Converts a tag to its string representation.
#[inline]
pub fn tag_to_str(tag: u64) -> &'static str {
    match tag {
        DT_NULL => "DT_NULL",
        DT_NEEDED => "DT_NEEDED",
        DT_PLTRELSZ => "DT_PLTRELSZ",
        DT_PLTGOT => "DT_PLTGOT",
        DT_HASH => "DT_HASH",
        DT_STRTAB => "DT_STRTAB",
        DT_SYMTAB => "DT_SYMTAB",
        DT_RELA => "DT_RELA",
        DT_RELASZ => "DT_RELASZ",
        DT_RELAENT => "DT_RELAENT",
        DT_STRSZ => "DT_STRSZ",
        DT_SYMENT => "DT_SYMENT",
        DT_INIT => "DT_INIT",
        DT_FINI => "DT_FINI",
        DT_SONAME => "DT_SONAME",
        DT_RPATH => "DT_RPATH",
        DT_SYMBOLIC => "DT_SYMBOLIC",
        DT_REL => "DT_REL",
        DT_RELSZ => "DT_RELSZ",
        DT_RELENT => "DT_RELENT",
        DT_PLTREL => "DT_PLTREL",
        DT_DEBUG => "DT_DEBUG",
        DT_TEXTREL => "DT_TEXTREL",
        DT_JMPREL => "DT_JMPREL",
        DT_BIND_NOW => "DT_BIND_NOW",
        DT_INIT_ARRAY => "DT_INIT_ARRAY",
        DT_FINI_ARRAY => "DT_FINI_ARRAY",
        DT_INIT_ARRAYSZ => "DT_INIT_ARRAYSZ",
        DT_FINI_ARRAYSZ => "DT_FINI_ARRAYSZ",
        DT_RUNPATH => "DT_RUNPATH",
        DT_FLAGS => "DT_FLAGS",
        DT_PREINIT_ARRAY => "DT_PREINIT_ARRAY",
        DT_PREINIT_ARRAYSZ => "DT_PREINIT_ARRAYSZ",
        DT_NUM => "DT_NUM",
        DT_LOOS => "DT_LOOS",
        DT_HIOS => "DT_HIOS",
        DT_LOPROC => "DT_LOPROC",
        DT_HIPROC => "DT_HIPROC",
        DT_VERSYM => "DT_VERSYM",
        DT_RELACOUNT => "DT_RELACOUNT",
        DT_RELCOUNT => "DT_RELCOUNT",
        DT_GNU_HASH => "DT_GNU_HASH",
        DT_VERDEF => "DT_VERDEF",
        DT_VERDEFNUM => "DT_VERDEFNUM",
        DT_VERNEED => "DT_VERNEED",
        DT_VERNEEDNUM => "DT_VERNEEDNUM",
        DT_FLAGS_1 => "DT_FLAGS_1",
        _ => "UNKNOWN_TAG",
    }
}

// Values of `d_un.d_val` in the DT_FLAGS entry
/// Object may use DF_ORIGIN.
pub const DF_ORIGIN: u64 = 0x0000_0001;
/// Symbol resolutions starts here.
pub const DF_SYMBOLIC: u64 = 0x0000_0002;
/// Object contains text relocations.
pub const DF_TEXTREL: u64 = 0x0000_0004;
/// No lazy binding for this object.
pub const DF_BIND_NOW: u64 = 0x0000_0008;
/// Module uses the static TLS model.
pub const DF_STATIC_TLS: u64 = 0x0000_0010;

pub fn df_tag_to_str(tag: u64) -> &'static str {
    match tag {
        DF_ORIGIN => "DF_ORIGIN",
        DF_SYMBOLIC => "DF_SYMBOLIC",
        DF_TEXTREL => "DF_TEXTREL",
        DF_BIND_NOW => "DF_BIND_NOW",
        DF_STATIC_TLS => "DF_STATIC_TLS",
        _ => "UNKNOWN_TAG",
    }
}

/// === State flags ===
/// selectable in the `d_un.d_val` element of the DT_FLAGS_1 entry in the dynamic section.
///
/// Set RTLD_NOW for this object.
pub const DF_1_NOW: u64 = 0x0000_0001;
/// Set RTLD_GLOBAL for this object.
pub const DF_1_GLOBAL: u64 = 0x0000_0002;
/// Set RTLD_GROUP for this object.
pub const DF_1_GROUP: u64 = 0x0000_0004;
/// Set RTLD_NODELETE for this object.
pub const DF_1_NODELETE: u64 = 0x0000_0008;
/// Trigger filtee loading at runtime.
pub const DF_1_LOADFLTR: u64 = 0x0000_0010;
/// Set RTLD_INITFIRST for this object.
pub const DF_1_INITFIRST: u64 = 0x0000_0020;
/// Set RTLD_NOOPEN for this object.
pub const DF_1_NOOPEN: u64 = 0x0000_0040;
/// $ORIGIN must be handled.
pub const DF_1_ORIGIN: u64 = 0x0000_0080;
/// Direct binding enabled.
pub const DF_1_DIRECT: u64 = 0x0000_0100;
pub const DF_1_TRANS: u64 = 0x0000_0200;
/// Object is used to interpose.
pub const DF_1_INTERPOSE: u64 = 0x0000_0400;
/// Ignore default lib search path.
pub const DF_1_NODEFLIB: u64 = 0x0000_0800;
/// Object can't be dldump'ed.
pub const DF_1_NODUMP: u64 = 0x0000_1000;
/// Configuration alternative created.
pub const DF_1_CONFALT: u64 = 0x0000_2000;
/// Filtee terminates filters search.
pub const DF_1_ENDFILTEE: u64 = 0x0000_4000;
/// Disp reloc applied at build time.
pub const DF_1_DISPRELDNE: u64 = 0x0000_8000;
/// Disp reloc applied at run-time.
pub const DF_1_DISPRELPND: u64 = 0x0001_0000;
/// Object has no-direct binding.
pub const DF_1_NODIRECT: u64 = 0x0002_0000;
pub const DF_1_IGNMULDEF: u64 = 0x0004_0000;
pub const DF_1_NOKSYMS: u64 = 0x0008_0000;
pub const DF_1_NOHDR: u64 = 0x0010_0000;
/// Object is modified after built.
pub const DF_1_EDITED: u64 = 0x0020_0000;
pub const DF_1_NORELOC: u64 = 0x0040_0000;
/// Object has individual interposers.
pub const DF_1_SYMINTPOSE: u64 = 0x0080_0000;
/// Global auditing required.
pub const DF_1_GLOBAUDIT: u64 = 0x0100_0000;
/// Singleton dyn are used.
pub const DF_1_SINGLETON: u64 = 0x0200_0000;
/// Object is a Position Independent Executable (PIE).
pub const DF_1_PIE: u64 = 0x0800_0000;

pub fn df_1_tag_to_str(tag: u64) -> &'static str {
    match tag {
        DF_1_NOW => "DF_1_NOW",
        DF_1_GLOBAL => "DF_1_GLOBAL",
        DF_1_GROUP => "DF_1_GROUP",
        DF_1_NODELETE => "DF_1_NODELETE",
        DF_1_LOADFLTR => "DF_1_LOADFLTR",
        DF_1_INITFIRST => "DF_1_INITFIRST",
        DF_1_NOOPEN => "DF_1_NOOPEN",
        DF_1_ORIGIN => "DF_1_ORIGIN",
        DF_1_DIRECT => "DF_1_DIRECT",
        DF_1_TRANS => "DF_1_TRANS",
        DF_1_INTERPOSE => "DF_1_INTERPOSE",
        DF_1_NODEFLIB => "DF_1_NODEFLIB",
        DF_1_NODUMP => "DF_1_NODUMP",
        DF_1_CONFALT => "DF_1_CONFALT",
        DF_1_ENDFILTEE => "DF_1_ENDFILTEE",
        DF_1_DISPRELDNE => "DF_1_DISPRELDNE",
        DF_1_DISPRELPND => "DF_1_DISPRELPND",
        DF_1_NODIRECT => "DF_1_NODIRECT",
        DF_1_IGNMULDEF => "DF_1_IGNMULDEF",
        DF_1_NOKSYMS => "DF_1_NOKSYMS",
        DF_1_NOHDR => "DF_1_NOHDR",
        DF_1_EDITED => "DF_1_EDITED",
        DF_1_NORELOC => "DF_1_NORELOC",
        DF_1_SYMINTPOSE => "DF_1_SYMINTPOSE",
        DF_1_GLOBAUDIT => "DF_1_GLOBAUDIT",
        DF_1_SINGLETON => "DF_1_SINGLETON",
        DF_1_PIE => "DF_1_PIE",
        _ => "UNKNOWN_TAG",
    }
}

if_alloc! {
    use core::fmt;
    use scroll::ctx;
    use core::result;
    use crate::container::{Ctx, Container};
    use crate::strtab::Strtab;
    use alloc::vec::Vec;

    #[derive(Default, PartialEq, Clone)]
    pub struct Dyn {
        pub d_tag: u64,
        pub d_val: u64,
    }

    impl Dyn {
        #[inline]
        pub fn size(container: Container) -> usize {
            use scroll::ctx::SizeWith;
            Self::size_with(&Ctx::from(container))
        }
    }

    impl fmt::Debug for Dyn {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Dyn")
                .field("d_tag", &tag_to_str(self.d_tag))
                .field("d_val", &format_args!("0x{:x}", self.d_val))
                .finish()
        }
    }

    impl ctx::SizeWith<Ctx> for Dyn {
        fn size_with(&Ctx { container, .. }: &Ctx) -> usize {
            match container {
                Container::Little => {
                    dyn32::SIZEOF_DYN
                },
                Container::Big => {
                    dyn64::SIZEOF_DYN
                },
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, Ctx> for Dyn {
        type Error = crate::error::Error;
        fn try_from_ctx(bytes: &'a [u8], Ctx { container, le}: Ctx) -> result::Result<(Self, usize), Self::Error> {
            use scroll::Pread;
            let dynamic = match container {
                Container::Little => {
                    (bytes.pread_with::<dyn32::Dyn>(0, le)?.into(), dyn32::SIZEOF_DYN)
                },
                Container::Big => {
                    (bytes.pread_with::<dyn64::Dyn>(0, le)?.into(), dyn64::SIZEOF_DYN)
                }
            };
            Ok(dynamic)
        }
    }

    impl ctx::TryIntoCtx<Ctx> for Dyn {
        type Error = crate::error::Error;
        fn try_into_ctx(self, bytes: &mut [u8], Ctx { container, le}: Ctx) -> result::Result<usize, Self::Error> {
            use scroll::Pwrite;
            match container {
                Container::Little => {
                    let dynamic: dyn32::Dyn = self.into();
                    Ok(bytes.pwrite_with(dynamic, 0, le)?)
                },
                Container::Big => {
                    let dynamic: dyn64::Dyn = self.into();
                    Ok(bytes.pwrite_with(dynamic, 0, le)?)
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Dynamic {
        pub dyns: Vec<Dyn>,
        pub info: DynamicInfo,
    }

    impl Dynamic {
        #[cfg(feature = "endian_fd")]
        /// Returns a vector of dynamic entries from the underlying byte `bytes`, with `endianness`, using the provided `phdrs`
        pub fn parse(bytes: &[u8], phdrs: &[crate::elf::program_header::ProgramHeader], ctx: Ctx) -> crate::error::Result<Option<Self>> {
            use scroll::ctx::SizeWith;
            use scroll::Pread;
            use crate::elf::program_header;
            for phdr in phdrs {
                if phdr.p_type == program_header::PT_DYNAMIC {
                    let offset = phdr.p_offset as usize;
                    let filesz = phdr.p_filesz as usize;
                    // Ensure offset and filesz are valid.
                    let bytes = if filesz > 0 {
                        bytes
                            .pread_with::<&[u8]>(offset, filesz)
                            .map_err(|_| crate::error::Error::Malformed(format!("Invalid PT_DYNAMIC size (offset {:#x}, filesz {:#x})",
                                                               offset, filesz)))?
                    } else {
                        &[]
                    };
                    let size = Dyn::size_with(&ctx);
                    // the validity of `count` was implicitly checked by reading `bytes`.
                    let count = filesz / size;
                    let mut dyns = Vec::with_capacity(count);
                    let mut offset = 0;
                    for _ in 0..count {
                        let dynamic = bytes.gread_with::<Dyn>(&mut offset, ctx)?;
                        let tag = dynamic.d_tag;
                        dyns.push(dynamic);
                        if tag == DT_NULL { break }
                    }
                    let mut info = DynamicInfo::default();
                    for dynamic in &dyns {
                        info.update(phdrs, dynamic);
                    }
                    return Ok(Some(Dynamic { dyns: dyns, info: info, }));
                }
            }
            Ok(None)
        }

        pub fn get_libraries<'a>(&self, strtab: &Strtab<'a>) -> Vec<&'a str> {
            use log::warn;
            let count = self.info.needed_count.min(self.dyns.len());
            let mut needed = Vec::with_capacity(count);
            for dynamic in &self.dyns {
                if dynamic.d_tag as u64 == DT_NEEDED {
                    if let Some(lib) = strtab.get_at(dynamic.d_val as usize) {
                        needed.push(lib)
                    } else {
                        warn!("Invalid DT_NEEDED {}", dynamic.d_val)
                    }
                }
            }
            needed
        }
    }
}

macro_rules! elf_dyn_std_impl {
    ($size:ident, $phdr:ty) => {

        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn size_of() {
                assert_eq!(::std::mem::size_of::<Dyn>(), SIZEOF_DYN);
            }
        }

        if_alloc! {
            use core::fmt;
            use core::slice;
            use alloc::vec::Vec;

            use crate::elf::program_header::{PT_DYNAMIC};
            use crate::strtab::Strtab;

            use crate::elf::dynamic::Dyn as ElfDyn;

            if_std! {
                use std::fs::File;
                use std::io::{Read, Seek};
                use std::io::SeekFrom::Start;
                use crate::error::Result;
            }

            impl From<ElfDyn> for Dyn {
                fn from(dynamic: ElfDyn) -> Self {
                    Dyn {
                        d_tag: dynamic.d_tag as $size,
                        d_val: dynamic.d_val as $size,
                    }
                }
            }
            impl From<Dyn> for ElfDyn {
                fn from(dynamic: Dyn) -> Self {
                    ElfDyn {
                        d_tag: u64::from(dynamic.d_tag),
                        d_val: u64::from(dynamic.d_val),
                    }
                }
            }

            impl fmt::Debug for Dyn {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_struct("Dyn")
                        .field("d_tag", &tag_to_str(u64::from(self.d_tag)))
                        .field("d_val", &format_args!("0x{:x}", self.d_val))
                        .finish()
                }
            }

            /// Returns a vector of dynamic entries from the given fd and program headers
            #[cfg(feature = "std")]
            pub fn from_fd(mut fd: &File, phdrs: &[$phdr]) -> Result<Option<Vec<Dyn>>> {
                for phdr in phdrs {
                    if phdr.p_type == PT_DYNAMIC {
                        // FIXME: validate filesz before allocating
                        let filesz = phdr.p_filesz as usize;
                        let dync = filesz / SIZEOF_DYN;
                        let mut dyns = vec![Dyn::default(); dync];
                        fd.seek(Start(u64::from(phdr.p_offset)))?;
                        unsafe {
                            fd.read_exact(plain::as_mut_bytes(&mut *dyns))?;
                        }
                        dyns.dedup();
                        return Ok(Some(dyns));
                    }
                }
                Ok(None)
            }

            /// Given a bias and a memory address (typically for a _correctly_ mmap'd binary in memory), returns the `_DYNAMIC` array as a slice of that memory
            pub unsafe fn from_raw<'a>(bias: usize, vaddr: usize) -> &'a [Dyn] {
                let dynp = vaddr.wrapping_add(bias) as *const Dyn;
                let mut idx = 0;
                while u64::from((*dynp.offset(idx)).d_tag) != DT_NULL {
                    idx += 1;
                }
                slice::from_raw_parts(dynp, idx as usize)
            }

            // TODO: these bare functions have always seemed awkward, but not sure where they should go...
            /// Maybe gets and returns the dynamic array with the same lifetime as the `phdrs`, using the provided bias with wrapping addition.
            /// If the bias is wrong, it will either segfault or give you incorrect values, beware
            pub unsafe fn from_phdrs(bias: usize, phdrs: &[$phdr]) -> Option<&[Dyn]> {
                for phdr in phdrs {
                    // FIXME: change to casting to u64 similar to DT_*?
                    if phdr.p_type as u32 == PT_DYNAMIC {
                        return Some(from_raw(bias, phdr.p_vaddr as usize));
                    }
                }
                None
            }

            /// Gets the needed libraries from the `_DYNAMIC` array, with the str slices lifetime tied to the dynamic array/strtab's lifetime(s)
            pub unsafe fn get_needed<'a>(dyns: &[Dyn], strtab: *const Strtab<'a>, count: usize) -> Vec<&'a str> {
                let mut needed = Vec::with_capacity(count.min(dyns.len()));
                for dynamic in dyns {
                    if u64::from(dynamic.d_tag) == DT_NEEDED {
                        let lib = &(*strtab)[dynamic.d_val as usize];
                        needed.push(lib);
                    }
                }
                needed
            }
        }
    };
}

macro_rules! elf_dynamic_info_std_impl {
    ($size:ident, $phdr:ty) => {
        /// Convert a virtual memory address to a file offset
        fn vm_to_offset(phdrs: &[$phdr], address: $size) -> Option<$size> {
            for ph in phdrs {
                if ph.p_type == crate::elf::program_header::PT_LOAD && address >= ph.p_vaddr {
                    let offset = address - ph.p_vaddr;
                    if offset < ph.p_memsz {
                        return ph.p_offset.checked_add(offset);
                    }
                }
            }
            None
        }

        /// Important dynamic linking info generated via a single pass through the `_DYNAMIC` array
        #[derive(Default, PartialEq)]
        pub struct DynamicInfo {
            pub rela: usize,
            pub relasz: usize,
            pub relaent: $size,
            pub relacount: usize,
            pub rel: usize,
            pub relsz: usize,
            pub relent: $size,
            pub relcount: usize,
            pub gnu_hash: Option<$size>,
            pub hash: Option<$size>,
            pub strtab: usize,
            pub strsz: usize,
            pub symtab: usize,
            pub syment: usize,
            pub pltgot: Option<$size>,
            pub pltrelsz: usize,
            pub pltrel: $size,
            pub jmprel: usize,
            pub verdef: $size,
            pub verdefnum: $size,
            pub verneed: $size,
            pub verneednum: $size,
            pub versym: $size,
            pub init: $size,
            pub fini: $size,
            pub init_array: $size,
            pub init_arraysz: usize,
            pub fini_array: $size,
            pub fini_arraysz: usize,
            pub needed_count: usize,
            pub flags: $size,
            pub flags_1: $size,
            pub soname: usize,
            pub textrel: bool,
        }

        impl DynamicInfo {
            #[inline]
            pub fn update(&mut self, phdrs: &[$phdr], dynamic: &Dyn) {
                match u64::from(dynamic.d_tag) {
                    DT_RELA => self.rela = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0) as usize, // .rela.dyn
                    DT_RELASZ => self.relasz = dynamic.d_val as usize,
                    DT_RELAENT => self.relaent = dynamic.d_val as _,
                    DT_RELACOUNT => self.relacount = dynamic.d_val as usize,
                    DT_REL => self.rel = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0) as usize, // .rel.dyn
                    DT_RELSZ => self.relsz = dynamic.d_val as usize,
                    DT_RELENT => self.relent = dynamic.d_val as _,
                    DT_RELCOUNT => self.relcount = dynamic.d_val as usize,
                    DT_GNU_HASH => self.gnu_hash = vm_to_offset(phdrs, dynamic.d_val),
                    DT_HASH => self.hash = vm_to_offset(phdrs, dynamic.d_val),
                    DT_STRTAB => {
                        self.strtab = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0) as usize
                    }
                    DT_STRSZ => self.strsz = dynamic.d_val as usize,
                    DT_SYMTAB => {
                        self.symtab = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0) as usize
                    }
                    DT_SYMENT => self.syment = dynamic.d_val as usize,
                    DT_PLTGOT => self.pltgot = vm_to_offset(phdrs, dynamic.d_val),
                    DT_PLTRELSZ => self.pltrelsz = dynamic.d_val as usize,
                    DT_PLTREL => self.pltrel = dynamic.d_val as _,
                    DT_JMPREL => {
                        self.jmprel = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0) as usize
                    } // .rela.plt
                    DT_VERDEF => self.verdef = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_VERDEFNUM => self.verdefnum = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_VERNEED => self.verneed = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_VERNEEDNUM => self.verneednum = dynamic.d_val as _,
                    DT_VERSYM => self.versym = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_INIT => self.init = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_FINI => self.fini = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0),
                    DT_INIT_ARRAY => {
                        self.init_array = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0)
                    }
                    DT_INIT_ARRAYSZ => self.init_arraysz = dynamic.d_val as _,
                    DT_FINI_ARRAY => {
                        self.fini_array = vm_to_offset(phdrs, dynamic.d_val).unwrap_or(0)
                    }
                    DT_FINI_ARRAYSZ => self.fini_arraysz = dynamic.d_val as _,
                    DT_NEEDED => self.needed_count += 1,
                    DT_FLAGS => self.flags = dynamic.d_val as _,
                    DT_FLAGS_1 => self.flags_1 = dynamic.d_val as _,
                    DT_SONAME => self.soname = dynamic.d_val as _,
                    DT_TEXTREL => self.textrel = true,
                    _ => (),
                }
            }
            pub fn new(dynamic: &[Dyn], phdrs: &[$phdr]) -> DynamicInfo {
                let mut info = DynamicInfo::default();
                for dyna in dynamic {
                    info.update(phdrs, &dyna);
                }
                info
            }
        }

        if_alloc! {
            impl fmt::Debug for DynamicInfo {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let gnu_hash = self.gnu_hash.unwrap_or(0);
                    let hash = self.hash.unwrap_or(0);
                    let pltgot = self.pltgot.unwrap_or(0);

                    let flags: Vec<&'static str> = [DF_ORIGIN, DF_SYMBOLIC, DF_TEXTREL, DF_BIND_NOW, DF_STATIC_TLS,][..]
                                    .iter()
                                    .filter(|f| (self.flags as u64 & *f) != 0)
                                    .map(|f| df_tag_to_str(*f))
                                    .collect();

                    let flags_1: Vec<&'static str> = [
                            DF_1_NOW,
                            DF_1_GLOBAL,
                            DF_1_GROUP,
                            DF_1_NODELETE,
                            DF_1_LOADFLTR,
                            DF_1_INITFIRST,
                            DF_1_NOOPEN,
                            DF_1_ORIGIN,
                            DF_1_DIRECT,
                            DF_1_TRANS,
                            DF_1_INTERPOSE,
                            DF_1_NODEFLIB,
                            DF_1_NODUMP,
                            DF_1_CONFALT,
                            DF_1_ENDFILTEE,
                            DF_1_DISPRELDNE,
                            DF_1_DISPRELPND,
                            DF_1_NODIRECT,
                            DF_1_IGNMULDEF,
                            DF_1_NOKSYMS,
                            DF_1_NOHDR,
                            DF_1_EDITED,
                            DF_1_NORELOC,
                            DF_1_SYMINTPOSE,
                            DF_1_GLOBAUDIT,
                            DF_1_SINGLETON,
                            DF_1_PIE,
                        ][..]
                        .iter()
                        .filter(|f| (self.flags_1 as u64 & *f) != 0)
                        .map(|f| df_1_tag_to_str(*f))
                        .collect();

                    f.debug_struct("DynamicInfo")
                        .field("rela", &format_args!("0x{:x}", self.rela))
                        .field("relasz", &self.relasz)
                        .field("relaent", &self.relaent)
                        .field("relacount", &self.relacount)
                        .field("gnu_hash", &format_args!("0x{:x}", gnu_hash))
                        .field("hash", &format_args!("0x{:x}", hash))
                        .field("strtab", &format_args!("0x{:x}", self.strtab))
                        .field("strsz", &self.strsz)
                        .field("symtab", &format_args!("0x{:x}", self.symtab))
                        .field("syment", &self.syment)
                        .field("pltgot", &format_args!("0x{:x}", pltgot))
                        .field("pltrelsz", &self.pltrelsz)
                        .field("pltrel", &self.pltrel)
                        .field("jmprel", &format_args!("0x{:x}", self.jmprel))
                        .field("verdef", &format_args!("0x{:x}", self.verdef))
                        .field("verdefnum", &self.verdefnum)
                        .field("verneed", &format_args!("0x{:x}", self.verneed))
                        .field("verneednum", &self.verneednum)
                        .field("versym", &format_args!("0x{:x}", self.versym))
                        .field("init", &format_args!("0x{:x}", self.init))
                        .field("fini", &format_args!("0x{:x}", self.fini))
                        .field("init_array", &format_args!("{:#x}", self.init_array))
                        .field("init_arraysz", &self.init_arraysz)
                        .field("needed_count", &self.needed_count)
                        .field("flags", &format_args!("{:#0width$x} {:?}", self.flags, flags, width = core::mem::size_of_val(&self.flags)))
                        .field("flags_1", &format_args!("{:#0width$x} {:?}", self.flags_1, flags_1, width = core::mem::size_of_val(&self.flags_1)))
                        .field("soname", &self.soname)
                        .field("textrel", &self.textrel)
                        .finish()
                }
            }
        }
    };
}

if_alloc! {
    elf_dynamic_info_std_impl!(u64, crate::elf::program_header::ProgramHeader);
}

pub mod dyn32 {
    pub use crate::elf::dynamic::*;

    elf_dyn!(u32);

    pub const SIZEOF_DYN: usize = 8;

    elf_dyn_std_impl!(u32, crate::elf32::program_header::ProgramHeader);
    elf_dynamic_info_std_impl!(
        u32,
        crate::elf::program_header::program_header32::ProgramHeader
    );
}

pub mod dyn64 {
    pub use crate::elf::dynamic::*;

    elf_dyn!(u64);

    pub const SIZEOF_DYN: usize = 16;

    elf_dyn_std_impl!(u64, crate::elf64::program_header::ProgramHeader);
    elf_dynamic_info_std_impl!(
        u64,
        crate::elf::program_header::program_header64::ProgramHeader
    );
}
