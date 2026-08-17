use crate::prelude::*;
use crate::{
    BinaryReader, BinaryReaderError, FromReader, Result, SectionLimited, Subsection, Subsections,
};
use core::ops::Range;

bitflags::bitflags! {
    /// Flags for WebAssembly symbols.
    ///
    /// These flags correspond to those described in
    /// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
    /// with the `WASM_SYM_*` prefix.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SymbolFlags: u32 {
        /* N.B.:
            Newly added flags should be keep in sync with `print_dylink0_flags`
            in `crates/wasmprinter/src/lib.rs`.
        */
        /// This is a weak symbol.
        const BINDING_WEAK = 1 << 0;
        /// This is a local symbol (this is exclusive with [BINDING_WEAK]).
        const BINDING_LOCAL = 1 << 1;
        /// This is a hidden symbol.
        const VISIBILITY_HIDDEN = 1 << 2;
        /// This symbol is not defined.
        const UNDEFINED = 1 << 4;
        /// This symbol is intended to be exported from the wasm module to the host environment.
        const EXPORTED = 1 << 5;
        /// This symbol uses an explicit symbol name, rather than reusing the name from a wasm import.
        const EXPLICIT_NAME = 1 << 6;
        /// This symbol is intended to be included in the linker output, regardless of whether it is used by the program.
        const NO_STRIP = 1 << 7;
        /// This symbol resides in thread local storage.
        const TLS = 1 << 8;
        /// This symbol represents an absolute address.
        const ABSOLUTE = 1 << 9;
    }

    /// Flags for WebAssembly segments.
    ///
    /// These flags are defined by implementation at the time of writing:
    /// <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/llvm/include/llvm/BinaryFormat/Wasm.h#L391-L394>
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct SegmentFlags: u32 {
        /// The segment contains only null-terminated strings, which allows the linker to perform merging.
        const STRINGS = 0x1;
        /// The segment contains thread-local data.
        const TLS = 0x2;
    }
}

impl<'a> FromReader<'a> for SymbolFlags {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(Self::from_bits_retain(reader.read_var_u32()?))
    }
}

impl<'a> FromReader<'a> for SegmentFlags {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(Self::from_bits_retain(reader.read_var_u32()?))
    }
}

/// A reader for the `linking` custom section of a WebAssembly module.
///
/// This format is currently defined upstream at
/// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>.
#[derive(Debug, Clone)]
pub struct LinkingSectionReader<'a> {
    /// The version of linking metadata contained in this section.
    version: u32,
    /// The subsections in this section.
    subsections: Subsections<'a, Linking<'a>>,
    /// The range of the entire section, including the version.
    range: Range<usize>,
}

/// Represents a reader for segments from the linking custom section.
pub type SegmentMap<'a> = SectionLimited<'a, Segment<'a>>;

/// Represents extra metadata about the data segments.
#[derive(Debug, Copy, Clone)]
pub struct Segment<'a> {
    /// The name for the segment.
    pub name: &'a str,
    /// The required alignment of the segment, encoded as a power of 2.
    pub alignment: u32,
    /// The flags for the segment.
    pub flags: SegmentFlags,
}

impl<'a> FromReader<'a> for Segment<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let name = reader.read_string()?;
        let alignment = reader.read_var_u32()?;
        let flags = reader.read()?;
        Ok(Self {
            name,
            alignment,
            flags,
        })
    }
}

/// Represents a reader for init functions from the linking custom section.
pub type InitFuncMap<'a> = SectionLimited<'a, InitFunc>;

/// Represents an init function in the linking custom section.
#[derive(Debug, Copy, Clone)]
pub struct InitFunc {
    /// The priority of the init function.
    pub priority: u32,
    /// The symbol index of init function (*not* the function index).
    pub symbol_index: u32,
}

impl<'a> FromReader<'a> for InitFunc {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let priority = reader.read_var_u32()?;
        let symbol_index = reader.read_var_u32()?;
        Ok(Self {
            priority,
            symbol_index,
        })
    }
}

/// Represents a reader for COMDAT data from the linking custom section.
pub type ComdatMap<'a> = SectionLimited<'a, Comdat<'a>>;

/// Represents [COMDAT](https://llvm.org/docs/LangRef.html#comdats) data in the linking custom section.
#[derive(Debug, Clone)]
pub struct Comdat<'a> {
    /// The name of this comdat.
    pub name: &'a str,
    /// The flags.
    pub flags: u32,
    /// The member symbols of this comdat.
    pub symbols: SectionLimited<'a, ComdatSymbol>,
}

impl<'a> FromReader<'a> for Comdat<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let name = reader.read_string()?;
        let flags = reader.read_var_u32()?;
        // FIXME(#188) ideally shouldn't need to skip here
        let symbols = reader.skip(|reader| {
            let count = reader.read_var_u32()?;
            for _ in 0..count {
                reader.read::<ComdatSymbol>()?;
            }
            Ok(())
        })?;
        Ok(Self {
            name,
            flags,
            symbols: SectionLimited::new(symbols)?,
        })
    }
}

/// Represents a symbol that is part of a comdat.
#[derive(Debug, Copy, Clone)]
pub struct ComdatSymbol {
    /// The kind of the symbol.
    pub kind: ComdatSymbolKind,
    /// The index of the symbol. Must not be an import.
    pub index: u32,
}

impl<'a> FromReader<'a> for ComdatSymbol {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let kind = reader.read()?;
        let index = reader.read_var_u32()?;
        Ok(Self { kind, index })
    }
}

/// Represents a symbol kind.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ComdatSymbolKind {
    /// The symbol is a data segment.
    Data,
    /// The symbol is a function.
    Func,
    /// The symbol is a global.
    Global,
    /// The symbol is an event.
    Event,
    /// The symbol is a table.
    Table,
    /// The symbol is a section.
    Section,
}

impl<'a> FromReader<'a> for ComdatSymbolKind {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let offset = reader.original_position();
        match reader.read_u8()? {
            0 => Ok(Self::Data),
            1 => Ok(Self::Func),
            2 => Ok(Self::Global),
            3 => Ok(Self::Event),
            4 => Ok(Self::Table),
            5 => Ok(Self::Section),
            k => Err(BinaryReader::invalid_leading_byte_error(
                k,
                "comdat symbol kind",
                offset,
            )),
        }
    }
}

/// Represents a reader for symbol info from the linking custom section.
pub type SymbolInfoMap<'a> = SectionLimited<'a, SymbolInfo<'a>>;

/// Represents extra information about symbols in the linking custom section.
///
/// The symbol flags correspond to those described in
/// <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
/// with the `WASM_SYM_*` prefix.
#[derive(Debug, Copy, Clone)]
pub enum SymbolInfo<'a> {
    /// The symbol is a function.
    Func {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The index of the function corresponding to this symbol.
        index: u32,
        /// The name for the function, if it is defined or uses an explicit name.
        name: Option<&'a str>,
    },
    /// The symbol is a data symbol.
    Data {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The name for the symbol.
        name: &'a str,
        /// The definition of the data symbol, if it is defined.
        symbol: Option<DefinedDataSymbol>,
    },
    /// The symbol is a global.
    Global {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The index of the global corresponding to this symbol.
        index: u32,
        /// The name for the global, if it is defined or uses an explicit name.
        name: Option<&'a str>,
    },
    /// The symbol is a section.
    Section {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The index of the function corresponding to this symbol.
        section: u32,
    },
    /// The symbol is an event.
    Event {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The index of the event corresponding to this symbol.
        index: u32,
        /// The name for the event, if it is defined or uses an explicit name.
        name: Option<&'a str>,
    },
    /// The symbol is a table.
    Table {
        /// The flags for the symbol.
        flags: SymbolFlags,
        /// The index of the table corresponding to this symbol.
        index: u32,
        /// The name for the table, if it is defined or uses an explicit name.
        name: Option<&'a str>,
    },
}

impl<'a> FromReader<'a> for SymbolInfo<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let offset = reader.original_position();
        let kind = reader.read_u8()?;
        let flags: SymbolFlags = reader.read()?;

        let defined = !flags.contains(SymbolFlags::UNDEFINED);
        let explicit_name = flags.contains(SymbolFlags::EXPLICIT_NAME);

        const SYMTAB_FUNCTION: u8 = 0;
        const SYMTAB_DATA: u8 = 1;
        const SYMTAB_GLOBAL: u8 = 2;
        const SYMTAB_SECTION: u8 = 3;
        const SYMTAB_EVENT: u8 = 4;
        const SYMTAB_TABLE: u8 = 5;

        // https://github.com/WebAssembly/wabt/blob/1.0.34/src/binary-writer.cc#L1226
        match kind {
            SYMTAB_FUNCTION | SYMTAB_GLOBAL | SYMTAB_EVENT | SYMTAB_TABLE => {
                let index = reader.read_var_u32()?;
                let name = match defined || explicit_name {
                    true => Some(reader.read_string()?),
                    false => None,
                };
                Ok(match kind {
                    SYMTAB_FUNCTION => Self::Func { flags, index, name },
                    SYMTAB_GLOBAL => Self::Global { flags, index, name },
                    SYMTAB_EVENT => Self::Event { flags, index, name },
                    SYMTAB_TABLE => Self::Table { flags, index, name },
                    _ => unreachable!(),
                })
            }
            SYMTAB_DATA => {
                let name = reader.read_string()?;
                let data = match defined {
                    true => Some(reader.read()?),
                    false => None,
                };
                Ok(Self::Data {
                    flags,
                    name,
                    symbol: data,
                })
            }
            SYMTAB_SECTION => {
                let section = reader.read_var_u32()?;
                Ok(Self::Section { flags, section })
            }
            k => Err(BinaryReader::invalid_leading_byte_error(
                k,
                "symbol kind",
                offset,
            )),
        }
    }
}

/// Represents the metadata about a data symbol defined in the wasm file.
#[derive(Debug, Copy, Clone)]
pub struct DefinedDataSymbol {
    /// The index of the data segment.
    pub index: u32,
    /// The offset within the segment. Must be <= the segment's size.
    pub offset: u32,
    /// The size of the data, which can be zero. `offset + size` must be <= the segment's size.
    pub size: u32,
}

impl<'a> FromReader<'a> for DefinedDataSymbol {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let index = reader.read_var_u32()?;
        let offset = reader.read_var_u32()?;
        let size = reader.read_var_u32()?;
        Ok(Self {
            index,
            offset,
            size,
        })
    }
}

/// Represents a subsection read from the linking custom section.
#[derive(Debug, Clone)]
pub enum Linking<'a> {
    /// Extra metadata about the data segments.
    SegmentInfo(SegmentMap<'a>),
    /// A list of constructor functions to be called at startup.
    InitFuncs(InitFuncMap<'a>),
    /// The [COMDAT](https://llvm.org/docs/LangRef.html#comdats) groups of associated linking objects.
    ComdatInfo(ComdatMap<'a>),
    /// Extra information about the symbols present in the module.
    SymbolTable(SymbolInfoMap<'a>),
    /// An unknown [linking subsection](https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#linking-metadata-section).
    Unknown {
        /// The identifier for this subsection.
        ty: u8,
        /// The contents of this subsection.
        data: &'a [u8],
        /// The range of bytes, relative to the start of the original data
        /// stream, that the contents of this subsection reside in.
        range: Range<usize>,
    },
}

impl<'a> Subsection<'a> for Linking<'a> {
    fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self> {
        let data = reader.remaining_buffer();
        let offset = reader.original_position();
        Ok(match id {
            5 => Self::SegmentInfo(SegmentMap::new(reader)?),
            6 => Self::InitFuncs(InitFuncMap::new(reader)?),
            7 => Self::ComdatInfo(ComdatMap::new(reader)?),
            8 => Self::SymbolTable(SymbolInfoMap::new(reader)?),
            ty => Self::Unknown {
                ty,
                data,
                range: offset..offset + data.len(),
            },
        })
    }
}

impl<'a> LinkingSectionReader<'a> {
    /// Creates a new reader for the linking section contents starting at
    /// `offset` within the original wasm file.
    pub fn new(mut reader: BinaryReader<'a>) -> Result<Self> {
        let range = reader.range();
        let offset = reader.original_position();

        let version = reader.read_var_u32()?;
        if version != 2 {
            return Err(BinaryReaderError::new(
                format!("unsupported linking section version: {}", version),
                offset,
            ));
        }

        let subsections = Subsections::new(reader.shrink());
        Ok(Self {
            version,
            subsections,
            range,
        })
    }

    /// Returns the version of linking metadata contained in this section.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Returns the original byte offset of this section.
    pub fn original_position(&self) -> usize {
        self.subsections.original_position()
    }

    /// Returns the range, as byte offsets, of this section within the original
    /// wasm binary.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Returns the iterator for advancing through the subsections.
    ///
    /// You can also use [`IntoIterator::into_iter`] directly on this type.
    pub fn subsections(&self) -> Subsections<'a, Linking<'a>> {
        self.subsections.clone()
    }
}

impl<'a> IntoIterator for LinkingSectionReader<'a> {
    type Item = Result<Linking<'a>>;
    type IntoIter = Subsections<'a, Linking<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.subsections
    }
}
