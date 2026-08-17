#![allow(
    clippy::decimal_literal_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::unreadable_literal,
    clippy::shadow_unrelated
)]

use indoc::indoc;
use serde::ser::SerializeMap;
use serde_derive::{Deserialize, Serialize};
use serde_yaml::{Mapping, Number, Value};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter;

fn test_serde<T>(thing: &T, yaml: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug,
{
    let serialized = serde_yaml::to_string(&thing).unwrap();
    assert_eq!(yaml, serialized);

    let value = serde_yaml::to_value(thing).unwrap();
    let serialized = serde_yaml::to_string(&value).unwrap();
    assert_eq!(yaml, serialized);

    let deserialized: T = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(*thing, deserialized);

    let value: Value = serde_yaml::from_str(yaml).unwrap();
    let deserialized = T::deserialize(&value).unwrap();
    assert_eq!(*thing, deserialized);

    let deserialized: T = serde_yaml::from_value(value).unwrap();
    assert_eq!(*thing, deserialized);

    serde_yaml::from_str::<serde::de::IgnoredAny>(yaml).unwrap();
}

#[test]
fn test_default() {
    assert_eq!(Value::default(), Value::Null);
}

#[test]
fn test_int() {
    let thing = 256;
    let yaml = indoc! {"
        256
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_int_max_u64() {
    let thing = u64::MAX;
    let yaml = indoc! {"
        18446744073709551615
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_int_min_i64() {
    let thing = i64::MIN;
    let yaml = indoc! {"
        -9223372036854775808
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_int_max_i64() {
    let thing = i64::MAX;
    let yaml = indoc! {"
        9223372036854775807
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_i128_small() {
    let thing: i128 = -256;
    let yaml = indoc! {"
        -256
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_u128_small() {
    let thing: u128 = 256;
    let yaml = indoc! {"
        256
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_float() {
    let thing = 25.6;
    let yaml = indoc! {"
        25.6
    "};
    test_serde(&thing, yaml);

    let thing = 25.;
    let yaml = indoc! {"
        25.0
    "};
    test_serde(&thing, yaml);

    let thing = f64::INFINITY;
    let yaml = indoc! {"
        .inf
    "};
    test_serde(&thing, yaml);

    let thing = f64::NEG_INFINITY;
    let yaml = indoc! {"
        -.inf
    "};
    test_serde(&thing, yaml);

    let float: f64 = serde_yaml::from_str(indoc! {"
        .nan
    "})
    .unwrap();
    assert!(float.is_nan());
}

#[test]
fn test_float32() {
    let thing: f32 = 25.5;
    let yaml = indoc! {"
        25.5
    "};
    test_serde(&thing, yaml);

    let thing = f32::INFINITY;
    let yaml = indoc! {"
        .inf
    "};
    test_serde(&thing, yaml);

    let thing = f32::NEG_INFINITY;
    let yaml = indoc! {"
        -.inf
    "};
    test_serde(&thing, yaml);

    let single_float: f32 = serde_yaml::from_str(indoc! {"
        .nan
    "})
    .unwrap();
    assert!(single_float.is_nan());
}

#[test]
fn test_char() {
    let ch = '.';
    let yaml = indoc! {"
        '.'
    "};
    assert_eq!(yaml, serde_yaml::to_string(&ch).unwrap());

    let ch = '#';
    let yaml = indoc! {"
        '#'
    "};
    assert_eq!(yaml, serde_yaml::to_string(&ch).unwrap());

    let ch = '-';
    let yaml = indoc! {"
        '-'
    "};
    assert_eq!(yaml, serde_yaml::to_string(&ch).unwrap());
}

#[test]
fn test_vec() {
    let thing = vec![1, 2, 3];
    let yaml = indoc! {"
        - 1
        - 2
        - 3
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_map() {
    let mut thing = BTreeMap::new();
    thing.insert("x".to_owned(), 1);
    thing.insert("y".to_owned(), 2);
    let yaml = indoc! {"
        x: 1
        y: 2
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_map_key_value() {
    struct Map;

    impl serde::Serialize for Map {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            // Test maps which do not serialize using serialize_entry.
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_key("k")?;
            map.serialize_value("v")?;
            map.end()
        }
    }

    let yaml = indoc! {"
        k: v
    "};
    assert_eq!(yaml, serde_yaml::to_string(&Map).unwrap());
}

#[test]
fn test_basic_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Basic {
        x: isize,
        y: String,
        z: bool,
    }
    let thing = Basic {
        x: -4,
        y: "hi\tquoted".to_owned(),
        z: true,
    };
    let yaml = indoc! {r#"
        x: -4
        y: "hi\tquoted"
        z: true
    "#};
    test_serde(&thing, yaml);
}

#[test]
fn test_string_escapes() {
    let yaml = indoc! {"
        ascii
    "};
    test_serde(&"ascii".to_owned(), yaml);

    let yaml = indoc! {r#"
        "\0\a\b\t\n\v\f\r\e\"\\\N\L\P"
    "#};
    test_serde(
        &"\0\u{7}\u{8}\t\n\u{b}\u{c}\r\u{1b}\"\\\u{85}\u{2028}\u{2029}".to_owned(),
        yaml,
    );

    let yaml = indoc! {r#"
        "\x1F\uFEFF"
    "#};
    test_serde(&"\u{1f}\u{feff}".to_owned(), yaml);

    let yaml = indoc! {"
        🎉
    "};
    test_serde(&"\u{1f389}".to_owned(), yaml);
}

#[test]
fn test_multiline_string() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Struct {
        trailing_newline: String,
        no_trailing_newline: String,
    }
    let thing = Struct {
        trailing_newline: "aaa\nbbb\n".to_owned(),
        no_trailing_newline: "aaa\nbbb".to_owned(),
    };
    let yaml = indoc! {"
        trailing_newline: |
          aaa
          bbb
        no_trailing_newline: |-
          aaa
          bbb
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_strings_needing_quote() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Struct {
        boolean: String,
        integer: String,
        void: String,
        leading_zeros: String,
    }
    let thing = Struct {
        boolean: "true".to_owned(),
        integer: "1".to_owned(),
        void: "null".to_owned(),
        leading_zeros: "007".to_owned(),
    };
    let yaml = indoc! {"
        boolean: 'true'
        integer: '1'
        void: 'null'
        leading_zeros: '007'
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_nested_vec() {
    let thing = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let yaml = indoc! {"
        - - 1
          - 2
          - 3
        - - 4
          - 5
          - 6
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_nested_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Inner {
        v: u16,
    }
    let thing = Outer {
        inner: Inner { v: 512 },
    };
    let yaml = indoc! {"
        inner:
          v: 512
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_nested_enum() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Outer {
        Inner(Inner),
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Inner {
        Unit,
    }
    let thing = Outer::Inner(Inner::Unit);
    let yaml = indoc! {"
        !Inner Unit
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_option() {
    let thing = vec![Some(1), None, Some(3)];
    let yaml = indoc! {"
        - 1
        - null
        - 3
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_unit() {
    let thing = vec![(), ()];
    let yaml = indoc! {"
        - null
        - null
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_unit_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Foo;
    let thing = Foo;
    let yaml = indoc! {"
        null
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_unit_variant() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Variant {
        First,
        Second,
    }
    let thing = Variant::First;
    let yaml = indoc! {"
        First
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_newtype_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct OriginalType {
        v: u16,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct NewType(OriginalType);
    let thing = NewType(OriginalType { v: 1 });
    let yaml = indoc! {"
        v: 1
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_newtype_variant() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Variant {
        Size(usize),
    }
    let thing = Variant::Size(127);
    let yaml = indoc! {"
        !Size 127
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_tuple_variant() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Variant {
        Rgb(u8, u8, u8),
    }
    let thing = Variant::Rgb(32, 64, 96);
    let yaml = indoc! {"
        !Rgb
        - 32
        - 64
        - 96
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_struct_variant() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Variant {
        Color { r: u8, g: u8, b: u8 },
    }
    let thing = Variant::Color {
        r: 32,
        g: 64,
        b: 96,
    };
    let yaml = indoc! {"
        !Color
        r: 32
        g: 64
        b: 96
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_tagged_map_value() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Bindings {
        profile: Profile,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Profile {
        ClassValidator { class_name: String },
    }
    let thing = Bindings {
        profile: Profile::ClassValidator {
            class_name: "ApplicationConfig".to_owned(),
        },
    };
    let yaml = indoc! {"
        profile: !ClassValidator
          class_name: ApplicationConfig
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_value() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct GenericInstructions {
        #[serde(rename = "type")]
        pub typ: String,
        pub config: Value,
    }
    let thing = GenericInstructions {
        typ: "primary".to_string(),
        config: Value::Sequence(vec![
            Value::Null,
            Value::Bool(true),
            Value::Number(Number::from(65535)),
            Value::Number(Number::from(0.54321)),
            Value::String("s".into()),
            Value::Mapping(Mapping::new()),
        ]),
    };
    let yaml = indoc! {"
        type: primary
        config:
        - null
        - true
        - 65535
        - 0.54321
        - s
        - {}
    "};
    test_serde(&thing, yaml);
}

#[test]
fn test_mapping() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        pub substructure: Mapping,
    }

    let mut thing = Data {
        substructure: Mapping::new(),
    };
    thing.substructure.insert(
        Value::String("a".to_owned()),
        Value::String("foo".to_owned()),
    );
    thing.substructure.insert(
        Value::String("b".to_owned()),
        Value::String("bar".to_owned()),
    );

    let yaml = indoc! {"
        substructure:
          a: foo
          b: bar
    "};

    test_serde(&thing, yaml);
}

#[test]
fn test_long_string() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        pub string: String,
    }

    let thing = Data {
        string: iter::repeat(["word", " "]).flatten().take(69).collect(),
    };

    let yaml = indoc! {"
        string: word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word word
    "};

    test_serde(&thing, yaml);
}
