// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types and helpers for the pest's own grammar parser.

use std::char;
use std::iter::Peekable;

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::{Parser, Position, Span};

use crate::ast::{Expr, Rule as AstRule, RuleType};
use crate::validator;

/// Note: `include!` adds here a code generated from build.rs file.
/// In case feature `not-bootstrap-in-src` is:
/// * OFF  -> include generated `grammar.rs` file from meta/src
/// * ON   -> include generated `__pest_grammar.rs` file from target/build/...
#[allow(missing_docs, unused_qualifications)]
mod grammar {
    #[cfg(not(feature = "not-bootstrap-in-src"))]
    include!("grammar.rs");

    #[cfg(feature = "not-bootstrap-in-src")]
    include!(concat!(env!("OUT_DIR"), "/__pest_grammar.rs"));
}

/// Import included grammar (`PestParser` class globally for current module).
pub use self::grammar::*;

/// A helper that will parse using the pest grammar
#[allow(clippy::perf)]
pub fn parse(rule: Rule, data: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    PestParser::parse(rule, data)
}

/// The pest grammar rule
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserRule<'i> {
    /// The rule's name
    pub name: String,
    /// The rule's span
    pub span: Span<'i>,
    /// The rule's type
    pub ty: RuleType,
    /// The rule's parser node
    pub node: ParserNode<'i>,
}

/// The pest grammar node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserNode<'i> {
    /// The node's expression
    pub expr: ParserExpr<'i>,
    /// The node's span
    pub span: Span<'i>,
}

impl<'i> ParserNode<'i> {
    /// will remove nodes that do not match `f`
    pub fn filter_map_top_down<F, T>(self, mut f: F) -> Vec<T>
    where
        F: FnMut(ParserNode<'i>) -> Option<T>,
    {
        pub fn filter_internal<'i, F, T>(node: ParserNode<'i>, f: &mut F, result: &mut Vec<T>)
        where
            F: FnMut(ParserNode<'i>) -> Option<T>,
        {
            if let Some(value) = f(node.clone()) {
                result.push(value);
            }

            match node.expr {
                ParserExpr::PosPred(node) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::NegPred(node) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::Seq(lhs, rhs) => {
                    filter_internal(*lhs, f, result);
                    filter_internal(*rhs, f, result);
                }
                ParserExpr::Choice(lhs, rhs) => {
                    filter_internal(*lhs, f, result);
                    filter_internal(*rhs, f, result);
                }
                ParserExpr::Rep(node) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::RepOnce(node) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::RepExact(node, _) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::RepMin(node, _) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::RepMax(node, _) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::RepMinMax(node, ..) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::Opt(node) => {
                    filter_internal(*node, f, result);
                }
                ParserExpr::Push(node) => {
                    filter_internal(*node, f, result);
                }
                _ => (),
            }
        }

        let mut result = vec![];

        filter_internal(self, &mut f, &mut result);

        result
    }
}

/// All possible parser expressions
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParserExpr<'i> {
    /// Matches an exact string, e.g. `"a"`
    Str(String),
    /// Matches an exact string, case insensitively (ASCII only), e.g. `^"a"`
    Insens(String),
    /// Matches one character in the range, e.g. `'a'..'z'`
    Range(String, String),
    /// Matches the rule with the given name, e.g. `a`
    Ident(String),
    /// Matches a custom part of the stack, e.g. `PEEK[..]`
    PeekSlice(i32, Option<i32>),
    /// Positive lookahead; matches expression without making progress, e.g. `&e`
    PosPred(Box<ParserNode<'i>>),
    /// Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`
    NegPred(Box<ParserNode<'i>>),
    /// Matches a sequence of two expressions, e.g. `e1 ~ e2`
    Seq(Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    /// Matches either of two expressions, e.g. `e1 | e2`
    Choice(Box<ParserNode<'i>>, Box<ParserNode<'i>>),
    /// Optionally matches an expression, e.g. `e?`
    Opt(Box<ParserNode<'i>>),
    /// Matches an expression zero or more times, e.g. `e*`
    Rep(Box<ParserNode<'i>>),
    /// Matches an expression one or more times, e.g. `e+`
    RepOnce(Box<ParserNode<'i>>),
    /// Matches an expression an exact number of times, e.g. `e{n}`
    RepExact(Box<ParserNode<'i>>, u32),
    /// Matches an expression at least a number of times, e.g. `e{n,}`
    RepMin(Box<ParserNode<'i>>, u32),
    /// Matches an expression at most a number of times, e.g. `e{,n}`
    RepMax(Box<ParserNode<'i>>, u32),
    /// Matches an expression a number of times within a range, e.g. `e{m, n}`
    RepMinMax(Box<ParserNode<'i>>, u32, u32),
    /// Matches an expression and pushes it to the stack, e.g. `push(e)`
    Push(Box<ParserNode<'i>>),
    /// Pushes a literal string to the stack, e.g. `push_literal("a")`
    #[cfg(feature = "grammar-extras")]
    PushLiteral(String),
    /// Matches an expression and assigns a label to it, e.g. #label = exp
    #[cfg(feature = "grammar-extras")]
    NodeTag(Box<ParserNode<'i>>, String),
}

fn convert_rule(rule: ParserRule<'_>) -> AstRule {
    let ParserRule { name, ty, node, .. } = rule;
    let expr = convert_node(node);
    AstRule { name, ty, expr }
}

fn convert_node(node: ParserNode<'_>) -> Expr {
    match node.expr {
        ParserExpr::Str(string) => Expr::Str(string),
        ParserExpr::Insens(string) => Expr::Insens(string),
        ParserExpr::Range(start, end) => Expr::Range(start, end),
        ParserExpr::Ident(ident) => Expr::Ident(ident),
        ParserExpr::PeekSlice(start, end) => Expr::PeekSlice(start, end),
        ParserExpr::PosPred(node) => Expr::PosPred(Box::new(convert_node(*node))),
        ParserExpr::NegPred(node) => Expr::NegPred(Box::new(convert_node(*node))),
        ParserExpr::Seq(node1, node2) => Expr::Seq(
            Box::new(convert_node(*node1)),
            Box::new(convert_node(*node2)),
        ),
        ParserExpr::Choice(node1, node2) => Expr::Choice(
            Box::new(convert_node(*node1)),
            Box::new(convert_node(*node2)),
        ),
        ParserExpr::Opt(node) => Expr::Opt(Box::new(convert_node(*node))),
        ParserExpr::Rep(node) => Expr::Rep(Box::new(convert_node(*node))),
        ParserExpr::RepOnce(node) => Expr::RepOnce(Box::new(convert_node(*node))),
        ParserExpr::RepExact(node, num) => Expr::RepExact(Box::new(convert_node(*node)), num),
        ParserExpr::RepMin(node, max) => Expr::RepMin(Box::new(convert_node(*node)), max),
        ParserExpr::RepMax(node, max) => Expr::RepMax(Box::new(convert_node(*node)), max),
        ParserExpr::RepMinMax(node, min, max) => {
            Expr::RepMinMax(Box::new(convert_node(*node)), min, max)
        }
        ParserExpr::Push(node) => Expr::Push(Box::new(convert_node(*node))),
        #[cfg(feature = "grammar-extras")]
        ParserExpr::PushLiteral(string) => Expr::PushLiteral(string),
        #[cfg(feature = "grammar-extras")]
        ParserExpr::NodeTag(node, tag) => Expr::NodeTag(Box::new(convert_node(*node)), tag),
    }
}

/// Converts a parser's result (`Pairs`) to an AST
pub fn consume_rules(pairs: Pairs<'_, Rule>) -> Result<Vec<AstRule>, Vec<Error<Rule>>> {
    let rules = consume_rules_with_spans(pairs)?;
    let errors = validator::validate_ast(&rules);
    if errors.is_empty() {
        Ok(rules.into_iter().map(convert_rule).collect())
    } else {
        Err(errors)
    }
}

/// A helper function to rename verbose rules
/// for the sake of better error messages
#[inline]
pub fn rename_meta_rule(rule: &Rule) -> String {
    match *rule {
        Rule::grammar_rule => "rule".to_owned(),
        Rule::_push => "PUSH".to_owned(),
        Rule::_push_literal => "PUSH_LITERAL".to_owned(),
        Rule::assignment_operator => "`=`".to_owned(),
        Rule::silent_modifier => "`_`".to_owned(),
        Rule::atomic_modifier => "`@`".to_owned(),
        Rule::compound_atomic_modifier => "`$`".to_owned(),
        Rule::non_atomic_modifier => "`!`".to_owned(),
        Rule::opening_brace => "`{`".to_owned(),
        Rule::closing_brace => "`}`".to_owned(),
        Rule::opening_brack => "`[`".to_owned(),
        Rule::closing_brack => "`]`".to_owned(),
        Rule::opening_paren => "`(`".to_owned(),
        Rule::positive_predicate_operator => "`&`".to_owned(),
        Rule::negative_predicate_operator => "`!`".to_owned(),
        Rule::sequence_operator => "`&`".to_owned(),
        Rule::choice_operator => "`|`".to_owned(),
        Rule::optional_operator => "`?`".to_owned(),
        Rule::repeat_operator => "`*`".to_owned(),
        Rule::repeat_once_operator => "`+`".to_owned(),
        Rule::comma => "`,`".to_owned(),
        Rule::closing_paren => "`)`".to_owned(),
        Rule::quote => "`\"`".to_owned(),
        Rule::insensitive_string => "`^`".to_owned(),
        Rule::range_operator => "`..`".to_owned(),
        Rule::single_quote => "`'`".to_owned(),
        Rule::grammar_doc => "//!".to_owned(),
        Rule::line_doc => "///".to_owned(),
        other_rule => format!("{:?}", other_rule),
    }
}

fn consume_rules_with_spans(
    pairs: Pairs<'_, Rule>,
) -> Result<Vec<ParserRule<'_>>, Vec<Error<Rule>>> {
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::choice_operator, Assoc::Left))
        .op(Op::infix(Rule::sequence_operator, Assoc::Left));

    pairs
        .filter(|pair| pair.as_rule() == Rule::grammar_rule)
        .filter(|pair| {
            // To ignore `grammar_rule > line_doc` pairs
            let mut pairs = pair.clone().into_inner();
            let pair = pairs.next().unwrap();

            pair.as_rule() != Rule::line_doc
        })
        .map(|pair| {
            let mut pairs = pair.into_inner().peekable();

            let span = pairs.next().unwrap().as_span();
            let name = span.as_str().to_owned();

            pairs.next().unwrap(); // assignment_operator

            let ty = if pairs.peek().unwrap().as_rule() != Rule::opening_brace {
                match pairs.next().unwrap().as_rule() {
                    Rule::silent_modifier => RuleType::Silent,
                    Rule::atomic_modifier => RuleType::Atomic,
                    Rule::compound_atomic_modifier => RuleType::CompoundAtomic,
                    Rule::non_atomic_modifier => RuleType::NonAtomic,
                    _ => unreachable!(),
                }
            } else {
                RuleType::Normal
            };

            pairs.next().unwrap(); // opening_brace

            // skip initial infix operators
            let mut inner_nodes = pairs.next().unwrap().into_inner().peekable();
            if inner_nodes.peek().unwrap().as_rule() == Rule::choice_operator {
                inner_nodes.next().unwrap();
            }

            let node = consume_expr(inner_nodes, &pratt)?;

            Ok(ParserRule {
                name,
                span,
                ty,
                node,
            })
        })
        .collect()
}

fn get_node_tag<'i>(
    pairs: &mut Peekable<Pairs<'i, Rule>>,
) -> (Pair<'i, Rule>, Option<(String, Position<'i>)>) {
    let pair_or_tag = pairs.next().unwrap();
    if let Some(next_pair) = pairs.peek() {
        if next_pair.as_rule() == Rule::assignment_operator {
            pairs.next().unwrap();
            let pair = pairs.next().unwrap();
            (
                pair,
                Some((
                    pair_or_tag.as_str()[1..].to_string(),
                    pair_or_tag.as_span().start_pos(),
                )),
            )
        } else {
            (pair_or_tag, None)
        }
    } else {
        (pair_or_tag, None)
    }
}

fn consume_expr<'i>(
    pairs: Peekable<Pairs<'i, Rule>>,
    pratt: &PrattParser<Rule>,
) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
    fn unaries<'i>(
        mut pairs: Peekable<Pairs<'i, Rule>>,
        pratt: &PrattParser<Rule>,
    ) -> Result<ParserNode<'i>, Vec<Error<Rule>>> {
        #[cfg(feature = "grammar-extras")]
        let (pair, tag_start) = get_node_tag(&mut pairs);
        #[cfg(not(feature = "grammar-extras"))]
        let (pair, _tag_start) = get_node_tag(&mut pairs);

        let node = match pair.as_rule() {
            Rule::opening_paren => {
                let node = unaries(pairs, pratt)?;
                let end = node.span.end_pos();

                ParserNode {
                    expr: node.expr,
                    span: pair.as_span().start_pos().span(&end),
                }
            }
            Rule::positive_predicate_operator => {
                let node = unaries(pairs, pratt)?;
                let end = node.span.end_pos();

                ParserNode {
                    expr: ParserExpr::PosPred(Box::new(node)),
                    span: pair.as_span().start_pos().span(&end),
                }
            }
            Rule::negative_predicate_operator => {
                let node = unaries(pairs, pratt)?;
                let end = node.span.end_pos();

                ParserNode {
                    expr: ParserExpr::NegPred(Box::new(node)),
                    span: pair.as_span().start_pos().span(&end),
                }
            }
            other_rule => {
                let node = match other_rule {
                    Rule::expression => consume_expr(pair.into_inner().peekable(), pratt)?,
                    Rule::_push => {
                        let start = pair.clone().as_span().start_pos();
                        let mut pairs = pair.into_inner();
                        pairs.next().unwrap(); // opening_paren
                        let pair = pairs.next().unwrap();

                        let node = consume_expr(pair.into_inner().peekable(), pratt)?;
                        let end = node.span.end_pos();

                        ParserNode {
                            expr: ParserExpr::Push(Box::new(node)),
                            span: start.span(&end),
                        }
                    }
                    #[cfg(feature = "grammar-extras")]
                    Rule::_push_literal => {
                        let mut pairs = pair.into_inner();
                        pairs.next().unwrap(); // opening_paren
                        let contents_pair = pairs.next().unwrap();
                        let string =
                            unescape(contents_pair.as_str()).expect("incorrect string literal");
                        ParserNode {
                            expr: ParserExpr::PushLiteral(string[1..string.len() - 1].to_owned()),
                            span: contents_pair.clone().as_span(),
                        }
                    }
                    #[cfg(not(feature = "grammar-extras"))]
                    Rule::_push_literal => {
                        return Err(vec![Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: "PUSH_LITERAL requires feature grammar-extras".to_owned(),
                            },
                            pair.as_span(),
                        )]);
                    }
                    Rule::peek_slice => {
                        let mut pairs = pair.clone().into_inner();
                        pairs.next().unwrap(); // opening_brack
                        let pair_start = pairs.next().unwrap(); // .. or integer
                        let start: i32 = match pair_start.as_rule() {
                            Rule::range_operator => 0,
                            Rule::integer => {
                                pairs.next().unwrap(); // ..
                                pair_start.as_str().parse().unwrap()
                            }
                            _ => unreachable!("peek start"),
                        };
                        let pair_end = pairs.next().unwrap(); // integer or }
                        let end: Option<i32> = match pair_end.as_rule() {
                            Rule::closing_brack => None,
                            Rule::integer => {
                                pairs.next().unwrap(); // }
                                Some(pair_end.as_str().parse().unwrap())
                            }
                            _ => unreachable!("peek end"),
                        };
                        ParserNode {
                            expr: ParserExpr::PeekSlice(start, end),
                            span: pair.as_span(),
                        }
                    }
                    Rule::identifier => ParserNode {
                        expr: ParserExpr::Ident(pair.as_str().to_owned()),
                        span: pair.clone().as_span(),
                    },
                    Rule::string => {
                        let string = unescape(pair.as_str()).expect("incorrect string literal");
                        ParserNode {
                            expr: ParserExpr::Str(string[1..string.len() - 1].to_owned()),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::insensitive_string => {
                        let string = unescape(pair.as_str()).expect("incorrect string literal");
                        ParserNode {
                            expr: ParserExpr::Insens(string[2..string.len() - 1].to_owned()),
                            span: pair.clone().as_span(),
                        }
                    }
                    Rule::range => {
                        let mut pairs = pair.into_inner();
                        let pair = pairs.next().unwrap();
                        let start = unescape(pair.as_str()).expect("incorrect char literal");
                        let start_pos = pair.clone().as_span().start_pos();
                        pairs.next();
                        let pair = pairs.next().unwrap();
                        let end = unescape(pair.as_str()).expect("incorrect char literal");
                        let end_pos = pair.clone().as_span().end_pos();

                        ParserNode {
                            expr: ParserExpr::Range(
                                start[1..start.len() - 1].to_owned(),
                                end[1..end.len() - 1].to_owned(),
                            ),
                            span: start_pos.span(&end_pos),
                        }
                    }
                    x => unreachable!("other rule: {:?}", x),
                };

                pairs.try_fold(node, |node: ParserNode<'i>, pair: Pair<'i, Rule>| {
                    let node = match pair.as_rule() {
                        Rule::optional_operator => {
                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::Opt(Box::new(node)),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_operator => {
                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::Rep(Box::new(node)),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_once_operator => {
                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::RepOnce(Box::new(node)),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_exact => {
                            let mut inner = pair.clone().into_inner();

                            inner.next().unwrap(); // opening_brace

                            let number = inner.next().unwrap();
                            let num = if let Ok(num) = number.as_str().parse::<u32>() {
                                num
                            } else {
                                return Err(vec![Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "number cannot overflow u32".to_owned(),
                                    },
                                    number.as_span(),
                                )]);
                            };

                            if num == 0 {
                                let error: Error<Rule> = Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "cannot repeat 0 times".to_owned(),
                                    },
                                    number.as_span(),
                                );

                                return Err(vec![error]);
                            }

                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::RepExact(Box::new(node), num),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_min => {
                            let mut inner = pair.clone().into_inner();

                            inner.next().unwrap(); // opening_brace

                            let min_number = inner.next().unwrap();
                            let min = if let Ok(min) = min_number.as_str().parse::<u32>() {
                                min
                            } else {
                                return Err(vec![Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "number cannot overflow u32".to_owned(),
                                    },
                                    min_number.as_span(),
                                )]);
                            };

                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::RepMin(Box::new(node), min),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_max => {
                            let mut inner = pair.clone().into_inner();

                            inner.next().unwrap(); // opening_brace
                            inner.next().unwrap(); // comma

                            let max_number = inner.next().unwrap();
                            let max = if let Ok(max) = max_number.as_str().parse::<u32>() {
                                max
                            } else {
                                return Err(vec![Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "number cannot overflow u32".to_owned(),
                                    },
                                    max_number.as_span(),
                                )]);
                            };

                            if max == 0 {
                                let error: Error<Rule> = Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "cannot repeat 0 times".to_owned(),
                                    },
                                    max_number.as_span(),
                                );

                                return Err(vec![error]);
                            }

                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::RepMax(Box::new(node), max),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::repeat_min_max => {
                            let mut inner = pair.clone().into_inner();

                            inner.next().unwrap(); // opening_brace

                            let min_number = inner.next().unwrap();
                            let min = if let Ok(min) = min_number.as_str().parse::<u32>() {
                                min
                            } else {
                                return Err(vec![Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "number cannot overflow u32".to_owned(),
                                    },
                                    min_number.as_span(),
                                )]);
                            };

                            inner.next().unwrap(); // comma

                            let max_number = inner.next().unwrap();
                            let max = if let Ok(max) = max_number.as_str().parse::<u32>() {
                                max
                            } else {
                                return Err(vec![Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "number cannot overflow u32".to_owned(),
                                    },
                                    max_number.as_span(),
                                )]);
                            };

                            if max == 0 {
                                let error: Error<Rule> = Error::new_from_span(
                                    ErrorVariant::CustomError {
                                        message: "cannot repeat 0 times".to_owned(),
                                    },
                                    max_number.as_span(),
                                );

                                return Err(vec![error]);
                            }

                            let start = node.span.start_pos();
                            ParserNode {
                                expr: ParserExpr::RepMinMax(Box::new(node), min, max),
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        Rule::closing_paren => {
                            let start = node.span.start_pos();

                            ParserNode {
                                expr: node.expr,
                                span: start.span(&pair.as_span().end_pos()),
                            }
                        }
                        rule => unreachable!("node: {:?}", rule),
                    };

                    Ok(node)
                })?
            }
        };
        #[cfg(feature = "grammar-extras")]
        if let Some((tag, start)) = tag_start {
            let span = start.span(&node.span.end_pos());
            Ok(ParserNode {
                expr: ParserExpr::NodeTag(Box::new(node), tag),
                span,
            })
        } else {
            Ok(node)
        }
        #[cfg(not(feature = "grammar-extras"))]
        Ok(node)
    }

    let term = |pair: Pair<'i, Rule>| unaries(pair.into_inner().peekable(), pratt);
    let infix = |lhs: Result<ParserNode<'i>, Vec<Error<Rule>>>,
                 op: Pair<'i, Rule>,
                 rhs: Result<ParserNode<'i>, Vec<Error<Rule>>>| match op.as_rule() {
        Rule::sequence_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Seq(Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        Rule::choice_operator => {
            let lhs = lhs?;
            let rhs = rhs?;

            let start = lhs.span.start_pos();
            let end = rhs.span.end_pos();

            Ok(ParserNode {
                expr: ParserExpr::Choice(Box::new(lhs), Box::new(rhs)),
                span: start.span(&end),
            })
        }
        _ => unreachable!("infix"),
    };

    pratt.map_primary(term).map_infix(infix).parse(pairs)
}

fn unescape(string: &str) -> Option<String> {
    let mut result = String::new();
    let mut chars = string.chars();

    loop {
        match chars.next() {
            Some('\\') => match chars.next()? {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                'r' => result.push('\r'),
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '0' => result.push('\0'),
                '\'' => result.push('\''),
                'x' => {
                    let string: String = chars.clone().take(2).collect();

                    if string.len() != 2 {
                        return None;
                    }

                    for _ in 0..string.len() {
                        chars.next()?;
                    }

                    let value = u8::from_str_radix(&string, 16).ok()?;

                    result.push(char::from(value));
                }
                'u' => {
                    if chars.next()? != '{' {
                        return None;
                    }

                    let string: String = chars.clone().take_while(|c| *c != '}').collect();

                    if string.len() < 2 || 6 < string.len() {
                        return None;
                    }

                    for _ in 0..string.len() + 1 {
                        chars.next()?;
                    }

                    let value = u32::from_str_radix(&string, 16).ok()?;

                    result.push(char::from_u32(value)?);
                }
                _ => return None,
            },
            Some(c) => result.push(c),
            None => return Some(result),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::super::unwrap_or_report;
    use super::*;

    #[test]
    fn rules() {
        parses_to! {
            parser: PestParser,
            input: "a = { b } c = { d }",
            rule: Rule::grammar_rules,
            tokens: [
                grammar_rule(0, 9, [
                    identifier(0, 1),
                    assignment_operator(2, 3),
                    opening_brace(4, 5),
                    expression(6, 8, [
                        term(6, 8, [
                            identifier(6, 7)
                        ])
                    ]),
                    closing_brace(8, 9)
                ]),
                grammar_rule(10, 19, [
                    identifier(10, 11),
                    assignment_operator(12, 13),
                    opening_brace(14, 15),
                    expression(16, 18, [
                        term(16, 18, [
                            identifier(16, 17)
                        ])
                    ]),
                    closing_brace(18, 19)
                ])
            ]
        };
    }

    #[test]
    fn rule() {
        parses_to! {
            parser: PestParser,
            input: "a = ! { b ~ c }",
            rule: Rule::grammar_rule,
            tokens: [
                grammar_rule(0, 15, [
                    identifier(0, 1),
                    assignment_operator(2, 3),
                    non_atomic_modifier(4, 5),
                    opening_brace(6, 7),
                    expression(8, 14, [
                        term(8, 10, [
                            identifier(8, 9)
                        ]),
                        sequence_operator(10, 11),
                        term(12, 14, [
                            identifier(12, 13)
                        ])
                    ]),
                    closing_brace(14, 15)
                ])
            ]
        };
    }

    #[test]
    fn expression() {
        parses_to! {
            parser: PestParser,
            input: "_a | 'a'..'b' ~ !^\"abc\" ~ (d | e)*?",
            rule: Rule::expression,
            tokens: [
                expression(0, 35, [
                    term(0, 3, [
                        identifier(0, 2)
                    ]),
                    choice_operator(3, 4),
                    term(5, 14, [
                        range(5, 13, [
                            character(5, 8, [
                                single_quote(5, 6),
                                inner_chr(6, 7),
                                single_quote(7, 8)
                            ]),
                            range_operator(8, 10),
                            character(10, 13, [
                                single_quote(10, 11),
                                inner_chr(11, 12),
                                single_quote(12, 13)
                            ])
                        ])
                    ]),
                    sequence_operator(14, 15),
                    term(16, 24, [
                        negative_predicate_operator(16, 17),
                        insensitive_string(17, 23, [
                            string(18, 23, [
                                quote(18, 19),
                                inner_str(19, 22),
                                quote(22, 23)
                            ])
                        ])
                    ]),
                    sequence_operator(24, 25),
                    term(26, 35, [
                        opening_paren(26, 27),
                        expression(27, 32, [
                            term(27, 29, [
                                identifier(27, 28)
                            ]),
                            choice_operator(29, 30),
                            term(31, 32, [
                                identifier(31, 32)
                            ])
                        ]),
                        closing_paren(32, 33),
                        repeat_operator(33, 34),
                        optional_operator(34, 35)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn repeat_exact() {
        parses_to! {
            parser: PestParser,
            input: "{1}",
            rule: Rule::repeat_exact,
            tokens: [
                repeat_exact(0, 3, [
                    opening_brace(0, 1),
                    number(1, 2),
                    closing_brace(2, 3)
                ])
            ]
        };
    }

    #[test]
    fn repeat_min() {
        parses_to! {
            parser: PestParser,
            input: "{2,}",
            rule: Rule::repeat_min,
            tokens: [
                repeat_min(0, 4, [
                    opening_brace(0,1),
                    number(1,2),
                    comma(2,3),
                    closing_brace(3,4)
                ])
            ]
        }
    }

    #[test]
    fn repeat_max() {
        parses_to! {
            parser: PestParser,
            input: "{, 3}",
            rule: Rule::repeat_max,
            tokens: [
                repeat_max(0, 5, [
                    opening_brace(0,1),
                    comma(1,2),
                    number(3,4),
                    closing_brace(4,5)
                ])
            ]
        }
    }

    #[test]
    fn repeat_min_max() {
        parses_to! {
            parser: PestParser,
            input: "{1, 2}",
            rule: Rule::repeat_min_max,
            tokens: [
                repeat_min_max(0, 6, [
                    opening_brace(0, 1),
                    number(1, 2),
                    comma(2, 3),
                    number(4, 5),
                    closing_brace(5, 6)
                ])
            ]
        };
    }

    #[test]
    fn push() {
        parses_to! {
            parser: PestParser,
            input: "PUSH ( a )",
            rule: Rule::_push,
            tokens: [
                _push(0, 10, [
                    opening_paren(5, 6),
                    expression(7, 9, [
                        term(7, 9, [
                            identifier(7, 8)
                        ])
                    ]),
                    closing_paren(9, 10)
                ])
            ]
        };
    }

    #[test]
    fn peek_slice_all() {
        parses_to! {
            parser: PestParser,
            input: "PEEK[..]",
            rule: Rule::peek_slice,
            tokens: [
                peek_slice(0, 8, [
                    opening_brack(4, 5),
                    range_operator(5, 7),
                    closing_brack(7, 8)
                ])
            ]
        };
    }

    #[test]
    fn peek_slice_start() {
        parses_to! {
            parser: PestParser,
            input: "PEEK[1..]",
            rule: Rule::peek_slice,
            tokens: [
                peek_slice(0, 9, [
                    opening_brack(4, 5),
                    integer(5, 6),
                    range_operator(6, 8),
                    closing_brack(8, 9)
                ])
            ]
        };
    }

    #[test]
    fn peek_slice_end() {
        parses_to! {
            parser: PestParser,
            input: "PEEK[ ..-1]",
            rule: Rule::peek_slice,
            tokens: [
                peek_slice(0, 11, [
                    opening_brack(4, 5),
                    range_operator(6, 8),
                    integer(8, 10),
                    closing_brack(10, 11)
                ])
            ]
        };
    }

    #[test]
    fn peek_slice_start_end() {
        parses_to! {
            parser: PestParser,
            input: "PEEK[-5..10]",
            rule: Rule::peek_slice,
            tokens: [
                peek_slice(0, 12, [
                    opening_brack(4, 5),
                    integer(5, 7),
                    range_operator(7, 9),
                    integer(9, 11),
                    closing_brack(11, 12)
                ])
            ]
        };
    }

    #[test]
    fn identifier() {
        parses_to! {
            parser: PestParser,
            input: "_a8943",
            rule: Rule::identifier,
            tokens: [
                identifier(0, 6)
            ]
        };
    }

    #[test]
    fn string() {
        parses_to! {
            parser: PestParser,
            input: "\"aaaaa\\n\\r\\t\\\\\\0\\'\\\"\\x0F\\u{123abC}\\u{12}aaaaa\"",
            rule: Rule::string,
            tokens: [
                string(0, 46, [
                    quote(0, 1),
                    inner_str(1, 45),
                    quote(45, 46)
                ])
            ]
        };
    }

    #[test]
    fn insensitive_string() {
        parses_to! {
            parser: PestParser,
            input: "^  \"\\\"hi\"",
            rule: Rule::insensitive_string,
            tokens: [
                insensitive_string(0, 9, [
                    string(3, 9, [
                        quote(3, 4),
                        inner_str(4, 8),
                        quote(8, 9)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn range() {
        parses_to! {
            parser: PestParser,
            input: "'\\n' .. '\\x1a'",
            rule: Rule::range,
            tokens: [
                range(0, 14, [
                    character(0, 4, [
                        single_quote(0, 1),
                        inner_chr(1, 3),
                        single_quote(3, 4)
                    ]),
                    range_operator(5, 7),
                    character(8, 14, [
                        single_quote(8, 9),
                        inner_chr(9, 13),
                        single_quote(13, 14)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn character() {
        parses_to! {
            parser: PestParser,
            input: "'\\u{123abC}'",
            rule: Rule::character,
            tokens: [
                character(0, 12, [
                    single_quote(0, 1),
                    inner_chr(1, 11),
                    single_quote(11, 12)
                ])
            ]
        };
    }

    #[test]
    fn number() {
        parses_to! {
            parser: PestParser,
            input: "0123",
            rule: Rule::number,
            tokens: [
                number(0, 4)
            ]
        };
    }

    #[test]
    fn comment() {
        parses_to! {
            parser: PestParser,
            input: "a ~    // asda\n b",
            rule: Rule::expression,
            tokens: [
                expression(0, 17, [
                    term(0, 2, [
                        identifier(0, 1)
                    ]),
                    sequence_operator(2, 3),
                    term(16, 17, [
                        identifier(16, 17)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn grammar_doc_and_line_doc() {
        let input = "//! hello\n/// world\na = { \"a\" }";
        parses_to! {
            parser: PestParser,
            input: input,
            rule: Rule::grammar_rules,
            tokens: [
                grammar_doc(0, 9, [
                    inner_doc(4, 9),
                ]),
                grammar_rule(10, 19, [
                    line_doc(10, 19, [
                        inner_doc(14, 19),
                    ]),
                ]),
                grammar_rule(20, 31, [
                    identifier(20, 21),
                    assignment_operator(22, 23),
                    opening_brace(24, 25),
                    expression(26, 30, [
                        term(26, 30, [
                            string(26, 29, [
                                quote(26, 27),
                                inner_str(27, 28),
                                quote(28, 29)
                            ])
                        ])
                    ]),
                    closing_brace(30, 31),
                ])
            ]
        };
    }

    #[test]
    fn wrong_identifier() {
        fails_with! {
            parser: PestParser,
            input: "0",
            rule: Rule::grammar_rules,
            positives: vec![Rule::EOI, Rule::grammar_rule, Rule::grammar_doc],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn missing_assignment_operator() {
        fails_with! {
            parser: PestParser,
            input: "a {}",
            rule: Rule::grammar_rules,
            positives: vec![Rule::assignment_operator],
            negatives: vec![],
            pos: 2
        };
    }

    #[test]
    fn wrong_modifier() {
        fails_with! {
            parser: PestParser,
            input: "a = *{}",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_brace,
                Rule::silent_modifier,
                Rule::atomic_modifier,
                Rule::compound_atomic_modifier,
                Rule::non_atomic_modifier
            ],
            negatives: vec![],
            pos: 4
        };
    }

    #[test]
    fn missing_opening_brace() {
        fails_with! {
            parser: PestParser,
            input: "a = _",
            rule: Rule::grammar_rules,
            positives: vec![Rule::opening_brace],
            negatives: vec![],
            pos: 5
        };
    }

    #[test]
    fn empty_rule() {
        fails_with! {
            parser: PestParser,
            input: "a = {}",
            rule: Rule::grammar_rules,
            positives: vec![Rule::expression],
            negatives: vec![],
            pos: 5
        };
    }

    #[test]
    fn missing_rhs() {
        fails_with! {
            parser: PestParser,
            input: "a = { b ~ }",
            rule: Rule::grammar_rules,
            positives: vec![Rule::term],
            negatives: vec![],
            pos: 10
        };
    }

    #[test]
    fn incorrect_prefix() {
        fails_with! {
            parser: PestParser,
            input: "a = { ~ b}",
            rule: Rule::grammar_rules,
            positives: vec![Rule::expression],
            negatives: vec![],
            pos: 6
        };
    }

    #[test]
    fn wrong_op() {
        fails_with! {
            parser: PestParser,
            input: "a = { b % }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_brace,
                Rule::closing_brace,
                Rule::sequence_operator,
                Rule::choice_operator,
                Rule::optional_operator,
                Rule::repeat_operator,
                Rule::repeat_once_operator
            ],
            negatives: vec![],
            pos: 8
        };
    }

    #[test]
    fn missing_closing_paren() {
        fails_with! {
            parser: PestParser,
            input: "a = { (b }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_brace,
                Rule::closing_paren,
                Rule::sequence_operator,
                Rule::choice_operator,
                Rule::optional_operator,
                Rule::repeat_operator,
                Rule::repeat_once_operator
            ],
            negatives: vec![],
            pos: 9
        };
    }

    #[test]
    fn missing_term() {
        fails_with! {
            parser: PestParser,
            input: "a = { ! }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_paren,
                Rule::positive_predicate_operator,
                Rule::negative_predicate_operator,
                Rule::_push,
                Rule::_push_literal,
                Rule::peek_slice,
                Rule::identifier,
                Rule::insensitive_string,
                Rule::quote,
                Rule::single_quote
            ],
            negatives: vec![],
            pos: 8
        };
    }

    #[test]
    fn string_missing_ending_quote() {
        fails_with! {
            parser: PestParser,
            input: "a = { \" }",
            rule: Rule::grammar_rules,
            positives: vec![Rule::quote],
            negatives: vec![],
            pos: 9
        };
    }

    #[test]
    fn insensitive_missing_string() {
        fails_with! {
            parser: PestParser,
            input: "a = { ^ }",
            rule: Rule::grammar_rules,
            positives: vec![Rule::quote],
            negatives: vec![],
            pos: 8
        };
    }

    #[test]
    fn char_missing_ending_single_quote() {
        fails_with! {
            parser: PestParser,
            input: "a = { \' }",
            rule: Rule::grammar_rules,
            positives: vec![Rule::single_quote],
            negatives: vec![],
            pos: 8
        };
    }

    #[test]
    fn range_missing_range_operator() {
        fails_with! {
            parser: PestParser,
            input: "a = { \'a\' }",
            rule: Rule::grammar_rules,
            positives: vec![Rule::range_operator],
            negatives: vec![],
            pos: 10
        };
    }

    #[test]
    fn wrong_postfix() {
        fails_with! {
            parser: PestParser,
            input: "a = { a& }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_brace,
                Rule::closing_brace,
                Rule::sequence_operator,
                Rule::choice_operator,
                Rule::optional_operator,
                Rule::repeat_operator,
                Rule::repeat_once_operator
            ],
            negatives: vec![],
            pos: 7
        };
    }

    #[test]
    fn node_tag() {
        parses_to! {
            parser: PestParser,
            input: "#a = a",
            rule: Rule::expression,
            tokens: [
                expression(0, 6, [
                    term(0, 6, [
                        tag_id(0, 2),
                        assignment_operator(3, 4),
                        identifier(5, 6)
                    ])
                ])
            ]
        };
    }

    #[test]
    fn incomplete_node_tag() {
        fails_with! {
            parser: PestParser,
            input: "a = { # }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::expression
            ],
            negatives: vec![],
            pos: 6
        };
    }

    #[test]
    fn incomplete_node_tag_assignment() {
        fails_with! {
            parser: PestParser,
            input: "a = { #a = }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_paren,
                Rule::positive_predicate_operator,
                Rule::negative_predicate_operator,
                Rule::_push,
                Rule::_push_literal,
                Rule::peek_slice,
                Rule::identifier,
                Rule::insensitive_string,
                Rule::quote,
                Rule::single_quote
            ],
            negatives: vec![],
            pos: 11
        };
    }

    #[test]
    fn incomplete_node_tag_pound_key() {
        fails_with! {
            parser: PestParser,
            input: "a = { a = a }",
            rule: Rule::grammar_rules,
            positives: vec![
                Rule::opening_brace,
                Rule::closing_brace,
                Rule::sequence_operator,
                Rule::choice_operator,
                Rule::optional_operator,
                Rule::repeat_operator,
                Rule::repeat_once_operator
            ],
            negatives: vec![],
            pos: 8
        };
    }

    #[test]
    fn ast() {
        let input = r#"
        /// This is line comment
        /// This is rule
        rule = _{ a{1} ~ "a"{3,} ~ b{, 2} ~ "b"{1, 2} | !(^"c" | PUSH('d'..'e'))?* }
        "#;

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        let ast = consume_rules_with_spans(pairs).unwrap();
        let ast: Vec<_> = ast.into_iter().map(convert_rule).collect();

        assert_eq!(
            ast,
            vec![AstRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: Expr::Choice(
                    Box::new(Expr::Seq(
                        Box::new(Expr::Seq(
                            Box::new(Expr::Seq(
                                Box::new(Expr::RepExact(Box::new(Expr::Ident("a".to_owned())), 1)),
                                Box::new(Expr::RepMin(Box::new(Expr::Str("a".to_owned())), 3))
                            )),
                            Box::new(Expr::RepMax(Box::new(Expr::Ident("b".to_owned())), 2))
                        )),
                        Box::new(Expr::RepMinMax(Box::new(Expr::Str("b".to_owned())), 1, 2))
                    )),
                    Box::new(Expr::NegPred(Box::new(Expr::Rep(Box::new(Expr::Opt(
                        Box::new(Expr::Choice(
                            Box::new(Expr::Insens("c".to_owned())),
                            Box::new(Expr::Push(Box::new(Expr::Range(
                                "d".to_owned(),
                                "e".to_owned()
                            ))))
                        ))
                    ))))))
                )
            },]
        );
    }

    #[cfg(feature = "grammar-extras")]
    #[test]
    fn ast_push_literal() {
        let input = r#"rule = _{ PUSH_LITERAL("a") }"#;

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        let ast = consume_rules_with_spans(pairs).unwrap();
        let ast: Vec<_> = ast.into_iter().map(convert_rule).collect();

        assert_eq!(
            ast,
            vec![AstRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: Expr::PushLiteral("a".to_string()),
            }],
        );
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:24
  |
1 | rule = _{ PUSH_LITERAL(a) }
  |                        ^---
  |
  = expected quote")]
    fn ast_push_literal_bad_input() {
        let input = r#"rule = _{ PUSH_LITERAL(a) }"#;

        let pairs_result = PestParser::parse(Rule::grammar_rules, input);
        match pairs_result {
            Ok(ok) => panic!("expected Err, but found {ok}"),
            Err(e) => panic!("grammar error\n\n{e}"),
        }
    }

    #[test]
    fn ast_peek_slice() {
        let input = "rule = _{ PEEK[-04..] ~ PEEK[..3] }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        let ast = consume_rules_with_spans(pairs).unwrap();
        let ast: Vec<_> = ast.into_iter().map(convert_rule).collect();

        assert_eq!(
            ast,
            vec![AstRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: Expr::Seq(
                    Box::new(Expr::PeekSlice(-4, None)),
                    Box::new(Expr::PeekSlice(0, Some(3))),
                ),
            }],
        );
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | rule = { \"\"{4294967297} }
  |             ^--------^
  |
  = number cannot overflow u32")]
    fn repeat_exact_overflow() {
        let input = "rule = { \"\"{4294967297} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | rule = { \"\"{0} }
  |             ^
  |
  = cannot repeat 0 times")]
    fn repeat_exact_zero() {
        let input = "rule = { \"\"{0} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | rule = { \"\"{4294967297,} }
  |             ^--------^
  |
  = number cannot overflow u32")]
    fn repeat_min_overflow() {
        let input = "rule = { \"\"{4294967297,} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:14
  |
1 | rule = { \"\"{,4294967297} }
  |              ^--------^
  |
  = number cannot overflow u32")]
    fn repeat_max_overflow() {
        let input = "rule = { \"\"{,4294967297} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:14
  |
1 | rule = { \"\"{,0} }
  |              ^
  |
  = cannot repeat 0 times")]
    fn repeat_max_zero() {
        let input = "rule = { \"\"{,0} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | rule = { \"\"{4294967297,4294967298} }
  |             ^--------^
  |
  = number cannot overflow u32")]
    fn repeat_min_max_overflow() {
        let input = "rule = { \"\"{4294967297,4294967298} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:15
  |
1 | rule = { \"\"{0,0} }
  |               ^
  |
  = cannot repeat 0 times")]
    fn repeat_min_max_zero() {
        let input = "rule = { \"\"{0,0} }";

        let pairs = PestParser::parse(Rule::grammar_rules, input).unwrap();
        unwrap_or_report(consume_rules_with_spans(pairs));
    }

    #[test]
    fn unescape_all() {
        let string = r"a\nb\x55c\u{111}d";

        assert_eq!(unescape(string), Some("a\nb\x55c\u{111}d".to_owned()));
    }

    #[test]
    fn unescape_empty_escape() {
        let string = r"\";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_wrong_escape() {
        let string = r"\w";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_backslash() {
        let string = "\\\\";
        assert_eq!(unescape(string), Some("\\".to_owned()));
    }

    #[test]
    fn unescape_return() {
        let string = "\\r";
        assert_eq!(unescape(string), Some("\r".to_owned()));
    }

    #[test]
    fn unescape_tab() {
        let string = "\\t";
        assert_eq!(unescape(string), Some("\t".to_owned()));
    }

    #[test]
    fn unescape_null() {
        let string = "\\0";
        assert_eq!(unescape(string), Some("\0".to_owned()));
    }

    #[test]
    fn unescape_single_quote() {
        let string = "\\'";
        assert_eq!(unescape(string), Some("\'".to_owned()));
    }

    #[test]
    fn unescape_wrong_byte() {
        let string = r"\xfg";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_short_byte() {
        let string = r"\xf";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_no_open_brace_unicode() {
        let string = r"\u11";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_no_close_brace_unicode() {
        let string = r"\u{11";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_short_unicode() {
        let string = r"\u{1}";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn unescape_long_unicode() {
        let string = r"\u{1111111}";

        assert_eq!(unescape(string), None);
    }

    #[test]
    fn handles_deep_nesting() {
        let sample1 = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fuzzsample1.grammar"
        ));
        let sample2 = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fuzzsample2.grammar"
        ));
        let sample3 = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fuzzsample3.grammar"
        ));
        let sample4 = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fuzzsample4.grammar"
        ));
        let sample5 = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fuzzsample5.grammar"
        ));
        const ERROR: &str = "call limit reached";
        pest::set_call_limit(Some(5_000usize.try_into().unwrap()));
        let s1 = parse(Rule::grammar_rules, sample1);
        assert!(s1.is_err());
        assert_eq!(s1.unwrap_err().variant.message(), ERROR);
        let s2 = parse(Rule::grammar_rules, sample2);
        assert!(s2.is_err());
        assert_eq!(s2.unwrap_err().variant.message(), ERROR);
        let s3 = parse(Rule::grammar_rules, sample3);
        assert!(s3.is_err());
        assert_eq!(s3.unwrap_err().variant.message(), ERROR);
        let s4 = parse(Rule::grammar_rules, sample4);
        assert!(s4.is_err());
        assert_eq!(s4.unwrap_err().variant.message(), ERROR);
        let s5 = parse(Rule::grammar_rules, sample5);
        assert!(s5.is_err());
        assert_eq!(s5.unwrap_err().variant.message(), ERROR);
    }
}
