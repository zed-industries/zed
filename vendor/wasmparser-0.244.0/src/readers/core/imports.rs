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

use core::mem;

use crate::{
    BinaryReader, BinaryReaderError, ExternalKind, FromReader, GlobalType, MemoryType, Result,
    SectionLimited, SectionLimitedIntoIterWithOffsets, TableType, TagType,
};

/// Represents a reference to a type definition in a WebAssembly module.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeRef {
    /// The type is a function.
    Func(u32),
    /// The type is a table.
    Table(TableType),
    /// The type is a memory.
    Memory(MemoryType),
    /// The type is a global.
    Global(GlobalType),
    /// The type is a tag.
    ///
    /// This variant is only used for the exception handling proposal.
    ///
    /// The value is an index in the types index space.
    Tag(TagType),
    /// The type is a function.
    FuncExact(u32),
}

/// Represents a group of imports in a WebAssembly module.
#[derive(Debug, Clone)]
pub enum Imports<'a> {
    /// The group contains a single import.
    Single(usize, Import<'a>),
    /// The group contains many imports that share the same module name, but have different types.
    Compact1 {
        /// The module being imported from.
        module: &'a str,
        /// The imported items.
        items: SectionLimited<'a, ImportItemCompact<'a>>,
    },
    /// The group contains many imports that share the same module name and type.
    Compact2 {
        /// The module each item will be imported from.
        module: &'a str,
        /// The type of the imported items.
        ty: TypeRef,
        /// The imported item names.
        names: SectionLimited<'a, &'a str>,
    },
}

/// Represents an import in a WebAssembly module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Import<'a> {
    /// The module being imported from.
    pub module: &'a str,
    /// The name of the imported item.
    pub name: &'a str,
    /// The type of the imported item.
    pub ty: TypeRef,
}

/// A single compact import item.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ImportItemCompact<'a> {
    /// The name of the imported item.
    pub name: &'a str,
    /// The type of the imported item.
    pub ty: TypeRef,
}

/// A reader for the import section of a WebAssembly module.
pub type ImportSectionReader<'a> = SectionLimited<'a, Imports<'a>>;

impl<'a> FromReader<'a> for Imports<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let start = reader.original_position();
        let module = reader.read_string()?;
        let single_item_name = reader.read_string()?;
        let discriminator = reader.peek_bytes(1)?[0];
        match (single_item_name, discriminator) {
            ("", 0x7F) => {
                if !reader.compact_imports() {
                    bail!(
                        reader.original_position(),
                        "invalid leading byte 0x7F with compact imports \
                         proposal disabled"
                    );
                }
                // Compact encoding 1: one module name, many item names / types
                reader.read_bytes(1)?;
                // FIXME(#188) shouldn't need to skip here
                let items = reader.skip(|reader| {
                    let count = reader.read_var_u32()?;
                    for _ in 0..count {
                        reader.skip_string()?;
                        reader.read::<TypeRef>()?;
                    }
                    Ok(())
                })?;
                Ok(Imports::Compact1 {
                    module,
                    items: SectionLimited::new(items)?,
                })
            }
            ("", 0x7E) => {
                if !reader.compact_imports() {
                    bail!(
                        reader.original_position(),
                        "invalid leading byte 0x7E with compact imports \
                         proposal disabled"
                    );
                }
                // Compact encoding 2: one module name / type, many item names
                reader.read_bytes(1)?;
                let ty: TypeRef = reader.read()?;
                // FIXME(#188) shouldn't need to skip here
                let names = reader.skip(|reader| {
                    let count = reader.read_var_u32()?;
                    for _ in 0..count {
                        reader.skip_string()?;
                    }
                    Ok(())
                })?;
                Ok(Imports::Compact2 {
                    module,
                    ty,
                    names: SectionLimited::new(names)?,
                })
            }
            _ => Ok(Imports::Single(
                start,
                Import {
                    module: module,
                    name: single_item_name,
                    ty: reader.read()?,
                },
            )),
        }
    }
}

impl<'a> FromReader<'a> for Import<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(Import {
            module: reader.read()?,
            name: reader.read()?,
            ty: reader.read()?,
        })
    }
}

impl<'a> FromReader<'a> for ImportItemCompact<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(ImportItemCompact {
            name: reader.read()?,
            ty: reader.read()?,
        })
    }
}

impl<'a> FromReader<'a> for TypeRef {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read()? {
            ExternalKind::Func => TypeRef::Func(reader.read_var_u32()?),
            ExternalKind::FuncExact => TypeRef::FuncExact(reader.read_var_u32()?),
            ExternalKind::Table => TypeRef::Table(reader.read()?),
            ExternalKind::Memory => TypeRef::Memory(reader.read()?),
            ExternalKind::Global => TypeRef::Global(reader.read()?),
            ExternalKind::Tag => TypeRef::Tag(reader.read()?),
        })
    }
}

// Iterator implementations to streamline usage of the Imports type in its
// various possible encodings

impl<'a> SectionLimited<'a, Imports<'a>> {
    /// Converts the section into an iterator over individual [`Import`]s, flattening any groups
    /// of compact imports.
    pub fn into_imports(self) -> impl Iterator<Item = Result<Import<'a>>> {
        self.into_imports_with_offsets()
            .map(|res| res.map(|(_, import)| import))
    }

    /// Converts the section into an iterator over individual [`Import`]s and their offsets,
    /// flattening any groups of compact imports.
    pub fn into_imports_with_offsets(self) -> impl Iterator<Item = Result<(usize, Import<'a>)>> {
        self.into_iter().flat_map(|res| match res {
            Ok(imports) => imports.into_iter(),
            Err(e) => ImportsIter {
                state: ImportsIterState::Error(e),
            },
        })
    }
}

impl<'a> IntoIterator for Imports<'a> {
    type Item = Result<(usize, Import<'a>)>;
    type IntoIter = ImportsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ImportsIter {
            state: match self {
                Imports::Single(start, import) => ImportsIterState::Single(start, import),
                Imports::Compact1 { module, items } => ImportsIterState::Compact1 {
                    module: module,
                    iter: items.into_iter_with_offsets(),
                },
                Imports::Compact2 {
                    module,
                    ty,
                    names: items,
                } => ImportsIterState::Compact2 {
                    module: module,
                    ty: ty,
                    iter: items.into_iter_with_offsets(),
                },
            },
        }
    }
}

/// An iterator over the [`Import`]s in a single [`Imports`] object.
pub struct ImportsIter<'a> {
    state: ImportsIterState<'a>,
}

enum ImportsIterState<'a> {
    Done,
    Error(BinaryReaderError),
    Single(usize, Import<'a>),
    Compact1 {
        module: &'a str,
        iter: SectionLimitedIntoIterWithOffsets<'a, ImportItemCompact<'a>>,
    },
    Compact2 {
        module: &'a str,
        ty: TypeRef,
        iter: SectionLimitedIntoIterWithOffsets<'a, &'a str>,
    },
}

impl<'a> Iterator for ImportsIter<'a> {
    type Item = Result<(usize, Import<'a>)>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            ImportsIterState::Done => None,
            ImportsIterState::Error(_) => {
                let ImportsIterState::Error(e) =
                    mem::replace(&mut self.state, ImportsIterState::Done)
                else {
                    unreachable!()
                };
                Some(Err(e))
            }

            ImportsIterState::Single(offset, i) => {
                let ret = Some(Ok((*offset, *i)));
                self.state = ImportsIterState::Done;
                ret
            }
            ImportsIterState::Compact1 { module, iter } => {
                let item = iter.next()?;
                Some(item.map(|(offset, item)| {
                    (
                        offset,
                        Import {
                            module,
                            name: item.name,
                            ty: item.ty,
                        },
                    )
                }))
            }
            ImportsIterState::Compact2 { module, ty, iter } => {
                let item = iter.next()?;
                Some(item.map(|(offset, name)| {
                    (
                        offset,
                        Import {
                            module,
                            name,
                            ty: *ty,
                        },
                    )
                }))
            }
        }
    }
}
