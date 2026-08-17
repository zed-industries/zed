//! Implements a simple parser and extractor for a Unix Archive.
//!
//! There are two "common" formats: BSD and SysV
//!
//! This crate currently only implements the SysV version, which essentially postfixes all
//! names in the archive with a / as a sigil for the end of the name, and uses a special symbol
//! index for looking up symbols faster.

use scroll::{Pread, Pwrite, SizeWith};

use crate::error::{Error, Result};
use crate::strtab;

use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use core::usize;

pub const SIZEOF_MAGIC: usize = 8;
/// The magic number of a Unix Archive
pub const MAGIC: &[u8; SIZEOF_MAGIC] = b"!<arch>\x0A";

const SIZEOF_FILE_IDENTIFER: usize = 16;
const SIZEOF_FILE_SIZE: usize = 10;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Pread, Pwrite, SizeWith)]
/// A Unix Archive Header - meta data for the file/byte blob/whatever that follows exactly after.
/// All data is right-padded with spaces ASCII `0x20`. The Binary layout is as follows:
///
/// |Offset|Length|Name                       |Format     |
/// |:-----|:-----|:--------------------------|:----------|
/// |0     |16    |File identifier            |ASCII      |
/// |16    |12    |File modification timestamp|Decimal    |
/// |28    |6     |Owner ID                   |Decimal    |
/// |34    |6     |Group ID                   |Decimal    |
/// |40    |8     |File mode                  |Octal      |
/// |48    |10    |Filesize in bytes          |Decimal    |
/// |58    |2     |Ending characters          |`0x60 0x0A`|
///
/// Byte alignment is according to the following:
/// > Each archive file member begins on an even byte boundary; a newline is inserted between files
/// > if necessary. Nevertheless, the size given reflects the actual size of the file exclusive
/// > of padding.
pub struct MemberHeader {
    /// The identifier, or name for this file/whatever.
    pub identifier: [u8; 16],
    /// The timestamp for when this file was last modified. Base 10 number
    pub timestamp: [u8; 12],
    /// The file's owner's id. Base 10 string number
    pub owner_id: [u8; 6],
    /// The file's group id. Base 10 string number
    pub group_id: [u8; 6],
    /// The file's permissions mode. Base 8 number number
    pub mode: [u8; 8],
    /// The size of this file. Base 10 string number
    pub file_size: [u8; 10],
    /// The file header's terminator, always `0x60 0x0A`
    pub terminator: [u8; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Header<'a> {
    pub name: &'a str,
    pub size: usize,
}

pub const SIZEOF_HEADER: usize = SIZEOF_FILE_IDENTIFER + 12 + 6 + 6 + 8 + SIZEOF_FILE_SIZE + 2;

impl MemberHeader {
    pub fn name(&self) -> Result<&str> {
        Ok(self
            .identifier
            .pread_with::<&str>(0, ::scroll::ctx::StrCtx::Length(SIZEOF_FILE_IDENTIFER))?)
    }
    pub fn size(&self) -> Result<usize> {
        match usize::from_str_radix(
            self.file_size
                .pread_with::<&str>(0, ::scroll::ctx::StrCtx::Length(self.file_size.len()))?
                .trim_end(),
            10,
        ) {
            Ok(file_size) => Ok(file_size),
            Err(err) => Err(Error::Malformed(format!(
                "{:?} Bad file_size in header: {:?}",
                err, self
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a single entry in the archive
pub struct Member<'a> {
    /// The entry header
    pub header: Header<'a>,
    /// File offset from the start of the archive to where the header begins
    pub header_offset: u64,
    /// File offset from the start of the archive to where the file begins
    pub offset: u64,
    /// BSD `ar` members store the filename separately
    bsd_name: Option<&'a str>,
    /// SysV `ar` members store the filename in a string table, a copy of which we hold here
    sysv_name: Option<&'a str>,
}

impl<'a> Member<'a> {
    /// Tries to parse the header in `R`, as well as the offset in `R.
    /// **NOTE** the Seek will be pointing at the first byte of whatever the file is, skipping padding.
    /// This is because just like members in the archive, the data section is 2-byte aligned.
    pub fn parse(buffer: &'a [u8], offset: &mut usize) -> Result<Member<'a>> {
        let header_offset = *offset;
        let name = buffer.pread_with::<&str>(
            *offset,
            ::scroll::ctx::StrCtx::Length(SIZEOF_FILE_IDENTIFER),
        )?;
        let archive_header = buffer.gread::<MemberHeader>(offset)?;
        let mut header = Header {
            name,
            size: archive_header.size()?,
        };

        // skip newline padding if we're on an uneven byte boundary
        if *offset & 1 == 1 {
            *offset += 1;
        }

        let bsd_name = if let Some(len) = Self::bsd_filename_length(name) {
            // there's a filename of length `len` right after the header
            let name = buffer.pread_with::<&str>(
                header_offset + SIZEOF_HEADER,
                ::scroll::ctx::StrCtx::Length(len),
            )?;

            // adjust the offset and size accordingly
            if header.size > len {
                *offset = header_offset + SIZEOF_HEADER + len;
                header.size -= len;

                // the name may have trailing NULs which we don't really want to keep
                Some(name.trim_end_matches('\0'))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Member {
            header,
            header_offset: header_offset as u64,
            offset: *offset as u64,
            bsd_name,
            sysv_name: None,
        })
    }

    /// The size of the Member's content, in bytes. Does **not** include newline padding,
    /// nor the size of the file header.
    pub fn size(&self) -> usize {
        self.header.size
    }

    /// Parse `#1/123` as `Some(123)`
    fn bsd_filename_length(name: &str) -> Option<usize> {
        use core::str::FromStr;

        if let Some(name) = name.strip_prefix("#1/") {
            let trimmed_name = name.trim_end_matches(' ');
            if let Ok(len) = usize::from_str(trimmed_name) {
                Some(len)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// The member name, accounting for SysV and BSD `ar` filename extensions
    pub fn extended_name(&self) -> &'a str {
        if let Some(bsd_name) = self.bsd_name {
            bsd_name
        } else if let Some(ref sysv_name) = self.sysv_name {
            sysv_name
        } else {
            self.header.name.trim_end_matches(' ').trim_end_matches('/')
        }
    }

    /// The untrimmed raw member name, i.e., includes right-aligned space padding and `'/'` end-of-string
    /// identifier
    pub fn raw_name(&self) -> &'a str {
        self.header.name
    }
}

#[derive(Debug, Default)]
/// The special index member signified by the name `'/'`.
/// The data element contains a list of symbol indexes and symbol names, giving their offsets
/// into the archive for a given name.
pub struct Index<'a> {
    /// Big Endian number of symbol_indexes and strings
    pub size: usize,
    /// Big Endian u32 index into the archive for this symbol (index in array is the index into the string table)
    pub symbol_indexes: Vec<u32>,
    /// Set of zero-terminated strings indexed by above. Number of strings = `self.size`
    pub strtab: Vec<&'a str>,
}

/// SysV Archive Variant Symbol Lookup Table "Magic" Name
const INDEX_NAME: &str = "/               ";
/// SysV Archive Variant Extended Filename String Table Name
const NAME_INDEX_NAME: &str = "//              ";
/// BSD symbol definitions
const BSD_SYMDEF_NAME: &str = "__.SYMDEF";
const BSD_SYMDEF_SORTED_NAME: &str = "__.SYMDEF SORTED";

impl<'a> Index<'a> {
    /// Parses the given byte buffer into an Index. NB: the buffer must be the start of the index
    pub fn parse_sysv_index(buffer: &'a [u8]) -> Result<Self> {
        let offset = &mut 0;
        let sizeof_table = buffer.gread_with::<u32>(offset, scroll::BE)? as usize;

        if sizeof_table > buffer.len() / 4 {
            return Err(Error::BufferTooShort(sizeof_table, "indices"));
        }

        let mut indexes = Vec::with_capacity(sizeof_table);
        for _ in 0..sizeof_table {
            indexes.push(buffer.gread_with::<u32>(offset, scroll::BE)?);
        }
        let sizeof_strtab = buffer.len() - ((sizeof_table * 4) + 4);
        let strtab = strtab::Strtab::parse(buffer, *offset, sizeof_strtab, 0x0)?;
        Ok(Index {
            size: sizeof_table,
            symbol_indexes: indexes,
            strtab: strtab.to_vec()?, // because i'm lazy
        })
    }

    /// Parses the given byte buffer into an Index, in BSD style archives
    pub fn parse_bsd_symdef(buffer: &'a [u8]) -> Result<Self> {
        // `llvm-ar` is a suitable reference:
        //   https://github.com/llvm-mirror/llvm/blob/6ea9891f9310510c621be562d1c5cdfcf5575678/lib/Object/Archive.cpp#L842-L870

        // BSD __.SYMDEF files look like:
        //
        //            ┌─────────────┐
        //  entries:  │   # bytes   │
        //            ├─────────────┼─────────────┐
        //            │ name offset │  .o offset  │
        //            ├─────────────┼─────────────┤
        //            │ name offset │  .o offset  │
        //            ├─────────────┼─────────────┤
        //            │ name offset │  .o offset  │
        //            ├─────────────┼─────────────┤
        //            │ name offset │  .o offset  │
        //            ├─────────────┼─────────────┘
        //   strings: │   # bytes   │
        //            ├─────────────┴───────────────────┐
        //            │  _symbol\0                      │
        //            ├─────────────────────────────────┴─────────────────────┐
        //            │  _longer_symbol\0                                     │
        //            ├────────────────┬──────────────────────────────────────┘
        //            │  _baz\0        │
        //            ├────────────────┴───┐
        //            │  _quxx\0           │
        //            └────────────────────┘
        //
        // All numeric values are u32s. Name offsets are relative to the start of the string table,
        // and .o offsets are relative to the the start of the archive.

        // Read the number of entries, which is at the start of the symdef (offset 0)
        let entries_bytes = buffer.pread_with::<u32>(0, scroll::LE)? as usize;
        let entries = entries_bytes / 8;

        // Set up the string table, the length of which is recorded after the entire entries table,
        // (`entries_bytes + 4`), and which starts immediately after that (`entries_bytes + 8`).
        let strtab_bytes = buffer.pread_with::<u32>(entries_bytes + 4, scroll::LE)? as usize;
        let strtab = strtab::Strtab::parse(buffer, entries_bytes + 8, strtab_bytes, 0x0)?;

        if entries_bytes > buffer.len() {
            return Err(Error::BufferTooShort(entries, "entries"));
        }

        // build the index
        let mut indexes = Vec::with_capacity(entries);
        let mut strings = Vec::with_capacity(entries);
        for i in 0..entries {
            // The entries table starts after the original length value (offset 4), and each entry
            // has two u32 values, making them 8 bytes long.
            //
            // Therefore, the `i`th entry starts at offset `(i*8)+4`. The first u32 is at that
            // address, and the second u32 follows 4 bytes later.
            let string_offset: u32 = buffer.pread_with(i * 8 + 4, scroll::LE)?;
            let archive_member: u32 = buffer.pread_with(i * 8 + 8, scroll::LE)?;

            let string = match strtab.get_at(string_offset as usize) {
                Some(result) => Ok(result),
                None => Err(Error::Malformed(format!(
                    "{} entry {} has string offset {}, which is out of bounds",
                    BSD_SYMDEF_NAME, i, string_offset
                ))),
            }?;

            indexes.push(archive_member);
            strings.push(string);
        }

        Ok(Index {
            size: entries,
            symbol_indexes: indexes,
            strtab: strings,
        })
    }

    // Parses Windows Second Linker Member:
    // number of members (m):   4
    // member offsets:          4 * m
    // number of symbols (n):   4
    // symbol member indexes:   2 * n
    // followed by SysV-style string table
    // https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#first-linker-member
    pub fn parse_windows_linker_member(buffer: &'a [u8]) -> Result<Self> {
        let offset = &mut 0;
        let members = buffer.gread_with::<u32>(offset, scroll::LE)? as usize;

        if members > buffer.len() / 4 {
            return Err(Error::BufferTooShort(members, "members"));
        }

        let mut member_offsets = Vec::with_capacity(members);
        for _ in 0..members {
            member_offsets.push(buffer.gread_with::<u32>(offset, scroll::LE)?);
        }

        let symbols = buffer.gread_with::<u32>(offset, scroll::LE)? as usize;

        if symbols > buffer.len() / 2 {
            return Err(Error::BufferTooShort(symbols, "symbols"));
        }

        let mut symbol_offsets = Vec::with_capacity(symbols);
        for _ in 0..symbols {
            if let Some(symbol_offset) =
                member_offsets.get(buffer.gread_with::<u16>(offset, scroll::LE)? as usize - 1)
            {
                symbol_offsets.push(*symbol_offset);
            } else {
                return Err(Error::BufferTooShort(members, "members"));
            }
        }
        let strtab = strtab::Strtab::parse(buffer, *offset, buffer.len() - *offset, 0x0)?;
        Ok(Index {
            size: symbols,
            symbol_indexes: symbol_offsets,
            strtab: strtab.to_vec()?,
        })
    }
}

/// Member names greater than 16 bytes are indirectly referenced using a `/<idx` schema,
/// where `idx` is an offset into a newline delimited string table directly following the `//` member
/// of the archive.
#[derive(Debug, Default)]
struct NameIndex<'a> {
    strtab: strtab::Strtab<'a>,
}

impl<'a> NameIndex<'a> {
    pub fn parse(buffer: &'a [u8], offset: &mut usize, size: usize) -> Result<NameIndex<'a>> {
        // This is a total hack, because strtab returns "" if idx == 0, need to change
        // but previous behavior might rely on this, as ELF strtab's have "" at 0th index...
        let hacked_size = size + 1;
        let strtab = strtab::Strtab::parse(buffer, *offset - 1, hacked_size, b'\n')?;
        // precious time was lost when refactoring because strtab::parse doesn't update the mutable seek...
        *offset += hacked_size - 2;
        Ok(NameIndex { strtab })
    }

    pub fn get(&self, name: &str) -> Result<&'a str> {
        let idx = name.trim_start_matches('/').trim_end();
        match usize::from_str_radix(idx, 10) {
            Ok(idx) => {
                let name = match self.strtab.get_at(idx + 1) {
                    Some(result) => Ok(result),
                    None => Err(Error::Malformed(format!(
                        "Name {} is out of range in archive NameIndex",
                        name
                    ))),
                }?;

                if name != "" {
                    Ok(name.trim_end_matches('/'))
                } else {
                    Err(Error::Malformed(format!(
                        "Could not find {:?} in index",
                        name
                    )))
                }
            }
            Err(_) => Err(Error::Malformed(format!(
                "Bad name index {:?} in index",
                name
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
/// The type of symbol index can be present in an archive. Can serve as an indication of the
/// archive format.
pub enum IndexType {
    /// No symbol index present.
    None,
    /// SystemV/GNU style symbol index, used on Windows as well.
    SysV,
    /// Windows specific extension of SysV symbol index, so called Second Linker Member. Has the
    /// same member name as SysV symbol index but different structure.
    Windows,
    /// BSD style symbol index.
    BSD,
}

// TODO: add pretty printer fmt::Display with number of members, and names of members, along with
// the values of the index symbols once implemented
#[derive(Debug)]
/// An in-memory representation of a parsed Unix Archive
pub struct Archive<'a> {
    // The array of members, which are indexed by the members hash and symbol index.
    // These are in the same order they are found in the file.
    member_array: Vec<Member<'a>>,
    // file name -> member
    members: BTreeMap<&'a str, usize>,
    // symbol -> member
    symbol_index: BTreeMap<&'a str, usize>,
}

impl<'a> Archive<'a> {
    pub fn parse(buffer: &'a [u8]) -> Result<Archive<'a>> {
        let mut magic = [0u8; SIZEOF_MAGIC];
        let offset = &mut 0usize;
        buffer.gread_inout(offset, &mut magic)?;
        if &magic != MAGIC {
            return Err(Error::BadMagic(magic.pread(0)?));
        }
        let mut member_array = Vec::new();
        let mut index = Index::default();
        let mut index_type = IndexType::None;
        let mut sysv_name_index = NameIndex::default();
        while *offset + 1 < buffer.len() {
            // realign the cursor to a word boundary, if it's not on one already
            if *offset & 1 == 1 {
                *offset += 1;
            }

            let member = Member::parse(buffer, offset)?;

            // advance to the next record
            *offset = member.offset as usize + member.size() as usize;

            let name = member.raw_name();
            if name == INDEX_NAME {
                let data: &[u8] = buffer.pread_with(member.offset as usize, member.size())?;
                index = match index_type {
                    IndexType::None => {
                        index_type = IndexType::SysV;
                        Index::parse_sysv_index(data)?
                    }
                    IndexType::SysV => {
                        index_type = IndexType::Windows;
                        // second symbol index is Microsoft's extension of SysV format
                        Index::parse_windows_linker_member(data)?
                    }
                    IndexType::BSD => {
                        return Err(Error::Malformed("SysV index occurs after BSD index".into()))
                    }
                    IndexType::Windows => {
                        return Err(Error::Malformed(
                            "More than two Windows Linker members".into(),
                        ))
                    }
                }
            } else if member.bsd_name == Some(BSD_SYMDEF_NAME)
                || member.bsd_name == Some(BSD_SYMDEF_SORTED_NAME)
            {
                if index_type != IndexType::None {
                    return Err(Error::Malformed("BSD index occurs after SysV index".into()));
                }
                index_type = IndexType::BSD;
                let data: &[u8] = buffer.pread_with(member.offset as usize, member.size())?;
                index = Index::parse_bsd_symdef(data)?;
            } else if name == NAME_INDEX_NAME {
                let mut name_index_offset: usize = member.offset as usize;
                sysv_name_index = NameIndex::parse(buffer, &mut name_index_offset, member.size())?;
            } else {
                // record this as an archive member
                member_array.push(member);
            }
        }

        // preprocess member names
        let mut members = BTreeMap::new();
        let mut member_index_by_offset: BTreeMap<u32, usize> = BTreeMap::new();
        for (i, member) in member_array.iter_mut().enumerate() {
            // copy in any SysV extended names
            if let Ok(sysv_name) = sysv_name_index.get(member.raw_name()) {
                member.sysv_name = Some(sysv_name);
            }

            // build a hashmap by extended name
            let key = member.extended_name();
            members.insert(key, i);

            // build a hashmap translating archive offset into member index
            member_index_by_offset.insert(member.header_offset as u32, i);
        }

        // build the symbol index, translating symbol names into member indexes
        let mut symbol_index: BTreeMap<&str, usize> = BTreeMap::new();
        for (member_offset, name) in index.symbol_indexes.iter().zip(index.strtab.iter()) {
            let member_index = *member_index_by_offset.get(member_offset).ok_or_else(|| {
                Error::Malformed(format!(
                    "Could not get member {:?} at offset: {}",
                    name, member_offset
                ))
            })?;
            symbol_index.insert(&name, member_index);
        }

        Ok(Archive {
            member_array,
            members,
            symbol_index,
        })
    }

    /// Get the member named `member` in this archive, if any. If there are
    /// multiple files in the archive with the same name it only returns one
    /// of them.
    pub fn get(&self, member: &str) -> Option<&Member> {
        if let Some(idx) = self.members.get(member) {
            Some(&self.member_array[*idx])
        } else {
            None
        }
    }

    /// Get the member at position `index` in this archive, if any.
    pub fn get_at(&self, index: usize) -> Option<&Member> {
        self.member_array.get(index)
    }

    /// Return the number of archive members.
    pub fn len(&self) -> usize {
        self.member_array.len()
    }

    /// Returns a slice of the raw bytes for the given `member` in the scrollable `buffer`
    pub fn extract<'b>(&self, member: &str, buffer: &'b [u8]) -> Result<&'b [u8]> {
        if let Some(member) = self.get(member) {
            let bytes = buffer.pread_with(member.offset as usize, member.size())?;
            Ok(bytes)
        } else {
            Err(Error::Malformed(format!(
                "Cannot extract member {:?}",
                member
            )))
        }
    }

    /// Gets a summary of this archive, returning a list of membername, the member, and the list of symbols the member contains
    pub fn summarize(&self) -> Vec<(&str, &Member, Vec<&'a str>)> {
        // build a result array, with indexes matching the member indexes
        let mut result = self
            .member_array
            .iter()
            .map(|ref member| (member.extended_name(), *member, Vec::new()))
            .collect::<Vec<_>>();

        // walk the symbol index once, adding each symbol to the appropriate result Vec
        for (symbol_name, member_index) in self.symbol_index.iter() {
            result[*member_index].2.push(*symbol_name);
        }

        result
    }

    /// Get the list of member names in this archive
    ///
    /// This returns members in alphabetical order, not in the order they
    /// occurred in the archive. If there are multiple files with the same
    /// name, the size of the returned array will be less than the size of
    /// `len()`.
    pub fn members(&self) -> Vec<&'a str> {
        self.members.keys().cloned().collect()
    }

    /// Returns the member's name which contains the given `symbol`, if it is in the archive
    pub fn member_of_symbol(&self, symbol: &str) -> Option<&'a str> {
        if let Some(idx) = self.symbol_index.get(symbol) {
            Some(self.member_array[*idx].extended_name())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_bsd_filename_length() {
        // non-BSD names should fall through
        assert_eq!(Member::bsd_filename_length(""), None);
        assert_eq!(Member::bsd_filename_length("123"), None);
        assert_eq!(Member::bsd_filename_length("#1"), None);
        assert_eq!(Member::bsd_filename_length("#1/"), None);
        assert_eq!(Member::bsd_filename_length("#2/1"), None);
        assert_eq!(Member::bsd_filename_length(INDEX_NAME), None);
        assert_eq!(Member::bsd_filename_length(NAME_INDEX_NAME), None);
        assert_eq!(Member::bsd_filename_length("👺"), None);

        // #1/<len> should be parsed as Some(len), with or without whitespace
        assert_eq!(Member::bsd_filename_length("#1/1"), Some(1));
        assert_eq!(Member::bsd_filename_length("#1/22"), Some(22));
        assert_eq!(Member::bsd_filename_length("#1/333"), Some(333));
        assert_eq!(Member::bsd_filename_length("#1/1          "), Some(1));
        assert_eq!(Member::bsd_filename_length("#1/22         "), Some(22));
        assert_eq!(Member::bsd_filename_length("#1/333      "), Some(333));

        // #!/<len><trailing garbage> should be None
        assert_eq!(Member::bsd_filename_length("#1/1A"), None);
        assert_eq!(Member::bsd_filename_length("#1/1 A"), None);
    }
}
