use crate::__private::maybestd::vec::Vec;
use crate::from_slice;
use crate::io::{Error, ErrorKind, Result};
use crate::schema::{BorshSchemaContainer, SchemaMaxSerializedSizeError};
use crate::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Deserialize this instance from a slice of bytes, but assume that at the beginning we have
/// bytes describing the schema of the type. We deserialize this schema and verify that it is
/// correct.
pub fn try_from_slice_with_schema<T: BorshDeserialize + BorshSchema>(v: &[u8]) -> Result<T> {
    let (schema, object) = from_slice::<(BorshSchemaContainer, T)>(v)?;
    if schema_container_of::<T>() != schema {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Borsh schema does not match",
        ));
    }
    Ok(object)
}

/// Serialize object into a vector of bytes and prefix with the schema serialized as vector of
/// bytes in Borsh format.
pub fn try_to_vec_with_schema<T: BorshSerialize + BorshSchema + ?Sized>(
    value: &T,
) -> Result<Vec<u8>> {
    let schema = schema_container_of::<T>();
    let mut res = crate::to_vec(&schema)?;
    value.serialize(&mut res)?;
    Ok(res)
}

/// generate [BorshSchemaContainer] for type `T`
///
/// this is an alias of [BorshSchemaContainer::for_type]
pub fn schema_container_of<T: BorshSchema + ?Sized>() -> BorshSchemaContainer {
    BorshSchemaContainer::for_type::<T>()
}

/// Returns the largest possible size of a serialised object based solely on its type `T`.
///
/// this is a shortcut for using [BorshSchemaContainer::max_serialized_size]
/// # Example
///
/// ```
/// use borsh::schema::BorshSchemaContainer;
///
/// assert_eq!(Ok(8), borsh::max_serialized_size::<usize>());
/// ```
pub fn max_serialized_size<T: BorshSchema + ?Sized>(
) -> core::result::Result<usize, SchemaMaxSerializedSizeError> {
    let schema = BorshSchemaContainer::for_type::<T>();
    schema.max_serialized_size()
}
