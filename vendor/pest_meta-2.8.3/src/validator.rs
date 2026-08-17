// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Helpers for validating pest grammars that could help with debugging
//! and provide a more user-friendly error message.

use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use pest::error::{Error, ErrorVariant, InputLocation};
use pest::iterators::Pairs;
use pest::unicode::unicode_property_names;
use pest::Span;

use crate::parser::{ParserExpr, ParserNode, ParserRule, Rule};

static RUST_KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
        "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
        "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pure",
        "pub", "ref", "return", "Self", "self", "sizeof", "static", "struct", "super", "trait",
        "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ]
    .iter()
    .cloned()
    .collect()
});

static PEST_KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "_", "ANY", "DROP", "EOI", "PEEK", "PEEK_ALL", "POP", "POP_ALL", "PUSH", "SOI",
    ]
    .iter()
    .cloned()
    .collect()
});

static BUILTINS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "ANY",
        "DROP",
        "EOI",
        "PEEK",
        "PEEK_ALL",
        "POP",
        "POP_ALL",
        "SOI",
        "ASCII_DIGIT",
        "ASCII_NONZERO_DIGIT",
        "ASCII_BIN_DIGIT",
        "ASCII_OCT_DIGIT",
        "ASCII_HEX_DIGIT",
        "ASCII_ALPHA_LOWER",
        "ASCII_ALPHA_UPPER",
        "ASCII_ALPHA",
        "ASCII_ALPHANUMERIC",
        "ASCII",
        "NEWLINE",
    ]
    .iter()
    .cloned()
    .chain(unicode_property_names())
    .collect::<HashSet<&str>>()
});

/// It checks the parsed grammar for common mistakes:
/// - using Pest keywords
/// - duplicate rules
/// - undefined rules
///
/// It returns a `Result` with a `Vec` of `Error`s if any of the above is found.
/// If no errors are found, it returns the vector of names of used builtin rules.
pub fn validate_pairs(pairs: Pairs<'_, Rule>) -> Result<Vec<&str>, Vec<Error<Rule>>> {
    let definitions: Vec<_> = pairs
        .clone()
        .filter(|pair| pair.as_rule() == Rule::grammar_rule)
        .map(|pair| pair.into_inner().next().unwrap())
        .filter(|pair| pair.as_rule() != Rule::line_doc)
        .map(|pair| pair.as_span())
        .collect();

    let called_rules: Vec<_> = pairs
        .clone()
        .filter(|pair| pair.as_rule() == Rule::grammar_rule)
        .flat_map(|pair| {
            pair.into_inner()
                .flatten()
                .skip(1)
                .filter(|pair| pair.as_rule() == Rule::identifier)
                .map(|pair| pair.as_span())
        })
        .collect();

    let mut errors = vec![];

    errors.extend(validate_pest_keywords(&definitions));
    errors.extend(validate_already_defined(&definitions));
    errors.extend(validate_undefined(&definitions, &called_rules));

    if !errors.is_empty() {
        return Err(errors);
    }

    let definitions: HashSet<_> = definitions.iter().map(|span| span.as_str()).collect();
    let called_rules: HashSet<_> = called_rules.iter().map(|span| span.as_str()).collect();

    let defaults = called_rules.difference(&definitions);

    Ok(defaults.cloned().collect())
}

/// Validates that the given `definitions` do not contain any Rust keywords.
#[allow(clippy::ptr_arg)]
#[deprecated = "Rust keywords are no longer restricted from the pest grammar"]
pub fn validate_rust_keywords(definitions: &Vec<Span<'_>>) -> Vec<Error<Rule>> {
    let mut errors = vec![];

    for definition in definitions {
        let name = definition.as_str();

        if RUST_KEYWORDS.contains(name) {
            errors.push(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("{} is a rust keyword", name),
                },
                *definition,
            ))
        }
    }

    errors
}

/// Validates that the given `definitions` do not contain any Pest keywords.
#[allow(clippy::ptr_arg)]
pub fn validate_pest_keywords(definitions: &Vec<Span<'_>>) -> Vec<Error<Rule>> {
    let mut errors = vec![];

    for definition in definitions {
        let name = definition.as_str();

        if PEST_KEYWORDS.contains(name) {
            errors.push(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("{} is a pest keyword", name),
                },
                *definition,
            ))
        }
    }

    errors
}

/// Validates that the given `definitions` do not contain any duplicate rules.
#[allow(clippy::ptr_arg)]
pub fn validate_already_defined(definitions: &Vec<Span<'_>>) -> Vec<Error<Rule>> {
    let mut errors = vec![];
    let mut defined = HashSet::new();

    for definition in definitions {
        let name = definition.as_str();

        if defined.contains(&name) {
            errors.push(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("rule {} already defined", name),
                },
                *definition,
            ))
        } else {
            defined.insert(name);
        }
    }

    errors
}

/// Validates that the given `definitions` do not contain any undefined rules.
#[allow(clippy::ptr_arg)]
pub fn validate_undefined<'i>(
    definitions: &Vec<Span<'i>>,
    called_rules: &Vec<Span<'i>>,
) -> Vec<Error<Rule>> {
    let mut errors = vec![];
    let definitions: HashSet<_> = definitions.iter().map(|span| span.as_str()).collect();

    for rule in called_rules {
        let name = rule.as_str();

        if !definitions.contains(name) && !BUILTINS.contains(name) {
            errors.push(Error::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("rule {} is undefined", name),
                },
                *rule,
            ))
        }
    }

    errors
}

/// Validates the abstract syntax tree for common mistakes:
/// - infinite repetitions
/// - choices that cannot be reached
/// - left recursion
#[allow(clippy::ptr_arg)]
pub fn validate_ast<'a, 'i: 'a>(rules: &'a Vec<ParserRule<'i>>) -> Vec<Error<Rule>> {
    let mut errors = vec![];

    // WARNING: validate_{repetition,choice,whitespace_comment}
    // use is_non_failing and is_non_progressing breaking assumptions:
    // - for every `ParserExpr::RepMinMax(inner,min,max)`,
    //   `min<=max` was not checked
    // - left recursion was not checked
    // - Every expression might not be checked
    errors.extend(validate_repetition(rules));
    errors.extend(validate_choices(rules));
    errors.extend(validate_whitespace_comment(rules));
    errors.extend(validate_left_recursion(rules));
    #[cfg(feature = "grammar-extras")]
    errors.extend(validate_tag_silent_rules(rules));

    errors.sort_by_key(|error| match error.location {
        InputLocation::Span(span) => span,
        _ => unreachable!(),
    });

    errors
}

#[cfg(feature = "grammar-extras")]
fn validate_tag_silent_rules<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> Vec<Error<Rule>> {
    use crate::ast::RuleType;

    fn to_type_hash_map<'a, 'i: 'a>(
        rules: &'a [ParserRule<'i>],
    ) -> HashMap<String, (&'a ParserNode<'i>, RuleType)> {
        rules
            .iter()
            .map(|r| (r.name.clone(), (&r.node, r.ty)))
            .collect()
    }
    let mut result = vec![];

    fn check_silent_builtin<'a, 'i: 'a>(
        expr: &ParserExpr<'i>,
        rules_ref: &HashMap<String, (&'a ParserNode<'i>, RuleType)>,
        span: Span<'a>,
    ) -> Option<Error<Rule>> {
        match &expr {
            ParserExpr::Ident(rule_name) => {
                let rule = rules_ref.get(rule_name);
                if matches!(rule, Some((_, RuleType::Silent))) {
                    return Some(Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: "tags on silent rules will not appear in the output"
                                .to_owned(),
                        },
                        span,
                    ));
                } else if BUILTINS.contains(rule_name.as_str()) {
                    return Some(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: "tags on built-in rules will not appear in the output"
                                .to_owned(),
                        },
                        span,
                    ));
                }
            }
            ParserExpr::Rep(node)
            | ParserExpr::RepMinMax(node, _, _)
            | ParserExpr::RepMax(node, _)
            | ParserExpr::RepMin(node, _)
            | ParserExpr::RepOnce(node)
            | ParserExpr::RepExact(node, _)
            | ParserExpr::Opt(node)
            | ParserExpr::Push(node)
            | ParserExpr::PosPred(node)
            | ParserExpr::NegPred(node) => {
                return check_silent_builtin(&node.expr, rules_ref, span);
            }
            _ => {}
        };
        None
    }

    let rules_map = to_type_hash_map(rules);
    for rule in rules {
        let rules_ref = &rules_map;
        let mut errors = rule.node.clone().filter_map_top_down(|node1| {
            if let ParserExpr::NodeTag(node2, _) = node1.expr {
                check_silent_builtin(&node2.expr, rules_ref, node1.span)
            } else {
                None
            }
        });
        result.append(&mut errors);
    }
    result
}

/// Checks if `expr` is non-progressing, that is the expression does not
/// consume any input or any stack. This includes expressions matching the empty input,
/// `SOI` and ̀ `EOI`, predicates and repetitions.
///
/// # Example
///
/// ```pest
/// not_progressing_1 = { "" }
/// not_progressing_2 = { "a"? }
/// not_progressing_3 = { !"a" }
/// ```
///
/// # Assumptions
/// - In `ParserExpr::RepMinMax(inner,min,max)`, `min<=max`
/// - All rules identiers have a matching definition
/// - There is no left-recursion (if only this one is broken returns false)
/// - Every expression is being checked
fn is_non_progressing<'i>(
    expr: &ParserExpr<'i>,
    rules: &HashMap<String, &ParserNode<'i>>,
    trace: &mut Vec<String>,
) -> bool {
    match *expr {
        ParserExpr::Str(ref string) | ParserExpr::Insens(ref string) => string.is_empty(),
        ParserExpr::Ident(ref ident) => {
            if ident == "SOI" || ident == "EOI" {
                return true;
            }

            if !trace.contains(ident) {
                if let Some(node) = rules.get(ident) {
                    trace.push(ident.clone());
                    let result = is_non_progressing(&node.expr, rules, trace);
                    trace.pop().unwrap();

                    return result;
                }
                // else
                // the ident is
                // - "POP","PEEK" => false
                //      the slice being checked is not non_progressing since every
                //      PUSH is being checked (assumption 4) and the expr
                //      of a PUSH has to be non_progressing.
                // - "POPALL", "PEEKALL" => false
                //      same as "POP", "PEEK" unless the following:
                //      BUG: if the stack is empty they are non_progressing
                // - "DROP" => false doesn't consume the input but consumes the stack,
                // - "ANY", "ASCII_*", UNICODE categories, "NEWLINE" => false
                // - referring to another rule that is undefined (breaks assumption)
            }
            // else referring to another rule that was already seen.
            //    this happens only if there is a left-recursion
            //    that is only if an assumption is broken,
            //    WARNING: we can choose to return false, but that might
            //    cause bugs into the left_recursion check

            false
        }
        ParserExpr::Seq(ref lhs, ref rhs) => {
            is_non_progressing(&lhs.expr, rules, trace)
                && is_non_progressing(&rhs.expr, rules, trace)
        }
        ParserExpr::Choice(ref lhs, ref rhs) => {
            is_non_progressing(&lhs.expr, rules, trace)
                || is_non_progressing(&rhs.expr, rules, trace)
        }
        // WARNING: the predicate indeed won't make progress on input but  it
        // might progress on the stack
        // ex: @{ PUSH(ANY) ~ (&(DROP))* ~ ANY }, input="AA"
        //     Notice that this is ex not working as of now, the debugger seems
        //     to run into an infinite loop on it
        ParserExpr::PosPred(_) | ParserExpr::NegPred(_) => true,
        ParserExpr::Rep(_) | ParserExpr::Opt(_) | ParserExpr::RepMax(_, _) => true,
        // it either always fail (failing is progressing)
        // or always match at least a character
        ParserExpr::Range(_, _) => false,
        ParserExpr::PeekSlice(_, _) => {
            // the slice being checked is not non_progressing since every
            // PUSH is being checked (assumption 4) and the expr
            // of a PUSH has to be non_progressing.
            // BUG: if the slice is of size 0, or the stack is not large
            // enough it might be non-progressing
            false
        }

        ParserExpr::RepExact(ref inner, min)
        | ParserExpr::RepMin(ref inner, min)
        | ParserExpr::RepMinMax(ref inner, min, _) => {
            min == 0 || is_non_progressing(&inner.expr, rules, trace)
        }
        ParserExpr::Push(ref inner) => is_non_progressing(&inner.expr, rules, trace),
        #[cfg(feature = "grammar-extras")]
        ParserExpr::PushLiteral(_) => true,
        ParserExpr::RepOnce(ref inner) => is_non_progressing(&inner.expr, rules, trace),
        #[cfg(feature = "grammar-extras")]
        ParserExpr::NodeTag(ref inner, _) => is_non_progressing(&inner.expr, rules, trace),
    }
}

/// Checks if `expr` is non-failing, that is it matches any input.
///
/// # Example
///
/// ```pest
/// non_failing_1 = { "" }
/// ```
///
/// # Assumptions
/// - In `ParserExpr::RepMinMax(inner,min,max)`, `min<=max`
/// - In `ParserExpr::PeekSlice(max,Some(min))`, `max>=min`
/// - All rules identiers have a matching definition
/// - There is no left-recursion
/// - All rules are being checked
fn is_non_failing<'i>(
    expr: &ParserExpr<'i>,
    rules: &HashMap<String, &ParserNode<'i>>,
    trace: &mut Vec<String>,
) -> bool {
    match *expr {
        ParserExpr::Str(ref string) | ParserExpr::Insens(ref string) => string.is_empty(),
        ParserExpr::Ident(ref ident) => {
            if !trace.contains(ident) {
                if let Some(node) = rules.get(ident) {
                    trace.push(ident.clone());
                    let result = is_non_failing(&node.expr, rules, trace);
                    trace.pop().unwrap();

                    result
                } else {
                    // else
                    // the ident is
                    // - "POP","PEEK" => false
                    //      the slice being checked is not non_failing since every
                    //      PUSH is being checked (assumption 4) and the expr
                    //      of a PUSH has to be non_failing.
                    // - "POP_ALL", "PEEK_ALL" => false
                    //      same as "POP", "PEEK" unless the following:
                    //      BUG: if the stack is empty they are non_failing
                    // - "DROP" => false
                    // - "ANY", "ASCII_*", UNICODE categories, "NEWLINE",
                    //      "SOI", "EOI" => false
                    // - referring to another rule that is undefined (breaks assumption)
                    //      WARNING: might want to introduce a panic or report the error
                    false
                }
            } else {
                // referring to another rule R that was already seen
                // WARNING: this might mean there is a circular non-failing path
                //   it's not obvious wether this can happen without left-recursion
                //   and thus breaking the assumption. Until there is answer to
                //   this, to avoid changing behaviour we return:
                false
            }
        }
        ParserExpr::Opt(_) => true,
        ParserExpr::Rep(_) => true,
        ParserExpr::RepMax(_, _) => true,
        ParserExpr::Seq(ref lhs, ref rhs) => {
            is_non_failing(&lhs.expr, rules, trace) && is_non_failing(&rhs.expr, rules, trace)
        }
        ParserExpr::Choice(ref lhs, ref rhs) => {
            is_non_failing(&lhs.expr, rules, trace) || is_non_failing(&rhs.expr, rules, trace)
        }
        // it either always fail
        // or always match at least a character
        ParserExpr::Range(_, _) => false,
        ParserExpr::PeekSlice(_, _) => {
            // the slice being checked is not non_failing since every
            // PUSH is being checked (assumption 4) and the expr
            // of a PUSH has to be non_failing.
            // BUG: if the slice is of size 0, or the stack is not large
            // enough it might be non-failing
            false
        }
        ParserExpr::RepExact(ref inner, min)
        | ParserExpr::RepMin(ref inner, min)
        | ParserExpr::RepMinMax(ref inner, min, _) => {
            min == 0 || is_non_failing(&inner.expr, rules, trace)
        }
        // BUG: the predicate may always fail, resulting in this expr non_failing
        // ex of always failing predicates :
        //     @{EOI ~ ANY | ANY ~ SOI | &("A") ~ &("B") | 'z'..'a'}
        ParserExpr::NegPred(_) => false,
        ParserExpr::RepOnce(ref inner) => is_non_failing(&inner.expr, rules, trace),
        ParserExpr::Push(ref inner) | ParserExpr::PosPred(ref inner) => {
            is_non_failing(&inner.expr, rules, trace)
        }
        #[cfg(feature = "grammar-extras")]
        ParserExpr::PushLiteral(_) => true,
        #[cfg(feature = "grammar-extras")]
        ParserExpr::NodeTag(ref inner, _) => is_non_failing(&inner.expr, rules, trace),
    }
}

fn validate_repetition<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> Vec<Error<Rule>> {
    let mut result = vec![];
    let map = to_hash_map(rules);

    for rule in rules {
        let mut errors = rule.node
            .clone()
            .filter_map_top_down(|node| match node.expr {
                ParserExpr::Rep(ref other)
                | ParserExpr::RepOnce(ref other)
                | ParserExpr::RepMin(ref other, _) => {
                    if is_non_failing(&other.expr, &map, &mut vec![]) {
                        Some(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message:
                                    "expression inside repetition cannot fail and will repeat \
                                     infinitely"
                                        .to_owned()
                            },
                            node.span
                        ))
                    } else if is_non_progressing(&other.expr, &map, &mut vec![]) {
                        Some(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message:
                                    "expression inside repetition is non-progressing and will repeat \
                                     infinitely"
                                        .to_owned(),
                            },
                            node.span
                        ))
                    } else {
                        None
                    }
                }
                _ => None
            });

        result.append(&mut errors);
    }

    result
}

fn validate_choices<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> Vec<Error<Rule>> {
    let mut result = vec![];
    let map = to_hash_map(rules);

    for rule in rules {
        let mut errors = rule
            .node
            .clone()
            .filter_map_top_down(|node| match node.expr {
                ParserExpr::Choice(ref lhs, _) => {
                    let node = match lhs.expr {
                        ParserExpr::Choice(_, ref rhs) => rhs,
                        _ => lhs,
                    };

                    if is_non_failing(&node.expr, &map, &mut vec![]) {
                        Some(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message:
                                    "expression cannot fail; following choices cannot be reached"
                                        .to_owned(),
                            },
                            node.span,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            });

        result.append(&mut errors);
    }

    result
}

fn validate_whitespace_comment<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> Vec<Error<Rule>> {
    let map = to_hash_map(rules);

    rules
        .iter()
        .filter_map(|rule| {
            if rule.name == "WHITESPACE" || rule.name == "COMMENT" {
                if is_non_failing(&rule.node.expr, &map, &mut vec![]) {
                    Some(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!(
                                "{} cannot fail and will repeat infinitely",
                                &rule.name
                            ),
                        },
                        rule.node.span,
                    ))
                } else if is_non_progressing(&rule.node.expr, &map, &mut vec![]) {
                    Some(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!(
                                "{} is non-progressing and will repeat infinitely",
                                &rule.name
                            ),
                        },
                        rule.node.span,
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn validate_left_recursion<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> Vec<Error<Rule>> {
    left_recursion(to_hash_map(rules))
}

fn to_hash_map<'a, 'i: 'a>(rules: &'a [ParserRule<'i>]) -> HashMap<String, &'a ParserNode<'i>> {
    rules.iter().map(|r| (r.name.clone(), &r.node)).collect()
}

fn left_recursion<'a, 'i: 'a>(rules: HashMap<String, &'a ParserNode<'i>>) -> Vec<Error<Rule>> {
    fn check_expr<'a, 'i: 'a>(
        node: &'a ParserNode<'i>,
        rules: &'a HashMap<String, &ParserNode<'i>>,
        trace: &mut Vec<String>,
    ) -> Option<Error<Rule>> {
        match node.expr.clone() {
            ParserExpr::Ident(other) => {
                if trace[0] == other {
                    trace.push(other);
                    let chain = trace
                        .iter()
                        .map(|ident| ident.as_ref())
                        .collect::<Vec<_>>()
                        .join(" -> ");

                    return Some(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!(
                                "rule {} is left-recursive ({}); pest::pratt_parser might be useful \
                                 in this case",
                                node.span.as_str(),
                                chain
                            )
                        },
                        node.span
                    ));
                }

                if !trace.contains(&other) {
                    if let Some(node) = rules.get(&other) {
                        trace.push(other);
                        let result = check_expr(node, rules, trace);
                        trace.pop().unwrap();

                        return result;
                    }
                }

                None
            }
            ParserExpr::Seq(ref lhs, ref rhs) => {
                if is_non_failing(&lhs.expr, rules, &mut vec![trace.last().unwrap().clone()])
                    || is_non_progressing(
                        &lhs.expr,
                        rules,
                        &mut vec![trace.last().unwrap().clone()],
                    )
                {
                    check_expr(rhs, rules, trace)
                } else {
                    check_expr(lhs, rules, trace)
                }
            }
            ParserExpr::Choice(ref lhs, ref rhs) => {
                check_expr(lhs, rules, trace).or_else(|| check_expr(rhs, rules, trace))
            }
            ParserExpr::Rep(ref node) => check_expr(node, rules, trace),
            ParserExpr::RepOnce(ref node) => check_expr(node, rules, trace),
            ParserExpr::Opt(ref node) => check_expr(node, rules, trace),
            ParserExpr::PosPred(ref node) => check_expr(node, rules, trace),
            ParserExpr::NegPred(ref node) => check_expr(node, rules, trace),
            ParserExpr::Push(ref node) => check_expr(node, rules, trace),
            _ => None,
        }
    }

    let mut errors = vec![];

    for (name, node) in &rules {
        let name = name.clone();

        if let Some(error) = check_expr(node, &rules, &mut vec![name]) {
            errors.push(error);
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::super::parser::{consume_rules, PestParser};
    use super::super::unwrap_or_report;
    use super::*;
    use pest::Parser;

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:1
  |
1 | ANY = { \"a\" }
  | ^-^
  |
  = ANY is a pest keyword")]
    fn pest_keyword() {
        let input = "ANY = { \"a\" }";
        unwrap_or_report(validate_pairs(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" } a = { \"a\" }
  |             ^
  |
  = rule a already defined")]
    fn already_defined() {
        let input = "a = { \"a\" } a = { \"a\" }";
        unwrap_or_report(validate_pairs(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { b }
  |       ^
  |
  = rule b is undefined")]
    fn undefined() {
        let input = "a = { b }";
        unwrap_or_report(validate_pairs(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    fn valid_recursion() {
        let input = "a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"b\") ~ a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:16
  |
1 | WHITESPACE = { \"\" }
  |                ^^
  |
  = WHITESPACE cannot fail and will repeat infinitely")]
    fn non_failing_whitespace() {
        let input = "WHITESPACE = { \"\" }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | COMMENT = { SOI }
  |             ^-^
  |
  = COMMENT is non-progressing and will repeat infinitely")]
    fn non_progressing_comment() {
        let input = "COMMENT = { SOI }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    fn non_progressing_empty_string() {
        assert!(is_non_failing(
            &ParserExpr::Insens("".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::Str("".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn progressing_non_empty_string() {
        assert!(!is_non_progressing(
            &ParserExpr::Insens("non empty".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &ParserExpr::Str("non empty".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_progressing_soi_eoi() {
        assert!(is_non_progressing(
            &ParserExpr::Ident("SOI".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::Ident("EOI".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_progressing_predicates() {
        let progressing = ParserExpr::Str("A".into());

        assert!(is_non_progressing(
            &ParserExpr::PosPred(Box::new(ParserNode {
                expr: progressing.clone(),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::NegPred(Box::new(ParserNode {
                expr: progressing,
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_progressing_0_length_repetitions() {
        let input_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("A".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(!is_non_progressing(
            &input_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(is_non_progressing(
            &ParserExpr::Rep(input_progressing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::Opt(input_progressing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::RepExact(input_progressing_node.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::RepMin(input_progressing_node.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::RepMax(input_progressing_node.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::RepMax(input_progressing_node.clone(), 17),
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(is_non_progressing(
            &ParserExpr::RepMinMax(input_progressing_node.clone(), 0, 12),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_progressing_nonzero_repetitions_with_non_progressing_expr() {
        let a = "";
        let non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str(a.into()),
            span: Span::new(a, 0, 0).unwrap(),
        });
        let exact = ParserExpr::RepExact(non_progressing_node.clone(), 7);
        let min = ParserExpr::RepMin(non_progressing_node.clone(), 23);
        let minmax = ParserExpr::RepMinMax(non_progressing_node.clone(), 12, 13);
        let reponce = ParserExpr::RepOnce(non_progressing_node);

        assert!(is_non_progressing(&exact, &HashMap::new(), &mut Vec::new()));
        assert!(is_non_progressing(&min, &HashMap::new(), &mut Vec::new()));
        assert!(is_non_progressing(
            &minmax,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &reponce,
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn progressing_repetitions() {
        let a = "A";
        let input_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str(a.into()),
            span: Span::new(a, 0, 1).unwrap(),
        });
        let exact = ParserExpr::RepExact(input_progressing_node.clone(), 1);
        let min = ParserExpr::RepMin(input_progressing_node.clone(), 2);
        let minmax = ParserExpr::RepMinMax(input_progressing_node.clone(), 4, 5);
        let reponce = ParserExpr::RepOnce(input_progressing_node);

        assert!(!is_non_progressing(
            &exact,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(&min, &HashMap::new(), &mut Vec::new()));
        assert!(!is_non_progressing(
            &minmax,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &reponce,
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_progressing_push() {
        let a = "";
        let non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str(a.into()),
            span: Span::new(a, 0, 0).unwrap(),
        });
        let push = ParserExpr::Push(non_progressing_node.clone());

        assert!(is_non_progressing(&push, &HashMap::new(), &mut Vec::new()));
    }

    #[test]
    fn progressing_push() {
        let a = "i'm make progress";
        let progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str(a.into()),
            span: Span::new(a, 0, 1).unwrap(),
        });
        let push = ParserExpr::Push(progressing_node.clone());

        assert!(!is_non_progressing(&push, &HashMap::new(), &mut Vec::new()));
    }

    #[cfg(feature = "grammar-extras")]
    #[test]
    fn push_literal_is_non_progressing() {
        let a = "";
        let non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::PushLiteral("a".to_string()),
            span: Span::new(a, 0, 0).unwrap(),
        });
        let push = ParserExpr::Push(non_progressing_node.clone());

        assert!(is_non_progressing(&push, &HashMap::new(), &mut Vec::new()));
    }

    #[test]
    fn node_tag_forwards_is_non_progressing() {
        let progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm make progress".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_progressing(
            &progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        let non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(is_non_progressing(
            &non_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        #[cfg(feature = "grammar-extras")]
        {
            let progressing = ParserExpr::NodeTag(progressing_node.clone(), "TAG".into());
            let non_progressing = ParserExpr::NodeTag(non_progressing_node.clone(), "TAG".into());

            assert!(!is_non_progressing(
                &progressing,
                &HashMap::new(),
                &mut Vec::new()
            ));
            assert!(is_non_progressing(
                &non_progressing,
                &HashMap::new(),
                &mut Vec::new()
            ));
        }
    }

    #[test]
    fn progressing_range() {
        let progressing = ParserExpr::Range("A".into(), "Z".into());
        let failing_is_progressing = ParserExpr::Range("Z".into(), "A".into());

        assert!(!is_non_progressing(
            &progressing,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &failing_is_progressing,
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn progressing_choice() {
        let left_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm make progress".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_progressing(
            &left_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_progressing(
            &ParserExpr::Choice(left_progressing_node, right_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_progressing_choices() {
        let left_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm make progress".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(!is_non_progressing(
            &left_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let left_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_progressing(
            &left_non_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_progressing(
            &right_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_progressing(
            &right_non_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(is_non_progressing(
            &ParserExpr::Choice(left_non_progressing_node.clone(), right_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::Choice(left_progressing_node, right_non_progressing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_progressing(
            &ParserExpr::Choice(left_non_progressing_node, right_non_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_progressing_seq() {
        let left_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        let right_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_progressing(
            &ParserExpr::Seq(left_non_progressing_node, right_non_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn progressing_seqs() {
        let left_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm make progress".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(!is_non_progressing(
            &left_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let left_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_progressing(
            &left_non_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_progressing(
            &right_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_non_progressing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_progressing(
            &right_non_progressing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(!is_non_progressing(
            &ParserExpr::Seq(left_non_progressing_node, right_progressing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &ParserExpr::Seq(left_progressing_node.clone(), right_non_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &ParserExpr::Seq(left_progressing_node, right_progressing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn progressing_stack_operations() {
        assert!(!is_non_progressing(
            &ParserExpr::Ident("DROP".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &ParserExpr::Ident("PEEK".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_progressing(
            &ParserExpr::Ident("POP".into()),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_failing_string() {
        let insens = ParserExpr::Insens("".into());
        let string = ParserExpr::Str("".into());

        assert!(is_non_failing(&insens, &HashMap::new(), &mut Vec::new()));

        assert!(is_non_failing(&string, &HashMap::new(), &mut Vec::new()))
    }

    #[test]
    fn failing_string() {
        assert!(!is_non_failing(
            &ParserExpr::Insens("i may fail!".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::Str("failure is not fatal".into()),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn failing_stack_operations() {
        assert!(!is_non_failing(
            &ParserExpr::Ident("DROP".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::Ident("POP".into()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::Ident("PEEK".into()),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_failing_zero_length_repetitions() {
        let failing = Box::new(ParserNode {
            expr: ParserExpr::Range("A".into(), "B".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_failing(
            &failing.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::Opt(failing.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::Rep(failing.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepExact(failing.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMin(failing.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMax(failing.clone(), 0),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMax(failing.clone(), 22),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMinMax(failing.clone(), 0, 73),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn non_failing_non_zero_repetitions_with_non_failing_expr() {
        let non_failing = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Range("A".into(), "B".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(is_non_failing(
            &non_failing.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepOnce(non_failing.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepExact(non_failing.clone(), 1),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMin(non_failing.clone(), 6),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::RepMinMax(non_failing.clone(), 32, 73),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    #[cfg(feature = "grammar-extras")]
    fn failing_non_zero_repetitions() {
        let failing = Box::new(ParserNode {
            expr: ParserExpr::NodeTag(
                Box::new(ParserNode {
                    expr: ParserExpr::Range("A".into(), "B".into()),
                    span: Span::new(" ", 0, 1).unwrap(),
                }),
                "Tag".into(),
            ),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_failing(
            &failing.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::RepOnce(failing.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::RepExact(failing.clone(), 3),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::RepMin(failing.clone(), 14),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::RepMinMax(failing.clone(), 47, 73),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn failing_choice() {
        let left_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm a failure".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_failing(
            &left_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_failing(
            &ParserExpr::Choice(left_failing_node, right_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_failing_choices() {
        let left_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm a failure".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(!is_non_failing(
            &left_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let left_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_failing(
            &left_non_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_failing(
            &right_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_failing(
            &right_non_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(is_non_failing(
            &ParserExpr::Choice(left_non_failing_node.clone(), right_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::Choice(left_failing_node, right_non_failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::Choice(left_non_failing_node, right_non_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn non_failing_seq() {
        let left_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        let right_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_failing(
            &ParserExpr::Seq(left_non_failing_node, right_non_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn failing_seqs() {
        let left_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm a failure".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(!is_non_failing(
            &left_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let left_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_failing(
            &left_non_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Ident("DROP".into()),
            span: Span::new("DROP", 0, 3).unwrap(),
        });

        assert!(!is_non_failing(
            &right_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        let right_non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Opt(Box::new(ParserNode {
                expr: ParserExpr::Str("   ".into()),
                span: Span::new(" ", 0, 1).unwrap(),
            })),
            span: Span::new(" ", 0, 1).unwrap(),
        });

        assert!(is_non_failing(
            &right_non_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(!is_non_failing(
            &ParserExpr::Seq(left_non_failing_node, right_failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::Seq(left_failing_node.clone(), right_non_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(!is_non_failing(
            &ParserExpr::Seq(left_failing_node, right_failing_node),
            &HashMap::new(),
            &mut Vec::new()
        ))
    }

    #[test]
    fn failing_range() {
        let failing = ParserExpr::Range("A".into(), "Z".into());
        let always_failing = ParserExpr::Range("Z".into(), "A".into());

        assert!(!is_non_failing(&failing, &HashMap::new(), &mut Vec::new()));
        assert!(!is_non_failing(
            &always_failing,
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    fn _push_node_tag_pos_pred_forwarding_is_non_failing() {
        let failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("i'm a failure".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(!is_non_failing(
            &failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));
        let non_failing_node = Box::new(ParserNode {
            expr: ParserExpr::Str("".into()),
            span: Span::new(" ", 0, 1).unwrap(),
        });
        assert!(is_non_failing(
            &non_failing_node.clone().expr,
            &HashMap::new(),
            &mut Vec::new()
        ));

        #[cfg(feature = "grammar-extras")]
        {
            assert!(!is_non_failing(
                &ParserExpr::NodeTag(failing_node.clone(), "TAG".into()),
                &HashMap::new(),
                &mut Vec::new()
            ));
            assert!(is_non_failing(
                &ParserExpr::NodeTag(non_failing_node.clone(), "TAG".into()),
                &HashMap::new(),
                &mut Vec::new()
            ));
        }

        assert!(!is_non_failing(
            &ParserExpr::Push(failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::Push(non_failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));

        assert!(!is_non_failing(
            &ParserExpr::PosPred(failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
        assert!(is_non_failing(
            &ParserExpr::PosPred(non_failing_node.clone()),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[cfg(feature = "grammar-extras")]
    #[test]
    fn push_literal_is_non_failing() {
        assert!(is_non_failing(
            &ParserExpr::PushLiteral("a".to_string()),
            &HashMap::new(),
            &mut Vec::new()
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { (\"\")* }
  |       ^---^
  |
  = expression inside repetition cannot fail and will repeat infinitely")]
    fn non_failing_repetition() {
        let input = "a = { (\"\")* }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:18
  |
1 | a = { \"\" } b = { a* }
  |                  ^^
  |
  = expression inside repetition cannot fail and will repeat infinitely")]
    fn indirect_non_failing_repetition() {
        let input = "a = { \"\" } b = { a* }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:20
  |
1 | a = { \"a\" ~ (\"b\" ~ (\"\")*) }
  |                    ^---^
  |
  = expression inside repetition cannot fail and will repeat infinitely")]
    fn deep_non_failing_repetition() {
        let input = "a = { \"a\" ~ (\"b\" ~ (\"\")*) }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { (\"\" ~ &\"a\" ~ !\"a\" ~ (SOI | EOI))* }
  |       ^-------------------------------^
  |
  = expression inside repetition is non-progressing and will repeat infinitely")]
    fn non_progressing_repetition() {
        let input = "a = { (\"\" ~ &\"a\" ~ !\"a\" ~ (SOI | EOI))* }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:20
  |
1 | a = { !\"a\" } b = { a* }
  |                    ^^
  |
  = expression inside repetition is non-progressing and will repeat infinitely")]
    fn indirect_non_progressing_repetition() {
        let input = "a = { !\"a\" } b = { a* }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { a }
  |       ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")]
    fn simple_left_recursion() {
        let input = "a = { a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { b } b = { a }
  |       ^
  |
  = rule b is left-recursive (b -> a -> b); pest::pratt_parser might be useful in this case

 --> 1:17
  |
1 | a = { b } b = { a }
  |                 ^
  |
  = rule a is left-recursive (a -> b -> a); pest::pratt_parser might be useful in this case")]
    fn indirect_left_recursion() {
        let input = "a = { b } b = { a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:39
  |
1 | a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"\") ~ a }
  |                                       ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")]
    fn non_failing_left_recursion() {
        let input = "a = { \"\" ~ \"a\"? ~ \"a\"* ~ (\"a\" | \"\") ~ a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" | a }
  |             ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")]
    fn non_primary_choice_left_recursion() {
        let input = "a = { \"a\" | a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:14
  |
1 | a = { !\"a\" ~ a }
  |              ^
  |
  = rule a is left-recursive (a -> a); pest::pratt_parser might be useful in this case")]
    fn non_progressing_left_recursion() {
        let input = "a = { !\"a\" ~ a }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { \"a\"* | \"a\" | \"b\" }
  |       ^--^
  |
  = expression cannot fail; following choices cannot be reached")]
    fn lhs_non_failing_choice() {
        let input = "a = { \"a\"* | \"a\" | \"b\" }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:13
  |
1 | a = { \"a\" | \"a\"* | \"b\" }
  |             ^--^
  |
  = expression cannot fail; following choices cannot be reached")]
    fn lhs_non_failing_choice_middle() {
        let input = "a = { \"a\" | \"a\"* | \"b\" }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { b | \"a\" } b = { \"b\"* | \"c\" }
  |       ^
  |
  = expression cannot fail; following choices cannot be reached

 --> 1:23
  |
1 | a = { b | \"a\" } b = { \"b\"* | \"c\" }
  |                       ^--^
  |
  = expression cannot fail; following choices cannot be reached")]
    fn lhs_non_failing_nested_choices() {
        let input = "a = { b | \"a\" } b = { \"b\"* | \"c\" }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    fn skip_can_be_defined() {
        let input = "skip = { \"\" }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { #b = b } b = _{ ASCII_DIGIT+ }
  |       ^----^
  |
  = tags on silent rules will not appear in the output")]
    #[cfg(feature = "grammar-extras")]
    fn tag_on_silent_rule() {
        let input = "a = { #b = b } b = _{ ASCII_DIGIT+ }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[should_panic(expected = "grammar error

 --> 1:7
  |
1 | a = { #b = ASCII_DIGIT+ }
  |       ^---------------^
  |
  = tags on built-in rules will not appear in the output")]
    #[cfg(feature = "grammar-extras")]
    fn tag_on_builtin_rule() {
        let input = "a = { #b = ASCII_DIGIT+ }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }

    #[test]
    #[cfg(feature = "grammar-extras")]
    fn tag_on_normal_rule() {
        let input = "a = { #b = b } b = { ASCII_DIGIT+ }";
        unwrap_or_report(consume_rules(
            PestParser::parse(Rule::grammar_rules, input).unwrap(),
        ));
    }
}
