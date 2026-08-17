//! Indicate for the type of file described by a header.
//!
//! Each `Header` has an `entry_type` method returning an instance of this type
//! which can be used to inspect what the header is describing.
//!
//! See <https://en.wikipedia.org/wiki/Tar_%28computing%29#UStar_format>

/// A non-exhaustive enum representing the possible entry types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum EntryType {
    /// Regular file
    Regular,
    /// Hard link
    Link,
    /// Symbolic link
    Symlink,
    /// Character device
    Char,
    /// Block device
    Block,
    /// Directory
    Directory,
    /// Named pipe (fifo)
    Fifo,
    /// Implementation-defined 'high-performance' type, treated as regular file
    Continuous,
    /// GNU extension - long file name
    GNULongName,
    /// GNU extension - long link name (link target)
    GNULongLink,
    /// GNU extension - sparse file
    GNUSparse,
    /// Global extended header
    XGlobalHeader,
    /// Extended Header
    XHeader,
    /// Unknown header,
    Other(u8),
}

impl EntryType {
    /// Creates a new entry type from a raw byte.
    ///
    /// Note that the other named constructors of entry type may be more
    /// appropriate to create a file type from.
    pub fn new(byte: u8) -> EntryType {
        match byte {
            b'\x00' | b'0' => EntryType::Regular,
            b'1' => EntryType::Link,
            b'2' => EntryType::Symlink,
            b'3' => EntryType::Char,
            b'4' => EntryType::Block,
            b'5' => EntryType::Directory,
            b'6' => EntryType::Fifo,
            b'7' => EntryType::Continuous,
            b'x' => EntryType::XHeader,
            b'g' => EntryType::XGlobalHeader,
            b'L' => EntryType::GNULongName,
            b'K' => EntryType::GNULongLink,
            b'S' => EntryType::GNUSparse,
            other => EntryType::Other(other),
        }
    }

    /// Returns the raw underlying byte that this entry type represents.
    pub fn as_byte(self) -> u8 {
        match self {
            EntryType::Regular => b'0',
            EntryType::Link => b'1',
            EntryType::Symlink => b'2',
            EntryType::Char => b'3',
            EntryType::Block => b'4',
            EntryType::Directory => b'5',
            EntryType::Fifo => b'6',
            EntryType::Continuous => b'7',
            EntryType::XHeader => b'x',
            EntryType::XGlobalHeader => b'g',
            EntryType::GNULongName => b'L',
            EntryType::GNULongLink => b'K',
            EntryType::GNUSparse => b'S',
            EntryType::Other(other) => other,
        }
    }

    /// Creates a new entry type representing a regular file.
    pub fn file() -> EntryType {
        EntryType::Regular
    }

    /// Creates a new entry type representing a hard link.
    pub fn hard_link() -> EntryType {
        EntryType::Link
    }

    /// Creates a new entry type representing a symlink.
    pub fn symlink() -> EntryType {
        EntryType::Symlink
    }

    /// Creates a new entry type representing a character special device.
    pub fn character_special() -> EntryType {
        EntryType::Char
    }

    /// Creates a new entry type representing a block special device.
    pub fn block_special() -> EntryType {
        EntryType::Block
    }

    /// Creates a new entry type representing a directory.
    pub fn dir() -> EntryType {
        EntryType::Directory
    }

    /// Creates a new entry type representing a FIFO.
    pub fn fifo() -> EntryType {
        EntryType::Fifo
    }

    /// Creates a new entry type representing a contiguous file.
    pub fn contiguous() -> EntryType {
        EntryType::Continuous
    }

    /// Returns whether this type represents a regular file.
    pub fn is_file(self) -> bool {
        self == EntryType::Regular
    }

    /// Returns whether this type represents a hard link.
    pub fn is_hard_link(self) -> bool {
        self == EntryType::Link
    }

    /// Returns whether this type represents a symlink.
    pub fn is_symlink(self) -> bool {
        self == EntryType::Symlink
    }

    /// Returns whether this type represents a character special device.
    pub fn is_character_special(self) -> bool {
        self == EntryType::Char
    }

    /// Returns whether this type represents a block special device.
    pub fn is_block_special(self) -> bool {
        self == EntryType::Block
    }

    /// Returns whether this type represents a directory.
    pub fn is_dir(self) -> bool {
        self == EntryType::Directory
    }

    /// Returns whether this type represents a FIFO.
    pub fn is_fifo(self) -> bool {
        self == EntryType::Fifo
    }

    /// Returns whether this type represents a contiguous file.
    pub fn is_contiguous(self) -> bool {
        self == EntryType::Continuous
    }

    /// Returns whether this type represents a GNU long name header.
    pub fn is_gnu_longname(self) -> bool {
        self == EntryType::GNULongName
    }

    /// Returns whether this type represents a GNU sparse header.
    pub fn is_gnu_sparse(self) -> bool {
        self == EntryType::GNUSparse
    }

    /// Returns whether this type represents a GNU long link header.
    pub fn is_gnu_longlink(self) -> bool {
        self == EntryType::GNULongLink
    }

    /// Returns whether this type represents a GNU long name header.
    pub fn is_pax_global_extensions(self) -> bool {
        self == EntryType::XGlobalHeader
    }

    /// Returns whether this type represents a GNU long link header.
    pub fn is_pax_local_extensions(self) -> bool {
        self == EntryType::XHeader
    }
}
