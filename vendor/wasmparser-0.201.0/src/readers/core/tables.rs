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

use crate::{BinaryReader, ConstExpr, FromReader, Result, SectionLimited, TableType};

/// A reader for the table section of a WebAssembly module.
pub type TableSectionReader<'a> = SectionLimited<'a, Table<'a>>;

/// Type information about a table defined in the table section of a WebAssembly
/// module.
#[derive(Debug)]
pub struct Table<'a> {
    /// The type of this table, including its element type and its limits.
    pub ty: TableType,
    /// The initialization expression for the table.
    pub init: TableInit<'a>,
}

/// Different modes of initializing a table.
#[derive(Debug)]
pub enum TableInit<'a> {
    /// The table is initialized to all null elements.
    RefNull,
    /// Each element in the table is initialized with the specified constant
    /// expression.
    Expr(ConstExpr<'a>),
}

impl<'a> FromReader<'a> for Table<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let has_init_expr = if reader.peek()? == 0x40 {
            reader.read_u8()?;
            true
        } else {
            false
        };

        if has_init_expr {
            if reader.read_u8()? != 0x00 {
                bail!(reader.original_position() - 1, "invalid table encoding");
            }
        }

        let ty = reader.read::<TableType>()?;
        let init = if has_init_expr {
            TableInit::Expr(reader.read()?)
        } else {
            TableInit::RefNull
        };
        Ok(Table { ty, init })
    }
}

impl<'a> FromReader<'a> for TableType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let element_type = reader.read()?;
        let has_max = match reader.read_u8()? {
            0x00 => false,
            0x01 => true,
            _ => {
                bail!(
                    reader.original_position() - 1,
                    "invalid table resizable limits flags",
                )
            }
        };
        let initial = reader.read()?;
        let maximum = if has_max { Some(reader.read()?) } else { None };
        Ok(TableType {
            element_type,
            initial,
            maximum,
        })
    }
}
