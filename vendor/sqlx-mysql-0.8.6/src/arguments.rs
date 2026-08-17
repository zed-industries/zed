use crate::encode::{Encode, IsNull};
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo};
pub(crate) use sqlx_core::arguments::*;
use sqlx_core::error::BoxDynError;
use std::ops::Deref;

/// Implementation of [`Arguments`] for MySQL.
#[derive(Debug, Default, Clone)]
pub struct MySqlArguments {
    pub(crate) values: Vec<u8>,
    pub(crate) types: Vec<MySqlTypeInfo>,
    pub(crate) null_bitmap: NullBitMap,
}

impl MySqlArguments {
    pub(crate) fn add<'q, T>(&mut self, value: T) -> Result<(), BoxDynError>
    where
        T: Encode<'q, MySql> + Type<MySql>,
    {
        let ty = value.produces().unwrap_or_else(T::type_info);

        let value_length_before_encoding = self.values.len();
        let is_null = match value.encode(&mut self.values) {
            Ok(is_null) => is_null,
            Err(error) => {
                // reset the value buffer to its previous value if encoding failed so we don't leave a half-encoded value behind
                self.values.truncate(value_length_before_encoding);
                return Err(error);
            }
        };

        self.types.push(ty);
        self.null_bitmap.push(is_null);

        Ok(())
    }
}

impl<'q> Arguments<'q> for MySqlArguments {
    type Database = MySql;

    fn reserve(&mut self, len: usize, size: usize) {
        self.types.reserve(len);
        self.values.reserve(size);
    }

    fn add<T>(&mut self, value: T) -> Result<(), BoxDynError>
    where
        T: Encode<'q, Self::Database> + Type<Self::Database>,
    {
        self.add(value)
    }

    fn len(&self) -> usize {
        self.types.len()
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct NullBitMap {
    bytes: Vec<u8>,
    length: usize,
}

impl NullBitMap {
    fn push(&mut self, is_null: IsNull) {
        let byte_index = self.length / (u8::BITS as usize);
        let bit_offset = self.length % (u8::BITS as usize);

        if bit_offset == 0 {
            self.bytes.push(0);
        }

        self.bytes[byte_index] |= u8::from(is_null.is_null()) << bit_offset;
        self.length += 1;
    }
}

impl Deref for NullBitMap {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn null_bit_map_should_push_is_null() {
        let mut bit_map = NullBitMap::default();

        bit_map.push(IsNull::Yes);
        bit_map.push(IsNull::No);
        bit_map.push(IsNull::Yes);
        bit_map.push(IsNull::No);
        bit_map.push(IsNull::Yes);
        bit_map.push(IsNull::No);
        bit_map.push(IsNull::Yes);
        bit_map.push(IsNull::No);
        bit_map.push(IsNull::Yes);

        assert_eq!([0b01010101, 0b1].as_slice(), bit_map.deref());
    }
}
