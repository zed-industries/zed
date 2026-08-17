use serde::de;
use serde_derive::Deserialize;

use std::collections::HashMap;
use std::fmt;

mod common;

use crate::common::{
    deserializes_to, deserializes_to_nan_f32, deserializes_to_nan_f64, deserializes_with_error,
    make_error,
};

/// Defines a struct `A` with a `de::Deserializer` implementation that returns an error. Works for
/// visitors that accept a single value.
macro_rules! error_struct {
    ($type:ty, $visit_fn:ident, $deserialize_fn:ident) => {
        #[derive(Debug, PartialEq)]
        struct A;
        impl<'de> de::Deserialize<'de> for A {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = A;
                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("...")
                    }
                    fn $visit_fn<E>(self, _v: $type) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("oops"))
                    }
                }
                deserializer.$deserialize_fn(Visitor)
            }
        }
    };
}

#[test]
fn deserializes_bool() {
    deserializes_to("true", true);
    deserializes_to("false", false);

    error_struct!(bool, visit_bool, deserialize_bool);
    deserializes_with_error::<A>("\n true", make_error("oops", 2, 2));
}

#[test]
fn deserializes_i8() {
    let x: i8 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);
    deserializes_to("-42", -x);
    deserializes_to("-42.", -x);
    deserializes_to("-42.0", -x);
    deserializes_to("-42e0", -x);
    deserializes_to("-4.2e1", -x);
    deserializes_to("-.42e2", -x);
    deserializes_to("-0.42e2", -x);

    error_struct!(i8, visit_i8, deserialize_i8);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_u8() {
    let x: u8 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);

    error_struct!(u8, visit_u8, deserialize_u8);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_i16() {
    let x: i16 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);
    deserializes_to("-42", -x);
    deserializes_to("-42.", -x);
    deserializes_to("-42.0", -x);
    deserializes_to("-42e0", -x);
    deserializes_to("-4.2e1", -x);
    deserializes_to("-.42e2", -x);
    deserializes_to("-0.42e2", -x);

    error_struct!(i16, visit_i16, deserialize_i16);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_u16() {
    let x: u16 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);

    error_struct!(u16, visit_u16, deserialize_u16);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_i32() {
    let x: i32 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);
    deserializes_to("-42", -x);
    deserializes_to("-42.", -x);
    deserializes_to("-42.0", -x);
    deserializes_to("-42e0", -x);
    deserializes_to("-4.2e1", -x);
    deserializes_to("-.42e2", -x);
    deserializes_to("-0.42e2", -x);

    error_struct!(i32, visit_i32, deserialize_i32);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_u32() {
    let x: u32 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);

    error_struct!(u32, visit_u32, deserialize_u32);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_i64() {
    let x: i64 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);
    deserializes_to("-42", -x);
    deserializes_to("-42.", -x);
    deserializes_to("-42.0", -x);
    deserializes_to("-42e0", -x);
    deserializes_to("-4.2e1", -x);
    deserializes_to("-.42e2", -x);
    deserializes_to("-0.42e2", -x);

    error_struct!(i64, visit_i64, deserialize_i64);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
    let over_i64 = format!("\n {}0", i64::max_value());
    deserializes_with_error::<serde_json::Value>(
        over_i64.as_str(),
        make_error("error parsing integer", 2, 2),
    );
}

#[test]
fn deserializes_u64() {
    let x: u64 = 42;
    deserializes_to("0x2A", x);
    deserializes_to("0x2a", x);
    deserializes_to("0X2A", x);
    deserializes_to("0X2a", x);
    deserializes_to("0x00002A", x);
    deserializes_to("42", x);
    deserializes_to("42.", x);
    deserializes_to("42.0", x);
    deserializes_to("42e0", x);
    deserializes_to("4.2e1", x);
    deserializes_to(".42e2", x);
    deserializes_to("0.42e2", x);

    deserializes_to("Infinity", std::f32::INFINITY);
    deserializes_to("-Infinity", std::f32::NEG_INFINITY);
    deserializes_to_nan_f32("NaN");

    error_struct!(u64, visit_u64, deserialize_u64);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_f32() {
    let x: f32 = 42.42;
    deserializes_to("42.42", x);
    deserializes_to("42.42e0", x);
    deserializes_to("4.242e1", x);
    deserializes_to(".4242e2", x);
    deserializes_to("0.4242e2", x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42e0", -x);
    deserializes_to("-4.242e1", -x);
    deserializes_to("-.4242e2", -x);
    deserializes_to("-0.4242e2", -x);

    deserializes_to("Infinity", std::f32::INFINITY);
    deserializes_to("-Infinity", std::f32::NEG_INFINITY);
    deserializes_to_nan_f32("NaN");
    deserializes_to_nan_f32("-NaN");

    error_struct!(f32, visit_f32, deserialize_f32);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_f64() {
    let x: f64 = 42.42;
    deserializes_to("42.42", x);
    deserializes_to("42.42e0", x);
    deserializes_to("4.242e1", x);
    deserializes_to(".4242e2", x);
    deserializes_to("0.4242e2", x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42", -x);
    deserializes_to("-42.42e0", -x);
    deserializes_to("-4.242e1", -x);
    deserializes_to("-.4242e2", -x);
    deserializes_to("-0.4242e2", -x);

    deserializes_to("Infinity", std::f64::INFINITY);
    deserializes_to("-Infinity", std::f64::NEG_INFINITY);
    deserializes_to_nan_f64("NaN");
    deserializes_to_nan_f64("-NaN");

    error_struct!(f64, visit_f64, deserialize_f64);
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
    deserializes_with_error::<f64>(
        "\n 1e309",
        make_error("error parsing number: too large", 2, 2),
    );
}

#[test]
fn deserializes_char() {
    deserializes_to("'x'", 'x');
    deserializes_to("\"자\"", '자');
    deserializes_to(r#""\"""#, '"');
    deserializes_to(r#""\r""#, '\r');
    deserializes_to(r#""\n""#, '\n');
    deserializes_to(r#""\t""#, '\t');
    deserializes_to(r#""\\""#, '\\');
    deserializes_to(r#""\/""#, '/');
    deserializes_to(r#""\b""#, '\u{0008}');
    deserializes_to(r#""\f""#, '\u{000c}');

    // `deserialize_char` calls `visit_str`
    error_struct!(&str, visit_str, deserialize_char);
    deserializes_with_error::<A>("\n 'x'", make_error("oops", 2, 2));
}

#[test]
#[ignore] // TODO currently unsupported
fn deserializes_str() {
    deserializes_to("'Hello!'", "Hello!");
    deserializes_to("\"안녕하세요\"", "안녕하세요");
    deserializes_to(r#""\uD83C\uDDEF\uD83C\uDDF5""#, "\u{1F1EF}\u{1F1F5}");
}

#[test]
fn deserializes_string() {
    deserializes_to("'Hello!'", "Hello!".to_owned());
    deserializes_to("\"안녕하세요\"", "안녕하세요".to_owned());
    deserializes_to(
        r#""\uD83C\uDDEF\uD83C\uDDF5""#,
        "\u{1F1EF}\u{1F1F5}".to_owned(),
    );

    error_struct!(&str, visit_str, deserialize_string);
    deserializes_with_error::<A>("\n 'Hello!'", make_error("oops", 2, 2));
}

#[test]
#[ignore] // TODO currently unsupported
fn deserializes_bytes() {}

#[test]
#[ignore] // TODO currently unsupported
fn deserializes_byte_buf() {}

#[test]
fn deserializes_option() {
    deserializes_to::<Option<i32>>("null", None);
    deserializes_to("42", Some(42));
    deserializes_to("42", Some(Some(42)));
}

#[test]
fn deserializes_option_error() {
    #[derive(Debug, PartialEq)]
    struct A;
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_option(Visitor)
        }
    }
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_unit() {
    deserializes_to("null", ());

    #[derive(Deserialize, Debug, PartialEq)]
    struct A;
    deserializes_to("null", A);
}

#[test]
fn deserializes_unit_error() {
    #[derive(Debug, PartialEq)]
    struct A;
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_unit(Visitor)
        }
    }
    deserializes_with_error::<A>("\n null", make_error("oops", 2, 2));
}

#[test]
fn deserializes_newtype_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct A(i32);

    #[derive(Deserialize, PartialEq, Debug)]
    struct B(f64);

    deserializes_to("42", A(42));
    deserializes_to("42", B(42.));
}

#[test]
fn deserializes_newtype_struct_error() {
    #[derive(Debug, PartialEq)]
    struct A;
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_newtype_struct<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_newtype_struct("A", Visitor)
        }
    }
    deserializes_with_error::<A>("\n 42", make_error("oops", 2, 2));
}

#[test]
fn deserializes_seq() {
    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum Val {
        Number(f64),
        Bool(bool),
        String(String),
    }

    deserializes_to("[1, 2, 3]", vec![1, 2, 3]);
    deserializes_to(
        "[42, true, 'hello']",
        vec![
            Val::Number(42.),
            Val::Bool(true),
            Val::String("hello".to_owned()),
        ],
    );
}

#[test]
fn deserializes_seq_size_hint() {
    #[derive(Debug, PartialEq)]
    struct Size(usize);
    impl<'de> de::Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Size;

                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }

                fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    Ok(Size(seq.size_hint().unwrap()))
                }
            }
            deserializer.deserialize_seq(Visitor)
        }
    }

    deserializes_to("[]", Size(0));
    deserializes_to("[42, true, 'hello']", Size(3));
    deserializes_to("[42, true, [1, 2]]", Size(3));
}

#[test]
fn deserializes_seq_error() {
    #[derive(Debug, PartialEq)]
    struct A;
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_seq<A>(self, _a: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_seq(Visitor)
        }
    }
    deserializes_with_error::<A>("\n [ true ]", make_error("oops", 2, 2));
}

#[test]
fn deserializes_tuple() {
    deserializes_to("[1, 2, 3]", (1, 2, 3));
}

#[test]
fn deserializes_tuple_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct A(i32, f64);

    #[derive(Deserialize, PartialEq, Debug)]
    struct B(f64, i32);

    deserializes_to("[1, 2]", A(1, 2.));
    deserializes_to("[1, 2]", B(1., 2));
}

#[test]
fn deserializes_tuple_error() {
    #[derive(Debug, PartialEq)]
    struct A;
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_seq<A>(self, _a: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_tuple(2, Visitor)
        }
    }
    deserializes_with_error::<A>("\n [1, 2]", make_error("oops", 2, 2));

    #[derive(Deserialize, Debug, PartialEq)]
    struct B(i32, f64);
    deserializes_with_error::<B>(
        "\n [1]",
        make_error(
            "invalid length 1, expected tuple struct B with 2 elements",
            2,
            2,
        ),
    );
}

#[test]
fn deserializes_map() {
    let mut m = HashMap::new();
    m.insert("a".to_owned(), 1);
    m.insert("b".to_owned(), 2);
    m.insert("c".to_owned(), 3);

    deserializes_to("{ a: 1, 'b': 2, \"c\": 3 }", m);
}

#[test]
fn deserializes_map_size_hint() {
    #[derive(Debug, PartialEq)]
    struct Size(usize);

    impl<'de> de::Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Size;

                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }

                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>,
                {
                    Ok(Size(map.size_hint().unwrap()))
                }
            }
            deserializer.deserialize_map(Visitor)
        }
    }

    deserializes_to("{}", Size(0));
    deserializes_to("{ a: 1, 'b': 2, \"c\": 3 }", Size(3));
    deserializes_to("{ a: 1, 'b': 2, \"c\": [1, 2] }", Size(3));
}

#[test]
fn deserializes_map_error() {
    #[derive(Debug, PartialEq)]
    struct A {}
    impl<'de> de::Deserialize<'de> for A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = A;
                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("...")
                }
                fn visit_map<A>(self, _a: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>,
                {
                    Err(de::Error::custom("oops"))
                }
            }
            deserializer.deserialize_map(Visitor)
        }
    }

    deserializes_with_error::<A>("\n { 'a': true }", make_error("oops", 2, 2));
}

#[test]
fn deserializes_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        a: i32,
        b: i32,
        c: i32,
    }

    deserializes_to("{ a: 1, 'b': 2, \"c\": 3 }", S { a: 1, b: 2, c: 3 });
}

#[test]
fn deserializes_struct_error() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        a: i32,
        b: i32,
        c: i32,
    }
    deserializes_with_error::<S>("\n { a: 1, 'b': 2 }", make_error("missing field `c`", 2, 2));
}

#[test]
fn deserializes_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        A,
        B(i32),
        C(i32, i32),
        D { a: i32, b: i32 },
        E {},
        F(),
    }

    deserializes_to("'A'", E::A);
    deserializes_to("{ B: 2 }", E::B(2));
    deserializes_to("{ C: [3, 5] }", E::C(3, 5));
    deserializes_to("{ D: { a: 7, b: 11 } }", E::D { a: 7, b: 11 });
    deserializes_to("{ E: {} }", E::E {});
    deserializes_to("{ F: [] }", E::F());
}

#[test]
fn deserializes_enum_error() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        A {},
        B(),
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        e: E,
    }

    deserializes_with_error::<S>("{ e: 'A' }", make_error("expected an object", 1, 6));
    deserializes_with_error::<S>("{ e: 'B' }", make_error("expected an array", 1, 6));
    deserializes_with_error::<E>(
        "\n 'C'",
        make_error("unknown variant `C`, expected `A` or `B`", 2, 2),
    );
}

#[test]
fn deserializes_ignored() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        a: i32,
        b: i32,
    }

    deserializes_to("{ a: 1, ignored: 42, b: 2 }", S { a: 1, b: 2 });
}

#[test]
fn deserializes_json_values() {
    // As int if json uses int type.
    deserializes_to("0x2a", serde_json::json!(42));
    deserializes_to("0x2A", serde_json::json!(42));
    deserializes_to("0X2A", serde_json::json!(42));
    deserializes_to("42", serde_json::json!(42));

    // As float if json calls for explicit float type.
    deserializes_to("42.", serde_json::json!(42.));
    deserializes_to("42e0", serde_json::json!(42.));
    deserializes_to("4e2", serde_json::json!(400.));
    deserializes_to("4e2", serde_json::json!(4e2));
}

#[test]
fn deserializes_parse_error() {
    let parse_err_str = r#" --> 1:2
  |
1 | {
  |  ^---
  |
  = expected identifier or string"#;
    #[derive(Deserialize, PartialEq, Debug)]
    struct A;
    deserializes_with_error::<A>("{", make_error(parse_err_str, 1, 2));

    deserializes_with_error::<bool>(
        "\n 42",
        make_error("invalid type: integer `42`, expected a boolean", 2, 2),
    );
}
