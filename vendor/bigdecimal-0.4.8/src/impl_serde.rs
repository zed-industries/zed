//!
//! Support for serde implementations
//!
use crate::*;
use serde_crate::{self as serde, de, ser, Serialize, Deserialize};

// const SERDE_SCALE_LIMIT: usize = = ${RUST_BIGDECIMAL_SERDE_SCALE_LIMIT} or  150_000;
#[cfg(feature = "serde_json")]
include!(concat!(env!("OUT_DIR"), "/serde_scale_limit.rs"));

impl ser::Serialize for BigDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

/// Used by SerDe to construct a BigDecimal
struct BigDecimalVisitor;

impl<'de> de::Visitor<'de> for BigDecimalVisitor {
    type Value = BigDecimal;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a number or formatted decimal string")
    }

    fn visit_str<E>(self, value: &str) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        BigDecimal::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        Ok(BigDecimal::from(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        Ok(BigDecimal::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        Ok(BigDecimal::from(value))
    }

    fn visit_i128<E>(self, value: i128) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        Ok(BigDecimal::from(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        BigDecimal::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<BigDecimal, E>
    where
        E: de::Error,
    {
        BigDecimal::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        match map.next_key::<&str>() {
            Ok(Some("$serde_json::private::Number")) => {
                map.next_value::<BigDecimal>()
            }
            _ => {
                Err(de::Error::invalid_type(de::Unexpected::Map, &self))
            }
        }
    }
}

#[cfg(not(feature = "string-only"))]
impl<'de> de::Deserialize<'de> for BigDecimal {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(BigDecimalVisitor)
    }
}

#[cfg(feature = "string-only")]
impl<'de> de::Deserialize<'de> for BigDecimal {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(BigDecimalVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    use serde_test::{
        Token, assert_tokens, assert_de_tokens, assert_de_tokens_error
    };

    mod serde_serialize_deserialize_str {
        use super::*;

        macro_rules! impl_case {
            ($name:ident : $input:literal => $output:literal) => {
                #[test]
                fn $name() {
                    let expected = Token::Str($output);
                    let decimal: BigDecimal = $input.parse().unwrap();
                    assert_tokens(&decimal, &[expected]);
                }
            }
        }

        impl_case!(case_1d0: "1.0" => "1.0");
        impl_case!(case_0d5: "0.5" => "0.5");
        impl_case!(case_50: "50" => "50");
        impl_case!(case_50000: "50000" => "50000");
        impl_case!(case_1en3: "1e-3" => "0.001");
        impl_case!(case_10e11: "10e11" => "1000000000000");
        impl_case!(case_d25: ".25" => "0.25");
        impl_case!(case_12d34e1: "12.34e1" => "123.4");
        impl_case!(case_40d0010: "40.0010" => "40.0010");
    }

    #[cfg(not(feature = "string-only"))]
    mod serde_deserialize_int {
        use super::*;

        macro_rules! impl_case {
            ( $( $ttype:ident ),+ : -$input:literal ) => {
                $( paste! { impl_case!([< case_n $input _ $ttype:lower >] : $ttype : -$input); } )*
            };
            ( $( $ttype:ident ),+ : $input:literal ) => {
                $( paste! { impl_case!([< case_ $input _ $ttype:lower >] : $ttype : $input); } )*
            };
            ($name:ident : $type:ident : $input:literal) => {
                #[test]
                fn $name() {
                    let expected = BigDecimal::from($input);
                    let token = Token::$type($input);
                    assert_de_tokens(&expected, &[token]);
                }
            };
        }

        impl_case!(I8, I16, I32, I64, U8, U16, U32, U64 : 0);
        impl_case!(I8, I16, I32, I64, U8, U16, U32, U64 : 1);
        impl_case!(I8, I16, I32, I64 : -1);
        impl_case!(I64: -99999999999i64);
        impl_case!(I64: -9_223_372_036_854_775_808i64);
    }

    #[cfg(not(feature = "string-only"))]
    mod serde_deserialize_float {
        use super::*;

        macro_rules! impl_case {
            ( $name:ident : $input:literal => $ttype:ident : $expected:literal ) => {
                paste! {
                    #[test]
                    fn [< $name _ $ttype:lower >]() {
                        let expected: BigDecimal = $expected.parse().unwrap();
                        let token = Token::$ttype($input);
                        assert_de_tokens(&expected, &[token]);
                    }
                }
            };
            ( $name:ident : $input:literal => $( $ttype:ident : $expected:literal )+ ) => {
                $( impl_case!($name : $input => $ttype : $expected); )*
            };
            ( $name:ident : $input:literal => $( $ttype:ident ),+ : $expected:literal ) => {
                $( impl_case!($name : $input => $ttype : $expected); )*
            };
        }

        impl_case!(case_1d0 : 1.0 => F32, F64 : "1");
        impl_case!(case_1d1 : 1.1 => F32 : "1.10000002384185791015625"
                                     F64 : "1.100000000000000088817841970012523233890533447265625");

        impl_case!(case_0d001834988943300:
            0.001834988943300 => F32 : "0.001834988943301141262054443359375"
                                 F64 : "0.00183498894330000003084768511740776375518180429935455322265625");

        impl_case!(case_n869651d9131236838:
            -869651.9131236838 => F32 : "-869651.9375"
                                  F64 : "-869651.91312368377111852169036865234375");

        impl_case!(case_n1en20:
            -1e-20 => F32 : "-9.999999682655225388967887463487205224055287544615566730499267578125E-21"
                      F64 : "-999999999999999945153271454209571651729503702787392447107715776066783064379706047475337982177734375e-119");
    }

    #[cfg(not(feature = "string-only"))]
    mod serde_deserialize_nan {
        use super::*;

        #[test]
        fn case_f32() {
            let tokens = [ Token::F32(f32::NAN) ];
            assert_de_tokens_error::<BigDecimal>(&tokens, "NAN");
        }

        #[test]
        fn case_f64() {
            let tokens = [ Token::F64(f64::NAN) ];
            assert_de_tokens_error::<BigDecimal>(&tokens, "NAN");
        }
    }


    #[cfg(feature = "serde_json")]
    mod json_support {
        use super::*;
        use impl_serde::{Serialize, Deserialize};
        use serde_json;

        #[derive(Serialize,Deserialize)]
        struct TestStruct {
            name: String,
            value: BigDecimal,
            #[serde(with = "crate::serde::json_num")]
            number: BigDecimal,
        }


        #[test]
        fn test_struct_parsing() {
            let json_src = r#"
                { "name": "foo", "value": 0.0008741329382918, "number": "12.34" }
            "#;
            let my_struct: TestStruct = serde_json::from_str(&json_src).unwrap();
            assert_eq!(&my_struct.name, "foo");
            assert_eq!(&my_struct.value, &"0.0008741329382918".parse().unwrap());
            assert_eq!(&my_struct.number, &"12.34".parse().unwrap());

            let s = serde_json::to_string(&my_struct).unwrap();
            assert_eq!(s, r#"{"name":"foo","value":"0.0008741329382918","number":12.34}"#);
        }

    }
}


/// Serialize/deserialize [`BigDecimal`] as arbitrary precision numbers in JSON using the `arbitrary_precision` feature within `serde_json`.
///
// The following example is ignored as it requires derives which we don't import and aren't compatible
// with our locked versions of rust due to proc_macro2.
/// ```ignore
/// # extern crate serde;
/// # use serde::{Serialize, Deserialize};
/// # use bigdecimal::BigDecimal;
/// # use std::str::FromStr;
///
/// #[derive(Serialize, Deserialize)]
/// pub struct ArbitraryExample {
///     #[serde(with = "bigdecimal::serde::json_num")]
///     value: BigDecimal,
/// }
///
/// let value = ArbitraryExample { value: BigDecimal::from_str("123.400").unwrap() };
/// assert_eq!(
///     &serde_json::to_string(&value).unwrap(),
///     r#"{"value":123.400}"#
/// );
/// ```
#[cfg(feature = "serde_json")]
pub mod arbitrary_precision {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let n = BigDecimal::deserialize(deserializer)?;

        if n.scale.abs() > SERDE_SCALE_LIMIT && SERDE_SCALE_LIMIT > 0 {
            let msg = format!("Calculated exponent '{}' out of bounds", -n.scale);
            Err(serde::de::Error::custom(msg))
        } else {
            Ok(n)
        }
    }

    pub fn serialize<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Number::from_str(&value.to_string())
                           .map_err(ser::Error::custom)?
                           .serialize(serializer)
    }
}


/// Serialize/deserialize [`Option<BigDecimal>`] as arbitrary precision numbers in JSON using the `arbitrary_precision` feature within `serde_json`.
///
// The following example is ignored as it requires derives which we don't import and aren't compatible
// with our locked versions of rust due to proc_macro2.
/// ```ignore
/// # extern crate serde;
/// # use serde::{Serialize, Deserialize};
/// # use bigdecimal::BigDecimal;
/// # use std::str::FromStr;
///
/// #[derive(Serialize, Deserialize)]
/// pub struct ArbitraryExample {
///     #[serde(with = "bigdecimal::impl_serde::arbitrary_precision_option")]
///     value: Option<BigDecimal>,
/// }
///
/// let value = ArbitraryExample { value: Some(BigDecimal::from_str("123.400").unwrap()) };
/// assert_eq!(
///     &serde_json::to_string(&value).unwrap(),
///     r#"{"value":123.400}"#
/// );
///
/// let value = ArbitraryExample { value: None };
/// assert_eq!(
///     &serde_json::to_string(&value).unwrap(),
///     r#"{"value":null}"#
/// );
/// ```
#[cfg(feature = "serde_json")]
pub mod arbitrary_precision_option {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BigDecimal>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Option::<serde_json::Number>::deserialize(deserializer)?
                                     .map(|num| num.as_str().parse().map_err(serde::de::Error::custom))
                                     .transpose()
    }

    pub fn serialize<S>(value: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *value {
            Some(ref decimal) => {
                serde_json::Number::from_str(&decimal.to_string())
                                   .map_err(serde::ser::Error::custom)?
                                   .serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }
}


#[cfg(all(test, feature = "serde_json"))]
mod test_jsonification {
    use super::*;
    extern crate serde_json;

    mod deserialize_f64 {
        use super::*;

        macro_rules! impl_case {
            ($name:ident : $input:expr) => {
                impl_case!($name: $input => $input);
            };
            ($name:ident : $input:expr => $expected:literal) => {
                #[test]
                fn $name() {
                    let mut json_deserialize = serde_json::Deserializer::from_str($input);
                    let value = arbitrary_precision::deserialize(&mut json_deserialize)
                                                    .expect("should parse JSON");
                    let expected: BigDecimal = $expected.parse().unwrap();

                    assert_eq!(expected, value);
                }
            };
        }

        impl_case!(case_1d0: "1.0" => "1.0");
        impl_case!(case_0d001: "0.001" => "0.001");
        impl_case!(case_41009d2207etc: "41009.22075436852032769878903135430037010");
    }

    mod serde_struct_decimals {
        use super::*;
        use crate as bigdecimal;

        #[derive(Serialize, Deserialize)]
        pub struct TestExample {
            #[serde(with = "bigdecimal::serde::json_num")]
            value: BigDecimal,
        }

        macro_rules! impl_case {
            ($name:ident : $input:expr => (error)) => {
                #[test]
                fn $name() {
                    let res = serde_json::from_str::<TestExample>($input);
                    assert!(res.is_err());
                }
            };
            ($name:ident : $input:expr => $expected:expr) => {
                #[test]
                fn $name() {
                    let obj: TestExample = serde_json::from_str($input).unwrap();
                    let expected: BigDecimal = $expected.parse().unwrap();

                    assert_eq!(expected, obj.value);

                    // expect output to be input with all spaces removed
                    let s = serde_json::to_string(&obj).unwrap();
                    let input_stripped = $input.replace(" ", "");

                    assert_eq!(&s, &input_stripped);
                }
            };
        }

        impl_case!(case_1d0: r#"{ "value": 1.0 }"# => "1.0" );
        impl_case!(case_2d01: r#"{ "value": 2.01 }"# => "2.01" );
        impl_case!(case_50: r#"{ "value": 50 }"# => "50" );
        impl_case!(case_high_prec: r#"{ "value": 64771126779.35857825871133263810255301911 }"# => "64771126779.35857825871133263810255301911" );

        impl_case!(case_nan: r#"{ "value": nan }"# => (error) );


        #[test]
        fn scale_out_of_bounds() {
            use serde_crate::de::Error;

            let src = r#"{ "value": 1e92233720392233 }"#;

            let parse_err = serde_json::from_str::<TestExample>(&src).err().unwrap();
            let err_str = parse_err.to_string();

            assert!(err_str.starts_with("Calculated exponent '92233720392233' out of bounds"), "{}", err_str);
        }
    }
}
