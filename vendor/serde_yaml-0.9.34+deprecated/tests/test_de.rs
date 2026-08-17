#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::derive_partial_eq_without_eq,
    clippy::similar_names,
    clippy::uninlined_format_args
)]

use indoc::indoc;
use serde_derive::Deserialize;
use serde_yaml::{Deserializer, Number, Value};
use std::collections::BTreeMap;
use std::fmt::Debug;

fn test_de<T>(yaml: &str, expected: &T)
where
    T: serde::de::DeserializeOwned + PartialEq + Debug,
{
    let deserialized: T = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(*expected, deserialized);

    let value: Value = serde_yaml::from_str(yaml).unwrap();
    let deserialized = T::deserialize(&value).unwrap();
    assert_eq!(*expected, deserialized);

    let deserialized: T = serde_yaml::from_value(value).unwrap();
    assert_eq!(*expected, deserialized);

    serde_yaml::from_str::<serde::de::IgnoredAny>(yaml).unwrap();

    let mut deserializer = Deserializer::from_str(yaml);
    let document = deserializer.next().unwrap();
    let deserialized = T::deserialize(document).unwrap();
    assert_eq!(*expected, deserialized);
    assert!(deserializer.next().is_none());
}

fn test_de_no_value<'de, T>(yaml: &'de str, expected: &T)
where
    T: serde::de::Deserialize<'de> + PartialEq + Debug,
{
    let deserialized: T = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(*expected, deserialized);

    serde_yaml::from_str::<serde_yaml::Value>(yaml).unwrap();
    serde_yaml::from_str::<serde::de::IgnoredAny>(yaml).unwrap();
}

fn test_de_seed<'de, T, S>(yaml: &'de str, seed: S, expected: &T)
where
    T: PartialEq + Debug,
    S: serde::de::DeserializeSeed<'de, Value = T>,
{
    let deserialized: T = seed.deserialize(Deserializer::from_str(yaml)).unwrap();
    assert_eq!(*expected, deserialized);

    serde_yaml::from_str::<serde_yaml::Value>(yaml).unwrap();
    serde_yaml::from_str::<serde::de::IgnoredAny>(yaml).unwrap();
}

#[test]
fn test_borrowed() {
    let yaml = indoc! {"
        - plain nonàscii
        - 'single quoted'
        - \"double quoted\"
    "};
    let expected = vec!["plain nonàscii", "single quoted", "double quoted"];
    test_de_no_value(yaml, &expected);
}

#[test]
fn test_alias() {
    let yaml = indoc! {"
        first:
          &alias
          1
        second:
          *alias
        third: 3
    "};
    let mut expected = BTreeMap::new();
    expected.insert("first".to_owned(), 1);
    expected.insert("second".to_owned(), 1);
    expected.insert("third".to_owned(), 3);
    test_de(yaml, &expected);
}

#[test]
fn test_option() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Data {
        a: Option<f64>,
        b: Option<String>,
        c: Option<bool>,
    }
    let yaml = indoc! {"
        b:
        c: true
    "};
    let expected = Data {
        a: None,
        b: None,
        c: Some(true),
    };
    test_de(yaml, &expected);
}

#[test]
fn test_option_alias() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Data {
        a: Option<f64>,
        b: Option<String>,
        c: Option<bool>,
        d: Option<f64>,
        e: Option<String>,
        f: Option<bool>,
    }
    let yaml = indoc! {"
        none_f:
          &none_f
          ~
        none_s:
          &none_s
          ~
        none_b:
          &none_b
          ~

        some_f:
          &some_f
          1.0
        some_s:
          &some_s
          x
        some_b:
          &some_b
          true

        a: *none_f
        b: *none_s
        c: *none_b
        d: *some_f
        e: *some_s
        f: *some_b
    "};
    let expected = Data {
        a: None,
        b: None,
        c: None,
        d: Some(1.0),
        e: Some("x".to_owned()),
        f: Some(true),
    };
    test_de(yaml, &expected);
}

#[test]
fn test_enum_alias() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        A,
        B(u8, u8),
    }
    #[derive(Deserialize, PartialEq, Debug)]
    struct Data {
        a: E,
        b: E,
    }
    let yaml = indoc! {"
        aref:
          &aref
          A
        bref:
          &bref
          !B
            - 1
            - 2

        a: *aref
        b: *bref
    "};
    let expected = Data {
        a: E::A,
        b: E::B(1, 2),
    };
    test_de(yaml, &expected);
}

#[test]
fn test_enum_representations() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum Enum {
        Unit,
        Tuple(i32, i32),
        Struct { x: i32, y: i32 },
        String(String),
        Number(f64),
    }

    let yaml = indoc! {"
        - Unit
        - 'Unit'
        - !Unit
        - !Unit ~
        - !Unit null
        - !Tuple [0, 0]
        - !Tuple
          - 0
          - 0
        - !Struct {x: 0, y: 0}
        - !Struct
          x: 0
          y: 0
        - !String '...'
        - !String ...
        - !Number 0
    "};

    let expected = vec![
        Enum::Unit,
        Enum::Unit,
        Enum::Unit,
        Enum::Unit,
        Enum::Unit,
        Enum::Tuple(0, 0),
        Enum::Tuple(0, 0),
        Enum::Struct { x: 0, y: 0 },
        Enum::Struct { x: 0, y: 0 },
        Enum::String("...".to_owned()),
        Enum::String("...".to_owned()),
        Enum::Number(0.0),
    ];

    test_de(yaml, &expected);

    let yaml = indoc! {"
        - !String
    "};
    let expected = vec![Enum::String(String::new())];
    test_de_no_value(yaml, &expected);
}

#[test]
fn test_number_as_string() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Num {
        value: String,
    }
    let yaml = indoc! {"
        # Cannot be represented as u128
        value: 340282366920938463463374607431768211457
    "};
    let expected = Num {
        value: "340282366920938463463374607431768211457".to_owned(),
    };
    test_de_no_value(yaml, &expected);
}

#[test]
fn test_empty_string() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Struct {
        empty: String,
        tilde: String,
    }
    let yaml = indoc! {"
        empty:
        tilde: ~
    "};
    let expected = Struct {
        empty: String::new(),
        tilde: "~".to_owned(),
    };
    test_de_no_value(yaml, &expected);
}

#[test]
fn test_i128_big() {
    let expected: i128 = i64::MIN as i128 - 1;
    let yaml = indoc! {"
        -9223372036854775809
    "};
    assert_eq!(expected, serde_yaml::from_str::<i128>(yaml).unwrap());

    let octal = indoc! {"
        -0o1000000000000000000001
    "};
    assert_eq!(expected, serde_yaml::from_str::<i128>(octal).unwrap());
}

#[test]
fn test_u128_big() {
    let expected: u128 = u64::MAX as u128 + 1;
    let yaml = indoc! {"
        18446744073709551616
    "};
    assert_eq!(expected, serde_yaml::from_str::<u128>(yaml).unwrap());

    let octal = indoc! {"
        0o2000000000000000000000
    "};
    assert_eq!(expected, serde_yaml::from_str::<u128>(octal).unwrap());
}

#[test]
fn test_number_alias_as_string() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Num {
        version: String,
        value: String,
    }
    let yaml = indoc! {"
        version: &a 1.10
        value: *a
    "};
    let expected = Num {
        version: "1.10".to_owned(),
        value: "1.10".to_owned(),
    };
    test_de_no_value(yaml, &expected);
}

#[test]
fn test_de_mapping() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Data {
        pub substructure: serde_yaml::Mapping,
    }
    let yaml = indoc! {"
        substructure:
          a: 'foo'
          b: 'bar'
    "};

    let mut expected = Data {
        substructure: serde_yaml::Mapping::new(),
    };
    expected.substructure.insert(
        serde_yaml::Value::String("a".to_owned()),
        serde_yaml::Value::String("foo".to_owned()),
    );
    expected.substructure.insert(
        serde_yaml::Value::String("b".to_owned()),
        serde_yaml::Value::String("bar".to_owned()),
    );

    test_de(yaml, &expected);
}

#[test]
fn test_byte_order_mark() {
    let yaml = "\u{feff}- 0\n";
    let expected = vec![0];
    test_de(yaml, &expected);
}

#[test]
fn test_bomb() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Data {
        expected: String,
    }

    // This would deserialize an astronomical number of elements if we were
    // vulnerable.
    let yaml = indoc! {"
        a: &a ~
        b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a]
        c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b]
        d: &d [*c,*c,*c,*c,*c,*c,*c,*c,*c]
        e: &e [*d,*d,*d,*d,*d,*d,*d,*d,*d]
        f: &f [*e,*e,*e,*e,*e,*e,*e,*e,*e]
        g: &g [*f,*f,*f,*f,*f,*f,*f,*f,*f]
        h: &h [*g,*g,*g,*g,*g,*g,*g,*g,*g]
        i: &i [*h,*h,*h,*h,*h,*h,*h,*h,*h]
        j: &j [*i,*i,*i,*i,*i,*i,*i,*i,*i]
        k: &k [*j,*j,*j,*j,*j,*j,*j,*j,*j]
        l: &l [*k,*k,*k,*k,*k,*k,*k,*k,*k]
        m: &m [*l,*l,*l,*l,*l,*l,*l,*l,*l]
        n: &n [*m,*m,*m,*m,*m,*m,*m,*m,*m]
        o: &o [*n,*n,*n,*n,*n,*n,*n,*n,*n]
        p: &p [*o,*o,*o,*o,*o,*o,*o,*o,*o]
        q: &q [*p,*p,*p,*p,*p,*p,*p,*p,*p]
        r: &r [*q,*q,*q,*q,*q,*q,*q,*q,*q]
        s: &s [*r,*r,*r,*r,*r,*r,*r,*r,*r]
        t: &t [*s,*s,*s,*s,*s,*s,*s,*s,*s]
        u: &u [*t,*t,*t,*t,*t,*t,*t,*t,*t]
        v: &v [*u,*u,*u,*u,*u,*u,*u,*u,*u]
        w: &w [*v,*v,*v,*v,*v,*v,*v,*v,*v]
        x: &x [*w,*w,*w,*w,*w,*w,*w,*w,*w]
        y: &y [*x,*x,*x,*x,*x,*x,*x,*x,*x]
        z: &z [*y,*y,*y,*y,*y,*y,*y,*y,*y]
        expected: string
    "};

    let expected = Data {
        expected: "string".to_owned(),
    };

    assert_eq!(expected, serde_yaml::from_str::<Data>(yaml).unwrap());
}

#[test]
fn test_numbers() {
    let cases = [
        ("0xF0", "240"),
        ("+0xF0", "240"),
        ("-0xF0", "-240"),
        ("0o70", "56"),
        ("+0o70", "56"),
        ("-0o70", "-56"),
        ("0b10", "2"),
        ("+0b10", "2"),
        ("-0b10", "-2"),
        ("127", "127"),
        ("+127", "127"),
        ("-127", "-127"),
        (".inf", ".inf"),
        (".Inf", ".inf"),
        (".INF", ".inf"),
        ("-.inf", "-.inf"),
        ("-.Inf", "-.inf"),
        ("-.INF", "-.inf"),
        (".nan", ".nan"),
        (".NaN", ".nan"),
        (".NAN", ".nan"),
        ("0.1", "0.1"),
    ];
    for &(yaml, expected) in &cases {
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        match value {
            Value::Number(number) => assert_eq!(number.to_string(), expected),
            _ => panic!("expected number. input={:?}, result={:?}", yaml, value),
        }
    }

    // NOT numbers.
    let cases = [
        "0127", "+0127", "-0127", "++.inf", "+-.inf", "++1", "+-1", "-+1", "--1", "0x+1", "0x-1",
        "-0x+1", "-0x-1", "++0x1", "+-0x1", "-+0x1", "--0x1",
    ];
    for yaml in &cases {
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        match value {
            Value::String(string) => assert_eq!(string, *yaml),
            _ => panic!("expected string. input={:?}, result={:?}", yaml, value),
        }
    }
}

#[test]
fn test_nan() {
    // There is no negative NaN in YAML.
    assert!(serde_yaml::from_str::<f32>(".nan")
        .unwrap()
        .is_sign_positive());
    assert!(serde_yaml::from_str::<f64>(".nan")
        .unwrap()
        .is_sign_positive());
}

#[test]
fn test_stateful() {
    struct Seed(i64);

    impl<'de> serde::de::DeserializeSeed<'de> for Seed {
        type Value = i64;
        fn deserialize<D>(self, deserializer: D) -> Result<i64, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor(i64);
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = i64;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "an integer")
                }

                fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<i64, E> {
                    Ok(v * self.0)
                }

                fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<i64, E> {
                    Ok(v as i64 * self.0)
                }
            }

            deserializer.deserialize_any(Visitor(self.0))
        }
    }

    let cases = [("3", 5, 15), ("6", 7, 42), ("-5", 9, -45)];
    for &(yaml, seed, expected) in &cases {
        test_de_seed(yaml, Seed(seed), &expected);
    }
}

#[test]
fn test_ignore_tag() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        struc: Struc,
        tuple: Tuple,
        newtype: Newtype,
        map: BTreeMap<char, usize>,
        vec: Vec<usize>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Struc {
        x: usize,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Tuple(usize, usize);

    #[derive(Deserialize, Debug, PartialEq)]
    struct Newtype(usize);

    let yaml = indoc! {"
        struc: !wat
          x: 0
        tuple: !wat
          - 0
          - 0
        newtype: !wat 0
        map: !wat
          x: 0
        vec: !wat
          - 0
    "};

    let expected = Data {
        struc: Struc { x: 0 },
        tuple: Tuple(0, 0),
        newtype: Newtype(0),
        map: {
            let mut map = BTreeMap::new();
            map.insert('x', 0);
            map
        },
        vec: vec![0],
    };

    test_de(yaml, &expected);
}

#[test]
fn test_no_required_fields() {
    #[derive(Deserialize, PartialEq, Debug)]
    pub struct NoRequiredFields {
        optional: Option<usize>,
    }

    for document in ["", "# comment\n"] {
        let expected = NoRequiredFields { optional: None };
        let deserialized: NoRequiredFields = serde_yaml::from_str(document).unwrap();
        assert_eq!(expected, deserialized);

        let expected = Vec::<String>::new();
        let deserialized: Vec<String> = serde_yaml::from_str(document).unwrap();
        assert_eq!(expected, deserialized);

        let expected = BTreeMap::new();
        let deserialized: BTreeMap<char, usize> = serde_yaml::from_str(document).unwrap();
        assert_eq!(expected, deserialized);

        let expected = None;
        let deserialized: Option<String> = serde_yaml::from_str(document).unwrap();
        assert_eq!(expected, deserialized);

        let expected = Value::Null;
        let deserialized: Value = serde_yaml::from_str(document).unwrap();
        assert_eq!(expected, deserialized);
    }
}

#[test]
fn test_empty_scalar() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Struct<T> {
        thing: T,
    }

    let yaml = "thing:\n";
    let expected = Struct {
        thing: serde_yaml::Sequence::new(),
    };
    test_de(yaml, &expected);

    let expected = Struct {
        thing: serde_yaml::Mapping::new(),
    };
    test_de(yaml, &expected);
}

#[test]
fn test_python_safe_dump() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Frob {
        foo: u32,
    }

    // This matches output produced by PyYAML's `yaml.safe_dump` when using the
    // default_style parameter.
    //
    //    >>> import yaml
    //    >>> d = {"foo": 7200}
    //    >>> print(yaml.safe_dump(d, default_style="|"))
    //    "foo": !!int |-
    //      7200
    //
    let yaml = indoc! {r#"
        "foo": !!int |-
            7200
    "#};

    let expected = Frob { foo: 7200 };
    test_de(yaml, &expected);
}

#[test]
fn test_tag_resolution() {
    // https://yaml.org/spec/1.2.2/#1032-tag-resolution
    let yaml = indoc! {"
        - null
        - Null
        - NULL
        - ~
        -
        - true
        - True
        - TRUE
        - false
        - False
        - FALSE
        - y
        - Y
        - yes
        - Yes
        - YES
        - n
        - N
        - no
        - No
        - NO
        - on
        - On
        - ON
        - off
        - Off
        - OFF
    "};

    let expected = vec![
        Value::Null,
        Value::Null,
        Value::Null,
        Value::Null,
        Value::Null,
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(false),
        Value::Bool(false),
        Value::String("y".to_owned()),
        Value::String("Y".to_owned()),
        Value::String("yes".to_owned()),
        Value::String("Yes".to_owned()),
        Value::String("YES".to_owned()),
        Value::String("n".to_owned()),
        Value::String("N".to_owned()),
        Value::String("no".to_owned()),
        Value::String("No".to_owned()),
        Value::String("NO".to_owned()),
        Value::String("on".to_owned()),
        Value::String("On".to_owned()),
        Value::String("ON".to_owned()),
        Value::String("off".to_owned()),
        Value::String("Off".to_owned()),
        Value::String("OFF".to_owned()),
    ];

    test_de(yaml, &expected);
}

#[test]
fn test_parse_number() {
    let n = "111".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(111));

    let n = "-111".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(-111));

    let n = "-1.1".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(-1.1));

    let n = ".nan".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(f64::NAN));
    assert!(n.as_f64().unwrap().is_sign_positive());

    let n = ".inf".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(f64::INFINITY));

    let n = "-.inf".parse::<Number>().unwrap();
    assert_eq!(n, Number::from(f64::NEG_INFINITY));

    let err = "null".parse::<Number>().unwrap_err();
    assert_eq!(err.to_string(), "failed to parse YAML number");

    let err = " 1 ".parse::<Number>().unwrap_err();
    assert_eq!(err.to_string(), "failed to parse YAML number");
}
