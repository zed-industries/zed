use core::fmt::Debug;
use core::fmt::Write;
use core::ops::Deref;

#[cfg(feature = "heapless")]
use heapless::{FnvIndexMap, String, Vec};

#[cfg(feature = "heapless")]
use postcard::to_vec;

use postcard::from_bytes;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BasicU8S {
    st: u16,
    ei: u8,
    sf: u64,
    tt: u32,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum BasicEnum {
    Bib,
    Bim,
    Bap,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct EnumStruct {
    eight: u8,
    sixt: u16,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum DataEnum {
    Bib(u16),
    Bim(u64),
    Bap(u8),
    Kim(EnumStruct),
    Chi { a: u8, b: u32 },
    Sho(u16, u8),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct NewTypeStruct(u32);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TupleStruct((u8, u16));

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

#[cfg(feature = "heapless")]
#[test]
fn loopback() {
    // Basic types
    test_one((), &[]);
    test_one(false, &[0x00]);
    test_one(true, &[0x01]);
    test_one(5u8, &[0x05]);
    test_one(0xA5C7u16, &[0xC7, 0xCB, 0x02]);
    test_one(0xCDAB3412u32, &[0x92, 0xE8, 0xAC, 0xED, 0x0C]);
    test_one(
        0x1234_5678_90AB_CDEFu64,
        &[0xEF, 0x9B, 0xAF, 0x85, 0x89, 0xCF, 0x95, 0x9A, 0x12],
    );

    // https://github.com/jamesmunns/postcard/pull/83
    test_one(32767i16, &[0xFE, 0xFF, 0x03]);
    test_one(-32768i16, &[0xFF, 0xFF, 0x03]);

    // chars
    test_one('z', &[0x01, 0x7a]);
    test_one('¢', &[0x02, 0xc2, 0xa2]);
    test_one('𐍈', &[0x04, 0xF0, 0x90, 0x8D, 0x88]);
    test_one('🥺', &[0x04, 0xF0, 0x9F, 0xA5, 0xBA]);

    // Structs
    test_one(
        BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        },
        &[
            0xCD, 0xD7, 0x02, 0xFE, 0xBA, 0xB9, 0xB7, 0xDE, 0x9A, 0xE4, 0x90, 0x9A, 0x12, 0xAC,
            0xD9, 0xB2, 0xE5, 0x0A,
        ],
    );

    // Enums!
    test_one(BasicEnum::Bim, &[0x01]);
    test_one(
        DataEnum::Bim(u64::MAX),
        &[
            0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
        ],
    );
    test_one(DataEnum::Bib(u16::MAX), &[0x00, 0xFF, 0xFF, 0x03]);
    test_one(DataEnum::Bap(u8::MAX), &[0x02, 0xFF]);
    test_one(
        DataEnum::Kim(EnumStruct {
            eight: 0xF0,
            sixt: 0xACAC,
        }),
        &[0x03, 0xF0, 0xAC, 0xD9, 0x02],
    );
    test_one(
        DataEnum::Chi {
            a: 0x0F,
            b: 0xC7C7C7C7,
        },
        &[0x04, 0x0F, 0xC7, 0x8F, 0x9F, 0xBE, 0x0C],
    );
    test_one(DataEnum::Sho(0x6969, 0x07), &[0x05, 0xE9, 0xD2, 0x01, 0x07]);

    // Tuples!
    test_one((0x12u8, 0xC7A5u16), &[0x12, 0xA5, 0x8F, 0x03]);

    // Structs!
    test_one(NewTypeStruct(5), &[0x05]);
    test_one(TupleStruct((0xA0, 0x1234)), &[0xA0, 0xB4, 0x24]);

    let mut input: Vec<u8, 4> = Vec::new();
    input.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]).unwrap();
    test_one(input, &[0x04, 0x01, 0x02, 0x03, 0x04]);

    let mut input: String<8> = String::new();
    write!(&mut input, "helLO!").unwrap();
    test_one(input, &[0x06, b'h', b'e', b'l', b'L', b'O', b'!']);

    let mut input: FnvIndexMap<u8, u8, 4> = FnvIndexMap::new();
    input.insert(0x01, 0x05).unwrap();
    input.insert(0x02, 0x06).unwrap();
    input.insert(0x03, 0x07).unwrap();
    input.insert(0x04, 0x08).unwrap();
    test_one(
        input,
        &[0x04, 0x01, 0x05, 0x02, 0x06, 0x03, 0x07, 0x04, 0x08],
    );

    // `CString` (uses `serialize_bytes`/`deserialize_byte_buf`)
    #[cfg(feature = "use-std")]
    test_one(
        std::ffi::CString::new("heLlo").unwrap(),
        &[0x05, b'h', b'e', b'L', b'l', b'o'],
    );
}

#[cfg(feature = "heapless")]
#[track_caller]
fn test_one<T>(data: T, ser_rep: &[u8])
where
    T: Serialize + DeserializeOwned + Eq + PartialEq + Debug,
{
    let serialized: Vec<u8, 2048> = to_vec(&data).unwrap();
    assert_eq!(serialized.len(), ser_rep.len());
    let mut x: ::std::vec::Vec<u8> = vec![];
    x.extend(serialized.deref().iter().cloned());
    // let bysl: &'de [u8] = serialized.deref();
    assert_eq!(x, ser_rep);
    {
        // let deserialized: T = from_bytes(serialized.deref()).unwrap();
        let deserialized: T = from_bytes(&x).unwrap();
        assert_eq!(data, deserialized);
    }
}

#[cfg(feature = "use-std")]
#[test]
fn std_io_loopback() {
    use postcard::from_io;
    use postcard::to_io;

    fn test_io<T>(data: T, ser_rep: &[u8])
    where
        T: Serialize + DeserializeOwned + Eq + PartialEq + Debug,
    {
        let serialized: ::std::vec::Vec<u8> = vec![];
        let ser = to_io(&data, serialized).unwrap();
        assert_eq!(ser.len(), ser_rep.len());
        assert_eq!(ser, ser_rep);
        {
            let mut buff = [0; 2048];
            let x = ser.clone();
            let deserialized: T = from_io((x.as_slice(), &mut buff)).unwrap().0;
            assert_eq!(data, deserialized);
        }
    }

    test_io(DataEnum::Sho(0x6969, 0x07), &[0x05, 0xE9, 0xD2, 0x01, 0x07]);
    test_io(
        BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        },
        &[
            0xCD, 0xD7, 0x02, 0xFE, 0xBA, 0xB9, 0xB7, 0xDE, 0x9A, 0xE4, 0x90, 0x9A, 0x12, 0xAC,
            0xD9, 0xB2, 0xE5, 0x0A,
        ],
    );
}

#[cfg(all(
    any(feature = "embedded-io-04", feature = "embedded-io-06"),
    feature = "alloc"
))]
#[test]
fn std_eio_loopback() {
    use postcard::from_eio;
    use postcard::to_eio;

    fn test_io<T>(data: T, ser_rep: &[u8])
    where
        T: Serialize + DeserializeOwned + Eq + PartialEq + Debug,
    {
        let serialized: ::std::vec::Vec<u8> = vec![];
        let ser = to_eio(&data, serialized).unwrap();
        assert_eq!(ser.len(), ser_rep.len());
        assert_eq!(ser, ser_rep);
        {
            let mut buff = [0; 2048];
            let x = ser.clone();
            let deserialized: T = from_eio((x.as_slice(), &mut buff)).unwrap().0;
            assert_eq!(data, deserialized);
        }
    }

    test_io(DataEnum::Sho(0x6969, 0x07), &[0x05, 0xE9, 0xD2, 0x01, 0x07]);
    test_io(
        BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        },
        &[
            0xCD, 0xD7, 0x02, 0xFE, 0xBA, 0xB9, 0xB7, 0xDE, 0x9A, 0xE4, 0x90, 0x9A, 0x12, 0xAC,
            0xD9, 0xB2, 0xE5, 0x0A,
        ],
    );
}
