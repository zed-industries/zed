use crate::prelude::*;
use crate::{BinaryReader, Result, Subsection, Subsections, SymbolFlags};
use core::ops::Range;

/// Parser for the dynamic linking `dylink.0` custom section.
///
/// This format is currently defined upstream at
/// <https://github.com/WebAssembly/tool-conventions/blob/main/DynamicLinking.md>.
pub type Dylink0SectionReader<'a> = Subsections<'a, Dylink0Subsection<'a>>;

const WASM_DYLINK_MEM_INFO: u8 = 1;
const WASM_DYLINK_NEEDED: u8 = 2;
const WASM_DYLINK_EXPORT_INFO: u8 = 3;
const WASM_DYLINK_IMPORT_INFO: u8 = 4;

/// Represents a `WASM_DYLINK_MEM_INFO` field
#[derive(Debug, Copy, Clone)]
pub struct MemInfo {
    /// Size of the memory area the loader should reserve for the module, which
    /// will begin at `env.__memory_base`.
    pub memory_size: u32,

    /// The required alignment of the memory area, in bytes, encoded as a power
    /// of 2.
    pub memory_alignment: u32,

    /// Size of the table area the loader should reserve for the module, which
    /// will begin at `env.__table_base`.
    pub table_size: u32,

    /// The required alignment of the table area, in elements, encoded as a
    /// power of 2.
    pub table_alignment: u32,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ExportInfo<'a> {
    pub name: &'a str,
    pub flags: SymbolFlags,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ImportInfo<'a> {
    pub module: &'a str,
    pub field: &'a str,
    pub flags: SymbolFlags,
}

/// Possible subsections of the `dylink.0` custom section.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Dylink0Subsection<'a> {
    MemInfo(MemInfo),
    Needed(Vec<&'a str>),
    ExportInfo(Vec<ExportInfo<'a>>),
    ImportInfo(Vec<ImportInfo<'a>>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: Range<usize>,
    },
}

impl<'a> Subsection<'a> for Dylink0Subsection<'a> {
    fn from_reader(id: u8, mut reader: BinaryReader<'a>) -> Result<Self> {
        let data = reader.remaining_buffer();
        let offset = reader.original_position();
        Ok(match id {
            WASM_DYLINK_MEM_INFO => Self::MemInfo(MemInfo {
                memory_size: reader.read_var_u32()?,
                memory_alignment: reader.read_var_u32()?,
                table_size: reader.read_var_u32()?,
                table_alignment: reader.read_var_u32()?,
            }),
            WASM_DYLINK_NEEDED => Self::Needed(
                (0..reader.read_var_u32()?)
                    .map(|_| reader.read_string())
                    .collect::<Result<_, _>>()?,
            ),
            WASM_DYLINK_EXPORT_INFO => Self::ExportInfo(
                (0..reader.read_var_u32()?)
                    .map(|_| {
                        Ok(ExportInfo {
                            name: reader.read_string()?,
                            flags: reader.read()?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            ),
            WASM_DYLINK_IMPORT_INFO => Self::ImportInfo(
                (0..reader.read_var_u32()?)
                    .map(|_| {
                        Ok(ImportInfo {
                            module: reader.read_string()?,
                            field: reader.read_string()?,
                            flags: reader.read()?,
                        })
                    })
                    .collect::<Result<_, _>>()?,
            ),
            ty => Self::Unknown {
                ty,
                data,
                range: offset..offset + data.len(),
            },
        })
    }
}
