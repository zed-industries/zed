use crate::{
    error::{ParseError, Reason},
    expr::{
        ExprNode, Expression, Func, InnerPredicate,
        lexer::{Lexer, Token},
    },
};
use smallvec::SmallVec;

impl Expression {
    /// Given a `cfg()` expression (the cfg( and ) are optional), attempts to
    /// parse it into a form where it can be evaluated
    ///
    /// ```
    /// assert!(cfg_expr::Expression::parse(r#"cfg(all(unix, target_arch = "x86_64"))"#).is_ok());
    /// ```
    pub fn parse(original: &str) -> Result<Self, ParseError> {
        let lexer = Lexer::new(original);

        // The lexer automatically trims any cfg( ), so reacquire
        // the string before we start walking tokens
        let original = lexer.inner;

        #[derive(Debug)]
        struct FuncAndSpan {
            func: Func,
            parens_index: usize,
            span: std::ops::Range<usize>,
            predicates: SmallVec<[InnerPredicate; 5]>,
            nest_level: u8,
        }

        let mut func_stack = SmallVec::<[FuncAndSpan; 5]>::new();
        let mut expr_queue = SmallVec::<[ExprNode; 5]>::new();

        // Keep track of the last token to simplify validation of the token stream
        let mut last_token: Option<Token<'_>> = None;

        let parse_predicate = |key: (&str, std::ops::Range<usize>),
                               val: Option<(&str, std::ops::Range<usize>)>|
         -> Result<InnerPredicate, ParseError> {
            // Warning: It is possible for arbitrarily-set configuration
            // options to have the same value as compiler-set configuration
            // options. For example, it is possible to do rustc --cfg "unix" program.rs
            // while compiling to a Windows target, and have both unix and windows
            // configuration options set at the same time. It is unwise to actually
            // do this.
            //
            // rustc is very permissive in this regard, but I'd rather be really
            // strict, as it's much easier to loosen restrictions over time than add
            // new ones
            macro_rules! err_if_val {
                () => {
                    if let Some((_, vspan)) = val {
                        return Err(ParseError {
                            original: original.to_owned(),
                            span: vspan,
                            reason: Reason::Unexpected(&[]),
                        });
                    }
                };
            }

            let span = key.1;
            let key = key.0;

            use super::{InnerTarget, Which};

            Ok(match key {
                // These are special cases in the cfg language that are
                // semantically the same as `target_family = "<family>"`,
                // so we just make them not special
                // NOTE: other target families like "wasm" are NOT allowed
                // as naked predicates; they must be specified through
                // `target_family`
                "unix" | "windows" => {
                    err_if_val!();

                    InnerPredicate::Target(InnerTarget {
                        which: Which::Family,
                        span: Some(span),
                    })
                }
                "test" => {
                    err_if_val!();
                    InnerPredicate::Test
                }
                "debug_assertions" => {
                    err_if_val!();
                    InnerPredicate::DebugAssertions
                }
                "proc_macro" => {
                    err_if_val!();
                    InnerPredicate::ProcMacro
                }
                "feature" => {
                    // rustc allows bare feature without a value, but the only way
                    // such a predicate would ever evaluate to true would be if they
                    // explicitly set --cfg feature, which would be terrible, so we
                    // just error instead
                    match val {
                        Some((_, span)) => InnerPredicate::Feature(span),
                        None => {
                            return Err(ParseError {
                                original: original.to_owned(),
                                span,
                                reason: Reason::Unexpected(&["= \"<feature_name>\""]),
                            });
                        }
                    }
                }
                "panic" => match val {
                    Some((_, vspan)) => InnerPredicate::Target(InnerTarget {
                        which: Which::Panic,
                        span: Some(vspan),
                    }),
                    None => {
                        return Err(ParseError {
                            original: original.to_owned(),
                            span,
                            reason: Reason::Unexpected(&["= \"<panic_strategy>\""]),
                        });
                    }
                },
                target_key if key.starts_with("target_") => {
                    let (val, vspan) = match val {
                        None => {
                            return Err(ParseError {
                                original: original.to_owned(),
                                span,
                                reason: Reason::Unexpected(&["= \"<target_cfg_value>\""]),
                            });
                        }
                        Some((val, vspan)) => (val, vspan),
                    };

                    macro_rules! tp {
                        ($which:ident) => {
                            InnerTarget {
                                which: Which::$which,
                                span: Some(vspan),
                            }
                        };
                    }

                    let tp = match &target_key[7..] {
                        "abi" => tp!(Abi),
                        "arch" => tp!(Arch),
                        "feature" => {
                            if val.is_empty() {
                                return Err(ParseError {
                                    original: original.to_owned(),
                                    span: vspan,
                                    reason: Reason::Unexpected(&["<feature>"]),
                                });
                            }

                            return Ok(InnerPredicate::TargetFeature(vspan));
                        }
                        "os" => tp!(Os),
                        "family" => tp!(Family),
                        "env" => tp!(Env),
                        "endian" => InnerTarget {
                            which: Which::Endian(val.parse().map_err(|_err| ParseError {
                                original: original.to_owned(),
                                span: vspan,
                                reason: Reason::InvalidInteger,
                            })?),
                            span: None,
                        },
                        "has_atomic" => InnerTarget {
                            which: Which::HasAtomic(val.parse().map_err(|_err| ParseError {
                                original: original.to_owned(),
                                span: vspan,
                                reason: Reason::InvalidHasAtomic,
                            })?),
                            span: None,
                        },
                        "pointer_width" => InnerTarget {
                            which: Which::PointerWidth(val.parse().map_err(|_err| ParseError {
                                original: original.to_owned(),
                                span: vspan,
                                reason: Reason::InvalidInteger,
                            })?),
                            span: None,
                        },
                        "vendor" => tp!(Vendor),
                        _ => {
                            return Err(ParseError {
                                original: original.to_owned(),
                                span,
                                reason: Reason::Unexpected(&[
                                    "target_arch",
                                    "target_feature",
                                    "target_os",
                                    "target_family",
                                    "target_env",
                                    "target_endian",
                                    "target_has_atomic",
                                    "target_pointer_width",
                                    "target_vendor",
                                ]),
                            });
                        }
                    };

                    InnerPredicate::Target(tp)
                }
                _other => InnerPredicate::Other {
                    identifier: span,
                    value: val.map(|(_, span)| span),
                },
            })
        };

        macro_rules! token_err {
            ($span:expr) => {{
                let expected: &[&str] = match last_token {
                    None => &["<key>", "all", "any", "not"],
                    Some(Token::All | Token::Any | Token::Not) => &["("],
                    Some(Token::CloseParen) => &[")", ","],
                    Some(Token::Comma) => &[")", "<key>"],
                    Some(Token::Equals) => &["\""],
                    Some(Token::Key(_)) => &["=", ",", ")"],
                    Some(Token::Value(_)) => &[",", ")"],
                    Some(Token::OpenParen) => &["<key>", ")", "all", "any", "not"],
                };

                return Err(ParseError {
                    original: original.to_owned(),
                    span: $span,
                    reason: Reason::Unexpected(&expected),
                });
            }};
        }

        let mut pred_key: Option<(&str, _)> = None;
        let mut pred_val: Option<(&str, _)> = None;

        let mut root_predicate_count = 0;

        // Basic implementation of the https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        'outer: for lt in lexer {
            let lt = lt?;
            match &lt.token {
                Token::Key(k) => {
                    if matches!(last_token, None | Some(Token::OpenParen | Token::Comma)) {
                        pred_key = Some((k, lt.span.clone()));
                    } else {
                        token_err!(lt.span)
                    }
                }
                Token::Value(v) => {
                    if matches!(last_token, Some(Token::Equals)) {
                        // We only record the span for keys and values
                        // so that the expression doesn't need a lifetime
                        // but in the value case we need to strip off
                        // the quotes so that the proper raw string is
                        // provided to callers when evaluating the expression
                        pred_val = Some((v, lt.span.start + 1..lt.span.end - 1));
                    } else {
                        token_err!(lt.span)
                    }
                }
                Token::Equals => {
                    if !matches!(last_token, Some(Token::Key(_))) {
                        token_err!(lt.span)
                    }
                }
                Token::All | Token::Any | Token::Not => {
                    if matches!(last_token, None | Some(Token::OpenParen | Token::Comma)) {
                        let new_fn = match lt.token {
                            // the 0 is a dummy value -- it will be substituted for the real
                            // number of predicates in the `CloseParen` branch below.
                            Token::All => Func::All(0),
                            Token::Any => Func::Any(0),
                            Token::Not => Func::Not,
                            _ => unreachable!(),
                        };

                        if let Some(fs) = func_stack.last_mut() {
                            fs.nest_level += 1;
                        }

                        func_stack.push(FuncAndSpan {
                            func: new_fn,
                            span: lt.span,
                            parens_index: 0,
                            predicates: SmallVec::new(),
                            nest_level: 0,
                        });
                    } else {
                        token_err!(lt.span)
                    }
                }
                Token::OpenParen => {
                    if matches!(last_token, Some(Token::All | Token::Any | Token::Not)) {
                        if let Some(ref mut fs) = func_stack.last_mut() {
                            fs.parens_index = lt.span.start;
                        }
                    } else {
                        token_err!(lt.span)
                    }
                }
                Token::CloseParen => {
                    if matches!(
                        last_token,
                        None | Some(Token::All | Token::Any | Token::Not | Token::Equals)
                    ) {
                        token_err!(lt.span)
                    } else {
                        if let Some(top) = func_stack.pop() {
                            let key = pred_key.take();
                            let val = pred_val.take();

                            // In this context, the boolean to int conversion is confusing.
                            #[allow(clippy::bool_to_int_with_if)]
                            let num_predicates = top.predicates.len()
                                + if key.is_some() { 1 } else { 0 }
                                + top.nest_level as usize;

                            let func = match top.func {
                                Func::All(_) => Func::All(num_predicates),
                                Func::Any(_) => Func::Any(num_predicates),
                                Func::Not => {
                                    // not() doesn't take a predicate list, but only a single predicate,
                                    // so ensure we have exactly 1
                                    if num_predicates != 1 {
                                        return Err(ParseError {
                                            original: original.to_owned(),
                                            span: top.span.start..lt.span.end,
                                            reason: Reason::InvalidNot(num_predicates),
                                        });
                                    }

                                    Func::Not
                                }
                            };

                            for pred in top.predicates {
                                expr_queue.push(ExprNode::Predicate(pred));
                            }

                            if let Some(key) = key {
                                let inner_pred = parse_predicate(key, val)?;
                                expr_queue.push(ExprNode::Predicate(inner_pred));
                            }

                            expr_queue.push(ExprNode::Fn(func));

                            // This is the only place we go back to the top of the outer loop,
                            // so make sure we correctly record this token
                            last_token = Some(Token::CloseParen);
                            continue 'outer;
                        }

                        // We didn't have an opening parentheses if we get here
                        return Err(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::UnopenedParens,
                        });
                    }
                }
                Token::Comma => {
                    if matches!(
                        last_token,
                        None | Some(
                            Token::OpenParen | Token::All | Token::Any | Token::Not | Token::Equals
                        )
                    ) {
                        token_err!(lt.span)
                    } else {
                        let key = pred_key.take();
                        let val = pred_val.take();

                        let inner_pred = key.map(|key| parse_predicate(key, val)).transpose()?;

                        match (inner_pred, func_stack.last_mut()) {
                            (Some(pred), Some(func)) => {
                                func.predicates.push(pred);
                            }
                            (Some(pred), None) => {
                                root_predicate_count += 1;

                                expr_queue.push(ExprNode::Predicate(pred));
                            }
                            _ => {}
                        }
                    }
                }
            }

            last_token = Some(lt.token);
        }

        if let Some(Token::Equals) = last_token {
            return Err(ParseError {
                original: original.to_owned(),
                span: original.len()..original.len(),
                reason: Reason::Unexpected(&["\"<value>\""]),
            });
        }

        // If we still have functions on the stack, it means we have an unclosed parens
        if let Some(top) = func_stack.pop() {
            if top.parens_index != 0 {
                Err(ParseError {
                    original: original.to_owned(),
                    span: top.parens_index..original.len(),
                    reason: Reason::UnclosedParens,
                })
            } else {
                Err(ParseError {
                    original: original.to_owned(),
                    span: top.span,
                    reason: Reason::Unexpected(&["("]),
                })
            }
        } else {
            let key = pred_key.take();
            let val = pred_val.take();

            if let Some(key) = key {
                root_predicate_count += 1;
                expr_queue.push(ExprNode::Predicate(parse_predicate(key, val)?));
            }

            if expr_queue.is_empty() {
                Err(ParseError {
                    original: original.to_owned(),
                    span: 0..original.len(),
                    reason: Reason::Empty,
                })
            } else if root_predicate_count > 1 {
                Err(ParseError {
                    original: original.to_owned(),
                    span: 0..original.len(),
                    reason: Reason::MultipleRootPredicates,
                })
            } else {
                Ok(Expression {
                    original: original.to_owned(),
                    expr: expr_queue,
                })
            }
        }
    }
}
