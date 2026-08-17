use std::str;

use protobuf_support::lexer::int;
use protobuf_support::lexer::loc::Loc;
use protobuf_support::lexer::parser_language::ParserLanguage;
use protobuf_support::lexer::str_lit::StrLitDecodeError;
use protobuf_support::lexer::tokenizer::Tokenizer;
use protobuf_support::lexer::tokenizer::TokenizerError;

use crate::message_dyn::MessageDyn;
use crate::message_full::MessageFull;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeType;

#[derive(Debug, thiserror::Error)]
pub enum ParseErrorWithoutLoc {
    #[error(transparent)]
    TokenizerError(#[from] TokenizerError),
    #[error(transparent)]
    StrLitDecodeError(#[from] StrLitDecodeError),
    #[error("Unknown field: `{}`", .0)]
    UnknownField(String),
    #[error("Unknown enum value: `{}`", .0)]
    UnknownEnumValue(String),
    #[error("Map field specified more than once: `{}`", .0)]
    MapFieldIsSpecifiedMoreThanOnce(String),
    #[error("Integer overflow")]
    IntegerOverflow,
    #[error("Expecting bool")]
    ExpectingBool,
    #[error("Message not initialized")]
    MessageNotInitialized,
}

impl From<int::Overflow> for ParseErrorWithoutLoc {
    fn from(_: int::Overflow) -> Self {
        ParseErrorWithoutLoc::IntegerOverflow
    }
}

/// Text format parse error.
#[derive(Debug, thiserror::Error)]
#[error("{}: {}", loc, error)]
pub struct ParseError {
    error: ParseErrorWithoutLoc,
    loc: Loc,
}

pub type ParseResult<A> = Result<A, ParseErrorWithoutLoc>;
pub type ParseWithLocResult<A> = Result<A, ParseError>;

#[derive(Clone)]
struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    // Text format

    fn next_field_name(&mut self) -> ParseResult<String> {
        Ok(self.tokenizer.next_ident()?)
    }

    fn read_colon(&mut self, desc: &'static str) -> ParseResult<()> {
        Ok(self.tokenizer.next_symbol_expect_eq(':', desc)?)
    }

    fn read_enum<'e>(&mut self, e: &'e EnumDescriptor) -> ParseResult<EnumValueDescriptor> {
        self.read_colon("enum")?;

        // TODO: read integer?
        let ident = self.tokenizer.next_ident()?;
        let value = match e.value_by_name(&ident) {
            Some(value) => value,
            None => return Err(ParseErrorWithoutLoc::UnknownEnumValue(ident)),
        };
        Ok(value)
    }

    fn read_u64(&mut self) -> ParseResult<u64> {
        self.read_colon("u64")?;

        Ok(self.tokenizer.next_int_lit()?)
    }

    fn read_u32(&mut self) -> ParseResult<u32> {
        self.read_colon("int value")?;

        let int_lit = self.tokenizer.next_int_lit()?;
        let value_u32 = int_lit as u32;
        if value_u32 as u64 != int_lit {
            return Err(ParseErrorWithoutLoc::IntegerOverflow);
        }
        Ok(value_u32)
    }

    fn read_i64(&mut self) -> ParseResult<i64> {
        self.read_colon("int value")?;

        if self.tokenizer.next_symbol_if_eq('-')? {
            let int_lit = self.tokenizer.next_int_lit()?;
            Ok(int::neg(int_lit)?)
        } else {
            let int_lit = self.tokenizer.next_int_lit()?;
            if int_lit > i64::MAX as u64 {
                return Err(ParseErrorWithoutLoc::IntegerOverflow);
            }
            Ok(int_lit as i64)
        }
    }

    fn read_i32(&mut self) -> ParseResult<i32> {
        let value = self.read_i64()?;
        if value < i32::min_value() as i64 || value > i32::max_value() as i64 {
            return Err(ParseErrorWithoutLoc::IntegerOverflow);
        }
        Ok(value as i32)
    }

    fn read_f64(&mut self) -> ParseResult<f64> {
        self.read_colon("float value")?;

        let minus = self.tokenizer.next_symbol_if_eq('-')?;

        let value = if let Ok(value) = self.tokenizer.next_int_lit() {
            value as f64
        } else {
            self.tokenizer.next_float_lit()?
        };

        Ok(if minus { -value } else { value })
    }

    fn read_f32(&mut self) -> ParseResult<f32> {
        Ok(self.read_f64()? as f32)
    }

    fn read_bool(&mut self) -> ParseResult<bool> {
        self.read_colon("bool value")?;

        if self.tokenizer.next_ident_if_eq("true")? {
            Ok(true)
        } else if self.tokenizer.next_ident_if_eq("false")? {
            Ok(false)
        } else {
            Err(ParseErrorWithoutLoc::ExpectingBool)
        }
    }

    fn read_string(&mut self) -> ParseResult<String> {
        self.read_colon("string value")?;

        Ok(self
            .tokenizer
            .next_str_lit()
            .and_then(|s| s.decode_utf8().map_err(From::from))?)
    }

    fn read_bytes(&mut self) -> ParseResult<Vec<u8>> {
        self.read_colon("bytes value")?;

        Ok(self
            .tokenizer
            .next_str_lit()
            .and_then(|s| s.decode_bytes().map_err(From::from))?)
    }

    fn read_message(&mut self, descriptor: &MessageDescriptor) -> ParseResult<Box<dyn MessageDyn>> {
        let mut message = descriptor.new_instance();

        let symbol = self.tokenizer.next_symbol_expect_eq_oneof(&['{', '<'])?;
        let terminator = if symbol == '{' { '}' } else { '>' };
        while !self.tokenizer.lookahead_is_symbol(terminator)? {
            self.merge_field(&mut *message, descriptor)?;
        }
        self.tokenizer
            .next_symbol_expect_eq(terminator, "message")?;
        Ok(message)
    }

    fn read_map_entry(
        &mut self,
        k: &RuntimeType,
        v: &RuntimeType,
    ) -> ParseResult<(ReflectValueBox, ReflectValueBox)> {
        let key_field_name: &str = "key";
        let value_field_name: &str = "value";

        let mut key = None;
        let mut value = None;
        self.tokenizer.next_symbol_expect_eq('{', "map entry")?;
        while !self.tokenizer.lookahead_is_symbol('}')? {
            let ident = self.next_field_name()?;
            let (field, field_type) = if ident == key_field_name {
                (&mut key, k)
            } else if ident == value_field_name {
                (&mut value, v)
            } else {
                return Err(ParseErrorWithoutLoc::UnknownField(ident));
            };

            if let Some(..) = *field {
                return Err(ParseErrorWithoutLoc::MapFieldIsSpecifiedMoreThanOnce(ident));
            }

            let field_value = self.read_value_of_type(field_type)?;

            *field = Some(field_value);
        }
        self.tokenizer.next_symbol_expect_eq('}', "map entry")?;
        let key = match key {
            Some(key) => key,
            None => k.default_value_ref().to_box(),
        };
        let value = match value {
            Some(value) => value,
            None => v.default_value_ref().to_box(),
        };
        Ok((key, value))
    }

    fn read_value_of_type(&mut self, t: &RuntimeType) -> ParseResult<ReflectValueBox> {
        Ok(match t {
            RuntimeType::Enum(d) => {
                let value = self.read_enum(&d)?.value();
                ReflectValueBox::Enum(d.clone(), value)
            }
            RuntimeType::U32 => ReflectValueBox::U32(self.read_u32()?),
            RuntimeType::U64 => ReflectValueBox::U64(self.read_u64()?),
            RuntimeType::I32 => ReflectValueBox::I32(self.read_i32()?),
            RuntimeType::I64 => ReflectValueBox::I64(self.read_i64()?),
            RuntimeType::F32 => ReflectValueBox::F32(self.read_f32()?),
            RuntimeType::F64 => ReflectValueBox::F64(self.read_f64()?),
            RuntimeType::Bool => ReflectValueBox::Bool(self.read_bool()?),
            RuntimeType::String => ReflectValueBox::String(self.read_string()?),
            RuntimeType::VecU8 => ReflectValueBox::Bytes(self.read_bytes()?),
            RuntimeType::Message(m) => ReflectValueBox::Message(self.read_message(&m)?),
        })
    }

    fn merge_field(
        &mut self,
        message: &mut dyn MessageDyn,
        descriptor: &MessageDescriptor,
    ) -> ParseResult<()> {
        let field_name = self.next_field_name()?;

        let field = match descriptor.field_by_name(&field_name) {
            Some(field) => field,
            None => {
                // TODO: shouldn't unknown fields be quietly skipped?
                return Err(ParseErrorWithoutLoc::UnknownField(field_name));
            }
        };

        match field.runtime_field_type() {
            RuntimeFieldType::Singular(t) => {
                let value = self.read_value_of_type(&t)?;
                field.set_singular_field(message, value);
            }
            RuntimeFieldType::Repeated(t) => {
                let value = self.read_value_of_type(&t)?;
                field.mut_repeated(message).push(value);
            }
            RuntimeFieldType::Map(k, v) => {
                let (k, v) = self.read_map_entry(&k, &v)?;
                field.mut_map(message).insert(k, v);
            }
        };

        Ok(())
    }

    fn merge_inner(&mut self, message: &mut dyn MessageDyn) -> ParseResult<()> {
        loop {
            if self.tokenizer.syntax_eof()? {
                break;
            }
            let descriptor = message.descriptor_dyn();
            self.merge_field(message, &descriptor)?;
        }
        Ok(())
    }

    fn merge(&mut self, message: &mut dyn MessageDyn) -> ParseWithLocResult<()> {
        match self.merge_inner(message) {
            Ok(()) => Ok(()),
            Err(error) => Err(ParseError {
                error,
                loc: self.tokenizer.loc(),
            }),
        }
    }
}

/// Parse text format message.
///
/// This function does not check if message required fields are set.
pub fn merge_from_str(message: &mut dyn MessageDyn, input: &str) -> ParseWithLocResult<()> {
    let mut parser = Parser {
        tokenizer: Tokenizer::new(input, ParserLanguage::TextFormat),
    };
    parser.merge(message)
}

/// Parse text format message.
pub fn parse_from_str<M: MessageFull>(input: &str) -> ParseWithLocResult<M> {
    let mut m = M::new();
    merge_from_str(&mut m, input)?;
    if let Err(_) = m.check_initialized() {
        return Err(ParseError {
            error: ParseErrorWithoutLoc::MessageNotInitialized,
            loc: Loc::start(),
        });
    }
    Ok(m)
}
