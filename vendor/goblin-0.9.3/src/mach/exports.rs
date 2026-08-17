//! Symbols exported by this binary and available for dynamic linking are encoded in mach-o binaries using a special trie
//!
//! **Note**: the trie is constructed lazily in case it won't be used, and since computing exports will require allocation, to compute the exports, you need call the export trie's [exports()](struct.ExportTrie.html#method.exports) method.

// TODO:
// (1) Weak of regular_symbol_info type probably needs to be added ?
// (3) /usr/lib/libstdc++.6.0.9.dylib has flag 0xc at many offsets... they're weak

use crate::error;
use crate::mach::load_command;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use core::ops::Range;
use scroll::{Pread, Uleb128};

type Flag = u64;

// "The following are used on the flags byte of a terminal node
// in the export information."
pub const EXPORT_SYMBOL_FLAGS_KIND_MASK: Flag = 0x03;
pub const EXPORT_SYMBOL_FLAGS_KIND_REGULAR: Flag = 0x00;
pub const EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE: Flag = 0x02; // this is a symbol not present in the loader.h but only in the dyld compressed image loader source code, and only available with a #def macro for export flags but libobjc. def has this
pub const EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL: Flag = 0x01;
pub const EXPORT_SYMBOL_FLAGS_WEAK_DEFINITION: Flag = 0x04;
pub const EXPORT_SYMBOL_FLAGS_REEXPORT: Flag = 0x08;
pub const EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER: Flag = 0x10;

#[derive(Debug)]
pub enum SymbolKind {
    Regular,
    Absolute,
    ThreadLocal,
    UnknownSymbolKind(Flag),
}

impl SymbolKind {
    pub fn new(kind: Flag) -> SymbolKind {
        match kind & EXPORT_SYMBOL_FLAGS_KIND_MASK {
            0x00 => SymbolKind::Regular,
            0x01 => SymbolKind::ThreadLocal,
            0x02 => SymbolKind::Absolute,
            _ => SymbolKind::UnknownSymbolKind(kind),
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            SymbolKind::Regular => "Regular",
            SymbolKind::Absolute => "Absolute",
            SymbolKind::ThreadLocal => "Thread_LOCAL",
            SymbolKind::UnknownSymbolKind(_k) => "Unknown",
        }
    }
}

#[derive(Debug)]
/// An export can be a regular export, a re-export, or a stub
pub enum ExportInfo<'a> {
    /// A regular exported symbol, which is an address where it is found, and the flags associated with it
    Regular { address: u64, flags: Flag },
    /// if lib_symbol_name None then same symbol name, otherwise reexport of lib_symbol_name with name in the trie
    /// "If the string is zero length, then the symbol is re-export from the specified dylib with the same name"
    Reexport {
        lib: &'a str,
        lib_symbol_name: Option<&'a str>,
        flags: Flag,
    },
    /// If the flags is `EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER`, then following the flags are two `Uleb128`s: the stub offset and the resolver offset. The stub is used by non-lazy pointers.  The resolver is used by lazy pointers and must be called to get the actual address to use
    Stub {
        stub_offset: scroll::Uleb128,
        resolver_offset: scroll::Uleb128,
        flags: Flag,
    },
}

impl<'a> ExportInfo<'a> {
    /// Parse out the export info from `bytes`, at `offset`
    pub fn parse(
        bytes: &'a [u8],
        libs: &[&'a str],
        flags: Flag,
        mut offset: usize,
    ) -> error::Result<ExportInfo<'a>> {
        use self::ExportInfo::*;
        let regular = |offset| -> error::Result<ExportInfo> {
            let address = bytes.pread::<Uleb128>(offset)?;
            Ok(Regular {
                address: address.into(),
                flags,
            })
        };
        let reexport = |mut offset| -> error::Result<ExportInfo<'a>> {
            let lib_ordinal: u64 = {
                let tmp = bytes.pread::<Uleb128>(offset)?;
                offset += tmp.size();
                tmp.into()
            };
            let lib_symbol_name = bytes.pread::<&str>(offset)?;
            let lib = libs[lib_ordinal as usize];
            let lib_symbol_name = if lib_symbol_name == "" {
                None
            } else {
                Some(lib_symbol_name)
            };
            Ok(Reexport {
                lib,
                lib_symbol_name,
                flags,
            })
        };
        match SymbolKind::new(flags) {
            SymbolKind::Regular => {
                if flags & EXPORT_SYMBOL_FLAGS_REEXPORT != 0 {
                    reexport(offset)
                } else if flags & EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER != 0 {
                    // 0x10
                    let stub_offset = bytes.pread::<Uleb128>(offset)?;
                    offset += stub_offset.size();
                    let resolver_offset = bytes.pread::<Uleb128>(offset)?;
                    Ok(Stub {
                        stub_offset,
                        resolver_offset,
                        flags,
                    })
                // else if (flags = kEXPORT_SYMBOL_FLAGS_WEAK_DEFINITION) then (*0x40 unused*)
                } else {
                    regular(offset)
                }
            }
            SymbolKind::ThreadLocal | SymbolKind::Absolute => {
                if flags & EXPORT_SYMBOL_FLAGS_REEXPORT != 0 {
                    reexport(offset)
                } else {
                    regular(offset)
                }
            }
            SymbolKind::UnknownSymbolKind(_kind) => {
                // 0x5f causes errors, but parsing as regular symbol resolves...
                //Err(error::Error::Malformed(format!("Unknown kind {:#x} from flags {:#x} in get_symbol_type at offset {}", kind, flags, offset)))
                regular(offset)
            }
        }
    }
}

#[derive(Debug)]
/// A finalized symbolic export reconstructed from the export trie
pub struct Export<'a> {
    /// The reconsituted export name which dyld matches against
    pub name: String,
    /// The export info in the node data
    pub info: ExportInfo<'a>,
    /// How large this export is
    pub size: usize,
    /// The offset this symbol export is found in the binary
    pub offset: u64,
}

impl<'a> Export<'a> {
    /// Create a new export from `name` and `info`
    pub fn new(name: String, info: ExportInfo<'a>) -> Export<'a> {
        let offset = match info {
            ExportInfo::Regular { address, .. } => address,
            _ => 0x0,
        };
        Export {
            name,
            info,
            size: 0,
            offset,
        }
    }
}

/// An export trie efficiently encodes all of the symbols exported by this binary for dynamic linking
pub struct ExportTrie<'a> {
    data: &'a [u8],
    location: Range<usize>,
}

impl<'a> ExportTrie<'a> {
    #[inline]
    fn walk_nodes(
        &self,
        libs: &[&'a str],
        branches: Vec<(String, usize)>,
        acc: &mut Vec<Export<'a>>,
    ) -> error::Result<()> {
        for (symbol, next_node) in branches {
            self.walk_trie(libs, symbol, next_node, acc)?;
        }
        Ok(())
    }

    // current_symbol can be a str iiuc
    fn walk_branches(
        &self,
        nbranches: usize,
        current_symbol: String,
        mut offset: usize,
    ) -> error::Result<Vec<(String, usize)>> {
        if nbranches > self.data.len() {
            return Err(error::Error::BufferTooShort(nbranches, "branches"));
        }
        let mut branches = Vec::with_capacity(nbranches);
        //println!("\t@{:#x}", *offset);
        for _i in 0..nbranches {
            // additional offset calculations are relative to the base we received
            let offset = &mut offset;
            let string = self.data.pread::<&str>(*offset)?;
            let mut key = current_symbol.clone();
            key.push_str(string);
            // +1 for null terminator
            *offset = *offset + string.len() + 1;
            //println!("\t({}) string_len: {} offset: {:#x}", i, string.len(), *offset);
            // value is relative to export trie base
            let next_node = Uleb128::read(&self.data, offset)? as usize + self.location.start;
            //println!("\t({}) string: {} next_node: {:#x}", _i, key, next_node);
            branches.push((key, next_node));
        }
        Ok(branches)
    }

    fn walk_trie(
        &self,
        libs: &[&'a str],
        current_symbol: String,
        start: usize,
        exports: &mut Vec<Export<'a>>,
    ) -> error::Result<()> {
        if start < self.location.end {
            let mut offset = start;
            let terminal_size = Uleb128::read(&self.data, &mut offset)?;
            // let mut input = String::new();
            // ::std::io::stdin().read_line(&mut input).unwrap();
            // println!("@ {:#x} node: {:#x} current_symbol: {}", start, terminal_size, current_symbol);
            if terminal_size == 0 {
                let nbranches = Uleb128::read(&self.data, &mut offset)? as usize;
                //println!("\t@ {:#x} BRAN {}", *offset, nbranches);
                let branches = self.walk_branches(nbranches, current_symbol, offset)?;
                self.walk_nodes(libs, branches, exports)
            } else {
                // terminal node, but the tricky part is that they can have children...
                let pos = offset;
                let children_start = &mut (pos + terminal_size as usize);
                let nchildren = Uleb128::read(&self.data, children_start)? as usize;
                let flags = Uleb128::read(&self.data, &mut offset)?;
                //println!("\t@ {:#x} TERM {} flags: {:#x}", offset, nchildren, flags);
                let info = ExportInfo::parse(&self.data, libs, flags, offset)?;
                let export = Export::new(current_symbol.clone(), info);
                //println!("\t{:?}", &export);
                exports.push(export);
                if nchildren == 0 {
                    // this branch is done
                    Ok(())
                } else {
                    // more branches to walk
                    let branches =
                        self.walk_branches(nchildren, current_symbol, *children_start)?;
                    self.walk_nodes(libs, branches, exports)
                }
            }
        } else {
            Ok(())
        }
    }

    fn new_impl(bytes: &'a [u8], start: usize, size: usize) -> Self {
        // FIXME: Ideally, this should validate `command`, but the best we can
        // do for now is return an empty `Range`.
        let location = match start
            .checked_add(size)
            .and_then(|end| bytes.get(start..end).map(|_| start..end))
        {
            Some(location) => location,
            None => {
                log::warn!("Invalid `DyldInfo` `command`.");
                0..0
            }
        };
        ExportTrie {
            data: bytes,
            location,
        }
    }

    /// Walk the export trie for symbols exported by this binary, using the provided `libs` to resolve re-exports
    pub fn exports(&self, libs: &[&'a str]) -> error::Result<Vec<Export<'a>>> {
        let offset = self.location.start;
        let current_symbol = String::new();
        let mut exports = Vec::new();
        self.walk_trie(libs, current_symbol, offset, &mut exports)?;
        Ok(exports)
    }

    /// Create a new, lazy, zero-copy export trie from the `DyldInfo` `command`
    pub fn new(bytes: &'a [u8], command: &load_command::DyldInfoCommand) -> Self {
        Self::new_impl(
            bytes,
            command.export_off as usize,
            command.export_size as usize,
        )
    }

    /// Create a new, lazy, zero-copy export trie from the `LinkeditDataCommand` `command`
    pub fn new_from_linkedit_data_command(
        bytes: &'a [u8],
        command: &load_command::LinkeditDataCommand,
    ) -> Self {
        Self::new_impl(bytes, command.dataoff as usize, command.datasize as usize)
    }
}

impl<'a> Debug for ExportTrie<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ExportTrie")
            .field("data", &"<... redacted ...>")
            .field(
                "location",
                &format_args!("{:#x}..{:#x}", self.location.start, self.location.end),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_trie() {
        const EXPORTS: [u8; 64] = [
            0x00, 0x01, 0x5f, 0x00, 0x05, 0x00, 0x02, 0x5f, 0x6d, 0x68, 0x5f, 0x65, 0x78, 0x65,
            0x63, 0x75, 0x74, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x00, 0x1f, 0x6d,
            0x61, 0x00, 0x23, 0x02, 0x00, 0x00, 0x00, 0x00, 0x02, 0x78, 0x69, 0x6d, 0x75, 0x6d,
            0x00, 0x30, 0x69, 0x6e, 0x00, 0x35, 0x03, 0x00, 0xc0, 0x1e, 0x00, 0x03, 0x00, 0xd0,
            0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let exports = &EXPORTS[..];
        let libs = vec!["/usr/lib/libderp.so", "/usr/lib/libthuglife.so"];
        let mut command = load_command::DyldInfoCommand::default();
        command.export_size = exports.len() as u32;
        let trie = ExportTrie::new(&exports, &command);
        println!("trie: {:#?}", &trie);
        let exports = trie.exports(&libs).unwrap();
        println!("len: {} exports: {:#?}", exports.len(), &exports);
        assert_eq!(exports.len() as usize, 3usize)
    }

    #[test]
    fn export_trie_linkedit_data() {
        const EXPORTS: [u8; 64] = [
            0x00, 0x01, 0x5f, 0x00, 0x05, 0x00, 0x02, 0x5f, 0x6d, 0x68, 0x5f, 0x65, 0x78, 0x65,
            0x63, 0x75, 0x74, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x00, 0x1f, 0x6d,
            0x61, 0x00, 0x23, 0x02, 0x00, 0x00, 0x00, 0x00, 0x02, 0x78, 0x69, 0x6d, 0x75, 0x6d,
            0x00, 0x30, 0x69, 0x6e, 0x00, 0x35, 0x03, 0x00, 0xc0, 0x1e, 0x00, 0x03, 0x00, 0xd0,
            0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let exports = &EXPORTS[..];
        let libs = vec!["/usr/lib/libderp.so", "/usr/lib/libthuglife.so"];
        let command = load_command::LinkeditDataCommand {
            datasize: exports.len() as u32,
            ..Default::default()
        };
        let trie = ExportTrie::new_from_linkedit_data_command(exports, &command);
        println!("trie: {:#?}", &trie);
        let exports = trie.exports(&libs).unwrap();
        println!("len: {} exports: {:#?}", exports.len(), &exports);
        assert_eq!(exports.len() as usize, 3usize);
    }

    #[test]
    fn invalid_range() {
        let mut command = load_command::DyldInfoCommand::default();
        command.export_off = 0xffff_ff00;
        command.export_size = 0x00ff_ff00;
        let trie = ExportTrie::new(&[], &command);
        // FIXME: it would have been nice if this were an `Err`.
        let exports = trie.exports(&[]).unwrap();
        assert_eq!(exports.len(), 0);
    }
}
