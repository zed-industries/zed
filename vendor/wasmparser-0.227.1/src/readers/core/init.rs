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

use crate::{BinaryReader, FromReader, OperatorsReader, Result};
use core::fmt;

/// Represents an initialization expression.
#[derive(Clone)]
pub struct ConstExpr<'a> {
    reader: BinaryReader<'a>,
}

impl PartialEq for ConstExpr<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.reader.remaining_buffer() == other.reader.remaining_buffer()
    }
}

impl Eq for ConstExpr<'_> {}

impl<'a> ConstExpr<'a> {
    /// Constructs a new `ConstExpr` from the given data and offset.
    pub fn new(reader: BinaryReader<'a>) -> ConstExpr<'a> {
        ConstExpr { reader }
    }

    /// Gets a binary reader for the initialization expression.
    pub fn get_binary_reader(&self) -> BinaryReader<'a> {
        self.reader.clone()
    }

    /// Gets an operators reader for the initialization expression.
    pub fn get_operators_reader(&self) -> OperatorsReader<'a> {
        OperatorsReader::new(self.get_binary_reader())
    }
}

impl<'a> FromReader<'a> for ConstExpr<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        // FIXME(#188) ideally shouldn't need to skip here
        let reader = reader.skip(|r| r.skip_const_expr())?;
        Ok(ConstExpr { reader })
    }
}

impl fmt::Debug for ConstExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstExpr")
            .field("offset", &self.reader.original_position())
            .field("data", &self.reader.remaining_buffer())
            .finish()
    }
}
