// SPDX-License-Identifier: Apache-2.0

use crate::value::Value;
use alloc::vec::Vec;
use core::cmp::Ordering;
use serde::{de, ser};

/// Manually serialize values to compare them.
fn serialized_canonical_cmp(v1: &Value, v2: &Value) -> Ordering {
    // There is an optimization to be done here, but it would take a lot more code
    // and using mixing keys, Arrays or Maps as CanonicalValue is probably not the
    // best use of this type as it is meant mainly to be used as keys.

    let mut bytes1 = Vec::new();
    let _ = crate::ser::into_writer(v1, &mut bytes1);
    let mut bytes2 = Vec::new();
    let _ = crate::ser::into_writer(v2, &mut bytes2);

    match bytes1.len().cmp(&bytes2.len()) {
        Ordering::Equal => bytes1.cmp(&bytes2),
        x => x,
    }
}

/// Compares two values uses canonical comparison, as defined in both
/// RFC 7049 Section 3.9 (regarding key sorting) and RFC 8949 4.2.3 (as errata).
///
/// In short, the comparison follow the following rules:
///   - If two keys have different lengths, the shorter one sorts earlier;
///   - If two keys have the same length, the one with the lower value in
///     (byte-wise) lexical order sorts earlier.
///
/// This specific comparison allows Maps and sorting that respect these two rules.
pub fn cmp_value(v1: &Value, v2: &Value) -> Ordering {
    use Value::*;

    match (v1, v2) {
        (Integer(i), Integer(o)) => {
            // Because of the first rule above, two numbers might be in a different
            // order than regular i128 comparison. For example, 10 < -1 in
            // canonical ordering, since 10 serializes to `0x0a` and -1 to `0x20`,
            // and -1 < -1000 because of their lengths.
            i.canonical_cmp(o)
        }
        (Text(s), Text(o)) => match s.len().cmp(&o.len()) {
            Ordering::Equal => s.cmp(o),
            x => x,
        },
        (Bool(s), Bool(o)) => s.cmp(o),
        (Null, Null) => Ordering::Equal,
        (Tag(t, v), Tag(ot, ov)) => match Value::from(*t).partial_cmp(&Value::from(*ot)) {
            Some(Ordering::Equal) | None => match v.partial_cmp(ov) {
                Some(x) => x,
                None => serialized_canonical_cmp(v1, v2),
            },
            Some(x) => x,
        },
        (_, _) => serialized_canonical_cmp(v1, v2),
    }
}

/// A CBOR Value that impl Ord and Eq to allow sorting of values as defined in both
/// RFC 7049 Section 3.9 (regarding key sorting) and RFC 8949 4.2.3 (as errata).
///
/// Since a regular [Value] can be
#[derive(Clone, Debug)]
pub struct CanonicalValue(Value);

impl PartialEq for CanonicalValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CanonicalValue {}

impl From<Value> for CanonicalValue {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

impl From<CanonicalValue> for Value {
    fn from(v: CanonicalValue) -> Self {
        v.0
    }
}

impl ser::Serialize for CanonicalValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for CanonicalValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Value::deserialize(deserializer).map(Into::into)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Value::deserialize_in_place(deserializer, &mut place.0)
    }
}

impl Ord for CanonicalValue {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_value(&self.0, &other.0)
    }
}

impl PartialOrd for CanonicalValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp_value(&self.0, &other.0))
    }
}
