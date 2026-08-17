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

use crate::{BinaryReader, BinaryReaderError, Result};
use std::fmt;
use std::marker;
use std::ops::Range;

mod component;
mod core;

pub use self::component::*;
pub use self::core::*;

/// A trait implemented for items that can be decoded directly from a
/// `BinaryReader`, or that which can be parsed from the WebAssembly binary
/// format.
///
/// Note that this is also accessible as a [`BinaryReader::read`] method.
pub trait FromReader<'a>: Sized {
    /// Attempts to read `Self` from the provided binary reader, returning an
    /// error if it is unable to do so.
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>;
}

impl<'a> FromReader<'a> for u32 {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        reader.read_var_u32()
    }
}

impl<'a> FromReader<'a> for &'a str {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        reader.read_string()
    }
}

impl<'a, T, U> FromReader<'a> for (T, U)
where
    T: FromReader<'a>,
    U: FromReader<'a>,
{
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok((reader.read()?, reader.read()?))
    }
}

/// A generic structure for reading a section of a WebAssembly binary which has
/// a limited number of items within it.
///
/// Many WebAssembly sections are a count of items followed by that many items.
/// This helper structure can be used to parse these sections and provides
/// an iteration-based API for reading the contents.
///
/// Note that this always implements the [`Clone`] trait to represent the
/// ability to parse the section multiple times.
pub struct SectionLimited<'a, T> {
    reader: BinaryReader<'a>,
    count: u32,
    _marker: marker::PhantomData<T>,
}

impl<'a, T> SectionLimited<'a, T> {
    /// Creates a new section reader from the provided contents.
    ///
    /// The `data` provided here is the data of the section itself that will be
    /// parsed. The `offset` argument is the byte offset, in the original wasm
    /// binary, that the section was found. The `offset` argument is used
    /// for error reporting.
    ///
    /// # Errors
    ///
    /// Returns an error if a 32-bit count couldn't be read from the `data`.
    pub fn new(data: &'a [u8], offset: usize) -> Result<Self> {
        let mut reader = BinaryReader::new_with_offset(data, offset);
        let count = reader.read_var_u32()?;
        Ok(SectionLimited {
            reader,
            count,
            _marker: marker::PhantomData,
        })
    }

    /// Returns the count of total items within this section.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Returns whether the original byte offset of this section.
    pub fn original_position(&self) -> usize {
        self.reader.original_position()
    }

    /// Returns the range, as byte offsets, of this section within the original
    /// wasm binary.
    pub fn range(&self) -> Range<usize> {
        self.reader.range()
    }

    /// Returns an iterator which yields not only each item in this section but
    /// additionally the offset of each item within the section.
    pub fn into_iter_with_offsets(self) -> SectionLimitedIntoIterWithOffsets<'a, T>
    where
        T: FromReader<'a>,
    {
        SectionLimitedIntoIterWithOffsets {
            iter: self.into_iter(),
        }
    }
}

impl<T> Clone for SectionLimited<'_, T> {
    fn clone(&self) -> Self {
        SectionLimited {
            reader: self.reader.clone(),
            count: self.count,
            _marker: self._marker,
        }
    }
}

impl<T> fmt::Debug for SectionLimited<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionLimited")
            .field("count", &self.count)
            .field("range", &self.range())
            .finish()
    }
}

impl<'a, T> IntoIterator for SectionLimited<'a, T>
where
    T: FromReader<'a>,
{
    type Item = Result<T>;
    type IntoIter = SectionLimitedIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SectionLimitedIntoIter {
            remaining: self.count,
            section: self,
            end: false,
        }
    }
}

/// A consuming iterator of a [`SectionLimited`].
///
/// This is created via the [`IntoIterator`] `impl` for the [`SectionLimited`]
/// type.
pub struct SectionLimitedIntoIter<'a, T> {
    section: SectionLimited<'a, T>,
    remaining: u32,
    end: bool,
}

impl<T> SectionLimitedIntoIter<'_, T> {
    /// Returns the current byte offset of the section within this iterator.
    pub fn original_position(&self) -> usize {
        self.section.reader.original_position()
    }
}

impl<'a, T> Iterator for SectionLimitedIntoIter<'a, T>
where
    T: FromReader<'a>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        if self.end {
            return None;
        }
        if self.remaining == 0 {
            self.end = true;
            if self.section.reader.eof() {
                return None;
            }
            return Some(Err(BinaryReaderError::new(
                "section size mismatch: unexpected data at the end of the section",
                self.section.reader.original_position(),
            )));
        }
        let result = self.section.reader.read();
        self.end = result.is_err();
        self.remaining -= 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for SectionLimitedIntoIter<'a, T> where T: FromReader<'a> {}

/// An iterator over a limited section iterator.
pub struct SectionLimitedIntoIterWithOffsets<'a, T> {
    iter: SectionLimitedIntoIter<'a, T>,
}

impl<'a, T> Iterator for SectionLimitedIntoIterWithOffsets<'a, T>
where
    T: FromReader<'a>,
{
    type Item = Result<(usize, T)>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.iter.section.reader.original_position();
        Some(self.iter.next()?.map(|item| (pos, item)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> ExactSizeIterator for SectionLimitedIntoIterWithOffsets<'a, T> where T: FromReader<'a> {}

/// A trait implemented for subsections of another outer section.
///
/// This is currently only used for subsections within custom sections, such as
/// the `name` section of core wasm.
///
/// This is used in conjunction with [`Subsections`].
pub trait Subsection<'a>: Sized {
    /// Converts the section identifier provided with the section contents into
    /// a typed section
    fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>;
}

/// Iterator/reader over the contents of a section which is composed of
/// subsections.
///
/// This reader is used for the core `name` section, for example. This type
/// primarily implements [`Iterator`] for advancing through the sections.
pub struct Subsections<'a, T> {
    reader: BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}

impl<'a, T> Subsections<'a, T> {
    /// Creates a new reader for the specified section contents starting at
    /// `offset` within the original wasm file.
    pub fn new(data: &'a [u8], offset: usize) -> Self {
        Subsections {
            reader: BinaryReader::new_with_offset(data, offset),
            _marker: marker::PhantomData,
        }
    }

    /// Returns whether the original byte offset of this section.
    pub fn original_position(&self) -> usize {
        self.reader.original_position()
    }

    /// Returns the range, as byte offsets, of this section within the original
    /// wasm binary.
    pub fn range(&self) -> Range<usize> {
        self.reader.range()
    }

    fn read(&mut self) -> Result<T>
    where
        T: Subsection<'a>,
    {
        let subsection_id = self.reader.read_u7()?;
        let reader = self.reader.read_reader("unexpected end of section")?;
        T::from_reader(subsection_id, reader)
    }
}

impl<T> Clone for Subsections<'_, T> {
    fn clone(&self) -> Self {
        Subsections {
            reader: self.reader.clone(),
            _marker: self._marker,
        }
    }
}

impl<T> fmt::Debug for Subsections<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subsections")
            .field("range", &self.range())
            .finish()
    }
}

impl<'a, T> Iterator for Subsections<'a, T>
where
    T: Subsection<'a>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        if self.reader.eof() {
            None
        } else {
            Some(self.read())
        }
    }
}
