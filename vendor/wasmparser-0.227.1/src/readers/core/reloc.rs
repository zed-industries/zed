use crate::{BinaryReader, FromReader, Result, SectionLimited};
use core::ops::Range;

/// Reader for relocation entries within a `reloc.*` section.
pub type RelocationEntryReader<'a> = SectionLimited<'a, RelocationEntry>;

/// Reader for reloc.* sections as defined by
/// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.
#[derive(Debug, Clone)]
pub struct RelocSectionReader<'a> {
    section: u32,
    range: Range<usize>,
    entries: SectionLimited<'a, RelocationEntry>,
}

impl<'a> RelocSectionReader<'a> {
    /// Creates a new reader for a `reloc.*` section starting at
    /// `original_position` within the wasm file.
    pub fn new(mut reader: BinaryReader<'a>) -> Result<Self> {
        let range = reader.range().clone();
        let section = reader.read_var_u32()?;
        Ok(Self {
            section,
            range,
            entries: SectionLimited::new(reader.shrink())?,
        })
    }

    /// Index of section to which the relocations apply.
    pub fn section_index(&self) -> u32 {
        self.section
    }

    /// The byte range of the entire section.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// The relocation entries.
    pub fn entries(&self) -> SectionLimited<'a, RelocationEntry> {
        self.entries.clone()
    }
}

macro_rules! back_to_enum {
    ($(#[$meta:meta])+ $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl TryFrom<u8> for $name {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {

    /// Relocation entry type. Each entry type corresponds to one of the
    /// `R_WASM_*` constants defined at
    /// <https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/WasmRelocs.def>
    /// and
    /// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    #[repr(u8)]
    pub enum RelocationType {
        /// A function index encoded as a 5-byte varuint32. Used for the
        /// immediate argument of a call instruction. (since LLVM 10.0)
        FunctionIndexLeb = 0,

        /// A function table index encoded as a 5-byte varint32. Used to refer
        /// to the immediate argument of a i32.const instruction, e.g. taking
        /// the address of a function. (since LLVM 10.0)
        TableIndexSleb = 1,

        /// A function table index encoded as a uint32, e.g. taking the address
        /// of a function in a static data initializer. (since LLVM 10.0)
        TableIndexI32 = 2,

        /// A linear memory index encoded as a 5-byte varuint32. Used for the
        /// immediate argument of a load or store instruction, e.g. directly
        /// loading from or storing to a C++ global. (since LLVM 10.0)
        MemoryAddrLeb = 3,

        /// A linear memory index encoded as a 5-byte varint32. Used for the
        /// immediate argument of a i32.const instruction, e.g. taking the
        /// address of a C++ global. (since LLVM 10.0)
        MemoryAddrSleb = 4,

        /// A linear memory index encoded as a uint32, e.g. taking the address
        /// of a C++ global in a static data initializer. (since LLVM 10.0)
        MemoryAddrI32 = 5,

        /// A type index encoded as a 5-byte varuint32, e.g. the type immediate
        /// in a call_indirect. (since LLVM 10.0)
        TypeIndexLeb = 6,

        /// A global index encoded as a 5-byte varuint32, e.g. the index
        /// immediate in a get_global. (since LLVM 10.0)
        GlobalIndexLeb = 7,

        /// A byte offset within code section for the specific function encoded
        /// as a uint32. The offsets start at the actual function code excluding
        /// its size field. (since LLVM 10.0)
        FunctionOffsetI32 = 8,

        /// A byte offset from start of the specified section encoded as a
        /// uint32. (since LLVM 10.0)
        SectionOffsetI32 = 9,

        /// An event index encoded as a 5-byte varuint32. Used for the immediate
        /// argument of a throw and if_except instruction. (since LLVM 10.0)
        EventIndexLeb = 10,

        /// A memory address relative to the __memory_base wasm global. Used in
        /// position independent code (-fPIC) where absolute memory addresses
        /// are not known at link time.
        MemoryAddrRelSleb = 11,

        /// A function address (table index) relative to the __table_base wasm
        /// global. Used in position indepenent code (-fPIC) where absolute
        /// function addresses are not known at link time.
        TableIndexRelSleb = 12,

        /// A global index encoded as uint32. (since LLVM 11.0)
        GlobalIndexI32 = 13,

        /// The 64-bit counterpart of `MemoryAddrLeb`. A 64-bit linear memory
        /// index encoded as a 10-byte varuint64, Used for the immediate
        /// argument of a load or store instruction on a 64-bit linear memory
        /// array. (since LLVM 11.0)
        MemoryAddrLeb64 = 14,

        /// The 64-bit counterpart of `MemoryAddrSleb`. A 64-bit linear memory
        /// index encoded as a 10-byte varint64. Used for the immediate argument
        /// of a i64.const instruction. (since LLVM 11.0)
        MemoryAddrSleb64 = 15,

        /// The 64-bit counterpart of `MemoryAddrI32`. A 64-bit linear memory
        /// index encoded as a uint64, e.g. taking the 64-bit address of a C++
        /// global in a static data initializer. (since LLVM 11.0)
        MemoryAddrI64 = 16,

        /// The 64-bit counterpart of `MemoryAddrRelSleb`.
        MemoryAddrRelSleb64 = 17,

        /// The 64-bit counterpart of `TableIndexSleb`. A function table index
        /// encoded as a 10-byte varint64. Used to refer to the immediate
        /// argument of a i64.const instruction, e.g. taking the address of a
        /// function in Wasm64. (in LLVM 12.0)
        TableIndexSleb64 = 18,

        /// The 64-bit counterpart of `TableIndexI32`. A function table index
        /// encoded as a uint64, e.g. taking the address of a function in a
        /// static data initializer. (in LLVM 12.0)
        TableIndexI64 = 19,

        /// A table number encoded as a 5-byte varuint32. Used for the table
        /// immediate argument in the table.* instructions. (in LLVM 12.0)
        TableNumberLeb = 20,

        /// An offset from the __tls_base symbol encoded as a 5-byte varint32.
        /// Used for PIC case to avoid absolute relocation. (in LLVM 12.0)
        MemoryAddrTlsSleb = 21,

        /// The 64-bit counterpart of `FunctionOffsetI32`. A byte offset within
        /// code section for the specific function encoded as a uint64. (in LLVM
        /// 12.0)
        FunctionOffsetI64 = 22,

        /// A byte offset between the relocating address and a linear memory
        /// index encoded as a uint32. Used for pointer-relative addressing. (in
        /// LLVM 13.0)
        MemoryAddrLocrelI32 = 23,

        /// The 64-bit counterpart of `TableIndexRelSleb`. A function table
        /// index encoded as a 10-byte varint64. (in LLVM 13.0)
        TableIndexRelSleb64 = 24,

        /// The 64-bit counterpart of `MemoryAddrTlsSleb`. (in LLVM 13.0)
        MemoryAddrTlsSleb64 = 25,

        /// A function index encoded as a uint32. Used in custom sections for
        /// function annotations (`__attribute__((annotate(<name>)))`) (in LLVM
        /// 17.0)
        FunctionIndexI32 = 26,
    }

}

impl<'a> FromReader<'a> for RelocationType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let num = reader.read_u8()?;
        num.try_into().or_else(|_| {
            Err(BinaryReader::invalid_leading_byte_error(
                num,
                "RelocEntryType",
                reader.original_position() - 1,
            ))
        })
    }
}

/// Indicates the kind of addend that applies to a relocation entry.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RelocAddendKind {
    /// Relocation entry does not include an addend.
    None,
    /// Relocation entry includes a 32-bit addend.
    Addend32,
    /// Relocation entry includes a 64-bit addend.
    Addend64,
}

/// Single relocation entry within a `reloc.*` section, as defined at
/// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RelocationEntry {
    /// Relocation entry type.
    pub ty: RelocationType,
    /// Offset in bytes from the start of the section indicated by
    /// `RelocSectionReader::section` targetted by this relocation.
    pub offset: u32,
    /// Index in the symbol table contained in the linking section that
    /// corresponds to the value at `offset`.
    pub index: u32,
    /// Addend to add to the address, or `0` if not applicable. The value must
    /// be consistent with the `self.ty.addend_kind()`.
    pub addend: i64,
}

impl RelocationEntry {
    /// Byte range relative to the start of the section indicated by
    /// `RelocSectionReader::section` targetted by this relocation.
    pub fn relocation_range(&self) -> Range<usize> {
        (self.offset as usize)..(self.offset as usize + self.ty.extent())
    }
}

impl RelocationType {
    /// Indicates if this relocation type has an associated `RelocEntry::addend`.
    pub const fn addend_kind(self: Self) -> RelocAddendKind {
        use RelocationType::*;
        match self {
            MemoryAddrLeb | MemoryAddrSleb | MemoryAddrI32 | FunctionOffsetI32
            | SectionOffsetI32 | MemoryAddrLocrelI32 | MemoryAddrRelSleb | MemoryAddrTlsSleb => {
                RelocAddendKind::Addend32
            }
            MemoryAddrRelSleb64 | MemoryAddrTlsSleb64 | MemoryAddrLeb64 | MemoryAddrSleb64
            | MemoryAddrI64 | FunctionOffsetI64 => RelocAddendKind::Addend64,
            _ => RelocAddendKind::None,
        }
    }

    /// Indicates the number of bytes that this relocation type targets.
    pub const fn extent(self) -> usize {
        use RelocationType::*;
        match self {
            FunctionIndexLeb | TableIndexSleb | MemoryAddrLeb | MemoryAddrSleb | TypeIndexLeb
            | GlobalIndexLeb | EventIndexLeb | MemoryAddrRelSleb | TableIndexRelSleb
            | TableNumberLeb | MemoryAddrTlsSleb => 5,
            MemoryAddrLeb64 | MemoryAddrSleb64 | TableIndexSleb64 | TableIndexRelSleb64
            | MemoryAddrRelSleb64 | MemoryAddrTlsSleb64 => 10,

            TableIndexI32 | MemoryAddrI32 | FunctionOffsetI32 | SectionOffsetI32
            | GlobalIndexI32 | MemoryAddrLocrelI32 | FunctionIndexI32 => 4,

            MemoryAddrI64 | TableIndexI64 | FunctionOffsetI64 => 8,
        }
    }
}

impl<'a> FromReader<'a> for RelocationEntry {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let ty = RelocationType::from_reader(reader)?;
        let offset = reader.read_var_u32()?;
        let index = reader.read_var_u32()?;
        let addend = match ty.addend_kind() {
            RelocAddendKind::None => 0,
            RelocAddendKind::Addend32 => reader.read_var_i32()? as i64,
            RelocAddendKind::Addend64 => reader.read_var_i64()?,
        };
        Ok(RelocationEntry {
            ty,
            offset,
            index,
            addend,
        })
    }
}
