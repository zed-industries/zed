// SPDX-License-Identifier: Apache-2.0

extern crate std;

use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::fmt::Debug;

use ciborium::value::Value;
use ciborium::{cbor, de::from_reader, de::from_reader_with_buffer, ser::into_writer};

use rstest::rstest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

macro_rules! val {
    ($x:expr) => {
        Value::try_from($x).unwrap()
    };
}

macro_rules! hex {
    ($x:expr) => {
        serde_bytes::ByteBuf::from(hex::decode($x).unwrap())
    };
}

macro_rules! map {
    ($($k:expr => $v:expr),*) => {{
        let mut map = BTreeMap::new();
        $(
            map.insert($k, $v);
        )*
        map
    }}
}

// Keep the first "case" aligned to a line number ending in 1 for ease in finding tests.
#[allow(clippy::excessive_precision)]
#[rstest(input, value, bytes, alternate, equality,

    case(0u8,   val!(0u8),   "00", false, same),
    case(0u16,  val!(0u16),  "00", false, same),
    case(0u32,  val!(0u32),  "00", false, same),
    case(0u64,  val!(0u64),  "00", false, same),
    case(0u128, val!(0u128), "00", false, same),
    case(0i8,   val!(0i8),   "00", false, same),
    case(0i16,  val!(0i16),  "00", false, same),
    case(0i32,  val!(0i32),  "00", false, same),
    case(0i64,  val!(0i64),  "00", false, same),
    case(0i128, val!(0i128), "00", false, same),
    case(1u8,   val!(1u8),   "01", false, same),
    case(1u16,  val!(1u16),  "01", false, same),
    case(1u32,  val!(1u32),  "01", false, same),
    case(1u64,  val!(1u64),  "01", false, same),
    case(1u128, val!(1u128), "01", false, same),
    case(1i8,   val!(1i8),   "01", false, same),
    case(1i16,  val!(1i16),  "01", false, same),
    case(1i32,  val!(1i32),  "01", false, same),
    case(1i64,  val!(1i64),  "01", false, same),
    case(1i128, val!(1i128), "01", false, same),
    case(1u8,   val!(1u8),   "1b0000000000000001", true, same),
    case(1u16,  val!(1u16),  "1b0000000000000001", true, same),
    case(1u32,  val!(1u32),  "1b0000000000000001", true, same),
    case(1u64,  val!(1u64),  "1b0000000000000001", true, same),
    case(1u128, val!(1u128), "1b0000000000000001", true, same),
    case(1i8,   val!(1i8),   "1b0000000000000001", true, same),
    case(1i16,  val!(1i16),  "1b0000000000000001", true, same),
    case(1i32,  val!(1i32),  "1b0000000000000001", true, same),
    case(1i64,  val!(1i64),  "1b0000000000000001", true, same),
    case(1i128, val!(1i128), "1b0000000000000001", true, same),
    case(1u8,   bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1u16,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1u32,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1u64,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1u128, bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1i8,   bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1i16,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1i32,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1i64,  bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(1i128, bigint(), "c2540000000000000000000000000000000000000001", true, same), // Not In RFC
    case(10u8,   val!(10u8),   "0a", false, same),
    case(10u16,  val!(10u16),  "0a", false, same),
    case(10u32,  val!(10u32),  "0a", false, same),
    case(10u64,  val!(10u64),  "0a", false, same),
    case(10u128, val!(10u128), "0a", false, same),
    case(10i8,   val!(10i8),   "0a", false, same),
    case(10i16,  val!(10i16),  "0a", false, same),
    case(10i32,  val!(10i32),  "0a", false, same),
    case(10i64,  val!(10i64),  "0a", false, same),
    case(10i128, val!(10i128), "0a", false, same),
    case(23u8,   val!(23u8),   "17", false, same),
    case(23u16,  val!(23u16),  "17", false, same),
    case(23u32,  val!(23u32),  "17", false, same),
    case(23u64,  val!(23u64),  "17", false, same),
    case(23u128, val!(23u128), "17", false, same),
    case(23i8,   val!(23i8),   "17", false, same),
    case(23i16,  val!(23i16),  "17", false, same),
    case(23i32,  val!(23i32),  "17", false, same),
    case(23i64,  val!(23i64),  "17", false, same),
    case(23i128, val!(23i128), "17", false, same),
    case(24u8,   val!(24u8),   "1818", false, same),
    case(24u16,  val!(24u16),  "1818", false, same),
    case(24u32,  val!(24u32),  "1818", false, same),
    case(24u64,  val!(24u64),  "1818", false, same),
    case(24u128, val!(24u128), "1818", false, same),
    case(24i8,   val!(24i8),   "1818", false, same),
    case(24i16,  val!(24i16),  "1818", false, same),
    case(24i32,  val!(24i32),  "1818", false, same),
    case(24i64,  val!(24i64),  "1818", false, same),
    case(24i128, val!(24i128), "1818", false, same),
    case(25u8,   val!(25u8),   "1819", false, same),
    case(25u16,  val!(25u16),  "1819", false, same),
    case(25u32,  val!(25u32),  "1819", false, same),
    case(25u64,  val!(25u64),  "1819", false, same),
    case(25u128, val!(25u128), "1819", false, same),
    case(25i8,   val!(25i8),   "1819", false, same),
    case(25i16,  val!(25i16),  "1819", false, same),
    case(25i32,  val!(25i32),  "1819", false, same),
    case(25i64,  val!(25i64),  "1819", false, same),
    case(25i128, val!(25i128), "1819", false, same),
    case(100u8,   val!(100u8),   "1864", false, same),
    case(100u16,  val!(100u16),  "1864", false, same),
    case(100u32,  val!(100u32),  "1864", false, same),
    case(100u64,  val!(100u64),  "1864", false, same),
    case(100u128, val!(100u128), "1864", false, same),
    case(100i8,   val!(100i8),   "1864", false, same),
    case(100i16,  val!(100i16),  "1864", false, same),
    case(100i32,  val!(100i32),  "1864", false, same),
    case(100i64,  val!(100i64),  "1864", false, same),
    case(100i128, val!(100i128), "1864", false, same),
    case(1000u16,  val!(1000u16),  "1903e8", false, same),
    case(1000u32,  val!(1000u32),  "1903e8", false, same),
    case(1000u64,  val!(1000u64),  "1903e8", false, same),
    case(1000u128, val!(1000u128), "1903e8", false, same),
    case(1000i16,  val!(1000i16),  "1903e8", false, same),
    case(1000i32,  val!(1000i32),  "1903e8", false, same),
    case(1000i64,  val!(1000i64),  "1903e8", false, same),
    case(1000i128, val!(1000i128), "1903e8", false, same),
    case(1000000u32,  val!(1000000u32),  "1a000f4240", false, same),
    case(1000000u64,  val!(1000000u64),  "1a000f4240", false, same),
    case(1000000u128, val!(1000000u128), "1a000f4240", false, same),
    case(1000000i32,  val!(1000000i32),  "1a000f4240", false, same),
    case(1000000i64,  val!(1000000i64),  "1a000f4240", false, same),
    case(1000000i128, val!(1000000i128), "1a000f4240", false, same),
    case(1000000000000u64,  val!(1000000000000u64),  "1b000000e8d4a51000", false, same),
    case(1000000000000u128, val!(1000000000000u128), "1b000000e8d4a51000", false, same),
    case(1000000000000i64,  val!(1000000000000i64),  "1b000000e8d4a51000", false, same),
    case(1000000000000i128, val!(1000000000000i128), "1b000000e8d4a51000", false, same),
    case(18446744073709551615u64,  val!(18446744073709551615u64),  "1bffffffffffffffff", false, same),
    case(18446744073709551615u128, val!(18446744073709551615u128), "1bffffffffffffffff", false, same),
    case(18446744073709551615i128, val!(18446744073709551615i128), "1bffffffffffffffff", false, same),
    case(18446744073709551616u128, val!(18446744073709551616u128), "c249010000000000000000", false, same),
    case(18446744073709551616i128, val!(18446744073709551616i128), "c249010000000000000000", false, same),
    case(-18446744073709551617i128, val!(-18446744073709551617i128), "c349010000000000000000", false, same),
    case(-18446744073709551616i128, val!(-18446744073709551616i128), "3bffffffffffffffff", false, same),
    case(-1000i16,  val!(-1000i16),  "3903e7", false, same),
    case(-1000i32,  val!(-1000i32),  "3903e7", false, same),
    case(-1000i64,  val!(-1000i64),  "3903e7", false, same),
    case(-1000i128, val!(-1000i128), "3903e7", false, same),
    case(-100i8,   val!(-100i8),   "3863", false, same),
    case(-100i16,  val!(-100i16),  "3863", false, same),
    case(-100i32,  val!(-100i32),  "3863", false, same),
    case(-100i64,  val!(-100i64),  "3863", false, same),
    case(-100i128, val!(-100i128), "3863", false, same),
    case(-10i8,   val!(-10i8),   "29", false, same),
    case(-10i16,  val!(-10i16),  "29", false, same),
    case(-10i32,  val!(-10i32),  "29", false, same),
    case(-10i64,  val!(-10i64),  "29", false, same),
    case(-10i128, val!(-10i128), "29", false, same),
    case(-1i8,   val!(-1i8),   "20", false, same),
    case(-1i16,  val!(-1i16),  "20", false, same),
    case(-1i32,  val!(-1i32),  "20", false, same),
    case(-1i64,  val!(-1i64),  "20", false, same),
    case(-1i128, val!(-1i128), "20", false, same),
    case(-1i8,   val!(-1i8),   "3b0000000000000000", true, same),
    case(-1i16,  val!(-1i16),  "3b0000000000000000", true, same),
    case(-1i32,  val!(-1i32),  "3b0000000000000000", true, same),
    case(-1i64,  val!(-1i64),  "3b0000000000000000", true, same),
    case(-1i128, val!(-1i128), "3b0000000000000000", true, same),
    case(0.0f32, val!(0.0f32), "f90000", false, Float),
    case(0.0f64, val!(0.0f64), "f90000", false, Float),
    case(-0.0f32, val!(-0.0f32), "f98000", false, Float),
    case(-0.0f64, val!(-0.0f64), "f98000", false, Float),
    case(1.0f32, val!(1.0f32), "f93c00", false, Float),
    case(1.0f64, val!(1.0f64), "f93c00", false, Float),
    case(1.1f32, val!(1.1f32), "fa3f8ccccd", false, Float), // Not In RFC
    case(1.1f64, val!(1.1f64), "fb3ff199999999999a", false, Float),
    case(1.5f32, val!(1.5f32), "f93e00", false, Float),
    case(1.5f64, val!(1.5f64), "f93e00", false, Float),
    case(65504.0f32, val!(65504.0f32), "f97bff", false, Float),
    case(65504.0f64, val!(65504.0f64), "f97bff", false, Float),
    case(100000.0f32, val!(100000.0f32), "fa47c35000", false, Float),
    case(100000.0f64, val!(100000.0f64), "fa47c35000", false, Float),
    case(3.4028234663852886e+38f32, val!(3.4028234663852886e+38f32), "fa7f7fffff", false, Float),
    case(3.4028234663852886e+38f64, val!(3.4028234663852886e+38f64), "fa7f7fffff", false, Float),
    case(1.0e+300f64, val!(1.0e+300f64), "fb7e37e43c8800759c", false, Float),
    case(5.960464477539063e-8f32, val!(5.960464477539063e-8f32), "f90001", false, Float),
    case(5.960464477539063e-8f64, val!(5.960464477539063e-8f64), "f90001", false, Float),
    case(0.00006103515625f32, val!(0.00006103515625f32), "f90400", false, Float),
    case(0.00006103515625f64, val!(0.00006103515625f64), "f90400", false, Float),
    case(-4.0f32, val!(-4.0f32), "f9c400", false, Float),
    case(-4.0f64, val!(-4.0f64), "f9c400", false, Float),
    case(-4.1f32, val!(-4.1f32), "fac0833333", false, Float), // Not In RFC
    case(-4.1f64, val!(-4.1f64), "fbc010666666666666", false, Float),
    case(core::f32::INFINITY, val!(core::f32::INFINITY), "f97c00", false, Float),
    case(core::f64::INFINITY, val!(core::f64::INFINITY), "f97c00", false, Float),
    case(core::f32::INFINITY, val!(core::f32::INFINITY), "fa7f800000", true, Float),
    case(core::f64::INFINITY, val!(core::f64::INFINITY), "fa7f800000", true, Float),
    case(core::f32::INFINITY, val!(core::f32::INFINITY), "fb7ff0000000000000", true, Float),
    case(core::f64::INFINITY, val!(core::f64::INFINITY), "fb7ff0000000000000", true, Float),
    case(-core::f32::INFINITY, val!(-core::f32::INFINITY), "f9fc00", false, Float),
    case(-core::f64::INFINITY, val!(-core::f64::INFINITY), "f9fc00", false, Float),
    case(-core::f32::INFINITY, val!(-core::f32::INFINITY), "faff800000", true, Float),
    case(-core::f64::INFINITY, val!(-core::f64::INFINITY), "faff800000", true, Float),
    case(-core::f32::INFINITY, val!(-core::f32::INFINITY), "fbfff0000000000000", true, Float),
    case(-core::f64::INFINITY, val!(-core::f64::INFINITY), "fbfff0000000000000", true, Float),
    case(core::f32::NAN, val!(core::f32::NAN), "f97e00", false, Float),
    case(core::f64::NAN, val!(core::f64::NAN), "f97e00", false, Float),
    case(core::f32::NAN, val!(core::f32::NAN), "fa7fc00000", true, Float),
    case(core::f64::NAN, val!(core::f64::NAN), "fa7fc00000", true, Float),
    case(core::f32::NAN, val!(core::f32::NAN), "fb7ff8000000000000", true, Float),
    case(core::f64::NAN, val!(core::f64::NAN), "fb7ff8000000000000", true, Float),
    case(-core::f32::NAN, val!(-core::f32::NAN), "f9fe00", false, Float),            // Not In RFC
    case(-core::f64::NAN, val!(-core::f64::NAN), "f9fe00", false, Float),            // Not In RFC
    case(-core::f32::NAN, val!(-core::f32::NAN), "faffc00000", true, Float),         // Not In RFC
    case(-core::f64::NAN, val!(-core::f64::NAN), "faffc00000", true, Float),         // Not In RFC
    case(-core::f32::NAN, val!(-core::f32::NAN), "fbfff8000000000000", true, Float), // Not In RFC
    case(-core::f64::NAN, val!(-core::f64::NAN), "fbfff8000000000000", true, Float), // Not In RFC
    case(false, val!(false), "f4", false, same),
    case(true, val!(true), "f5", false, same),
    case(Value::Null, Value::Null, "f6", false, same),
    case(hex!(""), val!(&b""[..]), "40", false, same),
    case(hex!("01020304"), val!(&b"\x01\x02\x03\x04"[..]), "4401020304", false, same),
    case(hex!("0102030405"), val!(&b"\x01\x02\x03\x04\x05"[..]), "5f42010243030405ff", true, same),
    case("", val!(""), "60", false, ToOwned::to_owned),
    case("a", val!("a"), "6161", false, ToOwned::to_owned),
    case('a', val!('a'), "6161", false, same),
    case("IETF", val!("IETF"), "6449455446", false, ToOwned::to_owned),
    case("\"\\", val!("\"\\"), "62225c", false, ToOwned::to_owned),
    case("ü", val!("ü"), "62c3bc", false, ToOwned::to_owned),
    case('ü', val!('ü'), "62c3bc", false, same),
    case("水", val!("水"), "63e6b0b4", false, ToOwned::to_owned),
    case('水', val!('水'), "63e6b0b4", false, same),
    case("𐅑", val!("𐅑"), "64f0908591", false, ToOwned::to_owned),
    case('𐅑', val!('𐅑'), "64f0908591", false, same),
    case("streaming", val!("streaming"), "7f657374726561646d696e67ff", true, ToOwned::to_owned),
    case(cbor!([]).unwrap(), Vec::<Value>::new().into(), "80", false, same),
    case(cbor!([]).unwrap(), Vec::<Value>::new().into(), "9fff", true, same),
    case(cbor!([1, 2, 3]).unwrap(), cbor!([1, 2, 3]).unwrap(), "83010203", false, same),
    case(cbor!([1, [2, 3], [4, 5]]).unwrap(), cbor!([1, [2, 3], [4, 5]]).unwrap(), "8301820203820405", false, same),
    case(cbor!([1, [2, 3], [4, 5]]).unwrap(), cbor!([1, [2, 3], [4, 5]]).unwrap(), "9f018202039f0405ffff", true, same),
    case(cbor!([1, [2, 3], [4, 5]]).unwrap(), cbor!([1, [2, 3], [4, 5]]).unwrap(), "9f01820203820405ff", true, same),
    case(cbor!([1, [2, 3], [4, 5]]).unwrap(), cbor!([1, [2, 3], [4, 5]]).unwrap(), "83018202039f0405ff", true, same),
    case(cbor!([1, [2, 3], [4, 5]]).unwrap(), cbor!([1, [2, 3], [4, 5]]).unwrap(), "83019f0203ff820405", true, same),
    case((1..=25).collect::<Vec<u8>>(), (1..=25).map(|x| x.into()).collect::<Vec<Value>>().into(), "98190102030405060708090a0b0c0d0e0f101112131415161718181819", false, same),
    case((1..=25).collect::<Vec<u8>>(), (1..=25).map(|x| x.into()).collect::<Vec<Value>>().into(), "9f0102030405060708090a0b0c0d0e0f101112131415161718181819ff", true, same),
    case(HashMap::<u8, u8>::new(), Value::Map(vec![]), "a0", false, same),
    case(BTreeMap::<u8, u8>::new(), Value::Map(vec![]), "a0", false, same),
    case(map!{1 => 2, 3 => 4}, cbor!({1 => 2, 3 => 4}).unwrap(), "a201020304", false, same),
    case(cbor!({"a" => 1, "b" => [2, 3]}).unwrap(), cbor!({"a" => 1, "b" => [2, 3]}).unwrap(), "a26161016162820203", false, same),
    case(cbor!({"a" => 1, "b" => [2, 3]}).unwrap(), cbor!({"a" => 1, "b" => [2, 3]}).unwrap(), "bf61610161629f0203ffff", true, same),
    case(cbor!(["a", {"b" => "c"}]).unwrap(), cbor!(["a", {"b" => "c"}]).unwrap(), "826161a161626163", false, same),
    case(cbor!(["a", {"b" => "c"}]).unwrap(), cbor!(["a", {"b" => "c"}]).unwrap(), "826161bf61626163ff", true, same),
    case(cbor!({"Fun" => true, "Amt" => -2}).unwrap(), cbor!({"Fun" => true, "Amt" => -2}).unwrap(), "bf6346756ef563416d7421ff", true, same),
    case(map_big(), vmap_big(), "a56161614161626142616361436164614461656145", false, same),
    case(Option::<u8>::None, Value::Null, "f6", false, same), // Not In RFC
    case(Option::Some(7u8), val!(7u8), "07", false, same), // Not In RFC
    case((), Value::Null, "f6", false, same), // Not In RFC
    case(UnitStruct, Value::Null, "f6", false, same), // Not In RFC
    case(Newtype(123), val!(123u8), "187b", false, same), // Not In RFC
    case((22u8, 23u16), cbor!([22, 23]).unwrap(), "821617", false, same), // Not In RFC
    case(TupleStruct(33, 34), cbor!([33, 34]).unwrap(), "8218211822", false, same), // Not In RFC
    case(Enum::Unit, cbor!("Unit").unwrap(), "64556e6974", false, same), // Not In RFC
    case(Enum::Newtype(45), cbor!({"Newtype" => 45}).unwrap(), "a1674e657774797065182d", false, same), // Not In RFC
    case(Enum::Tuple(56, 67), cbor!({"Tuple" => [56, 67]}).unwrap(), "a1655475706c658218381843", false, same), // Not In RFC
    case(Enum::Struct { first: 78, second: 89 }, cbor!({ "Struct" => { "first" => 78, "second" => 89 }}).unwrap(), "a166537472756374a2656669727374184e667365636f6e641859", false, same), // Not In RFC
)]
fn codec<'de, T: Serialize + Clone, V: Debug + PartialEq + DeserializeOwned, F: Fn(T) -> V>(
    input: T,
    value: Value,
    bytes: &str,
    alternate: bool,
    equality: F,
) {
    let bytes = hex::decode(bytes).unwrap();

    if !alternate {
        let mut encoded = Vec::new();
        into_writer(&input, &mut encoded).unwrap();
        eprintln!("{:x?} == {:x?}", bytes, encoded);
        assert_eq!(bytes, encoded);

        let mut encoded = Vec::new();
        into_writer(&value, &mut encoded).unwrap();
        eprintln!("{:x?} == {:x?}", bytes, encoded);
        assert_eq!(bytes, encoded);

        let encoded = Value::serialized(&input).unwrap();
        eprintln!("{:x?} == {:x?}", &value, &encoded);
        assert!(veq(&value, &encoded));
    }

    let decoded: V = from_reader(&bytes[..]).unwrap();
    let answer = equality(input.clone());
    eprintln!("{:x?} == {:x?}", answer, decoded);
    assert_eq!(answer, decoded);

    let decoded: Value = from_reader(&bytes[..]).unwrap();
    eprintln!("{:x?} == {:x?}", &value, &decoded);
    assert!(veq(&value, &decoded));

    let mut scratch = vec![0; 65536];
    let decoded: Value = from_reader_with_buffer(&bytes[..], &mut scratch).unwrap();
    eprintln!("{:x?} == {:x?}", &value, &decoded);
    assert!(veq(&value, &decoded));

    let decoded: V = value.deserialized().unwrap();
    let answer = equality(input);
    eprintln!("{:x?} == {:x?}", answer, decoded);
    assert_eq!(answer, decoded);
}

#[inline]
fn veq(lhs: &Value, rhs: &Value) -> bool {
    if let Value::Float(l) = lhs {
        if let Value::Float(r) = rhs {
            return Float(*l) == Float(*r);
        }
    }

    lhs == rhs
}

#[inline]
fn same<T>(x: T) -> T {
    x
}

#[derive(Debug, Deserialize)]
struct Float<T>(T);

impl PartialEq for Float<f32> {
    fn eq(&self, other: &Float<f32>) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            return true;
        }

        self.0 == other.0
    }
}

impl PartialEq for Float<f64> {
    fn eq(&self, other: &Float<f64>) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            return true;
        }

        self.0 == other.0
    }
}

#[inline]
fn map_big() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    map.insert("a".into(), "A".into());
    map.insert("b".into(), "B".into());
    map.insert("c".into(), "C".into());
    map.insert("d".into(), "D".into());
    map.insert("e".into(), "E".into());
    map
}

#[inline]
fn vmap_big() -> Value {
    Value::Map(
        map_big()
            .into_iter()
            .map(|x| (x.0.into(), x.1.into()))
            .collect(),
    )
}

#[inline]
fn bigint() -> Value {
    let bytes = hex::decode("0000000000000000000000000000000000000001").unwrap();
    Value::Tag(2, Value::Bytes(bytes).into())
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
struct UnitStruct;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
struct TupleStruct(u8, u16);

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
struct Newtype(u8);

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq)]
enum Enum {
    Unit,
    Newtype(u8),
    Tuple(u8, u16),
    Struct { first: u8, second: u16 },
}

#[rstest(
    input,
    case(vec![]),
    case(vec![0u8, 1, 2, 3]),
)]
fn byte_vec_serde_bytes_compatibility(input: Vec<u8>) {
    use serde_bytes::ByteBuf;

    let mut buf = Vec::new();
    into_writer(&input, &mut buf).unwrap();
    let bytes: ByteBuf = from_reader(&buf[..]).unwrap();
    assert_eq!(input, bytes.to_vec());

    let mut buf = Vec::new();
    into_writer(&ByteBuf::from(input.clone()), &mut buf).unwrap();
    let bytes: Vec<u8> = from_reader(&buf[..]).unwrap();
    assert_eq!(input, bytes);
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
struct Foo {
    bar: u8,
}

#[rstest(input, expected,
    case("a163626172182a", Foo { bar: 42 }),
    case("a143626172182a", Foo { bar: 42 }),
)]
fn handle_struct_field_names(input: &str, expected: Foo) {
    let buf = hex::decode(input).unwrap();
    let read = from_reader(&buf[..]).unwrap();
    assert_eq!(expected, read);
}
