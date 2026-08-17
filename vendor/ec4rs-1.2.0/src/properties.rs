mod iter;

pub use iter::*;

use crate::rawvalue::RawValue;
use crate::{PropertyKey, PropertyValue};

/// Map of property names to property values.
///
/// It features O(log n) lookup and preserves insertion order,
/// as well as convenience methods for type-safe access and parsing of values.
///
/// This structure is case-sensitive.
/// It's the caller's responsibility to ensure all keys and values are lowercased.
#[derive(Clone, Default)]
pub struct Properties {
    // Don't use Cow<'static, str> here because it's actually less-optimal
    // for the vastly more-common case of reading parsed properties.
    // It's a micro-optimization anyway.
    /// Key-value pairs, ordered from oldest to newest.
    pairs: Vec<(String, RawValue)>,
    /// Indices of `pairs`, ordered matching the key of the pair each index refers to.
    /// This part is what allows logarithmic lookups.
    idxes: Vec<usize>,
    // Unfortunately, we hand out `&mut RawValue`s all over the place,
    // so "no empty RawValues in Properties" cannot be made an invariant
    // without breaking API changes.
}

// TODO: Deletion.

impl Properties {
    /// Constructs a new empty [`Properties`].
    pub const fn new() -> Properties {
        Properties {
            pairs: Vec::new(),
            idxes: Vec::new(),
        }
    }

    /// Returns the number of key-value pairs, including those with empty values.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns `true` if `self` contains no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Returns either the index of the pair with the desired key in `pairs`,
    /// or the index to insert a new index into `index`.
    fn find_idx(&self, key: &str) -> Result<usize, usize> {
        self.idxes
            .as_slice()
            .binary_search_by_key(&key, |ki| self.pairs[*ki].0.as_str())
            .map(|idx| self.idxes[idx])
    }

    /// Returns the unparsed "raw" value for the specified key.
    ///
    /// Does not test for the "unset" value. Use [`RawValue::filter_unset`].
    pub fn get_raw_for_key(&self, key: impl AsRef<str>) -> &RawValue {
        self.find_idx(key.as_ref())
            .ok()
            .map_or(&crate::rawvalue::UNSET, |idx| &self.pairs[idx].1)
    }

    /// Returns the unparsed "raw" value for the specified property.
    ///
    /// Does not test for the "unset" value. Use [`RawValue::filter_unset`].
    pub fn get_raw<T: PropertyKey>(&self) -> &RawValue {
        self.get_raw_for_key(T::key())
    }

    /// Returns the parsed value for the specified property.
    ///
    /// Does not test for the "unset" value if parsing fails. Use [`RawValue::filter_unset`].
    pub fn get<T: PropertyKey + PropertyValue>(&self) -> Result<T, &RawValue> {
        let retval = self.get_raw::<T>();
        retval.parse::<T>().or(Err(retval))
    }

    /// Returns an iterator over the key-value pairs.
    ///
    /// If the `allow-empty-values` feature is NOT used,
    /// key-value pairs where the value is empty will be skipped.
    /// Otherwise, they will be returned as normal.
    ///
    /// Pairs are returned from oldest to newest.
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.pairs.iter())
    }

    /// Returns an iterator over the key-value pairs that allows mutation of the values.
    ///
    /// If the `allow-empty-values` feature is NOT used,
    /// key-value pairs where the value is empty will be skipped.
    /// Otherwise, they will be returned as normal.
    ///
    /// Pairs are returned from oldest to newest.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut(self.pairs.iter_mut())
    }

    fn get_at_mut(&mut self, idx: usize) -> &mut RawValue {
        &mut self.pairs.get_mut(idx).unwrap().1
    }

    fn insert_at(&mut self, idx: usize, key: String, val: RawValue) {
        self.idxes.insert(idx, self.pairs.len());
        self.pairs.push((key, val));
    }

    /// Sets the value for a specified key.
    pub fn insert_raw_for_key(&mut self, key: impl AsRef<str>, val: impl Into<RawValue>) {
        let key_str = key.as_ref();
        match self.find_idx(key_str) {
            Ok(idx) => {
                *self.get_at_mut(idx) = val.into();
            }
            Err(idx) => {
                self.insert_at(idx, key_str.to_owned(), val.into());
            }
        }
    }

    /// Sets the value for a specified property's key.
    pub fn insert_raw<K: PropertyKey, V: Into<RawValue>>(&mut self, val: V) {
        self.insert_raw_for_key(K::key(), val)
    }

    /// Inserts a specified property into the map.
    pub fn insert<T: PropertyKey + Into<RawValue>>(&mut self, prop: T) {
        self.insert_raw_for_key(T::key(), prop.into())
    }

    /// Attempts to add a new key-value pair to the map.
    ///
    /// If the key was already associated with a value,
    /// returns a mutable reference to the old value and does not update the map.
    pub fn try_insert_raw_for_key(
        &mut self,
        key: impl AsRef<str>,
        value: impl Into<RawValue>,
    ) -> Result<(), &mut RawValue> {
        let key_str = key.as_ref();
        #[allow(clippy::unit_arg)]
        match self.find_idx(key_str) {
            Ok(idx) => {
                let valref = self.get_at_mut(idx);
                if valref.is_unset() {
                    *valref = value.into();
                    Ok(())
                } else {
                    Err(valref)
                }
            }
            Err(idx) => Ok(self.insert_at(idx, key_str.to_owned(), value.into())),
        }
    }

    /// Attempts to add a new property to the map with a specified value.
    ///
    /// If the key was already associated with a value,
    /// returns a mutable reference to the old value and does not update the map.
    pub fn try_insert_raw<K: PropertyKey, V: Into<RawValue>>(
        &mut self,
        val: V,
    ) -> Result<(), &mut RawValue> {
        self.try_insert_raw_for_key(K::key(), val)
    }

    /// Attempts to add a new property to the map.
    ///
    /// If the key was already associated with a value,
    /// returns a mutable reference to the old value and does not update the map.
    pub fn try_insert<T: PropertyKey + Into<RawValue>>(
        &mut self,
        prop: T,
    ) -> Result<(), &mut RawValue> {
        self.try_insert_raw_for_key(T::key(), prop.into())
    }

    /// Adds fallback values for certain common key-value pairs.
    ///
    /// Used to obtain spec-compliant values for [`crate::property::IndentSize`]
    /// and [`crate::property::TabWidth`].
    pub fn use_fallbacks(&mut self) {
        crate::fallback::add_fallbacks(self, false)
    }

    /// Adds pre-0.9.0 fallback values for certain common key-value pairs.
    ///
    /// This shouldn't be used outside of narrow cases where
    /// compatibility with those older standards is required.
    /// Prefer [`Properties::use_fallbacks`] instead.
    pub fn use_fallbacks_legacy(&mut self) {
        crate::fallback::add_fallbacks(self, true)
    }
}

impl PartialEq for Properties {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.idxes
            .iter()
            .zip(other.idxes.iter())
            .all(|(idx_s, idx_o)| self.pairs[*idx_s] == other.pairs[*idx_o])
    }
}

impl Eq for Properties {}

impl std::fmt::Debug for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Properties")
            .field(&self.pairs.as_slice())
            .finish()
    }
}

impl<'a> IntoIterator for &'a Properties {
    type Item = <Iter<'a> as Iterator>::Item;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Properties {
    type Item = <IterMut<'a> as Iterator>::Item;

    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: AsRef<str>, V: Into<RawValue>> FromIterator<(K, V)> for Properties {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut result = Properties::new();
        result.extend(iter);
        result
    }
}

impl<K: AsRef<str>, V: Into<RawValue>> Extend<(K, V)> for Properties {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let min_len = iter.size_hint().0;
        self.pairs.reserve(min_len);
        self.idxes.reserve(min_len);
        for (k, v) in iter {
            let k = k.as_ref();
            let v = v.into();
            self.insert_raw_for_key(k, v);
        }
    }
}

/// Trait for types that can add properties to a [`Properties`] map.
pub trait PropertiesSource {
    /// Adds properties that apply to a file at the specified path
    /// to the provided [`Properties`].
    fn apply_to(
        self,
        props: &mut Properties,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error>;
}

impl<'a> PropertiesSource for &'a Properties {
    fn apply_to(
        self,
        props: &mut Properties,
        _: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        props.extend(self.pairs.iter().cloned());
        Ok(())
    }
}
