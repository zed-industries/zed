mod buf_read_iter;
mod buf_read_or_reader;
mod input_buf;
mod input_source;

use std::io;
use std::io::BufRead;
use std::io::Read;
use std::mem;
use std::mem::MaybeUninit;

#[cfg(feature = "bytes")]
use ::bytes::Bytes;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::coded_input_stream::buf_read_iter::BufReadIter;
use crate::enums::Enum;
use crate::error::ProtobufError;
use crate::error::WireError;
use crate::misc::maybe_ununit_array_assume_init;
use crate::reflect::types::ProtobufTypeBool;
use crate::reflect::types::ProtobufTypeDouble;
use crate::reflect::types::ProtobufTypeFixed;
use crate::reflect::types::ProtobufTypeFixed32;
use crate::reflect::types::ProtobufTypeFixed64;
use crate::reflect::types::ProtobufTypeFloat;
use crate::reflect::types::ProtobufTypeInt32;
use crate::reflect::types::ProtobufTypeInt64;
use crate::reflect::types::ProtobufTypeSfixed32;
use crate::reflect::types::ProtobufTypeSfixed64;
use crate::reflect::types::ProtobufTypeSint32;
use crate::reflect::types::ProtobufTypeSint64;
use crate::reflect::types::ProtobufTypeTrait;
use crate::reflect::types::ProtobufTypeUint32;
use crate::reflect::types::ProtobufTypeUint64;
use crate::reflect::MessageDescriptor;
use crate::unknown::UnknownValue;
use crate::varint::decode::decode_varint32;
use crate::varint::decode::decode_varint64;
use crate::varint::MAX_VARINT_ENCODED_LEN;
use crate::wire_format;
use crate::wire_format::WireType;
use crate::zigzag::decode_zig_zag_32;
use crate::zigzag::decode_zig_zag_64;
use crate::EnumOrUnknown;
use crate::Message;
use crate::MessageDyn;

// Default recursion level limit. 100 is the default value of C++'s implementation.
const DEFAULT_RECURSION_LIMIT: u32 = 100;

// Max allocated vec when reading length-delimited from unknown input stream
pub(crate) const READ_RAW_BYTES_MAX_ALLOC: usize = 10_000_000;

/// Buffered read with handy utilities.
#[derive(Debug)]
pub struct CodedInputStream<'a> {
    source: BufReadIter<'a>,
    recursion_level: u32,
    recursion_limit: u32,
}

impl<'a> CodedInputStream<'a> {
    /// Wrap a `Read`.
    ///
    /// Note resulting `CodedInputStream` is buffered.
    ///
    /// If `Read` is buffered, the resulting stream will be double buffered,
    /// consider using [`from_buf_read`](Self::from_buf_read) instead.
    pub fn new(read: &'a mut dyn Read) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_read(read))
    }

    /// Create from `BufRead`.
    ///
    /// `CodedInputStream` will utilize `BufRead` buffer.
    pub fn from_buf_read(buf_read: &'a mut dyn BufRead) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_buf_read(buf_read))
    }

    /// Read from byte slice
    pub fn from_bytes(bytes: &'a [u8]) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_byte_slice(bytes))
    }

    /// Read from `Bytes`.
    ///
    /// `CodedInputStream` operations like
    /// [`read_tokio_bytes`](crate::CodedInputStream::read_tokio_bytes)
    /// will return a shared copy of this bytes object.
    #[cfg(feature = "bytes")]
    pub fn from_tokio_bytes(bytes: &'a Bytes) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_bytes(bytes))
    }

    fn from_buf_read_iter(source: BufReadIter<'a>) -> CodedInputStream<'a> {
        CodedInputStream {
            source,
            recursion_level: 0,
            recursion_limit: DEFAULT_RECURSION_LIMIT,
        }
    }

    /// Set the recursion limit.
    pub fn set_recursion_limit(&mut self, limit: u32) {
        self.recursion_limit = limit;
    }

    #[inline]
    pub(crate) fn incr_recursion(&mut self) -> crate::Result<()> {
        if self.recursion_level >= self.recursion_limit {
            return Err(ProtobufError::WireError(WireError::OverRecursionLimit).into());
        }
        self.recursion_level += 1;
        Ok(())
    }

    #[inline]
    pub(crate) fn decr_recursion(&mut self) {
        self.recursion_level -= 1;
    }

    /// How many bytes processed
    pub fn pos(&self) -> u64 {
        self.source.pos()
    }

    /// How many bytes until current limit
    pub fn bytes_until_limit(&self) -> u64 {
        self.source.bytes_until_limit()
    }

    /// Read bytes into given `buf`.
    #[inline]
    pub fn read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> crate::Result<()> {
        self.source.read_exact(buf)
    }

    /// Read exact number of bytes as `Bytes` object.
    ///
    /// This operation returns a shared view if `CodedInputStream` is
    /// constructed with `Bytes` parameter.
    #[cfg(feature = "bytes")]
    fn read_raw_tokio_bytes(&mut self, count: usize) -> crate::Result<Bytes> {
        self.source.read_exact_bytes(count)
    }

    /// Read one byte
    #[inline(always)]
    pub fn read_raw_byte(&mut self) -> crate::Result<u8> {
        self.source.read_byte()
    }

    /// Push new limit, return previous limit.
    pub fn push_limit(&mut self, limit: u64) -> crate::Result<u64> {
        self.source.push_limit(limit)
    }

    /// Restore previous limit.
    pub fn pop_limit(&mut self, old_limit: u64) {
        self.source.pop_limit(old_limit);
    }

    /// Are we at EOF?
    #[inline(always)]
    pub fn eof(&mut self) -> crate::Result<bool> {
        self.source.eof()
    }

    /// Check we are at EOF.
    ///
    /// Return error if we are not at EOF.
    pub fn check_eof(&mut self) -> crate::Result<()> {
        let eof = self.eof()?;
        if !eof {
            return Err(ProtobufError::WireError(WireError::UnexpectedEof).into());
        }
        Ok(())
    }

    fn read_raw_varint64_slow(&mut self) -> crate::Result<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        loop {
            if i == MAX_VARINT_ENCODED_LEN {
                return Err(ProtobufError::WireError(WireError::IncorrectVarint).into());
            }
            let b = self.read_raw_byte()?;
            if i == 9 && (b & 0x7f) > 1 {
                return Err(ProtobufError::WireError(WireError::IncorrectVarint).into());
            }
            r = r | (((b & 0x7f) as u64) << (i * 7));
            i += 1;
            if b < 0x80 {
                return Ok(r);
            }
        }
    }

    fn read_raw_varint32_slow(&mut self) -> crate::Result<u32> {
        let v = self.read_raw_varint64_slow()?;
        if v > u32::MAX as u64 {
            return Err(ProtobufError::WireError(WireError::U32Overflow(v)).into());
        }
        Ok(v as u32)
    }

    /// Read varint
    #[inline]
    pub fn read_raw_varint64(&mut self) -> crate::Result<u64> {
        let rem = self.source.remaining_in_buf();

        match decode_varint64(rem)? {
            Some((r, c)) => {
                self.source.consume(c);
                Ok(r)
            }
            None => self.read_raw_varint64_slow(),
        }
    }

    /// Read varint
    #[inline]
    pub fn read_raw_varint32(&mut self) -> crate::Result<u32> {
        let rem = self.source.remaining_in_buf();

        match decode_varint32(rem)? {
            Some((r, c)) => {
                self.source.consume(c);
                Ok(r)
            }
            None => self.read_raw_varint32_slow(),
        }
    }

    #[inline]
    fn read_raw_varint32_or_eof(&mut self) -> crate::Result<Option<u32>> {
        let rem = self.source.remaining_in_buf();
        let v = decode_varint32(rem)?;
        match v {
            Some((r, c)) => {
                self.source.consume(c);
                Ok(Some(r))
            }
            None => {
                if self.eof()? {
                    Ok(None)
                } else {
                    let v = self.read_raw_varint32_slow()?;
                    Ok(Some(v))
                }
            }
        }
    }

    /// Read little-endian 32-bit integer
    pub fn read_raw_little_endian32(&mut self) -> crate::Result<u32> {
        let mut bytes = [MaybeUninit::uninit(); 4];
        self.read_exact(&mut bytes)?;
        // SAFETY: `read_exact` guarantees that the buffer is filled.
        let bytes = unsafe { maybe_ununit_array_assume_init(bytes) };
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read little-endian 64-bit integer
    pub fn read_raw_little_endian64(&mut self) -> crate::Result<u64> {
        let mut bytes = [MaybeUninit::uninit(); 8];
        self.read_exact(&mut bytes)?;
        // SAFETY: `read_exact` guarantees that the buffer is filled.
        let bytes = unsafe { maybe_ununit_array_assume_init(bytes) };
        Ok(u64::from_le_bytes(bytes))
    }

    /// Read tag number as `u32` or None if EOF is reached.
    #[inline]
    pub fn read_raw_tag_or_eof(&mut self) -> crate::Result<Option<u32>> {
        self.read_raw_varint32_or_eof()
    }

    /// Read tag
    #[inline]
    pub(crate) fn read_tag(&mut self) -> crate::Result<wire_format::Tag> {
        let v = self.read_raw_varint32()?;
        wire_format::Tag::new(v)
    }

    /// Read tag, return it is pair (field number, wire type)
    #[inline]
    pub(crate) fn read_tag_unpack(&mut self) -> crate::Result<(u32, WireType)> {
        self.read_tag().map(|t| t.unpack())
    }

    /// Read `double`
    pub fn read_double(&mut self) -> crate::Result<f64> {
        let bits = self.read_raw_little_endian64()?;
        Ok(f64::from_bits(bits))
    }

    /// Read `float`
    pub fn read_float(&mut self) -> crate::Result<f32> {
        let bits = self.read_raw_little_endian32()?;
        Ok(f32::from_bits(bits))
    }

    /// Read `int64`
    pub fn read_int64(&mut self) -> crate::Result<i64> {
        self.read_raw_varint64().map(|v| v as i64)
    }

    /// Read `int32`
    pub fn read_int32(&mut self) -> crate::Result<i32> {
        let v = self.read_int64()?;
        i32::try_from(v).map_err(|_| WireError::I32Overflow(v).into())
    }

    /// Read `uint64`
    pub fn read_uint64(&mut self) -> crate::Result<u64> {
        self.read_raw_varint64()
    }

    /// Read `uint32`
    pub fn read_uint32(&mut self) -> crate::Result<u32> {
        self.read_raw_varint32()
    }

    /// Read `sint64`
    pub fn read_sint64(&mut self) -> crate::Result<i64> {
        self.read_uint64().map(decode_zig_zag_64)
    }

    /// Read `sint32`
    pub fn read_sint32(&mut self) -> crate::Result<i32> {
        self.read_uint32().map(decode_zig_zag_32)
    }

    /// Read `fixed64`
    pub fn read_fixed64(&mut self) -> crate::Result<u64> {
        self.read_raw_little_endian64()
    }

    /// Read `fixed32`
    pub fn read_fixed32(&mut self) -> crate::Result<u32> {
        self.read_raw_little_endian32()
    }

    /// Read `sfixed64`
    pub fn read_sfixed64(&mut self) -> crate::Result<i64> {
        self.read_raw_little_endian64().map(|v| v as i64)
    }

    /// Read `sfixed32`
    pub fn read_sfixed32(&mut self) -> crate::Result<i32> {
        self.read_raw_little_endian32().map(|v| v as i32)
    }

    /// Read `bool`
    pub fn read_bool(&mut self) -> crate::Result<bool> {
        self.read_raw_varint64().map(|v| v != 0)
    }

    pub(crate) fn read_enum_value(&mut self) -> crate::Result<i32> {
        self.read_int32()
    }

    /// Read `enum` as `ProtobufEnum`
    pub fn read_enum<E: Enum>(&mut self) -> crate::Result<E> {
        let i = self.read_enum_value()?;
        match Enum::from_i32(i) {
            Some(e) => Ok(e),
            None => Err(ProtobufError::WireError(WireError::InvalidEnumValue(E::NAME, i)).into()),
        }
    }

    /// Read `enum` as `ProtobufEnumOrUnknown`
    pub fn read_enum_or_unknown<E: Enum>(&mut self) -> crate::Result<EnumOrUnknown<E>> {
        Ok(EnumOrUnknown::from_i32(self.read_int32()?))
    }

    fn read_repeated_packed_fixed_into<T: ProtobufTypeFixed>(
        &mut self,
        target: &mut Vec<T::ProtobufValue>,
    ) -> crate::Result<()> {
        let len_bytes = self.read_raw_varint64()?;

        let reserve = if len_bytes <= READ_RAW_BYTES_MAX_ALLOC as u64 {
            (len_bytes as usize) / (T::ENCODED_SIZE as usize)
        } else {
            // prevent OOM on malformed input
            // probably should truncate
            READ_RAW_BYTES_MAX_ALLOC / (T::ENCODED_SIZE as usize)
        };

        target.reserve(reserve);

        let old_limit = self.push_limit(len_bytes)?;
        while !self.eof()? {
            target.push(T::read(self)?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    fn read_repeated_packed_into<T: ProtobufTypeTrait>(
        &mut self,
        target: &mut Vec<T::ProtobufValue>,
    ) -> crate::Result<()> {
        let len_bytes = self.read_raw_varint64()?;

        // value is at least 1 bytes, so this is lower bound of element count
        let reserve = if len_bytes <= READ_RAW_BYTES_MAX_ALLOC as u64 {
            len_bytes as usize
        } else {
            // prevent OOM on malformed input
            READ_RAW_BYTES_MAX_ALLOC
        };

        target.reserve(reserve);

        let old_limit = self.push_limit(len_bytes)?;
        while !self.eof()? {
            target.push(T::read(self)?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    /// Read repeated packed `double`
    pub fn read_repeated_packed_double_into(&mut self, target: &mut Vec<f64>) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeDouble>(target)
    }

    /// Read repeated packed `float`
    pub fn read_repeated_packed_float_into(&mut self, target: &mut Vec<f32>) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeFloat>(target)
    }

    /// Read repeated packed `int64`
    pub fn read_repeated_packed_int64_into(&mut self, target: &mut Vec<i64>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeInt64>(target)
    }

    /// Read repeated packed `int32`
    pub fn read_repeated_packed_int32_into(&mut self, target: &mut Vec<i32>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeInt32>(target)
    }

    /// Read repeated packed `uint64`
    pub fn read_repeated_packed_uint64_into(&mut self, target: &mut Vec<u64>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeUint64>(target)
    }

    /// Read repeated packed `uint32`
    pub fn read_repeated_packed_uint32_into(&mut self, target: &mut Vec<u32>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeUint32>(target)
    }

    /// Read repeated packed `sint64`
    pub fn read_repeated_packed_sint64_into(&mut self, target: &mut Vec<i64>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeSint64>(target)
    }

    /// Read repeated packed `sint32`
    pub fn read_repeated_packed_sint32_into(&mut self, target: &mut Vec<i32>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeSint32>(target)
    }

    /// Read repeated packed `fixed64`
    pub fn read_repeated_packed_fixed64_into(
        &mut self,
        target: &mut Vec<u64>,
    ) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeFixed64>(target)
    }

    /// Read repeated packed `fixed32`
    pub fn read_repeated_packed_fixed32_into(
        &mut self,
        target: &mut Vec<u32>,
    ) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeFixed32>(target)
    }

    /// Read repeated packed `sfixed64`
    pub fn read_repeated_packed_sfixed64_into(
        &mut self,
        target: &mut Vec<i64>,
    ) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeSfixed64>(target)
    }

    /// Read repeated packed `sfixed32`
    pub fn read_repeated_packed_sfixed32_into(
        &mut self,
        target: &mut Vec<i32>,
    ) -> crate::Result<()> {
        self.read_repeated_packed_fixed_into::<ProtobufTypeSfixed32>(target)
    }

    /// Read repeated packed `bool`
    pub fn read_repeated_packed_bool_into(&mut self, target: &mut Vec<bool>) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeBool>(target)
    }

    /// Read repeated packed enum values into the vector.
    pub(crate) fn read_repeated_packed_enum_values_into(
        &mut self,
        target: &mut Vec<i32>,
    ) -> crate::Result<()> {
        self.read_repeated_packed_into::<ProtobufTypeInt32>(target)
    }

    fn skip_group(&mut self) -> crate::Result<()> {
        self.incr_recursion()?;
        let ret = self.skip_group_no_depth_check();
        self.decr_recursion();
        ret
    }

    fn skip_group_no_depth_check(&mut self) -> crate::Result<()> {
        while !self.eof()? {
            let wire_type = self.read_tag_unpack()?.1;
            if wire_type == WireType::EndGroup {
                break;
            }
            self.skip_field(wire_type)?;
        }
        Ok(())
    }

    /// Read `UnknownValue`
    pub fn read_unknown(&mut self, wire_type: WireType) -> crate::Result<UnknownValue> {
        match wire_type {
            WireType::Varint => self.read_raw_varint64().map(|v| UnknownValue::Varint(v)),
            WireType::Fixed64 => self.read_fixed64().map(|v| UnknownValue::Fixed64(v)),
            WireType::Fixed32 => self.read_fixed32().map(|v| UnknownValue::Fixed32(v)),
            WireType::LengthDelimited => {
                let len = self.read_raw_varint32()?;
                self.read_raw_bytes(len)
                    .map(|v| UnknownValue::LengthDelimited(v))
            }
            WireType::StartGroup => {
                self.skip_group()?;
                // We do not support groups, so just return something.
                Ok(UnknownValue::LengthDelimited(Vec::new()))
            }
            WireType::EndGroup => {
                Err(ProtobufError::WireError(WireError::UnexpectedWireType(wire_type)).into())
            }
        }
    }

    /// Skip field.
    pub fn skip_field(&mut self, wire_type: WireType) -> crate::Result<()> {
        match wire_type {
            WireType::Varint => self.read_raw_varint64().map(|_| ()),
            WireType::Fixed64 => self.read_fixed64().map(|_| ()),
            WireType::Fixed32 => self.read_fixed32().map(|_| ()),
            WireType::LengthDelimited => {
                let len = self.read_raw_varint32()?;
                self.skip_raw_bytes(len)
            }
            WireType::StartGroup => self.skip_group(),
            WireType::EndGroup => {
                Err(ProtobufError::WireError(WireError::UnexpectedWireType(wire_type)).into())
            }
        }
    }

    /// Read raw bytes into the supplied vector.  The vector will be resized as needed and
    /// overwritten.
    pub fn read_raw_bytes_into(&mut self, count: u32, target: &mut Vec<u8>) -> crate::Result<()> {
        self.source.read_exact_to_vec(count as usize, target)
    }

    /// Read exact number of bytes
    pub fn read_raw_bytes(&mut self, count: u32) -> crate::Result<Vec<u8>> {
        let mut r = Vec::new();
        self.read_raw_bytes_into(count, &mut r)?;
        Ok(r)
    }

    /// Skip exact number of bytes
    pub fn skip_raw_bytes(&mut self, count: u32) -> crate::Result<()> {
        self.source.skip_bytes(count)
    }

    /// Read `bytes` field, length delimited
    pub fn read_bytes(&mut self) -> crate::Result<Vec<u8>> {
        let mut r = Vec::new();
        self.read_bytes_into(&mut r)?;
        Ok(r)
    }

    /// Read `bytes` field, length delimited
    #[cfg(feature = "bytes")]
    pub fn read_tokio_bytes(&mut self) -> crate::Result<Bytes> {
        let len = self.read_raw_varint32()?;
        self.read_raw_tokio_bytes(len as usize)
    }

    /// Read `string` field, length delimited
    #[cfg(feature = "bytes")]
    pub fn read_tokio_chars(&mut self) -> crate::Result<Chars> {
        let bytes = self.read_tokio_bytes()?;
        Ok(Chars::from_bytes(bytes).map_err(ProtobufError::Utf8)?)
    }

    /// Read `bytes` field, length delimited
    pub fn read_bytes_into(&mut self, target: &mut Vec<u8>) -> crate::Result<()> {
        let len = self.read_raw_varint32()?;
        self.read_raw_bytes_into(len, target)?;
        Ok(())
    }

    /// Read `string` field, length delimited
    pub fn read_string(&mut self) -> crate::Result<String> {
        let mut r = String::new();
        self.read_string_into(&mut r)?;
        Ok(r)
    }

    /// Read `string` field, length delimited
    pub fn read_string_into(&mut self, target: &mut String) -> crate::Result<()> {
        target.clear();
        // take target's buffer
        let mut vec = mem::replace(target, String::new()).into_bytes();
        self.read_bytes_into(&mut vec)?;

        let s = match String::from_utf8(vec) {
            Ok(t) => t,
            Err(_) => return Err(ProtobufError::WireError(WireError::Utf8Error).into()),
        };
        *target = s;
        Ok(())
    }

    /// Read message, do not check if message is initialized
    pub fn merge_message<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
        self.incr_recursion()?;
        let ret = self.merge_message_no_depth_check(message);
        self.decr_recursion();
        ret
    }

    fn merge_message_no_depth_check<M: Message>(&mut self, message: &mut M) -> crate::Result<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        message.merge_from(self)?;
        self.pop_limit(old_limit);
        Ok(())
    }

    /// Like `merge_message`, but for dynamic messages.
    pub fn merge_message_dyn(&mut self, message: &mut dyn MessageDyn) -> crate::Result<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        message.merge_from_dyn(self)?;
        self.pop_limit(old_limit);
        Ok(())
    }

    /// Read message
    pub fn read_message<M: Message>(&mut self) -> crate::Result<M> {
        let mut r: M = Message::new();
        self.merge_message(&mut r)?;
        r.check_initialized()?;
        Ok(r)
    }

    /// Read message.
    pub fn read_message_dyn(
        &mut self,
        descriptor: &MessageDescriptor,
    ) -> crate::Result<Box<dyn MessageDyn>> {
        let mut r = descriptor.new_instance();
        self.merge_message_dyn(&mut *r)?;
        r.check_initialized_dyn()?;
        Ok(r)
    }
}

impl<'a> Read for CodedInputStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf).map_err(Into::into)
    }
}

impl<'a> BufRead for CodedInputStream<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf().map_err(Into::into)
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt)
    }
}

#[cfg(test)]
mod test {

    use std::fmt::Debug;
    use std::io;
    use std::io::BufRead;
    use std::io::Read;

    use super::CodedInputStream;
    use super::READ_RAW_BYTES_MAX_ALLOC;
    use crate::error::ProtobufError;
    use crate::hex::decode_hex;
    use crate::wire_format::Tag;
    use crate::wire_format::WireType;
    use crate::CodedOutputStream;

    fn test_read_partial<F>(hex: &str, mut callback: F)
    where
        F: FnMut(&mut CodedInputStream),
    {
        let d = decode_hex(hex);
        // Test with buffered reader.
        {
            let mut reader = io::Cursor::new(&d);
            let mut is = CodedInputStream::from_buf_read(&mut reader as &mut dyn BufRead);
            assert_eq!(0, is.pos());
            callback(&mut is);
        }
        // Test from bytes.
        {
            let mut is = CodedInputStream::from_bytes(&d);
            assert_eq!(0, is.pos());
            callback(&mut is);
        }
    }

    fn test_read<F>(hex: &str, mut callback: F)
    where
        F: FnMut(&mut CodedInputStream),
    {
        let len = decode_hex(hex).len();
        test_read_partial(hex, |reader| {
            callback(reader);
            assert!(reader.eof().expect("eof"));
            assert_eq!(len as u64, reader.pos());
        });
    }

    fn test_read_v<F, V>(hex: &str, v: V, mut callback: F)
    where
        F: FnMut(&mut CodedInputStream) -> crate::Result<V>,
        V: PartialEq + Debug,
    {
        test_read(hex, |reader| {
            assert_eq!(v, callback(reader).unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_raw_byte() {
        test_read("17", |is| {
            assert_eq!(23, is.read_raw_byte().unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint() {
        test_read_v("07", 7, |reader| reader.read_raw_varint32());
        test_read_v("07", 7, |reader| reader.read_raw_varint64());

        test_read_v("96 01", 150, |reader| reader.read_raw_varint32());
        test_read_v("96 01", 150, |reader| reader.read_raw_varint64());

        test_read_v(
            "ff ff ff ff ff ff ff ff ff 01",
            0xffffffffffffffff,
            |reader| reader.read_raw_varint64(),
        );

        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| {
            reader.read_raw_varint32()
        });
        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| {
            reader.read_raw_varint64()
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_out_of_range() {
        test_read_partial("ff ff ff ff ff ff ff ff ff 02", |is| {
            assert!(is.read_raw_varint64().is_err());
        });
        test_read_partial("ff ff ff ff ff ff ff ff ff 02", |is| {
            assert!(is.read_raw_varint32().is_err());
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_too_long() {
        // varint cannot have length > 10
        test_read_partial("ff ff ff ff ff ff ff ff ff ff 01", |reader| {
            let error = reader.read_raw_varint64().unwrap_err().0;
            match *error {
                ProtobufError::WireError(..) => (),
                _ => panic!(),
            }
        });
        test_read_partial("ff ff ff ff ff ff ff ff ff ff 01", |reader| {
            let error = reader.read_raw_varint32().unwrap_err().0;
            match *error {
                ProtobufError::WireError(..) => (),
                _ => panic!(),
            }
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_unexpected_eof() {
        test_read_partial("96 97", |reader| {
            let error = reader.read_raw_varint32().unwrap_err().0;
            match *error {
                ProtobufError::WireError(..) => (),
                _ => panic!(),
            }
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_pos() {
        test_read_partial("95 01 98", |reader| {
            assert_eq!(149, reader.read_raw_varint32().unwrap());
            assert_eq!(2, reader.pos());
        });
    }

    #[test]
    fn test_input_stream_read_int32() {
        test_read_v("02", 2, |reader| reader.read_int32());
    }

    #[test]
    fn test_input_stream_read_float() {
        test_read_v("95 73 13 61", 17e19, |is| is.read_float());
    }

    #[test]
    fn test_input_stream_read_double() {
        test_read_v("40 d5 ab 68 b3 07 3d 46", 23e29, |is| is.read_double());
    }

    #[test]
    fn test_input_stream_skip_raw_bytes() {
        test_read("", |reader| {
            reader.skip_raw_bytes(0).unwrap();
        });
        test_read("aa bb", |reader| {
            reader.skip_raw_bytes(2).unwrap();
        });
        test_read("aa bb cc dd ee ff", |reader| {
            reader.skip_raw_bytes(6).unwrap();
        });
    }

    #[test]
    fn test_input_stream_read_raw_bytes() {
        test_read("", |reader| {
            assert_eq!(
                Vec::from(&b""[..]),
                reader.read_raw_bytes(0).expect("read_raw_bytes")
            );
        })
    }

    #[test]
    fn test_input_stream_limits() {
        test_read("aa bb cc", |is| {
            let old_limit = is.push_limit(1).unwrap();
            assert_eq!(1, is.bytes_until_limit());
            let r1 = is.read_raw_bytes(1).unwrap();
            assert_eq!(&[0xaa as u8], &r1[..]);
            is.pop_limit(old_limit);
            let r2 = is.read_raw_bytes(2).unwrap();
            assert_eq!(&[0xbb as u8, 0xcc], &r2[..]);
        });
    }

    #[test]
    fn test_input_stream_io_read() {
        test_read("aa bb cc", |is| {
            let mut buf = [0; 3];
            assert_eq!(Read::read(is, &mut buf).expect("io::Read"), 3);
            assert_eq!(buf, [0xaa, 0xbb, 0xcc]);
        });
    }

    #[test]
    fn test_input_stream_io_bufread() {
        test_read("aa bb cc", |is| {
            assert_eq!(
                BufRead::fill_buf(is).expect("io::BufRead::fill_buf"),
                &[0xaa, 0xbb, 0xcc]
            );
            BufRead::consume(is, 3);
        });
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow for this test.
    fn test_input_stream_read_raw_bytes_into_huge() {
        let mut v = Vec::new();
        for i in 0..READ_RAW_BYTES_MAX_ALLOC + 1000 {
            v.push((i % 10) as u8);
        }

        let mut slice: &[u8] = v.as_slice();

        let mut is = CodedInputStream::new(&mut slice);

        let mut buf = Vec::new();

        is.read_raw_bytes_into(READ_RAW_BYTES_MAX_ALLOC as u32 + 10, &mut buf)
            .expect("read");

        assert_eq!(READ_RAW_BYTES_MAX_ALLOC + 10, buf.len());

        buf.clear();

        is.read_raw_bytes_into(1000 - 10, &mut buf).expect("read");

        assert_eq!(1000 - 10, buf.len());

        assert!(is.eof().expect("eof"));
    }

    // Copy of this test: https://tinyurl.com/34hfavtz
    #[test]
    fn test_skip_group() {
        // Create an output stream with a group in:
        // Field 1: string "field 1"
        // Field 2: group containing:
        //   Field 1: fixed int32 value 100
        //   Field 2: string "ignore me"
        //   Field 3: nested group containing
        //      Field 1: fixed int64 value 1000
        // Field 3: string "field 3"

        let mut vec = Vec::new();
        let mut os = CodedOutputStream::new(&mut vec);
        os.write_tag(1, WireType::LengthDelimited).unwrap();
        os.write_string_no_tag("field 1").unwrap();

        // The outer group...
        os.write_tag(2, WireType::StartGroup).unwrap();
        os.write_tag(1, WireType::Fixed32).unwrap();
        os.write_fixed32_no_tag(100).unwrap();
        os.write_tag(3, WireType::LengthDelimited).unwrap();
        os.write_string_no_tag("ignore me").unwrap();
        // The nested group...
        os.write_tag(3, WireType::StartGroup).unwrap();
        os.write_tag(1, WireType::Fixed64).unwrap();
        os.write_fixed64_no_tag(1000).unwrap();
        // Note: Not sure the field number is relevant for end group...
        os.write_tag(3, WireType::EndGroup).unwrap();

        // End the outer group
        os.write_tag(2, WireType::EndGroup).unwrap();

        os.write_tag(3, WireType::LengthDelimited).unwrap();
        os.write_string_no_tag("field 3").unwrap();
        os.flush().unwrap();
        drop(os);

        let mut input = CodedInputStream::from_bytes(&vec);
        // Now act like a generated client
        assert_eq!(
            Tag::make(1, WireType::LengthDelimited),
            input.read_tag().unwrap()
        );
        assert_eq!("field 1", &input.read_string().unwrap());
        assert_eq!(
            Tag::make(2, WireType::StartGroup),
            input.read_tag().unwrap()
        );
        input.skip_field(WireType::StartGroup).unwrap();
        assert_eq!(
            Tag::make(3, WireType::LengthDelimited),
            input.read_tag().unwrap()
        );
        assert_eq!("field 3", input.read_string().unwrap());
    }

    #[test]
    fn test_shallow_nested_unknown_groups() {
        // Test skip_group() succeeds on a start group tag 50 times
        // followed by end group tag 50 times. We should be able to
        // successfully skip the outermost group.
        let mut vec = Vec::new();
        let mut os = CodedOutputStream::new(&mut vec);
        for _ in 0..50 {
            os.write_tag(1, WireType::StartGroup).unwrap();
        }
        for _ in 0..50 {
            os.write_tag(1, WireType::EndGroup).unwrap();
        }
        drop(os);

        let mut input = CodedInputStream::from_bytes(&vec);
        assert!(input.skip_group().is_ok());
    }

    #[test]
    fn test_deeply_nested_unknown_groups() {
        // Create an output stream that has groups nested recursively 1000
        // deep, and try to skip the group.
        // This should fail the default depth limit of 100 which ensures we
        // don't blow the stack on adversial input.
        let mut vec = Vec::new();
        let mut os = CodedOutputStream::new(&mut vec);
        for _ in 0..1000 {
            os.write_tag(1, WireType::StartGroup).unwrap();
        }
        for _ in 0..1000 {
            os.write_tag(1, WireType::EndGroup).unwrap();
        }
        drop(os);

        let mut input = CodedInputStream::from_bytes(&vec);
        assert!(input
            .skip_group()
            .unwrap_err()
            .to_string()
            .contains("Over recursion limit"));
    }
}
