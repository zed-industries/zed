/* Copyright 2018 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{BinaryReader, FromReader, Result, SectionLimited};

/// A reader for the export section of a WebAssembly module.
pub type ExportSectionReader<'a> = SectionLimited<'a, Export<'a>>;

/// External types as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/syntax/types.html#external-types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExternalKind {
    /// The external kind is a function.
    Func,
    /// The external kind if a table.
    Table,
    /// The external kind is a memory.
    Memory,
    /// The external kind is a global.
    Global,
    /// The external kind is a tag.
    Tag,
}

/// Represents an export in a WebAssembly module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Export<'a> {
    /// The name of the exported item.
    pub name: &'a str,
    /// The kind of the export.
    pub kind: ExternalKind,
    /// The index of the exported item.
    pub index: u32,
}

impl<'a> FromReader<'a> for Export<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(Export {
            name: reader.read_string()?,
            kind: reader.read()?,
            index: reader.read_var_u32()?,
        })
    }
}

impl<'a> FromReader<'a> for ExternalKind {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let offset = reader.original_position();
        let byte = reader.read_u8()?;
        BinaryReader::external_kind_from_byte(byte, offset)
    }
}
