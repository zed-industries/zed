use crate::error;
use crate::strtab;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use scroll::{ctx, IOread, IOwrite, Pread, Pwrite, SizeWith};

/// Size of a single symbol in the COFF Symbol Table.
pub const COFF_SYMBOL_SIZE: usize = 18;

// Values for `Symbol::section_number`.

/// The symbol record is not yet assigned a section. A `value` of zero
/// indicates that a reference to an external symbol is defined elsewhere.
/// A `value` of non-zero is a common symbol with a size that is specified by the `value`.
pub const IMAGE_SYM_UNDEFINED: i16 = 0;
/// The symbol has an absolute (non-relocatable) `value` and is not an address.
pub const IMAGE_SYM_ABSOLUTE: i16 = -1;
/// The symbol provides general type or debugging information but does not
/// correspond to a section.
pub const IMAGE_SYM_DEBUG: i16 = -2;

// Base types for `Symbol::typ`.

/// No type information or unknown base type. Microsoft tools use this setting
pub const IMAGE_SYM_TYPE_NULL: u16 = 0;
/// No valid type; used with void pointers and functions
pub const IMAGE_SYM_TYPE_VOID: u16 = 1;
/// A character (signed byte)
pub const IMAGE_SYM_TYPE_CHAR: u16 = 2;
/// A 2-byte signed integer
pub const IMAGE_SYM_TYPE_SHORT: u16 = 3;
/// A natural integer type (normally 4 bytes in Windows)
pub const IMAGE_SYM_TYPE_INT: u16 = 4;
/// A 4-byte signed integer
pub const IMAGE_SYM_TYPE_LONG: u16 = 5;
/// A 4-byte floating-point number
pub const IMAGE_SYM_TYPE_FLOAT: u16 = 6;
/// An 8-byte floating-point number
pub const IMAGE_SYM_TYPE_DOUBLE: u16 = 7;
/// A structure
pub const IMAGE_SYM_TYPE_STRUCT: u16 = 8;
/// A union
pub const IMAGE_SYM_TYPE_UNION: u16 = 9;
/// An enumerated type
pub const IMAGE_SYM_TYPE_ENUM: u16 = 10;
/// A member of enumeration (a specific value)
pub const IMAGE_SYM_TYPE_MOE: u16 = 11;
/// A byte; unsigned 1-byte integer
pub const IMAGE_SYM_TYPE_BYTE: u16 = 12;
/// A word; unsigned 2-byte integer
pub const IMAGE_SYM_TYPE_WORD: u16 = 13;
/// An unsigned integer of natural size (normally, 4 bytes)
pub const IMAGE_SYM_TYPE_UINT: u16 = 14;
/// An unsigned 4-byte integer
pub const IMAGE_SYM_TYPE_DWORD: u16 = 15;

// Derived types for `Symbol::typ`.

/// No derived type; the symbol is a simple scalar variable.
pub const IMAGE_SYM_DTYPE_NULL: u16 = 0;
/// The symbol is a pointer to base type.
pub const IMAGE_SYM_DTYPE_POINTER: u16 = 1;
/// The symbol is a function that returns a base type.
pub const IMAGE_SYM_DTYPE_FUNCTION: u16 = 2;
/// The symbol is an array of base type.
pub const IMAGE_SYM_DTYPE_ARRAY: u16 = 3;

pub const IMAGE_SYM_TYPE_MASK: u16 = 0xf;
pub const IMAGE_SYM_DTYPE_SHIFT: usize = 4;

// Values for `Symbol::storage_class`.

/// A special symbol that represents the end of function, for debugging purposes.
pub const IMAGE_SYM_CLASS_END_OF_FUNCTION: u8 = 0xff;
/// No assigned storage class.
pub const IMAGE_SYM_CLASS_NULL: u8 = 0;
/// The automatic (stack) variable.
///
/// The `value` field specifies the stack frame offset.
pub const IMAGE_SYM_CLASS_AUTOMATIC: u8 = 1;
/// A value that Microsoft tools use for external symbols.
///
/// The `value` field indicates the size if the section number is
/// `IMAGE_SYM_UNDEFINED` (0).  If the section number is not zero,
/// then the `value` field specifies the offset within the section.
pub const IMAGE_SYM_CLASS_EXTERNAL: u8 = 2;
/// A static symbol.
///
/// The 'value' field specifies the offset of the symbol within the section.
/// If the `value` field is zero, then the symbol represents a section name.
pub const IMAGE_SYM_CLASS_STATIC: u8 = 3;
/// A register variable.
///
/// The `value` field specifies the register number.
pub const IMAGE_SYM_CLASS_REGISTER: u8 = 4;
/// A symbol that is defined externally.
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: u8 = 5;
/// A code label that is defined within the module.
///
/// The `value` field specifies the offset of the symbol within the section.
pub const IMAGE_SYM_CLASS_LABEL: u8 = 6;
/// A reference to a code label that is not defined.
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u8 = 7;
/// The structure member.
///
/// The `value` field specifies the n th member.
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u8 = 8;
/// A formal argument (parameter) of a function.
///
/// The `value` field specifies the n th argument.
pub const IMAGE_SYM_CLASS_ARGUMENT: u8 = 9;
/// The structure tag-name entry.
pub const IMAGE_SYM_CLASS_STRUCT_TAG: u8 = 10;
/// A union member.
///
/// The `value` field specifies the n th member.
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u8 = 11;
/// The Union tag-name entry.
pub const IMAGE_SYM_CLASS_UNION_TAG: u8 = 12;
/// A Typedef entry.
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: u8 = 13;
/// A static data declaration.
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u8 = 14;
/// An enumerated type tagname entry.
pub const IMAGE_SYM_CLASS_ENUM_TAG: u8 = 15;
/// A member of an enumeration.
///
/// The `value` field specifies the n th member.
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u8 = 16;
/// A register parameter.
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: u8 = 17;
/// A bit-field reference.
///
/// The `value` field specifies the n th bit in the bit field.
pub const IMAGE_SYM_CLASS_BIT_FIELD: u8 = 18;
/// A .bb (beginning of block) or .eb (end of block) record.
///
/// The `value` field is the relocatable address of the code location.
pub const IMAGE_SYM_CLASS_BLOCK: u8 = 100;
/// A value that Microsoft tools use for symbol records that define the extent of a function.
///
/// Records may be begin function (.bf ), end function ( .ef ), and lines in function ( .lf ).
/// For .lf records, the `value` field gives the number of source lines in the function.
/// For .ef records, the `value` field gives the size of the function code.
pub const IMAGE_SYM_CLASS_FUNCTION: u8 = 101;
/// An end-of-structure entry.
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: u8 = 102;
/// The source-file symbol record.
///
/// The symbol is followed by auxiliary records that name the file.
pub const IMAGE_SYM_CLASS_FILE: u8 = 103;
/// A definition of a section (Microsoft tools use STATIC storage class instead).
pub const IMAGE_SYM_CLASS_SECTION: u8 = 104;
/// A weak external.
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u8 = 105;
/// A CLR token symbol.
///
/// The name is an ASCII string that consists of the hexadecimal value of the token.
pub const IMAGE_SYM_CLASS_CLR_TOKEN: u8 = 107;

/// A COFF symbol.
///
/// Unwind information for this function can be loaded with [`ExceptionData::get_unwind_info`].
///
/// [`ExceptionData::get_unwind_info`]: struct.ExceptionData.html#method.get_unwind_info
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct Symbol {
    /// The name of the symbol.
    ///
    /// An array of 8 bytes is used if the name is not more than 8 bytes long.
    /// This array is padded with nulls on the right if the name is less than 8 bytes long.
    ///
    /// For longer names, the first 4 bytes are all zeros, and the second 4 bytes
    /// are an offset into the string table.
    pub name: [u8; 8],
    /// The value that is associated with the symbol.
    ///
    /// The interpretation of this field depends on `section_number` and
    /// `storage_class`. A typical meaning is the relocatable address.
    pub value: u32,
    /// A one-based index into the section table. Zero and negative values have special meanings.
    pub section_number: i16,
    /// A number that represents type.
    ///
    /// Microsoft tools set this field to 0x20 (function) or 0x0 (not a function).
    pub typ: u16,
    /// An enumerated value that represents storage class.
    pub storage_class: u8,
    /// The number of auxiliary symbol table entries that follow this record.
    ///
    /// Each auxiliary record is the same size as a standard symbol-table record (18 bytes),
    /// but rather than define a new symbol, the auxiliary record gives additional information
    /// on the last symbol defined.
    pub number_of_aux_symbols: u8,
}

impl Symbol {
    /// Parse the symbol at the given offset.
    ///
    /// If the symbol has an inline name, then also returns a reference to the name's
    /// location in `bytes`.
    pub fn parse<'a>(bytes: &'a [u8], offset: usize) -> error::Result<(Option<&'a str>, Symbol)> {
        let symbol = bytes.pread::<Symbol>(offset)?;
        let name = if symbol.name[0] != 0 {
            bytes
                .pread_with(offset, ctx::StrCtx::DelimiterUntil(0, 8))
                .ok()
        } else {
            None
        };
        Ok((name, symbol))
    }

    /// Returns the symbol name.
    ///
    /// This may be a reference to an inline name in the symbol, or to
    /// a strtab entry.
    pub fn name<'a>(&'a self, strtab: &'a strtab::Strtab) -> error::Result<&'a str> {
        if let Some(offset) = self.name_offset() {
            strtab.get_at(offset as usize).ok_or_else(|| {
                error::Error::Malformed(format!("Invalid Symbol name offset {:#x}", offset))
            })
        } else {
            Ok(self.name.pread(0)?)
        }
    }

    /// Return the strtab offset of the symbol name.
    ///
    /// Returns `None` if the name is inline.
    pub fn name_offset(&self) -> Option<u32> {
        // Symbol offset starts at the strtable's length, so let's adjust it
        let length_field_size = core::mem::size_of::<u32>() as u32;

        if self.name[0] == 0 {
            self.name
                .pread_with(4, scroll::LE)
                .ok()
                .map(|offset: u32| offset - length_field_size)
        } else {
            None
        }
    }

    /// Set the strtab offset of the symbol name.
    pub fn set_name_offset(&mut self, offset: u32) {
        self.name[..4].copy_from_slice(&[0; 4]);
        self.name.pwrite_with(offset, 4, scroll::LE).unwrap();
    }

    /// Return the base type of the symbol.
    ///
    /// This type uses the `IMAGE_SYM_TYPE_*` definitions.
    pub fn base_type(&self) -> u16 {
        self.typ & IMAGE_SYM_TYPE_MASK
    }

    /// Return the derived type of the symbol.
    ///
    /// This type uses the `IMAGE_SYM_DTYPE_*` definitions.
    pub fn derived_type(&self) -> u16 {
        self.typ >> IMAGE_SYM_DTYPE_SHIFT
    }

    /// Return true for function definitions.
    ///
    /// These symbols use `AuxFunctionDefinition` for auxiliary symbol records.
    pub fn is_function_definition(&self) -> bool {
        self.storage_class == IMAGE_SYM_CLASS_EXTERNAL
            && self.derived_type() == IMAGE_SYM_DTYPE_FUNCTION
            && self.section_number > 0
    }

    /// Return true for weak external symbols.
    ///
    /// These symbols use `AuxWeakExternal` for auxiliary symbol records.
    pub fn is_weak_external(&self) -> bool {
        self.storage_class == IMAGE_SYM_CLASS_WEAK_EXTERNAL
    }

    /// Return true for file symbol records.
    ///
    /// The auxiliary records contain the name of the source code file.
    pub fn is_file(&self) -> bool {
        self.storage_class == IMAGE_SYM_CLASS_FILE
    }

    /// Return true for section definitions.
    ///
    /// These symbols use `AuxSectionDefinition` for auxiliary symbol records.
    pub fn is_section_definition(&self) -> bool {
        self.storage_class == IMAGE_SYM_CLASS_STATIC && self.number_of_aux_symbols > 0
    }
}

/// Auxiliary symbol record for function definitions.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AuxFunctionDefinition {
    /// The symbol-table index of the corresponding `.bf` (begin function) symbol record.
    pub tag_index: u32,
    /// The size of the executable code for the function itself.
    ///
    /// If the function is in its own section, the `size_of_raw_data` in the section header
    /// is greater or equal to this field, depending on alignment considerations.
    pub total_size: u32,
    /// The file offset of the first COFF line-number entry for the function,
    /// or zero if none exists.
    pub pointer_to_line_number: u32,
    /// The symbol-table index of the record for the next function.
    ///
    /// If the function is the last in the symbol table, this field is set to zero.
    pub pointer_to_next_function: u32,
    /// Unused padding.
    pub unused: [u8; 2],
}

/// Auxiliary symbol record for symbols with storage class `IMAGE_SYM_CLASS_FUNCTION`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AuxBeginAndEndFunction {
    /// Unused padding.
    pub unused1: [u8; 4],
    /// The actual ordinal line number within the source file, corresponding
    /// to the `.bf` or `.ef` record.
    pub line_number: u16,
    /// Unused padding.
    pub unused2: [u8; 6],
    /// The symbol-table index of the next `.bf` symbol record.
    ///
    /// If the function is the last in the symbol table, this field is set to zero.
    /// It is not used for `.ef` records.
    pub pointer_to_next_function: u32,
    /// Unused padding.
    pub unused3: [u8; 2],
}

// Values for the `characteristics` field of `AuxWeakExternal`.

/// Indicates that no library search for the symbol should be performed.
pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1;
/// Indicates that a library search for the symbol should be performed.
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2;
/// Indicates that the symbol is an alias for the symbol given by the `tag_index` field.
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3;

/// Auxiliary symbol record for weak external symbols.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AuxWeakExternal {
    /// The symbol-table index of the symbol to be linked if an external definition is not found.
    pub tag_index: u32,
    /// Flags that control how the symbol should be linked.
    pub characteristics: u32,
    /// Unused padding.
    pub unused: [u8; 10],
}

// Values for the `selection` field of `AuxSectionDefinition`.

/// If this symbol is already defined, the linker issues a "multiply defined symbol" error.
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: u8 = 1;
/// Any section that defines the same COMDAT symbol can be linked; the rest are removed.
pub const IMAGE_COMDAT_SELECT_ANY: u8 = 2;
/// The linker chooses an arbitrary section among the definitions for this symbol.
///
/// If all definitions are not the same size, a "multiply defined symbol" error is issued.
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: u8 = 3;
/// The linker chooses an arbitrary section among the definitions for this symbol.
///
/// If all definitions do not match exactly, a "multiply defined symbol" error is issued.
pub const IMAGE_COMDAT_SELECT_EXACT_MATCH: u8 = 4;
/// The section is linked if a certain other COMDAT section is linked.
///
/// This other section is indicated by the `number` field of the auxiliary symbol record
/// for the section definition. This setting is useful for definitions that have components
/// in multiple sections (for example, code in one and data in another), but where all must
/// be linked or discarded as a set. The other section with which this section is associated
/// must be a COMDAT section; it cannot be another associative COMDAT section (that is, the
/// other section cannot have `IMAGE_COMDAT_SELECT_ASSOCIATIVE` set).
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u8 = 5;
/// The linker chooses the largest definition from among all of the definitions for this symbol.
///
/// If multiple definitions have this size, the choice between them is arbitrary.
pub const IMAGE_COMDAT_SELECT_LARGEST: u8 = 6;

/// Auxiliary symbol record for section definitions.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct AuxSectionDefinition {
    /// The size of section data; the same as `size_of_raw_data` in the section header.
    pub length: u32,
    /// The number of relocation entries for the section.
    pub number_of_relocations: u16,
    /// The number of line-number entries for the section.
    pub number_of_line_numbers: u16,
    /// The checksum for communal data.
    ///
    /// It is applicable if the `IMAGE_SCN_LNK_COMDAT` flag is set in the section header.
    pub checksum: u32,
    /// One-based index into the section table for the associated section.
    ///
    /// This is used when the `selection` field is `IMAGE_COMDAT_SELECT_ASSOCIATIVE`.
    pub number: u16,
    /// The COMDAT selection number.
    ///
    /// This is applicable if the section is a COMDAT section.
    pub selection: u8,
    /// Unused padding.
    pub unused: [u8; 3],
}

/// A COFF symbol table.
// TODO: #[derive(Pwrite)] produce unparseable tokens
pub struct SymbolTable<'a> {
    symbols: &'a [u8],
}

impl<'a> SymbolTable<'a> {
    /// Parse a COFF symbol table at the given offset.
    ///
    /// The offset and number of symbols should be from the COFF header.
    pub fn parse(bytes: &'a [u8], offset: usize, number: usize) -> error::Result<SymbolTable<'a>> {
        let symbols = bytes.pread_with(offset, Self::size(number))?;
        Ok(SymbolTable { symbols })
    }

    /// Get the size in bytes of the symbol table.
    pub fn size(number: usize) -> usize {
        number * COFF_SYMBOL_SIZE
    }

    /// Get the symbol at the given index.
    ///
    /// If the symbol has an inline name, then also returns a reference to the name's
    /// location in `bytes`.
    pub fn get(&self, index: usize) -> Option<(Option<&'a str>, Symbol)> {
        let offset = index * COFF_SYMBOL_SIZE;
        Symbol::parse(self.symbols, offset).ok()
    }

    /// Get the auxiliary symbol record for a function definition.
    pub fn aux_function_definition(&self, index: usize) -> Option<AuxFunctionDefinition> {
        let offset = index * COFF_SYMBOL_SIZE;
        self.symbols.pread(offset).ok()
    }

    /// Get the auxiliary symbol record for a `.bf` or `.ef` symbol record.
    pub fn aux_begin_and_end_function(&self, index: usize) -> Option<AuxBeginAndEndFunction> {
        let offset = index * COFF_SYMBOL_SIZE;
        self.symbols.pread(offset).ok()
    }

    /// Get the auxiliary symbol record for a weak external.
    pub fn aux_weak_external(&self, index: usize) -> Option<AuxWeakExternal> {
        let offset = index * COFF_SYMBOL_SIZE;
        self.symbols.pread(offset).ok()
    }

    /// Get the file name from the auxiliary symbol record for a file symbol record.
    pub fn aux_file(&self, index: usize, number: usize) -> Option<&'a str> {
        let offset = index * COFF_SYMBOL_SIZE;
        let length = number * COFF_SYMBOL_SIZE;
        self.symbols
            .pread_with(offset, ctx::StrCtx::DelimiterUntil(0, length))
            .ok()
    }

    /// Get the auxiliary symbol record for a section definition.
    pub fn aux_section_definition(&self, index: usize) -> Option<AuxSectionDefinition> {
        let offset = index * COFF_SYMBOL_SIZE;
        self.symbols.pread(offset).ok()
    }

    /// Return an iterator for the COFF symbols.
    ///
    /// This iterator skips over auxiliary symbol records.
    pub fn iter(&self) -> SymbolIterator<'a> {
        SymbolIterator {
            index: 0,
            symbols: self.symbols,
        }
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for SymbolTable<'a> {
    type Error = error::Error;

    fn try_into_ctx(self, bytes: &mut [u8], _ctx: scroll::Endian) -> Result<usize, Self::Error> {
        bytes.pwrite(self.symbols, 0).map_err(|err| err.into())
    }
}

impl<'a> Debug for SymbolTable<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SymbolTable")
            .field("symbols", &self.iter().collect::<Vec<_>>())
            .finish()
    }
}

/// An iterator for COFF symbols.
///
/// This iterator skips over auxiliary symbol records.
#[derive(Default)]
pub struct SymbolIterator<'a> {
    index: usize,
    symbols: &'a [u8],
}

impl<'a> Iterator for SymbolIterator<'a> {
    type Item = (usize, Option<&'a str>, Symbol);
    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.index * COFF_SYMBOL_SIZE;
        if offset >= self.symbols.len() {
            None
        } else {
            let index = self.index;
            let (name, symbol) = Symbol::parse(self.symbols, offset).ok()?;
            self.index += 1 + symbol.number_of_aux_symbols as usize;
            Some((index, name, symbol))
        }
    }
}
