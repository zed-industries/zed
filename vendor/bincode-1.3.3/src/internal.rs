use serde;
use std::io::{Read, Write};
use std::marker::PhantomData;

use config::{Infinite, InternalOptions, Options, SizeLimit, TrailingBytes};
use de::read::BincodeRead;
use Result;

pub(crate) fn serialize_into<W, T: ?Sized, O>(writer: W, value: &T, mut options: O) -> Result<()>
where
    W: Write,
    T: serde::Serialize,
    O: InternalOptions,
{
    if options.limit().limit().is_some() {
        // "compute" the size for the side-effect
        // of returning Err if the bound was reached.
        serialized_size(value, &mut options)?;
    }

    let mut serializer = ::ser::Serializer::<_, O>::new(writer, options);
    serde::Serialize::serialize(value, &mut serializer)
}

pub(crate) fn serialize<T: ?Sized, O>(value: &T, mut options: O) -> Result<Vec<u8>>
where
    T: serde::Serialize,
    O: InternalOptions,
{
    let mut writer = {
        let actual_size = serialized_size(value, &mut options)?;
        Vec::with_capacity(actual_size as usize)
    };

    serialize_into(&mut writer, value, options.with_no_limit())?;
    Ok(writer)
}

pub(crate) fn serialized_size<T: ?Sized, O: InternalOptions>(value: &T, options: O) -> Result<u64>
where
    T: serde::Serialize,
{
    let mut size_counter = ::ser::SizeChecker { options, total: 0 };

    let result = value.serialize(&mut size_counter);
    result.map(|_| size_counter.total)
}

pub(crate) fn deserialize_from<R, T, O>(reader: R, options: O) -> Result<T>
where
    R: Read,
    T: serde::de::DeserializeOwned,
    O: InternalOptions,
{
    deserialize_from_seed(PhantomData, reader, options)
}

pub(crate) fn deserialize_from_seed<'a, R, T, O>(seed: T, reader: R, options: O) -> Result<T::Value>
where
    R: Read,
    T: serde::de::DeserializeSeed<'a>,
    O: InternalOptions,
{
    let reader = ::de::read::IoReader::new(reader);
    deserialize_from_custom_seed(seed, reader, options)
}

pub(crate) fn deserialize_from_custom<'a, R, T, O>(reader: R, options: O) -> Result<T>
where
    R: BincodeRead<'a>,
    T: serde::de::DeserializeOwned,
    O: InternalOptions,
{
    deserialize_from_custom_seed(PhantomData, reader, options)
}

pub(crate) fn deserialize_from_custom_seed<'a, R, T, O>(
    seed: T,
    reader: R,
    options: O,
) -> Result<T::Value>
where
    R: BincodeRead<'a>,
    T: serde::de::DeserializeSeed<'a>,
    O: InternalOptions,
{
    let mut deserializer = ::de::Deserializer::<_, O>::with_bincode_read(reader, options);
    seed.deserialize(&mut deserializer)
}

pub(crate) fn deserialize_in_place<'a, R, T, O>(reader: R, options: O, place: &mut T) -> Result<()>
where
    R: BincodeRead<'a>,
    T: serde::de::Deserialize<'a>,
    O: InternalOptions,
{
    let mut deserializer = ::de::Deserializer::<_, _>::with_bincode_read(reader, options);
    serde::Deserialize::deserialize_in_place(&mut deserializer, place)
}

pub(crate) fn deserialize<'a, T, O>(bytes: &'a [u8], options: O) -> Result<T>
where
    T: serde::de::Deserialize<'a>,
    O: InternalOptions,
{
    deserialize_seed(PhantomData, bytes, options)
}

pub(crate) fn deserialize_seed<'a, T, O>(seed: T, bytes: &'a [u8], options: O) -> Result<T::Value>
where
    T: serde::de::DeserializeSeed<'a>,
    O: InternalOptions,
{
    let options = ::config::WithOtherLimit::new(options, Infinite);

    let reader = ::de::read::SliceReader::new(bytes);
    let mut deserializer = ::de::Deserializer::with_bincode_read(reader, options);
    let val = seed.deserialize(&mut deserializer)?;

    match O::Trailing::check_end(&deserializer.reader) {
        Ok(_) => Ok(val),
        Err(err) => Err(err),
    }
}
