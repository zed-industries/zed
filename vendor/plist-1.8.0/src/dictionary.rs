//! A map of `String` to `plist::Value`.
//!
//! The map is currently backed by an [`IndexMap`]. This may be changed in a future minor release.
//!
//! [`IndexMap`]: https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html

use indexmap::{map, IndexMap};
use std::{
    fmt::{self, Debug},
    ops,
};

use crate::Value;

/// Represents a plist dictionary type.
#[derive(Clone, Default, PartialEq)]
pub struct Dictionary {
    map: IndexMap<String, Value>,
}

impl Dictionary {
    /// Makes a new empty `Dictionary`.
    #[inline]
    pub fn new() -> Self {
        Dictionary {
            map: IndexMap::new(),
        }
    }

    /// Clears the dictionary, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Returns a reference to the value corresponding to the key.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }

    /// Returns true if the dictionary contains a value for the specified key.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.map.get_mut(key)
    }

    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did not have this key present, `None` is returned.
    ///
    /// If the dictionary did have this key present, the value is updated, and the old value is
    /// returned.
    #[inline]
    pub fn insert(&mut self, k: String, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }

    /// Removes a key from the dictionary, returning the value at the key if the key was previously
    /// in the dictionary.
    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.map.swap_remove(key)
    }

    /// Scan through each key-value pair in the map and keep those where the
    /// closure `keep` returns `true`.
    #[inline]
    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&String, &mut Value) -> bool,
    {
        self.map.retain(keep);
    }

    /// Sort the dictionary keys.
    ///
    /// This uses the default ordering defined on [`str`].
    ///
    /// This function is useful if you are serializing to XML, and wish to
    /// ensure a consistent key order.
    #[inline]
    pub fn sort_keys(&mut self) {
        self.map.sort_keys();
    }

    /// Gets the given key's corresponding entry in the dictionary for in-place manipulation.
    // Entry functionality is unstable until I can figure out how to use either Cow<str> or
    // T: AsRef<str> + Into<String>
    #[cfg(any(
        test,
        feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
    ))]
    pub fn entry<S>(&mut self, key: S) -> Entry<'_>
    where
        S: Into<String>,
    {
        match self.map.entry(key.into()) {
            map::Entry::Vacant(vacant) => Entry::Vacant(VacantEntry { vacant }),
            map::Entry::Occupied(occupied) => Entry::Occupied(OccupiedEntry { occupied }),
        }
    }

    /// Returns the number of elements in the dictionary.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the dictionary contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Gets an iterator over the entries of the dictionary.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            iter: self.map.iter(),
        }
    }

    /// Gets a mutable iterator over the entries of the dictionary.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }

    /// Gets an iterator over the keys of the dictionary.
    #[inline]
    pub fn keys(&self) -> Keys<'_> {
        Keys {
            iter: self.map.keys(),
        }
    }

    /// Gets an iterator over the values of the dictionary.
    #[inline]
    pub fn values(&self) -> Values<'_> {
        Values {
            iter: self.map.values(),
        }
    }

    /// Gets an iterator over mutable values of the dictionary.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_> {
        ValuesMut {
            iter: self.map.values_mut(),
        }
    }
}

/// Access an element of this dictionary. Panics if the given key is not present in the dictionary.
///
/// ```
/// # use plist::Value;
/// #
/// # let val = &Value::String("".to_owned());
/// # let _ =
/// match *val {
///     Value::Array(ref arr) => arr[0].as_string(),
///     Value::Dictionary(ref dict) => dict["type"].as_string(),
///     Value::String(ref s) => Some(s.as_str()),
///     _ => None,
/// }
/// # ;
/// ```
impl ops::Index<&str> for Dictionary {
    type Output = Value;

    fn index(&self, index: &str) -> &Value {
        self.map.index(index)
    }
}

/// Mutably access an element of this dictionary. Panics if the given key is not present in the
/// dictionary.
///
/// ```
/// # let mut dict = plist::Dictionary::new();
/// # dict.insert("key".to_owned(), plist::Value::Boolean(false));
/// #
/// dict["key"] = "value".into();
/// ```
impl ops::IndexMut<&str> for Dictionary {
    fn index_mut(&mut self, index: &str) -> &mut Value {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

impl Debug for Dictionary {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.map.fmt(formatter)
    }
}

impl<K: Into<String>, V: Into<Value>> FromIterator<(K, V)> for Dictionary {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Dictionary {
            map: iter
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl Extend<(String, Value)> for Dictionary {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        self.map.extend(iter);
    }
}

macro_rules! delegate_iterator {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* {
            type Item = $item;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }
        }

        /*impl $($generics)* DoubleEndedIterator for $name $($generics)* {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }*/

        impl $($generics)* ExactSizeIterator for $name $($generics)* {
            #[inline]
            fn len(&self) -> usize {
                self.iter.len()
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A view into a single entry in a dictionary, which may either be vacant or occupied.
/// This enum is constructed from the [`entry`] method on [`Dictionary`].
///
/// [`entry`]: struct.Dictionary.html#method.entry
/// [`Dictionary`]: struct.Dictionary.html
#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
pub enum Entry<'a> {
    /// A vacant Entry.
    Vacant(VacantEntry<'a>),
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a>),
}

/// A vacant Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
pub struct VacantEntry<'a> {
    vacant: map::VacantEntry<'a, String, Value>,
}

/// An occupied Entry. It is part of the [`Entry`] enum.
///
/// [`Entry`]: enum.Entry.html
#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
pub struct OccupiedEntry<'a> {
    occupied: map::OccupiedEntry<'a, String, Value>,
}

#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
impl<'a> Entry<'a> {
    /// Returns a reference to this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dict = plist::Dictionary::new();
    /// assert_eq!(dict.entry("serde").key(), &"serde");
    /// ```
    pub fn key(&self) -> &String {
        match *self {
            Entry::Vacant(ref e) => e.key(),
            Entry::Occupied(ref e) => e.key(),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable
    /// reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dict = plist::Dictionary::new();
    /// dict.entry("serde").or_insert(12.into());
    ///
    /// assert_eq!(dict["serde"], 12.into());
    /// ```
    pub fn or_insert(self, default: Value) -> &'a mut Value {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dict = plist::Dictionary::new();
    /// dict.entry("serde").or_insert_with(|| "hoho".into());
    ///
    /// assert_eq!(dict["serde"], "hoho".into());
    /// ```
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Value
    where
        F: FnOnce() -> Value,
    {
        match self {
            Entry::Vacant(entry) => entry.insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }
}

#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
impl<'a> VacantEntry<'a> {
    /// Gets a reference to the key that would be used when inserting a value through the
    /// VacantEntry.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    ///
    /// match dict.entry("serde") {
    ///     Entry::Vacant(vacant) => assert_eq!(vacant.key(), &"serde"),
    ///     Entry::Occupied(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn key(&self) -> &String {
        self.vacant.key()
    }

    /// Sets the value of the entry with the VacantEntry's key, and returns a mutable reference
    /// to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    ///
    /// match dict.entry("serde") {
    ///     Entry::Vacant(vacant) => vacant.insert("hoho".into()),
    ///     Entry::Occupied(_) => unimplemented!(),
    /// };
    /// ```
    #[inline]
    pub fn insert(self, value: Value) -> &'a mut Value {
        self.vacant.insert(value)
    }
}

#[cfg(any(
    test,
    feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"
))]
impl<'a> OccupiedEntry<'a> {
    /// Gets a reference to the key in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), 12.into());
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(occupied) => assert_eq!(occupied.key(), &"serde"),
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn key(&self) -> &String {
        self.occupied.key()
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::Value;
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), 12.into());
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(occupied) => assert_eq!(occupied.get(), &Value::from(12)),
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> &Value {
        self.occupied.get()
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::Value;
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), Value::Array(vec![1.into(), 2.into(), 3.into()]));
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         occupied.get_mut().as_array_mut().unwrap().push(4.into());
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    ///
    /// assert_eq!(dict["serde"].as_array().unwrap().len(), 4);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut Value {
        self.occupied.get_mut()
    }

    /// Converts the entry into a mutable reference to its value.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::Value;
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), Value::Array(vec![1.into(), 2.into(), 3.into()]));
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         occupied.into_mut().as_array_mut().unwrap().push(4.into());
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    ///
    /// assert_eq!(dict["serde"].as_array().unwrap().len(), 4);
    /// ```
    #[inline]
    pub fn into_mut(self) -> &'a mut Value {
        self.occupied.into_mut()
    }

    /// Sets the value of the entry with the `OccupiedEntry`'s key, and returns
    /// the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::Value;
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), 12.into());
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(mut occupied) => {
    ///         assert_eq!(occupied.insert(13.into()), 12.into());
    ///         assert_eq!(occupied.get(), &Value::from(13));
    ///     }
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn insert(&mut self, value: Value) -> Value {
        self.occupied.insert(value)
    }

    /// Takes the value of the entry out of the dictionary, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use plist::dictionary::Entry;
    ///
    /// let mut dict = plist::Dictionary::new();
    /// dict.insert("serde".to_owned(), 12.into());
    ///
    /// match dict.entry("serde") {
    ///     Entry::Occupied(occupied) => assert_eq!(occupied.remove(), 12.into()),
    ///     Entry::Vacant(_) => unimplemented!(),
    /// }
    /// ```
    #[inline]
    pub fn remove(self) -> Value {
        self.occupied.swap_remove()
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a Dictionary {
    type Item = (&'a String, &'a Value);
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.map.iter(),
        }
    }
}

/// An iterator over a `plist::Dictionary`'s entries.
pub struct Iter<'a> {
    iter: IterImpl<'a>,
}

type IterImpl<'a> = map::Iter<'a, String, Value>;

delegate_iterator!((Iter<'a>) => (&'a String, &'a Value));

//////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a mut Dictionary {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = IterMut<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
}

/// A mutable iterator over a `plist::Dictionary`'s entries.
pub struct IterMut<'a> {
    iter: map::IterMut<'a, String, Value>,
}

delegate_iterator!((IterMut<'a>) => (&'a String, &'a mut Value));

//////////////////////////////////////////////////////////////////////////////

impl IntoIterator for Dictionary {
    type Item = (String, Value);
    type IntoIter = IntoIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}

/// An owning iterator over a `plist::Dictionary`'s entries.
pub struct IntoIter {
    iter: map::IntoIter<String, Value>,
}

delegate_iterator!((IntoIter) => (String, Value));

//////////////////////////////////////////////////////////////////////////////

/// An iterator over a `plist::Dictionary`'s keys.
pub struct Keys<'a> {
    iter: map::Keys<'a, String, Value>,
}

delegate_iterator!((Keys<'a>) => &'a String);

//////////////////////////////////////////////////////////////////////////////

/// An iterator over a `plist::Dictionary`'s values.
pub struct Values<'a> {
    iter: map::Values<'a, String, Value>,
}

delegate_iterator!((Values<'a>) => &'a Value);

//////////////////////////////////////////////////////////////////////////////

/// A mutable iterator over a `plist::Dictionary`'s values.
pub struct ValuesMut<'a> {
    iter: map::ValuesMut<'a, String, Value>,
}

delegate_iterator!((ValuesMut<'a>) => &'a mut Value);

#[cfg(feature = "serde")]
pub mod serde_impls {
    use serde::{de, ser};
    use std::fmt;

    use crate::Dictionary;

    impl ser::Serialize for Dictionary {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self {
                map.serialize_key(k)?;
                map.serialize_value(v)?;
            }
            map.end()
        }
    }

    impl<'de> de::Deserialize<'de> for Dictionary {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Dictionary;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map")
                }

                #[inline]
                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Dictionary::new())
                }

                #[inline]
                fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where
                    V: de::MapAccess<'de>,
                {
                    let mut values = Dictionary::new();

                    while let Some((key, value)) = visitor.next_entry()? {
                        values.insert(key, value);
                    }

                    Ok(values)
                }
            }

            deserializer.deserialize_map(Visitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dictionary;

    #[test]
    fn from_hash_map_to_dict() {
        let dict: Dictionary = [
            ("Doge", "Shiba Inu"),
            ("Cheems", "Shiba Inu"),
            ("Walter", "Bull Terrier"),
            ("Perro", "Golden Retriever"),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            dict.get("Doge").and_then(|v| v.as_string()),
            Some("Shiba Inu")
        );
        assert_eq!(
            dict.get("Cheems").and_then(|v| v.as_string()),
            Some("Shiba Inu")
        );
        assert_eq!(
            dict.get("Walter").and_then(|v| v.as_string()),
            Some("Bull Terrier")
        );
        assert_eq!(
            dict.get("Perro").and_then(|v| v.as_string()),
            Some("Golden Retriever")
        );
    }
}
