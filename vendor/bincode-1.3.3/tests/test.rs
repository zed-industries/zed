#[macro_use]
extern crate serde_derive;

extern crate bincode;
#[macro_use]
extern crate serde;
extern crate serde_bytes;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::result::Result as StdResult;

use bincode::{
    deserialize, deserialize_from, deserialize_in_place, serialize, serialized_size,
    DefaultOptions, ErrorKind, Options, Result,
};
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};

const LEN_SIZE: u64 = 8;

fn the_same_impl<V, O>(element: V, options: &mut O)
where
    V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    O: Options,
{
    let size = options.serialized_size(&element).unwrap();

    {
        let encoded = options.serialize(&element).unwrap();
        let decoded: V = options.deserialize(&encoded[..]).unwrap();
        let decoded_reader = options.deserialize_from(&mut &encoded[..]).unwrap();

        assert_eq!(element, decoded);
        assert_eq!(element, decoded_reader);
        assert_eq!(size, encoded.len() as u64);
    }
}

fn the_same<V>(element: V)
where
    V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + Clone + 'static,
{
    // add a new macro which calls the previous when you add a new option set
    macro_rules! all_endians {
        ($element:expr, $options:expr) => {
            the_same_impl($element.clone(), &mut $options.with_native_endian());
            the_same_impl($element.clone(), &mut $options.with_big_endian());
            the_same_impl($element.clone(), &mut $options.with_little_endian());
        };
    }

    macro_rules! all_integer_encodings {
        ($element:expr, $options:expr) => {
            all_endians!($element, $options.with_fixint_encoding());
            all_endians!($element, $options.with_varint_encoding());
        };
    }

    all_integer_encodings!(element, DefaultOptions::new());
}

#[test]
fn test_numbers() {
    // unsigned positive
    the_same(5u8);
    the_same(5u16);
    the_same(5u32);
    the_same(5u64);
    the_same(5usize);
    // signed positive
    the_same(5i8);
    the_same(5i16);
    the_same(5i32);
    the_same(5i64);
    the_same(5isize);
    // signed negative
    the_same(-5i8);
    the_same(-5i16);
    the_same(-5i32);
    the_same(-5i64);
    the_same(-5isize);
    // floating
    the_same(-100f32);
    the_same(0f32);
    the_same(5f32);
    the_same(-100f64);
    the_same(5f64);
}

serde_if_integer128! {
    #[test]
    fn test_numbers_128bit() {
        // unsigned positive
        the_same(5u128);
        the_same(u128::max_value());
        // signed positive
        the_same(5i128);
        the_same(i128::max_value());
        // signed negative
        the_same(-5i128);
        the_same(i128::min_value());
    }
}

#[test]
fn test_string() {
    the_same("".to_string());
    the_same("a".to_string());
}

#[test]
fn test_tuple() {
    the_same((1isize,));
    the_same((1isize, 2isize, 3isize));
    the_same((1isize, "foo".to_string(), ()));
}

#[test]
fn test_basic_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Easy {
        x: isize,
        s: String,
        y: usize,
    }
    the_same(Easy {
        x: -4,
        s: "foo".to_string(),
        y: 10,
    });
}

#[test]
fn test_nested_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Easy {
        x: isize,
        s: String,
        y: usize,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Nest {
        f: Easy,
        b: usize,
        s: Easy,
    }

    the_same(Nest {
        f: Easy {
            x: -1,
            s: "foo".to_string(),
            y: 20,
        },
        b: 100,
        s: Easy {
            x: -100,
            s: "bar".to_string(),
            y: 20,
        },
    });
}

#[test]
fn test_struct_newtype() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct NewtypeStr(usize);

    the_same(NewtypeStr(5));
}

#[test]
fn test_struct_tuple() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct TubStr(usize, String, f32);

    the_same(TubStr(5, "hello".to_string(), 3.2));
}

#[test]
fn test_option() {
    the_same(Some(5usize));
    the_same(Some("foo bar".to_string()));
    the_same(None::<usize>);
}

#[test]
fn test_enum() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    enum TestEnum {
        NoArg,
        OneArg(usize),
        Args(usize, usize),
        AnotherNoArg,
        StructLike { x: usize, y: f32 },
    }
    the_same(TestEnum::NoArg);
    the_same(TestEnum::OneArg(4));
    //the_same(TestEnum::Args(4, 5));
    the_same(TestEnum::AnotherNoArg);
    the_same(TestEnum::StructLike { x: 4, y: 3.14159 });
    the_same(vec![
        TestEnum::NoArg,
        TestEnum::OneArg(5),
        TestEnum::AnotherNoArg,
        TestEnum::StructLike { x: 4, y: 1.4 },
    ]);
}

#[test]
fn test_vec() {
    let v: Vec<u8> = vec![];
    the_same(v);
    the_same(vec![1u64]);
    the_same(vec![1u64, 2, 3, 4, 5, 6]);
}

#[test]
fn test_map() {
    let mut m = HashMap::new();
    m.insert(4u64, "foo".to_string());
    m.insert(0u64, "bar".to_string());
    the_same(m);
}

#[test]
fn test_bool() {
    the_same(true);
    the_same(false);
}

#[test]
fn test_unicode() {
    the_same("å".to_string());
    the_same("aåååååååa".to_string());
}

#[test]
fn test_fixed_size_array() {
    the_same([24u32; 32]);
    the_same([1u64, 2, 3, 4, 5, 6, 7, 8]);
    the_same([0u8; 19]);
}

#[test]
fn deserializing_errors() {
    match *deserialize::<bool>(&vec![0xA][..]).unwrap_err() {
        ErrorKind::InvalidBoolEncoding(0xA) => {}
        _ => panic!(),
    }

    let invalid_str = vec![1, 0, 0, 0, 0, 0, 0, 0, 0xFF];

    match *deserialize::<String>(&invalid_str[..]).unwrap_err() {
        ErrorKind::InvalidUtf8Encoding(_) => {}
        _ => panic!(),
    }

    // Out-of-bounds variant
    #[derive(Serialize, Deserialize, Debug)]
    enum Test {
        One,
        Two,
    };

    let invalid_enum = vec![0, 0, 0, 5];

    match *deserialize::<Test>(&invalid_enum[..]).unwrap_err() {
        // Error message comes from serde
        ErrorKind::Custom(_) => {}
        _ => panic!(),
    }
    match *deserialize::<Option<u8>>(&vec![5, 0][..]).unwrap_err() {
        ErrorKind::InvalidTagEncoding(_) => {}
        _ => panic!(),
    }
}

#[test]
fn trailing_bytes() {
    match DefaultOptions::new()
        .deserialize::<char>(b"1x")
        .map_err(|e| *e)
    {
        Err(ErrorKind::Custom(_)) => {}
        other => panic!("Expecting TrailingBytes, got {:?}", other),
    }
}

#[test]
fn too_big_deserialize() {
    let serialized = vec![0, 0, 0, 3];
    let deserialized: Result<u32> = DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(3)
        .deserialize_from(&mut &serialized[..]);
    assert!(deserialized.is_err());

    let serialized = vec![0, 0, 0, 3];
    let deserialized: Result<u32> = DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(4)
        .deserialize_from(&mut &serialized[..]);
    assert!(deserialized.is_ok());
}

#[test]
fn char_serialization() {
    let chars = "Aa\0☺♪";
    for c in chars.chars() {
        let encoded = DefaultOptions::new()
            .with_limit(4)
            .serialize(&c)
            .expect("serializing char failed");
        let decoded: char = deserialize(&encoded).expect("deserializing failed");
        assert_eq!(decoded, c);
    }
}

#[test]
fn too_big_char_deserialize() {
    let serialized = vec![0x41];
    let deserialized: Result<char> = DefaultOptions::new()
        .with_limit(1)
        .deserialize_from(&mut &serialized[..]);
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), 'A');
}

#[test]
fn too_big_serialize() {
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(3)
        .serialize(&0u32)
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(4)
        .serialize(&0u32)
        .is_ok());

    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(LEN_SIZE + 4)
        .serialize(&"abcde")
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(LEN_SIZE + 5)
        .serialize(&"abcde")
        .is_ok());
}

#[test]
fn test_serialized_size() {
    assert!(serialized_size(&0u8).unwrap() == 1);
    assert!(serialized_size(&0u16).unwrap() == 2);
    assert!(serialized_size(&0u32).unwrap() == 4);
    assert!(serialized_size(&0u64).unwrap() == 8);

    // length isize stored as u64
    assert!(serialized_size(&"").unwrap() == LEN_SIZE);
    assert!(serialized_size(&"a").unwrap() == LEN_SIZE + 1);

    assert!(serialized_size(&vec![0u32, 1u32, 2u32]).unwrap() == LEN_SIZE + 3 * (4));
}

#[test]
fn test_serialized_size_bounded() {
    // JUST RIGHT
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(1)
            .serialized_size(&0u8)
            .unwrap()
            == 1
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(2)
            .serialized_size(&0u16)
            .unwrap()
            == 2
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(4)
            .serialized_size(&0u32)
            .unwrap()
            == 4
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(8)
            .serialized_size(&0u64)
            .unwrap()
            == 8
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(8)
            .serialized_size(&"")
            .unwrap()
            == LEN_SIZE
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(8 + 1)
            .serialized_size(&"a")
            .unwrap()
            == LEN_SIZE + 1
    );
    assert!(
        DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(LEN_SIZE + 3 * 4)
            .serialized_size(&vec![0u32, 1u32, 2u32])
            .unwrap()
            == LEN_SIZE + 3 * 4
    );
    // Below
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(0)
        .serialized_size(&0u8)
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(1)
        .serialized_size(&0u16)
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(3)
        .serialized_size(&0u32)
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(7)
        .serialized_size(&0u64)
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(7)
        .serialized_size(&"")
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(8 + 0)
        .serialized_size(&"a")
        .is_err());
    assert!(DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(8 + 3 * 4 - 1)
        .serialized_size(&vec![0u32, 1u32, 2u32])
        .is_err());
}

#[test]
fn encode_box() {
    the_same(Box::new(5));
}

#[test]
fn test_cow_serialize() {
    let large_object = vec![1u32, 2, 3, 4, 5, 6];
    let mut large_map = HashMap::new();
    large_map.insert(1, 2);

    #[derive(Serialize, Deserialize, Debug)]
    enum Message<'a> {
        M1(Cow<'a, Vec<u32>>),
        M2(Cow<'a, HashMap<u32, u32>>),
    }

    // Test 1
    {
        let serialized = serialize(&Message::M1(Cow::Borrowed(&large_object))).unwrap();
        let deserialized: Message<'static> = deserialize_from(&mut &serialized[..]).unwrap();

        match deserialized {
            Message::M1(b) => assert!(&b.into_owned() == &large_object),
            _ => assert!(false),
        }
    }

    // Test 2
    {
        let serialized = serialize(&Message::M2(Cow::Borrowed(&large_map))).unwrap();
        let deserialized: Message<'static> = deserialize_from(&mut &serialized[..]).unwrap();

        match deserialized {
            Message::M2(b) => assert!(&b.into_owned() == &large_map),
            _ => assert!(false),
        }
    }
}

#[test]
fn test_strbox_serialize() {
    let strx: &'static str = "hello world";
    let serialized = serialize(&Cow::Borrowed(strx)).unwrap();
    let deserialized: Cow<'static, String> = deserialize_from(&mut &serialized[..]).unwrap();
    let stringx: String = deserialized.into_owned();
    assert!(strx == &stringx[..]);
}

#[test]
fn test_slicebox_serialize() {
    let slice = [1u32, 2, 3, 4, 5];
    let serialized = serialize(&Cow::Borrowed(&slice[..])).unwrap();
    println!("{:?}", serialized);
    let deserialized: Cow<'static, Vec<u32>> = deserialize_from(&mut &serialized[..]).unwrap();
    {
        let sb: &[u32] = &deserialized;
        assert!(slice == sb);
    }
    let vecx: Vec<u32> = deserialized.into_owned();
    assert!(slice == &vecx[..]);
}

#[test]
fn test_multi_strings_serialize() {
    assert!(serialize(&("foo", "bar", "baz")).is_ok());
}

#[test]
fn test_oom_protection() {
    use std::io::Cursor;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct FakeVec {
        len: u64,
        byte: u8,
    }
    let x = DefaultOptions::new()
        .with_limit(10)
        .serialize(&FakeVec {
            len: 0xffffffffffffffffu64,
            byte: 1,
        })
        .unwrap();
    let y: Result<Vec<u8>> = DefaultOptions::new()
        .with_limit(10)
        .deserialize_from(&mut Cursor::new(&x[..]));
    assert!(y.is_err());
}

#[test]
fn path_buf() {
    use std::path::{Path, PathBuf};
    let path = Path::new("foo").to_path_buf();
    let serde_encoded = serialize(&path).unwrap();
    let decoded: PathBuf = deserialize(&serde_encoded).unwrap();
    assert!(path.to_str() == decoded.to_str());
}

#[test]
fn bytes() {
    use serde_bytes::Bytes;

    let data = b"abc\0123";
    let s = serialize(&data[..]).unwrap();
    let s2 = serialize(&Bytes::new(data)).unwrap();
    assert_eq!(s[..], s2[..]);
}

#[test]
fn serde_bytes() {
    use serde_bytes::ByteBuf;
    the_same(ByteBuf::from(vec![1, 2, 3, 4, 5]));
}

#[test]
fn endian_difference() {
    let x = 10u64;
    let little = serialize(&x).unwrap();
    let big = DefaultOptions::new()
        .with_big_endian()
        .serialize(&x)
        .unwrap();
    assert_ne!(little, big);
}

#[test]
fn test_zero_copy_parse() {
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Foo<'a> {
        borrowed_str: &'a str,
        borrowed_bytes: &'a [u8],
    }

    let f = Foo {
        borrowed_str: "hi",
        borrowed_bytes: &[0, 1, 2, 3],
    };
    {
        let encoded = serialize(&f).unwrap();
        let out: Foo = deserialize(&encoded[..]).unwrap();
        assert_eq!(out, f);
    }
}

#[test]
fn test_zero_copy_parse_deserialize_into() {
    use bincode::BincodeRead;
    use std::io;

    /// A BincodeRead implementation for byte slices
    pub struct SliceReader<'storage> {
        slice: &'storage [u8],
    }

    impl<'storage> SliceReader<'storage> {
        #[inline(always)]
        fn unexpected_eof() -> Box<::ErrorKind> {
            return Box::new(::ErrorKind::Io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "",
            )));
        }
    }

    impl<'storage> io::Read for SliceReader<'storage> {
        #[inline(always)]
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            (&mut self.slice).read(out)
        }
        #[inline(always)]
        fn read_exact(&mut self, out: &mut [u8]) -> io::Result<()> {
            (&mut self.slice).read_exact(out)
        }
    }

    impl<'storage> BincodeRead<'storage> for SliceReader<'storage> {
        #[inline(always)]
        fn forward_read_str<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'storage>,
        {
            use ErrorKind;
            if length > self.slice.len() {
                return Err(SliceReader::unexpected_eof());
            }

            let string = match ::std::str::from_utf8(&self.slice[..length]) {
                Ok(s) => s,
                Err(e) => return Err(ErrorKind::InvalidUtf8Encoding(e).into()),
            };
            let r = visitor.visit_borrowed_str(string);
            self.slice = &self.slice[length..];
            r
        }

        #[inline(always)]
        fn get_byte_buffer(&mut self, length: usize) -> Result<Vec<u8>> {
            if length > self.slice.len() {
                return Err(SliceReader::unexpected_eof());
            }

            let r = &self.slice[..length];
            self.slice = &self.slice[length..];
            Ok(r.to_vec())
        }

        #[inline(always)]
        fn forward_read_bytes<V>(&mut self, length: usize, visitor: V) -> Result<V::Value>
        where
            V: serde::de::Visitor<'storage>,
        {
            if length > self.slice.len() {
                return Err(SliceReader::unexpected_eof());
            }

            let r = visitor.visit_borrowed_bytes(&self.slice[..length]);
            self.slice = &self.slice[length..];
            r
        }
    }

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Foo<'a> {
        borrowed_str: &'a str,
        borrowed_bytes: &'a [u8],
    }

    let f = Foo {
        borrowed_str: "hi",
        borrowed_bytes: &[0, 1, 2, 3],
    };

    {
        let encoded = serialize(&f).unwrap();
        let mut target = Foo {
            borrowed_str: "hello",
            borrowed_bytes: &[10, 11, 12, 13],
        };
        deserialize_in_place(
            SliceReader {
                slice: &encoded[..],
            },
            &mut target,
        )
        .unwrap();
        assert_eq!(target, f);
    }
}

#[test]
fn not_human_readable() {
    use std::net::Ipv4Addr;
    let ip = Ipv4Addr::new(1, 2, 3, 4);
    the_same(ip);
    assert_eq!(&ip.octets()[..], &serialize(&ip).unwrap()[..]);
    assert_eq!(
        ::std::mem::size_of::<Ipv4Addr>() as u64,
        serialized_size(&ip).unwrap()
    );
}

// The example is taken from serde::de::DeserializeSeed.
struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
where
    T: Deserialize<'de>,
{
    // The return type of the `deserialize` method. This implementation
    // appends onto an existing vector but does not create any new data
    // structure, so the return type is ().
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> StdResult<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Visitor implementation that will walk an inner array of the JSON
        // input.
        struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

        impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
        where
            T: Deserialize<'de>,
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of integers")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<(), A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Visit each element in the inner array and push it onto
                // the existing vector.
                while let Some(elem) = seq.next_element()? {
                    self.0.push(elem);
                }
                Ok(())
            }
        }

        deserializer.deserialize_seq(ExtendVecVisitor(self.0))
    }
}

#[test]
fn test_default_deserialize_seed() {
    let config = DefaultOptions::new();

    let data: Vec<_> = (10..100).collect();
    let bytes = config.serialize(&data).expect("Config::serialize failed");

    let mut seed_data: Vec<_> = (0..10).collect();
    {
        let seed = ExtendVec(&mut seed_data);
        config
            .deserialize_seed(seed, &bytes)
            .expect("Config::deserialize_seed failed");
    }

    assert_eq!(seed_data, (0..100).collect::<Vec<_>>());
}

#[test]
fn test_big_endian_deserialize_seed() {
    let config = DefaultOptions::new().with_big_endian();

    let data: Vec<_> = (10..100).collect();
    let bytes = config.serialize(&data).expect("Config::serialize failed");

    let mut seed_data: Vec<_> = (0..10).collect();
    {
        let seed = ExtendVec(&mut seed_data);
        config
            .deserialize_seed(seed, &bytes)
            .expect("Config::deserialize_seed failed");
    }

    assert_eq!(seed_data, (0..100).collect::<Vec<_>>());
}

#[test]
fn test_default_deserialize_from_seed() {
    let config = DefaultOptions::new();

    let data: Vec<_> = (10..100).collect();
    let bytes = config.serialize(&data).expect("Config::serialize failed");

    let mut seed_data: Vec<_> = (0..10).collect();
    {
        let seed = ExtendVec(&mut seed_data);
        config
            .deserialize_from_seed(seed, &mut &*bytes)
            .expect("Config::deserialize_from_seed failed");
    }

    assert_eq!(seed_data, (0..100).collect::<Vec<_>>());
}

#[test]
fn test_big_endian_deserialize_from_seed() {
    let config = DefaultOptions::new().with_big_endian();

    let data: Vec<_> = (10..100).collect();
    let bytes = config.serialize(&data).expect("Config::serialize failed");

    let mut seed_data: Vec<_> = (0..10).collect();
    {
        let seed = ExtendVec(&mut seed_data);
        config
            .deserialize_from_seed(seed, &mut &*bytes)
            .expect("Config::deserialize_from_seed failed");
    }

    assert_eq!(seed_data, (0..100).collect::<Vec<_>>());
}

#[test]
fn test_varint_length_prefixes() {
    let a = vec![(); 127]; // should be a single byte
    let b = vec![(); 250]; // also should be a single byte
    let c = vec![(); 251];
    let d = vec![(); u16::max_value() as usize + 1];

    assert_eq!(
        DefaultOptions::new()
            .with_varint_encoding()
            .serialized_size(&a[..])
            .unwrap(),
        1
    ); // 2 ** 7 - 1
    assert_eq!(
        DefaultOptions::new()
            .with_varint_encoding()
            .serialized_size(&b[..])
            .unwrap(),
        1
    ); // 250
    assert_eq!(
        DefaultOptions::new()
            .with_varint_encoding()
            .serialized_size(&c[..])
            .unwrap(),
        (1 + std::mem::size_of::<u16>()) as u64
    ); // 251
    assert_eq!(
        DefaultOptions::new()
            .with_varint_encoding()
            .serialized_size(&d[..])
            .unwrap(),
        (1 + std::mem::size_of::<u32>()) as u64
    ); // 2 ** 16 + 1
}

#[test]
fn test_byte_vec_struct() {
    #[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
    struct ByteVecs {
        a: Vec<u8>,
        b: Vec<u8>,
        c: Vec<u8>,
    };

    let byte_struct = ByteVecs {
        a: vec![2; 20],
        b: vec![3; 30],
        c: vec![1; 10],
    };

    the_same(byte_struct);
}
