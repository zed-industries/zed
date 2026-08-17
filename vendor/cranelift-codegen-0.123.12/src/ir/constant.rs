//! Constants
//!
//! The constant pool defined here allows Cranelift to avoid emitting the same constant multiple
//! times. As constants are inserted in the pool, a handle is returned; the handle is a Cranelift
//! Entity. Inserting the same data multiple times will always return the same handle.
//!
//! Future work could include:
//! - ensuring alignment of constants within the pool,
//! - bucketing constants by size.

use crate::ir::Constant;
use crate::ir::immediates::{Ieee128, IntoBytes, V128Imm};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;
use core::slice::Iter;
use core::str::{FromStr, from_utf8};
use cranelift_entity::EntityRef;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// This type describes the actual constant data. Note that the bytes stored in this structure are
/// expected to be in little-endian order; this is due to ease-of-use when interacting with
/// WebAssembly values, which are [little-endian by design].
///
/// [little-endian by design]: https://github.com/WebAssembly/design/blob/master/Portability.md
#[derive(Clone, Hash, Eq, PartialEq, Debug, Default, PartialOrd, Ord)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ConstantData(Vec<u8>);

impl FromIterator<u8> for ConstantData {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let v = iter.into_iter().collect();
        Self(v)
    }
}

impl From<Vec<u8>> for ConstantData {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for ConstantData {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

impl From<V128Imm> for ConstantData {
    fn from(v: V128Imm) -> Self {
        Self(v.to_vec())
    }
}

impl From<Ieee128> for ConstantData {
    fn from(v: Ieee128) -> Self {
        Self(v.into_bytes())
    }
}

impl TryFrom<&ConstantData> for Ieee128 {
    type Error = <[u8; 16] as TryFrom<&'static [u8]>>::Error;

    fn try_from(value: &ConstantData) -> Result<Self, Self::Error> {
        Ok(Ieee128::with_bits(u128::from_le_bytes(
            value.as_slice().try_into()?,
        )))
    }
}

impl ConstantData {
    /// Return the number of bytes in the constant.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the constant contains any bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Convert the data to a vector.
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    /// Iterate over the constant's bytes.
    pub fn iter(&self) -> Iter<'_, u8> {
        self.0.iter()
    }

    /// Add new bytes to the constant data.
    pub fn append(mut self, bytes: impl IntoBytes) -> Self {
        let mut to_add = bytes.into_bytes();
        self.0.append(&mut to_add);
        self
    }

    /// Expand the size of the constant data to `expected_size` number of bytes by adding zeroes
    /// in the high-order byte slots.
    pub fn expand_to(mut self, expected_size: usize) -> Self {
        if self.len() > expected_size {
            panic!("The constant data is already expanded beyond {expected_size} bytes")
        }
        self.0.resize(expected_size, 0);
        self
    }
}

impl fmt::Display for ConstantData {
    /// Print the constant data in hexadecimal format, e.g. 0x000102030405060708090a0b0c0d0e0f.
    /// This function will flip the stored order of bytes--little-endian--to the more readable
    /// big-endian ordering.
    ///
    /// ```
    /// use cranelift_codegen::ir::ConstantData;
    /// let data = ConstantData::from([3, 2, 1, 0, 0].as_ref()); // note the little-endian order
    /// assert_eq!(data.to_string(), "0x0000010203");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_empty() {
            write!(f, "0x")?;
            for b in self.0.iter().rev() {
                write!(f, "{b:02x}")?;
            }
        }
        Ok(())
    }
}

impl FromStr for ConstantData {
    type Err = &'static str;

    /// Parse a hexadecimal string to `ConstantData`. This is the inverse of `Display::fmt`.
    ///
    /// ```
    /// use cranelift_codegen::ir::ConstantData;
    /// let c: ConstantData = "0x000102".parse().unwrap();
    /// assert_eq!(c.into_vec(), [2, 1, 0]);
    /// ```
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() <= 2 || &s[0..2] != "0x" {
            return Err("Expected a hexadecimal string, e.g. 0x1234");
        }

        // clean and check the string
        let cleaned: Vec<u8> = s[2..]
            .as_bytes()
            .iter()
            .filter(|&&b| b as char != '_')
            .cloned()
            .collect(); // remove 0x prefix and any intervening _ characters

        if cleaned.is_empty() {
            Err("Hexadecimal string must have some digits")
        } else if cleaned.len() % 2 != 0 {
            Err("Hexadecimal string must have an even number of digits")
        } else if cleaned.len() > 32 {
            Err("Hexadecimal string has too many digits to fit in a 128-bit vector")
        } else {
            let mut buffer = Vec::with_capacity((s.len() - 2) / 2);
            for i in (0..cleaned.len()).step_by(2) {
                let pair = from_utf8(&cleaned[i..i + 2])
                    .or_else(|_| Err("Unable to parse hexadecimal pair as UTF-8"))?;
                let byte = u8::from_str_radix(pair, 16)
                    .or_else(|_| Err("Unable to parse as hexadecimal"))?;
                buffer.insert(0, byte);
            }
            Ok(Self(buffer))
        }
    }
}

/// Maintains the mapping between a constant handle (i.e.  [`Constant`]) and
/// its constant data (i.e.  [`ConstantData`]).
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ConstantPool {
    /// This mapping maintains the insertion order as long as Constants are created with
    /// sequentially increasing integers.
    ///
    /// It is important that, by construction, no entry in that list gets removed. If that ever
    /// need to happen, don't forget to update the `Constant` generation scheme.
    handles_to_values: BTreeMap<Constant, ConstantData>,

    /// Mapping of hashed `ConstantData` to the index into the other hashmap.
    ///
    /// This allows for deduplication of entries into the `handles_to_values` mapping.
    values_to_handles: BTreeMap<ConstantData, Constant>,
}

impl ConstantPool {
    /// Create a new constant pool instance.
    pub fn new() -> Self {
        Self {
            handles_to_values: BTreeMap::new(),
            values_to_handles: BTreeMap::new(),
        }
    }

    /// Empty the constant pool of all data.
    pub fn clear(&mut self) {
        self.handles_to_values.clear();
        self.values_to_handles.clear();
    }

    /// Insert constant data into the pool, returning a handle for later referencing; when constant
    /// data is inserted that is a duplicate of previous constant data, the existing handle will be
    /// returned.
    pub fn insert(&mut self, constant_value: ConstantData) -> Constant {
        if let Some(cst) = self.values_to_handles.get(&constant_value) {
            return *cst;
        }

        let constant_handle = Constant::new(self.len());
        self.set(constant_handle, constant_value);
        constant_handle
    }

    /// Retrieve the constant data given a handle.
    pub fn get(&self, constant_handle: Constant) -> &ConstantData {
        assert!(self.handles_to_values.contains_key(&constant_handle));
        self.handles_to_values.get(&constant_handle).unwrap()
    }

    /// Link a constant handle to its value. This does not de-duplicate data but does avoid
    /// replacing any existing constant values. use `set` to tie a specific `const42` to its value;
    /// use `insert` to add a value and return the next available `const` entity.
    pub fn set(&mut self, constant_handle: Constant, constant_value: ConstantData) {
        let replaced = self
            .handles_to_values
            .insert(constant_handle, constant_value.clone());
        assert!(
            replaced.is_none(),
            "attempted to overwrite an existing constant {:?}: {:?} => {:?}",
            constant_handle,
            &constant_value,
            replaced.unwrap()
        );
        self.values_to_handles
            .insert(constant_value, constant_handle);
    }

    /// Iterate over the constants in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&Constant, &ConstantData)> {
        self.handles_to_values.iter()
    }

    /// Iterate over mutable entries in the constant pool in insertion order.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut ConstantData> {
        self.handles_to_values.values_mut()
    }

    /// Return the number of constants in the pool.
    pub fn len(&self) -> usize {
        self.handles_to_values.len()
    }

    /// Return the combined size of all of the constant values in the pool.
    pub fn byte_size(&self) -> usize {
        self.handles_to_values.values().map(|c| c.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn empty() {
        let sut = ConstantPool::new();
        assert_eq!(sut.len(), 0);
    }

    #[test]
    fn insert() {
        let mut sut = ConstantPool::new();
        sut.insert(vec![1, 2, 3].into());
        sut.insert(vec![4, 5, 6].into());
        assert_eq!(sut.len(), 2);
    }

    #[test]
    fn insert_duplicate() {
        let mut sut = ConstantPool::new();
        let a = sut.insert(vec![1, 2, 3].into());
        sut.insert(vec![4, 5, 6].into());
        let b = sut.insert(vec![1, 2, 3].into());
        assert_eq!(a, b);
    }

    #[test]
    fn clear() {
        let mut sut = ConstantPool::new();
        sut.insert(vec![1, 2, 3].into());
        assert_eq!(sut.len(), 1);

        sut.clear();
        assert_eq!(sut.len(), 0);
    }

    #[test]
    fn iteration_order() {
        let mut sut = ConstantPool::new();
        sut.insert(vec![1, 2, 3].into());
        sut.insert(vec![4, 5, 6].into());
        sut.insert(vec![1, 2, 3].into());
        let data = sut.iter().map(|(_, v)| v).collect::<Vec<&ConstantData>>();
        assert_eq!(data, vec![&vec![1, 2, 3].into(), &vec![4, 5, 6].into()]);
    }

    #[test]
    fn get() {
        let mut sut = ConstantPool::new();
        let data = vec![1, 2, 3];
        let handle = sut.insert(data.clone().into());
        assert_eq!(sut.get(handle), &data.into());
    }

    #[test]
    fn set() {
        let mut sut = ConstantPool::new();
        let handle = Constant::with_number(42).unwrap();
        let data = vec![1, 2, 3];
        sut.set(handle, data.clone().into());
        assert_eq!(sut.get(handle), &data.into());
    }

    #[test]
    #[should_panic]
    fn disallow_overwriting_constant() {
        let mut sut = ConstantPool::new();
        let handle = Constant::with_number(42).unwrap();
        sut.set(handle, vec![].into());
        sut.set(handle, vec![1].into());
    }

    #[test]
    #[should_panic]
    fn get_nonexistent_constant() {
        let sut = ConstantPool::new();
        let a = Constant::with_number(42).unwrap();
        sut.get(a); // panics, only use constants returned by ConstantPool
    }

    #[test]
    fn display_constant_data() {
        assert_eq!(ConstantData::from([0].as_ref()).to_string(), "0x00");
        assert_eq!(ConstantData::from([42].as_ref()).to_string(), "0x2a");
        assert_eq!(
            ConstantData::from([3, 2, 1, 0].as_ref()).to_string(),
            "0x00010203"
        );
        assert_eq!(
            ConstantData::from(3735928559u32.to_le_bytes().as_ref()).to_string(),
            "0xdeadbeef"
        );
        assert_eq!(
            ConstantData::from(0x0102030405060708u64.to_le_bytes().as_ref()).to_string(),
            "0x0102030405060708"
        );
    }

    #[test]
    fn iterate_over_constant_data() {
        let c = ConstantData::from([1, 2, 3].as_ref());
        let mut iter = c.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn add_to_constant_data() {
        let d = ConstantData::from([1, 2].as_ref());
        let e = d.append(i16::from(3u8));
        assert_eq!(e.into_vec(), vec![1, 2, 3, 0])
    }

    #[test]
    fn extend_constant_data() {
        let d = ConstantData::from([1, 2].as_ref());
        assert_eq!(d.expand_to(4).into_vec(), vec![1, 2, 0, 0])
    }

    #[test]
    #[should_panic]
    fn extend_constant_data_to_invalid_length() {
        ConstantData::from([1, 2].as_ref()).expand_to(1);
    }

    #[test]
    fn parse_constant_data_and_restringify() {
        // Verify that parsing of `from` succeeds and stringifies to `to`.
        fn parse_ok(from: &str, to: &str) {
            let parsed = from.parse::<ConstantData>().unwrap();
            assert_eq!(parsed.to_string(), to);
        }

        // Verify that parsing of `from` fails with `error_msg`.
        fn parse_err(from: &str, error_msg: &str) {
            let parsed = from.parse::<ConstantData>();
            assert!(
                parsed.is_err(),
                "Expected a parse error but parsing succeeded: {from}"
            );
            assert_eq!(parsed.err().unwrap(), error_msg);
        }

        parse_ok("0x00", "0x00");
        parse_ok("0x00000042", "0x00000042");
        parse_ok(
            "0x0102030405060708090a0b0c0d0e0f00",
            "0x0102030405060708090a0b0c0d0e0f00",
        );
        parse_ok("0x_0000_0043_21", "0x0000004321");

        parse_err("", "Expected a hexadecimal string, e.g. 0x1234");
        parse_err("0x", "Expected a hexadecimal string, e.g. 0x1234");
        parse_err(
            "0x042",
            "Hexadecimal string must have an even number of digits",
        );
        parse_err(
            "0x00000000000000000000000000000000000000000000000000",
            "Hexadecimal string has too many digits to fit in a 128-bit vector",
        );
        parse_err("0xrstu", "Unable to parse as hexadecimal");
        parse_err("0x__", "Hexadecimal string must have some digits");
    }

    #[test]
    fn verify_stored_bytes_in_constant_data() {
        assert_eq!("0x01".parse::<ConstantData>().unwrap().into_vec(), [1]);
        assert_eq!(ConstantData::from([1, 0].as_ref()).0, [1, 0]);
        assert_eq!(ConstantData::from(vec![1, 0, 0, 0]).0, [1, 0, 0, 0]);
    }

    #[test]
    fn check_constant_data_endianness_as_uimm128() {
        fn parse_to_uimm128(from: &str) -> Vec<u8> {
            from.parse::<ConstantData>()
                .unwrap()
                .expand_to(16)
                .into_vec()
        }

        assert_eq!(
            parse_to_uimm128("0x42"),
            [0x42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            parse_to_uimm128("0x00"),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            parse_to_uimm128("0x12345678"),
            [0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            parse_to_uimm128("0x1234_5678"),
            [0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn constant_ieee128() {
        let value = Ieee128::with_bits(0x000102030405060708090a0b0c0d0e0f);
        let constant = ConstantData::from(value);
        assert_eq!(
            constant.as_slice(),
            &[
                0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0
            ]
        );
        assert_eq!(Ieee128::try_from(&constant).unwrap().bits(), value.bits());
    }
}
