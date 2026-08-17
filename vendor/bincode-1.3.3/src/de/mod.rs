use config::{BincodeByteOrder, Options};
use std::io::Read;

use self::read::{BincodeRead, IoReader, SliceReader};
use byteorder::ReadBytesExt;
use config::{IntEncoding, SizeLimit};
use serde;
use serde::de::Error as DeError;
use serde::de::IntoDeserializer;
use {Error, ErrorKind, Result};

/// Specialized ways to read data into bincode.
pub mod read;

/// A Deserializer that reads bytes from a buffer.
///
/// This struct should rarely be used.
/// In most cases, prefer the `deserialize_from` function.
///
/// The ByteOrder that is chosen will impact the endianness that
/// is used to read integers out of the reader.
///
/// ```ignore
/// let d = Deserializer::new(&mut some_reader, SizeLimit::new());
/// serde::Deserialize::deserialize(&mut deserializer);
/// let bytes_read = d.bytes_read();
/// ```
pub struct Deserializer<R, O: Options> {
    pub(crate) reader: R,
    options: O,
}

macro_rules! impl_deserialize_literal {
    ($name:ident : $ty:ty = $read:ident()) => {
        #[inline]
        pub(crate) fn $name(&mut self) -> Result<$ty> {
            self.read_literal_type::<$ty>()?;
            self.reader
                .$read::<<O::Endian as BincodeByteOrder>::Endian>()
                .map_err(Into::into)
        }
    };
}

impl<'de, IR: Read, O: Options> Deserializer<IoReader<IR>, O> {
    /// Creates a new Deserializer with a given `Read`er and options.
    pub fn with_reader(r: IR, options: O) -> Self {
        Deserializer {
            reader: IoReader::new(r),
            options,
        }
    }
}

impl<'de, O: Options> Deserializer<SliceReader<'de>, O> {
    /// Creates a new Deserializer that will read from the given slice.
    pub fn from_slice(slice: &'de [u8], options: O) -> Self {
        Deserializer {
            reader: SliceReader::new(slice),
            options,
        }
    }
}

impl<'de, R: BincodeRead<'de>, O: Options> Deserializer<R, O> {
    /// Creates a new Deserializer with the given `BincodeRead`er
    pub fn with_bincode_read(r: R, options: O) -> Deserializer<R, O> {
        Deserializer { reader: r, options }
    }

    pub(crate) fn deserialize_byte(&mut self) -> Result<u8> {
        self.read_literal_type::<u8>()?;
        self.reader.read_u8().map_err(Into::into)
    }

    impl_deserialize_literal! { deserialize_literal_u16 : u16 = read_u16() }
    impl_deserialize_literal! { deserialize_literal_u32 : u32 = read_u32() }
    impl_deserialize_literal! { deserialize_literal_u64 : u64 = read_u64() }

    serde_if_integer128! {
        impl_deserialize_literal! { deserialize_literal_u128 : u128 = read_u128() }
    }

    fn read_bytes(&mut self, count: u64) -> Result<()> {
        self.options.limit().add(count)
    }

    fn read_literal_type<T>(&mut self) -> Result<()> {
        use std::mem::size_of;
        self.read_bytes(size_of::<T>() as u64)
    }

    fn read_vec(&mut self) -> Result<Vec<u8>> {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.get_byte_buffer(len)
    }

    fn read_string(&mut self) -> Result<String> {
        let vec = self.read_vec()?;
        String::from_utf8(vec).map_err(|e| ErrorKind::InvalidUtf8Encoding(e.utf8_error()).into())
    }
}

macro_rules! impl_deserialize_int {
    ($name:ident = $visitor_method:ident ($dser_method:ident)) => {
        #[inline]
        fn $name<V>(self, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'de>,
        {
            visitor.$visitor_method(O::IntEncoding::$dser_method(self)?)
        }
    };
}

impl<'de, 'a, R, O> serde::Deserializer<'de> for &'a mut Deserializer<R, O>
where
    R: BincodeRead<'de>,
    O: Options,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Box::new(ErrorKind::DeserializeAnyNotSupported))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.deserialize_byte()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            value => Err(ErrorKind::InvalidBoolEncoding(value).into()),
        }
    }

    impl_deserialize_int!(deserialize_u16 = visit_u16(deserialize_u16));
    impl_deserialize_int!(deserialize_u32 = visit_u32(deserialize_u32));
    impl_deserialize_int!(deserialize_u64 = visit_u64(deserialize_u64));
    impl_deserialize_int!(deserialize_i16 = visit_i16(deserialize_i16));
    impl_deserialize_int!(deserialize_i32 = visit_i32(deserialize_i32));
    impl_deserialize_int!(deserialize_i64 = visit_i64(deserialize_i64));

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_literal_type::<f32>()?;
        let value = self
            .reader
            .read_f32::<<O::Endian as BincodeByteOrder>::Endian>()?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_literal_type::<f64>()?;
        let value = self
            .reader
            .read_f64::<<O::Endian as BincodeByteOrder>::Endian>()?;
        visitor.visit_f64(value)
    }

    serde_if_integer128! {
        impl_deserialize_int!(deserialize_u128 = visit_u128(deserialize_u128));
        impl_deserialize_int!(deserialize_i128 = visit_i128(deserialize_i128));
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.deserialize_byte()? as u8)
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.deserialize_byte()? as i8)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        use std::str;

        let error = || ErrorKind::InvalidCharEncoding.into();

        let mut buf = [0u8; 4];

        // Look at the first byte to see how many bytes must be read
        self.reader.read_exact(&mut buf[..1])?;
        let width = utf8_char_width(buf[0]);
        if width == 1 {
            return visitor.visit_char(buf[0] as char);
        }
        if width == 0 {
            return Err(error());
        }

        if self.reader.read_exact(&mut buf[1..width]).is_err() {
            return Err(error());
        }

        let res = str::from_utf8(&buf[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or_else(error)?;
        visitor.visit_char(res)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.forward_read_str(len, visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;
        self.read_bytes(len as u64)?;
        self.reader.forward_read_bytes(len, visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.read_vec()?)
    }

    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        impl<'de, 'a, R: 'a, O> serde::de::EnumAccess<'de> for &'a mut Deserializer<R, O>
        where
            R: BincodeRead<'de>,
            O: Options,
        {
            type Error = Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let idx: u32 = O::IntEncoding::deserialize_u32(self)?;
                let val: Result<_> = seed.deserialize(idx.into_deserializer());
                Ok((val?, self))
            }
        }

        visitor.visit_enum(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a, R: Read + 'a, O: Options + 'a> {
            deserializer: &'a mut Deserializer<R, O>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options> serde::de::SeqAccess<'de>
            for Access<'a, R, O>
        {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let value: u8 = serde::de::Deserialize::deserialize(&mut *self)?;
        match value {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(&mut *self),
            v => Err(ErrorKind::InvalidTagEncoding(v as usize).into()),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = O::IntEncoding::deserialize_len(self)?;

        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a, R: Read + 'a, O: Options + 'a> {
            deserializer: &'a mut Deserializer<R, O>,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a, R: BincodeRead<'de> + 'b, O: Options> serde::de::MapAccess<'de>
            for Access<'a, R, O>
        {
            type Error = Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let key =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(key))
                } else {
                    Ok(None)
                }
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                Ok(value)
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        let len = O::IntEncoding::deserialize_len(self)?;

        visitor.visit_map(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "Bincode does not support Deserializer::deserialize_identifier";
        Err(Error::custom(message))
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let message = "Bincode does not support Deserializer::deserialize_ignored_any";
        Err(Error::custom(message))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'de, 'a, R, O> serde::de::VariantAccess<'de> for &'a mut Deserializer<R, O>
where
    R: BincodeRead<'de>,
    O: Options,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

// This function is a copy of core::str::utf8_char_width
fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
