use crate::combinator::empty;
use crate::combinator::fail;
use crate::combinator::opt;
use crate::combinator::trace;
use crate::error::ParserError;
use crate::stream::Stream;
use crate::stream::StreamIsPartial;
use crate::Parser;
use crate::Result;

/// Parses an expression based on operator precedence.
///
/// It uses a Pratt parsing algorithm, where operators are
/// associated with a binding power. The higher the power,
/// the more tightly an operator will bind to its operands.
///
/// This method returns an [`Expression`], which configures
/// the Pratt parser.
///
/// Each operator type is configured with [`Prefix`], [`Postfix`],
/// and [`Infix`]. These describe the operator's binding power,
/// a function that applies the operator to its operand, and the
/// operator's associativity (infix only).
///
/// For a more full-featured example, look at the [C-style Expressions][crate::_topic::language#c-style-expressions]
/// topic.
///
/// # Example
///
/// Parsing a simple arithmetic expression without parenthesis.
///
/// ```rust
/// # #[cfg(feature = "ascii")] {
/// # use winnow::prelude::*;
/// # use winnow::error::ContextError;
/// # use winnow::ascii::digit1;
/// # use winnow::combinator::{dispatch, fail};
/// # use winnow::token::any;
/// use winnow::combinator::expression;
/// use winnow::combinator::{Prefix, Postfix, Infix};
///
/// fn parser<'i>() -> impl Parser<&'i str, i32, ContextError> {
///     move |i: &mut &str| {
///         use Infix::*;
///         expression(digit1.parse_to::<i32>()) // operands are 32-bit integers
///             .prefix(dispatch! {any;
///                 '-' => Prefix(12, |_, a: i32| Ok(-a)),
///                 _ => fail,
///             })
///             .infix(dispatch! {any;
///                 '+' => Left(5, |_, a, b| Ok(a + b)),
///                 '-' => Left(5, |_, a, b| Ok(a - b)),
///                 '*' => Left(7, |_, a, b| Ok(a * b)),
///                 '/' => Left(7, |_, a: i32, b| Ok(a.checked_div(b).unwrap_or_default())),
///                 _ => fail,
///             })
///             .postfix(dispatch! {any;
///                 '!' => Postfix(15, |_, a| if a < 1 { Ok(1) } else { Ok((1..=a).fold(1, |acc, a| acc*a)) }),
///                 _ => fail,
///             })
///             .parse_next(i)
///     }
/// }
///
/// assert_eq!(parser().parse("1+1"), Ok(2));
/// assert_eq!(parser().parse("0!"), Ok(1));
/// assert_eq!(parser().parse("-1*5*2*10+30/3!"), Ok(-95));
/// # }
/// ```
#[doc(alias = "pratt")]
#[doc(alias = "separated")]
#[doc(alias = "shunting_yard")]
#[doc(alias = "precedence_climbing")]
#[inline(always)]
#[allow(clippy::type_complexity)]
pub fn expression<I, ParseOperand, O, E>(
    parse_operand: ParseOperand,
) -> Expression<
    I,
    O,
    ParseOperand,
    impl Parser<I, Prefix<I, O, E>, E>,
    impl Parser<I, Postfix<I, O, E>, E>,
    impl Parser<I, Infix<I, O, E>, E>,
    E,
>
where
    I: Stream + StreamIsPartial,
    ParseOperand: Parser<I, O, E>,
    E: ParserError<I>,
{
    Expression {
        precedence_level: 0,
        parse_operand,
        parse_prefix: fail,
        parse_postfix: fail,
        parse_infix: fail,
        marker: Default::default(),
    }
}

/// A helper struct for [`expression()`].
///
/// Holds the configuration for the Pratt parser, including
/// the operator and operand parsers. A precedence level can
/// also be set, which is useful to disambiguate parse trees
/// based on the parent operator's precedence.
///
/// Implements [`Parser`]. When parsing an input, it applies
/// the Pratt parser.
pub struct Expression<I, O, ParseOperand, Pre, Post, Pix, E>
where
    I: Stream + StreamIsPartial,
    ParseOperand: Parser<I, O, E>,
    E: ParserError<I>,
{
    precedence_level: i64,
    parse_operand: ParseOperand,
    parse_prefix: Pre,
    parse_postfix: Post,
    parse_infix: Pix,
    marker: core::marker::PhantomData<(I, O, E)>,
}

impl<I, O, ParseOperand, Pre, Post, Pix, E> Expression<I, O, ParseOperand, Pre, Post, Pix, E>
where
    ParseOperand: Parser<I, O, E>,
    I: Stream + StreamIsPartial,
    E: ParserError<I>,
{
    /// Sets the prefix operator parser.
    ///
    /// The parser should parse the input to a [`Prefix`],
    /// which contains the operator's binding power and
    /// a fold function which applies the operator to its
    /// operands.
    #[inline(always)]
    pub fn prefix<NewParsePrefix>(
        self,
        parser: NewParsePrefix,
    ) -> Expression<I, O, ParseOperand, NewParsePrefix, Post, Pix, E>
    where
        NewParsePrefix: Parser<I, Prefix<I, O, E>, E>,
    {
        Expression {
            precedence_level: self.precedence_level,
            parse_operand: self.parse_operand,
            parse_prefix: parser,
            parse_postfix: self.parse_postfix,
            parse_infix: self.parse_infix,
            marker: Default::default(),
        }
    }

    /// Sets the postfix operator parser.
    ///
    /// The parser should parse the input to a [`Postfix`],
    /// which contains the operator's binding power and
    /// a fold function which applies the operator to its
    /// operands.
    #[inline(always)]
    pub fn postfix<NewParsePostfix>(
        self,
        parser: NewParsePostfix,
    ) -> Expression<I, O, ParseOperand, Pre, NewParsePostfix, Pix, E>
    where
        NewParsePostfix: Parser<I, Postfix<I, O, E>, E>,
    {
        Expression {
            precedence_level: self.precedence_level,
            parse_operand: self.parse_operand,
            parse_prefix: self.parse_prefix,
            parse_postfix: parser,
            parse_infix: self.parse_infix,
            marker: Default::default(),
        }
    }

    /// Sets the infix operator parser.
    ///
    /// The parser should parse the input to a [`Infix`],
    /// which contains the operator's binding power and
    /// a fold function which applies the operator to its
    /// operands.
    #[inline(always)]
    pub fn infix<NewParseInfix>(
        self,
        parser: NewParseInfix,
    ) -> Expression<I, O, ParseOperand, Pre, Post, NewParseInfix, E>
    where
        NewParseInfix: Parser<I, Infix<I, O, E>, E>,
    {
        Expression {
            precedence_level: self.precedence_level,
            parse_operand: self.parse_operand,
            parse_prefix: self.parse_prefix,
            parse_postfix: self.parse_postfix,
            parse_infix: parser,
            marker: Default::default(),
        }
    }

    /// Sets the precedence level for the current instance of the parser.
    ///
    /// It defaults to 0, which is traditionally treated as the "lowest"
    /// possible precedence when parsing an expression.
    ///
    /// This is useful to disambiguate grammars based on the parent operator's
    /// precedence. This comes up primarily when parsing recursive expressions.
    ///
    /// The parsing machinery underpinning [`Expression`] assumes that a "more
    /// tightly binding" operator is numerically large, while a "more loosely
    /// binding" operator is numerically small. For example, `13` is a higher
    /// precedence level than `1` because `13 > 1`.
    ///
    /// Other ways of describing this relationship:
    /// - `13` has a higher precedence compared to `1`
    /// - `13` has a higher binding power compared to `1`
    ///
    /// Note: Binding power and precedence both refer to the same concept and
    /// may be used interchangeably.
    ///
    /// # Motivation
    ///
    /// If you don't understand why this is useful to have, this section tries
    /// to explain in more detail.
    ///
    /// The [C-style Expressions][crate::_topic::arithmetic#c-style-expression]
    /// example has source code for parsing the expression described below, and
    /// can provide a clearer usage example.
    ///
    /// Consider the following expression in the C language:
    ///
    /// ```c
    /// int x = (1 == 1 ? 0 : 1, -123); // <-- let's parse this
    /// printf("%d\n", x); // -123
    /// ```
    ///
    /// Let's look at the right-hand side of the expression on the first line,
    /// and replace some of the sub-expressions with symbols:
    ///
    /// ```text
    /// (1 == 1 ? 0 : 1, -123) // rhs
    /// (a      ? b : c, d  )  // symbolic
    /// (a ? b : c, d)         // remove whitespace
    /// (, (? a b c) d)        // prefix notation
    /// ```
    ///
    /// Written symbolically:
    /// - `a` is the condition, like `1 == 1`
    /// - `b` is the value when the condition is true
    /// - `c` is the value when the condition is false
    /// - `d` is a secondary expression unrelated to the ternary
    ///
    /// In prefix notation, it's easier to see the specific operators and what
    /// they bind to:
    /// - COMMA (`,`) binds to `(? a b c)` and `d`
    /// - TERNARY (`?`) binds to `a`, `b`, and `c`
    ///
    /// ## Parsing `c` and `d`
    ///
    /// Let's focus on parsing the sub-expressions `c` and `d`, as that
    /// motivates why a parser precedence level is necessary.
    ///
    /// To parse `c`, we would really like to re-use the parser produced by
    /// [`expression()`], because `c` is really *any* valid expression that
    /// can be parsed by `expression()` already.
    ///
    /// However, we can't re-use the parser naively. When parsing `c`, we need
    /// to "escape" from the inner parser when encountering the comma separating
    /// `c` from `d`.
    ///
    /// The reason we have to "escape" is because of how operator precedence is
    /// defined in the C language: the comma operator has the lowest precedence
    /// among all the operators. When we're parsing `c`, we're in the context of
    /// the ternary operator. We don't want to parse any valid expression! Just
    /// what the ternary operator captures.
    ///
    /// That's where the precedence level comes in: you specify the minimum
    /// precedence this parser is willing to accept. If you come across an
    /// expression in the top-level with a lower binding power than the starting
    /// precedence, you know to stop parsing.
    ///
    /// The parsing machinery inside of [`Expression`] handles most of this for
    /// you, but it can't determine what the precedence level should be for a
    /// given expression. That's a language-specific detail, and it depends on
    /// what you want to parse.
    #[inline(always)]
    pub fn current_precedence_level(
        mut self,
        level: i64,
    ) -> Expression<I, O, ParseOperand, Pre, Post, Pix, E> {
        self.precedence_level = level;
        self
    }
}

impl<I, O, Pop, Pre, Post, Pix, E> Parser<I, O, E> for Expression<I, O, Pop, Pre, Post, Pix, E>
where
    I: Stream + StreamIsPartial,
    Pop: Parser<I, O, E>,
    Pix: Parser<I, Infix<I, O, E>, E>,
    Pre: Parser<I, Prefix<I, O, E>, E>,
    Post: Parser<I, Postfix<I, O, E>, E>,
    E: ParserError<I>,
{
    #[inline(always)]
    fn parse_next(&mut self, input: &mut I) -> Result<O, E> {
        trace("expression", move |i: &mut I| {
            expression_impl(
                i,
                &mut self.parse_operand,
                &mut self.parse_prefix,
                &mut self.parse_postfix,
                &mut self.parse_infix,
                self.precedence_level,
            )
        })
        .parse_next(input)
    }
}

/// Opaque implementation of the Pratt parser.
fn expression_impl<I, O, Pop, Pre, Post, Pix, E>(
    i: &mut I,
    parse_operand: &mut Pop,
    prefix: &mut Pre,
    postfix: &mut Post,
    infix: &mut Pix,
    min_power: i64,
) -> Result<O, E>
where
    I: Stream + StreamIsPartial,
    Pop: Parser<I, O, E>,
    Pix: Parser<I, Infix<I, O, E>, E>,
    Pre: Parser<I, Prefix<I, O, E>, E>,
    Post: Parser<I, Postfix<I, O, E>, E>,
    E: ParserError<I>,
{
    let operand = opt(trace("operand", parse_operand.by_ref())).parse_next(i)?;
    let mut operand = if let Some(operand) = operand {
        operand
    } else {
        // Prefix unary operators
        let len = i.eof_offset();
        let Prefix(power, fold_prefix) = trace("prefix", prefix.by_ref()).parse_next(i)?;
        // infinite loop check: the parser must always consume
        if i.eof_offset() == len {
            return Err(E::assert(i, "`prefix` parsers must always consume"));
        }
        let operand = expression_impl(i, parse_operand, prefix, postfix, infix, power)?;
        fold_prefix(i, operand)?
    };

    // A variable to stop the `'parse` loop when `Assoc::Neither` with the same
    // precedence is encountered e.g. `a == b == c`. `Assoc::Neither` has similar
    // associativity rules as `Assoc::Left`, but we stop parsing when the next operator
    // is the same as the current one.
    let mut prev_op_is_neither = None;
    'parse: while i.eof_offset() > 0 {
        // Postfix unary operators
        let start = i.checkpoint();
        if let Some(Postfix(power, fold_postfix)) =
            opt(trace("postfix", postfix.by_ref())).parse_next(i)?
        {
            // control precedence over the prefix e.g.:
            // `--(i++)` or `(--i)++`
            if power < min_power {
                i.reset(&start);
                break 'parse;
            }
            operand = fold_postfix(i, operand)?;

            continue 'parse;
        }

        // Infix binary operators
        let start = i.checkpoint();
        let parse_result = opt(trace("infix", infix.by_ref())).parse_next(i)?;
        if let Some(infix_op) = parse_result {
            let mut is_neither = None;
            let (lpower, rpower, fold_infix) = match infix_op {
                Infix::Right(p, f) => (p, p - 1, f),
                Infix::Left(p, f) => (p, p + 1, f),
                Infix::Neither(p, f) => {
                    is_neither = Some(p);
                    (p, p + 1, f)
                }
            };
            if lpower < min_power
                // MSRV: `is_some_and`
                || match prev_op_is_neither {
                    None => false,
                    Some(p) => lpower == p,
                }
            {
                i.reset(&start);
                break 'parse;
            }
            prev_op_is_neither = is_neither;
            let rhs = expression_impl(i, parse_operand, prefix, postfix, infix, rpower)?;
            operand = fold_infix(i, operand, rhs)?;

            continue 'parse;
        }

        break 'parse;
    }

    Ok(operand)
}

/// Define an [`expression()`]'s prefix operator
///
/// It requires an operator binding power, as well as a
/// fold function which applies the operator.
pub struct Prefix<I, O, E>(
    /// Binding power
    pub i64,
    /// Unary operator
    pub fn(&mut I, O) -> Result<O, E>,
);

impl<I, O, E> Clone for Prefix<I, O, E> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Prefix(self.0, self.1)
    }
}

impl<I: Stream, O, E: ParserError<I>> Parser<I, Prefix<I, O, E>, E> for Prefix<I, O, E> {
    #[inline(always)]
    fn parse_next(&mut self, input: &mut I) -> Result<Prefix<I, O, E>, E> {
        empty.value(self.clone()).parse_next(input)
    }
}

/// Define an [`expression()`]'s postfix operator
///
/// It requires an operator binding power, as well as a
/// fold function which applies the operator.
pub struct Postfix<I, O, E>(
    /// Binding power
    pub i64,
    /// Unary operator
    pub fn(&mut I, O) -> Result<O, E>,
);

impl<I, O, E> Clone for Postfix<I, O, E> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Postfix(self.0, self.1)
    }
}

impl<I: Stream, O, E: ParserError<I>> Parser<I, Postfix<I, O, E>, E> for Postfix<I, O, E> {
    #[inline(always)]
    fn parse_next(&mut self, input: &mut I) -> Result<Postfix<I, O, E>, E> {
        empty.value(self.clone()).parse_next(input)
    }
}

/// Define an [`expression()`]'s infix operator
///
/// It requires an operator binding power, as well as a
/// fold function which applies the operator.
pub enum Infix<I, O, E> {
    /// Left-associative operator
    ///
    /// The operators will bind more tightly to their rightmost operands.
    ///
    /// e.g `A op B op C` -> `(A op B) op C`
    Left(
        /// Binding power
        i64,
        /// Binary operator
        fn(&mut I, O, O) -> Result<O, E>,
    ),
    /// Right-associative operator
    ///
    /// The operators will bind more tightly to their leftmost operands.
    ///
    /// e.g `A op B op C` -> `A op (B op C)`
    Right(
        /// Binding power
        i64,
        /// Binary operator
        fn(&mut I, O, O) -> Result<O, E>,
    ),
    /// Neither left or right associative
    ///
    /// `Infix::Neither` has similar associativity rules as `Assoc::Left`, but we stop
    /// parsing when the next operator is the same as the current one.
    ///
    /// e.g. `a == b == c` -> `(a == b)`, fail: `(== c)`
    Neither(
        /// Binding power
        i64,
        /// Binary operator
        fn(&mut I, O, O) -> Result<O, E>,
    ),
}

impl<I, O, E> Clone for Infix<I, O, E> {
    #[inline(always)]
    fn clone(&self) -> Self {
        match self {
            Infix::Left(p, f) => Infix::Left(*p, *f),
            Infix::Right(p, f) => Infix::Right(*p, *f),
            Infix::Neither(p, f) => Infix::Neither(*p, *f),
        }
    }
}

impl<I: Stream, O, E: ParserError<I>> Parser<I, Infix<I, O, E>, E> for Infix<I, O, E> {
    #[inline(always)]
    fn parse_next(&mut self, input: &mut I) -> Result<Infix<I, O, E>, E> {
        empty.value(self.clone()).parse_next(input)
    }
}

#[cfg(all(test, feature = "ascii"))]
mod tests {
    use crate::ascii::digit1;
    use crate::combinator::fail;
    use crate::dispatch;
    use crate::error::ContextError;
    use crate::token::any;

    use super::*;

    fn parser<'i>() -> impl Parser<&'i str, i32, ContextError> {
        move |i: &mut &str| {
            use Infix::*;
            expression(digit1.parse_to::<i32>())
                .current_precedence_level(0)
                .prefix(dispatch! {any;
                    '+' => Prefix(12, |_, a| Ok(a)),
                    '-' => Prefix(12, |_, a: i32| Ok(-a)),
                    _ => fail
                })
                .infix(dispatch! {any;
                   '+' => Left(5, |_, a, b| Ok(a + b)),
                   '-' => Left(5, |_, a, b| Ok(a - b)),
                   '*' => Left(7, |_, a, b| Ok(a * b)),
                   '/' => Left(7, |_, a, b| Ok(a / b)),
                   '%' => Left(7, |_, a, b| Ok(a % b)),
                   '^' => Left(9, |_, a, b| Ok(a ^ b)),
                   _ => fail
                })
                .parse_next(i)
        }
    }

    #[test]
    fn test_expression() {
        assert_eq!(parser().parse("-3+-3*4"), Ok(-15));
        assert_eq!(parser().parse("+2+3*4"), Ok(14));
        assert_eq!(parser().parse("2*3+4"), Ok(10));
    }
}
