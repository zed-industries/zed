//! The Mach-o, mostly zero-copy, binary format parser and raw struct definitions
use alloc::vec::Vec;
use core::fmt;

use log::debug;

use scroll::ctx::SizeWith;
use scroll::{Pread, BE};

use crate::{archive, container};
use crate::{error, take_hint_bytes};

pub mod bind_opcodes;
pub mod constants;
pub mod exports;
pub mod fat;
pub mod header;
pub mod imports;
pub mod load_command;
pub mod relocation;
pub mod segment;
pub mod symbols;

pub use self::constants::cputype;

/// Returns a big endian magical number
pub fn peek(bytes: &[u8], offset: usize) -> error::Result<u32> {
    Ok(bytes.pread_with::<u32>(offset, scroll::BE)?)
}

/// Parses a magic number, and an accompanying mach-o binary parsing context, according to the magic number.
pub fn parse_magic_and_ctx(
    bytes: &[u8],
    offset: usize,
) -> error::Result<(u32, Option<container::Ctx>)> {
    use crate::container::Container;
    use crate::mach::header::*;
    let magic = bytes.pread_with::<u32>(offset, BE)?;
    let ctx = match magic {
        MH_CIGAM_64 | MH_CIGAM | MH_MAGIC_64 | MH_MAGIC => {
            let is_lsb = magic == MH_CIGAM || magic == MH_CIGAM_64;
            let le = scroll::Endian::from(is_lsb);
            let container = if magic == MH_MAGIC_64 || magic == MH_CIGAM_64 {
                Container::Big
            } else {
                Container::Little
            };
            Some(container::Ctx::new(container, le))
        }
        _ => None,
    };
    Ok((magic, ctx))
}

/// A cross-platform, zero-copy, endian-aware, 32/64 bit Mach-o binary parser
pub struct MachO<'a> {
    /// The mach-o header
    pub header: header::Header,
    /// The load commands tell the kernel and dynamic linker how to use/interpret this binary
    pub load_commands: Vec<load_command::LoadCommand>,
    /// The load command "segments" - typically the pieces of the binary that are loaded into memory
    pub segments: segment::Segments<'a>,
    /// The "Nlist" style symbols in this binary - strippable
    pub symbols: Option<symbols::Symbols<'a>>,
    /// The dylibs this library depends on
    pub libs: Vec<&'a str>,
    /// The runtime search paths for dylibs this library depends on
    pub rpaths: Vec<&'a str>,
    /// The entry point (as a virtual memory address), 0 if none
    pub entry: u64,
    /// Whether `entry` refers to an older `LC_UNIXTHREAD` instead of the newer `LC_MAIN` entrypoint
    pub old_style_entry: bool,
    /// The name of the dylib, if any
    pub name: Option<&'a str>,
    /// Are we a little-endian binary?
    pub little_endian: bool,
    /// Are we a 64-bit binary
    pub is_64: bool,
    data: &'a [u8],
    ctx: container::Ctx,
    export_trie: Option<exports::ExportTrie<'a>>,
    bind_interpreter: Option<imports::BindInterpreter<'a>>,
}

impl<'a> fmt::Debug for MachO<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MachO")
            .field("header", &self.header)
            .field("load_commands", &self.load_commands)
            .field("segments", &self.segments)
            .field("entry", &self.entry)
            .field("old_style_entry", &self.old_style_entry)
            .field("libs", &self.libs)
            .field("name", &self.name)
            .field("little_endian", &self.little_endian)
            .field("is_64", &self.is_64)
            .field("symbols()", &self.symbols().collect::<Vec<_>>())
            .field("exports()", &self.exports())
            .field("imports()", &self.imports())
            .finish()
    }
}

impl<'a> MachO<'a> {
    /// Is this a relocatable object file?
    pub fn is_object_file(&self) -> bool {
        self.header.filetype == header::MH_OBJECT
    }
    /// Return an iterator over all the symbols in this binary
    pub fn symbols(&self) -> symbols::SymbolIterator<'a> {
        if let Some(ref symbols) = self.symbols {
            symbols.into_iter()
        } else {
            symbols::SymbolIterator::default()
        }
    }
    /// Return a vector of the relocations in this binary
    pub fn relocations(
        &self,
    ) -> error::Result<Vec<(usize, segment::RelocationIterator, segment::Section)>> {
        debug!("Iterating relocations");
        let mut relocs = Vec::new();
        for (_i, segment) in (&self.segments).into_iter().enumerate() {
            for (j, section) in segment.into_iter().enumerate() {
                let (section, _data) = section?;
                if section.nreloc > 0 {
                    relocs.push((j, section.iter_relocations(self.data, self.ctx), section));
                }
            }
        }
        Ok(relocs)
    }
    /// Return the exported symbols in this binary (if any)
    pub fn exports(&self) -> error::Result<Vec<exports::Export>> {
        if let Some(ref trie) = self.export_trie {
            trie.exports(self.libs.as_slice())
        } else {
            Ok(vec![])
        }
    }
    /// Return the imported symbols in this binary that dyld knows about (if any)
    pub fn imports(&self) -> error::Result<Vec<imports::Import>> {
        if let Some(ref interpreter) = self.bind_interpreter {
            interpreter.imports(self.libs.as_slice(), self.segments.as_slice(), self.ctx)
        } else {
            Ok(vec![])
        }
    }
    /// Parses the Mach-o binary from `bytes` at `offset`
    pub fn parse(bytes: &'a [u8], offset: usize) -> error::Result<MachO<'a>> {
        Self::parse_impl(bytes, offset, false)
    }

    /// Parses the Mach-o binary from `bytes` at `offset` in lossy mode
    pub fn parse_lossy(bytes: &'a [u8], offset: usize) -> error::Result<MachO<'a>> {
        Self::parse_impl(bytes, offset, true)
    }

    /// Parses the Mach-o binary from `bytes` at `offset` in `lossy` mode
    fn parse_impl(bytes: &'a [u8], mut offset: usize, lossy: bool) -> error::Result<MachO<'a>> {
        let (magic, maybe_ctx) = parse_magic_and_ctx(bytes, offset)?;
        let ctx = if let Some(ctx) = maybe_ctx {
            ctx
        } else {
            return Err(error::Error::BadMagic(u64::from(magic)));
        };
        debug!("Ctx: {:?}", ctx);
        let offset = &mut offset;
        let header: header::Header = bytes.pread_with(*offset, ctx)?;
        debug!("Mach-o header: {:?}", header);
        let little_endian = ctx.le.is_little();
        let is_64 = ctx.container.is_big();
        *offset += header::Header::size_with(&ctx.container);
        let ncmds = header.ncmds;

        let sizeofcmds = header.sizeofcmds as usize;
        // a load cmd is at least 2 * 4 bytes, (type, sizeof)
        if ncmds > sizeofcmds / 8 || sizeofcmds > bytes.len() {
            return Err(error::Error::BufferTooShort(ncmds, "load commands"));
        }

        let mut cmds: Vec<load_command::LoadCommand> = Vec::with_capacity(ncmds);
        let mut symbols = None;
        let mut libs = vec!["self"];
        let mut rpaths = vec![];
        let mut export_trie = None;
        let mut bind_interpreter = None;
        let mut unixthread_entry_address = None;
        let mut main_entry_offset = None;
        let mut name = None;
        let mut segments = segment::Segments::new(ctx);
        for i in 0..ncmds {
            let cmd = load_command::LoadCommand::parse(bytes, offset, ctx.le)?;
            debug!("{} - {:?}", i, cmd);
            match cmd.command {
                load_command::CommandVariant::Segment32(command) => segments.push(
                    segment::Segment::from_32_impl(bytes, &command, cmd.offset, ctx, lossy)?,
                ),
                load_command::CommandVariant::Segment64(command) => segments.push(
                    segment::Segment::from_64_impl(bytes, &command, cmd.offset, ctx, lossy)?,
                ),
                load_command::CommandVariant::Symtab(command) => {
                    match symbols::Symbols::parse(bytes, &command, ctx) {
                        Ok(s) => symbols = Some(s),
                        Err(e) if lossy => {
                            debug!("CommandVariant::Symtab failed: {e}");
                        }
                        Err(e) => return Err(e),
                    }
                }
                load_command::CommandVariant::LoadDylib(command)
                | load_command::CommandVariant::LoadUpwardDylib(command)
                | load_command::CommandVariant::ReexportDylib(command)
                | load_command::CommandVariant::LoadWeakDylib(command)
                | load_command::CommandVariant::LazyLoadDylib(command) => {
                    match bytes.pread::<&str>(cmd.offset + command.dylib.name as usize) {
                        Ok(lib) => libs.push(lib),
                        Err(e) if lossy => {
                            debug!("CommandVariant::Load/Reexport Dylib failed: {e}");
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                load_command::CommandVariant::Rpath(command) => {
                    match bytes.pread::<&str>(cmd.offset + command.path as usize) {
                        Ok(rpath) => rpaths.push(rpath),
                        Err(e) if lossy => {
                            debug!("CommandVariant::Rpath failed: {e}");
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                load_command::CommandVariant::DyldInfo(command)
                | load_command::CommandVariant::DyldInfoOnly(command) => {
                    export_trie = Some(exports::ExportTrie::new(bytes, &command));
                    bind_interpreter = Some(imports::BindInterpreter::new(bytes, &command));
                }
                load_command::CommandVariant::DyldExportsTrie(command) => {
                    export_trie = Some(exports::ExportTrie::new_from_linkedit_data_command(
                        bytes, &command,
                    ));
                }
                load_command::CommandVariant::Unixthread(command) => {
                    // dyld cares only about the first LC_UNIXTHREAD
                    if unixthread_entry_address.is_none() {
                        unixthread_entry_address =
                            Some(command.instruction_pointer(header.cputype)?);
                    }
                }
                load_command::CommandVariant::Main(command) => {
                    // dyld cares only about the first LC_MAIN
                    if main_entry_offset.is_none() {
                        main_entry_offset = Some(command.entryoff);
                    }
                }
                load_command::CommandVariant::IdDylib(command) => {
                    match bytes.pread::<&str>(cmd.offset + command.dylib.name as usize) {
                        Ok(id) => {
                            libs[0] = id;
                            name = Some(id);
                        }
                        Err(e) if lossy => {
                            debug!("CommandVariant::IdDylib failed: {e}");
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                _ => (),
            }
            cmds.push(cmd)
        }

        // dyld prefers LC_MAIN over LC_UNIXTHREAD
        // choose the same way here
        let (entry, old_style_entry) = if let Some(offset) = main_entry_offset {
            // map the entrypoint offset to a virtual memory address
            let base_address = segments
                .iter()
                .filter(|s| &s.segname[0..7] == b"__TEXT\0")
                .map(|s| s.vmaddr - s.fileoff)
                .next()
                .ok_or_else(|| {
                    error::Error::Malformed(format!(
                        "image specifies LC_MAIN offset {} but has no __TEXT segment",
                        offset
                    ))
                })?;

            (base_address + offset, false)
        } else if let Some(address) = unixthread_entry_address {
            (address, true)
        } else {
            (0, false)
        };

        Ok(MachO {
            header,
            load_commands: cmds,
            segments,
            symbols,
            libs,
            rpaths,
            export_trie,
            bind_interpreter,
            entry,
            old_style_entry,
            name,
            ctx,
            is_64,
            little_endian,
            data: bytes,
        })
    }
}

/// A Mach-o multi architecture (Fat) binary container
pub struct MultiArch<'a> {
    data: &'a [u8],
    start: usize,
    pub narches: usize,
}

/// Iterator over the fat architecture headers in a `MultiArch` container
pub struct FatArchIterator<'a> {
    index: usize,
    data: &'a [u8],
    narches: usize,
    start: usize,
}

/// A single architecture froma multi architecture binary container
/// ([MultiArch]).
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum SingleArch<'a> {
    MachO(MachO<'a>),
    Archive(archive::Archive<'a>),
}

impl<'a> Iterator for FatArchIterator<'a> {
    type Item = error::Result<fat::FatArch>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.narches {
            None
        } else {
            let offset = (self.index * fat::SIZEOF_FAT_ARCH) + self.start;
            let arch = self
                .data
                .pread_with::<fat::FatArch>(offset, scroll::BE)
                .map_err(core::convert::Into::into);
            self.index += 1;
            Some(arch)
        }
    }
}

/// Iterator over every entry contained in this `MultiArch` container
pub struct SingleArchIterator<'a> {
    index: usize,
    data: &'a [u8],
    narches: usize,
    start: usize,
}

pub fn peek_bytes(bytes: &[u8; 16]) -> error::Result<crate::Hint> {
    if &bytes[0..archive::SIZEOF_MAGIC] == archive::MAGIC {
        Ok(crate::Hint::Archive)
    } else {
        let (magic, maybe_ctx) = parse_magic_and_ctx(bytes, 0)?;
        match magic {
            header::MH_CIGAM_64 | header::MH_CIGAM | header::MH_MAGIC_64 | header::MH_MAGIC => {
                if let Some(ctx) = maybe_ctx {
                    Ok(crate::Hint::Mach(crate::HintData {
                        is_lsb: ctx.le.is_little(),
                        is_64: Some(ctx.container.is_big()),
                    }))
                } else {
                    Err(error::Error::Malformed(format!(
                        "Correct mach magic {:#x} does not have a matching parsing context!",
                        magic
                    )))
                }
            }
            fat::FAT_MAGIC => {
                // should probably verify this is always Big Endian...
                let narchitectures = bytes.pread_with::<u32>(4, BE)? as usize;
                Ok(crate::Hint::MachFat(narchitectures))
            }
            _ => Ok(crate::Hint::Unknown(bytes.pread::<u64>(0)?)),
        }
    }
}

fn extract_multi_entry(bytes: &[u8]) -> error::Result<SingleArch> {
    if let Some(hint_bytes) = take_hint_bytes(bytes) {
        match peek_bytes(hint_bytes)? {
            crate::Hint::Mach(_) => {
                let binary = MachO::parse(bytes, 0)?;
                Ok(SingleArch::MachO(binary))
            }
            crate::Hint::Archive => {
                let archive = archive::Archive::parse(bytes)?;
                Ok(SingleArch::Archive(archive))
            }
            _ => Err(error::Error::Malformed(format!(
                "multi-arch entry must be a Mach-O binary or an archive"
            ))),
        }
    } else {
        Err(error::Error::Malformed(format!("Object is too small")))
    }
}

impl<'a> Iterator for SingleArchIterator<'a> {
    type Item = error::Result<SingleArch<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.narches {
            None
        } else {
            let index = self.index;
            let offset = (index * fat::SIZEOF_FAT_ARCH) + self.start;
            self.index += 1;
            match self.data.pread_with::<fat::FatArch>(offset, scroll::BE) {
                Ok(arch) => {
                    let bytes = arch.slice(self.data);
                    Some(extract_multi_entry(bytes))
                }
                Err(e) => Some(Err(e.into())),
            }
        }
    }
}

impl<'a, 'b> IntoIterator for &'b MultiArch<'a> {
    type Item = error::Result<SingleArch<'a>>;
    type IntoIter = SingleArchIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        SingleArchIterator {
            index: 0,
            data: self.data,
            narches: self.narches,
            start: self.start,
        }
    }
}

impl<'a> MultiArch<'a> {
    /// Lazily construct `Self`
    pub fn new(bytes: &'a [u8]) -> error::Result<Self> {
        let header = fat::FatHeader::parse(bytes)?;
        Ok(MultiArch {
            data: bytes,
            start: fat::SIZEOF_FAT_HEADER,
            narches: header.nfat_arch as usize,
        })
    }
    /// Iterate every fat arch header
    pub fn iter_arches(&self) -> FatArchIterator {
        FatArchIterator {
            index: 0,
            data: self.data,
            narches: self.narches,
            start: self.start,
        }
    }
    /// Return all the architectures in this binary
    pub fn arches(&self) -> error::Result<Vec<fat::FatArch>> {
        if self.narches > self.data.len() / fat::SIZEOF_FAT_ARCH {
            return Err(error::Error::BufferTooShort(self.narches, "arches"));
        }

        let mut arches = Vec::with_capacity(self.narches);
        for arch in self.iter_arches() {
            arches.push(arch?);
        }
        Ok(arches)
    }
    /// Try to get the Mach-o binary at `index`
    pub fn get(&self, index: usize) -> error::Result<SingleArch<'a>> {
        if index >= self.narches {
            return Err(error::Error::Malformed(format!(
                "Requested the {}-th binary, but there are only {} architectures in this container",
                index, self.narches
            )));
        }
        let offset = (index * fat::SIZEOF_FAT_ARCH) + self.start;
        let arch = self.data.pread_with::<fat::FatArch>(offset, scroll::BE)?;
        let bytes = arch.slice(self.data);
        extract_multi_entry(bytes)
    }

    pub fn find<F: Fn(error::Result<fat::FatArch>) -> bool>(
        &'a self,
        f: F,
    ) -> Option<error::Result<SingleArch<'a>>> {
        for (i, arch) in self.iter_arches().enumerate() {
            if f(arch) {
                return Some(self.get(i));
            }
        }
        None
    }
    /// Try and find the `cputype` in `Self`, if there is one
    pub fn find_cputype(&self, cputype: u32) -> error::Result<Option<fat::FatArch>> {
        for arch in self.iter_arches() {
            let arch = arch?;
            if arch.cputype == cputype {
                return Ok(Some(arch));
            }
        }
        Ok(None)
    }
}

impl<'a> fmt::Debug for MultiArch<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("MultiArch")
            .field("arches", &self.arches().unwrap_or_default())
            .field("data", &self.data.len())
            .finish()
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
/// Either a collection of multiple architectures, or a single mach-o binary
pub enum Mach<'a> {
    /// A "fat" multi-architecture binary container
    Fat(MultiArch<'a>),
    /// A regular Mach-o binary
    Binary(MachO<'a>),
}

impl<'a> Mach<'a> {
    /// Parse from `bytes` either a multi-arch binary or a regular mach-o binary
    pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
        Self::parse_impl(bytes, false)
    }

    /// Parse from `bytes` either a multi-arch binary or a regular mach-o binary in lossy mode
    pub fn parse_lossy(bytes: &'a [u8]) -> error::Result<Self> {
        Self::parse_impl(bytes, true)
    }

    /// Parse from `bytes` either a multi-arch binary or a regular mach-o binary
    fn parse_impl(bytes: &'a [u8], lossy: bool) -> error::Result<Self> {
        let size = bytes.len();
        if size < 4 {
            let error = error::Error::Malformed("size is smaller than a magical number".into());
            return Err(error);
        }
        let magic = peek(&bytes, 0)?;
        match magic {
            fat::FAT_MAGIC => {
                let multi = MultiArch::new(bytes)?;
                Ok(Mach::Fat(multi))
            }
            // we might be a regular binary
            _ => {
                let binary = MachO::parse_impl(bytes, 0, lossy)?;
                Ok(Mach::Binary(binary))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Mach, SingleArch};

    #[test]
    fn parse_multi_arch_of_macho_binaries() {
        // Create via:
        // clang -arch arm64 -shared -o /tmp/hello_world_arm hello_world.c
        // clang -arch x86_64 -shared -o /tmp/hello_world_x86_64 hello_world.c
        // lipo -create -output hello_world_fat_binaries /tmp/hello_world_arm /tmp/hello_world_x86_64
        // strip hello_world_fat_binaries
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/hello_world_fat_binaries"
        ));
        let mach = Mach::parse(bytes).expect("failed to parse input file");
        match mach {
            Mach::Fat(fat) => {
                assert!(fat.into_iter().count() > 0);
                for entry in fat.into_iter() {
                    let entry = entry.expect("failed to read entry");
                    match entry {
                        SingleArch::MachO(macho) => {
                            assert!(macho.symbols().count() > 0);
                        }
                        _ => panic!("expected MultiArchEntry::MachO, got {:?}", entry),
                    }
                }
            }
            Mach::Binary(_) => panic!("expected Mach::Fat, got Mach::Binary"),
        }
    }

    #[test]
    fn parse_multi_arch_of_archives() {
        // Created with:
        // clang -c -o /tmp/hello_world.o hello_world.c
        // ar -r /tmp/hello_world.a /tmp/hello_world.o
        // lipo -create -output hello_world_fat_archives /tmp/hello_world.a
        // strip hello_world_fat_archives
        let bytes = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/hello_world_fat_archives"
        ));
        let mach = Mach::parse(bytes).expect("failed to parse input file");
        match mach {
            Mach::Fat(fat) => {
                assert!(fat.into_iter().count() > 0);
                for entry in fat.into_iter() {
                    let entry = entry.expect("failed to read entry");
                    match entry {
                        SingleArch::Archive(archive) => {
                            assert!(!archive.members().is_empty())
                        }
                        _ => panic!("expected MultiArchEntry::Archive, got {:?}", entry),
                    }
                }
            }
            Mach::Binary(_) => panic!("expected Mach::Fat, got Mach::Binary"),
        }
    }
}
