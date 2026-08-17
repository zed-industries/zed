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

use crate::{BinaryReader, FromReader, OperatorsReader, Result, SectionLimited, ValType};
use core::ops::Range;

/// A reader for the code section of a WebAssembly module.
pub type CodeSectionReader<'a> = SectionLimited<'a, FunctionBody<'a>>;

/// Represents a WebAssembly function body.
#[derive(Debug, Clone)]
pub struct FunctionBody<'a> {
    reader: BinaryReader<'a>,
}

impl<'a> FunctionBody<'a> {
    /// Constructs a new `FunctionBody` for the given data and offset.
    pub fn new(reader: BinaryReader<'a>) -> Self {
        Self { reader }
    }

    /// Gets a binary reader for this function body.
    pub fn get_binary_reader(&self) -> BinaryReader<'a> {
        self.reader.clone()
    }

    fn skip_locals(reader: &mut BinaryReader) -> Result<()> {
        let count = reader.read_var_u32()?;
        for _ in 0..count {
            reader.read_var_u32()?;
            reader.read::<ValType>()?;
        }
        Ok(())
    }

    /// Gets the locals reader for this function body.
    pub fn get_locals_reader(&self) -> Result<LocalsReader<'a>> {
        let mut reader = self.reader.clone();
        let count = reader.read_var_u32()?;
        Ok(LocalsReader { reader, count })
    }

    /// Gets the operators reader for this function body.
    pub fn get_operators_reader(&self) -> Result<OperatorsReader<'a>> {
        let mut reader = self.reader.clone();
        Self::skip_locals(&mut reader)?;
        Ok(OperatorsReader::new(reader))
    }

    /// Gets the range of the function body.
    pub fn range(&self) -> Range<usize> {
        self.reader.range()
    }

    /// Returns the body of this function as a list of bytes.
    ///
    /// Note that the returned bytes start with the function locals declaration.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.reader.remaining_buffer()
    }
}

impl<'a> FromReader<'a> for FunctionBody<'a> {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let reader = reader.read_reader()?;
        Ok(FunctionBody { reader })
    }
}

/// A reader for a function body's locals.
pub struct LocalsReader<'a> {
    reader: BinaryReader<'a>,
    count: u32,
}

impl<'a> LocalsReader<'a> {
    /// Gets the count of locals in the function body.
    pub fn get_count(&self) -> u32 {
        self.count
    }

    /// Gets the original position of the reader.
    pub fn original_position(&self) -> usize {
        self.reader.original_position()
    }

    /// Reads an item from the reader.
    pub fn read(&mut self) -> Result<(u32, ValType)> {
        let count = self.reader.read()?;
        let value_type = self.reader.read()?;
        Ok((count, value_type))
    }
}

impl<'a> IntoIterator for LocalsReader<'a> {
    type Item = Result<(u32, ValType)>;
    type IntoIter = LocalsIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        let count = self.count;
        LocalsIterator {
            reader: self,
            left: count,
            err: false,
        }
    }
}

/// An iterator over locals in a function body.
pub struct LocalsIterator<'a> {
    reader: LocalsReader<'a>,
    left: u32,
    err: bool,
}

impl<'a> Iterator for LocalsIterator<'a> {
    type Item = Result<(u32, ValType)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.err || self.left == 0 {
            return None;
        }
        let result = self.reader.read();
        self.err = result.is_err();
        self.left -= 1;
        Some(result)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.reader.get_count() as usize;
        (count, Some(count))
    }
}
