use crate::error::{Error, ErrorCode, Result};
use alloc::vec::Vec;
use core::cmp;
use core::mem;
use core::ops::Deref;
use core::str;

#[cfg(feature = "std")]
use crate::io;
#[cfg(feature = "std")]
use crate::iter::LineColIterator;

#[cfg(feature = "raw_value")]
use crate::raw::BorrowedRawDeserializer;
#[cfg(all(feature = "raw_value", feature = "std"))]
use crate::raw::OwnedRawDeserializer;
#[cfg(all(feature = "raw_value", feature = "std"))]
use alloc::string::String;
#[cfg(feature = "raw_value")]
use serde::de::Visitor;

/// Trait used by the deserializer for iterating over input. This is manually
/// "specialized" for iterating over `&[u8]`. Once feature(specialization) is
/// stable we can use actual specialization.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json_lenient`.
pub trait Read<'de>: private::Sealed {
    #[doc(hidden)]
    fn next(&mut self) -> Result<Option<u8>>;
    #[doc(hidden)]
    fn peek(&mut self) -> Result<Option<u8>>;

    /// Only valid after a call to peek(). Discards the peeked byte.
    #[doc(hidden)]
    fn discard(&mut self);

    /// Position of the most recent call to next().
    ///
    /// The most recent call was probably next() and not peek(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually peek() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn position(&self) -> Position;

    /// Position of the most recent call to peek().
    ///
    /// The most recent call was probably peek() and not next(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually next() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn peek_position(&self) -> Position;

    /// Offset from the beginning of the input to the next byte that would be
    /// returned by next() or peek().
    #[doc(hidden)]
    fn byte_offset(&self) -> usize;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    #[doc(hidden)]
    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>>;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    ///
    /// This function returns the raw bytes in the string with escape sequences
    /// expanded but without performing unicode validation.
    #[doc(hidden)]
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark but discards the data.
    #[doc(hidden)]
    fn ignore_str(&mut self) -> Result<()>;

    /// Assumes the previous byte was a hex escape sequence ('\u') in a string.
    /// Parses next hexadecimal sequence.
    #[doc(hidden)]
    fn decode_hex_escape(&mut self) -> Result<u16>;

    /// Assumes the previous byte was a 2-digit hex escape sequence ('\x') in a string.
    /// Parses next hexadecimal sequence.
    #[doc(hidden)]
    fn decode_hex_escape_2(&mut self) -> Result<u16>;

    /// Switch raw buffering mode on.
    ///
    /// This is used when deserializing `RawValue`.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn begin_raw_buffering(&mut self);

    /// Switch raw buffering mode off and provides the raw buffered data to the
    /// given visitor.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;

    /// Whether we should replace invalid unicode characters with \u{fffd}.
    fn replace_invalid_unicode(&self) -> bool;

    /// Allow \v escapes
    fn allow_v_escapes(&self) -> bool;

    /// Allow \x escapes
    fn allow_x_escapes(&self) -> bool;

    /// Whether StreamDeserializer::next needs to check the failed flag. True
    /// for IoRead, false for StrRead and SliceRead which can track failure by
    /// truncating their input slice to avoid the extra check on every next
    /// call.
    #[doc(hidden)]
    const should_early_return_if_failed: bool;

    /// Mark a persistent failure of StreamDeserializer, either by setting the
    /// flag or by truncating the input data.
    #[doc(hidden)]
    fn set_failed(&mut self, failed: &mut bool);
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'b T),
    Copied(&'c T),
}

impl<'b, 'c, T> Deref for Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}

/// Trait used by parse_str_bytes to convert the resulting bytes
/// into a string-like thing. Depending on the original caller, this may
/// be a &str or a &[u8].
trait UtfOutputStrategy<T: ?Sized> {
    fn to_result_simple<'de, 's, R: Read<'de>>(&self, read: &R, slice: &'s [u8]) -> Result<&'s T>;

    fn to_result_direct<'de, 's, R: Read<'de>>(
        &self,
        read: &R,
        slice: &'s [u8],
        _: &'de mut Vec<u8>,
    ) -> Result<Reference<'s, 'de, T>> {
        self.to_result_simple(read, slice)
            .map(|r| Reference::Borrowed(r))
    }

    fn to_result_from_scratch<'de, 's, R: Read<'de>>(
        &self,
        read: &R,
        slice: &'s [u8],
    ) -> Result<&'s T> {
        self.to_result_simple(read, slice)
    }
    fn extend_scratch(&self, scratch: &mut Vec<u8>, slice: &[u8]) {
        scratch.extend(slice);
    }
}

fn convert_or_error<'de, 's, R: Read<'de>>(read: &R, slice: &'s [u8]) -> Result<&'s str> {
    str::from_utf8(slice).or_else(|_| error(read, ErrorCode::InvalidUnicodeCodePoint))
}

struct StrUtfOutputStrategy;

impl UtfOutputStrategy<str> for StrUtfOutputStrategy {
    fn to_result_simple<'de, 's, R: Read<'de>>(
        &self,
        read: &R,
        slice: &'s [u8],
    ) -> Result<&'s str> {
        convert_or_error(read, slice)
    }

    fn to_result_from_scratch<'de, 's, R: Read<'de>>(
        &self,
        read: &R,
        slice: &'s [u8],
    ) -> Result<&'s str> {
        match str::from_utf8(slice) {
            Ok(s) => Ok(s),
            Err(_) => error(read, ErrorCode::InvalidUnicodeCodePoint),
        }
    }
}

struct SubstitutingStrUtfOutputStrategy;

impl SubstitutingStrUtfOutputStrategy {
    /// Returns whether conversion occurred. If not, output is unchanged
    /// and the caller should just directly use the input slice.
    fn convert_from_utf8_lossy(&self, output: &mut Vec<u8>, mut input: &[u8]) -> bool {
        let mut first = true;
        loop {
            match core::str::from_utf8(input) {
                Ok(valid) => {
                    if first {
                        return false;
                    }
                    output.extend(valid.as_bytes());
                    break;
                }
                Err(error) => {
                    let (valid, after_valid) = input.split_at(error.valid_up_to());
                    output.extend(valid);
                    output.extend("\u{fffd}".bytes());

                    if let Some(invalid_sequence_length) = error.error_len() {
                        input = &after_valid[invalid_sequence_length..];
                    } else {
                        break;
                    }
                }
            }
            first = false;
        }
        true
    }

    fn convert_unchecked<'a>(&self, slice: &'a [u8]) -> &'a str {
        unsafe { str::from_utf8_unchecked(slice) }
    }
}

impl UtfOutputStrategy<str> for SubstitutingStrUtfOutputStrategy {
    fn to_result_simple<'de, 's, R: Read<'de>>(
        &self,
        read: &R,
        slice: &'s [u8],
    ) -> Result<&'s str> {
        convert_or_error(read, slice)
    }

    fn to_result_direct<'de, 's, R: Read<'de>>(
        &self,
        _: &R,
        slice: &'s [u8],
        scratch: &'de mut Vec<u8>,
    ) -> Result<Reference<'s, 'de, str>> {
        let r = self.convert_from_utf8_lossy(scratch, slice);
        Ok(if r {
            Reference::Copied(self.convert_unchecked(scratch))
        } else {
            Reference::Borrowed(self.convert_unchecked(slice))
        })
    }

    fn to_result_from_scratch<'de, 's, R: Read<'de>>(
        &self,
        _: &R,
        slice: &'s [u8],
    ) -> Result<&'s str> {
        // We checked it on the way into the scratch buffer, so no need for further checks now
        Ok(self.convert_unchecked(slice))
    }

    fn extend_scratch(&self, scratch: &mut Vec<u8>, slice: &[u8]) {
        if !self.convert_from_utf8_lossy(scratch, slice) {
            scratch.extend(slice);
        }
    }
}

struct UncheckedStrUtfOutputStrategy;

impl UtfOutputStrategy<str> for UncheckedStrUtfOutputStrategy {
    fn to_result_simple<'de, 's, R: Read<'de>>(&self, _: &R, slice: &'s [u8]) -> Result<&'s str> {
        // The input is assumed to be valid UTF-8 and the \u-escapes are
        // checked along the way, so don't need to check here.
        Ok(unsafe { str::from_utf8_unchecked(slice) })
    }
}

struct SliceUtfOutputStrategy;

impl UtfOutputStrategy<[u8]> for SliceUtfOutputStrategy {
    fn to_result_simple<'de, 's, R: Read<'de>>(&self, _: &R, slice: &'s [u8]) -> Result<&'s [u8]> {
        Ok(slice)
    }
}

/// JSON input source that reads from a std::io input stream.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct IoRead<R>
where
    R: io::Read,
{
    iter: LineColIterator<io::Bytes<R>>,
    /// Temporary storage of peeked byte.
    ch: Option<u8>,
    #[cfg(feature = "raw_value")]
    raw_buffer: Option<Vec<u8>>,
}

/// JSON input source that reads from a slice of bytes.
//
// This is more efficient than other iterators because peek() can be read-only
// and we can compute line/col position only if an error happens.
#[allow(clippy::struct_excessive_bools)]
pub struct SliceRead<'a> {
    slice: &'a [u8],
    /// Index of the *next* byte that will be returned by next() or peek().
    index: usize,
    replace_invalid_characters: bool,
    allow_newlines_in_string: bool,
    allow_control_characters_in_string: bool,
    allow_x_escapes: bool,
    allow_v_escapes: bool,
    #[cfg(feature = "raw_value")]
    raw_buffering_start_index: usize,
}

/// JSON input source that reads from a UTF-8 string.
//
// Able to elide UTF-8 checks by assuming that the input is valid UTF-8.
pub struct StrRead<'a> {
    delegate: SliceRead<'a>,
    #[cfg(feature = "raw_value")]
    data: &'a str,
}

// Prevent users from implementing the Read trait.
mod private {
    pub trait Sealed {}
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    /// Create a JSON input source to read from a std::io input stream.
    pub fn new(reader: R) -> Self {
        IoRead {
            iter: LineColIterator::new(reader.bytes()),
            ch: None,
            #[cfg(feature = "raw_value")]
            raw_buffer: None,
        }
    }
}

#[cfg(feature = "std")]
impl<R> private::Sealed for IoRead<R> where R: io::Read {}

#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    #[allow(clippy::needless_pass_by_value)]
    fn parse_str_bytes<'s, T, S>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        utf_strategy: S,
    ) -> Result<&'s T>
    where
        T: ?Sized,
        S: UtfOutputStrategy<T>,
    {
        loop {
            let ch = tri!(next_or_eof(self));
            if !is_escape(ch, true, true) {
                scratch.push(ch);
                continue;
            }
            match ch {
                b'"' => {
                    return utf_strategy.to_result_simple(self, scratch);
                }
                b'\\' => {
                    tri!(parse_escape(self, validate, scratch));
                }
                _ => {
                    if validate {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                    scratch.push(ch);
                }
            }
        }
    }
}

#[cfg(feature = "std")]
impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    fn replace_invalid_unicode(&self) -> bool {
        false
    }

    fn allow_x_escapes(&self) -> bool {
        false
    }

    fn allow_v_escapes(&self) -> bool {
        false
    }

    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        match self.ch.take() {
            Some(ch) => {
                #[cfg(feature = "raw_value")]
                {
                    if let Some(buf) = &mut self.raw_buffer {
                        buf.push(ch);
                    }
                }
                Ok(Some(ch))
            }
            None => match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(ch)) => {
                    #[cfg(feature = "raw_value")]
                    {
                        if let Some(buf) = &mut self.raw_buffer {
                            buf.push(ch);
                        }
                    }
                    Ok(Some(ch))
                }
                None => Ok(None),
            },
        }
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        match self.ch {
            Some(ch) => Ok(Some(ch)),
            None => match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(ch)) => {
                    self.ch = Some(ch);
                    Ok(self.ch)
                }
                None => Ok(None),
            },
        }
    }

    #[cfg(not(feature = "raw_value"))]
    #[inline]
    fn discard(&mut self) {
        self.ch = None;
    }

    #[cfg(feature = "raw_value")]
    fn discard(&mut self) {
        if let Some(ch) = self.ch.take() {
            if let Some(buf) = &mut self.raw_buffer {
                buf.push(ch);
            }
        }
    }

    fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }

    fn peek_position(&self) -> Position {
        // The LineColIterator updates its position during peek() so it has the
        // right one here.
        self.position()
    }

    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        self.parse_str_bytes(scratch, true, StrUtfOutputStrategy)
            .map(Reference::Copied)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, SliceUtfOutputStrategy)
            .map(Reference::Copied)
    }

    fn ignore_str(&mut self) -> Result<()> {
        loop {
            let ch = tri!(next_or_eof(self));
            if !is_escape(ch, true, true) {
                continue;
            }
            match ch {
                b'"' => {
                    return Ok(());
                }
                b'\\' => {
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        let a = tri!(next_or_eof(self));
        let b = tri!(next_or_eof(self));
        let c = tri!(next_or_eof(self));
        let d = tri!(next_or_eof(self));
        match decode_four_hex_digits(a, b, c, d) {
            Some(val) => Ok(val),
            None => error(self, ErrorCode::InvalidEscape),
        }
    }

    fn decode_hex_escape_2(&mut self) -> Result<u16> {
        let a = tri!(next_or_eof(self));
        let b = tri!(next_or_eof(self));
        match decode_two_hex_digits(a, b) {
            Some(val) => Ok(val),
            None => error(self, ErrorCode::InvalidEscape),
        }
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffer = Some(Vec::new());
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let raw = self.raw_buffer.take().unwrap();
        let raw = match String::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor.visit_map(OwnedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = true;

    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        *failed = true;
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> SliceRead<'a> {
    /// Create a JSON input source to read from a slice of bytes.
    ///
    /// The options are as follows:
    /// - `replace_invalid_characters` - replace invalid characters with U+FFFD.
    /// - `allow_newlines_in_string` - allow CR and LF characters in strings.
    /// - `allow_control_characters_in_string` - allow control characters other than CR/LF in
    ///    strings.
    /// - `allow_v_escapes` - allow `\v` in strings.
    /// - `allow_x_escapes` - allow `\x##` in strings.
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn new(
        slice: &'a [u8],
        replace_invalid_characters: bool,
        allow_newlines_in_string: bool,
        allow_control_characters_in_string: bool,
        allow_v_escapes: bool,
        allow_x_escapes: bool,
    ) -> Self {
        SliceRead {
            slice,
            index: 0,
            replace_invalid_characters,
            allow_newlines_in_string,
            allow_control_characters_in_string,
            allow_v_escapes,
            allow_x_escapes,
            #[cfg(feature = "raw_value")]
            raw_buffering_start_index: 0,
        }
    }

    fn position_of_index(&self, i: usize) -> Position {
        let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
            Some(position) => position + 1,
            None => 0,
        };
        Position {
            line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
            column: i - start_of_line,
        }
    }

    fn skip_to_escape(&mut self, forbid_control_characters: bool, forbid_newlines_in_strings: bool) {
        // Immediately bail-out on empty strings and consecutive escapes (e.g. \u041b\u0435)
        if self.index == self.slice.len()
            || is_escape(self.slice[self.index], forbid_control_characters, forbid_newlines_in_strings)
        {
            return;
        }
        self.index += 1;

        let rest = &self.slice[self.index..];

        if !forbid_control_characters && forbid_newlines_in_strings {
            self.index += memchr::memchr2(b'"', b'\\', rest).unwrap_or(rest.len());
            return;
        }

        if !forbid_newlines_in_strings {
            // This is a serde_json_lenient change. In contexts where we've been asked
            // to allow newlines in strings, skip straight to the slow path so we don't
            // have to make the algorithm below generic over this possibility.
            self.skip_to_escape_slow(forbid_control_characters, forbid_newlines_in_strings);
            return;
        }

        // We wish to find the first byte in range 0x00..=0x1F or " or \. Ideally, we'd use
        // something akin to memchr3, but the memchr crate does not support this at the moment.
        // Therefore, we use a variation on Mycroft's algorithm [1] to provide performance better
        // than a naive loop. It runs faster than equivalent two-pass memchr2+SWAR code on
        // benchmarks and it's cross-platform, so probably the right fit.
        // [1]: https://groups.google.com/forum/#!original/comp.lang.c/2HtQXvg7iKc/xOJeipH6KLMJ

        #[cfg(fast_arithmetic = "64")]
        type Chunk = u64;
        #[cfg(fast_arithmetic = "32")]
        type Chunk = u32;

        const STEP: usize = mem::size_of::<Chunk>();
        const ONE_BYTES: Chunk = Chunk::MAX / 255; // 0x0101...01

        for chunk in rest.chunks_exact(STEP) {
            let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
            let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
            let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
            let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
            let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
            let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES) & !chars_backslash;
            let masked = (contains_ctrl | contains_quote | contains_backslash) & (ONE_BYTES << 7);
            if masked != 0 {
                // SAFETY: chunk is in-bounds for slice
                self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) } as usize
                    + masked.trailing_zeros() as usize / 8;
                return;
            }
        }

        self.index += rest.len() / STEP * STEP;
        self.skip_to_escape_slow(forbid_control_characters, forbid_newlines_in_strings);
    }

    #[cold]
    #[inline(never)]
    fn skip_to_escape_slow(&mut self, forbid_control_characters: bool, forbid_newlines_in_strings: bool) {
        while self.index < self.slice.len() && !is_escape(self.slice[self.index], forbid_control_characters, forbid_newlines_in_strings) {
            self.index += 1;
        }
    }

    /// The big optimization here over IoRead is that if the string contains no
    /// backslash escape sequences, the returned &str is a slice of the raw JSON
    /// data so we avoid copying into the scratch space.
    #[allow(clippy::needless_pass_by_value)]
    fn parse_str_bytes<'s, T, S>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        utf_strategy: S,
    ) -> Result<Reference<'a, 's, T>>
    where
        T: ?Sized + 's,
        S: UtfOutputStrategy<T>,
    {
        // Index of the first byte not yet copied into the scratch space.
        let mut start = self.index;

        loop {
            self.skip_to_escape(validate && !self.allow_control_characters_in_string, validate && !self.allow_newlines_in_string);
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    if scratch.is_empty() {
                        // Fast path: return a slice of the raw JSON without any
                        // copying.
                        let borrowed = &self.slice[start..self.index];
                        self.index += 1;
                        return utf_strategy.to_result_direct(self, borrowed, scratch);
                    } else {
                        utf_strategy.extend_scratch(scratch, &self.slice[start..self.index]);
                        self.index += 1;
                        return utf_strategy
                            .to_result_from_scratch(self, scratch)
                            .map(|r| Reference::Copied(r));
                    }
                }
                b'\\' => {
                    utf_strategy.extend_scratch(scratch, &self.slice[start..self.index]);
                    self.index += 1;
                    tri!(parse_escape(self, validate, scratch));
                    start = self.index;
                }
                _ => {
                    self.index += 1;
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
}

impl<'a> private::Sealed for SliceRead<'a> {}

impl<'a> Read<'a> for SliceRead<'a> {
    fn replace_invalid_unicode(&self) -> bool {
        self.replace_invalid_characters
    }

    fn allow_x_escapes(&self) -> bool {
        self.allow_x_escapes
    }

    fn allow_v_escapes(&self) -> bool {
        self.allow_v_escapes
    }

    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| { self.index += 1; *ch }))`
        // is about 10% slower.
        Ok(if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        })
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| *ch))` is about 10% slower
        // for some reason.
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        })
    }

    #[inline]
    fn discard(&mut self) {
        self.index += 1;
    }

    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }

    fn peek_position(&self) -> Position {
        // Cap it at slice.len() just in case the most recent call was next()
        // and it returned the last byte.
        self.position_of_index(cmp::min(self.slice.len(), self.index + 1))
    }

    fn byte_offset(&self) -> usize {
        self.index
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
        if self.replace_invalid_characters {
            self.parse_str_bytes(scratch, true, SubstitutingStrUtfOutputStrategy)
        } else {
            self.parse_str_bytes(scratch, true, StrUtfOutputStrategy)
        }
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, SliceUtfOutputStrategy)
    }

    fn ignore_str(&mut self) -> Result<()> {
        loop {
            self.skip_to_escape(!self.allow_control_characters_in_string, !self.allow_newlines_in_string);
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    self.index += 1;
                    return Ok(());
                }
                b'\\' => {
                    self.index += 1;
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }

    #[inline]
    fn decode_hex_escape(&mut self) -> Result<u16> {
        match self.slice[self.index..] {
            [a, b, c, d, ..] => {
                self.index += 4;
                match decode_four_hex_digits(a, b, c, d) {
                    Some(val) => Ok(val),
                    None => error(self, ErrorCode::InvalidEscape),
                }
            }
            _ => {
                self.index = self.slice.len();
                error(self, ErrorCode::EofWhileParsingString)
            }
        }
    }

    #[inline]
    fn decode_hex_escape_2(&mut self) -> Result<u16> {
        match self.slice[self.index..] {
            [a, b, ..] => {
                self.index += 2;
                match decode_two_hex_digits(a, b) {
                    Some(val) => Ok(val),
                    None => error(self, ErrorCode::InvalidEscape),
                }
            }
            _ => {
                self.index = self.slice.len();
                error(self, ErrorCode::EofWhileParsingString)
            }
        }
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffering_start_index = self.index;
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self.slice[self.raw_buffering_start_index..self.index];
        let raw = match str::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor.visit_map(BorrowedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = false;

    #[inline]
    #[cold]
    fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> StrRead<'a> {
    /// Create a JSON input source to read from a UTF-8 string.
    pub fn new(s: &'a str) -> Self {
        StrRead {
            delegate: SliceRead::new(s.as_bytes(), false, false, false, false, false),
            #[cfg(feature = "raw_value")]
            data: s,
        }
    }
}

impl<'a> private::Sealed for StrRead<'a> {}

impl<'a> Read<'a> for StrRead<'a> {
    fn replace_invalid_unicode(&self) -> bool {
        false
    }

    fn allow_x_escapes(&self) -> bool {
        false
    }

    fn allow_v_escapes(&self) -> bool {
        false
    }

    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        self.delegate.next()
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        self.delegate.peek()
    }

    #[inline]
    fn discard(&mut self) {
        self.delegate.discard();
    }

    fn position(&self) -> Position {
        self.delegate.position()
    }

    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }

    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
        self.delegate
            .parse_str_bytes(scratch, true, UncheckedStrUtfOutputStrategy)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.delegate.parse_str_raw(scratch)
    }

    fn ignore_str(&mut self) -> Result<()> {
        self.delegate.ignore_str()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        self.delegate.decode_hex_escape()
    }

    fn decode_hex_escape_2(&mut self) -> Result<u16> {
        self.delegate.decode_hex_escape_2()
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.delegate.begin_raw_buffering();
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self.data[self.delegate.raw_buffering_start_index..self.delegate.index];
        visitor.visit_map(BorrowedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = false;

    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        self.delegate.set_failed(failed);
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'de, R> private::Sealed for &mut R where R: Read<'de> {}

impl<'de, R> Read<'de> for &mut R
where
    R: Read<'de>,
{
    fn next(&mut self) -> Result<Option<u8>> {
        R::next(self)
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        R::peek(self)
    }

    fn discard(&mut self) {
        R::discard(self);
    }

    fn position(&self) -> Position {
        R::position(self)
    }

    fn peek_position(&self) -> Position {
        R::peek_position(self)
    }

    fn byte_offset(&self) -> usize {
        R::byte_offset(self)
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        R::parse_str(self, scratch)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        R::parse_str_raw(self, scratch)
    }

    fn ignore_str(&mut self) -> Result<()> {
        R::ignore_str(self)
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        R::decode_hex_escape(self)
    }

    fn decode_hex_escape_2(&mut self) -> Result<u16> {
        R::decode_hex_escape_2(self)
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        R::begin_raw_buffering(self);
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        R::end_raw_buffering(self, visitor)
    }

    const should_early_return_if_failed: bool = R::should_early_return_if_failed;

    fn set_failed(&mut self, failed: &mut bool) {
        R::set_failed(self, failed);
    }

    fn replace_invalid_unicode(&self) -> bool {
        R::replace_invalid_unicode(self)
    }

    fn allow_x_escapes(&self) -> bool {
        R::allow_x_escapes(self)
    }

    fn allow_v_escapes(&self) -> bool {
        R::allow_v_escapes(self)
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Marker for whether StreamDeserializer can implement FusedIterator.
pub trait Fused: private::Sealed {}
impl<'a> Fused for SliceRead<'a> {}
impl<'a> Fused for StrRead<'a> {}

fn is_escape(ch: u8, including_control_characters: bool, including_newlines: bool) -> bool {
    if ch == b'"' || ch == b'\\' {
        return true;
    }
    if (ch == b'\n' || ch == b'\r') && !including_newlines {
        return false;
    }
    ch < 0x20 && including_control_characters
}

fn next_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.next()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
    }
}

fn peek_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.peek()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
    }
}

fn error<'de, R, T>(read: &R, reason: ErrorCode) -> Result<T>
where
    R: ?Sized + Read<'de>,
{
    let position = read.position();
    Err(Error::syntax(reason, position.line, position.column))
}

// In the event of an error, if replacing invalid unicode, just return REPLACEMENT CHARACTER.
// Otherwise, discard the peeked byte representing the error if necessary and fall back to
// error().
fn error_or_replace<'de, R: Read<'de>>(read: &mut R, scratch: &mut Vec<u8>, need_discard: bool, reason: ErrorCode) -> Result<()> {
    if read.replace_invalid_unicode() {
        scratch.extend("\u{fffd}".as_bytes());
        Ok(())
    } else {
        if need_discard {
            read.discard();
        }
        error(read, reason)
    }
}

/// Parses a JSON escape sequence and appends it into the scratch space. Assumes
/// the previous byte read was a backslash.
fn parse_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let ch = tri!(next_or_eof(read));

    match ch {
        b'"' => scratch.push(b'"'),
        b'\\' => scratch.push(b'\\'),
        b'/' => scratch.push(b'/'),
        b'b' => scratch.push(b'\x08'),
        b'f' => scratch.push(b'\x0c'),
        b'n' => scratch.push(b'\n'),
        b'r' => scratch.push(b'\r'),
        b't' => scratch.push(b'\t'),
        b'v' if read.allow_v_escapes() => scratch.push(b'\x0b'),
        b'x' if read.allow_x_escapes() => {
            let c: u32 = tri!(read.decode_hex_escape_2()).into();
            let c = match char::from_u32(c) {
                Some(c) => c,
                None => {
                    return error_or_replace(read, scratch, false, ErrorCode::InvalidUnicodeCodePoint);
                }
            };
            scratch.extend_from_slice(c.encode_utf8(&mut [0_u8; 4]).as_bytes());
        }
        b'u' => return parse_unicode_escape(read, validate, scratch),
        _ => return error(read, ErrorCode::InvalidEscape),
    }

    Ok(())
}

/// Parses a JSON \u escape and appends it into the scratch space. Assumes `\u`
/// has just been read.
#[cold]
fn parse_unicode_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let mut n = tri!(read.decode_hex_escape());

    // Non-BMP characters are encoded as a sequence of two hex escapes,
    // representing UTF-16 surrogates. If deserializing a utf-8 string the
    // surrogates are required to be paired, whereas deserializing a byte string
    // accepts lone surrogates.
    if validate && n >= 0xDC00 && n <= 0xDFFF {
        // XXX: This is actually a trailing surrogate.
        return error_or_replace(read, scratch, false, ErrorCode::LoneLeadingSurrogateInHexEscape);
    }

    loop {
        if n < 0xD800 || n > 0xDBFF {
            // Every u16 outside of the surrogate ranges is guaranteed to be a
            // legal char.
            push_wtf8_codepoint(n as u32, scratch);
            return Ok(());
        }

        // n is a leading surrogate, we now expect a trailing surrogate.
        let n1 = n;

        if tri!(peek_or_eof(read)) != b'\\' {
            return if validate {
                error_or_replace(read, scratch, true, ErrorCode::UnexpectedEndOfHexEscape)
            } else {
                push_wtf8_codepoint(n1 as u32, scratch);
                Ok(())
            };
        }
        // Read past the `\` and expect `u`.
        read.discard();

        if tri!(peek_or_eof(read)) != b'u' {
            return if validate {
                error_or_replace(read, scratch, true, ErrorCode::UnexpectedEndOfHexEscape)
            } else {
                push_wtf8_codepoint(n1 as u32, scratch);
                Ok(())
            }
            // The \ prior to this byte started an escape sequence, so if there was no error then
            // we need to parse that escape. This recursive call does not blow the stack on
            // malicious input because the escape is not \u, so it will be handled by one of
            // the easy nonrecursive cases.
            .and_then(|()| parse_escape(read, validate, scratch));
        }
        // Read past the `u` and expect a hex escape.
        read.discard();

        let n2 = tri!(read.decode_hex_escape());

        if n2 < 0xDC00 || n2 > 0xDFFF {
            if validate {
                return error_or_replace(read, scratch, false, ErrorCode::LoneLeadingSurrogateInHexEscape);
            }
            push_wtf8_codepoint(n1 as u32, scratch);
            // If n2 is a leading surrogate, we need to restart.
            n = n2;
            continue;
        }

        // This value is in range U+10000..=U+10FFFF, which is always a valid
        // codepoint.
        let n = (((n1 - 0xD800) as u32) << 10 | (n2 - 0xDC00) as u32) + 0x1_0000;
        push_wtf8_codepoint(n, scratch);
        return Ok(());
    }
}

/// Adds a WTF-8 codepoint to the end of the buffer. This is a more efficient
/// implementation of String::push. The codepoint may be a surrogate.
#[inline]
fn push_wtf8_codepoint(n: u32, scratch: &mut Vec<u8>) {
    if n < 0x80 {
        scratch.push(n as u8);
        return;
    }

    scratch.reserve(4);

    // SAFETY: After the `reserve` call, `scratch` has at least 4 bytes of
    // allocated but unintialized memory after its last initialized byte, which
    // is where `ptr` points. All reachable match arms write `encoded_len` bytes
    // to that region and update the length accordingly, and `encoded_len` is
    // always <= 4.
    unsafe {
        let ptr = scratch.as_mut_ptr().add(scratch.len());

        let encoded_len = match n {
            0..=0x7F => unreachable!(),
            0x80..=0x7FF => {
                ptr.write((n >> 6 & 0b0001_1111) as u8 | 0b1100_0000);
                2
            }
            0x800..=0xFFFF => {
                ptr.write((n >> 12 & 0b0000_1111) as u8 | 0b1110_0000);
                ptr.add(1).write((n >> 6 & 0b0011_1111) as u8 | 0b1000_0000);
                3
            }
            0x1_0000..=0x10_FFFF => {
                ptr.write((n >> 18 & 0b0000_0111) as u8 | 0b1111_0000);
                ptr.add(1)
                    .write((n >> 12 & 0b0011_1111) as u8 | 0b1000_0000);
                ptr.add(2).write((n >> 6 & 0b0011_1111) as u8 | 0b1000_0000);
                4
            }
            0x11_0000.. => unreachable!(),
        };
        ptr.add(encoded_len - 1)
            .write((n & 0b0011_1111) as u8 | 0b1000_0000);

        scratch.set_len(scratch.len() + encoded_len);
    }
}

/// Parses a JSON escape sequence and discards the value. Assumes the previous
/// byte read was a backslash.
fn ignore_escape<'de, R>(read: &mut R) -> Result<()>
where
    R: ?Sized + Read<'de>,
{
    let ch = tri!(next_or_eof(read));

    match ch {
        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' | b'v' => {}
        b'u' => {
            // At this point we don't care if the codepoint is valid. We just
            // want to consume it. We don't actually know what is valid or not
            // at this point, because that depends on if this string will
            // ultimately be parsed into a string or a byte buffer in the "real"
            // parse.

            tri!(read.decode_hex_escape());
        }
        b'x' => {
            tri!(read.decode_hex_escape_2());
        }
        _ => {
            return error(read, ErrorCode::InvalidEscape);
        }
    }

    Ok(())
}

const fn decode_hex_val_slow(val: u8) -> Option<u8> {
    match val {
        b'0'..=b'9' => Some(val - b'0'),
        b'A'..=b'F' => Some(val - b'A' + 10),
        b'a'..=b'f' => Some(val - b'a' + 10),
        _ => None,
    }
}

const fn build_hex_table(shift: usize) -> [i16; 256] {
    let mut table = [0; 256];
    let mut ch = 0;
    while ch < 256 {
        table[ch] = match decode_hex_val_slow(ch as u8) {
            Some(val) => (val as i16) << shift,
            None => -1,
        };
        ch += 1;
    }
    table
}

static HEX0: [i16; 256] = build_hex_table(0);
static HEX1: [i16; 256] = build_hex_table(4);

fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
    let a = HEX1[a as usize] as i32;
    let b = HEX0[b as usize] as i32;
    let c = HEX1[c as usize] as i32;
    let d = HEX0[d as usize] as i32;

    let codepoint = ((a | b) << 8) | c | d;

    // A single sign bit check.
    if codepoint >= 0 {
        Some(codepoint as u16)
    } else {
        None
    }
}

fn decode_two_hex_digits(a: u8, b: u8) -> Option<u16> {
    let a = HEX1[a as usize] as i32;
    let b = HEX0[b as usize] as i32;

    let codepoint = a | b;

    // A single sign bit check.
    if codepoint >= 0 {
        Some(codepoint as u16)
    } else {
        None
    }
}
