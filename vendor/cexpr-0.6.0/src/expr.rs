// (C) Copyright 2016 Jethro G. Beekman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Evaluating C expressions from tokens.
//!
//! Numerical operators are supported. All numerical values are treated as
//! `i64` or `f64`. Type casting is not supported. `i64` are converted to
//! `f64` when used in conjunction with a `f64`. Right shifts are always
//! arithmetic shifts.
//!
//! The `sizeof` operator is not supported.
//!
//! String concatenation is supported, but width prefixes are ignored; all
//! strings are treated as narrow strings.
//!
//! Use the `IdentifierParser` to substitute identifiers found in expressions.

use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};

use crate::literal::{self, CChar};
use crate::token::{Kind as TokenKind, Token};
use crate::ToCexprResult;
use nom::branch::alt;
use nom::combinator::{complete, map, map_opt};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::*;

/// Expression parser/evaluator that supports identifiers.
#[derive(Debug)]
pub struct IdentifierParser<'ident> {
    identifiers: &'ident HashMap<Vec<u8>, EvalResult>,
}
#[derive(Copy, Clone)]
struct PRef<'a>(&'a IdentifierParser<'a>);

/// A shorthand for the type of cexpr expression evaluation results.
pub type CResult<'a, R> = IResult<&'a [Token], R, crate::Error<&'a [Token]>>;

/// The result of parsing a literal or evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum EvalResult {
    Int(Wrapping<i64>),
    Float(f64),
    Char(CChar),
    Str(Vec<u8>),
    Invalid,
}

macro_rules! result_opt (
	(fn $n:ident: $e:ident -> $t:ty) => (
		#[allow(dead_code)]
        #[allow(clippy::wrong_self_convention)]
		fn $n(self) -> Option<$t> {
			if let EvalResult::$e(v) = self {
				Some(v)
			} else {
				None
			}
		}
	);
);

impl EvalResult {
    result_opt!(fn as_int: Int -> Wrapping<i64>);
    result_opt!(fn as_float: Float -> f64);
    result_opt!(fn as_char: Char -> CChar);
    result_opt!(fn as_str: Str -> Vec<u8>);

    #[allow(clippy::wrong_self_convention)]
    fn as_numeric(self) -> Option<EvalResult> {
        match self {
            EvalResult::Int(_) | EvalResult::Float(_) => Some(self),
            _ => None,
        }
    }
}

impl From<Vec<u8>> for EvalResult {
    fn from(s: Vec<u8>) -> EvalResult {
        EvalResult::Str(s)
    }
}

// ===========================================
// ============= Clang tokens ================
// ===========================================

macro_rules! exact_token (
	($k:ident, $c:expr) => ({
        move |input: &[Token]| {
		if input.is_empty() {
			let res: CResult<'_, &[u8]> = Err(crate::nom::Err::Incomplete(Needed::new($c.len())));
			res
		} else {
			if input[0].kind==TokenKind::$k && &input[0].raw[..]==$c {
				Ok((&input[1..], &input[0].raw[..]))
			} else {
				Err(crate::nom::Err::Error((input, crate::ErrorKind::ExactToken(TokenKind::$k,$c)).into()))
			}
		}
        }
	});
);

fn identifier_token(input: &[Token]) -> CResult<'_, &[u8]> {
    if input.is_empty() {
        let res: CResult<'_, &[u8]> = Err(nom::Err::Incomplete(Needed::new(1)));
        res
    } else {
        if input[0].kind == TokenKind::Identifier {
            Ok((&input[1..], &input[0].raw[..]))
        } else {
            Err(crate::nom::Err::Error((input, crate::ErrorKind::TypedToken(TokenKind::Identifier)).into()))
        }
    }
}

fn p(c: &'static str) -> impl Fn(&[Token]) -> CResult<'_, &[u8]> {
    exact_token!(Punctuation, c.as_bytes())
}

fn one_of_punctuation(c: &'static [&'static str]) -> impl Fn(&[Token]) -> CResult<'_, &[u8]> {
    move |input| {
        if input.is_empty() {
            let min = c
                .iter()
                .map(|opt| opt.len())
                .min()
                .expect("at least one option");
            Err(crate::nom::Err::Incomplete(Needed::new(min)))
        } else if input[0].kind == TokenKind::Punctuation
            && c.iter().any(|opt| opt.as_bytes() == &input[0].raw[..])
        {
            Ok((&input[1..], &input[0].raw[..]))
        } else {
            Err(crate::nom::Err::Error(
                (
                    input,
                    crate::ErrorKind::ExactTokens(TokenKind::Punctuation, c),
                )
                    .into(),
            ))
        }
    }
}

// ==================================================
// ============= Numeric expressions ================
// ==================================================

impl<'a> AddAssign<&'a EvalResult> for EvalResult {
    fn add_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a + b),
            (&Float(a), &Int(b)) => Float(a + (b.0 as f64)),
            (&Int(a), &Float(b)) => Float(a.0 as f64 + b),
            (&Float(a), &Float(b)) => Float(a + b),
            _ => Invalid,
        };
    }
}
impl<'a> BitAndAssign<&'a EvalResult> for EvalResult {
    fn bitand_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a & b),
            _ => Invalid,
        };
    }
}
impl<'a> BitOrAssign<&'a EvalResult> for EvalResult {
    fn bitor_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a | b),
            _ => Invalid,
        };
    }
}
impl<'a> BitXorAssign<&'a EvalResult> for EvalResult {
    fn bitxor_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a ^ b),
            _ => Invalid,
        };
    }
}
impl<'a> DivAssign<&'a EvalResult> for EvalResult {
    fn div_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a / b),
            (&Float(a), &Int(b)) => Float(a / (b.0 as f64)),
            (&Int(a), &Float(b)) => Float(a.0 as f64 / b),
            (&Float(a), &Float(b)) => Float(a / b),
            _ => Invalid,
        };
    }
}
impl<'a> MulAssign<&'a EvalResult> for EvalResult {
    fn mul_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a * b),
            (&Float(a), &Int(b)) => Float(a * (b.0 as f64)),
            (&Int(a), &Float(b)) => Float(a.0 as f64 * b),
            (&Float(a), &Float(b)) => Float(a * b),
            _ => Invalid,
        };
    }
}
impl<'a> RemAssign<&'a EvalResult> for EvalResult {
    fn rem_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a % b),
            (&Float(a), &Int(b)) => Float(a % (b.0 as f64)),
            (&Int(a), &Float(b)) => Float(a.0 as f64 % b),
            (&Float(a), &Float(b)) => Float(a % b),
            _ => Invalid,
        };
    }
}
impl<'a> ShlAssign<&'a EvalResult> for EvalResult {
    fn shl_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a << (b.0 as usize)),
            _ => Invalid,
        };
    }
}
impl<'a> ShrAssign<&'a EvalResult> for EvalResult {
    fn shr_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a >> (b.0 as usize)),
            _ => Invalid,
        };
    }
}
impl<'a> SubAssign<&'a EvalResult> for EvalResult {
    fn sub_assign(&mut self, rhs: &'a EvalResult) {
        use self::EvalResult::*;
        *self = match (&*self, rhs) {
            (&Int(a), &Int(b)) => Int(a - b),
            (&Float(a), &Int(b)) => Float(a - (b.0 as f64)),
            (&Int(a), &Float(b)) => Float(a.0 as f64 - b),
            (&Float(a), &Float(b)) => Float(a - b),
            _ => Invalid,
        };
    }
}

fn unary_op(input: (&[u8], EvalResult)) -> Option<EvalResult> {
    use self::EvalResult::*;
    assert_eq!(input.0.len(), 1);
    match (input.0[0], input.1) {
        (b'+', i) => Some(i),
        (b'-', Int(i)) => Some(Int(Wrapping(i.0.wrapping_neg()))), // impl Neg for Wrapping not until rust 1.10...
        (b'-', Float(i)) => Some(Float(-i)),
        (b'-', _) => unreachable!("non-numeric unary op"),
        (b'~', Int(i)) => Some(Int(!i)),
        (b'~', Float(_)) => None,
        (b'~', _) => unreachable!("non-numeric unary op"),
        _ => unreachable!("invalid unary op"),
    }
}

fn numeric<I: Clone, E: nom::error::ParseError<I>, F>(
    f: F,
) -> impl FnMut(I) -> nom::IResult<I, EvalResult, E>
where
    F: FnMut(I) -> nom::IResult<I, EvalResult, E>,
{
    nom::combinator::map_opt(f, EvalResult::as_numeric)
}

impl<'a> PRef<'a> {
    fn unary(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        alt((
            delimited(p("("), |i| self.numeric_expr(i), p(")")),
            numeric(|i| self.literal(i)),
            numeric(|i| self.identifier(i)),
            map_opt(
                pair(one_of_punctuation(&["+", "-", "~"][..]), |i| self.unary(i)),
                unary_op,
            ),
        ))(input)
    }

    fn mul_div_rem(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.unary(input)?;
        fold_many0(
            pair(complete(one_of_punctuation(&["*", "/", "%"][..])), |i| {
                self.unary(i)
            }),
            move || acc.clone(),
            |mut acc, (op, val): (&[u8], EvalResult)| {
                match op[0] as char {
                    '*' => acc *= &val,
                    '/' => acc /= &val,
                    '%' => acc %= &val,
                    _ => unreachable!(),
                };
                acc
            },
        )(input)
    }

    fn add_sub(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.mul_div_rem(input)?;
        fold_many0(
            pair(complete(one_of_punctuation(&["+", "-"][..])), |i| {
                self.mul_div_rem(i)
            }),
            move || acc.clone(),
            |mut acc, (op, val): (&[u8], EvalResult)| {
                match op[0] as char {
                    '+' => acc += &val,
                    '-' => acc -= &val,
                    _ => unreachable!(),
                };
                acc
            },
        )(input)
    }

    fn shl_shr(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.add_sub(input)?;
        numeric(fold_many0(
            pair(complete(one_of_punctuation(&["<<", ">>"][..])), |i| {
                self.add_sub(i)
            }),
            move || acc.clone(),
            |mut acc, (op, val): (&[u8], EvalResult)| {
                match op {
                    b"<<" => acc <<= &val,
                    b">>" => acc >>= &val,
                    _ => unreachable!(),
                };
                acc
            },
        ))(input)
    }

    fn and(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.shl_shr(input)?;
        numeric(fold_many0(
            preceded(complete(p("&")), |i| self.shl_shr(i)),
            move || acc.clone(),
            |mut acc, val: EvalResult| {
                acc &= &val;
                acc
            },
        ))(input)
    }

    fn xor(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.and(input)?;
        numeric(fold_many0(
            preceded(complete(p("^")), |i| self.and(i)),
            move || acc.clone(),
            |mut acc, val: EvalResult| {
                acc ^= &val;
                acc
            },
        ))(input)
    }

    fn or(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        let (input, acc) = self.xor(input)?;
        numeric(fold_many0(
            preceded(complete(p("|")), |i| self.xor(i)),
            move || acc.clone(),
            |mut acc, val: EvalResult| {
                acc |= &val;
                acc
            },
        ))(input)
    }

    #[inline(always)]
    fn numeric_expr(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        self.or(input)
    }
}

// =======================================================
// ============= Literals and identifiers ================
// =======================================================

impl<'a> PRef<'a> {
    fn identifier(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        match input.split_first() {
            None => Err(Err::Incomplete(Needed::new(1))),
            Some((
                &Token {
                    kind: TokenKind::Identifier,
                    ref raw,
                },
                rest,
            )) => {
                if let Some(r) = self.identifiers.get(&raw[..]) {
                    Ok((rest, r.clone()))
                } else {
                    Err(Err::Error(
                        (input, crate::ErrorKind::UnknownIdentifier).into(),
                    ))
                }
            }
            Some(_) => Err(Err::Error(
                (input, crate::ErrorKind::TypedToken(TokenKind::Identifier)).into(),
            )),
        }
    }

    fn literal(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        match input.split_first() {
            None => Err(Err::Incomplete(Needed::new(1))),
            Some((
                &Token {
                    kind: TokenKind::Literal,
                    ref raw,
                },
                rest,
            )) => match literal::parse(raw) {
                Ok((_, result)) => Ok((rest, result)),
                _ => Err(Err::Error((input, crate::ErrorKind::InvalidLiteral).into())),
            },
            Some(_) => Err(Err::Error(
                (input, crate::ErrorKind::TypedToken(TokenKind::Literal)).into(),
            )),
        }
    }

    fn string(self, input: &'_ [Token]) -> CResult<'_, Vec<u8>> {
        alt((
            map_opt(|i| self.literal(i), EvalResult::as_str),
            map_opt(|i| self.identifier(i), EvalResult::as_str),
        ))(input)
        .to_cexpr_result()
    }

    // "string1" "string2" etc...
    fn concat_str(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        map(
            pair(|i| self.string(i), many0(complete(|i| self.string(i)))),
            |(first, v)| {
                Vec::into_iter(v)
                    .fold(first, |mut s, elem| {
                        Vec::extend_from_slice(&mut s, Vec::<u8>::as_slice(&elem));
                        s
                    })
                    .into()
            },
        )(input)
        .to_cexpr_result()
    }

    fn expr(self, input: &'_ [Token]) -> CResult<'_, EvalResult> {
        alt((
            |i| self.numeric_expr(i),
            delimited(p("("), |i| self.expr(i), p(")")),
            |i| self.concat_str(i),
            |i| self.literal(i),
            |i| self.identifier(i),
        ))(input)
        .to_cexpr_result()
    }

    fn macro_definition(self, input: &'_ [Token]) -> CResult<'_, (&'_ [u8], EvalResult)> {
        pair(identifier_token, |i| self.expr(i))(input)
    }
}

impl<'a> ::std::ops::Deref for PRef<'a> {
    type Target = IdentifierParser<'a>;
    fn deref(&self) -> &IdentifierParser<'a> {
        self.0
    }
}

impl<'ident> IdentifierParser<'ident> {
    fn as_ref(&self) -> PRef<'_> {
        PRef(self)
    }

    /// Create a new `IdentifierParser` with a set of known identifiers. When
    /// a known identifier is encountered during parsing, it is substituted
    /// for the value specified.
    pub fn new(identifiers: &HashMap<Vec<u8>, EvalResult>) -> IdentifierParser<'_> {
        IdentifierParser { identifiers }
    }

    /// Parse and evaluate an expression of a list of tokens.
    ///
    /// Returns an error if the input is not a valid expression or if the token
    /// stream contains comments, keywords or unknown identifiers.
    pub fn expr<'a>(&self, input: &'a [Token]) -> CResult<'a, EvalResult> {
        self.as_ref().expr(input)
    }

    /// Parse and evaluate a macro definition from a list of tokens.
    ///
    /// Returns the identifier for the macro and its replacement evaluated as an
    /// expression. The input should not include `#define`.
    ///
    /// Returns an error if the replacement is not a valid expression, if called
    /// on most function-like macros, or if the token stream contains comments,
    /// keywords or unknown identifiers.
    ///
    /// N.B. This is intended to fail on function-like macros, but if it the
    /// macro takes a single argument, the argument name is defined as an
    /// identifier, and the macro otherwise parses as an expression, it will
    /// return a result even on function-like macros.
    ///
    /// ```c
    /// // will evaluate into IDENTIFIER
    /// #define DELETE(IDENTIFIER)
    /// // will evaluate into IDENTIFIER-3
    /// #define NEGATIVE_THREE(IDENTIFIER)  -3
    /// ```
    pub fn macro_definition<'a>(&self, input: &'a [Token]) -> CResult<'a, (&'a [u8], EvalResult)> {
        crate::assert_full_parse(self.as_ref().macro_definition(input))
    }
}

/// Parse and evaluate an expression of a list of tokens.
///
/// Returns an error if the input is not a valid expression or if the token
/// stream contains comments, keywords or identifiers.
pub fn expr(input: &[Token]) -> CResult<'_, EvalResult> {
    IdentifierParser::new(&HashMap::new()).expr(input)
}

/// Parse and evaluate a macro definition from a list of tokens.
///
/// Returns the identifier for the macro and its replacement evaluated as an
/// expression. The input should not include `#define`.
///
/// Returns an error if the replacement is not a valid expression, if called
/// on a function-like macro, or if the token stream contains comments,
/// keywords or identifiers.
pub fn macro_definition(input: &[Token]) -> CResult<'_, (&'_ [u8], EvalResult)> {
    IdentifierParser::new(&HashMap::new()).macro_definition(input)
}

/// Parse a functional macro declaration from a list of tokens.
///
/// Returns the identifier for the macro and the argument list (in order). The
/// input should not include `#define`. The actual definition is not parsed and
/// may be obtained from the unparsed data returned.
///
/// Returns an error if the input is not a functional macro or if the token
/// stream contains comments.
///
/// # Example
/// ```
/// use cexpr::expr::{IdentifierParser, EvalResult, fn_macro_declaration};
/// use cexpr::assert_full_parse;
/// use cexpr::token::Kind::*;
/// use cexpr::token::Token;
///
/// // #define SUFFIX(arg) arg "suffix"
/// let tokens = vec![
///     (Identifier,  &b"SUFFIX"[..]).into(),
///     (Punctuation, &b"("[..]).into(),
///     (Identifier,  &b"arg"[..]).into(),
///     (Punctuation, &b")"[..]).into(),
///     (Identifier,  &b"arg"[..]).into(),
///     (Literal,     &br#""suffix""#[..]).into(),
/// ];
///
/// // Try to parse the functional part
/// let (expr, (ident, args)) = fn_macro_declaration(&tokens).unwrap();
/// assert_eq!(ident, b"SUFFIX");
///
/// // Create dummy arguments
/// let idents = args.into_iter().map(|arg|
///     (arg.to_owned(), EvalResult::Str(b"test".to_vec()))
/// ).collect();
///
/// // Evaluate the macro
/// let (_, evaluated) = assert_full_parse(IdentifierParser::new(&idents).expr(expr)).unwrap();
/// assert_eq!(evaluated, EvalResult::Str(b"testsuffix".to_vec()));
/// ```
pub fn fn_macro_declaration(input: &[Token]) -> CResult<'_, (&[u8], Vec<&[u8]>)> {
    pair(
        identifier_token,
        delimited(
            p("("),
            separated_list0(p(","), identifier_token),
            p(")"),
        ),
    )(input)
}
