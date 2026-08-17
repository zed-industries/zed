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

use crate::{BinaryReader, BinaryReaderError, ConstExpr, FromReader, Result, SectionLimited};
use std::ops::Range;

/// Represents a data segment in a core WebAssembly module.
#[derive(Debug, Clone)]
pub struct Data<'a> {
    /// The kind of data segment.
    pub kind: DataKind<'a>,
    /// The data of the data segment.
    pub data: &'a [u8],
    /// The range of the data segment.
    pub range: Range<usize>,
}

/// The kind of data segment.
#[derive(Debug, Copy, Clone)]
pub enum DataKind<'a> {
    /// The data segment is passive.
    Passive,
    /// The data segment is active.
    Active {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The initialization expression for the data segment.
        offset_expr: ConstExpr<'a>,
    },
}

/// A reader for the data section of a WebAssembly module.
pub type DataSectionReader<'a> = SectionLimited<'a, Data<'a>>;

impl<'a> FromReader<'a> for Data<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let segment_start = reader.original_position();

        // The current handling of the flags is largely specified in the `bulk-memory` proposal,
        // which at the time this comment is written has been merged to the main specification
        // draft.
        //
        // Notably, this proposal allows multiple different encodings of the memory index 0. `00`
        // and `02 00` are both valid ways to specify the 0-th memory. However it also makes
        // another encoding of the 0-th memory `80 00` no longer valid.
        //
        // We, however maintain this by parsing `flags` as a LEB128 integer. In that case, `80 00`
        // encoding is parsed out as `0` and is therefore assigned a `memidx` 0, even though the
        // current specification draft does not allow for this.
        //
        // See also https://github.com/WebAssembly/spec/issues/1439
        let flags = reader.read_var_u32()?;
        let kind = match flags {
            1 => DataKind::Passive,
            0 | 2 => {
                let memory_index = if flags == 0 {
                    0
                } else {
                    reader.read_var_u32()?
                };
                let offset_expr = reader.read()?;
                DataKind::Active {
                    memory_index,
                    offset_expr,
                }
            }
            _ => {
                return Err(BinaryReaderError::new(
                    "invalid flags byte in data segment",
                    segment_start,
                ));
            }
        };

        let data = reader.read_reader(
            "unexpected end of section or function: data segment extends past end of the section",
        )?;
        Ok(Data {
            kind,
            data: data.remaining_buffer(),
            range: segment_start..data.range().end,
        })
    }
}
