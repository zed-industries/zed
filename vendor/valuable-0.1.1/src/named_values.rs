use core::iter::{self, FusedIterator};

use crate::field::*;
use crate::*;

/// Set of values from a `Structable` or `Enumerable` with named fields.
#[derive(Debug)]
pub struct NamedValues<'a> {
    fields: &'a [NamedField<'a>],
    values: &'a [Value<'a>],
}

impl<'a> NamedValues<'a> {
    /// Create a new `NamedValues` instance.
    ///
    /// Both `fields` and `values` must be the same length.
    ///
    /// # Panics
    ///
    /// The method panics if `fields` and `values` are different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{NamedField, NamedValues, Value};
    ///
    /// let fields = [
    ///     NamedField::new("foo"),
    ///     NamedField::new("bar")
    /// ];
    /// let values = [
    ///     Value::U32(123),
    ///     Value::U32(456),
    /// ];
    ///
    /// let named_values = NamedValues::new(&fields, &values);
    ///
    /// assert_eq!(
    ///     named_values.get(&fields[0]).unwrap().as_u32(),
    ///     Some(123));
    /// ```
    pub fn new(fields: &'a [NamedField<'a>], values: &'a [Value<'a>]) -> NamedValues<'a> {
        assert!(
            fields.len() == values.len(),
            "`fields` and `values` must be the same length"
        );
        NamedValues { fields, values }
    }

    /// Get a value using a `NamedField` reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{NamedField, NamedValues, Value};
    ///
    /// let fields = [
    ///     NamedField::new("foo"),
    ///     NamedField::new("bar")
    /// ];
    /// let values = [
    ///     Value::U32(123),
    ///     Value::U32(456),
    /// ];
    ///
    /// let named_values = NamedValues::new(&fields, &values);
    ///
    /// assert_eq!(
    ///     named_values.get(&fields[0]).unwrap().as_u32(),
    ///     Some(123));
    /// ```
    pub fn get(&self, field: &NamedField<'_>) -> Option<&Value<'_>> {
        use core::mem;

        let idx = (field as *const _ as usize - &self.fields[0] as *const _ as usize)
            / mem::size_of::<NamedField<'_>>();
        self.values.get(idx)
    }

    /// Get a value using string.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{NamedField, NamedValues, Value};
    ///
    /// let fields = [
    ///     NamedField::new("foo"),
    ///     NamedField::new("bar")
    /// ];
    /// let values = [
    ///     Value::U32(123),
    ///     Value::U32(456),
    /// ];
    ///
    /// let named_values = NamedValues::new(&fields, &values);
    ///
    /// assert_eq!(
    ///     named_values.get_by_name("foo").unwrap().as_u32(),
    ///     Some(123));
    /// ```
    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&Value<'_>> {
        let name = name.as_ref();

        for (index, field) in self.fields.iter().enumerate() {
            if field.name() == name {
                return Some(&self.values[index]);
            }
        }

        None
    }

    /// Iterate all name-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{NamedField, NamedValues, Value};
    ///
    /// let fields = [
    ///     NamedField::new("foo"),
    ///     NamedField::new("bar")
    /// ];
    /// let values = [
    ///     Value::U32(123),
    ///     Value::U32(456),
    /// ];
    ///
    /// let named_values = NamedValues::new(&fields, &values);
    ///
    /// for (field, value) in named_values.iter() {
    ///     println!("{:?}: {:?}", field, value);
    /// }
    /// ```
    pub fn iter<'b>(&'b self) -> Iter<'a, 'b> {
        Iter {
            iter: self.fields.iter().enumerate(),
            values: self.values,
        }
    }

    /// Returns the length of fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` if fields have a length of 0.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl<'a, 'b> IntoIterator for &'b NamedValues<'a> {
    type Item = (&'b NamedField<'a>, &'b Value<'a>);
    type IntoIter = Iter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator of name-value pairs contained by [`NamedValues`].
///
/// Instances are created by the [`iter()`][NamedValues::iter] method on
/// [`NamedValues`]. See its documentation for more.
///
/// # Examples
///
/// ```
/// use valuable::{NamedField, NamedValues, Value};
///
/// let fields = [
///     NamedField::new("foo"),
///     NamedField::new("bar")
/// ];
/// let values = [
///     Value::U32(123),
///     Value::U32(456),
/// ];
///
/// let named_values = NamedValues::new(&fields, &values);
///
/// for (field, value) in named_values.iter() {
///     println!("{:?}: {:?}", field, value);
/// }
/// ```
#[derive(Debug)]
pub struct Iter<'a, 'b> {
    iter: iter::Enumerate<core::slice::Iter<'b, NamedField<'a>>>,
    values: &'a [Value<'a>],
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = (&'b NamedField<'a>, &'b Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(move |(i, field)| (field, &self.values[i]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(move |(i, field)| (field, &self.values[i]))
    }
}

impl ExactSizeIterator for Iter<'_, '_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Iter<'_, '_> {}
