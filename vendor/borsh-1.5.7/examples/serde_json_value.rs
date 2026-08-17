use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};

mod serde_json_value {
    pub use de::deserialize_value;
    pub use ser::serialize_value;
    mod ser {
        use borsh::{
            io::{ErrorKind, Result, Write},
            BorshSerialize,
        };
        use core::convert::TryFrom;

        /// this is mutually recursive with `serialize_array` and `serialize_map`
        pub fn serialize_value<W: Write>(value: &serde_json::Value, writer: &mut W) -> Result<()> {
            match value {
                serde_json::Value::Null => 0_u8.serialize(writer),
                serde_json::Value::Bool(b) => {
                    1_u8.serialize(writer)?;
                    b.serialize(writer)
                }
                serde_json::Value::Number(n) => {
                    2_u8.serialize(writer)?;
                    serialize_number(n, writer)
                }
                serde_json::Value::String(s) => {
                    3_u8.serialize(writer)?;
                    s.serialize(writer)
                }
                serde_json::Value::Array(a) => {
                    4_u8.serialize(writer)?;
                    serialize_array(a, writer)
                }
                serde_json::Value::Object(o) => {
                    5_u8.serialize(writer)?;
                    serialize_map(o, writer)
                }
            }
        }

        fn serialize_number<W: Write>(number: &serde_json::Number, writer: &mut W) -> Result<()> {
            // A JSON number can either be a non-negative integer (represented in
            // serde_json by a u64), a negative integer (by an i64), or a non-integer
            // (by an f64).
            // We identify these cases with the following single-byte discriminants:
            // 0 - u64
            // 1 - i64
            // 2 - f64
            if let Some(u) = number.as_u64() {
                0_u8.serialize(writer)?;
                return u.serialize(writer);
            }

            if let Some(i) = number.as_i64() {
                1_u8.serialize(writer)?;
                return i.serialize(writer);
            }

            if let Some(f) = number.as_f64() {
                2_u8.serialize(writer)?;
                return f.serialize(writer);
            }

            // technically, it should not be unreachable, but an error instead,
            // as assumption about unreachable depends on private implementation detail
            // but it's fine to leave it be unreachable! for an example
            unreachable!("number is neither a u64, i64, nor f64");
        }

        /// this is mutually recursive with `serialize_value`
        fn serialize_array<W: Write>(array: &Vec<serde_json::Value>, writer: &mut W) -> Result<()> {
            // The implementation here is very similar to that of Vec<V>.
            writer.write_all(
                &(u32::try_from(array.len()).map_err(|_| ErrorKind::InvalidData)?).to_le_bytes(),
            )?;
            for item in array {
                serialize_value(&item, writer)?;
            }
            Ok(())
        }

        /// this is mutually recursive with `serialize_value`
        fn serialize_map<W: Write>(
            map: &serde_json::Map<String, serde_json::Value>,
            writer: &mut W,
        ) -> Result<()> {
            // The implementation here is very similar to that of BTreeMap<K, V>.
            u32::try_from(map.len())
                .map_err(|_| ErrorKind::InvalidData)?
                .serialize(writer)?;

            for (key, value) in map {
                key.serialize(writer)?;
                serialize_value(&value, writer)?;
            }

            Ok(())
        }
    }
    mod de {
        use borsh::{
            io::{Error, ErrorKind, Read, Result},
            BorshDeserialize,
        };

        /// this is copy-paste of https://github.com/near/borsh-rs/blob/master/borsh/src/de/hint.rs#L2-L5
        fn hint_cautious<T>(hint: u32) -> usize {
            let el_size = core::mem::size_of::<T>() as u32;
            core::cmp::max(core::cmp::min(hint, 4096 / el_size), 1) as usize
        }

        /// this is mutually recursive with `deserialize_array`, `deserialize_map`
        pub fn deserialize_value<R: Read>(reader: &mut R) -> Result<serde_json::Value> {
            let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
            match flag {
                0 => Ok(serde_json::Value::Null),
                1 => {
                    let b: bool = BorshDeserialize::deserialize_reader(reader)?;
                    Ok(serde_json::Value::Bool(b))
                }
                2 => {
                    let n: serde_json::Number = deserialize_number(reader)?;
                    Ok(serde_json::Value::Number(n))
                }
                3 => {
                    let s: String = BorshDeserialize::deserialize_reader(reader)?;
                    Ok(serde_json::Value::String(s))
                }
                4 => {
                    let a: Vec<serde_json::Value> = deserialize_array(reader)?;
                    Ok(serde_json::Value::Array(a))
                }
                5 => {
                    let o: serde_json::Map<_, _> = deserialize_map(reader)?;
                    Ok(serde_json::Value::Object(o))
                }
                _ => {
                    let msg = format!(
                        "Invalid JSON value representation: {}. The first byte must be 0-5",
                        flag
                    );

                    Err(Error::new(ErrorKind::InvalidData, msg))
                }
            }
        }

        fn deserialize_number<R: Read>(reader: &mut R) -> Result<serde_json::Number> {
            let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
            match flag {
                0 => {
                    let u: u64 = BorshDeserialize::deserialize_reader(reader)?;
                    Ok(u.into())
                }
                1 => {
                    let i: i64 = BorshDeserialize::deserialize_reader(reader)?;
                    Ok(i.into())
                }
                2 => {
                    let f: f64 = BorshDeserialize::deserialize_reader(reader)?;
                    // This returns None if the number is a NaN or +/-Infinity,
                    // which are not valid JSON numbers.
                    serde_json::Number::from_f64(f).ok_or_else(|| {
                        let msg = format!("Invalid JSON number: {}", f);

                        Error::new(ErrorKind::InvalidData, msg)
                    })
                }
                _ => {
                    let msg = format!(
                        "Invalid JSON number representation: {}. The first byte must be 0-2",
                        flag
                    );

                    Err(Error::new(ErrorKind::InvalidData, msg))
                }
            }
        }

        /// this is mutually recursive with `deserialize_value`  
        fn deserialize_array<R: Read>(reader: &mut R) -> Result<Vec<serde_json::Value>> {
            // The implementation here is very similar to that of Vec<V>.
            let len = u32::deserialize_reader(reader)?;
            let mut result = Vec::with_capacity(hint_cautious::<(String, serde_json::Value)>(len));
            for _ in 0..len {
                let value = deserialize_value(reader)?;
                result.push(value);
            }
            Ok(result)
        }

        /// this is mutually recursive with `deserialize_value`  
        fn deserialize_map<R: Read>(
            reader: &mut R,
        ) -> Result<serde_json::Map<String, serde_json::Value>> {
            // The implementation here is very similar to that of BTreeMap<K, V>.

            let vec: Vec<(String, serde_json::Value)> = {
                // The implementation here is very similar to that of Vec<(K, V)>.
                let len = u32::deserialize_reader(reader)?;
                let mut result =
                    Vec::with_capacity(hint_cautious::<(String, serde_json::Value)>(len));
                for _ in 0..len {
                    let pair = {
                        let key = String::deserialize_reader(reader)?;
                        let value = deserialize_value(reader)?;
                        (key, value)
                    };
                    result.push(pair);
                }
                result
            };

            Ok(vec.into_iter().collect())
        }
    }
}

mod borsh_wrapper {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
    pub struct SerdeJsonBorshWrapper(
        #[borsh(
            serialize_with = "super::serde_json_value::serialize_value",
            deserialize_with = "super::serde_json_value::deserialize_value"
        )]
        pub serde_json::Value,
    );

    impl From<serde_json::Value> for SerdeJsonBorshWrapper {
        fn from(value: serde_json::Value) -> Self {
            Self(value)
        }
    }

    impl From<SerdeJsonBorshWrapper> for serde_json::Value {
        fn from(value: SerdeJsonBorshWrapper) -> Self {
            value.0
        }
    }
}

use borsh_wrapper::SerdeJsonBorshWrapper;

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
struct SerdeJsonAsField {
    pub examples: HashMap<String, SerdeJsonBorshWrapper>,
}

fn main() {
    // original code is from https://github.com/near/borsh-rs/pull/312
    let original = serde_json::json!({
        "null": null,
        "true": true,
        "false": false,
        "zero": 0,
        "positive_integer": 12345,
        "negative_integer": -88888,
        "positive_float": 123.45,
        "negative_float": -888.88,
        "positive_max": 1.7976931348623157e+308,
        "negative_max": -1.7976931348623157e+308,
        "string": "Larry",
        "array_of_nulls": [null, null, null],
        "array_of_numbers": [0, -1, 1, 1.1, -1.1, 34798324],
        "array_of_strings": ["Larry", "Jake", "Pumpkin"],
        "array_of_arrays": [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ],
        "array_of_objects": [
            {
                "name": "Larry",
                "age": 30
            },
            {
                "name": "Jake",
                "age": 7
            },
            {
                "name": "Pumpkin",
                "age": 8
            }
        ],
        "object": {
            "name": "Larry",
            "age": 30,
            "pets": [
                {
                    "name": "Jake",
                    "age": 7
                },
                {
                    "name": "Pumpkin",
                    "age": 8
                }
            ]
        }
    });

    let mut examples = HashMap::new();
    examples.insert("Larry Jake Pumpkin".into(), original.clone().into());

    let complex_struct = SerdeJsonAsField { examples };
    let serialized = borsh::to_vec(&complex_struct).unwrap();

    let mut deserialized: SerdeJsonAsField = borsh::from_slice(&serialized).unwrap();

    assert_eq!(complex_struct, deserialized);

    let deserialized_value: serde_json::Value = deserialized
        .examples
        .remove("Larry Jake Pumpkin")
        .expect("key present")
        .into();

    assert_eq!(original, deserialized_value);

    let number = deserialized_value
        .get("array_of_numbers")
        .expect("has key")
        .as_array()
        .expect("is array")
        .get(5)
        .expect("has index")
        .as_i64()
        .expect("is i64");

    assert_eq!(number, 34798324);
}
