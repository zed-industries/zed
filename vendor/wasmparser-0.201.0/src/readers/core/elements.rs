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

use crate::{
    BinaryReader, BinaryReaderError, ConstExpr, ExternalKind, FromReader, RefType, Result,
    SectionLimited,
};
use std::ops::Range;

/// Represents a core WebAssembly element segment.
#[derive(Clone)]
pub struct Element<'a> {
    /// The kind of the element segment.
    pub kind: ElementKind<'a>,
    /// The initial elements of the element segment.
    pub items: ElementItems<'a>,
    /// The range of the the element segment.
    pub range: Range<usize>,
}

/// The kind of element segment.
#[derive(Clone)]
pub enum ElementKind<'a> {
    /// The element segment is passive.
    Passive,
    /// The element segment is active.
    Active {
        /// The index of the table being initialized.
        table_index: Option<u32>,
        /// The initial expression of the element segment.
        offset_expr: ConstExpr<'a>,
    },
    /// The element segment is declared.
    Declared,
}

/// Represents the items of an element segment.
#[derive(Clone)]
pub enum ElementItems<'a> {
    /// This element contains function indices.
    Functions(SectionLimited<'a, u32>),
    /// This element contains constant expressions used to initialize the table.
    Expressions(RefType, SectionLimited<'a, ConstExpr<'a>>),
}

/// A reader for the element section of a WebAssembly module.
pub type ElementSectionReader<'a> = SectionLimited<'a, Element<'a>>;

impl<'a> FromReader<'a> for Element<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let elem_start = reader.original_position();
        // The current handling of the flags is largely specified in the `bulk-memory` proposal,
        // which at the time this commend is written has been merged to the main specification
        // draft.
        //
        // Notably, this proposal allows multiple different encodings of the table index 0. `00`
        // and `02 00` are both valid ways to specify the 0-th table. However it also makes
        // another encoding of the 0-th memory `80 00` no longer valid.
        //
        // We, however maintain this support by parsing `flags` as a LEB128 integer. In that case,
        // `80 00` encoding is parsed out as `0` and is therefore assigned a `tableidx` 0, even
        // though the current specification draft does not allow for this.
        //
        // See also https://github.com/WebAssembly/spec/issues/1439
        let flags = reader.read_var_u32()?;
        if (flags & !0b111) != 0 {
            return Err(BinaryReaderError::new(
                "invalid flags byte in element segment",
                reader.original_position() - 1,
            ));
        }
        let kind = if flags & 0b001 != 0 {
            if flags & 0b010 != 0 {
                ElementKind::Declared
            } else {
                ElementKind::Passive
            }
        } else {
            let table_index = if flags & 0b010 == 0 {
                None
            } else {
                Some(reader.read_var_u32()?)
            };
            let offset_expr = reader.read()?;
            ElementKind::Active {
                table_index,
                offset_expr,
            }
        };
        let exprs = flags & 0b100 != 0;
        let ty = if flags & 0b011 != 0 {
            if exprs {
                Some(reader.read()?)
            } else {
                match reader.read()? {
                    ExternalKind::Func => None,
                    _ => {
                        return Err(BinaryReaderError::new(
                            "only the function external type is supported in elem segment",
                            reader.original_position() - 1,
                        ));
                    }
                }
            }
        } else {
            None
        };
        // FIXME(#188) ideally wouldn't have to do skips here
        let data = reader.skip(|reader| {
            let items_count = reader.read_var_u32()?;
            if exprs {
                for _ in 0..items_count {
                    reader.skip_const_expr()?;
                }
            } else {
                for _ in 0..items_count {
                    reader.read_var_u32()?;
                }
            }
            Ok(())
        })?;
        let items = if exprs {
            ElementItems::Expressions(
                ty.unwrap_or(RefType::FUNCREF),
                SectionLimited::new(data.remaining_buffer(), data.original_position())?,
            )
        } else {
            assert!(ty.is_none());
            ElementItems::Functions(SectionLimited::new(
                data.remaining_buffer(),
                data.original_position(),
            )?)
        };

        let elem_end = reader.original_position();
        let range = elem_start..elem_end;

        Ok(Element { kind, items, range })
    }
}
