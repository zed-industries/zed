//! The generic ELF module, which gives access to ELF constants and other helper functions, which are independent of ELF bithood.  Also defines an `Elf` struct which implements a unified parser that returns a wrapped `Elf64` or `Elf32` binary.
//!
//! To access the exact 32-bit or 64-bit versions, use [goblin::elf32::Header](header/header32/struct.Header.html)/[goblin::elf64::Header](header/header64/struct.Header.html), etc., for the various 32/64-bit structs.
//!
//! # Example
//!
//! ```rust
//! use std::fs::File;
//!
//! pub fn read (bytes: &[u8]) {
//!   match goblin::elf::Elf::parse(&bytes) {
//!     Ok(binary) => {
//!       let entry = binary.entry;
//!       for ph in binary.program_headers {
//!         if ph.p_type == goblin::elf::program_header::PT_LOAD {
//!           // TODO: you should validate p_filesz before allocating.
//!           let mut _buf = vec![0u8; ph.p_filesz as usize];
//!           // read responsibly
//!          }
//!       }
//!     },
//!     Err(_) => ()
//!   }
//! }
//! ```
//!
//! This will properly access the underlying 32-bit or 64-bit binary automatically. Note that since
//! 32-bit binaries typically have shorter 32-bit values in some cases (specifically for addresses and pointer
//! values), these values are upcasted to u64/i64s when appropriate.
//!
//! See [goblin::elf::Elf](struct.Elf.html) for more information.
//!
//! You are still free to use the specific 32-bit or 64-bit versions by accessing them through `goblin::elf64`, etc., but you will have to parse and/or construct the various components yourself.
//! In other words, there is no unified 32/64-bit `Elf` struct.
//!
//! # Note
//! To use the automagic ELF datatype union parser, you _must_ enable/opt-in to the  `elf64`, `elf32`, and
//! `endian_fd` features if you disable `default`.

#[macro_use]
pub(crate) mod gnu_hash;

// These are shareable values for the 32/64 bit implementations.
//
// They are publicly re-exported by the pub-using module
pub mod compression_header;
pub mod header;
pub mod program_header;
pub mod section_header;
#[macro_use]
pub mod sym;
pub mod dynamic;
#[macro_use]
pub mod reloc;
pub mod note;
#[cfg(all(any(feature = "elf32", feature = "elf64"), feature = "alloc"))]
pub mod symver;

macro_rules! if_sylvan {
    ($($i:item)*) => ($(
        #[cfg(all(feature = "elf32", feature = "elf64", feature = "endian_fd"))]
        $i
    )*)
}

if_sylvan! {
    use scroll::{ctx, Pread, Endian};
    use crate::strtab::Strtab;
    use crate::error;
    use crate::container::{Container, Ctx};
    use alloc::vec::Vec;
    use core::cmp;

    pub use header::Header;
    pub use program_header::ProgramHeader;
    pub use section_header::SectionHeader;
    pub use sym::Symtab;
    pub use sym::Sym;
    pub use dynamic::Dyn;
    pub use dynamic::Dynamic;
    pub use reloc::Reloc;
    pub use reloc::RelocSection;
    pub use symver::{VersymSection, VerdefSection, VerneedSection};

    pub type ProgramHeaders = Vec<ProgramHeader>;
    pub type SectionHeaders = Vec<SectionHeader>;
    pub type ShdrIdx = usize;

    #[derive(Debug)]
    /// An ELF binary. The underlying data structures are read according to the headers byte order and container size (32 or 64).
    pub struct Elf<'a> {
        /// The ELF header, which provides a rudimentary index into the rest of the binary
        pub header: Header,
        /// The program headers; they primarily tell the kernel and the dynamic linker
        /// how to load this binary
        pub program_headers: ProgramHeaders,
        /// The sections headers. These are strippable, never count on them being
        /// here unless you're a static linker!
        pub section_headers: SectionHeaders,
        /// The section header string table
        pub shdr_strtab: Strtab<'a>,
        /// The string table for the dynamically accessible symbols
        pub dynstrtab: Strtab<'a>,
        /// The dynamically accessible symbols, i.e., exports, imports.
        /// This is what the dynamic linker uses to dynamically load and link your binary,
        /// or find imported symbols for binaries which dynamically link against your library
        pub dynsyms: Symtab<'a>,
        /// The debugging symbol table
        pub syms: Symtab<'a>,
        /// The string table for the symbol table
        pub strtab: Strtab<'a>,
        /// Contains dynamic linking information, with the _DYNAMIC array + a preprocessed DynamicInfo for that array
        pub dynamic: Option<Dynamic>,
        /// The dynamic relocation entries (strings, copy-data, etc.) with an addend
        pub dynrelas: RelocSection<'a>,
        /// The dynamic relocation entries without an addend
        pub dynrels: RelocSection<'a>,
        /// The plt relocation entries (procedure linkage table). For 32-bit binaries these are usually Rel (no addend)
        pub pltrelocs: RelocSection<'a>,
        /// Section relocations by section index (only present if this is a relocatable object file)
        pub shdr_relocs: Vec<(ShdrIdx, RelocSection<'a>)>,
        /// The binary's soname, if it has one
        pub soname: Option<&'a str>,
        /// The binary's program interpreter (e.g., dynamic linker), if it has one
        pub interpreter: Option<&'a str>,
        /// A list of this binary's dynamic libraries it uses, if there are any
        pub libraries: Vec<&'a str>,
        /// A list of runtime search paths for this binary's dynamic libraries it uses, if there
        /// are any. (deprecated)
        pub rpaths: Vec<&'a str>,
        /// A list of runtime search paths for this binary's dynamic libraries it uses, if there
        /// are any.
        pub runpaths: Vec<&'a str>,
        /// Whether this is a 64-bit elf or not
        pub is_64: bool,
        /// Whether this is a shared object or not
        pub is_lib: bool,
        /// The binaries entry point address, if it has one
        pub entry: u64,
        /// Whether the binary is little endian or not
        pub little_endian: bool,
        /// Contains the symbol version information from the optional section
        /// [`SHT_GNU_VERSYM`][section_header::SHT_GNU_VERSYM] (GNU extenstion).
        pub versym : Option<VersymSection<'a>>,
        /// Contains the version definition information from the optional section
        /// [`SHT_GNU_VERDEF`][section_header::SHT_GNU_VERDEF] (GNU extenstion).
        pub verdef : Option<VerdefSection<'a>>,
        /// Contains the version needed information from the optional section
        /// [`SHT_GNU_VERNEED`][section_header::SHT_GNU_VERNEED] (GNU extenstion).
        pub verneed : Option<VerneedSection<'a>>,
        ctx: Ctx,
    }

    impl<'a> Elf<'a> {
        /// Try to iterate notes in PT_NOTE program headers; returns `None` if there aren't any note headers in this binary
        pub fn iter_note_headers(&self, data: &'a [u8]) -> Option<note::NoteIterator<'a>> {
            let mut iters = vec![];
            for phdr in &self.program_headers {
                if phdr.p_type == program_header::PT_NOTE {
                    let offset = phdr.p_offset as usize;
                    let alignment = phdr.p_align as usize;

                    iters.push(note::NoteDataIterator {
                        data,
                        offset,
                        size: offset.saturating_add(phdr.p_filesz as usize),
                        ctx: (alignment, self.ctx)
                    });
                }
            }

            if iters.is_empty() {
                None
            } else {
                Some(note::NoteIterator {
                    iters: iters,
                    index: 0,
                })
            }
        }
        /// Try to iterate notes in SHT_NOTE sections; returns `None` if there aren't any note sections in this binary
        ///
        /// If a section_name is given, only the section with the according name is iterated.
        pub fn iter_note_sections(
            &self,
            data: &'a [u8],
            section_name: Option<&str>,
        ) -> Option<note::NoteIterator<'a>> {
            let mut iters = vec![];
            for sect in &self.section_headers {
                if sect.sh_type != section_header::SHT_NOTE {
                    continue;
                }

                if section_name.is_some() && self.shdr_strtab.get_at(sect.sh_name) != section_name {
                    continue;
                }

                let offset = sect.sh_offset as usize;
                let alignment = sect.sh_addralign as usize;
                iters.push(note::NoteDataIterator {
                    data,
                    offset,
                    size: offset.saturating_add(sect.sh_size as usize),
                    ctx: (alignment, self.ctx)
                });
            }

            if iters.is_empty() {
                None
            } else {
                Some(note::NoteIterator {
                    iters: iters,
                    index: 0,
                })
            }
        }
        pub fn is_object_file(&self) -> bool {
            self.header.e_type == header::ET_REL
        }

        /// Parses the contents to get the Header only. This `bytes` buffer should contain at least the length for parsing Header.
        pub fn parse_header(bytes: &'a [u8]) -> error::Result<Header> {
            bytes.pread::<Header>(0)
        }

        /// Lazy parse the ELF contents. This function mainly just assembles an Elf struct. Once we have the struct, we can choose to parse whatever we want.
        pub fn lazy_parse(header: Header) -> error::Result<Self> {
            let misc = parse_misc(&header)?;

            Ok(Elf {
                header,
                program_headers: vec![],
                section_headers: Default::default(),
                shdr_strtab: Default::default(),
                dynamic: None,
                dynsyms: Default::default(),
                dynstrtab: Strtab::default(),
                syms: Default::default(),
                strtab: Default::default(),
                dynrelas: Default::default(),
                dynrels: Default::default(),
                pltrelocs: Default::default(),
                shdr_relocs: Default::default(),
                soname: None,
                interpreter: None,
                libraries: vec![],
                rpaths: vec![],
                runpaths: vec![],
                is_64: misc.is_64,
                is_lib: misc.is_lib,
                entry: misc.entry,
                little_endian: misc.little_endian,
                ctx: misc.ctx,
                versym: None,
                verdef: None,
                verneed: None,
            })
        }

        /// Parses the contents of the byte stream in `bytes`, and maybe returns a unified binary
        pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
            let header = Self::parse_header(bytes)?;
            let misc = parse_misc(&header)?;
            let ctx = misc.ctx;

            let program_headers = ProgramHeader::parse(bytes, header.e_phoff as usize, header.e_phnum as usize, ctx)?;

            let mut interpreter = None;
            for ph in &program_headers {
                if ph.p_type == program_header::PT_INTERP && ph.p_filesz != 0 {
                    let count = (ph.p_filesz - 1) as usize;
                    let offset = ph.p_offset as usize;
                    interpreter = bytes.pread_with::<&str>(offset, ::scroll::ctx::StrCtx::Length(count)).ok();
                }
            }

            let section_headers = SectionHeader::parse(bytes, header.e_shoff as usize, header.e_shnum as usize, ctx)?;

            let get_strtab = |section_headers: &[SectionHeader], mut section_idx: usize| {
                if section_idx == section_header::SHN_XINDEX as usize {
                    if section_headers.is_empty() {
                        return Ok(Strtab::default())
                    }
                    section_idx = section_headers[0].sh_link as usize;
                }

                if section_idx >= section_headers.len() {
                    // FIXME: warn! here
                    Ok(Strtab::default())
                } else {
                    let shdr = &section_headers[section_idx];
                    shdr.check_size(bytes.len())?;
                    Strtab::parse(bytes, shdr.sh_offset as usize, shdr.sh_size as usize, 0x0)
                }
            };

            let strtab_idx = header.e_shstrndx as usize;
            let shdr_strtab = get_strtab(&section_headers, strtab_idx)?;

            let mut syms = Symtab::default();
            let mut strtab = Strtab::default();
            if let Some(shdr) = section_headers.iter().rfind(|shdr| shdr.sh_type as u32 == section_header::SHT_SYMTAB) {
                let size = shdr.sh_entsize;
                let count = if size == 0 { 0 } else { shdr.sh_size / size };
                syms = Symtab::parse(bytes, shdr.sh_offset as usize, count as usize, ctx)?;
                strtab = get_strtab(&section_headers, shdr.sh_link as usize)?;
            }

            let mut is_pie = false;
            let mut soname = None;
            let mut libraries = vec![];
            let mut rpaths = vec![];
            let mut runpaths = vec![];
            let mut dynsyms = Symtab::default();
            let mut dynrelas = RelocSection::default();
            let mut dynrels = RelocSection::default();
            let mut pltrelocs = RelocSection::default();
            let mut dynstrtab = Strtab::default();
            let dynamic = Dynamic::parse(bytes, &program_headers, ctx)?;
            if let Some(ref dynamic) = dynamic {
                let dyn_info = &dynamic.info;

                is_pie = dyn_info.flags_1 & dynamic::DF_1_PIE != 0;
                dynstrtab = Strtab::parse(bytes,
                                          dyn_info.strtab,
                                          dyn_info.strsz,
                                          0x0)?;

                if dyn_info.soname != 0 {
                    // FIXME: warn! here
                    soname = dynstrtab.get_at(dyn_info.soname);
                }
                if dyn_info.needed_count > 0 {
                    libraries = dynamic.get_libraries(&dynstrtab);
                }
                for dyn_ in &dynamic.dyns {
                    if dyn_.d_tag == dynamic::DT_RPATH {
                        if let Some(path) = dynstrtab.get_at(dyn_.d_val as usize) {
                            rpaths.push(path);
                        }
                    } else if dyn_.d_tag == dynamic::DT_RUNPATH {
                        if let Some(path) = dynstrtab.get_at(dyn_.d_val as usize) {
                            runpaths.push(path);
                        }
                    }
                }
                // parse the dynamic relocations
                dynrelas = RelocSection::parse(bytes, dyn_info.rela, dyn_info.relasz, true, ctx)?;
                dynrels = RelocSection::parse(bytes, dyn_info.rel, dyn_info.relsz, false, ctx)?;
                let is_rela = dyn_info.pltrel as u64 == dynamic::DT_RELA;
                pltrelocs = RelocSection::parse(bytes, dyn_info.jmprel, dyn_info.pltrelsz, is_rela, ctx)?;

                let mut num_syms = if let Some(gnu_hash) = dyn_info.gnu_hash {
                    gnu_hash_len(bytes, gnu_hash as usize, ctx)?
                } else if let Some(hash) = dyn_info.hash {
                    hash_len(bytes, hash as usize, header.e_machine, ctx)?
                } else {
                    0
                };
                let max_reloc_sym = dynrelas.iter()
                    .chain(dynrels.iter())
                    .chain(pltrelocs.iter())
                    .fold(0, |num, reloc| cmp::max(num, reloc.r_sym));
                if max_reloc_sym != 0 {
                    num_syms = cmp::max(num_syms, max_reloc_sym + 1);
                }
                dynsyms = Symtab::parse(bytes, dyn_info.symtab, num_syms, ctx)?;
            }

            let mut shdr_relocs = vec![];
            for (idx, section) in section_headers.iter().enumerate() {
                let is_rela = section.sh_type == section_header::SHT_RELA;
                if is_rela || section.sh_type == section_header::SHT_REL {
                    section.check_size(bytes.len())?;
                    let sh_relocs = RelocSection::parse(bytes, section.sh_offset as usize, section.sh_size as usize, is_rela, ctx)?;
                    shdr_relocs.push((idx, sh_relocs));
                }
            }

            let versym = symver::VersymSection::parse(bytes, &section_headers, ctx)?;
            let verdef = symver::VerdefSection::parse(bytes, &section_headers, ctx)?;
            let verneed = symver::VerneedSection::parse(bytes, &section_headers, ctx)?;

            let is_lib = misc.is_lib && !is_pie;

            Ok(Elf {
                header,
                program_headers,
                section_headers,
                shdr_strtab,
                dynamic,
                dynsyms,
                dynstrtab,
                syms,
                strtab,
                dynrelas,
                dynrels,
                pltrelocs,
                shdr_relocs,
                soname,
                interpreter,
                libraries,
                rpaths,
                runpaths,
                is_64: misc.is_64,
                is_lib,
                entry: misc.entry,
                little_endian: misc.little_endian,
                ctx: ctx,
                versym,
                verdef,
                verneed,
            })
        }
    }

    impl<'a> ctx::TryFromCtx<'a, (usize, Endian)> for Elf<'a> {
        type Error = crate::error::Error;
        fn try_from_ctx(src: &'a [u8], (_, _): (usize, Endian)) -> Result<(Elf<'a>, usize), Self::Error> {
            let elf = Elf::parse(src)?;
            Ok((elf, src.len()))
        }
    }

    fn gnu_hash_len(bytes: &[u8], offset: usize, ctx: Ctx) -> error::Result<usize> {
        let buckets_num = bytes.pread_with::<u32>(offset, ctx.le)? as usize;
        let min_chain = bytes.pread_with::<u32>(offset + 4, ctx.le)? as usize;
        let bloom_size = bytes.pread_with::<u32>(offset + 8, ctx.le)? as usize;
        // We could handle min_chain==0 if we really had to, but it shouldn't happen.
        if buckets_num == 0 || min_chain == 0 || bloom_size == 0 {
            return Err(error::Error::Malformed(format!("Invalid DT_GNU_HASH: buckets_num={} min_chain={} bloom_size={}",
                                                       buckets_num, min_chain, bloom_size)));
        }
        // Find the last bucket.
        let buckets_offset = offset + 16 + bloom_size * if ctx.container.is_big() { 8 } else { 4 };
        let mut max_chain = 0;
        for bucket in 0..buckets_num {
            let chain = bytes.pread_with::<u32>(buckets_offset + bucket * 4, ctx.le)? as usize;
            if max_chain < chain {
                max_chain = chain;
            }
        }
        if max_chain < min_chain {
            return Ok(0);
        }
        // Find the last chain within the bucket.
        let mut chain_offset = buckets_offset + buckets_num * 4 + (max_chain - min_chain) * 4;
        loop {
            let hash = bytes.pread_with::<u32>(chain_offset, ctx.le)?;
            max_chain += 1;
            chain_offset += 4;
            if hash & 1 != 0 {
                return Ok(max_chain);
            }
        }
    }

    fn hash_len(bytes: &[u8], offset: usize, machine: u16, ctx: Ctx) -> error::Result<usize> {
        // Based on readelf code.
        let nchain = if (machine == header::EM_FAKE_ALPHA || machine == header::EM_S390) && ctx.container.is_big() {
            bytes.pread_with::<u64>(offset.saturating_add(4), ctx.le)? as usize
        } else {
            bytes.pread_with::<u32>(offset.saturating_add(4), ctx.le)? as usize
        };
        Ok(nchain)
    }

    struct Misc {
        is_64: bool,
        is_lib: bool,
        entry: u64,
        little_endian: bool,
        ctx: Ctx,
    }

    fn parse_misc(header: &Header) -> error::Result<Misc> {
        let entry = header.e_entry as usize;
        let is_lib = header.e_type == header::ET_DYN;
        let is_lsb = header.e_ident[header::EI_DATA] == header::ELFDATA2LSB;
        let endianness = scroll::Endian::from(is_lsb);
        let class = header.e_ident[header::EI_CLASS];
        if class != header::ELFCLASS64 && class != header::ELFCLASS32 {
            return Err(error::Error::Malformed(format!("Unknown values in ELF ident header: class: {} endianness: {}",
                                                        class,
                                                        header.e_ident[header::EI_DATA])));
        }
        let is_64 = class == header::ELFCLASS64;
        let container = if is_64 { Container::Big } else { Container::Little };
        let ctx = Ctx::new(container, endianness);

        Ok(Misc{
            is_64,
            is_lib,
            entry: entry as u64,
            little_endian:is_lsb,
            ctx,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_crt1_64bit() {
        let crt1: Vec<u8> = include!("../../etc/crt1.rs");
        match Elf::parse(&crt1) {
            Ok(binary) => {
                assert!(binary.is_64);
                assert!(!binary.is_lib);
                assert_eq!(binary.entry, 0);
                assert!(binary.syms.get(1000).is_none());
                assert!(binary.syms.get(5).is_some());
                let syms = binary.syms.to_vec();
                assert!(!binary.section_headers.is_empty());
                for (i, sym) in syms.iter().enumerate() {
                    if i == 11 {
                        let symtab = binary.strtab;
                        println!("sym: {:?}", &sym);
                        assert_eq!(&symtab[sym.st_name], "_start");
                        break;
                    }
                }
                assert!(!syms.is_empty());
            }
            Err(err) => {
                panic!("failed: {}", err);
            }
        }
    }

    #[test]
    fn parse_crt1_32bit() {
        let crt1: Vec<u8> = include!("../../etc/crt132.rs");
        match Elf::parse(&crt1) {
            Ok(binary) => {
                assert!(!binary.is_64);
                assert!(!binary.is_lib);
                assert_eq!(binary.entry, 0);
                assert!(binary.syms.get(1000).is_none());
                assert!(binary.syms.get(5).is_some());
                let syms = binary.syms.to_vec();
                assert!(!binary.section_headers.is_empty());
                for (i, sym) in syms.iter().enumerate() {
                    if i == 11 {
                        let symtab = binary.strtab;
                        println!("sym: {:?}", &sym);
                        assert_eq!(&symtab[sym.st_name], "__libc_csu_fini");
                        break;
                    }
                }
                assert!(!syms.is_empty());
            }
            Err(err) => {
                panic!("failed: {}", err);
            }
        }
    }

    // See https://github.com/m4b/goblin/issues/257
    #[test]
    #[allow(unused)]
    fn no_use_statement_conflict() {
        use crate::elf::section_header::*;
        use crate::elf::*;

        fn f(_: SectionHeader) {}
    }
}
