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

use crate::{BinaryReader, ConstExpr, FromReader, GlobalType, Result, SectionLimited};

/// Represents a core WebAssembly global.
#[derive(Debug, Clone)]
pub struct Global<'a> {
    /// The global's type.
    pub ty: GlobalType,
    /// The global's initialization expression.
    pub init_expr: ConstExpr<'a>,
}

/// A reader for the global section of a WebAssembly module.
pub type GlobalSectionReader<'a> = SectionLimited<'a, Global<'a>>;

impl<'a> FromReader<'a> for Global<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let ty = reader.read()?;
        let init_expr = reader.read()?;
        Ok(Global { ty, init_expr })
    }
}

impl<'a> FromReader<'a> for GlobalType {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let content_type = reader.read()?;
        let flags = reader.read_u8()?;
        if reader.shared_everything_threads() {
            if flags > 0b11 {
                bail!(reader.original_position() - 1, "malformed global flags")
            }
        } else {
            if flags > 0b1 {
                bail!(
                    reader.original_position() - 1,
                    "malformed mutability -- or shared globals \
                     require the shared-everything-threads proposal"
                )
            }
        }
        Ok(GlobalType {
            content_type,
            mutable: (flags & 0b01) > 0,
            shared: (flags & 0b10) > 0,
        })
    }
}
