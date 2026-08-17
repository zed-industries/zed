use std::collections::hash_map;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::default::Default;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::hash::Hasher;
use std::slice;

use crate::reflect::ReflectValueRef;
use crate::rt;
use crate::wire_format::WireType;
use crate::zigzag::encode_zig_zag_32;
use crate::zigzag::encode_zig_zag_64;
use crate::CodedOutputStream;

/// Unknown value.
///
/// See [`UnknownFields`](crate::UnknownFields) for the explanations.
#[derive(Debug)]
pub enum UnknownValue {
    /// 32-bit unknown (e. g. `fixed32` or `float`)
    Fixed32(u32),
    /// 64-bit unknown (e. g. `fixed64` or `double`)
    Fixed64(u64),
    /// Varint unknown (e. g. `int32` or `bool`)
    Varint(u64),
    /// Length-delimited unknown (e. g. `message` or `string`)
    LengthDelimited(Vec<u8>),
}

impl UnknownValue {
    /// Wire type for this unknown
    pub fn wire_type(&self) -> WireType {
        self.get_ref().wire_type()
    }

    /// As ref
    pub fn get_ref<'s>(&'s self) -> UnknownValueRef<'s> {
        match *self {
            UnknownValue::Fixed32(fixed32) => UnknownValueRef::Fixed32(fixed32),
            UnknownValue::Fixed64(fixed64) => UnknownValueRef::Fixed64(fixed64),
            UnknownValue::Varint(varint) => UnknownValueRef::Varint(varint),
            UnknownValue::LengthDelimited(ref bytes) => UnknownValueRef::LengthDelimited(&bytes),
        }
    }

    /// Construct unknown value from `int64` value.
    pub fn int32(i: i32) -> UnknownValue {
        UnknownValue::int64(i as i64)
    }

    /// Construct unknown value from `int64` value.
    pub fn int64(i: i64) -> UnknownValue {
        UnknownValue::Varint(i as u64)
    }

    /// Construct unknown value from `sint32` value.
    pub fn sint32(i: i32) -> UnknownValue {
        UnknownValue::Varint(encode_zig_zag_32(i) as u64)
    }

    /// Construct unknown value from `sint64` value.
    pub fn sint64(i: i64) -> UnknownValue {
        UnknownValue::Varint(encode_zig_zag_64(i))
    }

    /// Construct unknown value from `float` value.
    pub fn float(f: f32) -> UnknownValue {
        UnknownValue::Fixed32(f.to_bits())
    }

    /// Construct unknown value from `double` value.
    pub fn double(f: f64) -> UnknownValue {
        UnknownValue::Fixed64(f.to_bits())
    }

    /// Construct unknown value from `sfixed32` value.
    pub fn sfixed32(i: i32) -> UnknownValue {
        UnknownValue::Fixed32(i as u32)
    }

    /// Construct unknown value from `sfixed64` value.
    pub fn sfixed64(i: i64) -> UnknownValue {
        UnknownValue::Fixed64(i as u64)
    }
}

/// Reference to unknown value.
///
/// See [`UnknownFields`](crate::UnknownFields) for explanations.
#[derive(Debug, PartialEq)]
pub enum UnknownValueRef<'o> {
    /// 32-bit unknown
    Fixed32(u32),
    /// 64-bit unknown
    Fixed64(u64),
    /// Varint unknown
    Varint(u64),
    /// Length-delimited unknown
    LengthDelimited(&'o [u8]),
}

impl<'o> UnknownValueRef<'o> {
    /// Wire-type to serialize this unknown
    pub fn wire_type(&self) -> WireType {
        match *self {
            UnknownValueRef::Fixed32(_) => WireType::Fixed32,
            UnknownValueRef::Fixed64(_) => WireType::Fixed64,
            UnknownValueRef::Varint(_) => WireType::Varint,
            UnknownValueRef::LengthDelimited(_) => WireType::LengthDelimited,
        }
    }

    pub(crate) fn to_reflect_value_ref(&'o self) -> ReflectValueRef<'o> {
        match self {
            UnknownValueRef::Fixed32(v) => ReflectValueRef::U32(*v),
            UnknownValueRef::Fixed64(v) => ReflectValueRef::U64(*v),
            UnknownValueRef::Varint(v) => ReflectValueRef::U64(*v),
            UnknownValueRef::LengthDelimited(v) => ReflectValueRef::Bytes(v),
        }
    }
}

/// Field unknown values.
///
/// See [`UnknownFields`](crate::UnknownFields) for explanations.
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub(crate) struct UnknownValues {
    /// 32-bit unknowns
    pub(crate) fixed32: Vec<u32>,
    /// 64-bit unknowns
    pub(crate) fixed64: Vec<u64>,
    /// Varint unknowns
    pub(crate) varint: Vec<u64>,
    /// Length-delimited unknowns
    pub(crate) length_delimited: Vec<Vec<u8>>,
}

impl UnknownValues {
    /// Add unknown value
    pub fn add_value(&mut self, value: UnknownValue) {
        match value {
            UnknownValue::Fixed64(fixed64) => self.fixed64.push(fixed64),
            UnknownValue::Fixed32(fixed32) => self.fixed32.push(fixed32),
            UnknownValue::Varint(varint) => self.varint.push(varint),
            UnknownValue::LengthDelimited(length_delimited) => {
                self.length_delimited.push(length_delimited)
            }
        };
    }

    /// Iterate over unknown values
    pub fn iter<'s>(&'s self) -> UnknownValuesIter<'s> {
        UnknownValuesIter {
            fixed32: self.fixed32.iter(),
            fixed64: self.fixed64.iter(),
            varint: self.varint.iter(),
            length_delimited: self.length_delimited.iter(),
        }
    }

    pub(crate) fn any(&self) -> Option<UnknownValueRef> {
        if let Some(last) = self.fixed32.last() {
            Some(UnknownValueRef::Fixed32(*last))
        } else if let Some(last) = self.fixed64.last() {
            Some(UnknownValueRef::Fixed64(*last))
        } else if let Some(last) = self.varint.last() {
            Some(UnknownValueRef::Varint(*last))
        } else if let Some(last) = self.length_delimited.last() {
            Some(UnknownValueRef::LengthDelimited(last))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a UnknownValues {
    type Item = UnknownValueRef<'a>;
    type IntoIter = UnknownValuesIter<'a>;

    fn into_iter(self) -> UnknownValuesIter<'a> {
        self.iter()
    }
}

/// Iterator over unknown values
pub(crate) struct UnknownValuesIter<'o> {
    fixed32: slice::Iter<'o, u32>,
    fixed64: slice::Iter<'o, u64>,
    varint: slice::Iter<'o, u64>,
    length_delimited: slice::Iter<'o, Vec<u8>>,
}

impl<'o> Iterator for UnknownValuesIter<'o> {
    type Item = UnknownValueRef<'o>;

    fn next(&mut self) -> Option<UnknownValueRef<'o>> {
        if let Some(fixed32) = self.fixed32.next() {
            return Some(UnknownValueRef::Fixed32(*fixed32));
        }
        if let Some(fixed64) = self.fixed64.next() {
            return Some(UnknownValueRef::Fixed64(*fixed64));
        }
        if let Some(varint) = self.varint.next() {
            return Some(UnknownValueRef::Varint(*varint));
        }
        if let Some(length_delimited) = self.length_delimited.next() {
            return Some(UnknownValueRef::LengthDelimited(&length_delimited));
        }
        None
    }
}

/// Hold "unknown" fields in parsed message.
///
/// Field may be unknown if it they are added in newer version of `.proto`.
/// Unknown fields are stored in `UnknownFields` structure, so
/// protobuf message could process messages without losing data.
///
/// For example, in this operation: load from DB, modify, store to DB,
/// even when working with older `.proto` file, new fields won't be lost.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnknownFields {
    /// The map.
    //
    // `Option` is needed, because HashMap constructor performs allocation,
    // and very expensive.
    //
    // We use "default hasher" to make iteration order deterministic.
    // Which is used to make codegen output deterministic in presence of unknown fields
    // (e. g. file options are represented as unknown fields).
    // Using default hasher is suboptimal, because it makes unknown fields less safe.
    // Note, Google Protobuf C++ simply uses linear map (which can exploitable the same way),
    // and Google Protobuf Java uses tree map to store unknown fields
    // (which is more expensive than hashmap).
    fields: Option<Box<HashMap<u32, UnknownValues, BuildHasherDefault<DefaultHasher>>>>,
}

/// Very simple hash implementation of `Hash` for `UnknownFields`.
/// Since map is unordered, we cannot put entry hashes into hasher,
/// instead we summing hashes of entries.
impl Hash for UnknownFields {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ref map) = self.fields {
            if !map.is_empty() {
                let mut hash: u64 = 0;
                for (k, v) in &**map {
                    let mut entry_hasher = DefaultHasher::new();
                    Hash::hash(&(k, v), &mut entry_hasher);
                    hash = hash.wrapping_add(entry_hasher.finish());
                }
                Hash::hash(&map.len(), state);
                Hash::hash(&hash, state);
            }
        }
    }
}

impl UnknownFields {
    /// Empty unknown fields.
    pub const fn new() -> UnknownFields {
        UnknownFields { fields: None }
    }

    /// Clear all unknown fields.
    pub fn clear(&mut self) {
        if let Some(ref mut fields) = self.fields {
            fields.clear();
        }
    }

    fn init_map(&mut self) {
        if self.fields.is_none() {
            self.fields = Some(Default::default());
        }
    }

    fn find_field<'a>(&'a mut self, number: &'a u32) -> &'a mut UnknownValues {
        self.init_map();

        match self.fields.as_mut().unwrap().entry(*number) {
            hash_map::Entry::Occupied(e) => e.into_mut(),
            hash_map::Entry::Vacant(e) => e.insert(Default::default()),
        }
    }

    /// Add unknown fixed 32-bit
    pub fn add_fixed32(&mut self, number: u32, fixed32: u32) {
        self.find_field(&number).fixed32.push(fixed32);
    }

    /// Add unknown fixed 64-bit
    pub fn add_fixed64(&mut self, number: u32, fixed64: u64) {
        self.find_field(&number).fixed64.push(fixed64);
    }

    /// Add unknown varint
    pub fn add_varint(&mut self, number: u32, varint: u64) {
        self.find_field(&number).varint.push(varint);
    }

    /// Add unknown length delimited
    pub fn add_length_delimited(&mut self, number: u32, length_delimited: Vec<u8>) {
        self.find_field(&number)
            .length_delimited
            .push(length_delimited);
    }

    /// Add unknown value
    pub fn add_value(&mut self, number: u32, value: UnknownValue) {
        self.find_field(&number).add_value(value);
    }

    /// Remove unknown field by number
    pub fn remove(&mut self, field_number: u32) {
        if let Some(fields) = &mut self.fields {
            fields.remove(&field_number);
        }
    }

    /// Iterate over all unknowns
    pub fn iter<'s>(&'s self) -> UnknownFieldsIter<'s> {
        UnknownFieldsIter {
            entries: self.fields.as_ref().map(|m| UnknownFieldsNotEmptyIter {
                fields: m.iter(),
                current: None,
            }),
        }
    }

    /// Get any value for unknown fields.
    pub fn get(&self, field_number: u32) -> Option<UnknownValueRef> {
        match &self.fields {
            Some(map) => map.get(&field_number).and_then(|v| v.any()),
            None => None,
        }
    }

    #[doc(hidden)]
    pub fn write_to_bytes(&self) -> Vec<u8> {
        let mut r = Vec::with_capacity(rt::unknown_fields_size(self) as usize);
        let mut stream = CodedOutputStream::vec(&mut r);
        // Do we need it stable everywhere?
        stream.write_unknown_fields_sorted(self).unwrap();
        stream.flush().unwrap();
        drop(stream);
        r
    }
}

impl<'a> IntoIterator for &'a UnknownFields {
    type Item = (u32, UnknownValueRef<'a>);
    type IntoIter = UnknownFieldsIter<'a>;

    fn into_iter(self) -> UnknownFieldsIter<'a> {
        self.iter()
    }
}

struct UnknownFieldsNotEmptyIter<'s> {
    fields: hash_map::Iter<'s, u32, UnknownValues>,
    current: Option<(u32, UnknownValuesIter<'s>)>,
}

/// Iterator over [`UnknownFields`](crate::UnknownFields)
pub struct UnknownFieldsIter<'s> {
    entries: Option<UnknownFieldsNotEmptyIter<'s>>,
}

impl<'s> Iterator for UnknownFieldsNotEmptyIter<'s> {
    type Item = (u32, UnknownValueRef<'s>);

    fn next(&mut self) -> Option<(u32, UnknownValueRef<'s>)> {
        loop {
            if let Some((field_number, values)) = &mut self.current {
                if let Some(value) = values.next() {
                    return Some((*field_number, value));
                }
            }
            let (field_number, values) = self.fields.next()?;
            self.current = Some((*field_number, values.iter()));
        }
    }
}

impl<'s> Iterator for UnknownFieldsIter<'s> {
    type Item = (u32, UnknownValueRef<'s>);

    fn next(&mut self) -> Option<(u32, UnknownValueRef<'s>)> {
        self.entries.as_mut().and_then(|entries| entries.next())
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    use super::UnknownFields;

    #[test]
    fn unknown_fields_hash() {
        let mut unknown_fields_1 = UnknownFields::new();
        let mut unknown_fields_2 = UnknownFields::new();

        // Check field order is not important

        unknown_fields_1.add_fixed32(10, 222);
        unknown_fields_1.add_fixed32(10, 223);
        unknown_fields_1.add_fixed64(14, 224);

        unknown_fields_2.add_fixed32(10, 222);
        unknown_fields_2.add_fixed64(14, 224);
        unknown_fields_2.add_fixed32(10, 223);

        fn hash(unknown_fields: &UnknownFields) -> u64 {
            let mut hasher = DefaultHasher::new();
            Hash::hash(unknown_fields, &mut hasher);
            hasher.finish()
        }

        assert_eq!(hash(&unknown_fields_1), hash(&unknown_fields_2));
    }

    #[test]
    fn unknown_fields_iteration_order_deterministic() {
        let mut u_1 = UnknownFields::new();
        let mut u_2 = UnknownFields::new();
        for u in &mut [&mut u_1, &mut u_2] {
            u.add_fixed32(10, 20);
            u.add_varint(30, 40);
            u.add_fixed64(50, 60);
            u.add_length_delimited(70, Vec::new());
            u.add_varint(80, 90);
            u.add_fixed32(11, 22);
            u.add_fixed64(33, 44);
        }

        let items_1: Vec<_> = u_1.iter().collect();
        let items_2: Vec<_> = u_2.iter().collect();
        assert_eq!(items_1, items_2);
    }
}
