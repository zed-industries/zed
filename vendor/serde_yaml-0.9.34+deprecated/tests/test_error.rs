#![allow(clippy::zero_sized_map_values)]

use indoc::indoc;
use serde::de::Deserialize;
#[cfg(not(miri))]
use serde::de::{SeqAccess, Visitor};
use serde_derive::{Deserialize, Serialize};
use serde_yaml::value::{Tag, TaggedValue};
use serde_yaml::{Deserializer, Value};
#[cfg(not(miri))]
use std::collections::BTreeMap;
#[cfg(not(miri))]
use std::fmt;
use std::fmt::Debug;

fn test_error<'de, T>(yaml: &'de str, expected: &str)
where
    T: Deserialize<'de> + Debug,
{
    let result = serde_yaml::from_str::<T>(yaml);
    assert_eq!(expected, result.unwrap_err().to_string());

    let mut deserializer = Deserializer::from_str(yaml);
    if let Some(first_document) = deserializer.next() {
        if deserializer.next().is_none() {
            let result = T::deserialize(first_document);
            assert_eq!(expected, result.unwrap_err().to_string());
        }
    }
}

#[test]
fn test_scan_error() {
    let yaml = ">\n@";
    let expected = "found character that cannot start any token at line 2 column 1, while scanning for the next token";
    test_error::<Value>(yaml, expected);
}

#[test]
fn test_incorrect_type() {
    let yaml = indoc! {"
        ---
        str
    "};
    let expected = "invalid type: string \"str\", expected i16 at line 2 column 1";
    test_error::<i16>(yaml, expected);
}

#[test]
fn test_incorrect_nested_type() {
    #[derive(Deserialize, Debug)]
    pub struct A {
        #[allow(dead_code)]
        pub b: Vec<B>,
    }
    #[derive(Deserialize, Debug)]
    pub enum B {
        C(#[allow(dead_code)] C),
    }
    #[derive(Deserialize, Debug)]
    pub struct C {
        #[allow(dead_code)]
        pub d: bool,
    }
    let yaml = indoc! {"
        b:
          - !C
            d: fase
    "};
    let expected = "b[0].d: invalid type: string \"fase\", expected a boolean at line 3 column 8";
    test_error::<A>(yaml, expected);
}

#[test]
fn test_empty() {
    let expected = "EOF while parsing a value";
    test_error::<String>("", expected);
}

#[test]
fn test_missing_field() {
    #[derive(Deserialize, Debug)]
    pub struct Basic {
        #[allow(dead_code)]
        pub v: bool,
        #[allow(dead_code)]
        pub w: bool,
    }
    let yaml = indoc! {"
        ---
        v: true
    "};
    let expected = "missing field `w` at line 2 column 1";
    test_error::<Basic>(yaml, expected);
}

#[test]
fn test_unknown_anchor() {
    let yaml = indoc! {"
        ---
        *some
    "};
    let expected = "unknown anchor at line 2 column 1";
    test_error::<String>(yaml, expected);
}

#[test]
fn test_ignored_unknown_anchor() {
    #[derive(Deserialize, Debug)]
    pub struct Wrapper {
        #[allow(dead_code)]
        pub c: (),
    }
    let yaml = indoc! {"
        b: [*a]
        c: ~
    "};
    let expected = "unknown anchor at line 1 column 5";
    test_error::<Wrapper>(yaml, expected);
}

#[test]
fn test_bytes() {
    let expected = "serialization and deserialization of bytes in YAML is not implemented";
    test_error::<&[u8]>("...", expected);
}

#[test]
fn test_two_documents() {
    let yaml = indoc! {"
        ---
        0
        ---
        1
    "};
    let expected = "deserializing from YAML containing more than one document is not supported";
    test_error::<usize>(yaml, expected);
}

#[test]
fn test_second_document_syntax_error() {
    let yaml = indoc! {"
        ---
        0
        ---
        ]
    "};

    let mut de = Deserializer::from_str(yaml);
    let first_doc = de.next().unwrap();
    let result = <usize as serde::Deserialize>::deserialize(first_doc);
    assert_eq!(0, result.unwrap());

    let second_doc = de.next().unwrap();
    let result = <usize as serde::Deserialize>::deserialize(second_doc);
    let expected =
        "did not find expected node content at line 4 column 1, while parsing a block node";
    assert_eq!(expected, result.unwrap_err().to_string());
}

#[test]
fn test_missing_enum_tag() {
    #[derive(Deserialize, Debug)]
    pub enum E {
        V(#[allow(dead_code)] usize),
    }
    let yaml = indoc! {r#"
        "V": 16
        "other": 32
    "#};
    let expected = "invalid type: map, expected a YAML tag starting with '!'";
    test_error::<E>(yaml, expected);
}

#[test]
fn test_serialize_nested_enum() {
    #[derive(Serialize, Debug)]
    pub enum Outer {
        Inner(Inner),
    }
    #[derive(Serialize, Debug)]
    pub enum Inner {
        Newtype(usize),
        Tuple(usize, usize),
        Struct { x: usize },
    }

    let expected = "serializing nested enums in YAML is not supported yet";

    let e = Outer::Inner(Inner::Newtype(0));
    let error = serde_yaml::to_string(&e).unwrap_err();
    assert_eq!(error.to_string(), expected);

    let e = Outer::Inner(Inner::Tuple(0, 0));
    let error = serde_yaml::to_string(&e).unwrap_err();
    assert_eq!(error.to_string(), expected);

    let e = Outer::Inner(Inner::Struct { x: 0 });
    let error = serde_yaml::to_string(&e).unwrap_err();
    assert_eq!(error.to_string(), expected);

    let e = Value::Tagged(Box::new(TaggedValue {
        tag: Tag::new("Outer"),
        value: Value::Tagged(Box::new(TaggedValue {
            tag: Tag::new("Inner"),
            value: Value::Null,
        })),
    }));
    let error = serde_yaml::to_string(&e).unwrap_err();
    assert_eq!(error.to_string(), expected);
}

#[test]
fn test_deserialize_nested_enum() {
    #[derive(Deserialize, Debug)]
    pub enum Outer {
        Inner(#[allow(dead_code)] Inner),
    }
    #[derive(Deserialize, Debug)]
    pub enum Inner {
        Variant(#[allow(dead_code)] Vec<usize>),
    }

    let yaml = indoc! {"
        ---
        !Inner []
    "};
    let expected = "deserializing nested enum in Outer::Inner from YAML is not supported yet at line 2 column 1";
    test_error::<Outer>(yaml, expected);

    let yaml = indoc! {"
        ---
        !Variant []
    "};
    let expected = "unknown variant `Variant`, expected `Inner`";
    test_error::<Outer>(yaml, expected);

    let yaml = indoc! {"
        ---
        !Inner !Variant []
    "};
    let expected = "deserializing nested enum in Outer::Inner from YAML is not supported yet at line 2 column 1";
    test_error::<Outer>(yaml, expected);
}

#[test]
fn test_variant_not_a_seq() {
    #[derive(Deserialize, Debug)]
    pub enum E {
        V(#[allow(dead_code)] usize),
    }
    let yaml = indoc! {"
        ---
        !V
        value: 0
    "};
    let expected = "invalid type: map, expected usize at line 2 column 1";
    test_error::<E>(yaml, expected);
}

#[test]
fn test_struct_from_sequence() {
    #[derive(Deserialize, Debug)]
    pub struct Struct {
        #[allow(dead_code)]
        pub x: usize,
        #[allow(dead_code)]
        pub y: usize,
    }
    let yaml = indoc! {"
        [0, 0]
    "};
    let expected = "invalid type: sequence, expected struct Struct";
    test_error::<Struct>(yaml, expected);
}

#[test]
fn test_bad_bool() {
    let yaml = indoc! {"
        ---
        !!bool str
    "};
    let expected = "invalid value: string \"str\", expected a boolean at line 2 column 1";
    test_error::<bool>(yaml, expected);
}

#[test]
fn test_bad_int() {
    let yaml = indoc! {"
        ---
        !!int str
    "};
    let expected = "invalid value: string \"str\", expected an integer at line 2 column 1";
    test_error::<i64>(yaml, expected);
}

#[test]
fn test_bad_float() {
    let yaml = indoc! {"
        ---
        !!float str
    "};
    let expected = "invalid value: string \"str\", expected a float at line 2 column 1";
    test_error::<f64>(yaml, expected);
}

#[test]
fn test_bad_null() {
    let yaml = indoc! {"
        ---
        !!null str
    "};
    let expected = "invalid value: string \"str\", expected null at line 2 column 1";
    test_error::<()>(yaml, expected);
}

#[test]
fn test_short_tuple() {
    let yaml = indoc! {"
        ---
        [0, 0]
    "};
    let expected = "invalid length 2, expected a tuple of size 3 at line 2 column 1";
    test_error::<(u8, u8, u8)>(yaml, expected);
}

#[test]
fn test_long_tuple() {
    let yaml = indoc! {"
        ---
        [0, 0, 0]
    "};
    let expected = "invalid length 3, expected sequence of 2 elements at line 2 column 1";
    test_error::<(u8, u8)>(yaml, expected);
}

#[test]
fn test_invalid_scalar_type() {
    #[derive(Deserialize, Debug)]
    pub struct S {
        #[allow(dead_code)]
        pub x: [i32; 1],
    }

    let yaml = "x: ''\n";
    let expected = "x: invalid type: string \"\", expected an array of length 1 at line 1 column 4";
    test_error::<S>(yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_infinite_recursion_objects() {
    #[derive(Deserialize, Debug)]
    pub struct S {
        #[allow(dead_code)]
        pub x: Option<Box<S>>,
    }

    let yaml = "&a {'x': *a}";
    let expected = "recursion limit exceeded";
    test_error::<S>(yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_infinite_recursion_arrays() {
    #[derive(Deserialize, Debug)]
    pub struct S(
        #[allow(dead_code)] pub usize,
        #[allow(dead_code)] pub Option<Box<S>>,
    );

    let yaml = "&a [0, *a]";
    let expected = "recursion limit exceeded";
    test_error::<S>(yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_infinite_recursion_newtype() {
    #[derive(Deserialize, Debug)]
    pub struct S(#[allow(dead_code)] pub Option<Box<S>>);

    let yaml = "&a [*a]";
    let expected = "recursion limit exceeded";
    test_error::<S>(yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_finite_recursion_objects() {
    #[derive(Deserialize, Debug)]
    pub struct S {
        #[allow(dead_code)]
        pub x: Option<Box<S>>,
    }

    let yaml = "{'x':".repeat(1_000) + &"}".repeat(1_000);
    let expected = "recursion limit exceeded at line 1 column 641";
    test_error::<S>(&yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_finite_recursion_arrays() {
    #[derive(Deserialize, Debug)]
    pub struct S(
        #[allow(dead_code)] pub usize,
        #[allow(dead_code)] pub Option<Box<S>>,
    );

    let yaml = "[0, ".repeat(1_000) + &"]".repeat(1_000);
    let expected = "recursion limit exceeded at line 1 column 513";
    test_error::<S>(&yaml, expected);
}

#[cfg(not(miri))]
#[test]
fn test_billion_laughs() {
    #[derive(Debug)]
    struct X;

    impl<'de> Visitor<'de> for X {
        type Value = X;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("exponential blowup")
        }

        fn visit_unit<E>(self) -> Result<X, E> {
            Ok(X)
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<X, S::Error>
        where
            S: SeqAccess<'de>,
        {
            while let Some(X) = seq.next_element()? {}
            Ok(X)
        }
    }

    impl<'de> Deserialize<'de> for X {
        fn deserialize<D>(deserializer: D) -> Result<X, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(X)
        }
    }

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
    "};
    let expected = "repetition limit exceeded";
    test_error::<BTreeMap<String, X>>(yaml, expected);
}

#[test]
fn test_duplicate_keys() {
    let yaml = indoc! {"
        ---
        thing: true
        thing: false
    "};
    let expected = "duplicate entry with key \"thing\" at line 2 column 1";
    test_error::<Value>(yaml, expected);

    let yaml = indoc! {"
        ---
        null: true
        ~: false
    "};
    let expected = "duplicate entry with null key at line 2 column 1";
    test_error::<Value>(yaml, expected);

    let yaml = indoc! {"
        ---
        99: true
        99: false
    "};
    let expected = "duplicate entry with key 99 at line 2 column 1";
    test_error::<Value>(yaml, expected);

    let yaml = indoc! {"
        ---
        {}: true
        {}: false
    "};
    let expected = "duplicate entry in YAML map at line 2 column 1";
    test_error::<Value>(yaml, expected);
}
