use crate::ast::{kw, Float32, Float64, Index, HeapType};
use crate::parser::{Parse, Parser, Result};

/// An expression that is valid inside an `assert_return` directive.
///
/// As of https://github.com/WebAssembly/spec/pull/1104, spec tests may include `assert_return`
/// directives that allow NaN patterns (`"nan:canonical"`, `"nan:arithmetic"`). Parsing an
/// `AssertExpression` means that:
/// - only constant values (e.g. `i32.const 4`) are used in the `assert_return` directive
/// - the NaN patterns are allowed (they are not allowed in regular `Expression`s).
#[derive(Debug)]
#[allow(missing_docs)]
pub enum AssertExpression<'a> {
    I32(i32),
    I64(i64),
    F32(NanPattern<Float32>),
    F64(NanPattern<Float64>),
    V128(V128Pattern),

    RefNull(Option<HeapType<'a>>),
    RefExtern(u32),
    RefFunc(Option<Index<'a>>),

    // Either matches an f32 or f64 for an arithmetic nan pattern
    LegacyArithmeticNaN,
    // Either matches an f32 or f64 for a canonical nan pattern
    LegacyCanonicalNaN,
}

impl<'a> Parse<'a> for AssertExpression<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let keyword = parser.step(|c| match c.keyword() {
            Some(pair) => Ok(pair),
            None => Err(c.error("expected a keyword")),
        })?;

        match keyword {
            "i32.const" => Ok(AssertExpression::I32(parser.parse()?)),
            "i64.const" => Ok(AssertExpression::I64(parser.parse()?)),
            "f32.const" => Ok(AssertExpression::F32(parser.parse()?)),
            "f64.const" => Ok(AssertExpression::F64(parser.parse()?)),
            "v128.const" => Ok(AssertExpression::V128(parser.parse()?)),
            "ref.null" => Ok(AssertExpression::RefNull(parser.parse()?)),
            "ref.extern" => Ok(AssertExpression::RefExtern(parser.parse()?)),
            "ref.func" => Ok(AssertExpression::RefFunc(parser.parse()?)),
            _ => Err(parser.error("expected a [type].const expression")),
        }
    }
}

/// Either a NaN pattern (`nan:canonical`, `nan:arithmetic`) or a value of type `T`.
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum NanPattern<T> {
    CanonicalNan,
    ArithmeticNan,
    Value(T),
}

impl<'a, T> Parse<'a> for NanPattern<T>
where
    T: Parse<'a>,
{
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek::<kw::nan_canonical>() {
            parser.parse::<kw::nan_canonical>()?;
            Ok(NanPattern::CanonicalNan)
        } else if parser.peek::<kw::nan_arithmetic>() {
            parser.parse::<kw::nan_arithmetic>()?;
            Ok(NanPattern::ArithmeticNan)
        } else {
            let val = parser.parse()?;
            Ok(NanPattern::Value(val))
        }
    }
}

/// A version of `V128Const` that allows `NanPattern`s.
///
/// This implementation is necessary because only float types can include NaN patterns; otherwise
/// it is largely similar to the implementation of `V128Const`.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum V128Pattern {
    I8x16([i8; 16]),
    I16x8([i16; 8]),
    I32x4([i32; 4]),
    I64x2([i64; 2]),
    F32x4([NanPattern<Float32>; 4]),
    F64x2([NanPattern<Float64>; 2]),
}

impl<'a> Parse<'a> for V128Pattern {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::i8x16>() {
            parser.parse::<kw::i8x16>()?;
            Ok(V128Pattern::I8x16([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i16x8>() {
            parser.parse::<kw::i16x8>()?;
            Ok(V128Pattern::I16x8([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i32x4>() {
            parser.parse::<kw::i32x4>()?;
            Ok(V128Pattern::I32x4([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i64x2>() {
            parser.parse::<kw::i64x2>()?;
            Ok(V128Pattern::I64x2([parser.parse()?, parser.parse()?]))
        } else if l.peek::<kw::f32x4>() {
            parser.parse::<kw::f32x4>()?;
            Ok(V128Pattern::F32x4([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::f64x2>() {
            parser.parse::<kw::f64x2>()?;
            Ok(V128Pattern::F64x2([parser.parse()?, parser.parse()?]))
        } else {
            Err(l.error())
        }
    }
}
