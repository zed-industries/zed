use crate::mapping::Entry;
use crate::{mapping, private, Mapping, Value};
use std::fmt::{self, Debug};
use std::ops;

/// A type that can be used to index into a `serde_yaml::Value`. See the `get`
/// and `get_mut` methods of `Value`.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_yaml`.
pub trait Index: private::Sealed {
    /// Return None if the key is not already in the sequence or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;

    /// Return None if the key is not already in the sequence or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;

    /// Panic if sequence index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v.untag_ref() {
            Value::Sequence(vec) => vec.get(*self),
            Value::Mapping(vec) => vec.get(&Value::Number((*self).into())),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v.untag_mut() {
            Value::Sequence(vec) => vec.get_mut(*self),
            Value::Mapping(vec) => vec.get_mut(&Value::Number((*self).into())),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, mut v: &'v mut Value) -> &'v mut Value {
        loop {
            match v {
                Value::Sequence(vec) => {
                    let len = vec.len();
                    return vec.get_mut(*self).unwrap_or_else(|| {
                        panic!(
                            "cannot access index {} of YAML sequence of length {}",
                            self, len
                        )
                    });
                }
                Value::Mapping(map) => {
                    let n = Value::Number((*self).into());
                    return map.entry(n).or_insert(Value::Null);
                }
                Value::Tagged(tagged) => v = &mut tagged.value,
                _ => panic!("cannot access index {} of YAML {}", self, Type(v)),
            }
        }
    }
}

fn index_into_mapping<'v, I>(index: &I, v: &'v Value) -> Option<&'v Value>
where
    I: ?Sized + mapping::Index,
{
    match v.untag_ref() {
        Value::Mapping(map) => map.get(index),
        _ => None,
    }
}

fn index_into_mut_mapping<'v, I>(index: &I, v: &'v mut Value) -> Option<&'v mut Value>
where
    I: ?Sized + mapping::Index,
{
    match v.untag_mut() {
        Value::Mapping(map) => map.get_mut(index),
        _ => None,
    }
}

fn index_or_insert_mapping<'v, I>(index: &I, mut v: &'v mut Value) -> &'v mut Value
where
    I: ?Sized + mapping::Index + ToOwned + Debug,
    Value: From<I::Owned>,
{
    if let Value::Null = *v {
        *v = Value::Mapping(Mapping::new());
        return match v {
            Value::Mapping(map) => match map.entry(index.to_owned().into()) {
                Entry::Vacant(entry) => entry.insert(Value::Null),
                Entry::Occupied(_) => unreachable!(),
            },
            _ => unreachable!(),
        };
    }
    loop {
        match v {
            Value::Mapping(map) => {
                return map.entry(index.to_owned().into()).or_insert(Value::Null);
            }
            Value::Tagged(tagged) => v = &mut tagged.value,
            _ => panic!("cannot access key {:?} in YAML {}", index, Type(v)),
        }
    }
}

impl Index for Value {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        index_into_mapping(self, v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        index_into_mut_mapping(self, v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        index_or_insert_mapping(self, v)
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        index_into_mapping(self, v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        index_into_mut_mapping(self, v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        index_or_insert_mapping(self, v)
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self.as_str().index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self.as_str().index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self.as_str().index_or_insert(v)
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

/// Used in panic messages.
struct Type<'a>(&'a Value);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("boolean"),
            Value::Number(_) => formatter.write_str("number"),
            Value::String(_) => formatter.write_str("string"),
            Value::Sequence(_) => formatter.write_str("sequence"),
            Value::Mapping(_) => formatter.write_str("mapping"),
            Value::Tagged(_) => unreachable!(),
        }
    }
}

// The usual semantics of Index is to panic on invalid indexing.
//
// That said, the usual semantics are for things like `Vec` and `BTreeMap` which
// have different use cases than Value. If you are working with a Vec, you know
// that you are working with a Vec and you can get the len of the Vec and make
// sure your indices are within bounds. The Value use cases are more
// loosey-goosey. You got some YAML from an endpoint and you want to pull values
// out of it. Outside of this Index impl, you already have the option of using
// `value.as_sequence()` and working with the Vec directly, or matching on
// `Value::Sequence` and getting the Vec directly. The Index impl means you can
// skip that and index directly into the thing using a concise syntax. You don't
// have to check the type, you don't have to check the len, it is all about what
// you expect the Value to look like.
//
// Basically the use cases that would be well served by panicking here are
// better served by using one of the other approaches: `get` and `get_mut`,
// `as_sequence`, or match. The value of this impl is that it adds a way of
// working with Value that is not well served by the existing approaches:
// concise and careless and sometimes that is exactly what you want.
impl<I> ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;

    /// Index into a `serde_yaml::Value` using the syntax `value[0]` or
    /// `value["k"]`.
    ///
    /// Returns `Value::Null` if the type of `self` does not match the type of
    /// the index, for example if the index is a string and `self` is a sequence
    /// or a number. Also returns `Value::Null` if the given key does not exist
    /// in the map or the given index is not within the bounds of the sequence.
    ///
    /// For retrieving deeply nested values, you should have a look at the
    /// `Value::pointer` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_yaml::Value;
    /// #
    /// # fn main() -> serde_yaml::Result<()> {
    /// let data: serde_yaml::Value = serde_yaml::from_str(r#"{ x: { y: [z, zz] } }"#)?;
    ///
    /// assert_eq!(data["x"]["y"], serde_yaml::from_str::<Value>(r#"["z", "zz"]"#).unwrap());
    /// assert_eq!(data["x"]["y"][0], serde_yaml::from_str::<Value>(r#""z""#).unwrap());
    ///
    /// assert_eq!(data["a"], serde_yaml::from_str::<Value>(r#"null"#).unwrap()); // returns null for undefined values
    /// assert_eq!(data["a"]["b"], serde_yaml::from_str::<Value>(r#"null"#).unwrap()); // does not panic
    /// # Ok(())
    /// # }
    /// ```
    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value
where
    I: Index,
{
    /// Write into a `serde_yaml::Value` using the syntax `value[0] = ...` or
    /// `value["k"] = ...`.
    ///
    /// If the index is a number, the value must be a sequence of length bigger
    /// than the index. Indexing into a value that is not a sequence or a
    /// sequence that is too small will panic.
    ///
    /// If the index is a string, the value must be an object or null which is
    /// treated like an empty object. If the key is not already present in the
    /// object, it will be inserted with a value of null. Indexing into a value
    /// that is neither an object nor null will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let mut data: serde_yaml::Value = serde_yaml::from_str(r#"{x: 0}"#)?;
    ///
    /// // replace an existing key
    /// data["x"] = serde_yaml::from_str(r#"1"#)?;
    ///
    /// // insert a new key
    /// data["y"] = serde_yaml::from_str(r#"[false, false, false]"#)?;
    ///
    /// // replace a value in a sequence
    /// data["y"][0] = serde_yaml::from_str(r#"true"#)?;
    ///
    /// // inserted a deeply nested key
    /// data["a"]["b"]["c"]["d"] = serde_yaml::from_str(r#"true"#)?;
    ///
    /// println!("{:?}", data);
    /// # Ok(())
    /// # }
    /// ```
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}
