//! Main points of the `c_expression` example:
//!
//! 1. [`Expr`], representing the AST of C-style expressions
//! 2. [`pratt_parser()`], the core parser.
//! 3. The [`test`]s, which demonstrate the expected input/outputs.
//!
//! There is one [`parse_example()`] function in this file, but the
//! associated test module for this example shows the other inputs/outputs.
//!
//! # Errata:
//! - There is a helper parser, [`identifier()`]
//! - Two print implementations: [`Expr::fmt_ast_with_indent()`] and [`Expr::fmt_delimited()`]
//! - For operator precedence, `1` has a low binding power while `13` has a high binding power.
//!
//! ## Printing
//!
//! `fmt_delimited()` is essentially prefix-notation.
//!
//! A short visual about the differences in printing, for parsing `1 + 1`:
//!
//! `fmt_ast_with_indent()` =
//! ~~~text
//! ADD
//!  VAL 1
//!  VAL 1
//! ~~~
//!
//! `fmt_delimited()` =
//! ~~~text
//! (+ 1 1)
//! ~~~
//!
//! ## Precedences
//!
//! Values uses an *inverted form* of the [C language operator precedence](c-precedence).
//!
//! In our implementation, a precedence of `1` is a very low precedence compared
//! to a value of `13`.
//!
//! In the linked C operator page, the table shows precedence in *descending precedence order*,
//! so `1` is the highest precedence, while `13` is a low precedence.
//!
//! The precedence levels you have to choose for your grammar depend on the language's
//! semantics.
//!
//! [c-precedence]: https://en.cppreference.com/w/c/language/operator_precedence.html
//!
//! ### Precedence Table
//!
//! An overview of the different operators for this C-style expression language.
//!
//! Note: this does not include the operands themselves, such as literals or
//! parenthesized expressions like: `(x)`
//!
//! Legend:
//! - Kind: one of Prefix, Postfix, or Infix
//! - Example: input text example
//! - Name: the [`Expr`] variant it corresponds to
//! - Power: the binding power/precedence of the operator
//! - Assoc: infix-only, represents the left/right associativity of an operator
//! - Recursive: only "TRUE" if parsing the operand invokes the parser again
//!
//! | Kind    | Example    | Name             | Power | Assoc   | Recursive |
//! |---------|------------|------------------|-------|---------|-----------|
//! | Prefix  | `++x`      | PreIncr          | 18    |         |           |
//! | Prefix  | `+x`       | Positive (no-op) | 18    |         |           |
//! | Prefix  | `--x`      | PreDecr          | 18    |         |           |
//! | Prefix  | `-x`       | Neg              | 18    |         |           |
//! | Prefix  | `&x`       | Addr             | 18    |         |           |
//! | Prefix  | `*x`       | Deref            | 18    |         |           |
//! | Prefix  | `!x`       | Not              | 18    |         |           |
//! | Prefix  | `~x`       | BitwiseNot       | 18    |         |           |
//! | Postfix | `x!`       | Fac              | 19    |         |           |
//! | Postfix | `x?`       | Ternary          | 3     |         | TRUE      |
//! | Postfix | `[x]`      | Index            | 20    |         | TRUE      |
//! | Postfix | `(x)`      | Function call    | 20    |         | TRUE      |
//! | Postfix | `x++`      | PostIncr         | 20    |         |           |
//! | Postfix | `x--`      | PostDecr         | 20    |         |           |
//! | Infix   | `a ** b`   | Pow              | 28    | Right   |           |
//! | Infix   | `a * b`    | Mul              | 16    | Left    |           |
//! | Infix   | `a / b`    | Div              | 16    | Left    |           |
//! | Infix   | `a % b`    | Rem              | 16    | Left    |           |
//! | Infix   | `a + b`    | Add              | 14    | Left    |           |
//! | Infix   | `a -ne b`  | NotEq            | 10    | Neither |           |
//! | Infix   | `a -eq b`  | Eq               | 10    | Neither |           |
//! | Infix   | `a -gt b`  | Greater          | 12    | Neither |           |
//! | Infix   | `a -ge b`  | GreaterEq        | 12    | Neither |           |
//! | Infix   | `a -lt b`  | Less             | 12    | Neither |           |
//! | Infix   | `a -le b`  | LessEqual        | 12    | Neither |           |
//! | Infix   | `a->b`     | ArrowOp          | 20    | Left    |           |
//! | Infix   | `a - b`    | Sub              | 14    | Left    |           |
//! | Infix   | `a.b`      | Dot              | 20    | Left    |           |
//! | Infix   | `a && b`   | And              | 6     | Left    |           |
//! | Infix   | `a & b`    | BitAnd           | 12    | Left    |           |
//! | Infix   | `a ^ b`    | BitXor           | 8     | Left    |           |
//! | Infix   | `a == b`   | Eq               | 10    | Neither |           |
//! | Infix   | `a = b`    | Assign           | 2     | Right   |           |
//! | Infix   | `a >= b`   | GreaterEq        | 12    | Neither |           |
//! | Infix   | `a > b`    | Greater          | 12    | Neither |           |
//! | Infix   | `a <= b`   | LessEqual        | 12    | Neither |           |
//! | Infix   | `a < b`    | Less             | 12    | Neither |           |
//! | Infix   | `a, b`     | Comma            | 0     | Left    |           |
//! | Infix   | `a != b`   | NotEq            | 10    | Neither |           |
//! | Infix   | `a \|\| b` | Or               | 4     | Left    |           |

use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{
    alt, cut_err, delimited, expression, fail, not, opt, peek, separated_pair, trace,
};
use winnow::combinator::{Infix, Postfix, Prefix};
use winnow::dispatch;
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::token::{any, one_of, take, take_while};

#[test]
fn parse_example() {
    // Check out the [`crate::parser::test`] in this example for more samples.
    let result = pratt_parser
        .parse("a*a + b - c")
        .map(|r| format!("{}", r))
        .unwrap();

    let expect = "\
SUB
  ADD
    MUL
      NAME a
      NAME a
    NAME b
  NAME c

(- (+ (* a a) b) c)";

    assert_eq!(result, expect);
}

// Abstract syntax tree for an expression
pub(crate) enum Expr {
    Name(String),
    Value(i64),

    Assign(Box<Expr>, Box<Expr>),

    Addr(Box<Expr>),
    Deref(Box<Expr>),

    Dot(Box<Expr>, Box<Expr>),
    ArrowOp(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Fac(Box<Expr>),

    PreIncr(Box<Expr>),
    PostIncr(Box<Expr>),
    PreDecr(Box<Expr>),
    PostDecr(Box<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    // `==`
    Eq(Box<Expr>, Box<Expr>),
    // `!=`
    NotEq(Box<Expr>, Box<Expr>),
    // `!`
    Not(Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),

    // A parenthesized expression.
    Paren(Box<Expr>),
    FunctionCall(Box<Expr>, Option<Box<Expr>>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    // foo[...]
    Index(Box<Expr>, Box<Expr>),
    // a, b
    Comma(Box<Expr>, Box<Expr>),

    // %
    Rem(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitwiseNot(Box<Expr>),
}

/// Parser definition
///
/// We define a helper function `parser()` and call it at the very end.
///
/// `parser()` accepts a minimum `precedence_level` for the entire expression,
/// and it returns a [`Parser`] that takes in a string and returns an [`Expr`] on
/// success.
pub(crate) fn pratt_parser(i: &mut &str) -> ModalResult<Expr> {
    fn parser<'i>(precedence: i64) -> impl Parser<&'i str, Expr, ErrMode<ContextError>> {
        move |i: &mut &str| {
            use Infix::{Left, Neither, Right};
            expression(
                // parsing an operand, optionally surrounded by whitespace
                delimited(
                    multispace0,
                    dispatch! {peek(any);
                        // Case one: Parenthesized expression
                        '(' => delimited('(',  parser(0).map(|e| Expr::Paren(Box::new(e))), cut_err(')')),
                        _ => alt((
                            // Case two: A C-style identifier
                            identifier.map(|s| Expr::Name(s.into())),
                            // Case three: An integer
                            digit1.parse_to::<i64>().map(Expr::Value)
                        )),
                    },
                    multispace0,
                )
            )
            .current_precedence_level(precedence)
            .prefix(
                // parsing prefix operators, optionally surrounded by whitespace
                // for example: `++1`
                //               ||^ operand, called `a` below
                //               ^^ prefix 'pre-increment' operator
                delimited(
                    multispace0,
                    dispatch! {any;
                        '+' => alt((
                            // ++
                            '+'.value(Prefix(18, |_: &mut _, a| Ok(Expr::PreIncr(Box::new(a))))),
                            Prefix(18, |_: &mut _, a| Ok(a) )
                        )),
                        '-' =>  alt((
                            // --
                            '-'.value(Prefix(18, |_: &mut _, a| Ok(Expr::PreDecr(Box::new(a))))),
                            Prefix(18, |_: &mut _, a| Ok(Expr::Neg(Box::new(a))))
                        )),
                        '&' => Prefix(18, |_: &mut _, a| Ok(Expr::Addr(Box::new(a)))),
                        '*' => Prefix(18, |_: &mut _, a| Ok(Expr::Deref(Box::new(a)))),
                        '!' => Prefix(18, |_: &mut _, a| Ok(Expr::Not(Box::new(a)))),
                        '~' => Prefix(18, |_: &mut _, a| Ok(Expr::BitwiseNot(Box::new(a)))),
                        _ => fail
                    },
                    multispace0,
                )
            )
            .postfix(
                // parsing postfix operators, optionally surrounded by whitespace
                // for example: `(1 == 1) ? 100 : 0`
                //               |||||||| | \\\\\\\\___ remainder of the input, called `i` below
                //               |||||||| ^ postfix ternary operator
                //               ^^^^^^^^ operand, called `cond` below
                delimited(
                    multispace0,
                    alt((
                        dispatch! {any;
                            '!' => not('=').value(Postfix(19, |_: &mut _, a| Ok(Expr::Fac(Box::new(a))))),
                            '?' => Postfix(3, |i: &mut &str, cond| {
                                let (left, right) = cut_err(separated_pair(parser(0), delimited(multispace0, ':', multispace0), parser(3))).parse_next(i)?;
                                Ok(Expr::Ternary(Box::new(cond), Box::new(left), Box::new(right)))
                            }),
                            '[' => Postfix(20, |i: &mut &str, a| {
                                let index = delimited(multispace0, parser(0), (multispace0, cut_err(']'), multispace0)).parse_next(i)?;
                                Ok(Expr::Index(Box::new(a), Box::new(index)))
                            }),
                            '(' => Postfix(20, |i: &mut &str, a| {
                                let args = delimited(multispace0, opt(parser(0)), (multispace0, cut_err(')'), multispace0)).parse_next(i)?;
                                Ok(Expr::FunctionCall(Box::new(a), args.map(Box::new)))
                            }),
                            _ => fail,
                        },
                        dispatch! {take(2usize);
                            "++" => Postfix(20, |_: &mut _, a| Ok(Expr::PostIncr(Box::new(a)))),
                            "--" => Postfix(20, |_: &mut _, a| Ok(Expr::PostDecr(Box::new(a)))),
                            _ => fail,
                        },
                    )),
                    multispace0,
                ),
            )
            .infix(
                // parsing infix operators
                //
                // we do not need to deal with whitespace here, it would be redundant
                // since the operands already ignore whitespace
                //
                // an example infix operator: `123 + 456`
                //         operand, called `a` ^^^ | ^^^ operand, called `b` below
                //                                 |
                //                                 ^ infix addition operator
                alt((
                    dispatch! {any;
                        '*' => alt((
                            // **
                            "*".value(Right(28, |_: &mut _, a, b| Ok(Expr::Pow(Box::new(a), Box::new(b))))),
                            Left(16, |_: &mut _, a, b| Ok(Expr::Mul(Box::new(a), Box::new(b)))),
                        )),
                        '/' => Left(16, |_: &mut _, a, b| Ok(Expr::Div(Box::new(a), Box::new(b)))),
                        '%' => Left(16, |_: &mut _, a, b| Ok(Expr::Rem(Box::new(a), Box::new(b)))),

                        '+' => Left(14, |_: &mut _, a, b| Ok(Expr::Add(Box::new(a), Box::new(b)))),
                        '-' => alt((
                            dispatch!{take(2usize);
                                "ne" => Neither(10, |_: &mut _, a, b| Ok(Expr::NotEq(Box::new(a), Box::new(b)))),
                                "eq" => Neither(10, |_: &mut _, a, b| Ok(Expr::Eq(Box::new(a), Box::new(b)))),
                                "gt" => Neither(12, |_: &mut _, a, b| Ok(Expr::Greater(Box::new(a), Box::new(b)))),
                                "ge" => Neither(12, |_: &mut _, a, b| Ok(Expr::GreaterEqual(Box::new(a), Box::new(b)))),
                                "lt" => Neither(12, |_: &mut _, a, b| Ok(Expr::Less(Box::new(a), Box::new(b)))),
                                "le" => Neither(12, |_: &mut _, a, b| Ok(Expr::LessEqual(Box::new(a), Box::new(b)))),
                                _ => fail
                            },
                            '>'.value(Left(20, |_: &mut _, a, b| Ok(Expr::ArrowOp(Box::new(a), Box::new(b))))),
                            Left(14, |_: &mut _, a, b| Ok(Expr::Sub(Box::new(a), Box::new(b))))
                        )),
                        '.' => Left(20, |_: &mut _, a, b| Ok(Expr::Dot(Box::new(a), Box::new(b)))),
                        '&' => alt((
                            // &&
                            "&".value(Left(6, |_: &mut _, a, b| Ok(Expr::And(Box::new(a), Box::new(b))))),

                            Left(12, |_: &mut _, a, b| Ok(Expr::BitAnd(Box::new(a), Box::new(b)))),
                        )),
                        '^' => Left(8, |_: &mut _, a, b| Ok(Expr::BitXor(Box::new(a), Box::new(b)))),
                        '=' => alt((
                            // ==
                            "=".value(Neither(10, |_: &mut _, a, b| Ok(Expr::Eq(Box::new(a), Box::new(b))))),
                            Right(2, |_: &mut _, a, b| Ok(Expr::Assign(Box::new(a), Box::new(b))))
                        )),

                        '>' => alt((
                            // >=
                            "=".value(Neither(12, |_: &mut _, a, b| Ok(Expr::GreaterEqual(Box::new(a), Box::new(b))))),
                            Neither(12, |_: &mut _, a, b| Ok(Expr::Greater(Box::new(a), Box::new(b))))
                        )),
                        '<' => alt((
                            // <=
                            "=".value(Neither(12, |_: &mut _, a, b| Ok(Expr::LessEqual(Box::new(a), Box::new(b))))),
                            Neither(12, |_: &mut _, a, b| Ok(Expr::Less(Box::new(a), Box::new(b))))
                        )),
                        ',' => Left(0, |_: &mut _, a, b| Ok(Expr::Comma(Box::new(a), Box::new(b)))),
                        _ => fail
                    },
                    dispatch! {take(2usize);
                        "!=" => Neither(10, |_: &mut _, a, b| Ok(Expr::NotEq(Box::new(a), Box::new(b)))),
                        "||" => Left(4, |_: &mut _, a, b| Ok(Expr::Or(Box::new(a), Box::new(b)))),
                        _ => fail
                    },
                )),
            )
            .parse_next(i)
        }
    }

    parser(0).parse_next(i)
}

// Helper parsers

fn identifier<'i>(i: &mut &'i str) -> ModalResult<&'i str> {
    trace(
        "identifier",
        (
            one_of(|c: char| c.is_alpha() || c == '_'),
            take_while(0.., |c: char| c.is_alphanum() || c == '_'),
        ),
    )
    .take()
    .parse_next(i)
}

// Formatted print implementations
impl Expr {
    fn fmt_ast_with_indent(
        &self,
        indent: u32,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        for _ in 0..indent {
            write!(f, "  ")?;
        }
        macro_rules! binary_fmt {
            ($a:ident, $b:ident, $name:literal) => {{
                writeln!(f, $name)?;
                $a.fmt_ast_with_indent(indent + 1, f)?;
                $b.fmt_ast_with_indent(indent + 1, f)
            }};
        }
        macro_rules! unary_fmt {
            ($a:ident, $name:literal) => {{
                writeln!(f, $name)?;
                $a.fmt_ast_with_indent(indent + 1, f)
            }};
        }
        match self {
            Self::Name(name) => writeln!(f, "NAME {name}"),
            Self::Value(value) => writeln!(f, "VAL {value}"),
            Self::Addr(a) => unary_fmt!(a, "ADDR"),
            Self::Deref(a) => unary_fmt!(a, "DEREF"),
            Self::Neg(a) => unary_fmt!(a, "NEG"),
            Self::Fac(a) => unary_fmt!(a, "FAC"),
            Self::PreIncr(a) => unary_fmt!(a, "PRE_INCR"),
            Self::PostIncr(a) => unary_fmt!(a, "POST_INCR"),
            Self::PreDecr(a) => unary_fmt!(a, "PRE_DECR"),
            Self::PostDecr(a) => unary_fmt!(a, "POST_DECR"),
            Self::Not(a) => unary_fmt!(a, "NOT"),
            Self::BitwiseNot(a) => unary_fmt!(a, "BIT_NOT"),
            Self::Paren(a) => unary_fmt!(a, "PAREN"),
            Self::Assign(a, b) => binary_fmt!(a, b, "ASSIGN"),
            Self::ArrowOp(a, b) => binary_fmt!(a, b, "ARROW"),
            Self::Dot(a, b) => binary_fmt!(a, b, "ARROW"),
            Self::FunctionCall(a, b) => {
                writeln!(f, "CALL")?;
                a.fmt_ast_with_indent(indent + 1, f)?;
                if let Some(b) = b {
                    b.fmt_ast_with_indent(indent + 1, f)?;
                }
                Ok(())
            }
            Self::Add(a, b) => binary_fmt!(a, b, "ADD"),
            Self::Sub(a, b) => binary_fmt!(a, b, "SUB"),
            Self::Mul(a, b) => binary_fmt!(a, b, "MUL"),
            Self::Div(a, b) => binary_fmt!(a, b, "DIV"),
            Self::Pow(a, b) => binary_fmt!(a, b, "POW"),
            Self::And(a, b) => binary_fmt!(a, b, "AND"),
            Self::Or(a, b) => binary_fmt!(a, b, "OR"),
            Self::Eq(a, b) => binary_fmt!(a, b, "EQ"),
            Self::NotEq(a, b) => binary_fmt!(a, b, "NEQ"),
            Self::Greater(a, b) => binary_fmt!(a, b, "GREATER"),
            Self::GreaterEqual(a, b) => binary_fmt!(a, b, "GTEQ"),
            Self::Less(a, b) => binary_fmt!(a, b, "LESS"),
            Self::LessEqual(a, b) => binary_fmt!(a, b, "LESSEQ"),
            Self::BitXor(a, b) => binary_fmt!(a, b, "BIT_XOR"),
            Self::Rem(a, b) => binary_fmt!(a, b, "REM"),
            Self::BitAnd(a, b) => binary_fmt!(a, b, "BIT_AND"),
            Self::Index(a, b) => binary_fmt!(a, b, "INDEX"),
            Self::Comma(a, b) => binary_fmt!(a, b, "COMMA"),
            Self::Ternary(cond, a, b) => {
                writeln!(f, "TERNARY")?;
                cond.fmt_ast_with_indent(indent + 1, f)?;
                a.fmt_ast_with_indent(indent + 2, f)?;
                b.fmt_ast_with_indent(indent + 2, f)
            }
        }
    }
    fn fmt_delimited(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(name) => return write!(f, "{name}"),
            Self::Value(value) => return write!(f, "{value}"),
            Self::Paren(a) => return a.fmt_delimited(f),
            _ => (),
        }
        macro_rules! unary {
            ($op:literal, $a:ident) => {{
                write!(f, $op)?;
                $a.fmt_delimited(f)?;
            }};
        }
        macro_rules! binary {
            ($op:literal, $a:ident, $b:ident) => {{
                write!(f, "{} ", $op)?;
                $a.fmt_delimited(f)?;
                write!(f, " ")?;
                $b.fmt_delimited(f)?;
            }};
        }
        write!(f, "(")?;
        match self {
            Self::Assign(a, b) => binary!("=", a, b),
            Self::FunctionCall(a, b) => {
                write!(f, "call ")?;
                a.fmt_delimited(f)?;
                if let Some(b) = b {
                    write!(f, " ")?;
                    b.fmt_delimited(f)?;
                }
            }
            Self::ArrowOp(a, b) => binary!("->", a, b),
            Self::Dot(a, b) => binary!(".", a, b),
            Self::Addr(a) => unary!("&", a),
            Self::Deref(a) => unary!("*", a),
            Self::Neg(a) => unary!("-", a),
            Self::Fac(a) => unary!("!", a),
            Self::Not(a) => unary!("!", a),
            Self::BitwiseNot(a) => unary!("~", a),
            Self::PreIncr(a) => unary!("pre++", a),
            Self::PostIncr(a) => unary!("post++", a),
            Self::PreDecr(a) => unary!("pre--", a),
            Self::PostDecr(a) => unary!("post--", a),
            Self::Add(a, b) => binary!("+", a, b),
            Self::Sub(a, b) => binary!("-", a, b),
            Self::Mul(a, b) => binary!("*", a, b),
            Self::Div(a, b) => binary!("/", a, b),
            Self::Pow(a, b) => binary!("**", a, b),
            Self::And(a, b) => binary!("&&", a, b),
            Self::Or(a, b) => binary!("||", a, b),
            Self::Eq(a, b) => binary!("==", a, b),
            Self::NotEq(a, b) => binary!("!=", a, b),
            Self::Greater(a, b) => binary!(">", a, b),
            Self::GreaterEqual(a, b) => binary!(">=", a, b),
            Self::Less(a, b) => binary!("<", a, b),
            Self::LessEqual(a, b) => binary!("<=", a, b),
            Self::BitXor(a, b) => binary!("^", a, b),
            Self::Rem(a, b) => binary!("%", a, b),
            Self::BitAnd(a, b) => binary!("&", a, b),
            Self::Index(a, b) => binary!("[]", a, b),
            Self::Comma(a, b) => binary!(",", a, b),
            Self::Ternary(cond, a, b) => {
                write!(f, "? ")?;
                cond.fmt_delimited(f)?;
                write!(f, " ")?;
                a.fmt_delimited(f)?;
                write!(f, " ")?;
                b.fmt_delimited(f)?;
            }
            _ => unreachable!(),
        }

        write!(f, ")")
    }
}

impl core::fmt::Display for Expr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt_ast_with_indent(0, f)?;
        writeln!(f)?;
        self.fmt_delimited(f)
    }
}
