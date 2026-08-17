use super::{BorshSchemaContainer, Declaration, Definition, Fields};
use crate::__private::maybestd::{string::ToString, vec::Vec};

use core::num::NonZeroUsize;

/// NonZeroUsize of value one.
// TODO: Replace usage by NonZeroUsize::MIN once MSRV is 1.70+.
const ONE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };

impl BorshSchemaContainer {
    /// Returns the largest possible size of a serialised object based solely on its type.
    ///
    /// Even when if returned upper bound is correct, the theoretical value may be
    /// *much* larger than any practical length.  For example, maximum encoded
    /// length of `String` is 4 GiB while in practice one may encounter strings of
    /// at most dozen of characters.
    ///
    /// # Example
    ///
    /// ```
    /// use borsh::schema::BorshSchemaContainer;
    ///
    /// let schema = BorshSchemaContainer::for_type::<()>();
    /// assert_eq!(Ok(0), schema.max_serialized_size());
    ///
    /// let schema = BorshSchemaContainer::for_type::<usize>();
    /// assert_eq!(Ok(8), schema.max_serialized_size());
    ///
    /// // 4 bytes of length and u32::MAX for the longest possible string.
    /// let schema = BorshSchemaContainer::for_type::<String>();
    /// assert_eq!(Ok(4 + 4294967295), schema.max_serialized_size());
    ///
    /// let schema = BorshSchemaContainer::for_type::<Vec<String>>();
    /// assert_eq!(Err(borsh::schema::SchemaMaxSerializedSizeError::Overflow),
    ///            schema.max_serialized_size());
    /// ```
    pub fn max_serialized_size(&self) -> Result<usize, Error> {
        let mut stack = Vec::new();
        max_serialized_size_impl(ONE, self.declaration(), self, &mut stack)
    }
}

/// Possible error when calculating theoretical maximum size of encoded type `T`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// The theoretical maximum size of the encoded value overflows `usize`.
    ///
    /// This may happen for nested dynamically-sized types such as
    /// `Vec<Vec<u8>>` whose maximum size is `4 + u32::MAX * (4 + u32::MAX)`.
    Overflow,

    /// The type is recursive and thus theoretical maximum size is infinite.
    ///
    /// Simple type in which this triggers is `struct Rec(Option<Box<Rec>>)`.
    Recursive,

    /// Some of the declared types were lacking definition making it impossible
    /// to calculate the size.
    MissingDefinition(Declaration),
}

/// Implementation of [`BorshSchema::max_serialized_size`].
fn max_serialized_size_impl<'a>(
    count: NonZeroUsize,
    declaration: &'a str,
    schema: &'a BorshSchemaContainer,
    stack: &mut Vec<&'a str>,
) -> Result<usize, Error> {
    use core::convert::TryFrom;

    /// Maximum number of elements in a vector or length of a string which can
    /// be serialised.
    const MAX_LEN: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(u32::MAX as usize) };

    fn add(x: usize, y: usize) -> Result<usize, Error> {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    fn mul(x: NonZeroUsize, y: usize) -> Result<usize, Error> {
        x.get().checked_mul(y).ok_or(Error::Overflow)
    }

    /// Calculates max serialised size of a tuple with given members.
    fn tuple<'a>(
        count: NonZeroUsize,
        elements: impl core::iter::IntoIterator<Item = &'a Declaration>,
        schema: &'a BorshSchemaContainer,
        stack: &mut Vec<&'a str>,
    ) -> Result<usize, Error> {
        let mut sum: usize = 0;
        for el in elements {
            sum = add(sum, max_serialized_size_impl(ONE, el, schema, stack)?)?;
        }
        mul(count, sum)
    }

    if stack.iter().any(|dec| *dec == declaration) {
        return Err(Error::Recursive);
    }
    stack.push(declaration);

    let res = match schema.get_definition(declaration).ok_or(declaration) {
        Ok(Definition::Primitive(size)) => match size {
            0 => Ok(0),
            size => {
                let count_sizes = usize::from(*size).checked_mul(count.get());
                count_sizes.ok_or(Error::Overflow)
            }
        },
        Ok(Definition::Sequence {
            length_width,
            length_range,
            elements,
        }) => {
            // Assume sequence has the maximum number of elements.
            let max_len = *length_range.end();
            let sz = match usize::try_from(max_len).map(NonZeroUsize::new) {
                Ok(Some(max_len)) => max_serialized_size_impl(max_len, elements, schema, stack)?,
                Ok(None) => 0,
                Err(_) if is_zero_size_impl(elements, schema, stack)? => 0,
                Err(_) => return Err(Error::Overflow),
            };
            mul(count, add(sz, usize::from(*length_width))?)
        }

        Ok(Definition::Enum {
            tag_width,
            variants,
        }) => {
            let mut max = 0;
            for (_, _, variant) in variants {
                let sz = max_serialized_size_impl(ONE, variant, schema, stack)?;
                max = max.max(sz);
            }
            add(max, usize::from(*tag_width))
        }

        // Tuples and structs sum sizes of all the members.
        Ok(Definition::Tuple { elements }) => tuple(count, elements, schema, stack),
        Ok(Definition::Struct { fields }) => match fields {
            Fields::NamedFields(fields) => {
                tuple(count, fields.iter().map(|(_, field)| field), schema, stack)
            }
            Fields::UnnamedFields(fields) => tuple(count, fields, schema, stack),
            Fields::Empty => Ok(0),
        },

        Err(declaration) => Err(Error::MissingDefinition(declaration.to_string())),
    }?;

    stack.pop();
    Ok(res)
}

/// Checks whether given declaration schema serialises to an empty string.
///
/// This is used by [`BorshSchemaContainer::max_serialized_size`] to handle weird types
/// such as `[[[(); u32::MAX]; u32::MAX]; u32::MAX]` which serialises to an
/// empty string even though its number of elements overflows `usize`.
///
/// Error value means that the method has been called recursively.
/// A recursive type either has no exit, so it cannot be instantiated
/// or it uses `Definiotion::Enum` or `Definition::Sequence` to exit from recursion
/// which make it non-zero size
pub(super) fn is_zero_size(
    declaration: &Declaration,
    schema: &BorshSchemaContainer,
) -> Result<bool, ZeroSizeError> {
    let mut stack = Vec::new();
    is_zero_size_impl(declaration, schema, &mut stack)
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ZeroSizeError {
    Recursive,
    MissingDefinition(Declaration),
}

impl From<ZeroSizeError> for Error {
    fn from(value: ZeroSizeError) -> Self {
        match value {
            ZeroSizeError::Recursive => Self::Recursive,
            ZeroSizeError::MissingDefinition(declaration) => Self::MissingDefinition(declaration),
        }
    }
}

fn is_zero_size_impl<'a>(
    declaration: &'a str,
    schema: &'a BorshSchemaContainer,
    stack: &mut Vec<&'a str>,
) -> Result<bool, ZeroSizeError> {
    fn all<'a, T: 'a>(
        iter: impl Iterator<Item = T>,
        f_key: impl Fn(&T) -> &'a Declaration,
        schema: &'a BorshSchemaContainer,
        stack: &mut Vec<&'a str>,
    ) -> Result<bool, ZeroSizeError> {
        for element in iter {
            let declaration = f_key(&element);
            if !is_zero_size_impl(declaration.as_str(), schema, stack)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    if stack.iter().any(|dec| *dec == declaration) {
        return Err(ZeroSizeError::Recursive);
    }
    stack.push(declaration);

    let res = match schema.get_definition(declaration).ok_or(declaration) {
        Ok(Definition::Primitive(size)) => *size == 0,
        Ok(Definition::Sequence {
            length_width,
            length_range,
            elements,
        }) => {
            if *length_width == 0 {
                // zero-sized array
                if length_range.clone().count() == 1 && *length_range.start() == 0 {
                    return Ok(true);
                }
                if is_zero_size_impl(elements.as_str(), schema, stack)? {
                    return Ok(true);
                }
            }
            false
        }
        Ok(Definition::Tuple { elements }) => all(elements.iter(), |key| *key, schema, stack)?,
        Ok(Definition::Enum {
            tag_width: 0,
            variants,
        }) => all(
            variants.iter(),
            |(_variant_discrim, _variant_name, declaration)| declaration,
            schema,
            stack,
        )?,
        Ok(Definition::Enum { .. }) => false,
        Ok(Definition::Struct { fields }) => match fields {
            Fields::NamedFields(fields) => all(
                fields.iter(),
                |(_field_name, declaration)| declaration,
                schema,
                stack,
            )?,
            Fields::UnnamedFields(fields) => {
                all(fields.iter(), |declaration| declaration, schema, stack)?
            }
            Fields::Empty => true,
        },

        Err(declaration) => {
            return Err(ZeroSizeError::MissingDefinition(declaration.into()));
        }
    };
    stack.pop();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    // this is not integration test module, so can use __private for ease of imports;
    // it cannot be made integration, as it tests `is_zero_size` function, chosen to be non-pub
    use crate::__private::maybestd::{boxed::Box, string::ToString};

    #[test]
    fn test_is_zero_size_recursive_check_bypassed() {
        use crate as borsh;

        #[derive(::borsh_derive::BorshSchema)]
        struct RecursiveExitSequence(Vec<RecursiveExitSequence>);

        let schema = BorshSchemaContainer::for_type::<RecursiveExitSequence>();
        assert_eq!(Ok(false), is_zero_size(schema.declaration(), &schema));
    }

    #[test]
    fn test_is_zero_size_recursive_check_err() {
        use crate as borsh;

        #[derive(::borsh_derive::BorshSchema)]
        struct RecursiveNoExitStructUnnamed(Box<RecursiveNoExitStructUnnamed>);

        let schema = BorshSchemaContainer::for_type::<RecursiveNoExitStructUnnamed>();
        assert_eq!(
            Err(ZeroSizeError::Recursive),
            is_zero_size(schema.declaration(), &schema)
        );
    }
}
