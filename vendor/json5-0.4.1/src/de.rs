use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;
use serde::de;
use serde::forward_to_deserialize_any;
use std::char;
use std::collections::VecDeque;
use std::f64;

use crate::error::{self, Error, Result};

#[derive(Parser)]
#[grammar = "json5.pest"]
struct Parser;

/// Deserialize an instance of type `T` from a string of JSON5 text. Can fail if the input is
/// invalid JSON5, or doesn&rsquo;t match the structure of the target type.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de> {
    pair: Option<Pair<'de, Rule>>,
}

impl<'de> Deserializer<'de> {
    /// Creates a JSON5 deserializer from a `&str`. This parses the input at construction time, so
    /// can fail if the input is not valid JSON5.
    pub fn from_str(input: &'de str) -> Result<Self> {
        let pair = Parser::parse(Rule::text, input)?.next().unwrap();
        Ok(Deserializer::from_pair(pair))
    }

    fn from_pair(pair: Pair<'de, Rule>) -> Self {
        Deserializer { pair: Some(pair) }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || match pair.as_rule() {
            Rule::null => visitor.visit_unit(),
            Rule::boolean => visitor.visit_bool(parse_bool(&pair)),
            Rule::string | Rule::identifier => visitor.visit_string(parse_string(pair)?),
            Rule::number => {
                if is_int(pair.as_str()) {
                    visitor.visit_i64(parse_integer(&pair)?)
                } else {
                    visitor.visit_f64(parse_number(&pair)?)
                }
            }
            Rule::array => visitor.visit_seq(Seq::new(pair)),
            Rule::object => visitor.visit_map(Map::new(pair)),
            _ => unreachable!(),
        })();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_enum(Enum { pair }))();
        error::set_location(&mut res, &span);
        res
    }

    // The below will get us the right types, but won't necessarily give
    // meaningful results if the source is out of the range of the target type.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_i8(parse_number(&pair)? as i8))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_i16(parse_number(&pair)? as i16))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_i32(parse_number(&pair)? as i32))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_i64(parse_number(&pair)? as i64))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_i128(parse_number(&pair)? as i128))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_u8(parse_number(&pair)? as u8))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_u16(parse_number(&pair)? as u16))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_u32(parse_number(&pair)? as u32))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_u64(parse_number(&pair)? as u64))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_u128(parse_number(&pair)? as u128))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_f32(parse_number(&pair)? as f32))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || visitor.visit_f64(parse_number(&pair)?))();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let pair = self.pair.take().unwrap();
        let span = pair.as_span();
        let mut res = (move || match pair.as_rule() {
            Rule::null => visitor.visit_none(),
            _ => visitor.visit_some(&mut Deserializer::from_pair(pair)),
        })();
        error::set_location(&mut res, &span);
        res
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let span = self.pair.as_ref().unwrap().as_span();
        let mut res = (move || visitor.visit_newtype_struct(self))();
        error::set_location(&mut res, &span);
        res
    }

    forward_to_deserialize_any! {
        bool char str string bytes byte_buf unit unit_struct seq
        tuple tuple_struct map struct identifier ignored_any
    }
}

fn parse_bool(pair: &Pair<'_, Rule>) -> bool {
    match pair.as_str() {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    }
}

fn parse_string_component(pair: Pair<'_, Rule>) -> Result<String> {
    let mut result = String::new();

    let mut component_iter = pair.into_inner();
    while let Some(component) = component_iter.next() {
        match component.as_rule() {
            Rule::char_literal => result.push_str(component.as_str()),
            Rule::char_escape_sequence => result.push_str(parse_char_escape_sequence(&component)),
            Rule::nul_escape_sequence => result.push_str("\u{0000}"),
            Rule::hex_escape_sequence => {
                let hex_escape = parse_hex(component.as_str())?;
                match char::from_u32(hex_escape) {
                    Some(c) => result.push(c),
                    None => return Err(de::Error::custom("error parsing hex prefix")),
                }
            }
            Rule::unicode_escape_sequence => {
                match parse_hex(component.as_str())? {
                    0xDC00..=0xDFFF => {
                        // Expecting a low surrogate (trail surrogate)
                        return Err(de::Error::custom("unexpected unicode trail surrogate"));
                    }

                    // Non-BMP characters are encoded as a sequence of to hex escapes,
                    // representing UTF-16 surrogate
                    rc1 @ 0xD800..=0xDBFF => {
                        let rc2 = match component_iter.next() {
                            Some(pc2) => match parse_hex(pc2.as_str())? {
                                rc2 @ 0xDC00..=0xDFFF => rc2,
                                _ => {
                                    return Err(de::Error::custom(
                                        "expecting unicode trail surrogate",
                                    ))
                                }
                            },
                            None => {
                                // Missing a low surrogate (trail surrogate)
                                return Err(de::Error::custom("missing unicode trail surrogate"));
                            }
                        };

                        // Join together
                        let rc = ((rc1 - 0xD800) << 10) | (rc2 - 0xDC00) + 0x1_0000;
                        match char::from_u32(rc) {
                            Some(c) => {
                                result.push(c);
                            }
                            None => {
                                return Err(de::Error::custom("invalid non-BMP unicode sequence"));
                            }
                        }
                    }

                    rc => match char::from_u32(rc) {
                        Some(c) => {
                            result.push(c);
                        }
                        None => {
                            return Err(de::Error::custom("invalid unicode character"));
                        }
                    },
                }
            }

            _ => unreachable!(),
        }
    }

    Ok(result)
}

fn parse_string(pair: Pair<'_, Rule>) -> Result<String> {
    let span = pair.as_span();
    let mut res = parse_string_component(pair);
    error::set_location(&mut res, &span);
    res
}

fn parse_char_escape_sequence<'a>(pair: &'a Pair<'_, Rule>) -> &'a str {
    match pair.as_str() {
        "b" => "\u{0008}",
        "f" => "\u{000C}",
        "n" => "\n",
        "r" => "\r",
        "t" => "\t",
        "v" => "\u{000B}",
        c => c,
    }
}

fn parse_number(pair: &Pair<'_, Rule>) -> Result<f64> {
    match pair.as_str() {
        "Infinity" => Ok(f64::INFINITY),
        "-Infinity" => Ok(f64::NEG_INFINITY),
        "NaN" | "-NaN" => Ok(f64::NAN),
        s if is_hex_literal(s) => parse_hex(&s[2..]).map(f64::from),
        s => {
            if let Ok(r) = s.parse::<f64>() {
                if r.is_finite() {
                    Ok(r)
                } else {
                    Err(de::Error::custom("error parsing number: too large"))
                }
            } else {
                Err(de::Error::custom("error parsing number"))
            }
        }
    }
}

fn parse_integer(pair: &Pair<'_, Rule>) -> Result<i64> {
    match pair.as_str() {
        s if is_hex_literal(s) => Ok(parse_hex(&s[2..])? as i64),
        s => s
            .parse()
            .or_else(|_| Err(de::Error::custom("error parsing integer"))),
    }
}

fn is_int(s: &str) -> bool {
    !s.contains('.')
        && (is_hex_literal(s) || (!s.contains('e') && !s.contains('E')))
        && !is_infinite(s)
        && !is_nan(s)
}

fn parse_hex(s: &str) -> Result<u32> {
    u32::from_str_radix(s, 16).or_else(|_| Err(de::Error::custom("error parsing hex")))
}

fn is_hex_literal(s: &str) -> bool {
    s.len() > 2 && (&s[..2] == "0x" || &s[..2] == "0X")
}

fn is_infinite(s: &str) -> bool {
    s == "Infinity" || s == "-Infinity"
}

fn is_nan(s: &str) -> bool {
    s == "NaN" || s == "-NaN"
}

struct Seq<'de> {
    pairs: VecDeque<Pair<'de, Rule>>,
}

impl<'de> Seq<'de> {
    pub fn new(pair: Pair<'de, Rule>) -> Self {
        Self {
            pairs: pair.into_inner().collect(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for Seq<'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.pairs.len())
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(pair) = self.pairs.pop_front() {
            seed.deserialize(&mut Deserializer::from_pair(pair))
                .map(Some)
        } else {
            Ok(None)
        }
    }
}

struct Map<'de> {
    pairs: VecDeque<Pair<'de, Rule>>,
}

impl<'de> Map<'de> {
    pub fn new(pair: Pair<'de, Rule>) -> Self {
        Self {
            pairs: pair.into_inner().collect(),
        }
    }
}

impl<'de> de::MapAccess<'de> for Map<'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.pairs.len() / 2)
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(pair) = self.pairs.pop_front() {
            seed.deserialize(&mut Deserializer::from_pair(pair))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::from_pair(
            self.pairs.pop_front().unwrap(),
        ))
    }
}

struct Enum<'de> {
    pair: Pair<'de, Rule>,
}

impl<'de> de::EnumAccess<'de> for Enum<'de> {
    type Error = Error;
    type Variant = Variant<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let span = self.pair.as_span();
        let mut res = (move || match self.pair.as_rule() {
            Rule::string => {
                let tag = seed.deserialize(&mut Deserializer::from_pair(self.pair))?;
                Ok((tag, Variant { pair: None }))
            }
            Rule::object => {
                let mut pairs = self.pair.into_inner();

                if let Some(tag_pair) = pairs.next() {
                    let tag = seed.deserialize(&mut Deserializer::from_pair(tag_pair))?;
                    Ok((tag, Variant { pair: pairs.next() }))
                } else {
                    Err(de::Error::custom("expected a nonempty object"))
                }
            }
            _ => Err(de::Error::custom("expected a string or an object")),
        })();
        error::set_location(&mut res, &span);
        res
    }
}

struct Variant<'de> {
    pair: Option<Pair<'de, Rule>>,
}

impl<'de, 'a> de::VariantAccess<'de> for Variant<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::from_pair(self.pair.unwrap()))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.pair {
            Some(pair) => match pair.as_rule() {
                Rule::array => visitor.visit_seq(Seq::new(pair)),
                _ => Err(de::Error::custom("expected an array")),
            },
            None => Err(de::Error::custom("expected an array")),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.pair {
            Some(pair) => match pair.as_rule() {
                Rule::object => visitor.visit_map(Map::new(pair)),
                _ => Err(de::Error::custom("expected an object")),
            },
            None => Err(de::Error::custom("expected an object")),
        }
    }
}
