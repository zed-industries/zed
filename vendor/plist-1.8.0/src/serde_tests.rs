use serde::{
    de::{Deserialize, DeserializeOwned},
    ser::Serialize,
};
use std::{borrow::Cow, collections::BTreeMap, fmt::Debug, fs::File, io::Cursor};

use crate::{
    from_value,
    stream::{private::Sealed, Event, OwnedEvent, Writer},
    to_value, Data, Date, Deserializer, Dictionary, Error, Integer, Serializer, Uid, Value,
};

struct VecWriter {
    events: Vec<OwnedEvent>,
}

impl VecWriter {
    pub fn new() -> VecWriter {
        VecWriter { events: Vec::new() }
    }

    pub fn into_inner(self) -> Vec<OwnedEvent> {
        self.events
    }
}

impl Writer for VecWriter {
    fn write_start_array(&mut self, len: Option<u64>) -> Result<(), Error> {
        self.events.push(Event::StartArray(len));
        Ok(())
    }

    fn write_start_dictionary(&mut self, len: Option<u64>) -> Result<(), Error> {
        self.events.push(Event::StartDictionary(len));
        Ok(())
    }

    fn write_end_collection(&mut self) -> Result<(), Error> {
        self.events.push(Event::EndCollection);
        Ok(())
    }

    fn write_boolean(&mut self, value: bool) -> Result<(), Error> {
        self.events.push(Event::Boolean(value));
        Ok(())
    }

    fn write_data(&mut self, value: Cow<[u8]>) -> Result<(), Error> {
        self.events
            .push(Event::Data(Cow::Owned(value.into_owned())));
        Ok(())
    }

    fn write_date(&mut self, value: Date) -> Result<(), Error> {
        self.events.push(Event::Date(value));
        Ok(())
    }

    fn write_integer(&mut self, value: Integer) -> Result<(), Error> {
        self.events.push(Event::Integer(value));
        Ok(())
    }

    fn write_real(&mut self, value: f64) -> Result<(), Error> {
        self.events.push(Event::Real(value));
        Ok(())
    }

    fn write_string(&mut self, value: Cow<str>) -> Result<(), Error> {
        self.events
            .push(Event::String(Cow::Owned(value.into_owned())));
        Ok(())
    }

    fn write_uid(&mut self, value: Uid) -> Result<(), Error> {
        self.events.push(Event::Uid(value));
        Ok(())
    }
}

impl Sealed for VecWriter {}

fn new_serializer() -> Serializer<VecWriter> {
    Serializer::new(VecWriter::new())
}

fn new_deserializer(events: Vec<Event>) -> Deserializer<Vec<Result<Event, Error>>> {
    let result_events = events.into_iter().map(Ok).collect();
    Deserializer::new(result_events)
}

fn assert_roundtrip<T>(obj: T, expected_events: &[Event], roundtrip_value: bool)
where
    T: Debug + DeserializeOwned + PartialEq + Serialize,
{
    let mut se = new_serializer();
    obj.serialize(&mut se).unwrap();
    let events = se.into_inner().into_inner();

    let value = if roundtrip_value {
        to_value(&obj).expect("failed to convert object into value")
    } else {
        Value::Boolean(false)
    };

    assert_eq!(&events[..], expected_events);

    if roundtrip_value {
        let expected_value = Value::from_events(expected_events.iter().cloned().map(Ok))
            .expect("failed to convert expected events into value");
        assert_eq!(value, expected_value);
    }

    let mut de = new_deserializer(events);
    let obj_events_roundtrip = T::deserialize(&mut de).unwrap();
    assert_eq!(obj_events_roundtrip, obj);

    if roundtrip_value {
        let obj_value_roundtrip: T = from_value(&value).unwrap();
        assert_eq!(obj_value_roundtrip, obj);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Animal {
    Cow,
    Dog(DogOuter),
    Frog(Result<String, bool>, Option<Vec<f64>>),
    Cat {
        age: Integer,
        name: String,
        firmware: Option<Vec<u8>>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct DogOuter {
    inner: Vec<DogInner>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct DogInner {
    a: (),
    b: usize,
    c: Vec<String>,
    d: Option<Uid>,
    e: Data,
}

#[test]
fn cow() {
    let cow = Animal::Cow;

    let comparison = &[Event::String("Cow".into())];

    assert_roundtrip(cow, comparison, true);
}

#[test]
fn dog() {
    let dog = Animal::Dog(DogOuter {
        inner: vec![DogInner {
            a: (),
            b: 12,
            c: vec!["a".to_string(), "b".to_string()],
            d: Some(Uid::new(42)),
            e: Data::new(vec![20, 22]),
        }],
    });

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("Dog".into()),
        Event::StartDictionary(None),
        Event::String("inner".into()),
        Event::StartArray(Some(1)),
        Event::StartDictionary(None),
        Event::String("a".into()),
        Event::String("".into()),
        Event::String("b".into()),
        Event::Integer(12.into()),
        Event::String("c".into()),
        Event::StartArray(Some(2)),
        Event::String("a".into()),
        Event::String("b".into()),
        Event::EndCollection,
        Event::String("d".into()),
        Event::Uid(Uid::new(42)),
        Event::String("e".into()),
        Event::Data(vec![20, 22].into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(dog, comparison, true);
}

#[test]
fn frog() {
    let frog = Animal::Frog(
        Ok("hello".to_owned()),
        Some(vec![1.0, 2.0, std::f64::consts::PI, 0.000000001, 1.27e31]),
    );

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("Frog".into()),
        Event::StartArray(Some(2)),
        Event::StartDictionary(Some(1)),
        Event::String("Ok".into()),
        Event::String("hello".into()),
        Event::EndCollection,
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartArray(Some(5)),
        Event::Real(1.0),
        Event::Real(2.0),
        Event::Real(std::f64::consts::PI),
        Event::Real(0.000000001),
        Event::Real(1.27e31),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(frog, comparison, true);
}

#[test]
fn cat_with_firmware() {
    let cat = Animal::Cat {
        age: 12.into(),
        name: "Paws".to_owned(),
        firmware: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
    };

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("Cat".into()),
        Event::StartDictionary(None),
        Event::String("age".into()),
        Event::Integer(12.into()),
        Event::String("name".into()),
        Event::String("Paws".into()),
        Event::String("firmware".into()),
        Event::StartArray(Some(9)),
        Event::Integer(0.into()),
        Event::Integer(1.into()),
        Event::Integer(2.into()),
        Event::Integer(3.into()),
        Event::Integer(4.into()),
        Event::Integer(5.into()),
        Event::Integer(6.into()),
        Event::Integer(7.into()),
        Event::Integer(8.into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(cat, comparison, true);
}

#[test]
fn cat_without_firmware() {
    let cat = Animal::Cat {
        age: Integer::from(-12),
        name: "Paws".to_owned(),
        firmware: None,
    };

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("Cat".into()),
        Event::StartDictionary(None),
        Event::String("age".into()),
        Event::Integer(Integer::from(-12)),
        Event::String("name".into()),
        Event::String("Paws".into()),
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(cat, comparison, true);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct NewtypeStruct(NewtypeInner);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct NewtypeInner(u8, u8, u8);

#[test]
fn newtype_struct() {
    let newtype = NewtypeStruct(NewtypeInner(34, 32, 13));

    let comparison = &[
        Event::StartArray(Some(3)),
        Event::Integer(34.into()),
        Event::Integer(32.into()),
        Event::Integer(13.into()),
        Event::EndCollection,
    ];

    assert_roundtrip(newtype, comparison, true);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TypeWithOptions {
    a: Option<String>,
    b: Option<Option<u32>>,
    c: Option<Box<TypeWithOptions>>,
}

#[test]
fn type_with_options() {
    let inner = TypeWithOptions {
        a: None,
        b: Some(Some(12)),
        c: None,
    };

    let obj = TypeWithOptions {
        a: Some("hello".to_owned()),
        b: Some(None),
        c: Some(Box::new(inner)),
    };

    let comparison = &[
        Event::StartDictionary(None),
        Event::String("a".into()),
        Event::String("hello".into()),
        Event::String("b".into()),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::String("c".into()),
        Event::StartDictionary(None),
        Event::String("b".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::Integer(12.into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TypeWithDate {
    a: Option<i32>,
    b: Option<Date>,
}

#[test]
fn type_with_date() {
    let date = Date::from_xml_format("1920-01-01T00:10:00Z").unwrap();

    let obj = TypeWithDate {
        a: Some(28),
        b: Some(date),
    };

    let comparison = &[
        Event::StartDictionary(None),
        Event::String("a".into()),
        Event::Integer(28.into()),
        Event::String("b".into()),
        Event::Date(date),
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn option_some() {
    let obj = Some(12);

    let comparison = &[Event::Integer(12.into())];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn option_none() {
    let obj: Option<u32> = None;

    let comparison = &[];

    // Written as nothing so can't be represented by a `Value`.
    assert_roundtrip(obj, comparison, false);
}

#[test]
fn option_some_some() {
    let obj = Some(Some(12));

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::Integer(12.into()),
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn option_some_none() {
    let obj: Option<Option<u32>> = Some(None);

    let comparison = &[
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn option_dictionary_values() {
    let mut obj = BTreeMap::new();
    obj.insert("a".to_owned(), None);
    obj.insert("b".to_owned(), Some(None));
    obj.insert("c".to_owned(), Some(Some(144)));

    let comparison = &[
        Event::StartDictionary(Some(3)),
        Event::String("a".into()),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::String("b".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::String("c".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::Integer(144.into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn option_dictionary_keys() {
    let mut obj = BTreeMap::new();
    obj.insert(None, 1);
    obj.insert(Some(None), 2);
    obj.insert(Some(Some(144)), 3);

    let comparison = &[
        Event::StartDictionary(Some(3)),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::Integer(1.into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::Integer(2.into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::Integer(144.into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::Integer(3.into()),
        Event::EndCollection,
    ];

    // This example uses non-string dictionary keys which can only be represented by binary plists
    // and not by a `Value`.
    assert_roundtrip(obj, comparison, false);
}

#[test]
fn option_array() {
    let obj = vec![None, Some(None), Some(Some(144))];

    let comparison = &[
        Event::StartArray(Some(3)),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("None".into()),
        Event::String("".into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::StartDictionary(Some(1)),
        Event::String("Some".into()),
        Event::Integer(144.into()),
        Event::EndCollection,
        Event::EndCollection,
        Event::EndCollection,
    ];

    assert_roundtrip(obj, comparison, true);
}

#[test]
fn enum_variant_types() {
    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    enum Foo {
        Unit,
        Newtype(u32),
        Tuple(u32, String),
        Struct { v: u32, s: String },
    }

    let expected = &[Event::String("Unit".into())];
    assert_roundtrip(Foo::Unit, expected, true);

    let expected = &[
        Event::StartDictionary(Some(1)),
        Event::String("Newtype".into()),
        Event::Integer(42.into()),
        Event::EndCollection,
    ];
    assert_roundtrip(Foo::Newtype(42), expected, true);

    let expected = &[
        Event::StartDictionary(Some(1)),
        Event::String("Tuple".into()),
        Event::StartArray(Some(2)),
        Event::Integer(42.into()),
        Event::String("bar".into()),
        Event::EndCollection,
        Event::EndCollection,
    ];
    assert_roundtrip(Foo::Tuple(42, "bar".into()), expected, true);

    let expected = &[
        Event::StartDictionary(Some(1)),
        Event::String("Struct".into()),
        Event::StartDictionary(None),
        Event::String("v".into()),
        Event::Integer(42.into()),
        Event::String("s".into()),
        Event::String("bar".into()),
        Event::EndCollection,
        Event::EndCollection,
    ];
    assert_roundtrip(
        Foo::Struct {
            v: 42,
            s: "bar".into(),
        },
        expected,
        true,
    );
}

#[test]
fn deserialise_old_enum_unit_variant_encoding() {
    #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
    enum Foo {
        Bar,
        Baz,
    }

    // `plist` before v1.1 serialised unit enum variants as if they were newtype variants
    // containing an empty string.
    let events = &[
        Event::StartDictionary(Some(1)),
        Event::String("Baz".into()),
        Event::String("".into()),
        Event::EndCollection,
    ];

    let mut de = new_deserializer(events.to_vec());
    let obj = Foo::deserialize(&mut de).unwrap();

    assert_eq!(obj, Foo::Baz);
}

#[test]
fn deserialize_dictionary_xml() {
    let reader = File::open("./tests/data/xml.plist").unwrap();
    let dict: Dictionary = crate::from_reader(reader).unwrap();

    check_common_plist(&dict);

    // xml.plist has this member, but binary.plist does not.
    assert_eq!(
        dict.get("HexademicalNumber") // sic
            .unwrap()
            .as_unsigned_integer()
            .unwrap(),
        0xDEADBEEF
    );
}

#[test]
fn deserialize_dictionary_binary() {
    let reader = File::open("./tests/data/binary.plist").unwrap();
    let dict: Dictionary = crate::from_reader(reader).unwrap();

    check_common_plist(&dict);
}

// Shared checks used by the tests deserialize_dictionary_xml() and
// deserialize_dictionary_binary(), which load files with different formats
// but the same data elements.
fn check_common_plist(dict: &Dictionary) {
    // Array elements

    let lines = dict.get("Lines").unwrap().as_array().unwrap();
    assert_eq!(lines.len(), 2);
    assert_eq!(
        lines[0].as_string().unwrap(),
        "It is a tale told by an idiot,     "
    );
    assert_eq!(
        lines[1].as_string().unwrap(),
        "Full of sound and fury, signifying nothing."
    );

    // Dictionary
    //
    // There is no embedded dictionary in this plist.  See
    // deserialize_dictionary_binary_nskeyedarchiver() below for an example
    // of that.

    // Boolean elements

    assert!(dict.get("IsTrue").unwrap().as_boolean().unwrap());

    assert!(!dict.get("IsNotFalse").unwrap().as_boolean().unwrap());

    // Data

    let data = dict.get("Data").unwrap().as_data().unwrap();
    assert_eq!(data.len(), 15);
    assert_eq!(
        data,
        &[0, 0, 0, 0xbe, 0, 0, 0, 0x03, 0, 0, 0, 0x1e, 0, 0, 0]
    );

    // Date

    assert_eq!(
        dict.get("Birthdate").unwrap().as_date().unwrap(),
        Date::from_xml_format("1981-05-16T11:32:06Z").unwrap()
    );

    // Real

    assert_eq!(dict.get("Height").unwrap().as_real().unwrap(), 1.6);

    // Integer

    assert_eq!(
        dict.get("BiggestNumber")
            .unwrap()
            .as_unsigned_integer()
            .unwrap(),
        18446744073709551615
    );

    assert_eq!(
        dict.get("Death").unwrap().as_unsigned_integer().unwrap(),
        1564
    );

    assert_eq!(
        dict.get("SmallestNumber")
            .unwrap()
            .as_signed_integer()
            .unwrap(),
        -9223372036854775808
    );

    // String

    assert_eq!(
        dict.get("Author").unwrap().as_string().unwrap(),
        "William Shakespeare"
    );

    assert_eq!(dict.get("Blank").unwrap().as_string().unwrap(), "");

    // Uid
    //
    // No checks for Uid value type in this test. See
    // deserialize_dictionary_binary_nskeyedarchiver() below for an example
    // of that.
}

#[test]
fn deserialize_dictionary_binary_nskeyedarchiver() {
    let reader = File::open("./tests/data/binary_NSKeyedArchiver.plist").unwrap();
    let dict: Dictionary = crate::from_reader(reader).unwrap();

    assert_eq!(
        dict.get("$archiver").unwrap().as_string().unwrap(),
        "NSKeyedArchiver"
    );

    let objects = dict.get("$objects").unwrap().as_array().unwrap();
    assert_eq!(objects.len(), 5);

    assert_eq!(objects[0].as_string().unwrap(), "$null");

    let objects_1 = objects[1].as_dictionary().unwrap();
    assert_eq!(
        *objects_1.get("$class").unwrap().as_uid().unwrap(),
        Uid::new(4)
    );
    assert_eq!(
        objects_1
            .get("NSRangeCount")
            .unwrap()
            .as_unsigned_integer()
            .unwrap(),
        42
    );
    assert_eq!(
        *objects_1.get("NSRangeData").unwrap().as_uid().unwrap(),
        Uid::new(2)
    );

    let objects_2 = objects[2].as_dictionary().unwrap();
    assert_eq!(
        *objects_2.get("$class").unwrap().as_uid().unwrap(),
        Uid::new(3)
    );
    let objects_2_nsdata = objects_2.get("NS.data").unwrap().as_data().unwrap();
    assert_eq!(objects_2_nsdata.len(), 103);
    assert_eq!(objects_2_nsdata[0], 0x03);
    assert_eq!(objects_2_nsdata[102], 0x01);

    let objects_3 = objects[3].as_dictionary().unwrap();
    let objects_3_classes = objects_3.get("$classes").unwrap().as_array().unwrap();
    assert_eq!(objects_3_classes.len(), 3);
    assert_eq!(objects_3_classes[0].as_string().unwrap(), "NSMutableData");
    assert_eq!(objects_3_classes[1].as_string().unwrap(), "NSData");
    assert_eq!(objects_3_classes[2].as_string().unwrap(), "NSObject");
    assert_eq!(
        objects_3.get("$classname").unwrap().as_string().unwrap(),
        "NSMutableData"
    );

    let objects_4 = objects[4].as_dictionary().unwrap();
    let objects_4_classes = objects_4.get("$classes").unwrap().as_array().unwrap();
    assert_eq!(objects_4_classes.len(), 3);
    assert_eq!(
        objects_4_classes[0].as_string().unwrap(),
        "NSMutableIndexSet"
    );
    assert_eq!(objects_4_classes[1].as_string().unwrap(), "NSIndexSet");
    assert_eq!(objects_4_classes[2].as_string().unwrap(), "NSObject");
    assert_eq!(
        objects_4.get("$classname").unwrap().as_string().unwrap(),
        "NSMutableIndexSet"
    );

    let top = dict.get("$top").unwrap().as_dictionary().unwrap();
    assert_eq!(
        *top.get("foundItems").unwrap().as_uid().unwrap(),
        Uid::new(1)
    );

    let version = dict.get("$version").unwrap().as_unsigned_integer().unwrap();
    assert_eq!(version, 100000);
}

fn try_parse_xml(bom: bool, whitespace: bool, decl: bool, comment: bool, doctype: bool) -> bool {
    #[derive(Deserialize)]
    struct LayerinfoData {
        color: Option<String>,
    }

    let mut data = Vec::new();
    if bom {
        // The UTF-8 byte order mark
        data.extend(b"\xef\xbb\xbf");
    }

    if whitespace {
        data.extend(b"\r\n\t ");
    }

    if decl {
        data.extend(br#"<?xml version="1.0" encoding="UTF-8"?>"#);
    }

    if comment {
        data.extend(br#"<!-- hello -->"#);
    }

    if doctype {
        data.extend(br#"<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">"#);
    }

    data.extend(
        br#"<plist version="1.0">
        <dict>
            <key>color</key>
            <string>1,0.75,0,0.7</string>
        </dict>
        </plist>
        "#,
    );

    if let Ok(lib_dict) = crate::from_bytes::<LayerinfoData>(&data) {
        lib_dict.color.unwrap() == "1,0.75,0,0.7"
    } else {
        false
    }
}

#[test]
fn xml_detection() {
    for bom in [true, false] {
        for whitespace in [true, false] {
            for decl in [true, false] {
                for comment in [true, false] {
                    for doctype in [true, false] {
                        assert!(
                        try_parse_xml(bom, whitespace, decl, comment, doctype),
                        "bom={bom}, whitespace={whitespace}, decl={decl}, comment={comment}, doctype={doctype}"
                    );
                    }
                }
            }
        }
    }
}

#[test]
fn dictionary_deserialize_dictionary_in_struct() {
    // Example from <https://github.com/ebarnard/rust-plist/issues/54>
    #[derive(Deserialize)]
    struct LayerinfoData {
        color: Option<String>,
        lib: Option<Dictionary>,
    }

    let lib_dict: LayerinfoData = crate::from_bytes(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>color</key>
            <string>1,0.75,0,0.7</string>
            <key>lib</key>
            <dict>
            <key>com.typemytype.robofont.segmentType</key>
            <string>curve</string>
            </dict>
        </dict>
        </plist>
        "#.as_bytes()).unwrap();

    assert_eq!(lib_dict.color.unwrap(), "1,0.75,0,0.7");
    assert_eq!(
        lib_dict
            .lib
            .unwrap()
            .get("com.typemytype.robofont.segmentType")
            .unwrap()
            .as_string()
            .unwrap(),
        "curve"
    );
}

#[test]
fn dictionary_serialize_xml() {
    // Dictionary to be embedded in dict, below.
    let mut inner_dict = Dictionary::new();
    inner_dict.insert(
        "FirstKey".to_owned(),
        Value::String("FirstValue".to_owned()),
    );
    inner_dict.insert("SecondKey".to_owned(), Value::Data(vec![10, 20, 30, 40]));
    inner_dict.insert("ThirdKey".to_owned(), Value::Real(1.234));
    inner_dict.insert(
        "FourthKey".to_owned(),
        Value::Date(Date::from_xml_format("1981-05-16T11:32:06Z").unwrap()),
    );

    // Top-level dictionary.
    let mut dict = Dictionary::new();
    dict.insert(
        "AnArray".to_owned(),
        Value::Array(vec![
            Value::String("Hello, world!".to_owned()),
            Value::Integer(Integer::from(345)),
        ]),
    );
    dict.insert("ADictionary".to_owned(), Value::Dictionary(inner_dict));
    dict.insert("AnInteger".to_owned(), Value::Integer(Integer::from(123)));
    dict.insert("ATrueBoolean".to_owned(), Value::Boolean(true));
    dict.insert("AFalseBoolean".to_owned(), Value::Boolean(false));

    // Serialize dictionary as an XML plist.
    let mut buf = Cursor::new(Vec::new());
    crate::to_writer_xml(&mut buf, &dict).unwrap();
    let buf = buf.into_inner();
    let xml = std::str::from_utf8(&buf).unwrap();

    let comparison = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>AnArray</key>
\t<array>
\t\t<string>Hello, world!</string>
\t\t<integer>345</integer>
\t</array>
\t<key>ADictionary</key>
\t<dict>
\t\t<key>FirstKey</key>
\t\t<string>FirstValue</string>
\t\t<key>SecondKey</key>
\t\t<data>
\t\tChQeKA==
\t\t</data>
\t\t<key>ThirdKey</key>
\t\t<real>1.234</real>
\t\t<key>FourthKey</key>
\t\t<date>1981-05-16T11:32:06Z</date>
\t</dict>
\t<key>AnInteger</key>
\t<integer>123</integer>
\t<key>ATrueBoolean</key>
\t<true/>
\t<key>AFalseBoolean</key>
\t<false/>
</dict>
</plist>";

    assert_eq!(xml, comparison);
}

#[test]
fn empty_array_and_dictionary_serialize_to_xml() {
    #[derive(Serialize, Default)]
    struct Empty {
        vec: Vec<String>,
        map: BTreeMap<String, String>,
    }

    // Serialize dictionary as an XML plist.
    let mut buf = Cursor::new(Vec::new());
    crate::to_writer_xml(&mut buf, &Empty::default()).unwrap();
    let buf = buf.into_inner();
    let xml = std::str::from_utf8(&buf).unwrap();

    let comparison = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>vec</key>
\t<array/>
\t<key>map</key>
\t<dict/>
</dict>
</plist>";

    assert_eq!(xml, comparison);
}

#[test]
fn serde_yaml_to_value() {
    let value: Value = serde_yaml::from_str("true").unwrap();
    assert_eq!(value, Value::Boolean(true));
}

#[test]
fn serialize_to_from_value() {
    let dog = Animal::Dog(DogOuter {
        inner: vec![DogInner {
            a: (),
            b: 12,
            c: vec!["a".to_string(), "b".to_string()],
            d: Some(Uid::new(42)),
            e: Data::new(vec![1, 2, 3]),
        }],
    });

    let dog_value = to_value(&dog).unwrap();

    assert_eq!(
        dog_value
            .as_dictionary()
            .unwrap()
            .get("Dog")
            .unwrap()
            .as_dictionary()
            .unwrap()
            .get("inner")
            .unwrap()
            .as_array()
            .unwrap()[0]
            .as_dictionary()
            .unwrap()["b"]
            .as_unsigned_integer()
            .unwrap(),
        12
    );

    let dog_roundtrip: Animal = from_value(&dog_value).unwrap();

    assert_eq!(dog_roundtrip, dog);
}

#[test]
fn deserialize_with_trailing_events_fails() {
    let events = [Event::String("Foo".into()), Event::String("Bar".into())];

    let value = crate::de::from_stream::<Value>(events.map(Ok));

    assert!(value.is_err());
}
