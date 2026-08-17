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

#[cfg(feature = "simd")]
mod simd;

use crate::prelude::*;
use crate::{limits::*, *};
use core::fmt;
use core::marker;
use core::ops::Range;
use core::str;

pub(crate) const WASM_MAGIC_NUMBER: &[u8; 4] = b"\0asm";

/// A binary reader for WebAssembly modules.
#[derive(Debug, Clone)]
pub struct BinaryReaderError {
    // Wrap the actual error data in a `Box` so that the error is just one
    // word. This means that we can continue returning small `Result`s in
    // registers.
    pub(crate) inner: Box<BinaryReaderErrorInner>,
}

#[derive(Debug, Clone)]
pub(crate) struct BinaryReaderErrorInner {
    pub(crate) message: String,
    pub(crate) kind: BinaryReaderErrorKind,
    pub(crate) offset: usize,
    pub(crate) needed_hint: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryReaderErrorKind {
    Custom,
    Invalid,
}

/// The result for `BinaryReader` operations.
pub type Result<T, E = BinaryReaderError> = core::result::Result<T, E>;

#[cfg(feature = "std")]
impl std::error::Error for BinaryReaderError {}

#[cfg(all(not(feature = "std"), core_error))]
impl core::error::Error for BinaryReaderError {}

impl fmt::Display for BinaryReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (at offset 0x{:x})",
            self.inner.message, self.inner.offset
        )
    }
}

impl BinaryReaderError {
    #[cold]
    pub(crate) fn _new(kind: BinaryReaderErrorKind, message: String, offset: usize) -> Self {
        BinaryReaderError {
            inner: Box::new(BinaryReaderErrorInner {
                kind,
                message,
                offset,
                needed_hint: None,
            }),
        }
    }

    #[cold]
    pub(crate) fn new(message: impl Into<String>, offset: usize) -> Self {
        Self::_new(BinaryReaderErrorKind::Custom, message.into(), offset)
    }

    #[cold]
    pub(crate) fn invalid(msg: &'static str, offset: usize) -> Self {
        Self::_new(BinaryReaderErrorKind::Invalid, msg.into(), offset)
    }

    #[cold]
    pub(crate) fn fmt(args: fmt::Arguments<'_>, offset: usize) -> Self {
        BinaryReaderError::new(args.to_string(), offset)
    }

    #[cold]
    pub(crate) fn eof(offset: usize, needed_hint: usize) -> Self {
        let mut err = BinaryReaderError::new("unexpected end-of-file", offset);
        err.inner.needed_hint = Some(needed_hint);
        err
    }

    pub(crate) fn kind(&mut self) -> BinaryReaderErrorKind {
        self.inner.kind
    }

    /// Get this error's message.
    pub fn message(&self) -> &str {
        &self.inner.message
    }

    /// Get the offset within the Wasm binary where the error occurred.
    pub fn offset(&self) -> usize {
        self.inner.offset
    }

    #[cfg(all(feature = "validate", feature = "component-model"))]
    pub(crate) fn add_context(&mut self, context: String) {
        self.inner.message = format!("{context}\n{}", self.inner.message);
    }

    pub(crate) fn set_message(&mut self, message: &str) {
        self.inner.message = message.to_string();
    }
}

/// A binary reader of the WebAssembly structures and types.
#[derive(Clone, Debug, Hash)]
pub struct BinaryReader<'a> {
    buffer: &'a [u8],
    position: usize,
    original_offset: usize,

    // When the `features` feature is disabled then the `WasmFeatures` type
    // still exists but this field is still omitted. When `features` is
    // disabled then the only constructor of this type is `BinaryReader::new`
    // which documents all known features being active. All known features
    // being active isn't represented by `WasmFeatures` when the feature is
    // disabled so the field is omitted here to prevent accidentally using the
    // wrong listing of features.
    //
    // Feature accessors are defined by `foreach_wasm_feature!` below with a
    // method-per-feature on `BinaryReader` which when the `features` feature
    // is disabled returns `true` by default.
    #[cfg(feature = "features")]
    features: WasmFeatures,
}

impl<'a> BinaryReader<'a> {
    /// Creates a new binary reader which will parse the `data` provided.
    ///
    /// The `original_offset` provided is used for byte offsets in errors that
    /// are generated. That offset is added to the current position in `data`.
    /// This can be helpful when `data` is just a window of a view into a larger
    /// wasm binary perhaps not even entirely stored locally.
    ///
    /// The returned binary reader will have all features known to this crate
    /// enabled. To reject binaries that aren't valid unless a certain feature
    /// is enabled use the [`BinaryReader::new_features`] constructor instead.
    pub fn new(data: &[u8], original_offset: usize) -> BinaryReader {
        BinaryReader {
            buffer: data,
            position: 0,
            original_offset,
            #[cfg(feature = "features")]
            features: WasmFeatures::all(),
        }
    }

    /// Creates a new binary reader which will parse the `data` provided.
    ///
    /// The `original_offset` provided is used for byte offsets in errors that
    /// are generated. That offset is added to the current position in `data`.
    /// This can be helpful when `data` is just a window of a view into a larger
    /// wasm binary perhaps not even entirely stored locally.
    ///
    /// The `features` argument provided controls which WebAssembly features are
    /// active when parsing this data. Wasm features typically don't affect
    /// parsing too too much and are generally more applicable during
    /// validation, but features and proposals will often reinterpret
    /// previously-invalid constructs as now-valid things meaning something
    /// slightly different. This means that invalid bytes before a feature may
    /// now be interpreted differently after a feature is implemented. This
    /// means that the set of activated features can affect what errors are
    /// generated and when they are generated.
    ///
    /// In general it's safe to pass `WasmFeatures::all()` here. There's no
    /// downside to enabling all features while parsing and only enabling a
    /// subset of features during validation.
    ///
    /// Note that the activated set of features does not guarantee that
    /// `BinaryReader` will return an error for disabled features. For example
    /// if SIMD is disabled then SIMD instructions will still be parsed via
    /// [`BinaryReader::visit_operator`]. Validation must still be performed to
    /// provide a strict guarantee that if a feature is disabled that a binary
    /// doesn't leverage the feature. The activated set of features here instead
    /// only affects locations where preexisting bytes are reinterpreted in
    /// different ways with future proposals, such as the `memarg` moving from a
    /// 32-bit offset to a 64-bit offset with the `memory64` proposal.
    #[cfg(feature = "features")]
    pub fn new_features(
        data: &[u8],
        original_offset: usize,
        features: WasmFeatures,
    ) -> BinaryReader {
        BinaryReader {
            buffer: data,
            position: 0,
            original_offset,
            features,
        }
    }

    /// "Shrinks" this binary reader to retain only the buffer left-to-parse.
    ///
    /// The primary purpose of this method is to change the return value of the
    /// `range()` method. That method returns the range of the original buffer
    /// within the wasm binary so calling `range()` on the returned
    /// `BinaryReader` will return a smaller range than if `range()` is called
    /// on `self`.
    ///
    /// Otherwise parsing values from either `self` or the return value should
    /// return the same thing.
    pub(crate) fn shrink(&self) -> BinaryReader<'a> {
        BinaryReader {
            buffer: &self.buffer[self.position..],
            position: 0,
            original_offset: self.original_offset + self.position,
            #[cfg(feature = "features")]
            features: self.features,
        }
    }

    /// Gets the original position of the binary reader.
    #[inline]
    pub fn original_position(&self) -> usize {
        self.original_offset + self.position
    }

    /// Returns the currently active set of wasm features that this reader is
    /// using while parsing.
    ///
    /// For more information see [`BinaryReader::new`].
    #[cfg(feature = "features")]
    pub fn features(&self) -> WasmFeatures {
        self.features
    }

    /// Sets the wasm features active while parsing to the `features` specified.
    ///
    /// For more information see [`BinaryReader::new`].
    #[cfg(feature = "features")]
    pub fn set_features(&mut self, features: WasmFeatures) {
        self.features = features;
    }

    /// Returns a range from the starting offset to the end of the buffer.
    pub fn range(&self) -> Range<usize> {
        self.original_offset..self.original_offset + self.buffer.len()
    }

    pub(crate) fn remaining_buffer(&self) -> &'a [u8] {
        &self.buffer[self.position..]
    }

    fn ensure_has_byte(&self) -> Result<()> {
        if self.position < self.buffer.len() {
            Ok(())
        } else {
            Err(BinaryReaderError::eof(self.original_position(), 1))
        }
    }

    pub(crate) fn ensure_has_bytes(&self, len: usize) -> Result<()> {
        if self.position + len <= self.buffer.len() {
            Ok(())
        } else {
            let hint = self.position + len - self.buffer.len();
            Err(BinaryReaderError::eof(self.original_position(), hint))
        }
    }

    /// Reads a value of type `T` from this binary reader, advancing the
    /// internal position in this reader forward as data is read.
    #[inline]
    pub fn read<T>(&mut self) -> Result<T>
    where
        T: FromReader<'a>,
    {
        T::from_reader(self)
    }

    pub(crate) fn read_u7(&mut self) -> Result<u8> {
        let b = self.read_u8()?;
        if (b & 0x80) != 0 {
            return Err(BinaryReaderError::new(
                "invalid u7",
                self.original_position() - 1,
            ));
        }
        Ok(b)
    }

    pub(crate) fn external_kind_from_byte(byte: u8, offset: usize) -> Result<ExternalKind> {
        match byte {
            0x00 => Ok(ExternalKind::Func),
            0x01 => Ok(ExternalKind::Table),
            0x02 => Ok(ExternalKind::Memory),
            0x03 => Ok(ExternalKind::Global),
            0x04 => Ok(ExternalKind::Tag),
            x => Err(Self::invalid_leading_byte_error(x, "external kind", offset)),
        }
    }

    /// Reads a variable-length 32-bit size from the byte stream while checking
    /// against a limit.
    pub fn read_size(&mut self, limit: usize, desc: &str) -> Result<usize> {
        let pos = self.original_position();
        let size = self.read_var_u32()? as usize;
        if size > limit {
            bail!(pos, "{desc} size is out of bounds");
        }
        Ok(size)
    }

    /// Reads a variable-length 32-bit size from the byte stream while checking
    /// against a limit.
    ///
    /// Then reads that many values of type `T` and returns them as an iterator.
    ///
    /// Note that regardless of how many items are read from the returned
    /// iterator the items will still be parsed from this reader.
    pub fn read_iter<'me, T>(
        &'me mut self,
        limit: usize,
        desc: &str,
    ) -> Result<BinaryReaderIter<'a, 'me, T>>
    where
        T: FromReader<'a>,
    {
        let size = self.read_size(limit, desc)?;
        Ok(BinaryReaderIter {
            remaining: size,
            reader: self,
            _marker: marker::PhantomData,
        })
    }

    fn read_memarg(&mut self, max_align: u8) -> Result<MemArg> {
        let flags_pos = self.original_position();
        let mut flags = self.read_var_u32()?;

        let memory = if self.multi_memory() && flags & (1 << 6) != 0 {
            flags ^= 1 << 6;
            self.read_var_u32()?
        } else {
            0
        };
        let align = if flags >= (1 << 6) {
            return Err(BinaryReaderError::new(
                "malformed memop alignment: alignment too large",
                flags_pos,
            ));
        } else {
            flags as u8
        };
        let offset = if self.memory64() {
            self.read_var_u64()?
        } else {
            u64::from(self.read_var_u32()?)
        };
        Ok(MemArg {
            align,
            max_align,
            offset,
            memory,
        })
    }

    fn read_ordering(&mut self) -> Result<Ordering> {
        let byte = self.read_var_u32()?;
        match byte {
            0 => Ok(Ordering::SeqCst),
            1 => Ok(Ordering::AcqRel),
            x => Err(BinaryReaderError::new(
                &format!("invalid atomic consistency ordering {}", x),
                self.original_position() - 1,
            )),
        }
    }

    fn read_br_table(&mut self) -> Result<BrTable<'a>> {
        let cnt = self.read_size(MAX_WASM_BR_TABLE_SIZE, "br_table")?;
        let reader = self.skip(|reader| {
            for _ in 0..cnt {
                reader.read_var_u32()?;
            }
            Ok(())
        })?;
        let default = self.read_var_u32()?;
        Ok(BrTable {
            reader,
            cnt: cnt as u32,
            default,
        })
    }

    /// Returns whether the `BinaryReader` has reached the end of the file.
    #[inline]
    pub fn eof(&self) -> bool {
        self.position >= self.buffer.len()
    }

    /// Returns the `BinaryReader`'s current position.
    #[inline]
    pub fn current_position(&self) -> usize {
        self.position
    }

    /// Returns the number of bytes remaining in the `BinaryReader`.
    #[inline]
    pub fn bytes_remaining(&self) -> usize {
        self.buffer.len() - self.position
    }

    /// Advances the `BinaryReader` `size` bytes, and returns a slice from the
    /// current position of `size` length.
    ///
    /// # Errors
    /// If `size` exceeds the remaining length in `BinaryReader`.
    pub fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        self.ensure_has_bytes(size)?;
        let start = self.position;
        self.position += size;
        Ok(&self.buffer[start..self.position])
    }

    /// Reads a length-prefixed list of bytes from this reader and returns a
    /// new `BinaryReader` to read that list of bytes.
    pub fn read_reader(&mut self) -> Result<BinaryReader<'a>> {
        let size = self.read_var_u32()? as usize;
        self.skip(|reader| {
            reader.read_bytes(size)?;
            Ok(())
        })
    }

    /// Advances the `BinaryReader` four bytes and returns a `u32`.
    /// # Errors
    /// If `BinaryReader` has less than four bytes remaining.
    pub fn read_u32(&mut self) -> Result<u32> {
        self.ensure_has_bytes(4)?;
        let word = u32::from_le_bytes(
            self.buffer[self.position..self.position + 4]
                .try_into()
                .unwrap(),
        );
        self.position += 4;
        Ok(word)
    }

    /// Advances the `BinaryReader` eight bytes and returns a `u64`.
    /// # Errors
    /// If `BinaryReader` has less than eight bytes remaining.
    pub fn read_u64(&mut self) -> Result<u64> {
        self.ensure_has_bytes(8)?;
        let word = u64::from_le_bytes(
            self.buffer[self.position..self.position + 8]
                .try_into()
                .unwrap(),
        );
        self.position += 8;
        Ok(word)
    }

    /// Advances the `BinaryReader` a single byte.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has no bytes remaining.
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8> {
        let b = match self.buffer.get(self.position) {
            Some(b) => *b,
            None => return Err(self.eof_err()),
        };
        self.position += 1;
        Ok(b)
    }

    #[cold]
    fn eof_err(&self) -> BinaryReaderError {
        BinaryReaderError::eof(self.original_position(), 1)
    }

    /// Advances the `BinaryReader` up to four bytes to parse a variable
    /// length integer as a `u32`.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less than one or up to four bytes remaining, or
    /// the integer is larger than 32 bits.
    #[inline]
    pub fn read_var_u32(&mut self) -> Result<u32> {
        // Optimization for single byte i32.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(u32::from(byte))
        } else {
            self.read_var_u32_big(byte)
        }
    }

    fn read_var_u32_big(&mut self, byte: u8) -> Result<u32> {
        let mut result = (byte & 0x7F) as u32;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u32) << shift;
            if shift >= 25 && (byte >> (32 - shift)) != 0 {
                let msg = if byte & 0x80 != 0 {
                    "invalid var_u32: integer representation too long"
                } else {
                    "invalid var_u32: integer too large"
                };
                // The continuation bit or unused bits are set.
                return Err(BinaryReaderError::new(msg, self.original_position() - 1));
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Advances the `BinaryReader` up to four bytes to parse a variable
    /// length integer as a `u64`.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less than one or up to eight bytes remaining, or
    /// the integer is larger than 64 bits.
    #[inline]
    pub fn read_var_u64(&mut self) -> Result<u64> {
        // Optimization for single byte u64.
        let byte = u64::from(self.read_u8()?);
        if (byte & 0x80) == 0 {
            Ok(byte)
        } else {
            self.read_var_u64_big(byte)
        }
    }

    fn read_var_u64_big(&mut self, byte: u64) -> Result<u64> {
        let mut result = byte & 0x7F;
        let mut shift = 7;
        loop {
            let byte = u64::from(self.read_u8()?);
            result |= (byte & 0x7F) << shift;
            if shift >= 57 && (byte >> (64 - shift)) != 0 {
                let msg = if byte & 0x80 != 0 {
                    "invalid var_u64: integer representation too long"
                } else {
                    "invalid var_u64: integer too large"
                };
                // The continuation bit or unused bits are set.
                return Err(BinaryReaderError::new(msg, self.original_position() - 1));
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    /// Executes `f` to skip some data in this binary reader and then returns a
    /// reader which will read the skipped data.
    pub fn skip(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<Self> {
        let start = self.position;
        f(self)?;
        let mut ret = self.clone();
        ret.buffer = &self.buffer[start..self.position];
        ret.position = 0;
        ret.original_offset = self.original_offset + start;
        Ok(ret)
    }

    /// Advances the `BinaryReader` past a WebAssembly string. This method does
    /// not perform any utf-8 validation.
    /// # Errors
    /// If `BinaryReader` has less than four bytes, the string's length exceeds
    /// the remaining bytes, or the string length
    /// exceeds `limits::MAX_WASM_STRING_SIZE`.
    pub fn skip_string(&mut self) -> Result<()> {
        let len = self.read_var_u32()? as usize;
        if len > MAX_WASM_STRING_SIZE {
            return Err(BinaryReaderError::new(
                "string size out of bounds",
                self.original_position() - 1,
            ));
        }
        self.ensure_has_bytes(len)?;
        self.position += len;
        Ok(())
    }

    /// Advances the `BinaryReader` up to four bytes to parse a variable
    /// length integer as a `i32`.
    /// # Errors
    /// If `BinaryReader` has less than one or up to four bytes remaining, or
    /// the integer is larger than 32 bits.
    #[inline]
    pub fn read_var_i32(&mut self) -> Result<i32> {
        // Optimization for single byte i32.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(((byte as i32) << 25) >> 25)
        } else {
            self.read_var_i32_big(byte)
        }
    }

    fn read_var_i32_big(&mut self, byte: u8) -> Result<i32> {
        let mut result = (byte & 0x7F) as i32;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i32) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    let msg = if continuation_bit {
                        "invalid var_i32: integer representation too long"
                    } else {
                        "invalid var_i32: integer too large"
                    };
                    return Err(BinaryReaderError::new(msg, self.original_position() - 1));
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 32 - shift;
        Ok((result << ashift) >> ashift)
    }

    /// Advances the `BinaryReader` up to four bytes to parse a variable
    /// length integer as a signed 33 bit integer, returned as a `i64`.
    /// # Errors
    /// If `BinaryReader` has less than one or up to five bytes remaining, or
    /// the integer is larger than 33 bits.
    pub fn read_var_s33(&mut self) -> Result<i64> {
        // Optimization for single byte.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            return Ok(((byte as i8) << 1) as i64 >> 1);
        }

        let mut result = (byte & 0x7F) as i64;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i64) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (33 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(BinaryReaderError::new(
                        "invalid var_s33: integer representation too long",
                        self.original_position() - 1,
                    ));
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 64 - shift;
        Ok((result << ashift) >> ashift)
    }

    /// Advances the `BinaryReader` up to eight bytes to parse a variable
    /// length integer as a 64 bit integer, returned as a `i64`.
    /// # Errors
    /// If `BinaryReader` has less than one or up to eight bytes remaining, or
    /// the integer is larger than 64 bits.
    pub fn read_var_i64(&mut self) -> Result<i64> {
        let mut result: i64 = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= i64::from(byte & 0x7F) << shift;
            if shift >= 57 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = ((byte << 1) as i8) >> (64 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    let msg = if continuation_bit {
                        "invalid var_i64: integer representation too long"
                    } else {
                        "invalid var_i64: integer too large"
                    };
                    return Err(BinaryReaderError::new(msg, self.original_position() - 1));
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 64 - shift;
        Ok((result << ashift) >> ashift)
    }

    /// Advances the `BinaryReader` four bytes to parse a 32 bit floating point
    /// number, returned as `Ieee32`.
    /// # Errors
    /// If `BinaryReader` has less than four bytes remaining.
    pub fn read_f32(&mut self) -> Result<Ieee32> {
        let value = self.read_u32()?;
        Ok(Ieee32(value))
    }

    /// Advances the `BinaryReader` eight bytes to parse a 64 bit floating point
    /// number, returned as `Ieee64`.
    /// # Errors
    /// If `BinaryReader` has less than eight bytes remaining.
    pub fn read_f64(&mut self) -> Result<Ieee64> {
        let value = self.read_u64()?;
        Ok(Ieee64(value))
    }

    /// (internal) Reads a fixed-size WebAssembly string from the module.
    fn internal_read_string(&mut self, len: usize) -> Result<&'a str> {
        let bytes = self.read_bytes(len)?;
        str::from_utf8(bytes).map_err(|_| {
            BinaryReaderError::new("malformed UTF-8 encoding", self.original_position() - 1)
        })
    }

    /// Reads a WebAssembly string from the module.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less than up to four bytes remaining, the string's
    /// length exceeds the remaining bytes, the string's length exceeds
    /// `limits::MAX_WASM_STRING_SIZE`, or the string contains invalid utf-8.
    pub fn read_string(&mut self) -> Result<&'a str> {
        let len = self.read_var_u32()? as usize;
        if len > MAX_WASM_STRING_SIZE {
            return Err(BinaryReaderError::new(
                "string size out of bounds",
                self.original_position() - 1,
            ));
        }
        return self.internal_read_string(len);
    }

    /// Reads a unlimited WebAssembly string from the module.
    ///
    /// Note that this is similar to [`BinaryReader::read_string`] except that
    /// it will not limit the size of the returned string by
    /// `limits::MAX_WASM_STRING_SIZE`.
    pub fn read_unlimited_string(&mut self) -> Result<&'a str> {
        let len = self.read_var_u32()? as usize;
        return self.internal_read_string(len);
    }

    #[cold]
    pub(crate) fn invalid_leading_byte<T>(&self, byte: u8, desc: &str) -> Result<T> {
        Err(Self::invalid_leading_byte_error(
            byte,
            desc,
            self.original_position() - 1,
        ))
    }

    pub(crate) fn invalid_leading_byte_error(
        byte: u8,
        desc: &str,
        offset: usize,
    ) -> BinaryReaderError {
        format_err!(offset, "invalid leading byte (0x{byte:x}) for {desc}")
    }

    pub(crate) fn peek(&self) -> Result<u8> {
        self.ensure_has_byte()?;
        Ok(self.buffer[self.position])
    }

    pub(crate) fn read_block_type(&mut self) -> Result<BlockType> {
        let b = self.peek()?;

        // Block types are encoded as either 0x40, a `valtype`, or `s33`. All
        // current `valtype` encodings are negative numbers when encoded with
        // sleb128, but it's also required that valtype encodings are in their
        // canonical form. For example an overlong encoding of -1 as `0xff 0x7f`
        // is not valid and it is required to be `0x7f`. This means that we
        // can't simply match on the `s33` that pops out below since reading the
        // whole `s33` might read an overlong encoding.
        //
        // To test for this the first byte `b` is inspected. The highest bit,
        // the continuation bit in LEB128 encoding, must be clear. The next bit,
        // the sign bit, must be set to indicate that the number is negative. If
        // these two conditions hold then we're guaranteed that this is a
        // negative number.
        //
        // After this a value type is read directly instead of looking for an
        // indexed value type.
        if b & 0x80 == 0 && b & 0x40 != 0 {
            if b == 0x40 {
                self.position += 1;
                return Ok(BlockType::Empty);
            }
            return Ok(BlockType::Type(self.read()?));
        }

        // Not empty or a singular type, so read the function type index
        let idx = self.read_var_s33()?;
        match u32::try_from(idx) {
            Ok(idx) => Ok(BlockType::FuncType(idx)),
            Err(_) => {
                return Err(BinaryReaderError::new(
                    "invalid function type",
                    self.original_position(),
                ));
            }
        }
    }

    /// Visit the next available operator with the specified [`VisitOperator`] instance.
    ///
    /// Note that this does not implicitly propagate any additional information such as instruction
    /// offsets. In order to do so, consider storing such data within the visitor before visiting.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less bytes remaining than required to parse the `Operator`.
    ///
    /// # Examples
    ///
    /// Store an offset for use in diagnostics or any other purposes:
    ///
    /// ```
    /// # use wasmparser::{BinaryReader, VisitOperator, Result, for_each_visit_operator};
    ///
    /// pub fn dump(mut reader: BinaryReader) -> Result<()> {
    ///     let mut visitor = Dumper { offset: 0 };
    ///     while !reader.eof() {
    ///         visitor.offset = reader.original_position();
    ///         reader.visit_operator(&mut visitor)?;
    ///     }
    ///     Ok(())
    /// }
    ///
    /// struct Dumper {
    ///     offset: usize
    /// }
    ///
    /// macro_rules! define_visit_operator {
    ///     ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
    ///         $(
    ///             fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
    ///                 println!("{}: {}", self.offset, stringify!($visit));
    ///             }
    ///         )*
    ///     }
    /// }
    ///
    /// impl<'a> VisitOperator<'a> for Dumper {
    ///     type Output = ();
    ///     for_each_visit_operator!(define_visit_operator);
    /// }
    ///
    /// ```
    pub fn visit_operator<T>(&mut self, visitor: &mut T) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        let pos = self.original_position();
        let code = self.read_u8()? as u8;
        Ok(match code {
            0x00 => visitor.visit_unreachable(),
            0x01 => visitor.visit_nop(),
            0x02 => visitor.visit_block(self.read_block_type()?),
            0x03 => visitor.visit_loop(self.read_block_type()?),
            0x04 => visitor.visit_if(self.read_block_type()?),
            0x05 => visitor.visit_else(),
            0x06 => visitor.visit_try(self.read_block_type()?),
            0x07 => visitor.visit_catch(self.read_var_u32()?),
            0x08 => visitor.visit_throw(self.read_var_u32()?),
            0x09 => visitor.visit_rethrow(self.read_var_u32()?),
            0x0a => visitor.visit_throw_ref(),
            0x0b => visitor.visit_end(),
            0x0c => visitor.visit_br(self.read_var_u32()?),
            0x0d => visitor.visit_br_if(self.read_var_u32()?),
            0x0e => visitor.visit_br_table(self.read_br_table()?),
            0x0f => visitor.visit_return(),
            0x10 => visitor.visit_call(self.read_var_u32()?),
            0x11 => {
                let index = self.read_var_u32()?;
                let table = self.read_table_index_or_zero_if_not_reference_types()?;
                visitor.visit_call_indirect(index, table)
            }
            0x12 => visitor.visit_return_call(self.read_var_u32()?),
            0x13 => visitor.visit_return_call_indirect(self.read_var_u32()?, self.read_var_u32()?),
            0x14 => visitor.visit_call_ref(self.read()?),
            0x15 => visitor.visit_return_call_ref(self.read()?),
            0x18 => visitor.visit_delegate(self.read_var_u32()?),
            0x19 => visitor.visit_catch_all(),
            0x1a => visitor.visit_drop(),
            0x1b => visitor.visit_select(),
            0x1c => {
                let results = self.read_var_u32()?;
                if results != 1 {
                    return Err(BinaryReaderError::new(
                        "invalid result arity",
                        self.position,
                    ));
                }
                visitor.visit_typed_select(self.read()?)
            }
            0x1f => visitor.visit_try_table(self.read()?),

            0x20 => visitor.visit_local_get(self.read_var_u32()?),
            0x21 => visitor.visit_local_set(self.read_var_u32()?),
            0x22 => visitor.visit_local_tee(self.read_var_u32()?),
            0x23 => visitor.visit_global_get(self.read_var_u32()?),
            0x24 => visitor.visit_global_set(self.read_var_u32()?),
            0x25 => visitor.visit_table_get(self.read_var_u32()?),
            0x26 => visitor.visit_table_set(self.read_var_u32()?),

            0x28 => visitor.visit_i32_load(self.read_memarg(2)?),
            0x29 => visitor.visit_i64_load(self.read_memarg(3)?),
            0x2a => visitor.visit_f32_load(self.read_memarg(2)?),
            0x2b => visitor.visit_f64_load(self.read_memarg(3)?),
            0x2c => visitor.visit_i32_load8_s(self.read_memarg(0)?),
            0x2d => visitor.visit_i32_load8_u(self.read_memarg(0)?),
            0x2e => visitor.visit_i32_load16_s(self.read_memarg(1)?),
            0x2f => visitor.visit_i32_load16_u(self.read_memarg(1)?),
            0x30 => visitor.visit_i64_load8_s(self.read_memarg(0)?),
            0x31 => visitor.visit_i64_load8_u(self.read_memarg(0)?),
            0x32 => visitor.visit_i64_load16_s(self.read_memarg(1)?),
            0x33 => visitor.visit_i64_load16_u(self.read_memarg(1)?),
            0x34 => visitor.visit_i64_load32_s(self.read_memarg(2)?),
            0x35 => visitor.visit_i64_load32_u(self.read_memarg(2)?),
            0x36 => visitor.visit_i32_store(self.read_memarg(2)?),
            0x37 => visitor.visit_i64_store(self.read_memarg(3)?),
            0x38 => visitor.visit_f32_store(self.read_memarg(2)?),
            0x39 => visitor.visit_f64_store(self.read_memarg(3)?),
            0x3a => visitor.visit_i32_store8(self.read_memarg(0)?),
            0x3b => visitor.visit_i32_store16(self.read_memarg(1)?),
            0x3c => visitor.visit_i64_store8(self.read_memarg(0)?),
            0x3d => visitor.visit_i64_store16(self.read_memarg(1)?),
            0x3e => visitor.visit_i64_store32(self.read_memarg(2)?),
            0x3f => {
                let mem = self.read_memory_index_or_zero_if_not_multi_memory()?;
                visitor.visit_memory_size(mem)
            }
            0x40 => {
                let mem = self.read_memory_index_or_zero_if_not_multi_memory()?;
                visitor.visit_memory_grow(mem)
            }

            0x41 => visitor.visit_i32_const(self.read_var_i32()?),
            0x42 => visitor.visit_i64_const(self.read_var_i64()?),
            0x43 => visitor.visit_f32_const(self.read_f32()?),
            0x44 => visitor.visit_f64_const(self.read_f64()?),

            0x45 => visitor.visit_i32_eqz(),
            0x46 => visitor.visit_i32_eq(),
            0x47 => visitor.visit_i32_ne(),
            0x48 => visitor.visit_i32_lt_s(),
            0x49 => visitor.visit_i32_lt_u(),
            0x4a => visitor.visit_i32_gt_s(),
            0x4b => visitor.visit_i32_gt_u(),
            0x4c => visitor.visit_i32_le_s(),
            0x4d => visitor.visit_i32_le_u(),
            0x4e => visitor.visit_i32_ge_s(),
            0x4f => visitor.visit_i32_ge_u(),
            0x50 => visitor.visit_i64_eqz(),
            0x51 => visitor.visit_i64_eq(),
            0x52 => visitor.visit_i64_ne(),
            0x53 => visitor.visit_i64_lt_s(),
            0x54 => visitor.visit_i64_lt_u(),
            0x55 => visitor.visit_i64_gt_s(),
            0x56 => visitor.visit_i64_gt_u(),
            0x57 => visitor.visit_i64_le_s(),
            0x58 => visitor.visit_i64_le_u(),
            0x59 => visitor.visit_i64_ge_s(),
            0x5a => visitor.visit_i64_ge_u(),
            0x5b => visitor.visit_f32_eq(),
            0x5c => visitor.visit_f32_ne(),
            0x5d => visitor.visit_f32_lt(),
            0x5e => visitor.visit_f32_gt(),
            0x5f => visitor.visit_f32_le(),
            0x60 => visitor.visit_f32_ge(),
            0x61 => visitor.visit_f64_eq(),
            0x62 => visitor.visit_f64_ne(),
            0x63 => visitor.visit_f64_lt(),
            0x64 => visitor.visit_f64_gt(),
            0x65 => visitor.visit_f64_le(),
            0x66 => visitor.visit_f64_ge(),
            0x67 => visitor.visit_i32_clz(),
            0x68 => visitor.visit_i32_ctz(),
            0x69 => visitor.visit_i32_popcnt(),
            0x6a => visitor.visit_i32_add(),
            0x6b => visitor.visit_i32_sub(),
            0x6c => visitor.visit_i32_mul(),
            0x6d => visitor.visit_i32_div_s(),
            0x6e => visitor.visit_i32_div_u(),
            0x6f => visitor.visit_i32_rem_s(),
            0x70 => visitor.visit_i32_rem_u(),
            0x71 => visitor.visit_i32_and(),
            0x72 => visitor.visit_i32_or(),
            0x73 => visitor.visit_i32_xor(),
            0x74 => visitor.visit_i32_shl(),
            0x75 => visitor.visit_i32_shr_s(),
            0x76 => visitor.visit_i32_shr_u(),
            0x77 => visitor.visit_i32_rotl(),
            0x78 => visitor.visit_i32_rotr(),
            0x79 => visitor.visit_i64_clz(),
            0x7a => visitor.visit_i64_ctz(),
            0x7b => visitor.visit_i64_popcnt(),
            0x7c => visitor.visit_i64_add(),
            0x7d => visitor.visit_i64_sub(),
            0x7e => visitor.visit_i64_mul(),
            0x7f => visitor.visit_i64_div_s(),
            0x80 => visitor.visit_i64_div_u(),
            0x81 => visitor.visit_i64_rem_s(),
            0x82 => visitor.visit_i64_rem_u(),
            0x83 => visitor.visit_i64_and(),
            0x84 => visitor.visit_i64_or(),
            0x85 => visitor.visit_i64_xor(),
            0x86 => visitor.visit_i64_shl(),
            0x87 => visitor.visit_i64_shr_s(),
            0x88 => visitor.visit_i64_shr_u(),
            0x89 => visitor.visit_i64_rotl(),
            0x8a => visitor.visit_i64_rotr(),
            0x8b => visitor.visit_f32_abs(),
            0x8c => visitor.visit_f32_neg(),
            0x8d => visitor.visit_f32_ceil(),
            0x8e => visitor.visit_f32_floor(),
            0x8f => visitor.visit_f32_trunc(),
            0x90 => visitor.visit_f32_nearest(),
            0x91 => visitor.visit_f32_sqrt(),
            0x92 => visitor.visit_f32_add(),
            0x93 => visitor.visit_f32_sub(),
            0x94 => visitor.visit_f32_mul(),
            0x95 => visitor.visit_f32_div(),
            0x96 => visitor.visit_f32_min(),
            0x97 => visitor.visit_f32_max(),
            0x98 => visitor.visit_f32_copysign(),
            0x99 => visitor.visit_f64_abs(),
            0x9a => visitor.visit_f64_neg(),
            0x9b => visitor.visit_f64_ceil(),
            0x9c => visitor.visit_f64_floor(),
            0x9d => visitor.visit_f64_trunc(),
            0x9e => visitor.visit_f64_nearest(),
            0x9f => visitor.visit_f64_sqrt(),
            0xa0 => visitor.visit_f64_add(),
            0xa1 => visitor.visit_f64_sub(),
            0xa2 => visitor.visit_f64_mul(),
            0xa3 => visitor.visit_f64_div(),
            0xa4 => visitor.visit_f64_min(),
            0xa5 => visitor.visit_f64_max(),
            0xa6 => visitor.visit_f64_copysign(),
            0xa7 => visitor.visit_i32_wrap_i64(),
            0xa8 => visitor.visit_i32_trunc_f32_s(),
            0xa9 => visitor.visit_i32_trunc_f32_u(),
            0xaa => visitor.visit_i32_trunc_f64_s(),
            0xab => visitor.visit_i32_trunc_f64_u(),
            0xac => visitor.visit_i64_extend_i32_s(),
            0xad => visitor.visit_i64_extend_i32_u(),
            0xae => visitor.visit_i64_trunc_f32_s(),
            0xaf => visitor.visit_i64_trunc_f32_u(),
            0xb0 => visitor.visit_i64_trunc_f64_s(),
            0xb1 => visitor.visit_i64_trunc_f64_u(),
            0xb2 => visitor.visit_f32_convert_i32_s(),
            0xb3 => visitor.visit_f32_convert_i32_u(),
            0xb4 => visitor.visit_f32_convert_i64_s(),
            0xb5 => visitor.visit_f32_convert_i64_u(),
            0xb6 => visitor.visit_f32_demote_f64(),
            0xb7 => visitor.visit_f64_convert_i32_s(),
            0xb8 => visitor.visit_f64_convert_i32_u(),
            0xb9 => visitor.visit_f64_convert_i64_s(),
            0xba => visitor.visit_f64_convert_i64_u(),
            0xbb => visitor.visit_f64_promote_f32(),
            0xbc => visitor.visit_i32_reinterpret_f32(),
            0xbd => visitor.visit_i64_reinterpret_f64(),
            0xbe => visitor.visit_f32_reinterpret_i32(),
            0xbf => visitor.visit_f64_reinterpret_i64(),

            0xc0 => visitor.visit_i32_extend8_s(),
            0xc1 => visitor.visit_i32_extend16_s(),
            0xc2 => visitor.visit_i64_extend8_s(),
            0xc3 => visitor.visit_i64_extend16_s(),
            0xc4 => visitor.visit_i64_extend32_s(),

            0xd0 => visitor.visit_ref_null(self.read()?),
            0xd1 => visitor.visit_ref_is_null(),
            0xd2 => visitor.visit_ref_func(self.read_var_u32()?),
            0xd3 => visitor.visit_ref_eq(),
            0xd4 => visitor.visit_ref_as_non_null(),
            0xd5 => visitor.visit_br_on_null(self.read_var_u32()?),
            0xd6 => visitor.visit_br_on_non_null(self.read_var_u32()?),

            0xe0 => visitor.visit_cont_new(self.read_var_u32()?),
            0xe1 => visitor.visit_cont_bind(self.read_var_u32()?, self.read_var_u32()?),
            0xe2 => visitor.visit_suspend(self.read_var_u32()?),
            0xe3 => visitor.visit_resume(self.read_var_u32()?, self.read()?),
            0xe4 => {
                visitor.visit_resume_throw(self.read_var_u32()?, self.read_var_u32()?, self.read()?)
            }
            0xe5 => visitor.visit_switch(self.read_var_u32()?, self.read_var_u32()?),

            0xfb => self.visit_0xfb_operator(pos, visitor)?,
            0xfc => self.visit_0xfc_operator(pos, visitor)?,
            0xfd => {
                #[cfg(feature = "simd")]
                if let Some(mut visitor) = visitor.simd_visitor() {
                    return self.visit_0xfd_operator(pos, &mut visitor);
                }
                bail!(pos, "unexpected SIMD opcode: 0x{code:x}")
            }
            0xfe => self.visit_0xfe_operator(pos, visitor)?,

            _ => bail!(pos, "illegal opcode: 0x{code:x}"),
        })
    }

    fn visit_0xfb_operator<T>(
        &mut self,
        pos: usize,
        visitor: &mut T,
    ) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        let code = self.read_var_u32()?;
        Ok(match code {
            0x0 => {
                let type_index = self.read_var_u32()?;
                visitor.visit_struct_new(type_index)
            }
            0x01 => {
                let type_index = self.read_var_u32()?;
                visitor.visit_struct_new_default(type_index)
            }
            0x02 => {
                let type_index = self.read_var_u32()?;
                let field_index = self.read_var_u32()?;
                visitor.visit_struct_get(type_index, field_index)
            }
            0x03 => {
                let type_index = self.read_var_u32()?;
                let field_index = self.read_var_u32()?;
                visitor.visit_struct_get_s(type_index, field_index)
            }
            0x04 => {
                let type_index = self.read_var_u32()?;
                let field_index = self.read_var_u32()?;
                visitor.visit_struct_get_u(type_index, field_index)
            }
            0x05 => {
                let type_index = self.read_var_u32()?;
                let field_index = self.read_var_u32()?;
                visitor.visit_struct_set(type_index, field_index)
            }
            0x06 => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_new(type_index)
            }
            0x07 => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_new_default(type_index)
            }
            0x08 => {
                let type_index = self.read_var_u32()?;
                let n = self.read_var_u32()?;
                visitor.visit_array_new_fixed(type_index, n)
            }
            0x09 => {
                let type_index = self.read_var_u32()?;
                let data_index = self.read_var_u32()?;
                visitor.visit_array_new_data(type_index, data_index)
            }
            0x0a => {
                let type_index = self.read_var_u32()?;
                let elem_index = self.read_var_u32()?;
                visitor.visit_array_new_elem(type_index, elem_index)
            }
            0x0b => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_get(type_index)
            }
            0x0c => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_get_s(type_index)
            }
            0x0d => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_get_u(type_index)
            }
            0x0e => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_set(type_index)
            }
            0x0f => visitor.visit_array_len(),
            0x10 => {
                let type_index = self.read_var_u32()?;
                visitor.visit_array_fill(type_index)
            }
            0x11 => {
                let type_index_dst = self.read_var_u32()?;
                let type_index_src = self.read_var_u32()?;
                visitor.visit_array_copy(type_index_dst, type_index_src)
            }
            0x12 => {
                let type_index = self.read_var_u32()?;
                let data_index = self.read_var_u32()?;
                visitor.visit_array_init_data(type_index, data_index)
            }
            0x13 => {
                let type_index = self.read_var_u32()?;
                let elem_index = self.read_var_u32()?;
                visitor.visit_array_init_elem(type_index, elem_index)
            }
            0x14 => visitor.visit_ref_test_non_null(self.read()?),
            0x15 => visitor.visit_ref_test_nullable(self.read()?),
            0x16 => visitor.visit_ref_cast_non_null(self.read()?),
            0x17 => visitor.visit_ref_cast_nullable(self.read()?),
            0x18 => {
                let pos = self.original_position();
                let cast_flags = self.read_u8()?;
                let relative_depth = self.read_var_u32()?;
                let (from_type_nullable, to_type_nullable) = match cast_flags {
                    0b00 => (false, false),
                    0b01 => (true, false),
                    0b10 => (false, true),
                    0b11 => (true, true),
                    _ => bail!(pos, "invalid cast flags: {cast_flags:08b}"),
                };
                let from_heap_type = self.read()?;
                let from_ref_type =
                    RefType::new(from_type_nullable, from_heap_type).ok_or_else(|| {
                        format_err!(pos, "implementation error: type index too large")
                    })?;
                let to_heap_type = self.read()?;
                let to_ref_type =
                    RefType::new(to_type_nullable, to_heap_type).ok_or_else(|| {
                        format_err!(pos, "implementation error: type index too large")
                    })?;
                visitor.visit_br_on_cast(relative_depth, from_ref_type, to_ref_type)
            }
            0x19 => {
                let pos = self.original_position();
                let cast_flags = self.read_u8()?;
                let relative_depth = self.read_var_u32()?;
                let (from_type_nullable, to_type_nullable) = match cast_flags {
                    0 => (false, false),
                    1 => (true, false),
                    2 => (false, true),
                    3 => (true, true),
                    _ => bail!(pos, "invalid cast flags: {cast_flags:08b}"),
                };
                let from_heap_type = self.read()?;
                let from_ref_type =
                    RefType::new(from_type_nullable, from_heap_type).ok_or_else(|| {
                        format_err!(pos, "implementation error: type index too large")
                    })?;
                let to_heap_type = self.read()?;
                let to_ref_type =
                    RefType::new(to_type_nullable, to_heap_type).ok_or_else(|| {
                        format_err!(pos, "implementation error: type index too large")
                    })?;
                visitor.visit_br_on_cast_fail(relative_depth, from_ref_type, to_ref_type)
            }

            0x1a => visitor.visit_any_convert_extern(),
            0x1b => visitor.visit_extern_convert_any(),

            0x1c => visitor.visit_ref_i31(),
            0x1d => visitor.visit_i31_get_s(),
            0x1e => visitor.visit_i31_get_u(),

            _ => bail!(pos, "unknown 0xfb subopcode: 0x{code:x}"),
        })
    }

    fn visit_0xfc_operator<T>(
        &mut self,
        pos: usize,
        visitor: &mut T,
    ) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        let code = self.read_var_u32()?;
        Ok(match code {
            0x00 => visitor.visit_i32_trunc_sat_f32_s(),
            0x01 => visitor.visit_i32_trunc_sat_f32_u(),
            0x02 => visitor.visit_i32_trunc_sat_f64_s(),
            0x03 => visitor.visit_i32_trunc_sat_f64_u(),
            0x04 => visitor.visit_i64_trunc_sat_f32_s(),
            0x05 => visitor.visit_i64_trunc_sat_f32_u(),
            0x06 => visitor.visit_i64_trunc_sat_f64_s(),
            0x07 => visitor.visit_i64_trunc_sat_f64_u(),

            0x08 => {
                let segment = self.read_var_u32()?;
                let mem = self.read_var_u32()?;
                visitor.visit_memory_init(segment, mem)
            }
            0x09 => {
                let segment = self.read_var_u32()?;
                visitor.visit_data_drop(segment)
            }
            0x0a => {
                let dst = self.read_var_u32()?;
                let src = self.read_var_u32()?;
                visitor.visit_memory_copy(dst, src)
            }
            0x0b => {
                let mem = self.read_var_u32()?;
                visitor.visit_memory_fill(mem)
            }
            0x0c => {
                let segment = self.read_var_u32()?;
                let table = self.read_var_u32()?;
                visitor.visit_table_init(segment, table)
            }
            0x0d => {
                let segment = self.read_var_u32()?;
                visitor.visit_elem_drop(segment)
            }
            0x0e => {
                let dst_table = self.read_var_u32()?;
                let src_table = self.read_var_u32()?;
                visitor.visit_table_copy(dst_table, src_table)
            }

            0x0f => {
                let table = self.read_var_u32()?;
                visitor.visit_table_grow(table)
            }
            0x10 => {
                let table = self.read_var_u32()?;
                visitor.visit_table_size(table)
            }

            0x11 => {
                let table = self.read_var_u32()?;
                visitor.visit_table_fill(table)
            }

            0x12 => {
                let mem = self.read_var_u32()?;
                visitor.visit_memory_discard(mem)
            }

            0x13 => visitor.visit_i64_add128(),
            0x14 => visitor.visit_i64_sub128(),
            0x15 => visitor.visit_i64_mul_wide_s(),
            0x16 => visitor.visit_i64_mul_wide_u(),

            _ => bail!(pos, "unknown 0xfc subopcode: 0x{code:x}"),
        })
    }

    fn visit_0xfe_operator<T>(
        &mut self,
        pos: usize,
        visitor: &mut T,
    ) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        let code = self.read_var_u32()?;
        Ok(match code {
            0x00 => visitor.visit_memory_atomic_notify(self.read_memarg(2)?),
            0x01 => visitor.visit_memory_atomic_wait32(self.read_memarg(2)?),
            0x02 => visitor.visit_memory_atomic_wait64(self.read_memarg(3)?),
            0x03 => {
                if self.read_u8()? != 0 {
                    bail!(pos, "nonzero byte after `atomic.fence`");
                }
                visitor.visit_atomic_fence()
            }
            0x10 => visitor.visit_i32_atomic_load(self.read_memarg(2)?),
            0x11 => visitor.visit_i64_atomic_load(self.read_memarg(3)?),
            0x12 => visitor.visit_i32_atomic_load8_u(self.read_memarg(0)?),
            0x13 => visitor.visit_i32_atomic_load16_u(self.read_memarg(1)?),
            0x14 => visitor.visit_i64_atomic_load8_u(self.read_memarg(0)?),
            0x15 => visitor.visit_i64_atomic_load16_u(self.read_memarg(1)?),
            0x16 => visitor.visit_i64_atomic_load32_u(self.read_memarg(2)?),
            0x17 => visitor.visit_i32_atomic_store(self.read_memarg(2)?),
            0x18 => visitor.visit_i64_atomic_store(self.read_memarg(3)?),
            0x19 => visitor.visit_i32_atomic_store8(self.read_memarg(0)?),
            0x1a => visitor.visit_i32_atomic_store16(self.read_memarg(1)?),
            0x1b => visitor.visit_i64_atomic_store8(self.read_memarg(0)?),
            0x1c => visitor.visit_i64_atomic_store16(self.read_memarg(1)?),
            0x1d => visitor.visit_i64_atomic_store32(self.read_memarg(2)?),
            0x1e => visitor.visit_i32_atomic_rmw_add(self.read_memarg(2)?),
            0x1f => visitor.visit_i64_atomic_rmw_add(self.read_memarg(3)?),
            0x20 => visitor.visit_i32_atomic_rmw8_add_u(self.read_memarg(0)?),
            0x21 => visitor.visit_i32_atomic_rmw16_add_u(self.read_memarg(1)?),
            0x22 => visitor.visit_i64_atomic_rmw8_add_u(self.read_memarg(0)?),
            0x23 => visitor.visit_i64_atomic_rmw16_add_u(self.read_memarg(1)?),
            0x24 => visitor.visit_i64_atomic_rmw32_add_u(self.read_memarg(2)?),
            0x25 => visitor.visit_i32_atomic_rmw_sub(self.read_memarg(2)?),
            0x26 => visitor.visit_i64_atomic_rmw_sub(self.read_memarg(3)?),
            0x27 => visitor.visit_i32_atomic_rmw8_sub_u(self.read_memarg(0)?),
            0x28 => visitor.visit_i32_atomic_rmw16_sub_u(self.read_memarg(1)?),
            0x29 => visitor.visit_i64_atomic_rmw8_sub_u(self.read_memarg(0)?),
            0x2a => visitor.visit_i64_atomic_rmw16_sub_u(self.read_memarg(1)?),
            0x2b => visitor.visit_i64_atomic_rmw32_sub_u(self.read_memarg(2)?),
            0x2c => visitor.visit_i32_atomic_rmw_and(self.read_memarg(2)?),
            0x2d => visitor.visit_i64_atomic_rmw_and(self.read_memarg(3)?),
            0x2e => visitor.visit_i32_atomic_rmw8_and_u(self.read_memarg(0)?),
            0x2f => visitor.visit_i32_atomic_rmw16_and_u(self.read_memarg(1)?),
            0x30 => visitor.visit_i64_atomic_rmw8_and_u(self.read_memarg(0)?),
            0x31 => visitor.visit_i64_atomic_rmw16_and_u(self.read_memarg(1)?),
            0x32 => visitor.visit_i64_atomic_rmw32_and_u(self.read_memarg(2)?),
            0x33 => visitor.visit_i32_atomic_rmw_or(self.read_memarg(2)?),
            0x34 => visitor.visit_i64_atomic_rmw_or(self.read_memarg(3)?),
            0x35 => visitor.visit_i32_atomic_rmw8_or_u(self.read_memarg(0)?),
            0x36 => visitor.visit_i32_atomic_rmw16_or_u(self.read_memarg(1)?),
            0x37 => visitor.visit_i64_atomic_rmw8_or_u(self.read_memarg(0)?),
            0x38 => visitor.visit_i64_atomic_rmw16_or_u(self.read_memarg(1)?),
            0x39 => visitor.visit_i64_atomic_rmw32_or_u(self.read_memarg(2)?),
            0x3a => visitor.visit_i32_atomic_rmw_xor(self.read_memarg(2)?),
            0x3b => visitor.visit_i64_atomic_rmw_xor(self.read_memarg(3)?),
            0x3c => visitor.visit_i32_atomic_rmw8_xor_u(self.read_memarg(0)?),
            0x3d => visitor.visit_i32_atomic_rmw16_xor_u(self.read_memarg(1)?),
            0x3e => visitor.visit_i64_atomic_rmw8_xor_u(self.read_memarg(0)?),
            0x3f => visitor.visit_i64_atomic_rmw16_xor_u(self.read_memarg(1)?),
            0x40 => visitor.visit_i64_atomic_rmw32_xor_u(self.read_memarg(2)?),
            0x41 => visitor.visit_i32_atomic_rmw_xchg(self.read_memarg(2)?),
            0x42 => visitor.visit_i64_atomic_rmw_xchg(self.read_memarg(3)?),
            0x43 => visitor.visit_i32_atomic_rmw8_xchg_u(self.read_memarg(0)?),
            0x44 => visitor.visit_i32_atomic_rmw16_xchg_u(self.read_memarg(1)?),
            0x45 => visitor.visit_i64_atomic_rmw8_xchg_u(self.read_memarg(0)?),
            0x46 => visitor.visit_i64_atomic_rmw16_xchg_u(self.read_memarg(1)?),
            0x47 => visitor.visit_i64_atomic_rmw32_xchg_u(self.read_memarg(2)?),
            0x48 => visitor.visit_i32_atomic_rmw_cmpxchg(self.read_memarg(2)?),
            0x49 => visitor.visit_i64_atomic_rmw_cmpxchg(self.read_memarg(3)?),
            0x4a => visitor.visit_i32_atomic_rmw8_cmpxchg_u(self.read_memarg(0)?),
            0x4b => visitor.visit_i32_atomic_rmw16_cmpxchg_u(self.read_memarg(1)?),
            0x4c => visitor.visit_i64_atomic_rmw8_cmpxchg_u(self.read_memarg(0)?),
            0x4d => visitor.visit_i64_atomic_rmw16_cmpxchg_u(self.read_memarg(1)?),
            0x4e => visitor.visit_i64_atomic_rmw32_cmpxchg_u(self.read_memarg(2)?),

            // Decode shared-everything-threads proposal.
            0x4f => visitor.visit_global_atomic_get(self.read_ordering()?, self.read_var_u32()?),
            0x50 => visitor.visit_global_atomic_set(self.read_ordering()?, self.read_var_u32()?),
            0x51 => {
                visitor.visit_global_atomic_rmw_add(self.read_ordering()?, self.read_var_u32()?)
            }
            0x52 => {
                visitor.visit_global_atomic_rmw_sub(self.read_ordering()?, self.read_var_u32()?)
            }
            0x53 => {
                visitor.visit_global_atomic_rmw_and(self.read_ordering()?, self.read_var_u32()?)
            }
            0x54 => visitor.visit_global_atomic_rmw_or(self.read_ordering()?, self.read_var_u32()?),
            0x55 => {
                visitor.visit_global_atomic_rmw_xor(self.read_ordering()?, self.read_var_u32()?)
            }
            0x56 => {
                visitor.visit_global_atomic_rmw_xchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x57 => {
                visitor.visit_global_atomic_rmw_cmpxchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x58 => visitor.visit_table_atomic_get(self.read_ordering()?, self.read_var_u32()?),
            0x59 => visitor.visit_table_atomic_set(self.read_ordering()?, self.read_var_u32()?),
            0x5a => {
                visitor.visit_table_atomic_rmw_xchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x5b => {
                visitor.visit_table_atomic_rmw_cmpxchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x5c => visitor.visit_struct_atomic_get(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x5d => visitor.visit_struct_atomic_get_s(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x5e => visitor.visit_struct_atomic_get_u(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x5f => visitor.visit_struct_atomic_set(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x60 => visitor.visit_struct_atomic_rmw_add(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x61 => visitor.visit_struct_atomic_rmw_sub(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x62 => visitor.visit_struct_atomic_rmw_and(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x63 => visitor.visit_struct_atomic_rmw_or(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x64 => visitor.visit_struct_atomic_rmw_xor(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x65 => visitor.visit_struct_atomic_rmw_xchg(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x66 => visitor.visit_struct_atomic_rmw_cmpxchg(
                self.read_ordering()?,
                self.read_var_u32()?,
                self.read_var_u32()?,
            ),
            0x67 => visitor.visit_array_atomic_get(self.read_ordering()?, self.read_var_u32()?),
            0x68 => visitor.visit_array_atomic_get_s(self.read_ordering()?, self.read_var_u32()?),
            0x69 => visitor.visit_array_atomic_get_u(self.read_ordering()?, self.read_var_u32()?),
            0x6a => visitor.visit_array_atomic_set(self.read_ordering()?, self.read_var_u32()?),
            0x6b => visitor.visit_array_atomic_rmw_add(self.read_ordering()?, self.read_var_u32()?),
            0x6c => visitor.visit_array_atomic_rmw_sub(self.read_ordering()?, self.read_var_u32()?),
            0x6d => visitor.visit_array_atomic_rmw_and(self.read_ordering()?, self.read_var_u32()?),
            0x6e => visitor.visit_array_atomic_rmw_or(self.read_ordering()?, self.read_var_u32()?),
            0x6f => visitor.visit_array_atomic_rmw_xor(self.read_ordering()?, self.read_var_u32()?),
            0x70 => {
                visitor.visit_array_atomic_rmw_xchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x71 => {
                visitor.visit_array_atomic_rmw_cmpxchg(self.read_ordering()?, self.read_var_u32()?)
            }
            0x72 => visitor.visit_ref_i31_shared(),

            _ => bail!(pos, "unknown 0xfe subopcode: 0x{code:x}"),
        })
    }

    /// Reads the next available `Operator`.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less bytes remaining than required to parse
    /// the `Operator`.
    pub fn read_operator(&mut self) -> Result<Operator<'a>> {
        self.visit_operator(&mut OperatorFactory::new())
    }

    /// Returns whether there is an `end` opcode followed by eof remaining in
    /// this reader.
    pub fn is_end_then_eof(&self) -> bool {
        self.remaining_buffer() == &[0x0b]
    }

    #[cfg(feature = "simd")]
    fn read_lane_index(&mut self, max: u8) -> Result<u8> {
        let index = self.read_u8()?;
        if index >= max {
            return Err(BinaryReaderError::new(
                "invalid lane index",
                self.original_position() - 1,
            ));
        }
        Ok(index)
    }

    #[cfg(feature = "simd")]
    fn read_v128(&mut self) -> Result<V128> {
        let mut bytes = [0; 16];
        bytes.clone_from_slice(self.read_bytes(16)?);
        Ok(V128(bytes))
    }

    pub(crate) fn read_header_version(&mut self) -> Result<u32> {
        let magic_number = self.read_bytes(4)?;
        if magic_number != WASM_MAGIC_NUMBER {
            return Err(BinaryReaderError::new(
                format!("magic header not detected: bad magic number - expected={WASM_MAGIC_NUMBER:#x?} actual={magic_number:#x?}"),
                self.original_position() - 4,
            ));
        }
        self.read_u32()
    }

    pub(crate) fn skip_const_expr(&mut self) -> Result<()> {
        // TODO add skip_operator() method and/or validate ConstExpr operators.
        loop {
            if let Operator::End = self.read_operator()? {
                return Ok(());
            }
        }
    }

    fn read_memory_index_or_zero_if_not_multi_memory(&mut self) -> Result<u32> {
        if self.multi_memory() {
            self.read_var_u32()
        } else {
            // Before bulk memory this byte was required to be a single zero
            // byte, not a LEB-encoded zero, so require a precise zero byte.
            match self.read_u8()? {
                0 => Ok(0),
                _ => bail!(self.original_position() - 1, "zero byte expected"),
            }
        }
    }

    fn read_table_index_or_zero_if_not_reference_types(&mut self) -> Result<u32> {
        if self.reference_types() {
            self.read_var_u32()
        } else {
            // Before reference types this byte was required to be a single zero
            // byte, not a LEB-encoded zero, so require a precise zero byte.
            match self.read_u8()? {
                0 => Ok(0),
                _ => bail!(self.original_position() - 1, "zero byte expected"),
            }
        }
    }
}

// See documentation on `BinaryReader::features` for more on what's going on
// here.
macro_rules! define_feature_accessor {
    ($feature:ident = $default:expr) => {
        impl BinaryReader<'_> {
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn $feature(&self) -> bool {
                #[cfg(feature = "features")]
                {
                    self.features.$feature()
                }
                #[cfg(not(feature = "features"))]
                {
                    true
                }
            }
        }
    };
}

super::features::foreach_wasm_feature!(define_feature_accessor);

impl<'a> BrTable<'a> {
    /// Returns the number of `br_table` entries, not including the default
    /// label
    pub fn len(&self) -> u32 {
        self.cnt
    }

    /// Returns whether `BrTable` doesn't have any labels apart from the default one.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the default target of this `br_table` instruction.
    pub fn default(&self) -> u32 {
        self.default
    }

    /// Returns the list of targets that this `br_table` instruction will be
    /// jumping to.
    ///
    /// This method will return an iterator which parses each target of this
    /// `br_table` except the default target. The returned iterator will
    /// yield `self.len()` elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasmparser::{BinaryReader, Operator};
    ///
    /// let buf = [0x0e, 0x02, 0x01, 0x02, 0x00];
    /// let mut reader = BinaryReader::new(&buf, 0);
    /// let op = reader.read_operator().unwrap();
    /// if let Operator::BrTable { targets } = op {
    ///     let targets = targets.targets().collect::<Result<Vec<_>, _>>().unwrap();
    ///     assert_eq!(targets, [1, 2]);
    /// }
    /// ```
    pub fn targets(&self) -> BrTableTargets {
        BrTableTargets {
            reader: self.reader.clone(),
            remaining: self.cnt,
        }
    }
}

/// An iterator over the targets of a [`BrTable`].
///
/// # Note
///
/// This iterator parses each target of the underlying `br_table`
/// except for the default target.
/// The iterator will yield exactly as many targets as the `br_table` has.
pub struct BrTableTargets<'a> {
    reader: crate::BinaryReader<'a>,
    remaining: u32,
}

impl<'a> Iterator for BrTableTargets<'a> {
    type Item = Result<u32>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = usize::try_from(self.remaining).unwrap_or_else(|error| {
            panic!("could not convert remaining `u32` into `usize`: {}", error)
        });
        (remaining, Some(remaining))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            if !self.reader.eof() {
                return Some(Err(BinaryReaderError::new(
                    "trailing data in br_table",
                    self.reader.original_position(),
                )));
            }
            return None;
        }
        self.remaining -= 1;
        Some(self.reader.read_var_u32())
    }
}

impl fmt::Debug for BrTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("BrTable");
        f.field("count", &self.cnt);
        f.field("default", &self.default);
        match self.targets().collect::<Result<Vec<_>>>() {
            Ok(targets) => {
                f.field("targets", &targets);
            }
            Err(_) => {
                f.field("reader", &self.reader);
            }
        }
        f.finish()
    }
}

/// A factory to construct [`Operator`] instances via the [`VisitOperator`] trait.
struct OperatorFactory<'a> {
    marker: core::marker::PhantomData<fn() -> &'a ()>,
}

impl<'a> OperatorFactory<'a> {
    /// Creates a new [`OperatorFactory`].
    fn new() -> Self {
        Self {
            marker: core::marker::PhantomData,
        }
    }
}

macro_rules! define_visit_operator {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Operator<'a> {
                Operator::$op $({ $($arg),* })?
            }
        )*
    }
}

impl<'a> VisitOperator<'a> for OperatorFactory<'a> {
    type Output = Operator<'a>;

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

    crate::for_each_visit_operator!(define_visit_operator);
}

#[cfg(feature = "simd")]
impl<'a> VisitSimdOperator<'a> for OperatorFactory<'a> {
    crate::for_each_visit_simd_operator!(define_visit_operator);
}

/// Iterator returned from [`BinaryReader::read_iter`].
pub struct BinaryReaderIter<'a, 'me, T: FromReader<'a>> {
    remaining: usize,
    pub(crate) reader: &'me mut BinaryReader<'a>,
    _marker: marker::PhantomData<T>,
}

impl<'a, T> Iterator for BinaryReaderIter<'a, '_, T>
where
    T: FromReader<'a>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        if self.remaining == 0 {
            None
        } else {
            let ret = self.reader.read::<T>();
            if ret.is_err() {
                self.remaining = 0;
            } else {
                self.remaining -= 1;
            }
            Some(ret)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T> Drop for BinaryReaderIter<'a, '_, T>
where
    T: FromReader<'a>,
{
    fn drop(&mut self) {
        while self.next().is_some() {
            // ...
        }
    }
}
