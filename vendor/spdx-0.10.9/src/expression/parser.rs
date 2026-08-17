use crate::{
    LicenseItem, LicenseReq, ParseMode,
    error::{ParseError, Reason},
    expression::{ExprNode, Expression, ExpressionReq, Operator},
    lexer::{Lexer, Token},
};
use smallvec::SmallVec;

impl Expression {
    /// Given a license expression, attempts to parse and validate it as a valid
    /// SPDX expression. Uses `ParseMode::Strict`.
    ///
    /// The validation can fail for many reasons:
    /// * The expression contains invalid characters
    /// * An unknown/invalid license or exception identifier was found. Only
    ///   [SPDX short identifiers](https://spdx.org/ids) are allowed
    /// * The expression contained unbalanced parentheses
    /// * A license or exception immediately follows another license or exception, without
    ///   a valid AND, OR, or WITH operator separating them
    /// * An AND, OR, or WITH doesn't have a license or `)` preceding it
    ///
    /// ```
    /// spdx::Expression::parse("MIT OR Apache-2.0 WITH LLVM-exception").unwrap();
    /// ```
    pub fn parse(original: &str) -> Result<Self, ParseError> {
        Self::parse_mode(original, ParseMode::STRICT)
    }

    /// Canonicalizes the input expression into a form that can be parsed with
    /// [`ParseMode::STRICT`]
    ///
    /// ## Transforms
    ///
    /// 1. '/' is replaced with ' OR '
    /// 1. Lower-cased operators ('or', 'and', 'with') are upper-cased
    /// 1. '+' is tranformed to `-or-later` for GNU licenses
    /// 1. Invalid/imprecise license identifiers (eg. `apache2`) are replaced
    ///    with their valid identifiers
    ///
    /// If the provided expression is not modified then `None` is returned
    ///
    /// Note that this only does fixup of otherwise valid expressions, passing
    /// the resulting string to [`Expression::parse`] can still result in
    /// additional parse errors, eg. unbalanced parentheses
    ///
    /// ```
    /// assert_eq!(spdx::Expression::canonicalize("apache with LLVM-exception/gpl-3.0+").unwrap().unwrap(), "Apache-2.0 WITH LLVM-exception OR GPL-3.0-or-later");
    /// ```
    pub fn canonicalize(original: &str) -> Result<Option<String>, ParseError> {
        let mut can = String::with_capacity(original.len());

        let lexer = Lexer::new_mode(original, ParseMode::LAX);

        // Keep track if the last license id is a GNU license that uses the -or-later
        // convention rather than the + like all other licenses
        let mut last_is_gnu = false;
        for tok in lexer {
            let tok = tok?;

            match tok.token {
                Token::Spdx(id) => {
                    last_is_gnu = id.is_gnu();
                    can.push_str(id.name);
                }
                Token::And => can.push_str(" AND "),
                Token::Or => can.push_str(" OR "),
                Token::With => can.push_str(" WITH "),
                Token::Plus => {
                    if last_is_gnu {
                        can.push_str("-or-later");
                    } else {
                        can.push('+');
                    }
                }
                Token::OpenParen => can.push('('),
                Token::CloseParen => can.push(')'),
                Token::Exception(exc) => can.push_str(exc.name),
                Token::LicenseRef { doc_ref, lic_ref } => {
                    if let Some(dr) = doc_ref {
                        can.push_str("DocumentRef-");
                        can.push_str(dr);
                        can.push(':');
                    }

                    can.push_str("LicenseRef-");
                    can.push_str(lic_ref);
                }
            }
        }

        Ok((can != original).then_some(can))
    }

    /// Parses an expression with the specified `ParseMode`. With
    /// `ParseMode::Lax` it permits some non-SPDX syntax, such as imprecise
    /// license names and "/" used instead of "OR" in exprssions.
    ///
    /// ```
    /// spdx::Expression::parse_mode(
    ///     "mit/Apache-2.0 WITH LLVM-exception",
    ///     spdx::ParseMode::LAX
    /// ).unwrap();
    /// ```
    pub fn parse_mode(original: &str, mode: ParseMode) -> Result<Self, ParseError> {
        // Operator precedence in SPDX 2.1
        // +
        // WITH
        // AND
        // OR
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        enum Op {
            //Plus,
            //With,
            And,
            Or,
            Open,
        }

        struct OpAndSpan {
            op: Op,
            span: std::ops::Range<usize>,
        }

        let lexer = Lexer::new_mode(original, mode);
        let mut op_stack = SmallVec::<[OpAndSpan; 3]>::new();
        let mut expr_queue = SmallVec::<[ExprNode; 5]>::new();

        // Keep track of the last token to simplify validation of the token stream
        let mut last_token: Option<Token<'_>> = None;

        let apply_op = |op: OpAndSpan, q: &mut SmallVec<[ExprNode; 5]>| {
            let op = match op.op {
                Op::And => Operator::And,
                Op::Or => Operator::Or,
                Op::Open => unreachable!(),
            };

            q.push(ExprNode::Op(op));
            Ok(())
        };

        let make_err_for_token = |last_token: Option<Token<'_>>, span: std::ops::Range<usize>| {
            let expected: &[&str] = match last_token {
                None | Some(Token::And | Token::Or | Token::OpenParen) => &["<license>", "("],
                Some(Token::CloseParen) => &["AND", "OR"],
                Some(Token::Exception(_)) => &["AND", "OR", ")"],
                Some(Token::Spdx(_)) => &["AND", "OR", "WITH", ")", "+"],
                Some(Token::LicenseRef { .. } | Token::Plus) => &["AND", "OR", "WITH", ")"],
                Some(Token::With) => &["<exception>"],
            };

            Err(ParseError {
                original: original.to_owned(),
                span,
                reason: Reason::Unexpected(expected),
            })
        };

        // Basic implementation of the https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        'outer: for tok in lexer {
            let lt = tok?;
            match &lt.token {
                Token::Spdx(id) => match last_token {
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
                        expr_queue.push(ExprNode::Req(ExpressionReq {
                            req: LicenseReq::from(*id),
                            span: lt.span.start as u32..lt.span.end as u32,
                        }));
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::LicenseRef { doc_ref, lic_ref } => match last_token {
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
                        expr_queue.push(ExprNode::Req(ExpressionReq {
                            req: LicenseReq {
                                license: LicenseItem::Other {
                                    doc_ref: doc_ref.map(String::from),
                                    lic_ref: String::from(*lic_ref),
                                },
                                exception: None,
                            },
                            span: lt.span.start as u32..lt.span.end as u32,
                        }));
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::Plus => match last_token {
                    Some(Token::Spdx(_)) => match expr_queue.last_mut().unwrap() {
                        ExprNode::Req(ExpressionReq {
                            req:
                                LicenseReq {
                                    license: LicenseItem::Spdx { or_later, id },
                                    ..
                                },
                            ..
                        }) => {
                            // Handle GNU licenses differently, as they should *NOT* be used with the `+`
                            if !mode.allow_postfix_plus_on_gpl && id.is_gnu() {
                                return Err(ParseError {
                                    original: original.to_owned(),
                                    span: lt.span,
                                    reason: Reason::GnuNoPlus,
                                });
                            }

                            *or_later = true;
                        }
                        _ => unreachable!(),
                    },
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::With => match last_token {
                    Some(Token::Spdx(_) | Token::LicenseRef { .. } | Token::Plus) => {}
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::Or | Token::And => match last_token {
                    Some(
                        Token::Spdx(_)
                        | Token::LicenseRef { .. }
                        | Token::CloseParen
                        | Token::Exception(_)
                        | Token::Plus,
                    ) => {
                        let new_op = match lt.token {
                            Token::Or => Op::Or,
                            Token::And => Op::And,
                            _ => unreachable!(),
                        };

                        while let Some(op) = op_stack.last() {
                            match &op.op {
                                Op::Open => break,
                                top => {
                                    if *top < new_op {
                                        let top = op_stack.pop().unwrap();

                                        match top.op {
                                            Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                                            Op::Open => unreachable!(),
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }

                        op_stack.push(OpAndSpan {
                            op: new_op,
                            span: lt.span,
                        });
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::OpenParen => match last_token {
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
                        op_stack.push(OpAndSpan {
                            op: Op::Open,
                            span: lt.span,
                        });
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::CloseParen => {
                    match last_token {
                        Some(
                            Token::Spdx(_)
                            | Token::LicenseRef { .. }
                            | Token::Plus
                            | Token::Exception(_)
                            | Token::CloseParen,
                        ) => {
                            while let Some(top) = op_stack.pop() {
                                match top.op {
                                    Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                                    Op::Open => {
                                        // This is the only place we go back to the top of the outer loop,
                                        // so make sure we correctly record this token
                                        last_token = Some(Token::CloseParen);
                                        continue 'outer;
                                    }
                                }
                            }

                            // We didn't have an opening parentheses if we get here
                            return Err(ParseError {
                                original: original.to_owned(),
                                span: lt.span,
                                reason: Reason::UnopenedParens,
                            });
                        }
                        _ => return make_err_for_token(last_token, lt.span),
                    }
                }
                Token::Exception(exc) => match last_token {
                    Some(Token::With) => match expr_queue.last_mut() {
                        Some(ExprNode::Req(lic)) => {
                            lic.req.exception = Some(*exc);
                        }
                        _ => unreachable!(),
                    },
                    _ => return make_err_for_token(last_token, lt.span),
                },
            }

            last_token = Some(lt.token);
        }

        // Validate that the terminating token is valid
        match last_token {
            Some(
                Token::Spdx(_)
                | Token::LicenseRef { .. }
                | Token::Exception(_)
                | Token::CloseParen
                | Token::Plus,
            ) => {}
            // We have to have at least one valid license requirement
            None => {
                return Err(ParseError {
                    original: original.to_owned(),
                    span: 0..original.len(),
                    reason: Reason::Empty,
                });
            }
            Some(_) => return make_err_for_token(last_token, original.len()..original.len()),
        }

        while let Some(top) = op_stack.pop() {
            match top.op {
                Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                Op::Open => {
                    return Err(ParseError {
                        original: original.to_owned(),
                        span: top.span,
                        reason: Reason::UnclosedParens,
                    });
                }
            }
        }

        // TODO: Investigate using https://github.com/oli-obk/quine-mc_cluskey to simplify
        // expressions, but not really critical. Just cool.

        Ok(Expression {
            original: original.to_owned(),
            expr: expr_queue,
        })
    }
}
